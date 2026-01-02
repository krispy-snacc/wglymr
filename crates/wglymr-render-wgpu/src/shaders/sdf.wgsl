struct ViewportUniform {
    viewport: vec2<f32>,
    _padding: vec2<f32>,
}

@group(0) @binding(0) var<uniform> viewport: ViewportUniform;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) rect_min: vec2<f32>,
    @location(2) rect_max: vec2<f32>,
    @location(3) radius: f32,
    @location(4) border_width: f32,
    @location(5) fill_color: vec4<f32>,
    @location(6) border_color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) frag_pos: vec2<f32>,
    @location(1) rect_min: vec2<f32>,
    @location(2) rect_max: vec2<f32>,
    @location(3) radius: f32,
    @location(4) border_width: f32,
    @location(5) fill_color: vec4<f32>,
    @location(6) border_color: vec4<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let clip_x = (in.position.x / viewport.viewport.x) * 2.0 - 1.0;
    let clip_y = 1.0 - (in.position.y / viewport.viewport.y) * 2.0;
    out.clip_position = vec4<f32>(clip_x, clip_y, 0.0, 1.0);
    out.frag_pos = in.position;
    out.rect_min = in.rect_min;
    out.rect_max = in.rect_max;
    out.radius = in.radius;
    out.border_width = in.border_width;
    out.fill_color = in.fill_color;
    out.border_color = in.border_color;
    return out;
}

fn sdf_rounded_rect(p: vec2<f32>, b: vec2<f32>, r: f32) -> f32 {
    let q = abs(p) - b + vec2<f32>(r, r);
    return min(max(q.x, q.y), 0.0) + length(max(q, vec2<f32>(0.0, 0.0))) - r;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let center = (in.rect_min + in.rect_max) * 0.5;
    let half_size = (in.rect_max - in.rect_min) * 0.5;
    let p = in.frag_pos - center;
    
    let dist = sdf_rounded_rect(p, half_size, in.radius);
    
    let aa_width = 1.0;
    let alpha_fill = 1.0 - smoothstep(-aa_width, 0.0, dist);
    
    let border_outer = 0.0;
    let border_inner = -in.border_width;
    let alpha_border = smoothstep(border_inner - aa_width, border_inner, dist) - 
                       smoothstep(border_outer - aa_width, border_outer, dist);
    
    let fill = in.fill_color * alpha_fill;
    let border = in.border_color * alpha_border;
    
    let final_color = mix(fill, border, alpha_border);
    
    return final_color;
}
