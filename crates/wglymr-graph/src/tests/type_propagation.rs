use crate::type_propagation::propagate_types;
use crate::{build_graph_view, Graph, MathOp, NodeKind, ValueType};
use glam::Vec2;

#[test]
fn test_simple_value_chain() {
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
        Err(crate::TypeError::Mismatch { expected, found }) => {
            assert_eq!(expected, ValueType::Float);
            assert_eq!(found, ValueType::Vec3);
        }
        _ => panic!("expected type mismatch error"),
    }
}

#[test]
fn test_unreachable_nodes_ignored() {
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
