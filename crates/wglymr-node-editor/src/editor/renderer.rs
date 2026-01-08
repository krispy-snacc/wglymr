use crate::editor::render_model::{RenderEdge, RenderNode};
use crate::engine::{EditorView, GlobalInteractionState};

pub trait NodeEditorRenderer {
    fn draw_node(&mut self, node: &RenderNode, view: &EditorView, global: &GlobalInteractionState);
    fn draw_edge(&mut self, edge: &RenderEdge, view: &EditorView, global: &GlobalInteractionState, all_nodes: &[RenderNode]);
    fn upload(&mut self, queue: &wgpu::Queue);
}
