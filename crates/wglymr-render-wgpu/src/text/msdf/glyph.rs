/// Unique identifier for a glyph in the cache
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GlyphKey {
    pub font_id: u32,
    pub glyph_id: u16,
    pub pixel_size: u16,
}

impl GlyphKey {
    pub fn new(font_id: u32, glyph_id: u16, pixel_size: u16) -> Self {
        Self {
            font_id,
            glyph_id,
            pixel_size,
        }
    }
}

/// UV coordinates in the atlas texture
#[derive(Debug, Clone, Copy)]
pub struct GlyphUv {
    pub min: [f32; 2],
    pub max: [f32; 2],
}

/// Metrics for positioning a glyph
#[derive(Debug, Clone, Copy)]
pub struct GlyphMetrics {
    pub advance_x: f32,
    pub bearing_x: f32,
    pub bearing_y: f32,
    pub width: f32,
    pub height: f32,
}

/// Cached glyph data with atlas location
#[derive(Debug, Clone, Copy)]
pub struct CachedGlyph {
    pub key: GlyphKey,
    pub uv: GlyphUv,
    pub metrics: GlyphMetrics,
    pub atlas_x: u32,
    pub atlas_y: u32,
    pub atlas_width: u32,
    pub atlas_height: u32,
    pub atlas_height_f32: f32,
}
