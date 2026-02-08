// Read-only descriptor structs for rendering
// Immutable snapshots derived from document state

use crate::commands::{EdgeId, LiteralValue, NodeId, NodePosition, SocketId};

// Socket direction on a node
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketDirection {
    Input,
    Output,
}

// Diagnostic severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Info,
}

// Diagnostic message attached to graph elements
#[derive(Debug, Clone, PartialEq)]
pub struct Diagnostic {
    pub message: String,
    pub severity: DiagnosticSeverity,
}

// Read-only socket descriptor for rendering
#[derive(Debug, Clone, PartialEq)]
pub struct SocketDescriptor {
    pub socket_id: SocketId,
    pub node_id: NodeId,
    pub name: String,
    pub direction: SocketDirection,
    pub type_name: Option<String>,
    pub default_value: Option<LiteralValue>,
    pub connected_edges: Vec<EdgeId>,
    pub diagnostics: Vec<Diagnostic>,
}

// Read-only node descriptor for rendering
#[derive(Debug, Clone, PartialEq)]
pub struct NodeDescriptor {
    pub node_id: NodeId,
    pub node_kind: String,
    pub position: NodePosition,
    pub inputs: Vec<SocketId>,
    pub outputs: Vec<SocketId>,
    pub diagnostics: Vec<Diagnostic>,
}

// Read-only edge descriptor for rendering
#[derive(Debug, Clone, PartialEq)]
pub struct EdgeDescriptor {
    pub edge_id: EdgeId,
    pub from: SocketId,
    pub to: SocketId,
    pub diagnostics: Vec<Diagnostic>,
}
