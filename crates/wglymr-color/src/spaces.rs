use crate::Color;
use palette::{FromColor, Hsla, Oklab, Srgba};

impl Color {
    pub fn to_hsla(self) -> Hsla {
        Hsla::from_color(Srgba::<f32>::from(self))
    }

    pub fn from_hsla(h: f32, s: f32, l: f32, a: f32) -> Self {
        let hsla = Hsla::new(h, s, l, a);
        let srgba = Srgba::from_color(hsla);
        Color::from_srgba(srgba)
    }

    pub fn to_oklab(self) -> Oklab {
        Oklab::from_color(Srgba::<f32>::from(self))
    }

    pub fn from_oklab(l: f32, a: f32, b: f32, alpha: f32) -> Self {
        let oklab = Oklab::new(l, a, b);
        let mut srgba = Srgba::from_color(oklab);
        srgba.alpha = alpha;
        Color::from_srgba(srgba)
    }
}
