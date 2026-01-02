use super::layout::FontMetrics;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextStyle {
    pub font_size: f32,
    pub color: [f32; 4],
    pub line_height: Option<f32>,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            font_size: 14.0,
            color: [1.0, 1.0, 1.0, 1.0],
            line_height: None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TextBounds {
    pub min: [f32; 2],
    pub max: [f32; 2],
}

impl TextBounds {
    pub fn new(min: [f32; 2], max: [f32; 2]) -> Self {
        Self { min, max }
    }

    pub fn zero() -> Self {
        Self {
            min: [0.0, 0.0],
            max: [0.0, 0.0],
        }
    }

    pub fn width(&self) -> f32 {
        self.max[0] - self.min[0]
    }

    pub fn height(&self) -> f32 {
        self.max[1] - self.min[1]
    }

    pub fn offset(&self, offset: [f32; 2]) -> Self {
        Self {
            min: [self.min[0] + offset[0], self.min[1] + offset[1]],
            max: [self.max[0] + offset[0], self.max[1] + offset[1]],
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ShapedGlyph {
    pub glyph_id: u32,
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub uv_min: [f32; 2],
    pub uv_max: [f32; 2],
}

#[derive(Debug, Clone)]
pub struct RenderText {
    pub glyphs: Vec<ShapedGlyph>,
    pub bounds: TextBounds,
    pub style: TextStyle,
    pub font_metrics: FontMetrics,
}

impl RenderText {
    pub fn new(glyphs: Vec<ShapedGlyph>, bounds: TextBounds, style: TextStyle, font_metrics: FontMetrics) -> Self {
        Self {
            glyphs,
            bounds,
            style,
            font_metrics,
        }
    }

    pub fn empty(style: TextStyle) -> Self {
        Self {
            glyphs: Vec::new(),
            bounds: TextBounds::zero(),
            style,
            font_metrics: FontMetrics::default(),
        }
    }

    pub fn offset(&self, offset: [f32; 2]) -> Self {
        let glyphs = self
            .glyphs
            .iter()
            .map(|g| ShapedGlyph {
                position: [g.position[0] + offset[0], g.position[1] + offset[1]],
                ..*g
            })
            .collect();

        Self {
            glyphs,
            bounds: self.bounds.offset(offset),
            style: self.style,
            font_metrics: self.font_metrics,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.glyphs.is_empty()
    }

    pub fn glyph_count(&self) -> usize {
        self.glyphs.len()
    }
}
