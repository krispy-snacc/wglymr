use super::cosmic_shaper::ShapedTextRun;
use crate::editor::wgpu_renderer::world_to_screen;
use crate::engine::EditorView;
use crate::runtime::logging;
use wglymr_render_wgpu::SdfTextRenderer;
use wglymr_render_wgpu::text::sdf::GlyphKey;

pub const TEXT_SHADOW: u8 = 3;
pub const TEXT: u8 = 4;

pub fn render_shaped_text(
    text_run: &ShapedTextRun,
    view: &EditorView,
    sdf_renderer: &mut SdfTextRenderer,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    color: [f32; 4],
    layer: u8,
) {
    let zoom = view.zoom();
    let screen_font_size = text_run.world_font_size * zoom;
    let atlas_size = (screen_font_size.ceil() as u32).clamp(24, 72);
    let scale = screen_font_size / atlas_size as f32;

    logging::log(&format!(
        "Rendering {} glyphs, zoom: {}, screen_font_size: {}, atlas_size: {}, scale: {}",
        text_run.glyphs.len(),
        zoom,
        screen_font_size,
        atlas_size,
        scale
    ));

    for shaped_glyph in &text_run.glyphs {
        let [world_x, world_y] = [shaped_glyph.world_x, shaped_glyph.world_y];
        let [screen_x, screen_y] = world_to_screen([world_x, world_y], view);

        let glyph_key = GlyphKey {
            font_id: 0,
            glyph_id: shaped_glyph.glyph_id,
            pixel_size: atlas_size,
        };

        sdf_renderer.draw_glyph(
            device,
            queue,
            glyph_key,
            [screen_x, screen_y],
            scale,
            color,
            layer,
        );
    }
}
