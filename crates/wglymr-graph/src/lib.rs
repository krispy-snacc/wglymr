//! # wglymr-graph
//!
//! A deterministic, headless node graph representation for a node-based graphics system.
//!
//! This crate provides:
//! - Stable opaque IDs for nodes, sockets, and links
//! - Strong type safety for socket connections
//! - Serializable graph structure
//! - Explicit error handling

mod error;
mod graph;
mod link;
mod node;
mod socket;

#[cfg(test)]
mod tests;

pub use error::GraphError;
pub use graph::Graph;
pub use link::{Link, LinkId};
pub use node::{MathOp, Node, NodeId, NodeKind};
pub use socket::{Socket, SocketDirection, SocketId, ValueType};
