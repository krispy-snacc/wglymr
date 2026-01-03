struct ViewportUniform {
    viewport: vec2<f32>,
    _padding: vec2<f32>,
}

@group(0) @binding(0) var<uniform> viewport: ViewportUniform;
@group(1) @binding(0) var msdf_texture: texture_2d<f32>;
@group(1) @binding(1) var msdf_sampler: sampler;

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

fn median(r: f32, g: f32, b: f32) -> f32 {
    return max(min(r, g), min(max(r, g), b));
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let msd = textureSample(msdf_texture, msdf_sampler, in.uv).rgb;
    let sd = median(msd.r, msd.g, msd.b);
    
    let pxRange = 4.0;
    let unitRange = pxRange / vec2<f32>(textureDimensions(msdf_texture).xy);
    let screenTexSize = vec2<f32>(1.0) / fwidth(in.uv);
    let screenPxRange = max(0.5 * dot(unitRange, screenTexSize), 1.0);
    
    let screenPxDistance = screenPxRange * (sd - 0.5);
    let alpha = clamp(screenPxDistance + 0.5, 0.0, 1.0);
    
    return vec4<f32>(in.color.rgb, in.color.a * alpha);
}
