use super::embedded::{self, AtlasGlyph};
use super::glyph::{CachedGlyph, GlyphKey, GlyphMetrics, GlyphUv};
use std::collections::HashMap;

pub struct MsdfAtlas {
    width: u32,
    height: u32,
    texture_data: Vec<u8>,
    glyph_map: HashMap<u32, AtlasGlyph>,
    distance_range: f32,
    _font_size: f32,
}

impl MsdfAtlas {
    pub fn new() -> Self {
        let metrics = embedded::load_roboto_metrics().expect("Failed to load embedded metrics");

        let atlas_image = image::load_from_memory(embedded::ROBOTO_ATLAS)
            .expect("Failed to load embedded atlas image");

        let rgba_image = atlas_image.to_rgba8();
        let texture_data = rgba_image.into_raw();

        let mut glyph_map = HashMap::new();
        for glyph in metrics.glyphs {
            glyph_map.insert(glyph.unicode, glyph);
        }

        Self {
            width: metrics.metadata.width,
            height: metrics.metadata.height,
            texture_data,
            glyph_map,
            distance_range: metrics.metadata.distance_range,
            _font_size: metrics.metadata.size,
        }
    }

    pub fn get_glyph(&self, key: GlyphKey) -> Option<CachedGlyph> {
        let unicode = key.glyph_id as u32;
        let glyph = self.glyph_map.get(&unicode)?;

        let plane_bounds = glyph.plane_bounds.as_ref()?;
        let atlas_bounds = glyph.atlas_bounds.as_ref()?;

        let scale = key.pixel_size as f32;

        let width = (plane_bounds.right - plane_bounds.left) * scale;
        let height = (plane_bounds.top - plane_bounds.bottom) * scale;

        let uv = GlyphUv {
            min: [
                atlas_bounds.left / self.width as f32,
                1.0 - (atlas_bounds.top / self.height as f32),
            ],
            max: [
                atlas_bounds.right / self.width as f32,
                1.0 - (atlas_bounds.bottom / self.height as f32),
            ],
        };

        let metrics = GlyphMetrics {
            advance_x: glyph.advance * scale,
            bearing_x: plane_bounds.left * scale,
            bearing_y: (plane_bounds.top - 0.8) * scale,
            width,
            height,
        };

        Some(CachedGlyph {
            key,
            uv,
            metrics,
            atlas_x: atlas_bounds.left as u32,
            atlas_y: atlas_bounds.bottom as u32,
            atlas_width: (atlas_bounds.right - atlas_bounds.left) as u32,
            atlas_height: (atlas_bounds.top - atlas_bounds.bottom) as u32,
        })
    }

    pub fn texture_data(&self) -> &[u8] {
        &self.texture_data
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn distance_range(&self) -> f32 {
        self.distance_range
    }
}

impl Default for MsdfAtlas {
    fn default() -> Self {
        Self::new()
    }
}
