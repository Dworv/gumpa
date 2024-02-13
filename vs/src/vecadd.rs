use rand::prelude::*;
use wgpu::{
    include_wgsl,
    util::{BufferInitDescriptor, DeviceExt},
    BindGroupDescriptor, BindGroupEntry, BufferAsyncError, BufferDescriptor, BufferUsages,
    CommandEncoderDescriptor, ComputePassDescriptor, ComputePipelineDescriptor, Device, Queue,
};

use crate::utils::Test;

pub fn vecadd() -> Test {
    let mut rng = thread_rng();

    let mut cpu_a = (0..500).collect::<Vec<u32>>();
    cpu_a.shuffle(&mut rng);
    let mut cpu_b = (0..500).collect::<Vec<u32>>();
    cpu_b.shuffle(&mut rng);
    let gpu_a = cpu_a.clone();
    let gpu_b = cpu_b.clone();

    let cpu = move || {
        let mut cpu_c = Vec::with_capacity(500);
        for i in 0..500 {
            cpu_c.push(cpu_a[i] + cpu_b[i]);
        }
        assert_eq!(cpu_c.len(), 500);
        println!("{:?}\n{:?}\n{:?}", &cpu_a[0..10], &cpu_b[0..10], &cpu_c[0..10]);
    };

    let gpu = move |x: &mut (Device, Queue)| {
        let (dev, queue) = x;
        let module = dev.create_shader_module(include_wgsl!("vecadd.wgsl"));

        let compute_pipeline = dev.create_compute_pipeline(&ComputePipelineDescriptor {
            label: None,
            layout: None,
            module: &module,
            entry_point: "main",
        });

        let a_buf = dev.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(gpu_a.as_slice()),
            usage: BufferUsages::COPY_DST | BufferUsages::STORAGE,
        });

        let b_buf = dev.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(gpu_b.as_slice()),
            usage: BufferUsages::COPY_DST | BufferUsages::STORAGE,
        });

        let c_buf = dev.create_buffer(&BufferDescriptor {
            label: None,
            size: (4 * gpu_a.len()) as u64,
            usage: BufferUsages::COPY_SRC | BufferUsages::STORAGE,
            mapped_at_creation: false,
        });

        let bgl = compute_pipeline.get_bind_group_layout(0);
        let bind_group = dev.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &bgl,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: a_buf.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: b_buf.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: c_buf.as_entire_binding(),
                },
            ],
        });

        let stag_buf = dev.create_buffer(&BufferDescriptor {
            label: None,
            size: (4 * gpu_a.len()) as u64,
            usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        let mut encoder = dev.create_command_encoder(&CommandEncoderDescriptor::default());

        {
            let mut cpass = encoder.begin_compute_pass(&ComputePassDescriptor::default());
            cpass.set_pipeline(&compute_pipeline);
            cpass.set_bind_group(0, &bind_group, &[]);
            cpass.dispatch_workgroups(gpu_a.len() as u32, 1, 1);
        }

        encoder.copy_buffer_to_buffer(&c_buf, 0, &stag_buf, 0, (4 * gpu_a.len()) as u64);

        queue.submit(Some(encoder.finish()));

        let buffer_slice = stag_buf.slice(..);
        let (sender, reciever) = std::sync::mpsc::channel::<Result<(), BufferAsyncError>>();
        buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

        dev.poll(wgpu::Maintain::wait()).panic_on_timeout();
        let _ = reciever.recv().unwrap();
        let data = buffer_slice.get_mapped_range();
        let result: Vec<u32> = bytemuck::cast_slice(&data).to_vec();

        drop(data);
        stag_buf.unmap();

        println!("{:?}\n{:?}\n{:?}", &gpu_a[0..10], &gpu_b[0..10], &result[0..10]);
    };

    Test {
        name: "vecadd".to_string(),
        cpu: Box::new(cpu),
        gpu: Box::new(gpu),
    }
}
