#[cfg(test)]
mod tests {
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

    mod passes {
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
    }
}
