use crate::render_model::{Rect, RenderEdge, RenderNode};
use crate::visual_state::EditorView;

pub fn compute_view_bounds(view: &EditorView) -> Rect {
    let width = view.backing_width() as f32;
    let height = view.backing_height() as f32;
    let pan = view.pan();
    let zoom = view.zoom();

    let half_w = width / (2.0 * zoom);
    let half_h = height / (2.0 * zoom);

    Rect::new(
        [pan[0] - half_w, pan[1] - half_h],
        [pan[0] + half_w, pan[1] + half_h],
    )
}

pub fn is_node_visible(node: &RenderNode, view_bounds: &Rect) -> bool {
    node.bounds.intersects(view_bounds)
}

pub fn is_edge_visible(edge: &RenderEdge, view_bounds: &Rect) -> bool {
    view_bounds.contains_point(edge.from) || view_bounds.contains_point(edge.to)
}
