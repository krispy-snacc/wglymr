use crate::document::commands::{EdgeId, NodeId, SocketId};

/// Entity metadata for DrawItems to enable semantic interaction mapping.
/// Purely for CPU-side interaction logic; does not affect rendering.
#[derive(Debug, Clone, PartialEq)]
pub enum EntityMetadata {
    None,
    Node(NodeId),
    Socket { node_id: NodeId, socket_id: SocketId },
    Edge(EdgeId),
}

impl Default for EntityMetadata {
    fn default() -> Self {
        EntityMetadata::None
    }
}
