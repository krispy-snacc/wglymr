use std::collections::HashSet;

use glam::Vec2;

use crate::{
    passes::{build_graph_view, detect_cycles, reachable_from, topological_sort},
    Graph, GraphError, NodeId, NodeKind, ValueType,
};

#[test]
fn test_simple_dag_has_no_cycles() {
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
        vec![("out".to_string(), ValueType::Float)],
    );

    let node3 = graph.add_node(
        NodeKind::Generic("Node3".to_string()),
        Vec2::ZERO,
        vec![("in".to_string(), ValueType::Float)],
        vec![],
    );

    let out1 = graph.node(node1).unwrap().outputs[0];
    let in2 = graph.node(node2).unwrap().inputs[0];
    let out2 = graph.node(node2).unwrap().outputs[0];
    let in3 = graph.node(node3).unwrap().inputs[0];

    graph.connect(out1, in2).unwrap();
    graph.connect(out2, in3).unwrap();

    let cycles = detect_cycles(&graph);
    assert!(cycles.is_empty());
}

#[test]
fn test_self_cycle_detected() {
    let mut graph = Graph::new();

    let node = graph.add_node(
        NodeKind::Generic("SelfLoop".to_string()),
        Vec2::ZERO,
        vec![("in".to_string(), ValueType::Float)],
        vec![("out".to_string(), ValueType::Float)],
    );

    let input = graph.node(node).unwrap().inputs[0];
    let output = graph.node(node).unwrap().outputs[0];

    graph.connect(output, input).unwrap();

    let cycles = detect_cycles(&graph);
    assert_eq!(cycles.len(), 1);
    assert_eq!(cycles[0], vec![node]);
}

#[test]
fn test_multi_node_cycle_detected() {
    let mut graph = Graph::new();

    let node1 = graph.add_node(
        NodeKind::Generic("Node1".to_string()),
        Vec2::ZERO,
        vec![("in".to_string(), ValueType::Float)],
        vec![("out".to_string(), ValueType::Float)],
    );

    let node2 = graph.add_node(
        NodeKind::Generic("Node2".to_string()),
        Vec2::ZERO,
        vec![("in".to_string(), ValueType::Float)],
        vec![("out".to_string(), ValueType::Float)],
    );

    let node3 = graph.add_node(
        NodeKind::Generic("Node3".to_string()),
        Vec2::ZERO,
        vec![("in".to_string(), ValueType::Float)],
        vec![("out".to_string(), ValueType::Float)],
    );

    let out1 = graph.node(node1).unwrap().outputs[0];
    let in2 = graph.node(node2).unwrap().inputs[0];
    let out2 = graph.node(node2).unwrap().outputs[0];
    let in3 = graph.node(node3).unwrap().inputs[0];
    let out3 = graph.node(node3).unwrap().outputs[0];
    let in1 = graph.node(node1).unwrap().inputs[0];

    graph.connect(out1, in2).unwrap();
    graph.connect(out2, in3).unwrap();
    graph.connect(out3, in1).unwrap();

    let cycles = detect_cycles(&graph);
    assert_eq!(cycles.len(), 1);
    assert_eq!(cycles[0].len(), 3);
}

#[test]
fn test_topological_sort_includes_disconnected_nodes() {
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

    let disconnected = graph.add_node(
        NodeKind::Generic("Disconnected".to_string()),
        Vec2::ZERO,
        vec![],
        vec![],
    );

    let out1 = graph.node(node1).unwrap().outputs[0];
    let in2 = graph.node(node2).unwrap().inputs[0];
    graph.connect(out1, in2).unwrap();

    let sorted = topological_sort(&graph).unwrap();
    assert_eq!(sorted.len(), 3);
    assert!(sorted.contains(&node1));
    assert!(sorted.contains(&node2));
    assert!(sorted.contains(&disconnected));

    let pos1 = sorted.iter().position(|&n| n == node1).unwrap();
    let pos2 = sorted.iter().position(|&n| n == node2).unwrap();
    assert!(pos1 < pos2);
}

