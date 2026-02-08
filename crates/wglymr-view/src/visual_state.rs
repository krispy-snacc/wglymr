use wglymr_document::{NodeId, SocketId};
use wglymr_render::DrawItem;

// TODO: ADR-required - NodeDragState is an interaction concern but needed here
// Consider moving to wglymr-interaction and passing as parameter
#[derive(Debug, Clone)]
pub struct NodeDragState {
    pub node_ids: Vec<NodeId>,
    pub drag_delta: [f32; 2],
}

#[derive(Debug, Clone, Default)]
pub struct GlobalInteractionState {
    pub node_drag: Option<NodeDragState>,
}

/// Camera state and resolution. CSS dimensions for layout, backing dimensions for GPU rendering.
#[derive(Debug, Clone, PartialEq)]
pub struct EditorView {
    pan: [f32; 2],
    zoom: f32,

    css_width: u32,
    css_height: u32,

    backing_width: u32,
    backing_height: u32,

    backing_scale: f32,

    visual: ViewVisualState,
    pub draw_items: Vec<DrawItem>,
}

impl Default for EditorView {
    fn default() -> Self {
        Self::new()
    }
}

impl EditorView {
    pub fn new() -> Self {
        Self {
            pan: [0.0, 0.0],
            zoom: 1.0,
            css_width: 800,
            css_height: 600,
            backing_width: 800,
            backing_height: 600,
            backing_scale: 1.0,
            visual: ViewVisualState::default(),
            draw_items: Vec::new(),
        }
    }

    pub fn resize(&mut self, css_width: u32, css_height: u32, backing_scale: f32) {
        self.css_width = css_width;
        self.css_height = css_height;
        self.backing_scale = backing_scale;
        self.backing_width = (css_width as f32 * backing_scale) as u32;
        self.backing_height = (css_height as f32 * backing_scale) as u32;
    }

    pub fn set_camera(&mut self, pan: [f32; 2], zoom: f32) {
        self.pan = pan;
        self.zoom = zoom;
    }

    pub fn pan(&self) -> [f32; 2] {
        self.pan
    }

    pub fn zoom(&self) -> f32 {
        self.zoom
    }

    pub fn css_width(&self) -> u32 {
        self.css_width
    }

    pub fn css_height(&self) -> u32 {
        self.css_height
    }

    pub fn backing_width(&self) -> u32 {
        self.backing_width
    }

    pub fn backing_height(&self) -> u32 {
        self.backing_height
    }

    pub fn backing_scale(&self) -> f32 {
        self.backing_scale
    }

    pub fn visual(&self) -> &ViewVisualState {
        &self.visual
    }

    pub fn visual_mut(&mut self) -> &mut ViewVisualState {
        &mut self.visual
    }
}

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
