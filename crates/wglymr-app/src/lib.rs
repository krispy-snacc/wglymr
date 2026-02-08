// WGLYMR Application Shell
// Editor and panel registration.

pub mod engine;
pub mod ui;
pub mod visual_state;

pub use engine::*;
pub use ui::*;
pub use visual_state::*;

// Re-export depth from wglymr-view (no duplication)
pub use wglymr_view::{
    resolve_depth, DepthBand, DepthLayer, DEPTH_BANDS, Z_BODY, Z_HEADER, Z_SOCKET, Z_TEXT,
};
