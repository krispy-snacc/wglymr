// Adapter layer to wglymr-graph
// Translates editor operations to graph API calls

use crate::document::commands::EditorCommand;
use crate::document::descriptors::{EdgeDescriptor, NodeDescriptor, SocketDescriptor};

// Boundary between editor UI and underlying graph
// Owns document snapshot and applies atomic mutations
pub trait DocumentAdapter {
    // Apply a single atomic command to the document
    fn apply_command(&mut self, command: EditorCommand);

    // Monotonically increasing revision counter
    // Increments after each successful command
    fn document_revision(&self) -> u64;

    // Immutable snapshot of all nodes in the document
    fn nodes(&self) -> &[NodeDescriptor];

    // Immutable snapshot of all sockets in the document
    fn sockets(&self) -> &[SocketDescriptor];

    // Immutable snapshot of all edges in the document
    fn edges(&self) -> &[EdgeDescriptor];
}

// Basic concrete adapter implementation
// Satisfies the interface without graph integration
pub struct BasicDocumentAdapter {
    revision: u64,
    nodes: Vec<NodeDescriptor>,
    sockets: Vec<SocketDescriptor>,
    edges: Vec<EdgeDescriptor>,
}

impl BasicDocumentAdapter {
    pub fn new() -> Self {
        Self {
            revision: 0,
            nodes: Vec::new(),
            sockets: Vec::new(),
            edges: Vec::new(),
        }
    }
}

impl DocumentAdapter for BasicDocumentAdapter {
    fn apply_command(&mut self, _command: EditorCommand) {
        self.revision += 1;
    }

    fn document_revision(&self) -> u64 {
        self.revision
    }

    fn nodes(&self) -> &[NodeDescriptor] {
        &self.nodes
    }

    fn sockets(&self) -> &[SocketDescriptor] {
        &self.sockets
    }

    fn edges(&self) -> &[EdgeDescriptor] {
        &self.edges
    }
}
