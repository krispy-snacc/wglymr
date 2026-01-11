use crate::editor::render_model::{RenderEdge, RenderNode};
use crate::engine::{EditorView, GlobalInteractionState};

pub trait NodeEditorRenderer {
    fn draw_node(&mut self, node: &RenderNode, view: &EditorView, global: &GlobalInteractionState);
    fn draw_edge(
        &mut self,
        edge: &RenderEdge,
        view: &EditorView,
        global: &GlobalInteractionState,
        all_nodes: &[RenderNode],
    );
    fn upload(&mut self, device: &wgpu::Device, queue: &wgpu::Queue);
    fn upload_primitives(&mut self, queue: &wgpu::Queue);
    fn upload_sdf(&mut self, queue: &wgpu::Queue);
    fn upload_text(&mut self, device: &wgpu::Device, queue: &wgpu::Queue);
}
