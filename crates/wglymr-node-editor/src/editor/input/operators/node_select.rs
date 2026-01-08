use crate::document::commands::NodeId;
use crate::editor::input::event::{MouseButton, MouseEvent, MouseEventKind};
use crate::editor::input::hit_test::{HitResult, HitTestContext, NodeRegion, hit_test};
use crate::editor::input::operator::{EditorOperator, OperatorContext, OperatorResult};

pub struct NodeSelectOperator;

impl EditorOperator for NodeSelectOperator {
    fn on_enter(&mut self, _ctx: &mut OperatorContext) {}

    fn handle_event(&mut self, event: &MouseEvent, ctx: &mut OperatorContext) -> OperatorResult {
        match event.kind {
            MouseEventKind::Move => {
                self.update_hover(ctx);
                OperatorResult::Continue
            }
            MouseEventKind::Down(button) => {
                if button == MouseButton::Left {
                    self.handle_left_mouse_down(ctx)
                } else {
                    OperatorResult::Continue
                }
            }
            MouseEventKind::Up(_) | MouseEventKind::Wheel { .. } => OperatorResult::Continue,
        }
    }

    fn on_exit(&mut self, _ctx: &mut OperatorContext) {}
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

    fn handle_left_mouse_down(&self, ctx: &mut OperatorContext) -> OperatorResult {
        let hit = hit_test(
            ctx.mouse_world,
            HitTestContext::Click,
            ctx.render_nodes,
            ctx.render_edges,
            ctx.zoom,
        );

        match hit {
            HitResult::Socket { socket_id, .. } => {
                OperatorResult::StartLinkDrag { from_socket: socket_id }
            }
            HitResult::Node { node_id, region } => {
                self.handle_node_selection(ctx, node_id, region)
            }
            HitResult::Background => {
                self.handle_background_click(ctx)
            }
            HitResult::Edge { .. } => {
                OperatorResult::Continue
            }
        }
    }

    fn handle_node_selection(&self, ctx: &mut OperatorContext, node_id: NodeId, region: NodeRegion) -> OperatorResult {
        if !ctx.modifiers.shift {
            ctx.view_visual.select_single_node(node_id);
        } else {
            ctx.view_visual.toggle_node_selection(node_id);
        }

        if region == NodeRegion::Header {
            let node_ids = if ctx.view_visual.selected_nodes.contains(&node_id) {
                ctx.view_visual.selected_nodes.clone()
            } else {
                vec![node_id]
            };
            OperatorResult::StartDragNodes { node_ids }
        } else {
            OperatorResult::Finished
        }
    }

    fn handle_background_click(&self, ctx: &mut OperatorContext) -> OperatorResult {
        if !ctx.modifiers.shift {
            ctx.view_visual.clear_selection();
        }
        OperatorResult::StartBoxSelect
    }
}

