// Immutable document snapshot
// Provides type-safe, versioned access to document state

use crate::descriptors::{EdgeDescriptor, NodeDescriptor, SocketDescriptor};
use std::sync::Arc;

/// Immutable snapshot of document state at a specific revision
///
/// This type enforces immutability and enables efficient cloning via Arc.
/// Snapshots are created on each document read to ensure view layer
/// sees consistent state even as document is mutated.
#[derive(Clone, Debug)]
pub struct GraphSnapshot {
    revision: u64,
    nodes: Arc<[NodeDescriptor]>,
    sockets: Arc<[SocketDescriptor]>,
    edges: Arc<[EdgeDescriptor]>,
}

impl GraphSnapshot {
    /// Create new snapshot from vectors (takes ownership)
    pub fn new(
        revision: u64,
        nodes: Vec<NodeDescriptor>,
        sockets: Vec<SocketDescriptor>,
        edges: Vec<EdgeDescriptor>,
    ) -> Self {
        Self {
            revision,
            nodes: nodes.into(),
            sockets: sockets.into(),
            edges: edges.into(),
        }
    }

    /// Create snapshot from slices (copies data)
    pub fn from_slices(
        revision: u64,
        nodes: &[NodeDescriptor],
        sockets: &[SocketDescriptor],
        edges: &[EdgeDescriptor],
    ) -> Self {
        Self {
            revision,
            nodes: nodes.to_vec().into(),
            sockets: sockets.to_vec().into(),
            edges: edges.to_vec().into(),
        }
    }

    /// Create empty snapshot
    pub fn empty() -> Self {
        Self {
            revision: 0,
            nodes: Arc::new([]),
            sockets: Arc::new([]),
            edges: Arc::new([]),
        }
    }

    /// Revision number - monotonically increasing
    pub fn revision(&self) -> u64 {
        self.revision
    }

    /// All nodes in snapshot
    pub fn nodes(&self) -> &[NodeDescriptor] {
        &self.nodes
    }

    /// All sockets in snapshot
    pub fn sockets(&self) -> &[SocketDescriptor] {
        &self.sockets
    }

    /// All edges in snapshot
    pub fn edges(&self) -> &[EdgeDescriptor] {
        &self.edges
    }
}

impl Default for GraphSnapshot {
    fn default() -> Self {
        Self::empty()
    }
}
