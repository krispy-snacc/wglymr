use wglymr_color::Color;
use wglymr_document::NodeId;

#[derive(Debug, Clone, PartialEq)]
pub struct GlyphRun {
    pub text: String,
    pub world_position: [f32; 2],
    pub font_size: f32,
    pub color: Color,
    pub z_index: i32,
    pub node_id: Option<NodeId>,
}
