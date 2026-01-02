use crate::editor::render_model::{RenderEdge, RenderNode};
use crate::editor::renderer::NodeEditorRenderer;
use crate::engine::EditorView;

/// Render layer constants for z-ordering.
/// Lower values are rendered first (behind).
pub mod layers {
    pub const GRID: u8 = 0;
    pub const EDGES: u8 = 1;
    pub const NODES: u8 = 2;
    pub const NODE_TEXT: u8 = 3;
    pub const WIDGETS: u8 = 4;
}

pub fn world_to_screen(point: [f32; 2], view: &EditorView) -> [f32; 2] {
    let pan = view.pan();
    let zoom = view.zoom();

    [
        (point[0] - pan[0]) * zoom + 0.5 * view.width() as f32,
        (point[1] - pan[1]) * zoom + 0.5 * view.height() as f32,
    ]
}

pub fn world_to_screen_size(size: f32, view: &EditorView) -> f32 {
    size * view.zoom()
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
        let color = if node.selected {
            [0.3, 0.3, 0.5, 1.0]
        } else {
            [0.2, 0.2, 0.3, 1.0]
        };
        self.primitive_renderer
            .draw_rect(screen_min, screen_max, color);
    }

    fn draw_edge(&mut self, edge: &RenderEdge, view: &EditorView) {
        let screen_from = world_to_screen(edge.from, view);
        let screen_to = world_to_screen(edge.to, view);
        let color = [0.8, 0.8, 0.8, 1.0];
        self.primitive_renderer
            .draw_line(screen_from, screen_to, color);
    }
}
