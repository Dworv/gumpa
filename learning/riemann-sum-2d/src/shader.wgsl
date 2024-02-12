@group(0)
@binding(0)
var<storage, read_write> out: array<f32>;

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let x = (f32(global_id.x) + 0.5 ) * 2. / 1000.;
    var sum = 0.;
    for (var y = 0.5 * 2 / 1000; y < 2.; y += 2. / 1000.) {
        sum += (pow(x, 2.) + pow(y, 2.));
    }
    out[global_id.x] = sum * 2. / 1000.;
}
