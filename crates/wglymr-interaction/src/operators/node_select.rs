use crate::event::{MouseButton, MouseEvent, MouseEventKind};
use crate::operator::{EditorOperator, OperatorContext, OperatorResult};
use crate::{map_draw_item_to_target, InteractionTarget};
use wglymr_view::hit_test;

// TODO: Replace with proper logging facade in Phase 4
mod logging {
    #[allow(unused)]
    pub fn log(_msg: &str) {}
    #[allow(unused)]
    pub fn debug(_msg: &str) {}
}

const DRAG_THRESHOLD_WORLD: f32 = 3.0;

#[derive(Clone)]
struct ClickCandidate {
    target: InteractionTarget,
    start_mouse_world: [f32; 2],
    was_selected: bool,
}

pub struct NodeSelectOperator {
    click: Option<ClickCandidate>,
}

impl Default for NodeSelectOperator {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeSelectOperator {
    pub fn new() -> Self {
        Self { click: None }
    }
}

impl EditorOperator for NodeSelectOperator {
    fn on_enter(&mut self, _ctx: &mut OperatorContext) {
        logging::log("[Operator] NodeSelectOperator::on_enter");
        self.click = None;
    }

    fn handle_event(&mut self, event: &MouseEvent, ctx: &mut OperatorContext) -> OperatorResult {
        match event.kind {
            MouseEventKind::Leave(button) => match button {
                MouseButton::Left => self.handle_left_mouse_leave(ctx),
                MouseButton::Right => self.handle_right_mouse_leave(ctx),
                _ => OperatorResult::Continue,
            },

            MouseEventKind::Enter(_button) => OperatorResult::Continue,

            MouseEventKind::Move => {
                if self.click.is_some() {
                    let click = self.click.clone().unwrap();
                    self.handle_mouse_move_with_click(ctx, &click)
                } else {
                    self.update_hover(ctx);
                    OperatorResult::Continue
                }
            }
            MouseEventKind::Down(button) => match button {
                MouseButton::Left => self.handle_left_mouse_down(ctx),
                MouseButton::Right => OperatorResult::Cancelled,
                _ => OperatorResult::Continue,
            },
            MouseEventKind::Up(button) => {
                if button == MouseButton::Left {
                    self.handle_left_mouse_up(ctx)
                } else {
                    OperatorResult::Continue
                }
            }
            MouseEventKind::Wheel { .. } => OperatorResult::Continue,
        }
    }

    fn on_exit(&mut self, _ctx: &mut OperatorContext) {
        logging::log("[Operator] NodeSelectOperator::on_exit");
        self.click = None;
    }
}

impl NodeSelectOperator {
    fn update_hover(&self, ctx: &mut OperatorContext) {
        let hit_result = hit_test(ctx.mouse_world, ctx.draw_items);

        logging::debug(&format!(
            "[Hover] mouse_world: [{:.2}, {:.2}], draw_items: {}, hit: {}",
            ctx.mouse_world[0],
            ctx.mouse_world[1],
            ctx.draw_items.len(),
            if hit_result.is_some() { "YES" } else { "NO" }
        ));

        let target = hit_result
            .and_then(|h| {
                logging::debug(&format!(
                    "[Hover] hit_result: item_index={}, hit_layer={:?}, depth={:.4}",
                    h.item_index, h.hit_layer, h.depth
                ));
                ctx.draw_items.get(h.item_index)
            })
            .map(|item| {
                let t = map_draw_item_to_target(item);
                logging::debug(&format!("[Hover] mapped to target: {:?}", t));
                t
            })
            .unwrap_or(InteractionTarget::None);

        ctx.view_visual.hovered_node = None;
        ctx.view_visual.hovered_socket = None;

        match target {
            InteractionTarget::Node { node_id } | InteractionTarget::NodeHeader { node_id } => {
                ctx.view_visual.hovered_node = Some(node_id);
            }
            InteractionTarget::Socket { socket_id, .. } => {
                ctx.view_visual.hovered_socket = Some(socket_id);
            }
            InteractionTarget::Edge { .. }
            | InteractionTarget::None
            | InteractionTarget::Overlay { .. } => {}
        }
    }

