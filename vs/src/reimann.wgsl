@group(0) @binding(0) var<storage, read> start: f32;
@group(0) @binding(1) var<storage, read_write> sum: array<f32>;

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    var local_sum = 0.;
    let local_start = start + f32(global_id.x) / 1000. * 10.;

    for (var i=0;i<1000;i++) {
        let num = local_start + f32(i) / 1000000. * 10.;
        local_sum += pow(num, 2.);
    }

    sum[global_id.x] = local_sum;
}