use crate::event::{MouseButton, MouseEvent, MouseEventKind};
use crate::operator::{EditorOperator, OperatorContext, OperatorResult};
use wglymr_document::SocketId;

pub struct LinkDragOperator {
    from_socket: SocketId,
}

impl LinkDragOperator {
    pub fn new(from_socket: SocketId) -> Self {
        Self { from_socket }
    }
}

impl EditorOperator for LinkDragOperator {
    fn on_enter(&mut self, ctx: &mut OperatorContext) {
        ctx.view_visual.active_socket = Some(self.from_socket);
    }

    fn handle_event(&mut self, event: &MouseEvent, _ctx: &mut OperatorContext) -> OperatorResult {
        match event.kind {
            MouseEventKind::Move => OperatorResult::Continue,
            MouseEventKind::Up(MouseButton::Left) => OperatorResult::Finished,
            MouseEventKind::Down(MouseButton::Right) => OperatorResult::Cancelled,
            _ => OperatorResult::Continue,
        }
    }

    fn on_exit(&mut self, ctx: &mut OperatorContext) {
        ctx.view_visual.active_socket = None;
    }
}
