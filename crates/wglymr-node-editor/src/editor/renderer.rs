use crate::editor::render_model::{RenderEdge, RenderNode};
use crate::editor::text::GlyphRun;
use crate::engine::{EditorView, GlobalInteractionState};

pub trait NodeEditorRenderer {
    fn draw_node(&mut self, node: &RenderNode, view: &EditorView, global: &GlobalInteractionState);
    fn draw_node_geometry(
        &mut self,
        node: &RenderNode,
        view: &EditorView,
        global: &GlobalInteractionState,
    );
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
    fn upload_primitives_for_node(&mut self, queue: &wgpu::Queue);
    fn upload_sdf_for_node(&mut self, queue: &wgpu::Queue);
    fn upload_text_for_node(&mut self, device: &wgpu::Device, queue: &wgpu::Queue);
}

pub trait NodeEditorTextRenderer {
    fn begin_frame(&mut self);
    fn draw_runs(
        &mut self,
        view: &EditorView,
        global: &crate::engine::GlobalInteractionState,
        runs: &[GlyphRun],
    );
    fn draw_text_immediate(
        &mut self,
        text: &str,
        screen_position: [f32; 2],
        font_size_px: f32,
        color: wglymr_color::Color,
    );
    fn upload(&mut self, device: &wgpu::Device, queue: &wgpu::Queue);
    fn render<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>);
}
