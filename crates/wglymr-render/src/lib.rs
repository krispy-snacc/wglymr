// WGLYMR Render ABI
// Pure rendering contract - no graph or editor knowledge.

pub mod draw_item;
pub mod draw_kind;
pub mod draw_layer;
pub mod entity_metadata;

pub use draw_item::DrawItem;
pub use draw_kind::{CircleDraw, DrawKind, GlyphDraw, LineDraw, RectDraw, RoundedRectDraw};
pub use draw_layer::DrawLayer;
pub use entity_metadata::EntityMetadata;

// HitLayer is now just an opaque u8 at the render ABI level
// Actual enum with semantics lives in wglymr-view
pub use draw_item::HitLayer;
