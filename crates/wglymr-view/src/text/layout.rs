use crate::render_model::RenderNode;
use crate::text::{GlyphRun, TextStyle};

pub fn layout_node_title(node: &RenderNode, z_index: i32) -> GlyphRun {
    let style = TextStyle::node_title();

    GlyphRun {
        text: node.title.clone(),
        world_position: node.title_position,
        font_size: style.font_size,
        color: style.color,
        z_index,
        node_id: Some(node.node_id),
    }
}
