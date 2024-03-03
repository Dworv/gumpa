struct Element {
    @location(0) position: vec2<f32>,
    @location(1) size: vec2<f32>
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
    let x = floor(size.x * 2 / res.x);
    let y = floor(size.y * 2 / res.y);
    return vec2<f32>(x, y);
}

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    let element_index = in_vertex_index / 6;
    let v_index = in_vertex_index % 6;

    let pos = pos_to_normalized(element_buffer[element_index].position, res_buffer);
    let size = size_to_normalized(element_buffer[element_index].size, res_buffer);

    if v_index == 0 || v_index == 3 {
        return pos;
    }
    else if v_index == 1 {
        let vmod = vec4<f32>(0.0, -size.y, 0.0, 0.0);
        return pos + vmod;
    }
    else if v_index == 2 || v_index == 5 {
        let vhmod = vec4<f32>(size.x, -size.y, 0.0, 0.0);
        return pos + vhmod;
    }
    else {
        let hmod = vec4<f32>(size.x, 0.0, 0.0, 0.0);
        return pos + hmod;
    }
}


@fragment
fn fs_main(@builtin(position) in: vec4<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}