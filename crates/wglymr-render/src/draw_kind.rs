use wglymr_color::Color;

#[derive(Clone, Debug, PartialEq)]
pub enum DrawKind {
    Line(LineDraw),
    Rect(RectDraw),
    RoundedRect(RoundedRectDraw),
    Circle(CircleDraw),
    Glyph(GlyphDraw),
}

#[derive(Clone, Debug, PartialEq)]
pub struct LineDraw {
    pub start: [f32; 2],
    pub end: [f32; 2],
    pub color: Color,
    pub thickness: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RectDraw {
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub color: Color,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RoundedRectDraw {
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub corner_radius: f32,
    pub color: Color,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CircleDraw {
    pub center: [f32; 2],
    pub radius: f32,
    pub color: Color,
    pub filled: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GlyphDraw {
    pub text: String,
    pub world_position: [f32; 2],
    pub font_size: f32,
    pub color: Color,
}
