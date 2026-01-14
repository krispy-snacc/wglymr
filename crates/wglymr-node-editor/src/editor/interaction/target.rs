use crate::document::commands::{EdgeId, NodeId, SocketId};

#[derive(Debug, Clone, PartialEq)]
pub enum InteractionTarget {
    None,

    Node {
        node_id: NodeId,
    },

    NodeHeader {
        node_id: NodeId,
    },

    Socket {
        node_id: NodeId,
        socket_id: SocketId,
    },

    Edge {
        edge_id: EdgeId,
    },

    Overlay {
        kind: OverlayKind,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum OverlayKind {
    SelectionBox,
    Gizmo,
}
