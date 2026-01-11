use crate::document::commands::SocketId;
use crate::document::descriptors::NodeDescriptor;
use crate::editor::render_model::{NodeColors, Rect, RenderNode, RenderSocket};
use crate::editor::ui::NodeUIDefinition;
use std::collections::HashMap;

pub struct NodeLayoutConstants {
    pub width: f32,
    pub socket_height: f32,
    pub socket_spacing: f32,
}

impl Default for NodeLayoutConstants {
    fn default() -> Self {
        Self {
            width: 200.0,
            socket_height: 24.0,
            socket_spacing: 8.0,
        }
    }
}

pub fn build_render_node(
    node: &NodeDescriptor,
    ui_def: &NodeUIDefinition,
    constants: &NodeLayoutConstants,
) -> RenderNode {
    let input_count = ui_def.inputs.len();
    let output_count = ui_def.outputs.len();
    let max_sockets = input_count.max(output_count);

    let header_height = ui_def.header.height;
    let padding = ui_def.body.padding;

    let body_content_height = max_sockets as f32 * constants.socket_height
        + (max_sockets.saturating_sub(1)) as f32 * constants.socket_spacing;

    let height = header_height + padding * 2.0 + body_content_height;

    let bounds = Rect::new(
        [node.position.x, node.position.y],
        [node.position.x + constants.width, node.position.y + height],
    );

    let header_bounds = Rect::new(
        [node.position.x, node.position.y],
        [
            node.position.x + constants.width,
            node.position.y + header_height,
        ],
    );

    let body_bounds = Rect::new(
        [node.position.x, node.position.y + header_height],
        [node.position.x + constants.width, node.position.y + height],
    );

    let socket_start_y = node.position.y + header_height + padding;

    let input_sockets = ui_def
        .inputs
        .iter()
        .enumerate()
        .map(|(i, socket_ui)| {
            let y = socket_start_y
                + i as f32 * (constants.socket_height + constants.socket_spacing)
                + constants.socket_height / 2.0;
            RenderSocket {
                socket_id: socket_ui.id,
                center: [node.position.x, y],
                visual: socket_ui.visual,
            }
        })
        .collect();

    let output_sockets = ui_def
        .outputs
        .iter()
        .enumerate()
        .map(|(i, socket_ui)| {
            let y = socket_start_y
                + i as f32 * (constants.socket_height + constants.socket_spacing)
                + constants.socket_height / 2.0;
            RenderSocket {
                socket_id: socket_ui.id,
                center: [node.position.x + constants.width, y],
                visual: socket_ui.visual,
            }
        })
        .collect();

    let title_position = [node.position.x + padding, node.position.y + padding];

    let colors = NodeColors {
        header: ui_def.header.color,
        body: ui_def.body.background,
    };

    RenderNode {
        node_id: node.node_id,
        bounds,
        header_bounds,
        body_bounds,
        title: ui_def.header.title.clone(),
        title_position,
        colors,
        corner_radius: ui_def.body.corner_radius,
        input_sockets,
        output_sockets,
        z_index: 0,
    }
}

pub fn build_socket_position_map(render_nodes: &[RenderNode]) -> HashMap<SocketId, [f32; 2]> {
    let mut map = HashMap::new();

    for node in render_nodes {
        for socket in &node.input_sockets {
            map.insert(socket.socket_id, socket.center);
        }
        for socket in &node.output_sockets {
            map.insert(socket.socket_id, socket.center);
        }
    }

    map
}
