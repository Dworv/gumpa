use std::mem::size_of;

use wgpu::{
    include_wgsl, util::{BufferInitDescriptor, DeviceExt}, BindGroup, BindGroupDescriptor, BindGroupEntry, Buffer, BufferAddress, BufferAsyncError, BufferDescriptor, BufferUsages, ComputePipeline, ComputePipelineDescriptor, Device, DeviceDescriptor, Features, Instance, Limits, Queue, RequestAdapterOptions, ShaderModuleDescriptor
};

#[pollster::main]
async fn main() {
    let (device, queue, mut compute_pipeline) = init_gpu().await;

    let a = [1,2,3,4,5];
    let mut b = [3,3,3,3,3];

    for _ in 0..3 {
        b = [b[0] + 1;5];
        let (bind_group, out_buffer, staging_buffer) = create_buffers(&device, &mut compute_pipeline, &a, &b);

        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: None,
                timestamp_writes: None,
            });
            cpass.set_pipeline(&compute_pipeline);
            cpass.set_bind_group(0, &bind_group, &[]);
            cpass.insert_debug_marker("bonjour!");
            cpass.dispatch_workgroups(a.len() as u32, 1, 1);
        }

        encoder.copy_buffer_to_buffer(&out_buffer, 0, &staging_buffer, 0, std::mem::size_of_val(&a) as u64);

        queue.submit(Some(encoder.finish()));

        let buffer_slice = staging_buffer.slice(..);
        let (sender, reciever) = std::sync::mpsc::channel::<Result<(), BufferAsyncError>>();

        buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

        device.poll(wgpu::Maintain::wait()).panic_on_timeout();

        let _ = reciever.recv().unwrap();
        let data = buffer_slice.get_mapped_range();
        let result: Vec<u32> = bytemuck::cast_slice(&data).to_vec();

        drop(data);
        staging_buffer.unmap();

        // println!("result: {:?}", result);
        println!("results: {:?}", result);
    }
    
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

fn create_buffers(device: &Device, pipeline: &mut ComputePipeline, a: &[u32], b: &[u32]) -> (BindGroup, Buffer, Buffer) {
    let size = (size_of::<u32>() * a.len()) as BufferAddress;

    let in1_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(a),
        usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
    });

    let in2_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(b),
        usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
    });

    let out_buffer = device.create_buffer(&BufferDescriptor {
        label: None,
        size,
        usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });

    let staging_buffer = device.create_buffer(&BufferDescriptor {
        label: None,
        size,
        usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let bind_group_layout = pipeline.get_bind_group_layout(0);

    let bind_group = device.create_bind_group(&BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout,
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: in1_buffer.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 1,
                resource: in2_buffer.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 2,
                resource: out_buffer.as_entire_binding(),
            },
        ],
    });

    (bind_group, out_buffer, staging_buffer)
}
