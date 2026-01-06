use palette::{FromColor, Srgba};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    rgba: Srgba<f32>,
}

impl Color {
    const fn const_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            rgba: Srgba::new(r, g, b, a),
        }
    }

    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self {
            rgba: Srgba::new(r, g, b, 1.0),
        }
    }

    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            rgba: Srgba::new(r, g, b, a),
        }
    }

    pub fn hex(hex: u32) -> Self {
        let r = ((hex >> 16) & 0xFF) as f32 / 255.0;
        let g = ((hex >> 8) & 0xFF) as f32 / 255.0;
        let b = (hex & 0xFF) as f32 / 255.0;
        Self::rgb(r, g, b)
    }

    pub fn hex_a(hex: u32) -> Self {
        let r = ((hex >> 24) & 0xFF) as f32 / 255.0;
        let g = ((hex >> 16) & 0xFF) as f32 / 255.0;
        let b = ((hex >> 8) & 0xFF) as f32 / 255.0;
        let a = (hex & 0xFF) as f32 / 255.0;
        Self::rgba(r, g, b, a)
    }

    pub fn gray(v: f32) -> Self {
        Self::rgb(v, v, v)
    }

    pub fn alpha(&self) -> f32 {
        self.rgba.alpha
    }

    pub fn with_alpha(mut self, a: f32) -> Self {
        self.rgba.alpha = a;
        self
    }

    pub fn lighten(self, amount: f32) -> Self {
        let r = (self.rgba.red + amount).min(1.0);
        let g = (self.rgba.green + amount).min(1.0);
        let b = (self.rgba.blue + amount).min(1.0);
        Self::rgba(r, g, b, self.rgba.alpha)
    }

    pub fn to_rgba_srgb(self) -> [f32; 4] {
        [
            self.rgba.red,
            self.rgba.green,
            self.rgba.blue,
            self.rgba.alpha,
        ]
    }

    pub fn to_rgba_linear(self) -> [f32; 4] {
        let linear = palette::LinSrgba::from_color(self.rgba);
        [linear.red, linear.green, linear.blue, linear.alpha]
    }

    pub(crate) fn from_srgba(rgba: Srgba<f32>) -> Self {
        Self { rgba }
    }

    pub const WHITE: Color = Color::const_rgba(1.0, 1.0, 1.0, 1.0);
    pub const BLACK: Color = Color::const_rgba(0.0, 0.0, 0.0, 1.0);
    pub const TRANSPARENT: Color = Color::const_rgba(0.0, 0.0, 0.0, 0.0);
    
    pub const NODE_BG: Color = Color::const_rgba(48.0 / 255.0, 48.0 / 255.0, 48.0 / 255.0, 1.0);
    pub const NODE_BORDER: Color = Color::const_rgba(0.0, 0.0, 0.0, 1.0);
    pub const TEXT_PRIMARY: Color = Color::const_rgba(1.0, 1.0, 1.0, 1.0);
    pub const TEXT_MUTED: Color = Color::const_rgba(0.7, 0.7, 0.7, 1.0);
    pub const GRID_MINOR: Color = Color::const_rgba(1.0, 1.0, 1.0, 0.03);
}

impl From<Srgba<f32>> for Color {
    fn from(rgba: Srgba<f32>) -> Self {
        Self::from_srgba(rgba)
    }
}

impl From<Color> for Srgba<f32> {
    fn from(color: Color) -> Self {
        color.rgba
    }
}
