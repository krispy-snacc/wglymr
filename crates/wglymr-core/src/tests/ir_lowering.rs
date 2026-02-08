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

#[test]
fn test_optional_input_emits_constant_in_ir() {
    use crate::{InputDef, Literal};

    let mut graph = Graph::new();

    let node = graph.add_node_with_config(
        NodeKind::Generic("pass".to_string()),
        Vec2::ZERO,
        vec![InputDef::optional(
            "in",
            ValueType::Float,
            Literal::Float(42.0),
        )],
        vec![("out".to_string(), ValueType::Float)],
    );

    let view = build_graph_view(&graph, &[node]).unwrap();
    let types = propagate_types(&view).unwrap();
    let program = lower_to_ir(&view, &types).unwrap();

    assert_eq!(program.instructions.len(), 1);
    match &program.instructions[0] {
        IrInst::Constant { value, ty } => {
            assert_eq!(*ty, IrType::Float);
            match value {
                Literal::Float(v) => assert_eq!(*v, 42.0),
                _ => panic!("expected Float literal"),
            }
        }
        _ => panic!("expected Constant instruction"),
    }
}

#[test]
fn test_math_with_optional_inputs_emits_constants() {
    use crate::{InputDef, Literal};

    let mut graph = Graph::new();

    let add_node = graph.add_node_with_config(
        NodeKind::Math(MathOp::Add),
        Vec2::ZERO,
        vec![
            InputDef::optional("a", ValueType::Float, Literal::Float(10.0)),
            InputDef::optional("b", ValueType::Float, Literal::Float(20.0)),
        ],
        vec![("result".to_string(), ValueType::Float)],
    );

    let view = build_graph_view(&graph, &[add_node]).unwrap();
    let types = propagate_types(&view).unwrap();
    let program = lower_to_ir(&view, &types).unwrap();

    assert_eq!(program.instructions.len(), 3);

    match &program.instructions[0] {
        IrInst::Constant {
            value: Literal::Float(v),
            ty: IrType::Float,
        } => assert_eq!(*v, 10.0),
        _ => panic!("expected first constant 10.0"),
    }

    match &program.instructions[1] {
        IrInst::Constant {
            value: Literal::Float(v),
            ty: IrType::Float,
        } => assert_eq!(*v, 20.0),
        _ => panic!("expected second constant 20.0"),
    }

    match &program.instructions[2] {
        IrInst::Binary { op, lhs, rhs, ty } => {
            assert_eq!(*op, BinaryOp::Add);
            assert_eq!(*ty, IrType::Float);
            assert_eq!(lhs.0, 0);
            assert_eq!(rhs.0, 1);
        }
        _ => panic!("expected Binary instruction"),
    }
}

#[test]
fn test_mixed_connected_and_optional_inputs() {
    use crate::{InputDef, Literal};

    let mut graph = Graph::new();

    let value_node = graph.add_node(
        NodeKind::Value(ValueType::Float),
        Vec2::ZERO,
        vec![],
        vec![("out".to_string(), ValueType::Float)],
    );

    let add_node = graph.add_node_with_config(
        NodeKind::Math(MathOp::Add),
        Vec2::new(100.0, 0.0),
        vec![
            InputDef::required("a", ValueType::Float),
            InputDef::optional("b", ValueType::Float, Literal::Float(5.0)),
        ],
        vec![("result".to_string(), ValueType::Float)],
    );

    let value_out = graph.node(value_node).unwrap().outputs[0];
    let add_in_a = graph.node(add_node).unwrap().inputs[0];

    graph.connect(value_out, add_in_a).unwrap();

    let view = build_graph_view(&graph, &[add_node]).unwrap();
    let types = propagate_types(&view).unwrap();
    let program = lower_to_ir(&view, &types).unwrap();

    assert_eq!(program.instructions.len(), 3);

    match &program.instructions[0] {
        IrInst::Constant {
            ty: IrType::Float, ..
        } => {}
        _ => panic!("expected value node constant"),
    }

    match &program.instructions[1] {
        IrInst::Constant {
            value: Literal::Float(v),
            ty: IrType::Float,
        } => assert_eq!(*v, 5.0),
        _ => panic!("expected default constant 5.0"),
    }

    match &program.instructions[2] {
        IrInst::Binary {
            op: BinaryOp::Add,
            lhs,
            rhs,
            ..
        } => {
            assert_eq!(lhs.0, 0);
            assert_eq!(rhs.0, 1);
        }
        _ => panic!("expected Binary instruction"),
    }
}
