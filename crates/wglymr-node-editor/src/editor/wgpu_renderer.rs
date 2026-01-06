use crate::editor::render_model::{RenderEdge, RenderNode};
use crate::editor::renderer::NodeEditorRenderer;
use crate::editor::visual_state::{EditorVisualState, InteractionState};
use crate::engine::EditorView;
use crate::document::commands::NodeId;
use wglymr_color::Color;
use wglymr_render_wgpu::{GlyphonTextRenderer, PrimitiveRenderer, RoundedRect, SdfRenderer};

pub mod layers {
    pub const GRID: u8 = 0;
    pub const EDGES: u8 = 1;
    pub const NODE_BODY: u8 = 2;
    pub const NODE_HEADER: u8 = 3;
    pub const NODE_SOCKETS: u8 = 4;
    pub const NODE_TEXT: u8 = 5;
    pub const WIDGETS: u8 = 6;
}

fn get_drag_offset(node_id: NodeId, visual: &EditorVisualState) -> [f32; 2] {
    if let InteractionState::DraggingNodes { node_ids, drag } = &visual.interaction {
        if node_ids.contains(&node_id) {
            return drag.drag_delta;
        }
    }
    [0.0, 0.0]
}

fn get_socket_drag_offset(socket_id: crate::document::commands::SocketId, render_nodes: &[RenderNode], visual: &EditorVisualState) -> [f32; 2] {
    for node in render_nodes {
        let socket_in_node = node.input_sockets.iter().any(|s| s.socket_id == socket_id)
            || node.output_sockets.iter().any(|s| s.socket_id == socket_id);
        
        if socket_in_node {
            return get_drag_offset(node.node_id, visual);
        }
    }
    [0.0, 0.0]
}

pub fn world_to_screen(point: [f32; 2], view: &EditorView) -> [f32; 2] {
    let pan = view.pan();
    let zoom = view.zoom();
    let w = view.backing_width() as f32;
    let h = view.backing_height() as f32;

    [
        (point[0] - pan[0]) * zoom + 0.5 * w,
        (point[1] - pan[1]) * zoom + 0.5 * h,
    ]
}

pub fn world_to_screen_size(size: f32, view: &EditorView) -> f32 {
    size * view.zoom()
}

pub struct WgpuNodeEditorRenderer<'a> {
    primitive_renderer: &'a mut PrimitiveRenderer,
    sdf_renderer: Option<&'a mut SdfRenderer>,
    text_renderer: Option<&'a mut GlyphonTextRenderer>,
}

impl<'a> WgpuNodeEditorRenderer<'a> {
    pub fn new(primitive_renderer: &'a mut PrimitiveRenderer) -> Self {
        Self {
            primitive_renderer,
            sdf_renderer: None,
            text_renderer: None,
        }
    }

    pub fn with_sdf_renderer(mut self, sdf_renderer: &'a mut SdfRenderer) -> Self {
        self.sdf_renderer = Some(sdf_renderer);
        self
    }

    pub fn with_text_renderer(mut self, text_renderer: &'a mut GlyphonTextRenderer) -> Self {
        self.text_renderer = Some(text_renderer);
        self
    }

    fn draw_node_body(&mut self, node: &RenderNode, view: &EditorView, visual: &EditorVisualState) {
        if let Some(sdf) = &mut self.sdf_renderer {
            let offset = get_drag_offset(node.node_id, visual);
            let min = [node.body_bounds.min[0] + offset[0], node.body_bounds.min[1] + offset[1]];
            let max = [node.body_bounds.max[0] + offset[0], node.body_bounds.max[1] + offset[1]];
            
            let screen_min = world_to_screen(min, view);
            let screen_max = world_to_screen(max, view);
            let radius = world_to_screen_size(node.corner_radius, view);

            sdf.set_layer(layers::NODE_BODY);

            let is_active = visual.active_node == Some(node.node_id);
            let is_selected = visual.selected_nodes.contains(&node.node_id);
            let is_hovered = visual.hovered_node == Some(node.node_id);

            let mut body_color = node.colors.body;
            let mut border_width = 0.0;
            let mut border_color = Color::BLACK;

            if is_active {
                body_color = body_color.lighten(0.15);
                border_width = world_to_screen_size(2.0, view);
                border_color = Color::hex(0x5588FF);
            } else if is_selected {
                body_color = body_color.lighten(0.08);
                border_width = world_to_screen_size(1.5, view);
                border_color = Color::hex(0xFF8800);
            } else if is_hovered {
                body_color = body_color.lighten(0.05);
            }

            let rect = RoundedRect::new(screen_min, screen_max)
                .with_radius(radius)
                .with_fill_color(body_color)
                .with_border(border_width, border_color);
            sdf.draw_rounded_rect(&rect);
        }
    }

    fn draw_node_header(
        &mut self,
        node: &RenderNode,
        view: &EditorView,
        visual: &EditorVisualState,
    ) {
        if let Some(sdf) = &mut self.sdf_renderer {
            let offset = get_drag_offset(node.node_id, visual);
            let min = [node.header_bounds.min[0] + offset[0], node.header_bounds.min[1] + offset[1]];
            let max = [node.header_bounds.max[0] + offset[0], node.header_bounds.max[1] + offset[1]];
            
            let screen_min = world_to_screen(min, view);
            let screen_max = world_to_screen(max, view);
            let radius = world_to_screen_size(node.corner_radius, view);

            sdf.set_layer(layers::NODE_HEADER);

            let is_active = visual.active_node == Some(node.node_id);
            let is_selected = visual.selected_nodes.contains(&node.node_id);
            let is_hovered = visual.hovered_node == Some(node.node_id);

            let mut header_color = node.colors.header;

            if is_active {
                header_color = header_color.lighten(0.2);
            } else if is_selected {
                header_color = header_color.lighten(0.12);
            } else if is_hovered {
                header_color = header_color.lighten(0.06);
            }

            let rect = RoundedRect::new(screen_min, screen_max)
                .with_radius(radius)
                .with_fill_color(header_color);
            sdf.draw_rounded_rect(&rect);
        }
    }

