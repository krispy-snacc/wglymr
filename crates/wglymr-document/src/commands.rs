// Command interface for graph mutations
// All graph modifications flow through explicit commands

// Stable identifier for a node in the document graph
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub u64);

// Stable identifier for a socket on a node
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SocketId(pub u64);

// Stable identifier for an edge connecting two sockets
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EdgeId(pub u64);

// Node position in document space
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NodePosition {
    pub x: f32,
    pub y: f32,
}

// Typed literal value for socket defaults
#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4([f32; 4]),
}

// Atomic document mutation command
#[derive(Debug, Clone, PartialEq)]
pub enum EditorCommand {
    CreateNode {
        node_kind: String,
        position: NodePosition,
        node_id: Option<NodeId>,
    },
    DeleteNode {
        node_id: NodeId,
    },
    MoveNode {
        node_id: NodeId,
        new_position: NodePosition,
    },
    CreateEdge {
        from: SocketId,
        to: SocketId,
        edge_id: Option<EdgeId>,
    },
    DeleteEdge {
        edge_id: EdgeId,
    },
    SetDefaultValue {
        socket_id: SocketId,
        value: LiteralValue,
    },
    ClearDefaultValue {
        socket_id: SocketId,
    },
}
