use crate::editor::wgpu_renderer::world_to_screen;
use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, Shaping, SwashCache};
use std::collections::HashMap;
use wglymr_render_wgpu::{GpuGlyph, TextRenderer};

use crate::engine::EditorView;

pub const TEXT_SHADOW: u8 = 3;
pub const TEXT: u8 = 4;

#[derive(Debug, Clone, Copy)]
pub struct TextShadow {
    pub offset_px: [f32; 2],
    pub color: [f32; 4],
    pub layer: u8,
    pub scale: f32,
    pub blur: f32,
}

/// Baseline-positioned glyph identity in world space.
#[derive(Debug, Clone)]
pub struct CosmicGlyph {
    pub font_id: cosmic_text::fontdb::ID,
    pub glyph_id: u16,
    pub world_baseline: [f32; 2],
}

/// Cache key for shaped text buffers.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct BufferKey {
    text: String,
    font_size_bits: u32,
}

/// Manages cosmic-text integration for text shaping and rendering.
pub struct CosmicTextEngine {
    font_system: FontSystem,
    swash_cache: SwashCache,
    buffer_cache: HashMap<BufferKey, Buffer>,
}

impl CosmicTextEngine {
    pub fn new() -> Self {
        let mut font_system = FontSystem::new_with_locale_and_db(
            "en-US".into(),
            cosmic_text::fontdb::Database::new(),
        );

        #[cfg(target_arch = "wasm32")]
        {
            static ROBOTO_REGULAR: &[u8] = include_bytes!("../../../fonts/Roboto-Regular.ttf");
            font_system.db_mut().load_font_data(ROBOTO_REGULAR.to_vec());
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            font_system.db_mut().load_system_fonts();
        }

        Self {
            font_system,
            swash_cache: SwashCache::new(),
            buffer_cache: HashMap::new(),
        }
    }

    /// Shape text at a given world-space font size and position.
    /// Returns list of glyphs with baseline-anchored world positions.
    pub fn shape_text(
        &mut self,
        text: &str,
        world_font_size: f32,
        world_position: [f32; 2],
    ) -> (Vec<CosmicGlyph>, f32) {
        let key = BufferKey {
            text: text.to_string(),
            font_size_bits: world_font_size.to_bits(),
        };

        let buffer = self.buffer_cache.entry(key).or_insert_with(|| {
            let metrics = Metrics::new(world_font_size, world_font_size * 1.2);
            let mut buffer = Buffer::new(&mut self.font_system, metrics);
            buffer.set_size(&mut self.font_system, None, None);
            buffer.set_text(&mut self.font_system, text, Attrs::new(), Shaping::Advanced);
            buffer
        });

        let mut glyphs = Vec::new();
        let [base_x, base_y] = world_position;

        for run in buffer.layout_runs() {
            for glyph in run.glyphs {
                let x = base_x + glyph.x;
                let y = base_y + run.line_y + glyph.y;

                glyphs.push(CosmicGlyph {
                    font_id: glyph.font_id,
                    glyph_id: glyph.glyph_id,
                    world_baseline: [x, y],
                });
            }
        }

        (glyphs, world_font_size)
    }

