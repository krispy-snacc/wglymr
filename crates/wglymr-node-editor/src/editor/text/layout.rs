use crate::editor::render_model::RenderNode;
use crate::editor::text::{GlyphRun, TextStyle};
use crate::engine::EditorView;

pub fn layout_node_title(node: &RenderNode, _view: &EditorView, z_index: i32) -> GlyphRun {
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

pub fn collect_node_title_text(view: &mut EditorView, node: &RenderNode, z_index: i32) {
    let run = layout_node_title(node, view, z_index);
    view.text_runs.push(run);
}
