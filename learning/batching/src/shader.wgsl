@group(0)
@binding(0)
var<storage, read> in1: array<u32>;

@group(0)
@binding(1)
var<storage, read> in2: array<u32>;

@group(0)
@binding(2)
var<storage, read_write> out: array<u32>;

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    out[global_id.x] = in1[global_id.x] + in2[global_id.x];
}
