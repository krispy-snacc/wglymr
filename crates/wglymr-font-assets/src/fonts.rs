//! Raw font file access

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontFamily {
    Roboto,
    Inter,
    Mono,
}

pub struct FontAsset {
    pub family: FontFamily,
    pub data: &'static [u8],
}

pub fn get_font_asset(family: FontFamily) -> Option<FontAsset> {
    match family {
        FontFamily::Roboto => Some(FontAsset {
            family: FontFamily::Roboto,
            data: &[], // Placeholder - will be populated with actual font data
        }),
        FontFamily::Inter => Some(FontAsset {
            family: FontFamily::Inter,
            data: &[],
        }),
        FontFamily::Mono => Some(FontAsset {
            family: FontFamily::Mono,
            data: &[],
        }),
    }
}
