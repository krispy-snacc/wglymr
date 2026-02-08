use crate::depth::{resolve_depth, DepthLayer, Z_BODY, Z_HEADER, Z_SOCKET, Z_TEXT};
use crate::hit_layer::HitLayer;
use crate::render_model::{RenderEdge, RenderNode};
use crate::visual_state::{EditorView, GlobalInteractionState};
use wglymr_color::Color;
use wglymr_document::NodeId;
use wglymr_render::{
    CircleDraw, DrawItem, DrawKind, DrawLayer, EntityMetadata, GlyphDraw, LineDraw, RoundedRectDraw,
};

fn get_drag_offset(node_id: NodeId, global: &GlobalInteractionState) -> [f32; 2] {
    if let Some(drag) = &global.node_drag {
        if drag.node_ids.contains(&node_id) {
            return drag.drag_delta;
        }
    }
    [0.0, 0.0]
}

fn get_socket_drag_offset(
    socket_id: wglymr_document::SocketId,
    render_nodes: &[RenderNode],
    global: &GlobalInteractionState,
) -> [f32; 2] {
    for node in render_nodes {
        for socket in &node.input_sockets {
            if socket.socket_id == socket_id {
                return get_drag_offset(node.node_id, global);
            }
        }
        for socket in &node.output_sockets {
            if socket.socket_id == socket_id {
                return get_drag_offset(node.node_id, global);
            }
        }
    }
    [0.0, 0.0]
}

pub fn emit_node_draw_items(
    node: &RenderNode,
    view: &EditorView,
    global: &GlobalInteractionState,
) -> Vec<DrawItem> {
    let mut items = Vec::new();
    let z = node.z_index;
    let offset = get_drag_offset(node.node_id, global);

    let is_active = view.visual().active_node == Some(node.node_id);
    let is_selected = view.visual().selected_nodes.contains(&node.node_id);
    let is_hovered = view.visual().hovered_node == Some(node.node_id);

    let mut body_color = node.colors.body;
    if is_active {
        body_color = body_color.lighten(0.15);
    } else if is_selected {
        body_color = body_color.lighten(0.08);
    } else if is_hovered {
        body_color = body_color.lighten(0.05);
    }

    let body_min = [
        node.body_bounds.min[0] + offset[0],
        node.body_bounds.min[1] + offset[1],
    ];
    let body_max = [
        node.body_bounds.max[0] + offset[0],
        node.body_bounds.max[1] + offset[1],
    ];

    items.push(
        DrawItem::new(
            DrawLayer::NodeBody,
            HitLayer::NodeBody as u8,
            z,
            resolve_depth(node.depth_layer, Z_BODY),
            DrawKind::RoundedRect(RoundedRectDraw {
                position: body_min,
                size: [body_max[0] - body_min[0], body_max[1] - body_min[1]],
                corner_radius: node.corner_radius,
                color: body_color,
            }),
        )
        .with_entity(EntityMetadata::Node(node.node_id.0)),
    );

    let header_min = [
        node.header_bounds.min[0] + offset[0],
        node.header_bounds.min[1] + offset[1],
    ];
    let header_max = [
        node.header_bounds.max[0] + offset[0],
        node.header_bounds.max[1] + offset[1],
    ];

    items.push(
        DrawItem::new(
            DrawLayer::NodeHeader,
            HitLayer::NodeHeader as u8,
            z,
            resolve_depth(node.depth_layer, Z_HEADER),
            DrawKind::RoundedRect(RoundedRectDraw {
                position: header_min,
                size: [header_max[0] - header_min[0], header_max[1] - header_min[1]],
                corner_radius: node.corner_radius,
                color: node.colors.header,
            }),
        )
        .with_entity(EntityMetadata::Node(node.node_id.0)),
    );

    for text_run in &node.text_runs {
        items.push(
            DrawItem::new(
                DrawLayer::NodeText,
                HitLayer::NoHit as u8,
                z,
                resolve_depth(node.depth_layer, Z_TEXT),
                DrawKind::Glyph(GlyphDraw {
                    text: text_run.text.clone(),
                    world_position: [
                        text_run.world_position[0] + offset[0],
                        text_run.world_position[1] + offset[1],
                    ],
                    font_size: text_run.font_size,
                    color: text_run.color,
                }),
            )
            .with_entity(EntityMetadata::Node(node.node_id.0)),
        );
    }

    for socket in &node.input_sockets {
        let center = [socket.center[0] + offset[0], socket.center[1] + offset[1]];
        let is_hovered = view.visual().hovered_socket == Some(socket.socket_id);
        let is_active = view.visual().active_socket == Some(socket.socket_id);

        let mut color = Color::hex(0x88AAFF);
        let mut radius = 6.0;

        if is_active {
            color = Color::hex(0xAADDFF);
            radius *= 1.3;
        } else if is_hovered {
            color = Color::hex(0x99BBFF);
            radius *= 1.15;
        }

        items.push(
            DrawItem::new(
                DrawLayer::NodeSockets,
                HitLayer::NodeSockets as u8,
                z,
                resolve_depth(node.depth_layer, Z_SOCKET),
                DrawKind::Circle(CircleDraw {
                    center,
                    radius,
                    color,
                    filled: true,
                }),
            )
            .with_entity(EntityMetadata::Socket {
                node_id: node.node_id.0,
                socket_id: socket.socket_id.0,
            }),
        );
    }

    for socket in &node.output_sockets {
        let center = [socket.center[0] + offset[0], socket.center[1] + offset[1]];
        let is_hovered = view.visual().hovered_socket == Some(socket.socket_id);
        let is_active = view.visual().active_socket == Some(socket.socket_id);

        let mut color = Color::hex(0xFFAA88);
        let mut radius = 6.0;

        if is_active {
            color = Color::hex(0xFFDDAA);
            radius *= 1.3;
        } else if is_hovered {
            color = Color::hex(0xFFBB99);
            radius *= 1.15;
        }

        items.push(
            DrawItem::new(
                DrawLayer::NodeSockets,
                HitLayer::NodeSockets as u8,
                z,
                resolve_depth(node.depth_layer, Z_SOCKET),
                DrawKind::Circle(CircleDraw {
                    center,
                    radius,
                    color,
                    filled: true,
                }),
            )
            .with_entity(EntityMetadata::Socket {
                node_id: node.node_id.0,
                socket_id: socket.socket_id.0,
            }),
        );
    }

    items
}

pub fn emit_edge_draw_items(
    edge: &RenderEdge,
    render_nodes: &[RenderNode],
    global: &GlobalInteractionState,
) -> Vec<DrawItem> {
    let mut items = Vec::new();

    let from_offset = get_socket_drag_offset(edge.from_socket, render_nodes, global);
    let to_offset = get_socket_drag_offset(edge.to_socket, render_nodes, global);

    let from = [edge.from[0] + from_offset[0], edge.from[1] + from_offset[1]];
    let to = [edge.to[0] + to_offset[0], edge.to[1] + to_offset[1]];

    items.push(
        DrawItem::new(
            DrawLayer::Edges,
            HitLayer::Edges as u8,
            0,
            resolve_depth(DepthLayer::Edges, 0.0),
            DrawKind::Line(LineDraw {
                start: from,
                end: to,
                color: Color::gray(0.8),
                thickness: 2.0,
            }),
        )
        .with_entity(EntityMetadata::Edge(edge.edge_id.0)),
    );

    items
}
