use wglymr_color::Color;

#[derive(Debug, Clone)]
pub struct TextStyle {
    pub font_size: f32,
    pub color: Color,
}

impl TextStyle {
    pub fn node_title() -> Self {
        Self {
            font_size: 14.0,
            color: Color::WHITE,
        }
    }
}
