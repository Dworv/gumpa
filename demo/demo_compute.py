"""
Example compute shader that does ... nothing but copy a value from one
buffer into another.
"""

import wgpu
from wgpu.utils.compute import compute_with_buffers

shader_source = """

@group(0) @binding(0)
var<storage,read> data1: array<i32>;

@group(0) @binding(1)
var<storage,read_write> data2: array<i32>;

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) index: vec3<u32>) {
    let i: u32 = index.x;
    data2[i] = data1[i];
}
"""

n = 20
data = memoryview(bytearray(n * 4)).cast("i")
for i in range(n):
    data[i] = i

out = compute_with_buffers({0: data}, {1: (n, "i")}, shader_source, n=n)

result = out[1].tolist()
print(result)
assert result == list(range(20))

device = wgpu.utils.get_default_device()

cshader = device.create_shader_module(code=shader_source)

buffer1 = device.create_buffer_with_data(data=data, usage=wgpu.BufferUsage.STORAGE)
buffer2 = device.create_buffer(
    size=data.nbytes, usage=wgpu.BufferUsage.STORAGE | wgpu.BufferUsage.COPY_SRC
)

binding_layouts = [
    {
        "binding": 0,
        "visibility": wgpu.ShaderStage.COMPUTE,
        "buffer": {
            "type": wgpu.BufferBindingType.read_only_storage,
        },
    },
    {
        "binding": 1,
        "visibility": wgpu.ShaderStage.COMPUTE,
        "buffer": {
            "type": wgpu.BufferBindingType.storage,
        },
    },
]
bindings = [
    {
        "binding": 0,
        "resource": {"buffer": buffer1, "offset": 0, "size": buffer1.size},
    },
    {
        "binding": 1,
        "resource": {"buffer": buffer2, "offset": 0, "size": buffer2.size},
    },
]

bind_group_layout = device.create_bind_group_layout(entries=binding_layouts)
pipeline_layout = device.create_pipeline_layout(bind_group_layouts=[bind_group_layout])
bind_group = device.create_bind_group(layout=bind_group_layout, entries=bindings)

compute_pipeline = device.create_compute_pipeline(
    layout=pipeline_layout,
    compute={"module": cshader, "entry_point": "main"},
)
command_encoder = device.create_command_encoder()
compute_pass = command_encoder.begin_compute_pass()
compute_pass.set_pipeline(compute_pipeline)
compute_pass.set_bind_group(0, bind_group, [], 0, 999999)
compute_pass.dispatch_workgroups(n, 1, 1)
compute_pass.end()
device.queue.submit([command_encoder.finish()])

out = device.queue.read_buffer(buffer2).cast("i")
result = out.tolist()
print(result)
assert result == list(range(20))