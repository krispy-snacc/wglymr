struct ViewportUniform {
    viewport: vec2<f32>,
    _padding: vec2<f32>,
}

@group(0) @binding(0) var<uniform> viewport: ViewportUniform;
@group(1) @binding(0) var glyph_texture: texture_2d<f32>;
@group(1) @binding(1) var glyph_sampler: sampler;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let clip_x = (in.position.x / viewport.viewport.x) * 2.0 - 1.0;
    let clip_y = 1.0 - (in.position.y / viewport.viewport.y) * 2.0;
    out.clip_position = vec4<f32>(clip_x, clip_y, 0.0, 1.0);
    out.uv = in.uv;
    out.color = in.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let sample = textureSample(glyph_texture, glyph_sampler, in.uv);
    let dist = sample.r;
    let w = fwidth(dist) * 0.5;
    let alpha = smoothstep(0.5 - w, 0.5 + w, dist);
    return vec4(in.color.rgb, in.color.a * alpha);
}
