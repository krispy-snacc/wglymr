use crate::editor::hit_test::{HitResult, HitTestContext, NodeRegion, hit_test};
use crate::editor::render_model::{RenderEdge, RenderNode};
use crate::editor::visual_state::{EditorVisualState, InteractionState};
use crate::engine::EditorView;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct KeyModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum MouseEventKind {
    Move,
    Down(MouseButton),
    Up(MouseButton),
    Wheel { delta: f32 },
}

#[derive(Debug, Clone, Copy)]
pub struct MouseEvent {
    pub kind: MouseEventKind,
    pub screen_pos: [f32; 2],
}

pub struct EditorInputHandler {
    last_mouse_world: [f32; 2],
    last_mouse_screen: [f32; 2],
    mouse_down_button: Option<MouseButton>,
    modifiers: KeyModifiers,
}

impl EditorInputHandler {
    pub fn new() -> Self {
        Self {
            last_mouse_world: [0.0, 0.0],
            last_mouse_screen: [0.0, 0.0],
            mouse_down_button: None,
            modifiers: KeyModifiers::default(),
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

    pub fn handle_mouse_event(
        &mut self,
        event: MouseEvent,
        view: &EditorView,
        render_nodes: &[RenderNode],
        render_edges: &[RenderEdge],
        visual_state: &mut EditorVisualState,
    ) {
        let mouse_world = self.screen_to_world(event.screen_pos, view);
        self.last_mouse_screen = event.screen_pos;
        self.last_mouse_world = mouse_world;

        match event.kind {
            MouseEventKind::Move => {
                self.handle_mouse_move(mouse_world, render_nodes, render_edges, visual_state);
            }
            MouseEventKind::Down(button) => {
                self.mouse_down_button = Some(button);
                if button == MouseButton::Left {
                    self.handle_left_mouse_down(
                        mouse_world,
                        render_nodes,
                        render_edges,
                        visual_state,
                    );
                }
            }
            MouseEventKind::Up(button) => {
                if self.mouse_down_button == Some(button) {
                    self.handle_mouse_up(button, visual_state);
                    self.mouse_down_button = None;
                }
            }
            MouseEventKind::Wheel { delta: _ } => {}
        }
    }

    fn handle_mouse_move(
        &mut self,
        mouse_world: [f32; 2],
        render_nodes: &[RenderNode],
        render_edges: &[RenderEdge],
        visual_state: &mut EditorVisualState,
    ) {
        match &visual_state.interaction {
            InteractionState::DraggingNode { node_id } => {
                visual_state.interaction = InteractionState::DraggingNode { node_id: *node_id };
            }
            InteractionState::DraggingLink { from_socket } => {
                visual_state.interaction = InteractionState::DraggingLink {
                    from_socket: *from_socket,
                };
            }
            InteractionState::BoxSelecting { start, current: _ } => {
                visual_state.interaction = InteractionState::BoxSelecting {
                    start: *start,
                    current: mouse_world,
                };
            }
            InteractionState::Idle | InteractionState::Panning => {
                let hit = hit_test(
                    mouse_world,
                    HitTestContext::Hover,
                    render_nodes,
                    render_edges,
                );

                visual_state.hovered_node = None;
                visual_state.hovered_socket = None;
                visual_state.hovered_edge = None;

                match hit {
                    HitResult::Node { node_id, region: _ } => {
                        visual_state.hovered_node = Some(node_id);
                    }
                    HitResult::Socket {
                        socket_id,
                        direction: _,
                    } => {
                        visual_state.hovered_socket = Some(socket_id);
                    }
                    HitResult::Edge { edge_id } => {
                        visual_state.hovered_edge = Some(edge_id);
                    }
                    HitResult::Background => {}
                }
            }
        }
    }

    fn handle_left_mouse_down(
        &mut self,
        mouse_world: [f32; 2],
        render_nodes: &[RenderNode],
        render_edges: &[RenderEdge],
        visual_state: &mut EditorVisualState,
    ) {
        let hit = hit_test(
            mouse_world,
            HitTestContext::Click,
            render_nodes,
            render_edges,
        );

        match hit {
            HitResult::Socket {
                socket_id,
                direction: _,
            } => {
                visual_state.active_socket = Some(socket_id);
                visual_state.interaction = InteractionState::DraggingLink {
                    from_socket: socket_id,
                };
            }
            HitResult::Node { node_id, region } => {
                visual_state.active_node = Some(node_id);

                if !self.modifiers.shift {
                    visual_state.selected_nodes.clear();
                    visual_state.selected_nodes.push(node_id);
                } else {
                    if let Some(pos) = visual_state
                        .selected_nodes
                        .iter()
                        .position(|&id| id == node_id)
                    {
                        visual_state.selected_nodes.remove(pos);
                    } else {
                        visual_state.selected_nodes.push(node_id);
                    }
                }

                if region == NodeRegion::Header {
                    visual_state.interaction = InteractionState::DraggingNode { node_id };
                }
            }
            HitResult::Edge { edge_id } => {
                if !self.modifiers.shift {
                    visual_state.selected_edges.clear();
                    visual_state.selected_edges.push(edge_id);
                } else {
                    if let Some(pos) = visual_state
                        .selected_edges
                        .iter()
                        .position(|&id| id == edge_id)
                    {
                        visual_state.selected_edges.remove(pos);
                    } else {
                        visual_state.selected_edges.push(edge_id);
                    }
                }
            }
            HitResult::Background => {
                if !self.modifiers.shift {
                    visual_state.selected_nodes.clear();
                    visual_state.selected_edges.clear();
                    visual_state.active_node = None;
                }
                visual_state.interaction = InteractionState::BoxSelecting {
                    start: mouse_world,
                    current: mouse_world,
                };
            }
        }
    }

    fn handle_mouse_up(&mut self, _button: MouseButton, visual_state: &mut EditorVisualState) {
        visual_state.interaction = InteractionState::Idle;
    }
}
