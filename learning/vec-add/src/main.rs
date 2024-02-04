use std::time::Instant;

use wgpu::{
    include_wgsl, util::{BufferInitDescriptor, DeviceExt}, BindGroupDescriptor, BindGroupEntry, BufferAddress, BufferAsyncError, BufferDescriptor, BufferUsages, ComputePipelineDescriptor, DeviceDescriptor, Features, Instance, Limits, RequestAdapterOptions, ShaderModuleDescriptor
};

#[pollster::main]
async fn main() {
    // instantiate
    let mut gap = Instant::now();
    let instance = Instance::default();
    print_time_gap("instantiating wgpu", &mut gap);

    let adapter = instance
        .request_adapter(&RequestAdapterOptions::default())
        .await
        .unwrap();
    print_time_gap("getting adapter", &mut gap);

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
    print_time_gap("getting device", &mut gap);

    // prep
    let input_1 = [12;65535];
    let input_2 = [13;65535];
    reset_time_gap(&mut gap);

    let shader_module = device.create_shader_module(ShaderModuleDescriptor {
        label: None,
        source: include_wgsl!("shader.wgsl").source,
    });
    print_time_gap("making shader module", &mut gap);

    let size = std::mem::size_of_val(&input_1) as BufferAddress;
    reset_time_gap(&mut gap);

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
    print_time_gap("making two input buffers", &mut gap);

    let out_buffer = device.create_buffer(&BufferDescriptor {
        label: None,
        size,
        usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    print_time_gap("making output buffer", &mut gap);

    let staging_buffer = device.create_buffer(&BufferDescriptor {
        label: None,
        size,
        usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    print_time_gap("making staging buffer", &mut gap);

    let compute_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
        label: None,
        layout: None,
        module: &shader_module,
        entry_point: "main",
    });
    print_time_gap("making compute pipeline", &mut gap);

    let bind_group_layout = compute_pipeline.get_bind_group_layout(0);
    print_time_gap("getting bind group layout", &mut gap);

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
    print_time_gap("making bind group", &mut gap);

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    print_time_gap("making encoder", &mut gap);

    {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes: None });
        print_time_gap("beginning compute pass", &mut gap);
        cpass.set_pipeline(&compute_pipeline);
        cpass.set_bind_group(0, &bind_group, &[]);
        cpass.insert_debug_marker("bonjour!");
        print_time_gap("configuring compute pass", &mut gap);
        cpass.dispatch_workgroups(input_1.len() as u32, 1, 1);
        print_time_gap("dispatching compute pass", &mut gap);
    }

    encoder.copy_buffer_to_buffer(&out_buffer, 0, &staging_buffer, 0, size);
    print_time_gap("copying buffur to buffer", &mut gap);

    queue.submit(Some(encoder.finish()));
    print_time_gap("submitting queue", &mut gap);

    let buffer_slice = staging_buffer.slice(..);
    let (sender, reciever) = std::sync::mpsc::channel::<Result<(), BufferAsyncError>>();
    reset_time_gap(&mut gap);

    buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
    print_time_gap("async mapping staging buffer", &mut gap);

    device.poll(wgpu::Maintain::wait()).panic_on_timeout();
    print_time_gap("polling device", &mut gap);

    let _ = reciever.recv().unwrap();
    print_time_gap("waiting for map", &mut gap);
    let data = buffer_slice.get_mapped_range();
    print_time_gap("mapping", &mut gap);
    let result: Vec<u32> = bytemuck::cast_slice(&data).to_vec();
    print_time_gap("casting", &mut gap);

    drop(data);
    staging_buffer.unmap();

    // println!("result: {:?}", result);
    println!("len: {}, first: {}", result.len(), result[0])
}

fn print_time_gap(activity: &str, gap: &mut Instant) {
    println!("Spent {:>8.2}ms {}", gap.elapsed().as_secs_f64() * 1000., activity);
    *gap = Instant::now();
}

fn reset_time_gap(gap: &mut Instant) {
    *gap = Instant::now();
}
