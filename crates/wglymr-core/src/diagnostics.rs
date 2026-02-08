//! Editor-facing diagnostics for translating compiler errors and warnings
//! into node/socket-level messages.
//!
//! Diagnostics are pure data that do not modify the graph or stop compilation.
//! The editor consumes these diagnostics to present feedback to the user.

use std::collections::HashSet;

use crate::{ConversionError, GraphView, IrLoweringError, IrType, NodeId, SocketId, TypeError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub node: Option<NodeId>,
    pub socket: Option<SocketId>,
}

#[allow(dead_code)]
impl Diagnostic {
    fn error(message: String) -> Self {
        Self {
            severity: DiagnosticSeverity::Error,
            message,
            node: None,
            socket: None,
        }
    }

    fn error_at_socket(socket: SocketId, message: String) -> Self {
        Self {
            severity: DiagnosticSeverity::Error,
            message,
            node: None,
            socket: Some(socket),
        }
    }

    fn error_at_node(node: NodeId, message: String) -> Self {
        Self {
            severity: DiagnosticSeverity::Error,
            message,
            node: Some(node),
            socket: None,
        }
    }

    fn warning(message: String) -> Self {
        Self {
            severity: DiagnosticSeverity::Warning,
            message,
            node: None,
            socket: None,
        }
    }

    fn warning_at_socket(socket: SocketId, message: String) -> Self {
        Self {
            severity: DiagnosticSeverity::Warning,
            message,
            node: None,
            socket: Some(socket),
        }
    }

    fn warning_at_node(node: NodeId, message: String) -> Self {
        Self {
            severity: DiagnosticSeverity::Warning,
            message,
            node: Some(node),
            socket: None,
        }
    }
}

/// Convert a TypeError into user-facing diagnostics
pub fn diagnostics_from_type_error(err: &TypeError) -> Vec<Diagnostic> {
    match err {
        TypeError::Mismatch { expected, found } => {
            vec![Diagnostic::error(format!(
                "Type mismatch: expected {:?}, found {:?}",
                expected, found
            ))]
        }

        TypeError::EmptyUnification => {
            vec![Diagnostic::error("Cannot unify empty type set".to_string())]
        }

        TypeError::OptionalInputMissingDefault { socket } => {
            vec![Diagnostic::error_at_socket(
                *socket,
                "Optional input missing default value".to_string(),
            )]
        }

        TypeError::DefaultLiteralTypeMismatch {
            socket,
            expected,
            found,
        } => {
            vec![Diagnostic::error_at_socket(
                *socket,
                format!(
                    "Default value type mismatch: expected {:?}, found {:?}",
                    expected, found
                ),
            )]
        }

        TypeError::UnconnectedRequiredInput { socket } => {
            vec![Diagnostic::error_at_socket(
                *socket,
                "Required input is not connected".to_string(),
            )]
        }
    }
}

/// Convert an IrLoweringError into user-facing diagnostics
pub fn diagnostics_from_lowering_error(err: &IrLoweringError) -> Vec<Diagnostic> {
    match err {
        IrLoweringError::MissingInput(socket) => {
            vec![Diagnostic::error_at_socket(
                *socket,
                "Required input is not connected".to_string(),
            )]
        }

        IrLoweringError::MissingType(socket) => {
            vec![Diagnostic::error_at_socket(
                *socket,
                "Type information missing".to_string(),
            )]
        }

        IrLoweringError::UnsupportedNode => {
            vec![Diagnostic::error("Unsupported node type".to_string())]
        }

        IrLoweringError::OptionalInputMissingDefault(socket) => {
            vec![Diagnostic::error_at_socket(
                *socket,
                "Optional input missing default value".to_string(),
            )]
        }
    }
}

/// Convert a ConversionError into user-facing diagnostics
pub fn diagnostics_from_conversion_error(err: &ConversionError) -> Vec<Diagnostic> {
    match err {
        ConversionError::NoConversion { from, to } => {
            vec![Diagnostic::error(format!(
                "No valid conversion from {:?} to {:?}",
                from, to
            ))]
        }
    }
}

/// Create a warning diagnostic for an inserted conversion
pub fn warning_conversion_inserted(socket: SocketId, from: IrType, to: IrType) -> Diagnostic {
    Diagnostic::warning_at_socket(
        socket,
        format!("Inserted {:?} => {:?} conversion", from, to),
    )
}

/// Create a warning diagnostic for default value usage
pub fn warning_default_value_used(socket: SocketId) -> Diagnostic {
    Diagnostic::warning_at_socket(socket, "Using default value for input".to_string())
}

/// Create warning diagnostics for unreachable nodes
pub fn warnings_unreachable_nodes(view: &GraphView) -> Vec<Diagnostic> {
    let mut warnings = Vec::new();

    for node_id in view.graph.node_ids() {
        if !view.reachable.contains(&node_id) {
            warnings.push(Diagnostic::warning_at_node(
                node_id,
                "Node is unreachable".to_string(),
            ));
        }
    }

    warnings
}

/// Collect all unreachable nodes from a graph view as a set
pub fn unreachable_nodes(view: &GraphView) -> HashSet<NodeId> {
    let mut unreachable = HashSet::new();

    for node_id in view.graph.node_ids() {
        if !view.reachable.contains(&node_id) {
            unreachable.insert(node_id);
        }
    }

    unreachable
}
