use wgpu::{
    include_wgsl, util::{BufferInitDescriptor, DeviceExt}, BindGroupDescriptor, BindGroupEntry, BufferAddress, BufferAsyncError, BufferDescriptor, BufferUsages, ComputePipelineDescriptor, DeviceDescriptor, Features, Instance, Limits, RequestAdapterOptions, ShaderModuleDescriptor
};

#[pollster::main]
async fn main() {
    // instantiate

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

    // prep
    let input_1 = [1, 2, 3, 4, 5];
    let input_2 = [3, 3, 3, 3, 10];

    let shader_module = device.create_shader_module(ShaderModuleDescriptor {
        label: None,
        source: include_wgsl!("shader.wgsl").source,
    });

    let size = std::mem::size_of_val(&input_1) as BufferAddress;

    let in1_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(input_1.as_slice()),
        usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
    });

    let in2_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(input_2.as_slice()),
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

    let compute_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
        label: None,
        layout: None,
        module: &shader_module,
        entry_point: "main",
    });

    let bind_group_layout = compute_pipeline.get_bind_group_layout(0);
    let bind_group = device.create_bind_group(&BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout,
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: in1_buffer.as_entire_binding()
            },
            BindGroupEntry {
                binding: 1,
                resource: in2_buffer.as_entire_binding()
            },
            BindGroupEntry {
                binding: 2,
                resource: out_buffer.as_entire_binding()
            }
        ],
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes: None });
        cpass.set_pipeline(&compute_pipeline);
        cpass.set_bind_group(0, &bind_group, &[]);
        cpass.insert_debug_marker("bonjour!");
        cpass.dispatch_workgroups(input_1.len() as u32, 1, 1)
    }

    encoder.copy_buffer_to_buffer(&out_buffer, 0, &staging_buffer, 0, size);

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

    println!("result: {:?}", result);
}
