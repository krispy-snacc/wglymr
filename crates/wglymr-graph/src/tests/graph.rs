use glam::Vec2;

use crate::{Graph, GraphError, NodeKind, SocketDirection, ValueType};

#[test]
fn test_create_node_creates_sockets() {
    let mut graph = Graph::new();

    let node_id = graph.add_node(
        NodeKind::Generic("TestNode".to_string()),
        Vec2::new(0.0, 0.0),
        vec![("input1".to_string(), ValueType::Float)],
        vec![("output1".to_string(), ValueType::Float)],
    );

    let node = graph.node(node_id).unwrap();
    assert_eq!(node.inputs.len(), 1);
    assert_eq!(node.outputs.len(), 1);

    let input_socket = graph.socket(node.inputs[0]).unwrap();
    assert_eq!(input_socket.direction, SocketDirection::Input);
    assert_eq!(input_socket.value_type, ValueType::Float);
    assert_eq!(input_socket.name, "input1");
    assert_eq!(input_socket.node, node_id);

    let output_socket = graph.socket(node.outputs[0]).unwrap();
    assert_eq!(output_socket.direction, SocketDirection::Output);
    assert_eq!(output_socket.value_type, ValueType::Float);
    assert_eq!(output_socket.name, "output1");
    assert_eq!(output_socket.node, node_id);
}

#[test]
fn test_valid_connection_succeeds() {
    let mut graph = Graph::new();

    let node1 = graph.add_node(
        NodeKind::Generic("Node1".to_string()),
        Vec2::new(0.0, 0.0),
        vec![],
        vec![("out".to_string(), ValueType::Float)],
    );

    let node2 = graph.add_node(
        NodeKind::Generic("Node2".to_string()),
        Vec2::new(100.0, 0.0),
        vec![("in".to_string(), ValueType::Float)],
        vec![],
    );

    let output = graph.node(node1).unwrap().outputs[0];
    let input = graph.node(node2).unwrap().inputs[0];

    let link_id = graph.connect(output, input).unwrap();

    let link = graph.link(link_id).unwrap();
    assert_eq!(link.from, output);
    assert_eq!(link.to, input);
}

#[test]
fn test_input_to_input_fails() {
    let mut graph = Graph::new();

    let node1 = graph.add_node(
        NodeKind::Generic("Node1".to_string()),
        Vec2::new(0.0, 0.0),
        vec![("in1".to_string(), ValueType::Float)],
        vec![],
    );

    let node2 = graph.add_node(
        NodeKind::Generic("Node2".to_string()),
        Vec2::new(100.0, 0.0),
        vec![("in2".to_string(), ValueType::Float)],
        vec![],
    );

    let input1 = graph.node(node1).unwrap().inputs[0];
    let input2 = graph.node(node2).unwrap().inputs[0];

    let result = graph.connect(input1, input2);
    assert!(matches!(result, Err(GraphError::WrongDirection { .. })));
}

#[test]
fn test_type_mismatch_fails() {
    let mut graph = Graph::new();

    let node1 = graph.add_node(
        NodeKind::Generic("Node1".to_string()),
        Vec2::new(0.0, 0.0),
        vec![],
        vec![("out".to_string(), ValueType::Float)],
    );

    let node2 = graph.add_node(
        NodeKind::Generic("Node2".to_string()),
        Vec2::new(100.0, 0.0),
        vec![("in".to_string(), ValueType::Vec3)],
        vec![],
    );

    let output = graph.node(node1).unwrap().outputs[0];
    let input = graph.node(node2).unwrap().inputs[0];

    let result = graph.connect(output, input);
    assert!(matches!(result, Err(GraphError::TypeMismatch { .. })));
}

#[test]
fn test_double_connection_to_input_fails() {
    let mut graph = Graph::new();

    let node1 = graph.add_node(
        NodeKind::Generic("Node1".to_string()),
        Vec2::new(0.0, 0.0),
        vec![],
        vec![("out".to_string(), ValueType::Float)],
    );

    let node2 = graph.add_node(
        NodeKind::Generic("Node2".to_string()),
        Vec2::new(100.0, 0.0),
        vec![],
        vec![("out".to_string(), ValueType::Float)],
    );

    let node3 = graph.add_node(
        NodeKind::Generic("Node3".to_string()),
        Vec2::new(200.0, 0.0),
        vec![("in".to_string(), ValueType::Float)],
        vec![],
    );

    let output1 = graph.node(node1).unwrap().outputs[0];
    let output2 = graph.node(node2).unwrap().outputs[0];
    let input = graph.node(node3).unwrap().inputs[0];

    graph.connect(output1, input).unwrap();

    let result = graph.connect(output2, input);
    assert!(matches!(result, Err(GraphError::InputAlreadyConnected)));
}

#[test]
fn test_disconnect_removes_link() {
    let mut graph = Graph::new();

    let node1 = graph.add_node(
        NodeKind::Generic("Node1".to_string()),
        Vec2::new(0.0, 0.0),
        vec![],
        vec![("out".to_string(), ValueType::Float)],
    );

    let node2 = graph.add_node(
        NodeKind::Generic("Node2".to_string()),
        Vec2::new(100.0, 0.0),
        vec![("in".to_string(), ValueType::Float)],
        vec![],
    );

    let output = graph.node(node1).unwrap().outputs[0];
    let input = graph.node(node2).unwrap().inputs[0];

    let link_id = graph.connect(output, input).unwrap();

    assert!(graph.link(link_id).is_some());
    assert_eq!(graph.links_into(input).count(), 1);

    let removed = graph.disconnect(link_id);
    assert!(removed);

    assert!(graph.link(link_id).is_none());
    assert_eq!(graph.links_into(input).count(), 0);

    graph.connect(output, input).unwrap();
}

