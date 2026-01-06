use std::collections::HashMap;

use crate::document::adapter::DocumentAdapter;
use crate::editor::culling::{compute_view_bounds, is_edge_visible, is_node_visible};
use crate::editor::input::{EditorInputHandler, KeyModifiers, MouseEvent, MouseEventKind};
use crate::editor::layout::{NodeLayoutConstants, build_render_model};
use crate::editor::renderer::NodeEditorRenderer;
use crate::editor::visual_state::EditorVisualState;
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
    visual_state: EditorVisualState,
    input_handler: EditorInputHandler,
}

impl EditorEngine {
    pub fn new(document: Box<dyn DocumentAdapter>) -> Self {
        Self {
            views: HashMap::new(),
            document,
            visual_state: EditorVisualState::default(),
            input_handler: EditorInputHandler::new(),
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

    pub fn visual_state(&self) -> &EditorVisualState {
        &self.visual_state
    }

    pub fn draw_view(
        &mut self,
        view_id: &ViewId,
        queue: &wgpu::Queue,
        primitive_renderer: &mut wglymr_render_wgpu::PrimitiveRenderer,
        sdf_renderer: Option<&mut wglymr_render_wgpu::SdfRenderer>,
        text_renderer: Option<&mut wglymr_render_wgpu::GlyphonTextRenderer>,
    ) {
        let view = match self.views.get(view_id) {
            Some(v) => v,
            None => return,
        };

        let constants = NodeLayoutConstants::default();
        let (render_nodes, render_edges) = build_render_model(self.document.as_ref(), &constants);
        let view_bounds = compute_view_bounds(view);

        let simulated_mouse_screen = [400.0, 300.0];
        let simulated_move_event = MouseEvent {
            kind: MouseEventKind::Move,
            screen_pos: simulated_mouse_screen,
        };

        self.input_handler.set_modifiers(KeyModifiers {
            shift: false,
            ctrl: false,
            alt: false,
        });

        self.input_handler.handle_mouse_event(
            simulated_move_event,
            view,
            &render_nodes,
            &render_edges,
            &mut self.visual_state,
        );

        if render_nodes.len() > 1 {
            self.visual_state.selected_nodes = vec![render_nodes[1].node_id];
            self.visual_state.active_node = Some(render_nodes[1].node_id);
        }

        let mut renderer = WgpuNodeEditorRenderer::new(primitive_renderer);

        if let Some(sdf) = sdf_renderer {
            sdf.begin_frame();
            renderer = renderer.with_sdf_renderer(sdf);
        }

        if let Some(text) = text_renderer {
            text.begin_frame();
            renderer = renderer.with_text_renderer(text);
        }

        for edge in &render_edges {
            if is_edge_visible(edge, &view_bounds) {
                renderer.draw_edge(edge, view, &self.visual_state);
            }
        }

        for node in &render_nodes {
            if is_node_visible(node, &view_bounds) {
                renderer.draw_node(node, view, &self.visual_state);
            }
        }

        renderer.upload(queue);
    }
}