#[test]
fn test_topological_sort_fails_on_cycle() {
    let mut graph = Graph::new();

    let node1 = graph.add_node(
        NodeKind::Generic("Node1".to_string()),
        Vec2::ZERO,
        vec![("in".to_string(), ValueType::Float)],
        vec![("out".to_string(), ValueType::Float)],
    );

    let node2 = graph.add_node(
        NodeKind::Generic("Node2".to_string()),
        Vec2::ZERO,
        vec![("in".to_string(), ValueType::Float)],
        vec![("out".to_string(), ValueType::Float)],
    );

    let out1 = graph.node(node1).unwrap().outputs[0];
    let in2 = graph.node(node2).unwrap().inputs[0];
    let out2 = graph.node(node2).unwrap().outputs[0];
    let in1 = graph.node(node1).unwrap().inputs[0];

    graph.connect(out1, in2).unwrap();
    graph.connect(out2, in1).unwrap();

    let result = topological_sort(&graph);
    assert!(matches!(result, Err(GraphError::CycleDetected)));
}

#[test]
fn test_reachability_includes_only_contributing_nodes() {
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
        vec![("out".to_string(), ValueType::Float)],
    );

    let node3 = graph.add_node(
        NodeKind::Generic("Node3".to_string()),
        Vec2::ZERO,
        vec![("in".to_string(), ValueType::Float)],
        vec![],
    );

    let disconnected = graph.add_node(
        NodeKind::Generic("Disconnected".to_string()),
        Vec2::ZERO,
        vec![],
        vec![],
    );

    let out1 = graph.node(node1).unwrap().outputs[0];
    let in2 = graph.node(node2).unwrap().inputs[0];
    let out2 = graph.node(node2).unwrap().outputs[0];
    let in3 = graph.node(node3).unwrap().inputs[0];

    graph.connect(out1, in2).unwrap();
    graph.connect(out2, in3).unwrap();

    let reachable = reachable_from(&graph, &[node3]);

    assert_eq!(reachable.len(), 3);
    assert!(reachable.contains(&node1));
    assert!(reachable.contains(&node2));
    assert!(reachable.contains(&node3));
    assert!(!reachable.contains(&disconnected));
}

#[test]
fn test_graph_remains_unchanged_after_analysis() {
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

    let out1 = graph.node(node1).unwrap().outputs[0];
    let in2 = graph.node(node2).unwrap().inputs[0];
    graph.connect(out1, in2).unwrap();

    let nodes_before: HashSet<_> = graph.node_ids().collect();
    let link_count_before = graph.links_into(in2).count();

    detect_cycles(&graph);
    topological_sort(&graph).unwrap();
    reachable_from(&graph, &[node2]);

    let nodes_after: HashSet<_> = graph.node_ids().collect();
    let link_count_after = graph.links_into(in2).count();

    assert_eq!(nodes_before, nodes_after);
    assert_eq!(link_count_before, link_count_after);
}

#[test]
fn test_build_graph_view() {
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
        vec![("out".to_string(), ValueType::Float)],
    );

    let node3 = graph.add_node(
        NodeKind::Generic("Node3".to_string()),
        Vec2::ZERO,
        vec![("in".to_string(), ValueType::Float)],
        vec![],
    );

    let out1 = graph.node(node1).unwrap().outputs[0];
    let in2 = graph.node(node2).unwrap().inputs[0];
    let out2 = graph.node(node2).unwrap().outputs[0];
    let in3 = graph.node(node3).unwrap().inputs[0];

    graph.connect(out1, in2).unwrap();
    graph.connect(out2, in3).unwrap();

    let view = build_graph_view(&graph, &[node3]).unwrap();

    assert_eq!(view.topo_order.len(), 3);
    assert_eq!(view.reachable.len(), 3);
    assert!(view.reachable.contains(&node1));
    assert!(view.reachable.contains(&node2));
    assert!(view.reachable.contains(&node3));

    let pos1 = view.topo_order.iter().position(|&n| n == node1).unwrap();
    let pos2 = view.topo_order.iter().position(|&n| n == node2).unwrap();
    let pos3 = view.topo_order.iter().position(|&n| n == node3).unwrap();
    assert!(pos1 < pos2);
    assert!(pos2 < pos3);
}

