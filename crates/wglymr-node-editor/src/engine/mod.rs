use std::collections::HashMap;

use crate::document::adapter::DocumentAdapter;
use crate::editor::layout::build_render_model;
use crate::editor::renderer::NodeEditorRenderer;
use crate::editor::wgpu_renderer::WgpuNodeEditorRenderer;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ViewId(String);

impl ViewId {
    pub fn new(id: String) -> Self {
        Self(id)
    }
}

pub struct EditorView {
    pan: [f32; 2],
    zoom: f32,
    width: u32,
    height: u32,
}

impl EditorView {
    pub fn new() -> Self {
        Self {
            pan: [0.0, 0.0],
            zoom: 1.0,
            width: 800,
            height: 600,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub fn set_camera(&mut self, pan: [f32; 2], zoom: f32) {
        self.pan = pan;
        self.zoom = zoom;
    }

    pub fn pan(&self) -> [f32; 2] {
        self.pan
    }

    pub fn zoom(&self) -> f32 {
        self.zoom
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

pub struct EditorEngine {
    views: HashMap<ViewId, EditorView>,
    document: Box<dyn DocumentAdapter>,
}

impl EditorEngine {
    pub fn new(document: Box<dyn DocumentAdapter>) -> Self {
        Self {
            views: HashMap::new(),
            document,
        }
    }

    pub fn create_view(&mut self, view_id: ViewId) {
        self.views.insert(view_id, EditorView::new());
    }

    pub fn has_view(&self, view_id: &ViewId) -> bool {
        self.views.contains_key(view_id)
    }

    pub fn destroy_view(&mut self, view_id: ViewId) {
        self.views.remove(&view_id);
    }

    pub fn resize_view(&mut self, view_id: ViewId, width: u32, height: u32) {
        if let Some(view) = self.views.get_mut(&view_id) {
            view.resize(width, height);
        }
    }

    pub fn set_view_camera(&mut self, view_id: ViewId, pan: [f32; 2], zoom: f32) {
        if let Some(view) = self.views.get_mut(&view_id) {
            view.set_camera(pan, zoom);
        }
    }

    /// Orchestrates rendering for a single view.
    /// Builds render model from document, then submits draw calls via renderer.
    /// Draw order: edges first (background), then nodes (foreground).
    pub fn draw_view(
        &mut self,
        view_id: &ViewId,
        queue: &wgpu::Queue,
        primitive_renderer: &mut wglymr_render_wgpu::PrimitiveRenderer,
    ) {
        let view = match self.views.get(view_id) {
            Some(v) => v,
            None => return,
        };

        let (render_nodes, render_edges) = build_render_model(self.document.as_ref(), view);

        let mut renderer = WgpuNodeEditorRenderer::new(primitive_renderer);

        for edge in &render_edges {
            renderer.draw_edge(edge);
        }

        for node in &render_nodes {
            renderer.draw_node(node);
        }

        primitive_renderer.upload(queue);
    }
}
