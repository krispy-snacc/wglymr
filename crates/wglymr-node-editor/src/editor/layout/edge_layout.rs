use crate::document::commands::SocketId;
use crate::document::descriptors::EdgeDescriptor;
use crate::editor::render_model::RenderEdge;
use std::collections::HashMap;

pub fn build_render_edges(
    edges: &[EdgeDescriptor],
    socket_positions: &HashMap<SocketId, [f32; 2]>,
) -> Vec<RenderEdge> {
    edges
        .iter()
        .filter_map(|edge| build_render_edge(edge, socket_positions))
        .collect()
}

fn build_render_edge(
    edge: &EdgeDescriptor,
    socket_positions: &HashMap<SocketId, [f32; 2]>,
) -> Option<RenderEdge> {
    let from = *socket_positions.get(&edge.from)?;
    let to = *socket_positions.get(&edge.to)?;

    Some(RenderEdge {
        edge_id: edge.edge_id,
        from_socket: edge.from,
        to_socket: edge.to,
        from,
        to,
    })
}
