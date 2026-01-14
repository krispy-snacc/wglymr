use crate::document::commands::{NodeId, SocketId};
use crate::editor::draw::DrawItem;
use crate::editor::input::event::{KeyModifiers, MouseEvent};
use crate::editor::visual_state::ViewVisualState;
use crate::engine::GlobalInteractionState;

#[derive(Debug, Clone, PartialEq)]
pub enum OperatorResult {
    Continue,
    Finished,
    Cancelled,
    StartDragNodes { node_ids: Vec<NodeId> },
    StartBoxSelect,
    StartLinkDrag { from_socket: SocketId },
}

pub struct OperatorContext<'a> {
    pub view_visual: &'a mut ViewVisualState,
    pub global_interaction: &'a mut GlobalInteractionState,
    pub draw_items: &'a [DrawItem],
    pub zoom: f32,
    pub modifiers: KeyModifiers,
    pub mouse_world: [f32; 2],
    pub mouse_screen: [f32; 2],
}

impl<'a> OperatorContext<'a> {
    pub fn new(
        view_visual: &'a mut ViewVisualState,
        global_interaction: &'a mut GlobalInteractionState,
        draw_items: &'a [DrawItem],
        zoom: f32,
        modifiers: KeyModifiers,
        mouse_world: [f32; 2],
        mouse_screen: [f32; 2],
    ) -> Self {
        Self {
            view_visual,
            global_interaction,
            draw_items,
            zoom,
            modifiers,
            mouse_world,
            mouse_screen,
        }
    }
}

pub trait EditorOperator {
    fn on_enter(&mut self, ctx: &mut OperatorContext);
    fn handle_event(&mut self, event: &MouseEvent, ctx: &mut OperatorContext) -> OperatorResult;
    fn on_exit(&mut self, ctx: &mut OperatorContext);
}
