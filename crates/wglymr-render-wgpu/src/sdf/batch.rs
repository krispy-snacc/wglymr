use super::{RoundedRect, SdfVertex};

pub struct SdfBatch {
    pub(crate) vertices: Vec<SdfVertex>,
    current_layer: u8,
}

impl Default for SdfBatch {
    fn default() -> Self {
        Self::new()
    }
}

impl SdfBatch {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            current_layer: 0,
        }
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
        self.current_layer = 0;
    }

    pub fn set_layer(&mut self, layer: u8) {
        self.current_layer = layer;
    }

    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }

    pub fn vertices(&self) -> &[SdfVertex] {
        &self.vertices
    }

    pub fn draw_rounded_rect(&mut self, rect: &RoundedRect) {
        let min = rect.min;
        let max = rect.max;

        let vertices = [
            SdfVertex {
                position: [min[0], min[1]],
                rect_min: rect.min,
                rect_max: rect.max,
                radius: rect.radius,
                border_width: rect.border_width,
                fill_color: rect.fill_color,
                border_color: rect.border_color,
            },
            SdfVertex {
                position: [max[0], min[1]],
                rect_min: rect.min,
                rect_max: rect.max,
                radius: rect.radius,
                border_width: rect.border_width,
                fill_color: rect.fill_color,
                border_color: rect.border_color,
            },
            SdfVertex {
                position: [max[0], max[1]],
                rect_min: rect.min,
                rect_max: rect.max,
                radius: rect.radius,
                border_width: rect.border_width,
                fill_color: rect.fill_color,
                border_color: rect.border_color,
            },
            SdfVertex {
                position: [min[0], min[1]],
                rect_min: rect.min,
                rect_max: rect.max,
                radius: rect.radius,
                border_width: rect.border_width,
                fill_color: rect.fill_color,
                border_color: rect.border_color,
            },
            SdfVertex {
                position: [max[0], max[1]],
                rect_min: rect.min,
                rect_max: rect.max,
                radius: rect.radius,
                border_width: rect.border_width,
                fill_color: rect.fill_color,
                border_color: rect.border_color,
            },
            SdfVertex {
                position: [min[0], max[1]],
                rect_min: rect.min,
                rect_max: rect.max,
                radius: rect.radius,
                border_width: rect.border_width,
                fill_color: rect.fill_color,
                border_color: rect.border_color,
            },
        ];

        self.vertices.extend_from_slice(&vertices);
    }
}
