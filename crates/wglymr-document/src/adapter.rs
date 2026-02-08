// Adapter layer to wglymr-graph
// Translates editor operations to graph API calls

use crate::commands::EditorCommand;
use crate::descriptors::{EdgeDescriptor, NodeDescriptor, SocketDescriptor};
use crate::snapshot::GraphSnapshot;

// Boundary between editor UI and underlying graph
// Owns document snapshot and applies atomic mutations
pub trait DocumentAdapter {
    // Apply a single atomic command to the document
    fn apply_command(&mut self, command: EditorCommand);

    // Monotonically increasing revision counter
    // Increments after each successful command
    fn document_revision(&self) -> u64;

    // Get immutable snapshot of current document state
    fn snapshot(&self) -> GraphSnapshot;

    // Legacy slice-based API (kept for backward compatibility during transition)
    // Prefer using snapshot() for new code
    fn nodes(&self) -> &[NodeDescriptor];
    fn sockets(&self) -> &[SocketDescriptor];
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

impl Default for BasicDocumentAdapter {
    fn default() -> Self {
        Self::new()
    }
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

    fn snapshot(&self) -> GraphSnapshot {
        GraphSnapshot::from_slices(self.revision, &self.nodes, &self.sockets, &self.edges)
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
