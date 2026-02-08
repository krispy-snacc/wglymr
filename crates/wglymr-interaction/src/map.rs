use super::target::InteractionTarget;
use wglymr_document::{EdgeId, NodeId, SocketId};
use wglymr_render::{DrawItem, EntityMetadata};
use wglymr_view::hit_layer::HitLayer;

/// Maps a DrawItem to its semantic InteractionTarget.
/// This is pure mapping based on hit layer and entity metadata.
/// No special-casing by depth or visual state.
pub fn map_draw_item_to_target(item: &DrawItem) -> InteractionTarget {
    // Convert u8 hit_layer back to HitLayer enum
    let hit_layer_enum = match item.hit_layer {
        x if x == HitLayer::NodeHeader as u8 => HitLayer::NodeHeader,
        x if x == HitLayer::NodeBody as u8 => HitLayer::NodeBody,
        x if x == HitLayer::NodeSockets as u8 => HitLayer::NodeSockets,
        x if x == HitLayer::Edge as u8 => HitLayer::Edge,
        x if x == HitLayer::NoHit as u8 => HitLayer::NoHit,
        _ => HitLayer::None,
    };

    match &item.entity {
        EntityMetadata::None => InteractionTarget::None,

        EntityMetadata::Node(node_id) => match hit_layer_enum {
            HitLayer::NodeHeader => InteractionTarget::NodeHeader {
                node_id: NodeId(*node_id),
            },
            HitLayer::NodeBody => InteractionTarget::Node {
                node_id: NodeId(*node_id),
            },
            _ => InteractionTarget::None,
        },

        EntityMetadata::Socket { node_id, socket_id } => InteractionTarget::Socket {
            node_id: NodeId(*node_id),
            socket_id: SocketId(*socket_id),
        },

        EntityMetadata::Edge(edge_id) => InteractionTarget::Edge {
            edge_id: EdgeId(*edge_id),
        },
    }
}
