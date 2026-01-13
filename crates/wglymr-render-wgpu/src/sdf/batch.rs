use super::{RoundedRect, SdfVertex};

pub struct SdfBatch {
    pub(crate) vertices: Vec<SdfVertex>,
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
        }
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
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
        let fill_color_gpu = rect.fill_color.to_gpu_linear();
        let border_color_gpu = rect.border_color.to_gpu_linear();

        let vertices = [
            SdfVertex {
                position: [min[0], min[1]],
                rect_min: rect.min,
                rect_max: rect.max,
                radius: rect.radius,
                border_width: rect.border_width,
                fill_color: fill_color_gpu,
                border_color: border_color_gpu,
                depth: rect.depth,
            },
            SdfVertex {
                position: [max[0], min[1]],
                rect_min: rect.min,
                rect_max: rect.max,
                radius: rect.radius,
                border_width: rect.border_width,
                fill_color: fill_color_gpu,
                border_color: border_color_gpu,
                depth: rect.depth,
            },
            SdfVertex {
                position: [max[0], max[1]],
                rect_min: rect.min,
                rect_max: rect.max,
                radius: rect.radius,
                border_width: rect.border_width,
                fill_color: fill_color_gpu,
                border_color: border_color_gpu,
                depth: rect.depth,
            },
            SdfVertex {
                position: [min[0], min[1]],
                rect_min: rect.min,
                rect_max: rect.max,
                radius: rect.radius,
                border_width: rect.border_width,
                fill_color: fill_color_gpu,
                border_color: border_color_gpu,
                depth: rect.depth,
            },
            SdfVertex {
                position: [max[0], max[1]],
                rect_min: rect.min,
                rect_max: rect.max,
                radius: rect.radius,
                border_width: rect.border_width,
                fill_color: fill_color_gpu,
                border_color: border_color_gpu,
                depth: rect.depth,
            },
            SdfVertex {
                position: [min[0], max[1]],
                rect_min: rect.min,
                rect_max: rect.max,
                radius: rect.radius,
                border_width: rect.border_width,
                fill_color: fill_color_gpu,
                border_color: border_color_gpu,
                depth: rect.depth,
            },
        ];

        self.vertices.extend_from_slice(&vertices);
    }
}
