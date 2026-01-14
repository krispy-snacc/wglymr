use super::target::InteractionTarget;
use crate::editor::draw::{DrawItem, EntityMetadata};

/// Maps a DrawItem to its semantic InteractionTarget.
/// This is pure mapping based on hit layer and entity metadata.
/// No special-casing by depth or visual state.
pub fn map_draw_item_to_target(item: &DrawItem) -> InteractionTarget {
    use crate::editor::hit::HitLayer;

    match &item.entity {
        EntityMetadata::None => InteractionTarget::None,

        EntityMetadata::Node(node_id) => match item.hit_layer {
            HitLayer::NodeHeader => InteractionTarget::NodeHeader { node_id: *node_id },
            HitLayer::NodeBody => InteractionTarget::Node { node_id: *node_id },
            _ => InteractionTarget::None,
        },

        EntityMetadata::Socket { node_id, socket_id } => InteractionTarget::Socket {
            node_id: *node_id,
            socket_id: *socket_id,
        },

        EntityMetadata::Edge(edge_id) => InteractionTarget::Edge { edge_id: *edge_id },
    }
}