#[test]
fn test_build_graph_view_fails_on_cycle() {
    let mut graph = Graph::new();

    let node1 = graph.add_node(
        NodeKind::Generic("Node1".to_string()),
        Vec2::ZERO,
        vec![("in".to_string(), ValueType::Float)],
        vec![("out".to_string(), ValueType::Float)],
    );

    let node2 = graph.add_node(
        NodeKind::Generic("Node2".to_string()),
        Vec2::ZERO,
        vec![("in".to_string(), ValueType::Float)],
        vec![("out".to_string(), ValueType::Float)],
    );

    let out1 = graph.node(node1).unwrap().outputs[0];
    let in2 = graph.node(node2).unwrap().inputs[0];
    let out2 = graph.node(node2).unwrap().outputs[0];
    let in1 = graph.node(node1).unwrap().inputs[0];

    graph.connect(out1, in2).unwrap();
    graph.connect(out2, in1).unwrap();

    let result = build_graph_view(&graph, &[node1]);
    assert!(matches!(result, Err(GraphError::CycleDetected)));
}

#[test]
fn test_multiple_roots_work_correctly() {
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
        vec![("out".to_string(), ValueType::Float)],
    );

    let node3 = graph.add_node(
        NodeKind::Generic("Node3".to_string()),
        Vec2::ZERO,
        vec![("in".to_string(), ValueType::Float)],
        vec![],
    );

    let node4 = graph.add_node(
        NodeKind::Generic("Node4".to_string()),
        Vec2::ZERO,
        vec![],
        vec![("out".to_string(), ValueType::Float)],
    );

    let node5 = graph.add_node(
        NodeKind::Generic("Node5".to_string()),
        Vec2::ZERO,
        vec![("in".to_string(), ValueType::Float)],
        vec![],
    );

    let out1 = graph.node(node1).unwrap().outputs[0];
    let in2 = graph.node(node2).unwrap().inputs[0];
    let out2 = graph.node(node2).unwrap().outputs[0];
    let in3 = graph.node(node3).unwrap().inputs[0];
    let out4 = graph.node(node4).unwrap().outputs[0];
    let in5 = graph.node(node5).unwrap().inputs[0];

    graph.connect(out1, in2).unwrap();
    graph.connect(out2, in3).unwrap();
    graph.connect(out4, in5).unwrap();

    let view = build_graph_view(&graph, &[node3, node5]).unwrap();

    assert_eq!(view.roots.len(), 2);
    assert!(view.roots.contains(&node3));
    assert!(view.roots.contains(&node5));

    assert_eq!(view.reachable.len(), 5);
    assert!(view.reachable.contains(&node1));
    assert!(view.reachable.contains(&node2));
    assert!(view.reachable.contains(&node3));
    assert!(view.reachable.contains(&node4));
    assert!(view.reachable.contains(&node5));
}

#[test]
fn test_empty_roots_results_in_empty_reachable_set() {
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

    let out1 = graph.node(node1).unwrap().outputs[0];
    let in2 = graph.node(node2).unwrap().inputs[0];
    graph.connect(out1, in2).unwrap();

    let view = build_graph_view(&graph, &[]).unwrap();

    assert_eq!(view.roots.len(), 0);
    assert_eq!(view.reachable.len(), 0);
    assert_eq!(view.topo_order.len(), 2);
}

#[test]
fn test_invalid_root_node_returns_error() {
    let mut graph = Graph::new();

    let node1 = graph.add_node(
        NodeKind::Generic("Node1".to_string()),
        Vec2::ZERO,
        vec![],
        vec![("out".to_string(), ValueType::Float)],
    );

    let invalid_node = NodeId(9999);

    let result = build_graph_view(&graph, &[node1, invalid_node]);
    assert!(matches!(result, Err(GraphError::NodeNotFound { .. })));
}
