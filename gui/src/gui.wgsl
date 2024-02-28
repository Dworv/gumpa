struct Element {
    @location(0) position: vec2<f32>,
    @location(1) size: vec2<f32>
}

@group(0)
@binding(0)
var<storage, read> elements: array<Element>;

@group(0)
@binding(1)
var<storage, read_write> vertexes: vec4<f32>;

@compute
@workgroup_size(1)
fn cs_main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let element = elements[global_id.x];
    let pos = element.position;
    let size = element.size;
    let tl = vec4<f32>(pos.x, pos.y, 0.0, 1.0);
    let tr = vec4<f32>(pos.x + size.x, pos.y, 0.0, 1.0);
    let bl = vec4<f32>(pos.x, pos.y + size.y, 0.0, 1.0);
    let br = vec4<f32>(pos.x + size.x, pos.y + size.y, 0.0, 1.0);
    vertexes[global_id.x] = tl;
    vertexes[global_id.x + 1] = br;
    vertexes[global_id.x + 2] = bl;
    vertexes[global_id.x + 3] = tl;
    vertexes[global_id.x + 4] = tr;
    vertexes[global_id.x + 5] = br;
}

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32
) -> @builtin(position) vec4<f32> {
    return vertexes[in_vertex_index];
}

@fragment
fn fs_main(in: vec4<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}
 