    fn draw_node_title(&mut self, node: &RenderNode, view: &EditorView, visual: &EditorVisualState) {
        if let Some(text) = &mut self.text_renderer {
            let offset = get_drag_offset(node.node_id, visual);
            let pos = [node.title_position[0] + offset[0], node.title_position[1] + offset[1]];
            
            let screen_pos = world_to_screen(pos, view);
            let font_size = world_to_screen_size(14.0, view);

            text.draw_text(
                &node.title,
                screen_pos,
                font_size,
                Color::WHITE,
                layers::NODE_TEXT,
            );
        }
    }

    fn draw_sockets(&mut self, node: &RenderNode, view: &EditorView, visual: &EditorVisualState) {
        let base_socket_radius = world_to_screen_size(6.0, view);
        let offset = get_drag_offset(node.node_id, visual);

        for socket in &node.input_sockets {
            let center = [socket.center[0] + offset[0], socket.center[1] + offset[1]];
            let screen_center = world_to_screen(center, view);
            let is_hovered = visual.hovered_socket == Some(socket.socket_id);
            let is_active = visual.active_socket == Some(socket.socket_id);

            let mut color = Color::hex(0x88AAFF);
            let mut radius = base_socket_radius;

            if is_active {
                color = Color::hex(0xAADDFF);
                radius *= 1.3;
            } else if is_hovered {
                color = Color::hex(0x99BBFF);
                radius *= 1.15;
            }

            self.draw_socket_shape(screen_center, radius, color, is_hovered || is_active);
        }

        for socket in &node.output_sockets {
            let center = [socket.center[0] + offset[0], socket.center[1] + offset[1]];
            let screen_center = world_to_screen(center, view);
            let is_hovered = visual.hovered_socket == Some(socket.socket_id);
            let is_active = visual.active_socket == Some(socket.socket_id);

            let mut color = Color::hex(0xFFAA88);
            let mut radius = base_socket_radius;

            if is_active {
                color = Color::hex(0xFFDDAA);
                radius *= 1.3;
            } else if is_hovered {
                color = Color::hex(0xFFBB99);
                radius *= 1.15;
            }

            self.draw_socket_shape(screen_center, radius, color, is_hovered || is_active);
        }
    }

    fn draw_socket_shape(
        &mut self,
        center: [f32; 2],
        radius: f32,
        color: Color,
        highlighted: bool,
    ) {
        if let Some(sdf) = &mut self.sdf_renderer {
            sdf.set_layer(layers::NODE_SOCKETS);
            let min = [center[0] - radius, center[1] - radius];
            let max = [center[0] + radius, center[1] + radius];

            if highlighted {
                let glow_radius = radius * 1.5;
                let glow_min = [center[0] - glow_radius, center[1] - glow_radius];
                let glow_max = [center[0] + glow_radius, center[1] + glow_radius];
                let glow_color = color.with_alpha(0.3);
                let glow_rect = RoundedRect::new(glow_min, glow_max)
                    .with_radius(glow_radius)
                    .with_fill_color(glow_color);
                sdf.draw_rounded_rect(&glow_rect);
            }

            let rect = RoundedRect::new(min, max)
                .with_radius(radius)
                .with_fill_color(color);
            sdf.draw_rounded_rect(&rect);
        }
    }
}

impl<'a> NodeEditorRenderer for WgpuNodeEditorRenderer<'a> {
    fn draw_node(&mut self, node: &RenderNode, view: &EditorView, visual: &EditorVisualState) {
        self.draw_node_body(node, view, visual);
        self.draw_node_header(node, view, visual);
        self.draw_sockets(node, view, visual);
        self.draw_node_title(node, view, visual);
    }

    fn draw_edge(&mut self, edge: &RenderEdge, view: &EditorView, visual: &EditorVisualState, all_nodes: &[RenderNode]) {
        let from_offset = get_socket_drag_offset(edge.from_socket, all_nodes, visual);
        let to_offset = get_socket_drag_offset(edge.to_socket, all_nodes, visual);
        
        let from = [edge.from[0] + from_offset[0], edge.from[1] + from_offset[1]];
        let to = [edge.to[0] + to_offset[0], edge.to[1] + to_offset[1]];
        
        let screen_from = world_to_screen(from, view);
        let screen_to = world_to_screen(to, view);

        let is_hovered = visual.hovered_edge == Some(edge.edge_id);
        let is_selected = visual.selected_edges.contains(&edge.edge_id);

        let color = if is_selected {
            Color::hex(0xFFDD88)
        } else if is_hovered {
            Color::hex(0xCCCCCC)
        } else {
            Color::gray(0.8)
        };

        self.primitive_renderer
            .draw_line(screen_from, screen_to, color);
    }

    fn upload(&mut self, queue: &wgpu::Queue) {
        if let Some(sdf) = &mut self.sdf_renderer {
            sdf.finish_batch();
            sdf.upload(queue);
        }
        if let Some(text) = &mut self.text_renderer {
            text.finish_batch();
        }
        self.primitive_renderer.upload(queue);
    }
}
