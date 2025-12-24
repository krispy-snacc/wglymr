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

pub mod diagnostics;
mod error;
mod graph;
pub mod ir;
pub mod ir_conversion;
pub mod ir_debug;
mod ir_lowering;
mod link;
mod node;
pub mod passes;
mod socket;
mod type_propagation;
mod types;
pub mod wgsl;

#[cfg(test)]
mod tests;

pub use diagnostics::{
    diagnostics_from_conversion_error, diagnostics_from_lowering_error,
    diagnostics_from_type_error, unreachable_nodes, warning_conversion_inserted,
    warning_default_value_used, warnings_unreachable_nodes, Diagnostic, DiagnosticSeverity,
};
pub use error::GraphError;
pub use graph::{Graph, InputDef};
pub use ir::{BinaryOp, IrInst, IrProgram, IrType, Literal, ValueId};
pub use ir_conversion::{insert_conversions, ConversionError};
pub use ir_debug::{pretty_print, validate_ir, IrValidationError};
pub use ir_lowering::{lower_to_ir, IrLoweringError};
pub use link::{Link, LinkId};
pub use node::{MathOp, Node, NodeId, NodeKind};
pub use passes::{build_graph_view, detect_cycles, reachable_from, topological_sort, GraphView};
pub use socket::{InputSocketConfig, Socket, SocketDirection, SocketId};
pub use type_propagation::{propagate_types, TypeMap};
pub use types::{are_compatible, unify, TypeError, ValueType};
pub use wgsl::emit_wgsl;
