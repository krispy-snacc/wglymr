use crate::event::{KeyModifiers, MouseEvent};
use crate::operator::{OperatorContext, OperatorResult};
use crate::operator_stack::OperatorStack;
use crate::operators::box_select::BoxSelectOperator;
use crate::operators::link_drag::LinkDragOperator;
use crate::operators::node_drag::NodeDragOperator;
use crate::operators::node_select::NodeSelectOperator;
use wglymr_render::DrawItem;
use wglymr_view::{EditorView, GlobalInteractionState};

pub struct InputDispatcher {
    last_mouse_world: [f32; 2],
    last_mouse_screen: [f32; 2],
    modifiers: KeyModifiers,
    operator_stack: OperatorStack,
    operator_just_finished: bool,
}

impl Default for InputDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl InputDispatcher {
    pub fn new() -> Self {
        Self {
            last_mouse_world: [0.0, 0.0],
            last_mouse_screen: [0.0, 0.0],
            modifiers: KeyModifiers::default(),
            operator_stack: OperatorStack::new(),
            operator_just_finished: false,
        }
    }

    pub fn set_modifiers(&mut self, modifiers: KeyModifiers) {
        self.modifiers = modifiers;
    }

    fn screen_to_world(&self, screen_pos: [f32; 2], view: &EditorView) -> [f32; 2] {
        let pan = view.pan();
        let zoom = view.zoom();
        let w = view.backing_width() as f32;
        let h = view.backing_height() as f32;

        [
            (screen_pos[0] - 0.5 * w) / zoom + pan[0],
            (screen_pos[1] - 0.5 * h) / zoom + pan[1],
        ]
    }

    fn mouse_coords_normalized(&self, screen_pos: [f32; 2], view: &EditorView) -> [f32; 2] {
        let s = view.backing_scale();
        [(screen_pos[0] * s), (screen_pos[1] * s)]
    }

    pub fn handle_mouse_event(
        &mut self,
        event: MouseEvent,
        view: &mut EditorView,
        draw_items: &[DrawItem],
        global_interaction: &mut GlobalInteractionState,
    ) {
        let norm_screen_pos = self.mouse_coords_normalized(event.screen_pos, view);
        let mouse_world = self.screen_to_world(norm_screen_pos, view);
        self.last_mouse_screen = norm_screen_pos;
        self.last_mouse_world = mouse_world;

        let zoom = view.zoom();
        let view_visual = view.visual_mut();

        let mut ctx = OperatorContext::new(
            view_visual,
            global_interaction,
            draw_items,
            zoom,
            self.modifiers,
            mouse_world,
            norm_screen_pos,
        );

        if !self.operator_stack.has_active() {
            self.operator_stack
                .start(Box::new(NodeSelectOperator::new()), &mut ctx);
        }

        let result = self.operator_stack.handle_event(&event, &mut ctx);

        self.operator_just_finished = false;

        match result {
            OperatorResult::Continue => {}
            OperatorResult::Finished | OperatorResult::Cancelled => {
                self.operator_just_finished = true;
                self.operator_stack
                    .start(Box::new(NodeSelectOperator::new()), &mut ctx);
            }
            OperatorResult::StartDragNodes { node_ids } => {
                self.operator_just_finished = true;
                self.operator_stack
                    .start(Box::new(NodeDragOperator::new(node_ids)), &mut ctx);
            }
            OperatorResult::StartBoxSelect => {
                self.operator_just_finished = true;
                self.operator_stack
                    .start(Box::new(BoxSelectOperator::new(mouse_world)), &mut ctx);
            }
            OperatorResult::StartLinkDrag { from_socket } => {
                self.operator_just_finished = true;
                self.operator_stack
                    .start(Box::new(LinkDragOperator::new(from_socket)), &mut ctx);
            }
        }
    }

    pub fn operator_just_finished(&self) -> bool {
        self.operator_just_finished
    }

    pub fn clear_operator_finished_flag(&mut self) {
        self.operator_just_finished = false;
    }
}