    /// Render shaped glyphs to screen space with optional shadow.
    /// If shadow is provided, glyphs are rendered twice: shadow first, then main text.
    pub fn render_glyphs(
        &mut self,
        glyphs: &[CosmicGlyph],
        world_font_size: f32,
        view: &EditorView,
        text_renderer: &mut TextRenderer,
        queue: &wgpu::Queue,
        device: &wgpu::Device,
        color: [f32; 4],
        layer: u8,
        shadow: Option<TextShadow>,
    ) {
        let zoom = view.zoom();
        let screen_font_size = world_font_size * zoom;

        // Render shadow pass first (if enabled)
        if let Some(shadow_config) = shadow {
            let shadow_font_size = screen_font_size * shadow_config.scale;

            if shadow_config.blur > 0.0 {
                // Multi-pass blur: render shadow multiple times with offset variations
                let samples = (shadow_config.blur * 2.0).max(1.0) as i32;
                let blur_radius = shadow_config.blur;
                let alpha_per_sample = shadow_config.color[3] / samples as f32;

                for i in 0..samples {
                    let angle = (i as f32 / samples as f32) * std::f32::consts::TAU;
                    let radius_factor = (i as f32 / samples as f32).sqrt();
                    let blur_offset = [
                        shadow_config.offset_px[0] + angle.cos() * blur_radius * radius_factor,
                        shadow_config.offset_px[1] + angle.sin() * blur_radius * radius_factor,
                    ];
                    let blur_color = [
                        shadow_config.color[0],
                        shadow_config.color[1],
                        shadow_config.color[2],
                        alpha_per_sample,
                    ];

                    self.render_pass(
                        glyphs,
                        shadow_font_size,
                        view,
                        text_renderer,
                        queue,
                        device,
                        blur_color,
                        shadow_config.layer,
                        Some(blur_offset),
                    );
                }
            } else {
                // No blur: single shadow pass
                self.render_pass(
                    glyphs,
                    shadow_font_size,
                    view,
                    text_renderer,
                    queue,
                    device,
                    shadow_config.color,
                    shadow_config.layer,
                    Some(shadow_config.offset_px),
                );
            }
        }

        // Render main text pass
        self.render_pass(
            glyphs,
            screen_font_size,
            view,
            text_renderer,
            queue,
            device,
            color,
            layer,
            None,
        );
    }

    fn render_pass(
        &mut self,
        glyphs: &[CosmicGlyph],
        screen_font_size: f32,
        view: &EditorView,
        text_renderer: &mut TextRenderer,
        queue: &wgpu::Queue,
        device: &wgpu::Device,
        color: [f32; 4],
        layer: u8,
        offset: Option<[f32; 2]>,
    ) {
        for glyph in glyphs {
            let [screen_baseline_x, screen_baseline_y] =
                world_to_screen(glyph.world_baseline, view);

            let [offset_x, offset_y] = offset.unwrap_or([0.0, 0.0]);
            let final_x = screen_baseline_x + offset_x;
            let final_y = screen_baseline_y + offset_y;

            let cache_key = cosmic_text::CacheKey {
                font_id: glyph.font_id,
                glyph_id: glyph.glyph_id,
                font_size_bits: screen_font_size.to_bits(),
                x_bin: cosmic_text::SubpixelBin::Zero,
                y_bin: cosmic_text::SubpixelBin::Zero,
                flags: cosmic_text::CacheKeyFlags::empty(),
            };

            let image = self
                .swash_cache
                .get_image_uncached(&mut self.font_system, cache_key);

            if let Some(image) = image {
                let glyph_key = wglymr_render_wgpu::GlyphKey {
                    id: glyph.glyph_id as u32,
                    size_px: screen_font_size as u16,
                };

                if !text_renderer.atlas().contains(&glyph_key) {
                    text_renderer.atlas_mut().insert(
                        queue,
                        glyph_key,
                        image.placement.width as u16,
                        image.placement.height as u16,
                        &image.data,
                    );

                    text_renderer.rebuild_texture_bind_group(device);
                }

                if let Some(entry) = text_renderer.atlas().get(&glyph_key) {
                    let quad_x = (final_x + image.placement.left as f32).round();
                    let quad_y = (final_y - image.placement.top as f32).round();
                    let quad_width = image.placement.width as f32;
                    let quad_height = image.placement.height as f32;

                    text_renderer.draw_glyph(GpuGlyph {
                        screen_pos: [quad_x, quad_y],
                        size: [quad_width, quad_height],
                        uv_min: entry.uv_min,
                        uv_max: entry.uv_max,
                        color,
                        layer,
                    });
                }
            }
        }
    }

    pub fn clear_cache(&mut self) {
        self.buffer_cache.clear();
    }
}

impl Default for CosmicTextEngine {
    fn default() -> Self {
        Self::new()
    }
}
