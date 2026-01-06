// TEMPORARY TEST DOCUMENT ADAPTER
// THIS FILE WILL BE REMOVED ONCE REAL GRAPH INTEGRATION IS COMPLETE
//
// Purpose: Provide hardcoded test nodes to validate:
// - Hit testing
// - Visual state (hover, select, active)
// - Dragging with transient offsets
// - Edge rendering and drag following
// - Multi-view rendering
//
// This adapter returns fixed semantic data (nodes, sockets, edges)
// that flows through the same pipeline as real document data:
// document => render_model => hit_test => input => renderer

use crate::document::adapter::DocumentAdapter;
use crate::document::commands::{EdgeId, EditorCommand, NodeId, NodePosition, SocketId};
use crate::document::descriptors::{
    EdgeDescriptor, NodeDescriptor, SocketDescriptor, SocketDirection,
};

pub struct TestDocumentAdapter {
    revision: u64,
}

impl TestDocumentAdapter {
    pub fn new() -> Self {
        Self { revision: 0 }
    }

    fn build_test_nodes(&self) -> Vec<NodeDescriptor> {
        vec![
            NodeDescriptor {
                node_id: NodeId(1),
                node_kind: "TestInput".to_string(),
                position: NodePosition { x: 50.0, y: 100.0 },
                inputs: vec![],
                outputs: vec![SocketId(1)],
                diagnostics: vec![],
            },
            NodeDescriptor {
                node_id: NodeId(2),
                node_kind: "TestProcess".to_string(),
                position: NodePosition { x: 300.0, y: 80.0 },
                inputs: vec![SocketId(2)],
                outputs: vec![SocketId(3)],
                diagnostics: vec![],
            },
            NodeDescriptor {
                node_id: NodeId(3),
                node_kind: "TestOutput".to_string(),
                position: NodePosition { x: 550.0, y: 120.0 },
                inputs: vec![SocketId(4)],
                outputs: vec![],
                diagnostics: vec![],
            },
        ]
    }

    fn build_test_sockets(&self) -> Vec<SocketDescriptor> {
        vec![
            SocketDescriptor {
                socket_id: SocketId(1),
                node_id: NodeId(1),
                name: "Output".to_string(),
                direction: SocketDirection::Output,
                type_name: Some("Value".to_string()),
                default_value: None,
                connected_edges: vec![EdgeId(1)],
                diagnostics: vec![],
            },
            SocketDescriptor {
                socket_id: SocketId(2),
                node_id: NodeId(2),
                name: "Input".to_string(),
                direction: SocketDirection::Input,
                type_name: Some("Value".to_string()),
                default_value: None,
                connected_edges: vec![EdgeId(1)],
                diagnostics: vec![],
            },
            SocketDescriptor {
                socket_id: SocketId(3),
                node_id: NodeId(2),
                name: "Output".to_string(),
                direction: SocketDirection::Output,
                type_name: Some("Value".to_string()),
                default_value: None,
                connected_edges: vec![EdgeId(2)],
                diagnostics: vec![],
            },
            SocketDescriptor {
                socket_id: SocketId(4),
                node_id: NodeId(3),
                name: "Input".to_string(),
                direction: SocketDirection::Input,
                type_name: Some("Value".to_string()),
                default_value: None,
                connected_edges: vec![EdgeId(2)],
                diagnostics: vec![],
            },
        ]
    }

    fn build_test_edges(&self) -> Vec<EdgeDescriptor> {
        vec![
            EdgeDescriptor {
                edge_id: EdgeId(1),
                from: SocketId(1),
                to: SocketId(2),
                diagnostics: vec![],
            },
            EdgeDescriptor {
                edge_id: EdgeId(2),
                from: SocketId(3),
                to: SocketId(4),
                diagnostics: vec![],
            },
        ]
    }
}

impl DocumentAdapter for TestDocumentAdapter {
    fn apply_command(&mut self, _command: EditorCommand) {
        self.revision += 1;
    }

    fn document_revision(&self) -> u64 {
        self.revision
    }

    fn nodes(&self) -> &[NodeDescriptor] {
        thread_local! {
            static NODES: Vec<NodeDescriptor> = TestDocumentAdapter::new().build_test_nodes();
        }
        NODES.with(|nodes| unsafe {
            std::mem::transmute::<&[NodeDescriptor], &'static [NodeDescriptor]>(nodes.as_slice())
        })
    }

    fn sockets(&self) -> &[SocketDescriptor] {
        thread_local! {
            static SOCKETS: Vec<SocketDescriptor> = TestDocumentAdapter::new().build_test_sockets();
        }
        SOCKETS.with(|sockets| unsafe {
            std::mem::transmute::<&[SocketDescriptor], &'static [SocketDescriptor]>(
                sockets.as_slice(),
            )
        })
    }

    fn edges(&self) -> &[EdgeDescriptor] {
        thread_local! {
            static EDGES: Vec<EdgeDescriptor> = TestDocumentAdapter::new().build_test_edges();
        }
        EDGES.with(|edges| unsafe {
            std::mem::transmute::<&[EdgeDescriptor], &'static [EdgeDescriptor]>(edges.as_slice())
        })
    }
}
