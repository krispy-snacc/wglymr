//! Font asset management for WGLYMR
//!
//! This crate owns all font assets used by the editor, including:
//! - Raw font files (.ttf)
//! - Generated MSDF atlases for scalable UI text
//! - Generated bitmap font atlases for pixel-aligned text
//!
//! Font assets are embedded into the binary at compile time and
//! accessed through typed interfaces.
//!
//! To regenerate font atlases, run:
//! ```bash
//! cargo run -p xtask -- fonts
//! ```

pub mod bitmap;
pub mod fonts;
pub mod msdf;

pub use bitmap::{BitmapAtlas, get_bitmap_atlas};
pub use fonts::{FontAsset, get_font_asset};
pub use msdf::{MsdfAtlas, get_msdf_atlas};
