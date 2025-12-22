//! Static type propagation for node graphs.
//!
//! Assigns concrete ValueTypes to every reachable socket in a GraphView.
//! Operates deterministically in topological order with strict typing rules.

use std::collections::HashMap;

use crate::{GraphView, NodeKind, SocketId, TypeError, ValueType};

/// Holds resolved types for all sockets that have been analyzed
pub struct TypeMap {
    socket_types: HashMap<SocketId, ValueType>,
}

impl TypeMap {
    /// Get the resolved type for a socket, if known
    pub fn get(&self, socket: SocketId) -> Option<ValueType> {
        self.socket_types.get(&socket).copied()
    }

    fn new() -> Self {
        Self {
            socket_types: HashMap::new(),
        }
    }

    fn set(&mut self, socket: SocketId, value_type: ValueType) {
        self.socket_types.insert(socket, value_type);
    }
}

impl NodeKind {
    /// Infer output type based on input types
    ///
    /// Each node kind defines how output types depend on inputs
    pub fn infer_output_type(&self, input_types: &[ValueType]) -> Result<ValueType, TypeError> {
        match self {
            NodeKind::Value(value_type) => {
                // Value nodes have no inputs, output is fixed
                if !input_types.is_empty() {
                    return Err(TypeError::Mismatch {
                        expected: *value_type,
                        found: input_types[0],
                    });
                }
                Ok(*value_type)
            }

            NodeKind::Math(_) => {
                // Binary math nodes: two inputs, unified output
                if input_types.len() != 2 {
                    return Err(TypeError::EmptyUnification);
                }

                let unified = crate::unify(input_types)?;
                Ok(unified)
            }

            NodeKind::Generic(_) => {
                // Pass-through: output type equals input type
                if input_types.len() != 1 {
                    return Err(TypeError::EmptyUnification);
                }
                Ok(input_types[0])
            }
        }
    }
}

/// Propagate types through a graph view
///
/// Iterates nodes in topological order, resolves input types from links,
/// and infers output types using NodeKind rules.
pub fn propagate_types(view: &GraphView) -> Result<TypeMap, TypeError> {
    let mut type_map = TypeMap::new();

    // Process nodes in topological order
    for &node_id in &view.topo_order {
        // Skip unreachable nodes
        if !view.reachable.contains(&node_id) {
            continue;
        }

        let node = view
            .graph
            .node(node_id)
            .expect("node from topo_order must exist");

        // Collect input types from connected links
        let mut input_types = Vec::new();
        for &input_socket in &node.inputs {
            // Find the link connected to this input socket
            let mut found_type = None;
            for link in view.graph.links_into(input_socket) {
                // Get the type of the source socket
                if let Some(source_type) = type_map.get(link.from) {
                    found_type = Some(source_type);
                    break;
                }
            }

            if let Some(ty) = found_type {
                input_types.push(ty);
            } else {
                // Input has no connection or source type not yet resolved
                // For now, treat missing inputs as errors
                return Err(TypeError::EmptyUnification);
            }
        }

        // Infer output type using NodeKind rules
        let output_type = node.kind.infer_output_type(&input_types)?;

        // Assign the output type to all output sockets
        for &output_socket in &node.outputs {
            type_map.set(output_socket, output_type);
        }
    }

    Ok(type_map)
}
