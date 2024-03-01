struct Element {
    @location(0) position: vec2<f32>,
    @location(1) size: vec2<f32>
}

@group(0)
@binding(0)
var<storage, read> element_buffer: array<Element>;

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    let element_index = in_vertex_index / 6;
    let v_index = in_vertex_index % 6;
    let pos = element_buffer[element_index].position;
    let size = element_buffer[element_index].size;
    if v_index == 0 || v_index == 3 {
        return vec4<f32>(pos, 0.0, 1.0);
    }
    else if v_index == 1 {
        return vec4<f32>(pos + vec2<f32>(0.0, size.y), 0.0, 1.0);
    }
    else if v_index == 2 || v_index == 5 {
        return vec4<f32>(pos + size, 0.0, 1.0);
    }
    else {
        return vec4<f32>(pos + vec2<f32>(size.x, 0.0), 0.0, 1.0);
    }
}


@fragment
fn fs_main(@builtin(position) in: vec4<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}
 

// struct VertexOutput {
//     @builtin(position) clip_position: vec4<f32>,
// };

// @vertex
// fn vs_main(
//     @builtin(vertex_index) in_vertex_index: u32,
// ) -> VertexOutput {
//     var out: VertexOutput;
//     let x = f32(1 - i32(in_vertex_index)) * 0.5;
//     let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
//     out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
//     return out;
// }

// @fragment
// fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
//     return vec4<f32>(0.3, 0.2, 0.1, 1.0);
// }