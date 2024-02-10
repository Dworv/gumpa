use std::{iter, mem::size_of};

use wgpu::{
    include_wgsl, util::{BufferInitDescriptor, DeviceExt}, BindGroup, BindGroupDescriptor, BindGroupEntry, Buffer, BufferAddress, BufferAsyncError, BufferDescriptor, BufferUsages, CommandEncoderDescriptor, ComputePassDescriptor, ComputePipeline, ComputePipelineDescriptor, Device, DeviceDescriptor, Features, Instance, Limits, Queue, RequestAdapterOptions, ShaderModuleDescriptor
};

const N: u64 = 1000;


#[pollster::main]
async fn main() {
    println!("getting the riemann sum of x^2 from 0 to 2 with 10000 rectangles");

    let (device, queue, pipeline) = init_gpu().await;

    let out_buf = device.create_buffer(&BufferDescriptor { label: None, size: N * size_of::<f32>() as u64, usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC, mapped_at_creation: false });
    let stag_buf = device.create_buffer(&BufferDescriptor { label: None, size: N * size_of::<f32>() as u64, usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST, mapped_at_creation: false });

    let bgl = pipeline.get_bind_group_layout(0);
    let bg = device.create_bind_group(&BindGroupDescriptor {
        label: None,
        layout: &bgl,
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: out_buf.as_entire_binding()
            }
        ]
    });

    let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor { label: None });

    {
        let mut cpass = encoder.begin_compute_pass(&ComputePassDescriptor {
            label: None,
            timestamp_writes: None
        });

        cpass.set_pipeline(&pipeline);
        cpass.set_bind_group(0, &bg, &[]);
        cpass.dispatch_workgroups(N as u32, 1, 1);
    }

    encoder.copy_buffer_to_buffer(&out_buf, 0, &stag_buf, 0, N * size_of::<f32>() as u64);
    queue.submit(Some(encoder.finish()));

    let buffer_slice = stag_buf.slice(..);
    let (sender, reciever) = std::sync::mpsc::channel::<Result<(), BufferAsyncError>>();

    buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

    device.poll(wgpu::Maintain::wait()).panic_on_timeout();

    let _ = reciever.recv().unwrap();
    let data = buffer_slice.get_mapped_range();
    let result: Vec<f32> = bytemuck::cast_slice(&data).to_vec();

    drop(data);
    stag_buf.unmap();

    println!("riemann sum is {}", result.iter().sum::<f32>() * 2. / N as f32);
}

async fn init_gpu() -> (Device, Queue, ComputePipeline) {
    let instance = Instance::default();

    let adapter = instance
        .request_adapter(&RequestAdapterOptions::default())
        .await
        .unwrap();

    let (device, queue) = adapter
        .request_device(
            &DeviceDescriptor {
                label: None,
                required_features: Features::empty(),
                required_limits: Limits::downlevel_defaults(),
            },
            None,
        )
        .await
        .unwrap();

    let shader_module = device.create_shader_module(ShaderModuleDescriptor {
        label: None,
        source: include_wgsl!("shader.wgsl").source,
    });

    let compute_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
        label: None,
        layout: None,
        module: &shader_module,
        entry_point: "main",
    });

    (device, queue, compute_pipeline)
}
