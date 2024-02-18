use std::mem::size_of;

use rand::prelude::*;
use wgpu::{
    include_wgsl,
    util::{BufferInitDescriptor, DeviceExt},
    BindGroupDescriptor, BindGroupEntry, BufferAsyncError, BufferDescriptor, BufferUsages,
    CommandEncoderDescriptor, ComputePassDescriptor, ComputePipelineDescriptor, Device, Queue,
};

use crate::utils::Test;

pub fn riemann() -> Test {
    let mut rng = thread_rng();
    let start = rng.gen_range(0..500) as f32;
    let answer = ((start + 10.).powi(3) - start.powi(3))/3.;

    let cpu = move || {
        let mut sum = 0.;
        for i in 0..1000000 {
            let num = start + (i as f32) / 1000000. * 10.;
            sum += num.powf(2.);
        }
        let res = sum * 10. / 1000000.;
        assert!(res > answer - 1. || res < answer + 1.);
    };

    let gpu = move |x: &mut (Device, Queue)| {
        let (dev, queue) = x;
        let module = dev.create_shader_module(include_wgsl!("reimann.wgsl"));

        let compute_pipeline = dev.create_compute_pipeline(&ComputePipelineDescriptor {
            label: None,
            layout: None,
            module: &module,
            entry_point: "main",
        });

        let start_buf = dev.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[start]),
            usage: BufferUsages::COPY_DST | BufferUsages::STORAGE,
        });

        let sum_buf = dev.create_buffer(&BufferDescriptor {
            label: None,
            size: size_of::<f32>() as u64 * 1000,
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
                    resource: start_buf.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: sum_buf.as_entire_binding(),
                }
            ],
        });

        let stag_buf = dev.create_buffer(&BufferDescriptor {
            label: None,
            size: size_of::<f32>() as u64 * 1000,
            usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        let mut encoder = dev.create_command_encoder(&CommandEncoderDescriptor::default());

        {
            let mut cpass = encoder.begin_compute_pass(&ComputePassDescriptor::default());
            cpass.set_pipeline(&compute_pipeline);
            cpass.set_bind_group(0, &bind_group, &[]);
            cpass.dispatch_workgroups(1000, 1, 1);
        }

        encoder.copy_buffer_to_buffer(&sum_buf, 0, &stag_buf, 0, size_of::<f32>() as u64 * 1000);

        queue.submit(Some(encoder.finish()));

        let buffer_slice = stag_buf.slice(..);
        let (sender, reciever) = std::sync::mpsc::channel::<Result<(), BufferAsyncError>>();
        buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

        dev.poll(wgpu::Maintain::wait()).panic_on_timeout();
        let _ = reciever.recv().unwrap();
        let data = buffer_slice.get_mapped_range();
        let sums: &[f32] = bytemuck::cast_slice(&data);
        let res = sums.iter().sum::<f32>() * 10. / 1000000.;

        drop(data);
        stag_buf.unmap();

        assert!(res > answer - 1. || res < answer + 1.);
    };

    Test {
        name: "reimann".to_string(),
        cpu: Box::new(cpu),
        gpu: Box::new(gpu),
    }
}
