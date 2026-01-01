use crate::editor::render_model::{RenderEdge, RenderNode};
use crate::engine::EditorView;

pub trait NodeEditorRenderer {
    fn draw_node(&mut self, node: &RenderNode, view: &EditorView);
    fn draw_edge(&mut self, edge: &RenderEdge, view: &EditorView);
}
