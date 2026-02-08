// WGLYMR View System
// Layout, hit-testing, and render model generation (GPU-free).

pub mod culling;
pub mod depth;
pub mod emit;
pub mod hit_layer;
pub mod layout;
pub mod render_model;
pub mod text;
pub mod ui;
pub mod visual_state;

// Re-export hit testing
pub mod hit {
    pub use super::hit_impl::*;
    pub use super::hit_layer::HitLayer;
}

mod hit_impl;

pub use culling::*;
pub use depth::*;
pub use emit::{emit_edge_draw_items, emit_node_draw_items};
pub use hit::*;
pub use hit_layer::HitLayer;
pub use layout::*;
pub use render_model::{RenderEdge, RenderNode};
pub use text::*;
pub use ui::*;
pub use visual_state::*;

// TODO: ADR-required - EditorView and GlobalInteractionState are app-level concepts
// but needed by view layer for emit. Consider snapshot-based approach.
