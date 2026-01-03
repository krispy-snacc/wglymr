mod color;
mod gpu;
mod spaces;
mod theme;

pub use color::Color;
pub use theme::Theme;

pub mod prelude {
    pub use crate::Color;
}
