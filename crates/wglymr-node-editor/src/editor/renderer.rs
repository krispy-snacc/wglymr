use crate::editor::view_state::{RenderEdge, RenderNode};

pub trait NodeEditorRenderer {
    fn draw_node(&mut self, node: &RenderNode);
    fn draw_edge(&mut self, edge: &RenderEdge);
}

pub fn draw_canvas(
    renderer: &mut dyn NodeEditorRenderer,
    nodes: &[RenderNode],
    edges: &[RenderEdge],
) {
    for edge in edges {
        renderer.draw_edge(edge);
    }

    for node in nodes {
        renderer.draw_node(node);
    }
}
