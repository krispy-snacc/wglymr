use crate::editor::input::event::MouseEvent;
use crate::editor::input::operator::{EditorOperator, OperatorContext, OperatorResult};

pub struct OperatorStack {
    active: Option<Box<dyn EditorOperator>>,
}

impl OperatorStack {
    pub fn new() -> Self {
        Self { active: None }
    }

    pub fn start(&mut self, operator: Box<dyn EditorOperator>, ctx: &mut OperatorContext) {
        if let Some(mut old) = self.active.take() {
            old.on_exit(ctx);
        }
        
        let mut op = operator;
        op.on_enter(ctx);
        self.active = Some(op);
    }

    pub fn handle_event(&mut self, event: &MouseEvent, ctx: &mut OperatorContext) -> OperatorResult {
        if let Some(operator) = &mut self.active {
            let result = operator.handle_event(event, ctx);
            
            match &result {
                OperatorResult::Continue => {},
                OperatorResult::Finished | OperatorResult::Cancelled => {
                    if let Some(mut op) = self.active.take() {
                        op.on_exit(ctx);
                    }
                }
                OperatorResult::StartDragNodes { .. } 
                | OperatorResult::StartBoxSelect 
                | OperatorResult::StartLinkDrag { .. } => {
                    if let Some(mut op) = self.active.take() {
                        op.on_exit(ctx);
                    }
                }
            }
            
            result
        } else {
            OperatorResult::Continue
        }
    }

    pub fn clear(&mut self, ctx: &mut OperatorContext) {
        if let Some(mut op) = self.active.take() {
            op.on_exit(ctx);
        }
    }

    pub fn has_active(&self) -> bool {
        self.active.is_some()
    }
}

