use std::collections::HashMap;

use crate::document::adapter::DocumentAdapter;
use crate::editor::culling::{compute_view_bounds, is_edge_visible, is_node_visible};
use crate::editor::input::{InputDispatcher, KeyModifiers, MouseEvent, NodeDragState};
use crate::editor::layout::{NodeLayoutConstants, build_render_model};
use crate::editor::renderer::NodeEditorRenderer;
use crate::editor::text::GlyphRun;
use crate::editor::visual_state::ViewVisualState;
use crate::editor::wgpu_renderer::WgpuNodeEditorRenderer;

#[derive(Debug, Clone)]
pub struct GlobalInteractionState {
    pub node_drag: Option<NodeDragState>,
}

impl Default for GlobalInteractionState {
    fn default() -> Self {
        Self { node_drag: None }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ViewId(String);

impl ViewId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Camera state and resolution. CSS dimensions for layout, backing dimensions for GPU rendering.
#[derive(Debug, Clone, PartialEq)]
pub struct EditorView {
    pan: [f32; 2],
    zoom: f32,

    css_width: u32,
    css_height: u32,

    backing_width: u32,
    backing_height: u32,

    backing_scale: f32,

    visual: ViewVisualState,
    pub text_runs: Vec<GlyphRun>,
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
            visual: ViewVisualState::default(),
            text_runs: Vec::new(),
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

    pub fn visual(&self) -> &ViewVisualState {
        &self.visual
    }

    pub fn visual_mut(&mut self) -> &mut ViewVisualState {
        &mut self.visual
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
    global_interaction: GlobalInteractionState,
    input_handler: InputDispatcher,
}

impl EditorEngine {
    pub fn new(document: Box<dyn DocumentAdapter>) -> Self {
        Self {
            views: HashMap::new(),
            document,
            global_interaction: GlobalInteractionState::default(),
            input_handler: InputDispatcher::new(),
        }
    }

    pub fn create_view(&mut self, view_id: ViewId) {
        self.views.insert(view_id, EditorView::new());
    }

    pub fn get_view(&self, view_id: &ViewId) -> Option<&EditorView> {
        self.views.get(view_id)
    }

    pub fn has_view(&self, view_id: &ViewId) -> bool {
        self.views.contains_key(view_id)
    }

    pub fn destroy_view(&mut self, view_id: &ViewId) {
        self.views.remove(view_id);
    }

    pub fn resize_view(
        &mut self,
        view_id: &ViewId,
        css_width: u32,
        css_height: u32,
        backing_width: u32,
        backing_height: u32,
    ) {
        if let Some(view) = self.views.get_mut(view_id) {
            view.css_width = css_width;
            view.css_height = css_height;
            view.backing_width = backing_width;
            view.backing_height = backing_height;
            view.backing_scale = backing_width as f32 / css_width as f32;
        }
    }

    pub fn set_view_camera(&mut self, view_id: &ViewId, x: f32, y: f32, zoom: f32) {
        if let Some(view) = self.views.get_mut(view_id) {
            view.set_camera([x, y], zoom);
        }
    }

    pub fn global_interaction(&self) -> &GlobalInteractionState {
        &self.global_interaction
    }

    pub fn global_interaction_mut(&mut self) -> &mut GlobalInteractionState {
        &mut self.global_interaction
    }

    pub fn is_modal_active(&self) -> bool {
        self.global_interaction.node_drag.is_some()
    }

    pub fn operator_just_finished(&self) -> bool {
        self.input_handler.operator_just_finished()
    }

    pub fn clear_operator_finished_flag(&mut self) {
        self.input_handler.clear_operator_finished_flag()
    }

    pub fn handle_mouse_event(
        &mut self,
        view_id: &ViewId,
        event: MouseEvent,
        modifiers: KeyModifiers,
    ) {
        let view = match self.views.get_mut(view_id) {
            Some(v) => v,
            None => return,
        };

        let constants = NodeLayoutConstants::default();
        let (render_nodes, render_edges) = build_render_model(self.document.as_ref(), &constants);

        self.input_handler.set_modifiers(modifiers);
        self.input_handler.handle_mouse_event(
            event,
            view,
            &render_nodes,
            &render_edges,
            &mut self.global_interaction,
        );
    }

    pub fn draw_view(
        &mut self,
        view_id: &ViewId,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        primitive_renderer: &mut wglymr_render_wgpu::PrimitiveRenderer,
        sdf_renderer: Option<&mut wglymr_render_wgpu::SdfRenderer>,
        text_renderer: Option<&mut dyn crate::editor::renderer::NodeEditorTextRenderer>,
    ) {
        let view = match self.views.get_mut(view_id) {
            Some(v) => v,
            None => return,
        };

        view.text_runs.clear();

        let constants = NodeLayoutConstants::default();
        let (mut render_nodes, render_edges) =
            build_render_model(self.document.as_ref(), &constants);

        for node in &mut render_nodes {
            let mut z = 0;

            if view.visual().selected_nodes.contains(&node.node_id) {
                z += 100;
            }

            if view.visual().active_node == Some(node.node_id) {
                z += 200;
            }

            if let Some(drag) = &self.global_interaction.node_drag {
                if drag.node_ids.contains(&node.node_id) {
                    z += 1000;
                }
            }

            node.z_index = z;

            let text_run = crate::editor::text::layout_node_title(node, view, z);
            node.text_runs.push(text_run);
        }

        render_nodes.sort_by_key(|n| n.z_index);

        let view_bounds = compute_view_bounds(view);

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
                renderer.draw_edge(edge, view, &self.global_interaction, &render_nodes);
            }
        }

        renderer.upload_primitives(queue);

        for node in &render_nodes {
            if is_node_visible(node, &view_bounds) {
                renderer.draw_node(node, view, &self.global_interaction);
                renderer.upload_primitives_for_node(queue);
                renderer.upload_sdf_for_node(queue);
                renderer.upload_text_for_node(device, queue);
            }
        }
    }
}
