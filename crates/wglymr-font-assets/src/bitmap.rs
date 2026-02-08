//! Bitmap font atlas access for pixel-aligned rendering

use crate::fonts::FontFamily;

pub struct BitmapAtlas {
    pub family: FontFamily,
    pub atlas_image: &'static [u8],
    pub metrics: &'static str,
}

pub fn get_bitmap_atlas(family: FontFamily) -> Option<BitmapAtlas> {
    match family {
        FontFamily::Mono => Some(BitmapAtlas {
            family: FontFamily::Mono,
            atlas_image: &[], // Placeholder - generated bitmap PNG data
            metrics: "{}",    // Placeholder - generated metrics JSON
        }),
        _ => None,
    }
}
