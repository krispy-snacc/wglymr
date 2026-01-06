use crate::editor::render_model::{RenderEdge, RenderNode};
use crate::editor::visual_state::EditorVisualState;
use crate::engine::EditorView;

pub trait NodeEditorRenderer {
    fn draw_node(&mut self, node: &RenderNode, view: &EditorView, visual: &EditorVisualState);
    fn draw_edge(&mut self, edge: &RenderEdge, view: &EditorView, visual: &EditorVisualState, all_nodes: &[RenderNode]);
    fn upload(&mut self, queue: &wgpu::Queue);
}
