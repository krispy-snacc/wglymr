use glam::Vec2;

use crate::{
    build_graph_view, lower_to_ir, propagate_types, BinaryOp, Graph, IrInst, IrType, MathOp,
    NodeKind, ValueType,
};

#[test]
fn test_simple_value_node() {
    let mut graph = Graph::new();
    let node_id = graph.add_node(
        NodeKind::Value(ValueType::Float),
        Vec2::ZERO,
        vec![],
        vec![("output".to_string(), ValueType::Float)],
    );

    let view = build_graph_view(&graph, &[node_id]).unwrap();
    let types = propagate_types(&view).unwrap();

    let program = lower_to_ir(&view, &types).unwrap();

    assert_eq!(program.instructions.len(), 1);
    match &program.instructions[0] {
        IrInst::Constant { ty, .. } => {
            assert_eq!(*ty, IrType::Float);
        }
        _ => panic!("expected constant instruction"),
    }
}

#[test]
fn test_value_binary_value() {
    let mut graph = Graph::new();

    let value1 = graph.add_node(
        NodeKind::Value(ValueType::Float),
        Vec2::ZERO,
        vec![],
        vec![("output".to_string(), ValueType::Float)],
    );

    let value2 = graph.add_node(
        NodeKind::Value(ValueType::Float),
        Vec2::ZERO,
        vec![],
        vec![("output".to_string(), ValueType::Float)],
    );

    let add_node = graph.add_node(
        NodeKind::Math(MathOp::Add),
        Vec2::ZERO,
        vec![
            ("a".to_string(), ValueType::Float),
            ("b".to_string(), ValueType::Float),
        ],
        vec![("result".to_string(), ValueType::Float)],
    );

    let value1_out = graph.node(value1).unwrap().outputs[0];
    let value2_out = graph.node(value2).unwrap().outputs[0];
    let add_in_a = graph.node(add_node).unwrap().inputs[0];
    let add_in_b = graph.node(add_node).unwrap().inputs[1];

    graph.connect(value1_out, add_in_a).unwrap();
    graph.connect(value2_out, add_in_b).unwrap();

    let view = build_graph_view(&graph, &[add_node]).unwrap();
    let types = propagate_types(&view).unwrap();

    let program = lower_to_ir(&view, &types).unwrap();

    assert_eq!(program.instructions.len(), 3);

    match &program.instructions[0] {
        IrInst::Constant { ty, .. } => assert_eq!(*ty, IrType::Float),
        _ => panic!("expected constant"),
    }

    match &program.instructions[1] {
        IrInst::Constant { ty, .. } => assert_eq!(*ty, IrType::Float),
        _ => panic!("expected constant"),
    }

    match &program.instructions[2] {
        IrInst::Binary { op, ty, lhs, rhs } => {
            assert_eq!(*op, BinaryOp::Add);
            assert_eq!(*ty, IrType::Float);
            // Both lhs and rhs should be valid ValueIds from the constants
            assert!(lhs.0 < 2, "lhs should be one of the two constants");
            assert!(rhs.0 < 2, "rhs should be one of the two constants");
            assert_ne!(lhs, rhs, "lhs and rhs should be different");
        }
        _ => panic!("expected binary"),
    }
}

#[test]
fn test_pass_through_node() {
    let mut graph = Graph::new();

    let value_node = graph.add_node(
        NodeKind::Value(ValueType::Float),
        Vec2::ZERO,
        vec![],
        vec![("output".to_string(), ValueType::Float)],
    );

    let pass_node = graph.add_node(
        NodeKind::Generic("pass".to_string()),
        Vec2::ZERO,
        vec![("input".to_string(), ValueType::Float)],
        vec![("output".to_string(), ValueType::Float)],
    );

    let value_out = graph.node(value_node).unwrap().outputs[0];
    let pass_in = graph.node(pass_node).unwrap().inputs[0];

    graph.connect(value_out, pass_in).unwrap();

    let view = build_graph_view(&graph, &[pass_node]).unwrap();
    let types = propagate_types(&view).unwrap();

    let program = lower_to_ir(&view, &types).unwrap();

    assert_eq!(program.instructions.len(), 1);
    match &program.instructions[0] {
        IrInst::Constant { ty, .. } => assert_eq!(*ty, IrType::Float),
        _ => panic!("expected constant"),
    }
}

#[test]
fn test_unreachable_nodes_emit_nothing() {
    let mut graph = Graph::new();

    let reachable_node = graph.add_node(
        NodeKind::Value(ValueType::Float),
        Vec2::ZERO,
        vec![],
        vec![("output".to_string(), ValueType::Float)],
    );

    graph.add_node(
        NodeKind::Value(ValueType::Float),
        Vec2::ZERO,
        vec![],
        vec![("output".to_string(), ValueType::Float)],
    );

    let view = build_graph_view(&graph, &[reachable_node]).unwrap();
    let types = propagate_types(&view).unwrap();

    let program = lower_to_ir(&view, &types).unwrap();

    assert_eq!(program.instructions.len(), 1);
}

#[test]
fn test_value_ids_reused_correctly() {
    let mut graph = Graph::new();

    let value_node = graph.add_node(
        NodeKind::Value(ValueType::Float),
        Vec2::ZERO,
        vec![],
        vec![("output".to_string(), ValueType::Float)],
    );

    let pass1 = graph.add_node(
        NodeKind::Generic("pass".to_string()),
        Vec2::ZERO,
        vec![("input".to_string(), ValueType::Float)],
        vec![("output".to_string(), ValueType::Float)],
    );

    let pass2 = graph.add_node(
        NodeKind::Generic("pass".to_string()),
        Vec2::ZERO,
        vec![("input".to_string(), ValueType::Float)],
        vec![("output".to_string(), ValueType::Float)],
    );

    let value_out = graph.node(value_node).unwrap().outputs[0];
    let pass1_in = graph.node(pass1).unwrap().inputs[0];
    let pass1_out = graph.node(pass1).unwrap().outputs[0];
    let pass2_in = graph.node(pass2).unwrap().inputs[0];

    graph.connect(value_out, pass1_in).unwrap();
    graph.connect(pass1_out, pass2_in).unwrap();

    let view = build_graph_view(&graph, &[pass2]).unwrap();
    let types = propagate_types(&view).unwrap();

    let program = lower_to_ir(&view, &types).unwrap();

    assert_eq!(program.instructions.len(), 1);
}
