use crate::editor::renderer::NodeEditorRenderer;
use crate::editor::view_state::{RenderEdge, RenderNode};

/// WGPU-backed renderer for node editor primitives.
/// Receives pixel-space coordinates only.
/// Camera transforms are applied on CPU before rendering.
pub struct WgpuNodeEditorRenderer<'a> {
    primitive_renderer: &'a mut wglymr_render_wgpu::PrimitiveRenderer,
}

impl<'a> WgpuNodeEditorRenderer<'a> {
    pub fn new(primitive_renderer: &'a mut wglymr_render_wgpu::PrimitiveRenderer) -> Self {
        Self { primitive_renderer }
    }
}

impl<'a> NodeEditorRenderer for WgpuNodeEditorRenderer<'a> {
    fn draw_node(&mut self, node: &RenderNode) {
        let color = [0.2, 0.2, 0.3, 1.0];
        self.primitive_renderer
            .draw_rect(node.rect.min, node.rect.max, color);
    }

    fn draw_edge(&mut self, edge: &RenderEdge) {
        let color = [0.8, 0.8, 0.8, 1.0];
        self.primitive_renderer.draw_line(edge.from, edge.to, color);
    }
}
