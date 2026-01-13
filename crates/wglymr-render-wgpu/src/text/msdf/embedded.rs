use serde::Deserialize;

pub const ROBOTO_ATLAS: &[u8] = include_bytes!("../../../assets/generated/roboto.msdf.png");

pub const ROBOTO_METRICS: &str = include_str!("../../../assets/generated/roboto.metrics.json");

#[derive(Debug, Clone, Deserialize)]
pub struct AtlasGlyph {
    pub unicode: u32,
    pub advance: f32,
    #[serde(rename = "planeBounds")]
    pub plane_bounds: Option<Bounds>,
    #[serde(rename = "atlasBounds")]
    pub atlas_bounds: Option<Bounds>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Bounds {
    pub left: f32,
    pub bottom: f32,
    pub right: f32,
    pub top: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Atlas {
    #[serde(rename = "atlas")]
    pub metadata: AtlasMetadata,
    pub glyphs: Vec<AtlasGlyph>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AtlasMetadata {
    #[serde(rename = "type")]
    pub atlas_type: String,
    #[serde(rename = "distanceRange")]
    pub distance_range: f32,
    pub size: f32,
    pub width: u32,
    pub height: u32,
    #[serde(rename = "yOrigin")]
    pub y_origin: String,
}

pub fn load_roboto_metrics() -> Result<Atlas, serde_json::Error> {
    serde_json::from_str(ROBOTO_METRICS)
}
