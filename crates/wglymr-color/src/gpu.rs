use crate::Color;

impl Color {
    pub fn to_gpu_linear(self) -> [f32; 4] {
        self.to_rgba_linear()
    }

    pub fn to_gpu_srgb(self) -> [f32; 4] {
        self.to_rgba_srgb()
    }
}
