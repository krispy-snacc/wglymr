use std::collections::HashMap;

use crate::document::adapter::DocumentAdapter;
use crate::editor::culling::{compute_view_bounds, is_edge_visible, is_node_visible};
use crate::editor::layout::{NodeLayoutConstants, build_render_model};
use crate::editor::renderer::NodeEditorRenderer;
use crate::editor::wgpu_renderer::WgpuNodeEditorRenderer;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ViewId(String);

impl ViewId {
    pub fn new(id: String) -> Self {
        Self(id)
    }
}

/// Camera state and resolution. CSS dimensions for layout, backing dimensions for GPU rendering.
pub struct EditorView {
    pan: [f32; 2],
    zoom: f32,

    css_width: u32,
    css_height: u32,

    backing_width: u32,
    backing_height: u32,

    backing_scale: f32,
}

impl EditorView {
    pub fn new() -> Self {
        Self {
            pan: [0.0, 0.0],
            zoom: 1.0,
            css_width: 800,
            css_height: 600,
            backing_width: 800,
            backing_height: 600,
            backing_scale: 1.0,
        }
    }

    /// Update CSS and backing dimensions. Backing scale typically = devicePixelRatio.
    pub fn resize(&mut self, css_width: u32, css_height: u32, backing_scale: f32) {
        self.css_width = css_width;
        self.css_height = css_height;
        self.backing_scale = backing_scale;
        self.backing_width = (css_width as f32 * backing_scale) as u32;
        self.backing_height = (css_height as f32 * backing_scale) as u32;
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

    pub fn css_width(&self) -> u32 {
        self.css_width
    }

    pub fn css_height(&self) -> u32 {
        self.css_height
    }

    /// Actual GPU render resolution. Use for all rendering math.
    pub fn backing_width(&self) -> u32 {
        self.backing_width
    }

    pub fn backing_height(&self) -> u32 {
        self.backing_height
    }

    pub fn backing_scale(&self) -> f32 {
        self.backing_scale
    }

    #[deprecated(note = "Use backing_width() or css_width() explicitly")]
    pub fn width(&self) -> u32 {
        self.backing_width
    }

    #[deprecated(note = "Use backing_height() or css_height() explicitly")]
    pub fn height(&self) -> u32 {
        self.backing_height
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

    pub fn resize_view(
        &mut self,
        view_id: ViewId,
        css_width: u32,
        css_height: u32,
        backing_scale: f32,
    ) {
        if let Some(view) = self.views.get_mut(&view_id) {
            view.resize(css_width, css_height, backing_scale);
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

        let constants = NodeLayoutConstants::default();
        let (render_nodes, render_edges) = build_render_model(self.document.as_ref(), &constants);
        let view_bounds = compute_view_bounds(view);

        let mut renderer = WgpuNodeEditorRenderer::new(primitive_renderer);

        for edge in &render_edges {
            if is_edge_visible(edge, &view_bounds) {
                renderer.draw_edge(edge, view);
            }
        }

        for node in &render_nodes {
            if is_node_visible(node, &view_bounds) {
                renderer.draw_node(node, view);
            }
        }

        primitive_renderer.upload(queue);
    }
}
