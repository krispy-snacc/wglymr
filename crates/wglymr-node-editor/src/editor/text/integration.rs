use super::cosmic_shaper::ShapedTextRun;
use crate::editor::wgpu_renderer::world_to_screen;
use crate::engine::EditorView;
use crate::runtime::logging;
use wglymr_render_wgpu::msdf::{MSDFGlyph, MSDFTextRenderer};

pub const TEXT_SHADOW: u8 = 3;
pub const TEXT: u8 = 4;

/// Convert world-space shaped glyphs to screen-space MSDF glyphs
pub fn render_shaped_text(
    text_run: &ShapedTextRun,
    view: &EditorView,
    msdf_renderer: &mut MSDFTextRenderer,
    color: [f32; 4],
    layer: u8,
) {
    let zoom = view.zoom();
    let screen_font_size = text_run.world_font_size * zoom;

    let (em_size, atlas_width, atlas_height) = {
        let font = match msdf_renderer.font() {
            Some(f) => f,
            None => {
                logging::log("MSDF renderer has no font set!");
                return;
            }
        };

        let atlas = font.atlas();
        let metrics = atlas.metrics();
        (
            metrics.em_size,
            metrics.atlas_width as f32,
            metrics.atlas_height as f32,
        )
    };

    let scale = screen_font_size / em_size;

    logging::log(&format!(
        "Rendering {} glyphs, zoom: {}, screen_font_size: {}, scale: {}",
        text_run.glyphs.len(),
        zoom,
        screen_font_size,
        scale
    ));

    for shaped_glyph in &text_run.glyphs {
        let [world_x, world_y] = [shaped_glyph.world_x, shaped_glyph.world_y];
        let [screen_x, screen_y] = world_to_screen([world_x, world_y], view);

        let font = match msdf_renderer.font() {
            Some(f) => f,
            None => return,
        };

        let atlas = font.atlas();

        if let Some(glyph_metrics) = atlas.get_glyph(shaped_glyph.unicode) {
            let uv_min = [
                glyph_metrics.atlas_bounds_left as f32 / atlas_width,
                1.0 - (glyph_metrics.atlas_bounds_top as f32 / atlas_height),
            ];
            let uv_max = [
                glyph_metrics.atlas_bounds_right as f32 / atlas_width,
                1.0 - (glyph_metrics.atlas_bounds_bottom as f32 / atlas_height),
            ];

            crate::runtime::logging::log(&format!(
                "Glyph '{}' (U+{:04X}): atlas_bounds L:{} R:{} T:{} B:{}, uv_min:{:?} uv_max:{:?}",
                char::from_u32(shaped_glyph.unicode).unwrap_or('?'),
                shaped_glyph.unicode,
                glyph_metrics.atlas_bounds_left,
                glyph_metrics.atlas_bounds_right,
                glyph_metrics.atlas_bounds_top,
                glyph_metrics.atlas_bounds_bottom,
                uv_min,
                uv_max
            ));

            let plane_width = glyph_metrics.plane_bounds_right - glyph_metrics.plane_bounds_left;
            let plane_height = glyph_metrics.plane_bounds_top - glyph_metrics.plane_bounds_bottom;

            let screen_width = plane_width * scale;
            let screen_height = plane_height * scale;

            let screen_pos_x = screen_x + glyph_metrics.plane_bounds_left * scale;
            let screen_pos_y = screen_y - glyph_metrics.plane_bounds_top * scale;

            msdf_renderer.draw_glyph(MSDFGlyph {
                screen_pos: [screen_pos_x.round(), screen_pos_y.round()],
                screen_size: [screen_width, screen_height],
                uv_min,
                uv_max,
                color,
                layer,
            });
        } else {
            logging::log(&format!(
                "No glyph metrics for unicode: {}",
                shaped_glyph.unicode
            ));
        }
    }
}
