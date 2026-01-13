use super::glyph::{CachedGlyph, GlyphKey};
use std::collections::HashMap;

/// CPU-side glyph cache
pub struct GlyphCache {
    glyphs: HashMap<GlyphKey, CachedGlyph>,
}

impl GlyphCache {
    pub fn new() -> Self {
        Self {
            glyphs: HashMap::new(),
        }
    }

    pub fn get(&self, key: &GlyphKey) -> Option<&CachedGlyph> {
        self.glyphs.get(key)
    }

    pub fn insert(&mut self, glyph: CachedGlyph) {
        self.glyphs.insert(glyph.key, glyph);
    }

    pub fn clear(&mut self) {
        self.glyphs.clear();
    }

    pub fn len(&self) -> usize {
        self.glyphs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.glyphs.is_empty()
    }
}

impl Default for GlyphCache {
    fn default() -> Self {
        Self::new()
    }
}
