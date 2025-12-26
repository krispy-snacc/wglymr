use crate::document::adapter::DocumentAdapter;
use crate::document::descriptors::{EdgeDescriptor, NodeDescriptor, SocketDescriptor};
use crate::editor::view_state::{Rect, RenderEdge, RenderNode, RenderSocket};
use crate::engine::EditorView;
use std::collections::HashMap;

const NODE_WIDTH: f32 = 200.0;
const NODE_HEADER_HEIGHT: f32 = 30.0;
const SOCKET_HEIGHT: f32 = 24.0;
const SOCKET_SPACING: f32 = 8.0;
const NODE_PADDING: f32 = 10.0;

/// Transforms a point from document-space to pixel-space.
/// (0,0) in document space maps to viewport center when pan=[0,0].
fn doc_to_pixel(point: [f32; 2], view: &EditorView) -> [f32; 2] {
    let pan = view.pan();
    let zoom = view.zoom();
    let width = view.width() as f32;
    let height = view.height() as f32;

    let centered_x = (point[0] - pan[0]) * zoom + width / 2.0;
    let centered_y = (point[1] - pan[1]) * zoom + height / 2.0;

    [centered_x, centered_y]
}

/// Converts all coordinates from document-space to pixel-space.
/// Applies camera transforms (pan, zoom) and viewport centering.
/// Pure function: no GPU allocation, no state mutation.
pub fn build_render_model(
    document: &dyn DocumentAdapter,
    view: &EditorView,
) -> (Vec<RenderNode>, Vec<RenderEdge>) {
    let nodes = document.nodes();
    let sockets = document.sockets();
    let edges = document.edges();

    let socket_map: HashMap<_, _> = sockets.iter().map(|s| (s.socket_id, s)).collect();

    let render_nodes: Vec<RenderNode> = nodes
        .iter()
        .map(|node| build_render_node(node, &socket_map, &view))
        .collect();

    let socket_positions = build_socket_position_map(&render_nodes);

    let render_edges: Vec<RenderEdge> = edges
        .iter()
        .filter_map(|edge| build_render_edge(edge, &socket_positions))
        .collect();

    (render_nodes, render_edges)
}

fn build_render_node(
    node: &NodeDescriptor,
    socket_map: &HashMap<crate::document::commands::SocketId, &SocketDescriptor>,
    view: &EditorView,
) -> RenderNode {
    let input_count = node.inputs.len();
    let output_count = node.outputs.len();
    let max_sockets = input_count.max(output_count);

    let doc_height = NODE_HEADER_HEIGHT
        + NODE_PADDING * 2.0
        + max_sockets as f32 * SOCKET_HEIGHT
        + (max_sockets.saturating_sub(1)) as f32 * SOCKET_SPACING;

    let doc_min = [node.position.x, node.position.y];
    let doc_max = [node.position.x + NODE_WIDTH, node.position.y + doc_height];

    let rect = Rect {
        min: doc_to_pixel(doc_min, &view),
        max: doc_to_pixel(doc_max, &view),
    };

    let socket_start_y = node.position.y + NODE_HEADER_HEIGHT + NODE_PADDING;

    let input_sockets = node
        .inputs
        .iter()
        .enumerate()
        .filter_map(|(i, socket_id)| {
            socket_map.get(socket_id).map(|_| {
                let y = socket_start_y
                    + i as f32 * (SOCKET_HEIGHT + SOCKET_SPACING)
                    + SOCKET_HEIGHT / 2.0;
                let doc_pos = [node.position.x, y];
                RenderSocket {
                    socket_id: *socket_id,
                    center: doc_to_pixel(doc_pos, &view),
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
                    + i as f32 * (SOCKET_HEIGHT + SOCKET_SPACING)
                    + SOCKET_HEIGHT / 2.0;
                let doc_pos = [node.position.x + NODE_WIDTH, y];
                RenderSocket {
                    socket_id: *socket_id,
                    center: doc_to_pixel(doc_pos, &view),
                }
            })
        })
        .collect();

    RenderNode {
        node_id: node.node_id,
        rect,
        input_sockets,
        output_sockets,
    }
}

fn build_socket_position_map(
    render_nodes: &[RenderNode],
) -> HashMap<crate::document::commands::SocketId, [f32; 2]> {
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

fn build_render_edge(
    edge: &EdgeDescriptor,
    socket_positions: &HashMap<crate::document::commands::SocketId, [f32; 2]>,
) -> Option<RenderEdge> {
    let from = *socket_positions.get(&edge.from)?;
    let to = *socket_positions.get(&edge.to)?;

    Some(RenderEdge {
        edge_id: edge.edge_id,
        from,
        to,
    })
}
