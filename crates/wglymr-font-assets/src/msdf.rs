//! MSDF (Multi-channel Signed Distance Field) font atlas access

use crate::fonts::FontFamily;

pub struct MsdfAtlas {
    pub family: FontFamily,
    pub atlas_image: &'static [u8],
    pub metrics: &'static str,
}

pub fn get_msdf_atlas(family: FontFamily) -> Option<MsdfAtlas> {
    match family {
        FontFamily::Roboto => Some(MsdfAtlas {
            family: FontFamily::Roboto,
            atlas_image: &[], // Placeholder - generated atlas PNG data
            metrics: "{}",    // Placeholder - generated metrics JSON
        }),
        FontFamily::Inter => Some(MsdfAtlas {
            family: FontFamily::Inter,
            atlas_image: &[],
            metrics: "{}",
        }),
        FontFamily::Mono => Some(MsdfAtlas {
            family: FontFamily::Mono,
            atlas_image: &[],
            metrics: "{}",
        }),
    }
}
