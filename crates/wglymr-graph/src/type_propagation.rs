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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{build_graph_view, Graph, MathOp, NodeKind};
    use glam::Vec2;

    #[test]
    fn test_simple_value_chain() {
        // Value(Float) -> Generic("pass") -> check output
        let mut graph = Graph::new();

        let value_node = graph.add_node(
            NodeKind::Value(ValueType::Float),
            Vec2::ZERO,
            vec![],
            vec![("out".to_string(), ValueType::Float)],
        );

        let pass_node = graph.add_node(
            NodeKind::Generic("pass".to_string()),
            Vec2::new(100.0, 0.0),
            vec![("in".to_string(), ValueType::Float)],
            vec![("out".to_string(), ValueType::Float)],
        );

        let value_out = graph.node(value_node).unwrap().outputs[0];
        let pass_in = graph.node(pass_node).unwrap().inputs[0];
        let pass_out = graph.node(pass_node).unwrap().outputs[0];

        graph.connect(value_out, pass_in).unwrap();

        let view = build_graph_view(&graph, &[pass_node]).unwrap();
        let type_map = propagate_types(&view).unwrap();

        assert_eq!(type_map.get(value_out), Some(ValueType::Float));
        assert_eq!(type_map.get(pass_out), Some(ValueType::Float));
    }

    #[test]
    fn test_binary_math_matching_types() {
        // Value(Float) -> Add <- Value(Float)
        let mut graph = Graph::new();

        let value1 = graph.add_node(
            NodeKind::Value(ValueType::Float),
            Vec2::ZERO,
            vec![],
            vec![("out".to_string(), ValueType::Float)],
        );

        let value2 = graph.add_node(
            NodeKind::Value(ValueType::Float),
            Vec2::new(0.0, 100.0),
            vec![],
            vec![("out".to_string(), ValueType::Float)],
        );

        let add_node = graph.add_node(
            NodeKind::Math(MathOp::Add),
            Vec2::new(200.0, 50.0),
            vec![
                ("a".to_string(), ValueType::Float),
                ("b".to_string(), ValueType::Float),
            ],
            vec![("out".to_string(), ValueType::Float)],
        );

        let v1_out = graph.node(value1).unwrap().outputs[0];
        let v2_out = graph.node(value2).unwrap().outputs[0];
        let add_in1 = graph.node(add_node).unwrap().inputs[0];
        let add_in2 = graph.node(add_node).unwrap().inputs[1];
        let add_out = graph.node(add_node).unwrap().outputs[0];

        graph.connect(v1_out, add_in1).unwrap();
        graph.connect(v2_out, add_in2).unwrap();

        let view = build_graph_view(&graph, &[add_node]).unwrap();
        let type_map = propagate_types(&view).unwrap();

        assert_eq!(type_map.get(v1_out), Some(ValueType::Float));
        assert_eq!(type_map.get(v2_out), Some(ValueType::Float));
        assert_eq!(type_map.get(add_out), Some(ValueType::Float));
    }

    #[test]
    fn test_binary_math_mismatched_types() {
        // Value(Float) -> Add <- Value(Vec3) should fail
        let mut graph = Graph::new();

        let value1 = graph.add_node(
            NodeKind::Value(ValueType::Float),
            Vec2::ZERO,
            vec![],
            vec![("out".to_string(), ValueType::Float)],
        );

        let value2 = graph.add_node(
            NodeKind::Value(ValueType::Vec3),
            Vec2::new(0.0, 100.0),
            vec![],
            vec![("out".to_string(), ValueType::Vec3)],
        );

        let add_node = graph.add_node(
            NodeKind::Math(MathOp::Add),
            Vec2::new(200.0, 50.0),
            vec![
                ("a".to_string(), ValueType::Float),
                ("b".to_string(), ValueType::Vec3),
            ],
            vec![("out".to_string(), ValueType::Float)],
        );

        let v1_out = graph.node(value1).unwrap().outputs[0];
        let v2_out = graph.node(value2).unwrap().outputs[0];
        let add_in1 = graph.node(add_node).unwrap().inputs[0];
        let add_in2 = graph.node(add_node).unwrap().inputs[1];

        graph.connect(v1_out, add_in1).unwrap();
        graph.connect(v2_out, add_in2).unwrap();

        let view = build_graph_view(&graph, &[add_node]).unwrap();
        let result = propagate_types(&view);

        assert!(result.is_err());
        match result {
            Err(TypeError::Mismatch { expected, found }) => {
                assert_eq!(expected, ValueType::Float);
                assert_eq!(found, ValueType::Vec3);
            }
            _ => panic!("expected type mismatch error"),
        }
    }

    #[test]
    fn test_unreachable_nodes_ignored() {
        // Value(Float) -> pass -> output
        // Value(Vec3) (unreachable)
        let mut graph = Graph::new();

        let reachable_value = graph.add_node(
            NodeKind::Value(ValueType::Float),
            Vec2::ZERO,
            vec![],
            vec![("out".to_string(), ValueType::Float)],
        );

        let _unreachable_value = graph.add_node(
            NodeKind::Value(ValueType::Vec3),
            Vec2::new(0.0, 200.0),
            vec![],
            vec![("out".to_string(), ValueType::Vec3)],
        );

        let pass_node = graph.add_node(
            NodeKind::Generic("pass".to_string()),
            Vec2::new(200.0, 0.0),
            vec![("in".to_string(), ValueType::Float)],
            vec![("out".to_string(), ValueType::Float)],
        );

        let reach_out = graph.node(reachable_value).unwrap().outputs[0];
        let pass_in = graph.node(pass_node).unwrap().inputs[0];
        let pass_out = graph.node(pass_node).unwrap().outputs[0];

        graph.connect(reach_out, pass_in).unwrap();

        let view = build_graph_view(&graph, &[pass_node]).unwrap();
        let type_map = propagate_types(&view).unwrap();

        assert_eq!(type_map.get(reach_out), Some(ValueType::Float));
        assert_eq!(type_map.get(pass_out), Some(ValueType::Float));
    }

    #[test]
    fn test_multiple_roots() {
        // Root1: Value(Float) -> pass1
        // Root2: Value(Vec3) -> pass2
        let mut graph = Graph::new();

        let value1 = graph.add_node(
            NodeKind::Value(ValueType::Float),
            Vec2::ZERO,
            vec![],
            vec![("out".to_string(), ValueType::Float)],
        );

        let pass1 = graph.add_node(
            NodeKind::Generic("pass".to_string()),
            Vec2::new(100.0, 0.0),
            vec![("in".to_string(), ValueType::Float)],
            vec![("out".to_string(), ValueType::Float)],
        );

        let value2 = graph.add_node(
            NodeKind::Value(ValueType::Vec3),
            Vec2::new(0.0, 100.0),
            vec![],
            vec![("out".to_string(), ValueType::Vec3)],
        );

        let pass2 = graph.add_node(
            NodeKind::Generic("pass".to_string()),
            Vec2::new(100.0, 100.0),
            vec![("in".to_string(), ValueType::Vec3)],
            vec![("out".to_string(), ValueType::Vec3)],
        );

        let v1_out = graph.node(value1).unwrap().outputs[0];
        let p1_in = graph.node(pass1).unwrap().inputs[0];
        let p1_out = graph.node(pass1).unwrap().outputs[0];

        let v2_out = graph.node(value2).unwrap().outputs[0];
        let p2_in = graph.node(pass2).unwrap().inputs[0];
        let p2_out = graph.node(pass2).unwrap().outputs[0];

        graph.connect(v1_out, p1_in).unwrap();
        graph.connect(v2_out, p2_in).unwrap();

        let view = build_graph_view(&graph, &[pass1, pass2]).unwrap();
        let type_map = propagate_types(&view).unwrap();

        assert_eq!(type_map.get(p1_out), Some(ValueType::Float));
        assert_eq!(type_map.get(p2_out), Some(ValueType::Vec3));
    }
}
