use std::collections::HashMap;

use crate::document::commands::NodeId;
use crate::editor::input::event::{MouseButton, MouseEventKind};
use crate::editor::input::operator::{EditorOperator, OperatorContext, OperatorResult};
use crate::editor::input::state::NodeDragState;

pub struct NodeDragOperator {
    node_ids: Vec<NodeId>,
}

impl NodeDragOperator {
    pub fn new(node_ids: Vec<NodeId>) -> Self {
        Self { node_ids }
    }
}

impl EditorOperator for NodeDragOperator {
    fn on_enter(&mut self, ctx: &mut OperatorContext) {
        let mut start_positions = HashMap::new();
        for &id in &self.node_ids {
            if let Some(node) = ctx.render_nodes.iter().find(|n| n.node_id == id) {
                start_positions.insert(id, [node.bounds.min[0], node.bounds.min[1]]);
            }
        }

        ctx.global_interaction.node_drag = Some(NodeDragState {
            node_ids: self.node_ids.clone(),
            start_mouse_world: ctx.mouse_world,
            drag_delta: [0.0, 0.0],
            start_positions,
        });
    }

    fn handle_event(
        &mut self,
        event: &crate::editor::input::event::MouseEvent,
        ctx: &mut OperatorContext,
    ) -> OperatorResult {
        match event.kind {
            MouseEventKind::Move => {
                if let Some(drag) = &mut ctx.global_interaction.node_drag {
                    drag.drag_delta = [
                        ctx.mouse_world[0] - drag.start_mouse_world[0],
                        ctx.mouse_world[1] - drag.start_mouse_world[1],
                    ];
                }
                OperatorResult::Continue
            }
            MouseEventKind::Up(MouseButton::Left) => OperatorResult::Finished,
            MouseEventKind::Down(MouseButton::Right) => OperatorResult::Cancelled,
            _ => OperatorResult::Continue,
        }
    }

    fn on_exit(&mut self, ctx: &mut OperatorContext) {
        ctx.global_interaction.node_drag = None;
    }
}