    fn handle_left_mouse_down(&mut self, ctx: &mut OperatorContext) -> OperatorResult {
        logging::log(&format!(
            "[MouseDown] at world [{:.2}, {:.2}], draw_items: {}",
            ctx.mouse_world[0],
            ctx.mouse_world[1],
            ctx.draw_items.len()
        ));

        let hit_result = hit_test(ctx.mouse_world, ctx.draw_items);
        let target = hit_result
            .and_then(|h| {
                logging::log(&format!(
                    "[MouseDown] hit: item_index={}, hit_layer={:?}",
                    h.item_index, h.hit_layer
                ));
                ctx.draw_items.get(h.item_index)
            })
            .map(|item| {
                let t = map_draw_item_to_target(item);
                logging::log(&format!("[MouseDown] target: {:?}", t));
                t
            })
            .unwrap_or(InteractionTarget::None);

        if let InteractionTarget::Socket { socket_id, .. } = target {
            logging::log(&format!(
                "[MouseDown] Starting link drag from socket {:?}",
                socket_id
            ));
            return OperatorResult::StartLinkDrag {
                from_socket: socket_id,
            };
        }

        let was_selected = if let InteractionTarget::Node { node_id }
        | InteractionTarget::NodeHeader { node_id } = &target
        {
            ctx.view_visual.selected_nodes.contains(node_id)
        } else {
            false
        };

        self.click = Some(ClickCandidate {
            target: target.clone(),
            start_mouse_world: ctx.mouse_world,
            was_selected,
        });

        OperatorResult::Continue
    }

    fn handle_mouse_move_with_click(
        &mut self,
        ctx: &mut OperatorContext,
        click: &ClickCandidate,
    ) -> OperatorResult {
        let dx = ctx.mouse_world[0] - click.start_mouse_world[0];
        let dy = ctx.mouse_world[1] - click.start_mouse_world[1];
        let distance = (dx * dx + dy * dy).sqrt();

        if distance > DRAG_THRESHOLD_WORLD {
            if let InteractionTarget::NodeHeader { node_id } = &click.target {
                logging::log(&format!(
                    "[Drag] Starting node drag: distance={:.2}, node_id={:?}, was_selected={}",
                    distance, node_id, click.was_selected
                ));

                let node_ids = if click.was_selected {
                    ctx.view_visual.selected_nodes.clone()
                } else {
                    vec![*node_id]
                };

                self.click = None;
                return OperatorResult::StartDragNodes { node_ids };
            }
        }

        OperatorResult::Continue
    }

    fn handle_left_mouse_up(&mut self, ctx: &mut OperatorContext) -> OperatorResult {
        if let Some(click) = self.click.take() {
            self.apply_selection(ctx, &click);
        }

        OperatorResult::Finished
    }

    fn handle_left_mouse_leave(&mut self, ctx: &mut OperatorContext) -> OperatorResult {
        ctx.view_visual.hovered_node = None;
        self.click = None;
        OperatorResult::Continue
    }

    fn handle_right_mouse_leave(&mut self, ctx: &mut OperatorContext) -> OperatorResult {
        ctx.view_visual.hovered_node = None;
        self.click = None;
        OperatorResult::Continue
    }

    fn apply_selection(&self, ctx: &mut OperatorContext, click: &ClickCandidate) {
        match &click.target {
            InteractionTarget::Node { node_id } | InteractionTarget::NodeHeader { node_id } => {
                if ctx.modifiers.shift {
                    ctx.view_visual.toggle_node_selection(*node_id);
                } else {
                    ctx.view_visual.select_single_node(*node_id);
                }
            }
            InteractionTarget::None => {
                if !ctx.modifiers.shift {
                    ctx.view_visual.clear_selection();
                }
            }
            InteractionTarget::Edge { .. }
            | InteractionTarget::Socket { .. }
            | InteractionTarget::Overlay { .. } => {}
        }
    }
}
