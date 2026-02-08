// WGLYMR Frontend - Web
// WASM bindings and web platform integration.

#[cfg(target_arch = "wasm32")]
pub mod wasm;

#[cfg(target_arch = "wasm32")]
pub use wasm::*;
