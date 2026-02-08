// WGLYMR Document System
// Owns editable state, undo/redo, and command processing.

pub mod adapter;
pub mod commands;
pub mod descriptors;
pub mod snapshot;
pub mod test_adapter;

pub use adapter::DocumentAdapter;
pub use commands::{EdgeId, EditorCommand, LiteralValue, NodeId, NodePosition, SocketId};
pub use descriptors::{EdgeDescriptor, NodeDescriptor, SocketDescriptor, SocketDirection};
pub use snapshot::GraphSnapshot;