#[test]
fn test_links_into_and_out_of() {
    let mut graph = Graph::new();

    let node1 = graph.add_node(
        NodeKind::Generic("Node1".to_string()),
        Vec2::new(0.0, 0.0),
        vec![],
        vec![("out".to_string(), ValueType::Float)],
    );

    let node2 = graph.add_node(
        NodeKind::Generic("Node2".to_string()),
        Vec2::new(100.0, 0.0),
        vec![("in1".to_string(), ValueType::Float)],
        vec![],
    );

    let node3 = graph.add_node(
        NodeKind::Generic("Node3".to_string()),
        Vec2::new(100.0, 100.0),
        vec![("in2".to_string(), ValueType::Float)],
        vec![],
    );

    let output = graph.node(node1).unwrap().outputs[0];
    let input1 = graph.node(node2).unwrap().inputs[0];
    let input2 = graph.node(node3).unwrap().inputs[0];

    graph.connect(output, input1).unwrap();
    graph.connect(output, input2).unwrap();

    assert_eq!(graph.links_out_of(output).count(), 2);

    assert_eq!(graph.links_into(input1).count(), 1);
    assert_eq!(graph.links_into(input2).count(), 1);

    assert_eq!(graph.links_out_of(input1).count(), 0);
    assert_eq!(graph.links_out_of(input2).count(), 0);

    assert_eq!(graph.links_into(output).count(), 0);
}

#[test]
fn test_node_positions_stored() {
    let mut graph = Graph::new();

    let position = Vec2::new(42.5, 123.75);
    let node_id = graph.add_node(
        NodeKind::Generic("TestNode".to_string()),
        position,
        vec![],
        vec![],
    );

    let node = graph.node(node_id).unwrap();
    assert_eq!(node.position, position);
}

#[test]
fn test_stable_ids_not_reused() {
    let mut graph = Graph::new();

    let node1 = graph.add_node(
        NodeKind::Generic("Node1".to_string()),
        Vec2::ZERO,
        vec![],
        vec![("out".to_string(), ValueType::Float)],
    );

    let node2 = graph.add_node(
        NodeKind::Generic("Node2".to_string()),
        Vec2::ZERO,
        vec![("in".to_string(), ValueType::Float)],
        vec![],
    );

    let output = graph.node(node1).unwrap().outputs[0];
    let input = graph.node(node2).unwrap().inputs[0];
    let link_id = graph.connect(output, input).unwrap();

    graph.disconnect(link_id);
    let new_link_id = graph.connect(output, input).unwrap();

    assert_ne!(link_id, new_link_id);
}

#[test]
fn test_add_node_with_optional_input() {
    use crate::{InputDef, Literal};

    let mut graph = Graph::new();

    let node_id = graph.add_node_with_config(
        NodeKind::Generic("TestNode".to_string()),
        Vec2::ZERO,
        vec![InputDef::optional(
            "input".to_string(),
            ValueType::Float,
            Literal::Float(1.0),
        )],
        vec![("out".to_string(), ValueType::Float)],
    );

    let node = graph.node(node_id).unwrap();
    assert_eq!(node.inputs.len(), 1);

    let input_socket = graph.socket(node.inputs[0]).unwrap();
    let config = input_socket.input_config.as_ref().unwrap();
    assert!(config.optional);
    assert!(config.default.is_some());
    match config.default.as_ref().unwrap() {
        Literal::Float(v) => assert_eq!(*v, 1.0),
        _ => panic!("expected Float literal"),
    }
}

#[test]
fn test_add_node_with_required_input() {
    use crate::InputDef;

    let mut graph = Graph::new();

    let node_id = graph.add_node_with_config(
        NodeKind::Generic("TestNode".to_string()),
        Vec2::ZERO,
        vec![InputDef::required("input", ValueType::Float)],
        vec![("out".to_string(), ValueType::Float)],
    );

    let node = graph.node(node_id).unwrap();
    let input_socket = graph.socket(node.inputs[0]).unwrap();
    let config = input_socket.input_config.as_ref().unwrap();
    assert!(!config.optional);
    assert!(config.default.is_none());
}

#[test]
fn test_add_node_simple_creates_required_config() {
    let mut graph = Graph::new();

    let node_id = graph.add_node(
        NodeKind::Generic("TestNode".to_string()),
        Vec2::ZERO,
        vec![("input".to_string(), ValueType::Float)],
        vec![],
    );

    let node = graph.node(node_id).unwrap();
    let input_socket = graph.socket(node.inputs[0]).unwrap();
    let config = input_socket.input_config.as_ref().unwrap();
    assert!(!config.optional);
    assert!(config.default.is_none());
}

#[test]
#[cfg(feature = "debug-graph")]
fn test_invariant_checker_valid_graph() {
    let mut graph = Graph::new();

    let node1 = graph.add_node(
        NodeKind::Generic("Node1".to_string()),
        Vec2::ZERO,
        vec![],
        vec![("out".to_string(), ValueType::Float)],
    );

    let node2 = graph.add_node(
        NodeKind::Generic("Node2".to_string()),
        Vec2::ZERO,
        vec![("in".to_string(), ValueType::Float)],
        vec![],
    );

    let output = graph.node(node1).unwrap().outputs[0];
    let input = graph.node(node2).unwrap().inputs[0];
    graph.connect(output, input).unwrap();

    assert!(graph.check_invariants().is_ok());
}
