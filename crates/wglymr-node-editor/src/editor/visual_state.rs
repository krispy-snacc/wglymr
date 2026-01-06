use crate::document::commands::{EdgeId, NodeId, SocketId};

#[derive(Debug, Clone)]
pub struct EditorVisualState {
    pub hovered_node: Option<NodeId>,
    pub selected_nodes: Vec<NodeId>,
    pub active_node: Option<NodeId>,

    pub hovered_socket: Option<SocketId>,
    pub active_socket: Option<SocketId>,

    pub hovered_edge: Option<EdgeId>,
    pub selected_edges: Vec<EdgeId>,

    pub interaction: InteractionState,
}

#[derive(Debug, Clone)]
pub enum InteractionState {
    Idle,
    Panning,
    BoxSelecting { start: [f32; 2], current: [f32; 2] },
    DraggingNode { node_id: NodeId },
    DraggingLink { from_socket: SocketId },
}

impl Default for EditorVisualState {
    fn default() -> Self {
        Self {
            hovered_node: None,
            selected_nodes: Vec::new(),
            active_node: None,
            hovered_socket: None,
            active_socket: None,
            hovered_edge: None,
            selected_edges: Vec::new(),
            interaction: InteractionState::Idle,
        }
    }
}
