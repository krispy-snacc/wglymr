//! # wglymr-graph
//!
//! A deterministic, headless node graph representation for a node-based graphics system.
//!
//! This crate provides:
//! - Stable opaque IDs for nodes, sockets, and links
//! - Strong type safety for socket connections
//! - Serializable graph structure
//! - Explicit error handling
//! - Read-only graph analysis passes

mod error;
mod graph;
pub mod ir;
mod link;
mod node;
pub mod passes;
mod socket;
mod type_propagation;
mod types;

#[cfg(test)]
mod tests;

pub use error::GraphError;
pub use graph::Graph;
pub use ir::{BinaryOp, IrInst, IrProgram, IrType, Literal, ValueId};
pub use link::{Link, LinkId};
pub use node::{MathOp, Node, NodeId, NodeKind};
pub use passes::{build_graph_view, detect_cycles, reachable_from, topological_sort, GraphView};
pub use socket::{Socket, SocketDirection, SocketId};
pub use type_propagation::{propagate_types, TypeMap};
pub use types::{are_compatible, unify, TypeError, ValueType};
