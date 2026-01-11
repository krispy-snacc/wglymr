use crate::editor::input::event::{MouseButton, MouseEvent, MouseEventKind};
use crate::editor::input::hit_test::{HitResult, HitTestContext, NodeRegion, hit_test};
use crate::editor::input::operator::{EditorOperator, OperatorContext, OperatorResult};

const DRAG_THRESHOLD_WORLD: f32 = 3.0;

#[derive(Clone)]
struct ClickCandidate {
    hit: HitResult,
    start_mouse_world: [f32; 2],
    was_selected: bool,
}

pub struct NodeSelectOperator {
    click: Option<ClickCandidate>,
}

impl NodeSelectOperator {
    pub fn new() -> Self {
        Self { click: None }
    }
}

impl EditorOperator for NodeSelectOperator {
    fn on_enter(&mut self, _ctx: &mut OperatorContext) {
        self.click = None;
    }

    fn handle_event(&mut self, event: &MouseEvent, ctx: &mut OperatorContext) -> OperatorResult {
        match event.kind {
            MouseEventKind::Leave(button) => match button {
                MouseButton::Left => self.handle_left_mouse_leave(ctx),
                MouseButton::Right => self.handle_right_mouse_leave(ctx),
                _ => OperatorResult::Continue,
            },

            MouseEventKind::Enter(button) => match button {
                // MouseButton::Left => self.handle_left_mouse_enter(ctx),
                // MouseButton::Right => self.handle_right_mouse_enter(ctx),
                _ => OperatorResult::Continue,
            },

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
        self.click = None;
    }
}

impl NodeSelectOperator {
    fn update_hover(&self, ctx: &mut OperatorContext) {
        let hit = hit_test(
            ctx.mouse_world,
            HitTestContext::Hover,
            ctx.render_nodes,
            ctx.render_edges,
            ctx.zoom,
        );

        ctx.view_visual.hovered_node = None;
        ctx.view_visual.hovered_socket = None;

        match hit {
            HitResult::Node { node_id, .. } => {
                ctx.view_visual.hovered_node = Some(node_id);
            }
            HitResult::Socket { socket_id, .. } => {
                ctx.view_visual.hovered_socket = Some(socket_id);
            }
            HitResult::Edge { .. } | HitResult::Background => {}
        }
    }

    fn handle_left_mouse_down(&mut self, ctx: &mut OperatorContext) -> OperatorResult {
        let hit = hit_test(
            ctx.mouse_world,
            HitTestContext::Click,
            ctx.render_nodes,
            ctx.render_edges,
            ctx.zoom,
        );

        if let HitResult::Socket { socket_id, .. } = hit {
            return OperatorResult::StartLinkDrag {
                from_socket: socket_id,
            };
        }

        let was_selected = if let HitResult::Node { node_id, .. } = &hit {
            ctx.view_visual.selected_nodes.contains(node_id)
        } else {
            false
        };

        self.click = Some(ClickCandidate {
            hit: hit.clone(),
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
            if let HitResult::Node { node_id, region } = &click.hit {
                if *region == NodeRegion::Header {
                    let node_ids = if click.was_selected {
                        ctx.view_visual.selected_nodes.clone()
                    } else {
                        vec![*node_id]
                    };

                    self.click = None;
                    return OperatorResult::StartDragNodes { node_ids };
                }
            }

            self.click = None;
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
        match &click.hit {
            HitResult::Node { node_id, .. } => {
                if ctx.modifiers.shift {
                    ctx.view_visual.toggle_node_selection(*node_id);
                } else {
                    ctx.view_visual.select_single_node(*node_id);
                }
            }
            HitResult::Background => {
                if !ctx.modifiers.shift {
                    ctx.view_visual.clear_selection();
                }
            }
            HitResult::Edge { .. } => {}
            HitResult::Socket { .. } => {}
        }
    }
}
