use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, Shaping};
use std::collections::HashMap;

/// World-space shaped glyph
#[derive(Debug, Clone, Copy)]
pub struct ShapedGlyph {
    pub glyph_id: u16,
    pub unicode: u32,
    pub world_x: f32,
    pub world_y: f32,
}

/// Shaped text run in world space
#[derive(Debug, Clone)]
pub struct ShapedTextRun {
    pub glyphs: Vec<ShapedGlyph>,
    pub world_font_size: f32,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct BufferKey {
    text: String,
    font_size_bits: u32,
}

/// Text shaper using cosmic-text
/// Performs ONLY shaping and layout - NO rendering
pub struct CosmicShaper {
    font_system: FontSystem,
    buffer_cache: HashMap<BufferKey, Buffer>,
}

impl CosmicShaper {
    pub fn new() -> Self {
        Self::with_font_data(None)
    }

    pub fn with_font_data(font_data: Option<&[u8]>) -> Self {
        let mut font_system = FontSystem::new_with_locale_and_db(
            "en-US".into(),
            cosmic_text::fontdb::Database::new(),
        );

        if let Some(data) = font_data {
            font_system.db_mut().load_font_data(data.to_vec());
        } else {
            #[cfg(target_arch = "wasm32")]
            {
                static ROBOTO_REGULAR: &[u8] =
                    include_bytes!("../../../../../fonts/DejaVuSans.ttf");
                font_system.db_mut().load_font_data(ROBOTO_REGULAR.to_vec());
            }

            #[cfg(not(target_arch = "wasm32"))]
            {
                font_system.db_mut().load_system_fonts();
            }
        }

        Self {
            font_system,
            buffer_cache: HashMap::new(),
        }
    }

    /// Shape text at world-space position and font size
    /// Returns glyphs with world-space baseline coordinates
    pub fn shape_text(
        &mut self,
        text: &str,
        world_font_size: f32,
        world_position: [f32; 2],
    ) -> ShapedTextRun {
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
                let text_chars: Vec<char> = text.chars().collect();
                let unicode = if glyph.start < text_chars.len() {
                    text_chars[glyph.start] as u32
                } else {
                    32
                };

                glyphs.push(ShapedGlyph {
                    glyph_id: glyph.glyph_id as u16,
                    unicode,
                    world_x: base_x + glyph.x,
                    world_y: base_y + run.line_y + glyph.y,
                });
            }
        }

        ShapedTextRun {
            glyphs,
            world_font_size,
        }
    }

    pub fn clear_cache(&mut self) {
        self.buffer_cache.clear();
    }
}

impl Default for CosmicShaper {
    fn default() -> Self {
        Self::new()
    }
}
