use crate::event::{MouseButton, MouseEvent, MouseEventKind};
use crate::operator::{EditorOperator, OperatorContext, OperatorResult};
use wglymr_document::NodeId;
use wglymr_view::NodeDragState;

pub struct NodeDragOperator {
    node_ids: Vec<NodeId>,
    start_mouse_world: [f32; 2],
}

impl NodeDragOperator {
    pub fn new(node_ids: Vec<NodeId>) -> Self {
        Self {
            node_ids,
            start_mouse_world: [0.0, 0.0],
        }
    }
}

impl EditorOperator for NodeDragOperator {
    fn on_enter(&mut self, ctx: &mut OperatorContext) {
        self.start_mouse_world = ctx.mouse_world;
        ctx.global_interaction.node_drag = Some(NodeDragState {
            node_ids: self.node_ids.clone(),
            drag_delta: [0.0, 0.0],
        });
    }

    fn handle_event(&mut self, event: &MouseEvent, ctx: &mut OperatorContext) -> OperatorResult {
        match event.kind {
            MouseEventKind::Move => {
                if let Some(drag) = &mut ctx.global_interaction.node_drag {
                    drag.drag_delta = [
                        ctx.mouse_world[0] - self.start_mouse_world[0],
                        ctx.mouse_world[1] - self.start_mouse_world[1],
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
