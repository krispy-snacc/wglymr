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
///
/// For optional inputs without connections, uses the default value's type.
pub fn propagate_types(view: &GraphView) -> Result<TypeMap, TypeError> {
    let mut type_map = TypeMap::new();

    for &node_id in &view.topo_order {
        if !view.reachable.contains(&node_id) {
            continue;
        }

        let node = view
            .graph
            .node(node_id)
            .expect("node from topo_order must exist");

        let mut input_types = Vec::new();
        for &input_socket in &node.inputs {
            let socket = view
                .graph
                .socket(input_socket)
                .expect("socket from node must exist");

            let link = view.graph.links_into(input_socket).next();

            if let Some(link) = link {
                if let Some(source_type) = type_map.get(link.from) {
                    input_types.push(source_type);
                    continue;
                }
            }

            let config = socket.input_config.as_ref();
            let is_optional = config.map(|c| c.optional).unwrap_or(false);

            if is_optional {
                if let Some(default_literal) = config.and_then(|c| c.default.as_ref()) {
                    let default_type = default_literal.value_type();
                    if default_type != socket.value_type {
                        return Err(TypeError::DefaultLiteralTypeMismatch {
                            socket: input_socket,
                            expected: socket.value_type,
                            found: default_type,
                        });
                    }
                    input_types.push(socket.value_type);
                    continue;
                } else {
                    return Err(TypeError::OptionalInputMissingDefault {
                        socket: input_socket,
                    });
                }
            }

            return Err(TypeError::UnconnectedRequiredInput {
                socket: input_socket,
            });
        }

        let output_type = node.kind.infer_output_type(&input_types)?;

        for &output_socket in &node.outputs {
            type_map.set(output_socket, output_type);
        }
    }

    Ok(type_map)
}
