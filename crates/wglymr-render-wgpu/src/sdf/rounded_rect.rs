pub struct RoundedRect {
    pub min: [f32; 2],
    pub max: [f32; 2],
    pub radius: f32,
    pub border_width: f32,
    pub fill_color: [f32; 4],
    pub border_color: [f32; 4],
}

impl RoundedRect {
    pub fn new(min: [f32; 2], max: [f32; 2]) -> Self {
        Self {
            min,
            max,
            radius: 0.0,
            border_width: 0.0,
            fill_color: [1.0, 1.0, 1.0, 1.0],
            border_color: [0.0, 0.0, 0.0, 1.0],
        }
    }

    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    pub fn with_border(mut self, width: f32, color: [f32; 4]) -> Self {
        self.border_width = width;
        self.border_color = color;
        self
    }

    pub fn with_fill_color(mut self, color: [f32; 4]) -> Self {
        self.fill_color = color;
        self
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SdfVertex {
    pub position: [f32; 2],
    pub rect_min: [f32; 2],
    pub rect_max: [f32; 2],
    pub radius: f32,
    pub border_width: f32,
    pub fill_color: [f32; 4],
    pub border_color: [f32; 4],
}
