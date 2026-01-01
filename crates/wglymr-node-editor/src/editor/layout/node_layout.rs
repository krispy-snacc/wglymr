use crate::document::commands::SocketId;
use crate::document::descriptors::{NodeDescriptor, SocketDescriptor};
use crate::editor::render_model::{Rect, RenderNode, RenderSocket};
use std::collections::HashMap;

pub struct NodeLayoutConstants {
    pub width: f32,
    pub header_height: f32,
    pub socket_height: f32,
    pub socket_spacing: f32,
    pub padding: f32,
}

impl Default for NodeLayoutConstants {
    fn default() -> Self {
        Self {
            width: 200.0,
            header_height: 30.0,
            socket_height: 24.0,
            socket_spacing: 8.0,
            padding: 10.0,
        }
    }
}

pub fn build_render_node(
    node: &NodeDescriptor,
    socket_map: &HashMap<SocketId, &SocketDescriptor>,
    constants: &NodeLayoutConstants,
) -> RenderNode {
    let input_count = node.inputs.len();
    let output_count = node.outputs.len();
    let max_sockets = input_count.max(output_count);

    let height = constants.header_height
        + constants.padding * 2.0
        + max_sockets as f32 * constants.socket_height
        + (max_sockets.saturating_sub(1)) as f32 * constants.socket_spacing;

    let bounds = Rect::new(
        [node.position.x, node.position.y],
        [node.position.x + constants.width, node.position.y + height],
    );

    let socket_start_y = node.position.y + constants.header_height + constants.padding;

    let input_sockets = node
        .inputs
        .iter()
        .enumerate()
        .filter_map(|(i, socket_id)| {
            socket_map.get(socket_id).map(|_| {
                let y = socket_start_y
                    + i as f32 * (constants.socket_height + constants.socket_spacing)
                    + constants.socket_height / 2.0;
                RenderSocket {
                    socket_id: *socket_id,
                    center: [node.position.x, y],
                }
            })
        })
        .collect();

    let output_sockets = node
        .outputs
        .iter()
        .enumerate()
        .filter_map(|(i, socket_id)| {
            socket_map.get(socket_id).map(|_| {
                let y = socket_start_y
                    + i as f32 * (constants.socket_height + constants.socket_spacing)
                    + constants.socket_height / 2.0;
                RenderSocket {
                    socket_id: *socket_id,
                    center: [node.position.x + constants.width, y],
                }
            })
        })
        .collect();

    let title_bounds = Rect::new(
        [
            node.position.x + constants.padding,
            node.position.y + constants.padding,
        ],
        [
            node.position.x + constants.width - constants.padding,
            node.position.y + constants.header_height,
        ],
    );

    RenderNode {
        node_id: node.node_id,
        bounds,
        title: node.node_kind.clone(),
        title_bounds,
        input_sockets,
        output_sockets,
        selected: false,
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
