use wglymr_document::{NodeId, SocketId};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ViewVisualState {
    pub hovered_node: Option<NodeId>,
    pub selected_nodes: Vec<NodeId>,
    pub active_node: Option<NodeId>,
    pub hovered_socket: Option<SocketId>,
    pub active_socket: Option<SocketId>,
}

impl ViewVisualState {
    pub fn clear_selection(&mut self) {
        self.selected_nodes.clear();
        self.active_node = None;
    }

    pub fn select_single_node(&mut self, node_id: NodeId) {
        self.selected_nodes.clear();
        self.selected_nodes.push(node_id);
        self.active_node = Some(node_id);
    }

    pub fn toggle_node_selection(&mut self, node_id: NodeId) {
        if let Some(pos) = self.selected_nodes.iter().position(|&id| id == node_id) {
            self.selected_nodes.remove(pos);
            if self.active_node == Some(node_id) {
                self.active_node = self.selected_nodes.last().copied();
            }
        } else {
            self.selected_nodes.push(node_id);
            self.active_node = Some(node_id);
        }

        if self.selected_nodes.is_empty() {
            self.active_node = None;
        }
    }

    pub fn set_active_node(&mut self, node_id: NodeId) {
        if self.selected_nodes.contains(&node_id) {
            self.active_node = Some(node_id);
        }
    }

    pub fn clear_active_node(&mut self) {
        self.active_node = None;
    }
}
