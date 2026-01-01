use crate::editor::render_model::{RenderEdge, RenderNode};
use crate::editor::renderer::NodeEditorRenderer;
use crate::engine::EditorView;

fn world_to_screen(point: [f32; 2], view: &EditorView) -> [f32; 2] {
    let pan = view.pan();
    let zoom = view.zoom();
    let width = view.width() as f32;
    let height = view.height() as f32;

    [
        (point[0] - pan[0]) * zoom + width / 2.0,
        (point[1] - pan[1]) * zoom + height / 2.0,
    ]
}

pub struct WgpuNodeEditorRenderer<'a> {
    primitive_renderer: &'a mut wglymr_render_wgpu::PrimitiveRenderer,
}

impl<'a> WgpuNodeEditorRenderer<'a> {
    pub fn new(primitive_renderer: &'a mut wglymr_render_wgpu::PrimitiveRenderer) -> Self {
        Self { primitive_renderer }
    }
}

impl<'a> NodeEditorRenderer for WgpuNodeEditorRenderer<'a> {
    fn draw_node(&mut self, node: &RenderNode, view: &EditorView) {
        let screen_min = world_to_screen(node.bounds.min, view);
        let screen_max = world_to_screen(node.bounds.max, view);
        let color = [0.2, 0.2, 0.3, 1.0];
        self.primitive_renderer.draw_rect(screen_min, screen_max, color);
    }

    fn draw_edge(&mut self, edge: &RenderEdge, view: &EditorView) {
        let screen_from = world_to_screen(edge.from, view);
        let screen_to = world_to_screen(edge.to, view);
        let color = [0.8, 0.8, 0.8, 1.0];
        self.primitive_renderer.draw_line(screen_from, screen_to, color);
    }
}
