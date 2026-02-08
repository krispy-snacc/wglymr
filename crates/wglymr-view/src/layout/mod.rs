mod edge_layout;
mod node_layout;

pub use edge_layout::build_render_edges;
pub use node_layout::{build_render_node, build_socket_position_map, NodeLayoutConstants};

use crate::render_model::{RenderEdge, RenderNode};
use crate::ui::{DefaultNodeUIProvider, NodeUIProvider};
use std::collections::HashMap;
use wglymr_document::DocumentAdapter;

pub fn build_render_model(
    document: &dyn DocumentAdapter,
    constants: &NodeLayoutConstants,
) -> (Vec<RenderNode>, Vec<RenderEdge>) {
    let ui_provider = DefaultNodeUIProvider::default();
    build_render_model_with_provider(document, constants, &ui_provider)
}

pub fn build_render_model_with_provider(
    document: &dyn DocumentAdapter,
    constants: &NodeLayoutConstants,
    ui_provider: &dyn NodeUIProvider,
) -> (Vec<RenderNode>, Vec<RenderEdge>) {
    let nodes = document.nodes();
    let sockets = document.sockets();
    let edges = document.edges();

    let socket_map: HashMap<_, _> = sockets.iter().map(|s| (s.socket_id, s)).collect();

    let render_nodes: Vec<RenderNode> = nodes
        .iter()
        .map(|node| {
            let ui_def = ui_provider.ui_definition(node, &socket_map);
            build_render_node(node, &ui_def, constants)
        })
        .collect();

    let socket_positions = build_socket_position_map(&render_nodes);
    let render_edges = build_render_edges(edges, &socket_positions);

    (render_nodes, render_edges)
}
