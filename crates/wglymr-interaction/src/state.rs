use wglymr_document::NodeId;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct NodeDragState {
    pub node_ids: Vec<NodeId>,
    pub start_mouse_world: [f32; 2],
    pub drag_delta: [f32; 2],
    pub start_positions: HashMap<NodeId, [f32; 2]>,
}
