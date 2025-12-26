use crate::editor::view_state::{RenderEdge, RenderNode};

pub trait NodeEditorRenderer {
    fn draw_node(&mut self, node: &RenderNode);
    fn draw_edge(&mut self, edge: &RenderEdge);
}
