struct Camera {
    pan: vec2<f32>,
    zoom: f32,
    viewport: vec2<f32>,
}

@group(0) @binding(0) var<uniform> camera: Camera;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let pixel_pos = (in.position - camera.pan) * camera.zoom;
    let clip_x = (pixel_pos.x / camera.viewport.x) * 2.0 - 1.0;
    let clip_y = 1.0 - (pixel_pos.y / camera.viewport.y) * 2.0;
    out.clip_position = vec4<f32>(clip_x, clip_y, 0.0, 1.0);
    out.color = in.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
