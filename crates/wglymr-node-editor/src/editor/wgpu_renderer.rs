use crate::editor::renderer::NodeEditorRenderer;
use crate::editor::view_state::{Camera, RenderEdge, RenderNode};

pub struct WgpuNodeEditorRenderer<'a> {
    queue: &'a wgpu::Queue,
    primitive_renderer: &'a mut wglymr_render_wgpu::PrimitiveRenderer,
    camera: Camera,
}

impl<'a> WgpuNodeEditorRenderer<'a> {
    pub fn new(
        _device: &'a wgpu::Device,
        queue: &'a wgpu::Queue,
        primitive_renderer: &'a mut wglymr_render_wgpu::PrimitiveRenderer,
        camera: Camera,
    ) -> Self {
        Self {
            queue,
            primitive_renderer,
            camera,
        }
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
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
