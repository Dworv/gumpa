struct Element {
    @location(0) position: vec2<f32>,
    @location(1) size: vec2<f32>,
    @location(2) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) nrv: f32 // Normalized relative vertex 
}

@group(0)
@binding(0)
var<storage, read> element_buffer: array<Element>;

@group(0)
@binding(1)
var<storage, read> res_buffer: vec2<f32>;

fn pos_to_normalized(pos: vec2<f32>, res: vec2<f32>) -> vec4<f32> {
    let x = (pos.x * 2 / res.x) - 1;
    let y = -((pos.y * 2 / res.y) - 1);
    return vec4<f32>(x, y, 0.0, 1.0);
}

fn size_to_normalized(size: vec2<f32>, res: vec2<f32>) -> vec2<f32> {
    let x = size.x * 2 / res.x;
    let y = size.y * 2 / res.y;
    return vec2<f32>(x, y);
}

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    let element_index = in_vertex_index / 6;
    let v_index = in_vertex_index % 6;

    var pos = pos_to_normalized(element_buffer[element_index].position, res_buffer);
    let size = size_to_normalized(element_buffer[element_index].size, res_buffer);

    if v_index == 1 {
        let vmod = vec4<f32>(0.0, -size.y, 0.0, 0.0);
        pos += vmod;
    }
    else if v_index == 2 || v_index == 5 {
        let vhmod = vec4<f32>(size.x, -size.y, 0.0, 0.0);
        pos += vhmod;
    }
    else if v_index == 4 {
        let hmod = vec4<f32>(size.x, 0.0, 0.0, 0.0);
        pos += hmod;
    }

    var nrv = 0.0;
    if v_index == 2 {
        nrv = 1.0;
    }

    var out: VertexOutput;
    out.clip_position = pos;
    out.color = element_buffer[element_index].color;
    out.nrv = nrv;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    if in.nrv > 0.7 {
        return vec4<f32>(1.0, 0.0, 0.0, 1.0);
    }
    return in.color;
}
