#[cfg(test)]
use crate::*;

#[test]
fn test_type_mismatch_error_produces_diagnostic() {
    let err = TypeError::Mismatch {
        expected: ValueType::Vec3,
        found: ValueType::Float,
    };

    let diags = diagnostics::diagnostics_from_type_error(&err);

    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0].severity, diagnostics::DiagnosticSeverity::Error);
    assert!(diags[0].message.contains("Type mismatch"));
    assert!(diags[0].message.contains("Vec3"));
    assert!(diags[0].message.contains("Float"));
}

#[test]
fn test_unconnected_required_input_has_socket_id() {
    let socket = SocketId(42);
    let err = TypeError::UnconnectedRequiredInput { socket };

    let diags = diagnostics::diagnostics_from_type_error(&err);

    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0].severity, diagnostics::DiagnosticSeverity::Error);
    assert_eq!(diags[0].socket, Some(socket));
    assert!(diags[0].message.contains("Required input"));
}

#[test]
fn test_optional_input_missing_default_has_socket_id() {
    let socket = SocketId(99);
    let err = TypeError::OptionalInputMissingDefault { socket };

    let diags = diagnostics::diagnostics_from_type_error(&err);

    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0].severity, diagnostics::DiagnosticSeverity::Error);
    assert_eq!(diags[0].socket, Some(socket));
    assert!(diags[0].message.contains("Optional input"));
}

#[test]
fn test_default_literal_type_mismatch_has_socket_id() {
    let socket = SocketId(77);
    let err = TypeError::DefaultLiteralTypeMismatch {
        socket,
        expected: ValueType::Float,
        found: ValueType::Int,
    };

    let diags = diagnostics::diagnostics_from_type_error(&err);

    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0].severity, diagnostics::DiagnosticSeverity::Error);
    assert_eq!(diags[0].socket, Some(socket));
    assert!(diags[0].message.contains("Default value type mismatch"));
}

#[test]
fn test_lowering_missing_input_produces_socket_error() {
    let socket = SocketId(10);
    let err = IrLoweringError::MissingInput(socket);

    let diags = diagnostics::diagnostics_from_lowering_error(&err);

    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0].severity, diagnostics::DiagnosticSeverity::Error);
    assert_eq!(diags[0].socket, Some(socket));
    assert!(diags[0].message.contains("Required input"));
}

#[test]
fn test_lowering_missing_type_produces_socket_error() {
    let socket = SocketId(20);
    let err = IrLoweringError::MissingType(socket);

    let diags = diagnostics::diagnostics_from_lowering_error(&err);

    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0].severity, diagnostics::DiagnosticSeverity::Error);
    assert_eq!(diags[0].socket, Some(socket));
    assert!(diags[0].message.contains("Type information"));
}

#[test]
fn test_conversion_error_produces_diagnostic() {
    let err = ConversionError::NoConversion {
        from: IrType::Vec3,
        to: IrType::Float,
    };

    let diags = diagnostics::diagnostics_from_conversion_error(&err);

    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0].severity, diagnostics::DiagnosticSeverity::Error);
    assert!(diags[0].message.contains("No valid conversion"));
    assert!(diags[0].message.contains("Vec3"));
    assert!(diags[0].message.contains("Float"));
}

#[test]
fn test_conversion_inserted_warning() {
    let socket = SocketId(50);
    let diag = diagnostics::warning_conversion_inserted(socket, IrType::Float, IrType::Vec3);

    assert_eq!(diag.severity, diagnostics::DiagnosticSeverity::Warning);
    assert_eq!(diag.socket, Some(socket));
    assert!(diag.message.contains("Inserted"));
    assert!(diag.message.contains("Float"));
    assert!(diag.message.contains("Vec3"));
    assert!(diag.message.contains("conversion"));
}

#[test]
fn test_default_value_used_warning() {
    let socket = SocketId(60);
    let diag = diagnostics::warning_default_value_used(socket);

    assert_eq!(diag.severity, diagnostics::DiagnosticSeverity::Warning);
    assert_eq!(diag.socket, Some(socket));
    assert!(diag.message.contains("default value"));
}

#[test]
fn test_unreachable_nodes_warning() {
    let mut graph = Graph::new();

    let node1 = graph.add_node(
        NodeKind::Value(ValueType::Float),
        glam::Vec2::ZERO,
        vec![],
        vec![("out".to_string(), ValueType::Float)],
    );
    let node2 = graph.add_node(
        NodeKind::Value(ValueType::Float),
        glam::Vec2::ZERO,
        vec![],
        vec![("out".to_string(), ValueType::Float)],
    );

    let view = build_graph_view(&graph, &[node1]).unwrap();

    let warnings = diagnostics::warnings_unreachable_nodes(&view);

    assert_eq!(warnings.len(), 1);
    assert_eq!(
        warnings[0].severity,
        diagnostics::DiagnosticSeverity::Warning
    );
    assert_eq!(warnings[0].node, Some(node2));
    assert!(warnings[0].message.contains("unreachable"));
}

#[test]
fn test_unreachable_nodes_helper() {
    let mut graph = Graph::new();

    let node1 = graph.add_node(
        NodeKind::Value(ValueType::Float),
        glam::Vec2::ZERO,
        vec![],
        vec![("out".to_string(), ValueType::Float)],
    );
    let node2 = graph.add_node(
        NodeKind::Value(ValueType::Float),
        glam::Vec2::ZERO,
        vec![],
        vec![("out".to_string(), ValueType::Float)],
    );
    let node3 = graph.add_node(
        NodeKind::Value(ValueType::Float),
        glam::Vec2::ZERO,
        vec![],
        vec![("out".to_string(), ValueType::Float)],
    );

    let view = build_graph_view(&graph, &[node1]).unwrap();

    let unreachable = diagnostics::unreachable_nodes(&view);

    assert_eq!(unreachable.len(), 2);
    assert!(unreachable.contains(&node2));
    assert!(unreachable.contains(&node3));
    assert!(!unreachable.contains(&node1));
}

#[test]
fn test_all_nodes_reachable_no_warnings() {
    let mut graph = Graph::new();

    let node1 = graph.add_node(
        NodeKind::Value(ValueType::Float),
        glam::Vec2::ZERO,
        vec![],
        vec![("out".to_string(), ValueType::Float)],
    );
    let node2 = graph.add_node(
        NodeKind::Math(MathOp::Add),
        glam::Vec2::ZERO,
        vec![("a".to_string(), ValueType::Float)],
        vec![("result".to_string(), ValueType::Float)],
    );

    let output1 = graph.node(node1).unwrap().outputs[0];
    let input2 = graph.node(node2).unwrap().inputs[0];

    graph.connect(output1, input2).unwrap();

    let view = build_graph_view(&graph, &[node2]).unwrap();

    let warnings = diagnostics::warnings_unreachable_nodes(&view);

    assert_eq!(warnings.len(), 0);
}

#[test]
fn test_diagnostic_equality() {
    let socket = SocketId(123);

    let diag1 = diagnostics::Diagnostic {
        severity: diagnostics::DiagnosticSeverity::Error,
        message: "Test error".to_string(),
        node: None,
        socket: Some(socket),
    };

    let diag2 = diagnostics::Diagnostic {
        severity: diagnostics::DiagnosticSeverity::Error,
        message: "Test error".to_string(),
        node: None,
        socket: Some(socket),
    };

    assert_eq!(diag1.severity, diag2.severity);
    assert_eq!(diag1.message, diag2.message);
    assert_eq!(diag1.socket, diag2.socket);
}
