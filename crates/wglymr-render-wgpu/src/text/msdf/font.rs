use super::atlas::MSDFAtlas;

/// Single font face with MSDF atlas
pub struct FontFace {
    pub(crate) atlas: MSDFAtlas,
    pub(crate) name: String,
}

impl FontFace {
    pub fn from_msdf_atlas(name: String, atlas: MSDFAtlas) -> Self {
        Self { atlas, name }
    }

    pub fn atlas(&self) -> &MSDFAtlas {
        &self.atlas
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
