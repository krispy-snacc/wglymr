use crate::event::{MouseButton, MouseEvent, MouseEventKind};
use crate::operator::{EditorOperator, OperatorContext, OperatorResult};

pub struct BoxSelectOperator {
    start: [f32; 2],
}

impl BoxSelectOperator {
    pub fn new(start: [f32; 2]) -> Self {
        Self { start }
    }
}

impl EditorOperator for BoxSelectOperator {
    fn on_enter(&mut self, ctx: &mut OperatorContext) {
        self.start = ctx.mouse_world;
    }

    fn handle_event(&mut self, event: &MouseEvent, _ctx: &mut OperatorContext) -> OperatorResult {
        match event.kind {
            MouseEventKind::Move => OperatorResult::Continue,
            MouseEventKind::Up(MouseButton::Left) => OperatorResult::Finished,
            MouseEventKind::Down(MouseButton::Right) => OperatorResult::Cancelled,
            _ => OperatorResult::Continue,
        }
    }

    fn on_exit(&mut self, _ctx: &mut OperatorContext) {}
}
