mod edge_layout;
mod node_layout;

pub use edge_layout::build_render_edges;
pub use node_layout::{NodeLayoutConstants, build_render_node, build_socket_position_map};

use crate::document::adapter::DocumentAdapter;
use crate::editor::render_model::{RenderEdge, RenderNode};
use std::collections::HashMap;

pub fn build_render_model(
    document: &dyn DocumentAdapter,
    constants: &NodeLayoutConstants,
) -> (Vec<RenderNode>, Vec<RenderEdge>) {
    let nodes = document.nodes();
    let sockets = document.sockets();
    let edges = document.edges();

    let socket_map: HashMap<_, _> = sockets.iter().map(|s| (s.socket_id, s)).collect();

    let render_nodes: Vec<RenderNode> = nodes
        .iter()
        .map(|node| build_render_node(node, &socket_map, constants))
        .collect();

    let socket_positions = build_socket_position_map(&render_nodes);
    let render_edges = build_render_edges(&edges, &socket_positions);

    (render_nodes, render_edges)
}
