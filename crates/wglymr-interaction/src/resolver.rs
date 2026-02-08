use super::map::map_draw_item_to_target;
use super::target::InteractionTarget;
use crate::event::KeyModifiers;
use std::collections::HashSet;
use wglymr_document::{EdgeId, NodeId};
use wglymr_render::DrawItem;
use wglymr_view::ViewVisualState;

/// Centralized resolver for all Blender-style interaction state updates.
/// This is the ONLY place where interaction resolution rules live.
pub struct InteractionResolver;

/// Interaction state that tracks hover, selection, and active elements.
#[derive(Debug, Clone, Default)]
pub struct InteractionState {
    pub hovered: Option<InteractionTarget>,
    pub active: Option<InteractionTarget>,
    pub selected_nodes: HashSet<NodeId>,
    pub selected_edges: HashSet<EdgeId>,
}

impl InteractionState {
    /// Convert to legacy ViewVisualState for compatibility during migration.
    pub fn to_visual_state(&self) -> ViewVisualState {
        let mut state = ViewVisualState::default();

        if let Some(InteractionTarget::Node { node_id }) = self.hovered {
            state.hovered_node = Some(node_id);
        }

        if let Some(InteractionTarget::Socket { socket_id, .. }) = self.hovered {
            state.hovered_socket = Some(socket_id);
        }

        state.selected_nodes = self.selected_nodes.iter().copied().collect();

        if let Some(InteractionTarget::Node { node_id }) = self.active {
            state.active_node = Some(node_id);
        } else if let Some(InteractionTarget::NodeHeader { node_id }) = self.active {
            state.active_node = Some(node_id);
        }

        if let Some(InteractionTarget::Socket { socket_id, .. }) = self.active {
            state.active_socket = Some(socket_id);
        }

        state
    }
}

impl InteractionResolver {
    /// Resolve hover state from hit result.
    /// Hover NEVER changes selection or active state.
    pub fn resolve_hover(hit: Option<&DrawItem>) -> Option<InteractionTarget> {
        hit.map(map_draw_item_to_target)
    }

    /// Resolve full interaction state from mouse down event.
    /// Implements Blender-style selection and active element rules.
    pub fn resolve_mouse_down(
        hit: Option<&DrawItem>,
        modifiers: &KeyModifiers,
        prev_state: &InteractionState,
    ) -> InteractionState {
        let mut state = prev_state.clone();

        let target = hit
            .map(map_draw_item_to_target)
            .unwrap_or(InteractionTarget::None);

        match target {
            InteractionTarget::None => {
                if !modifiers.shift {
                    state.selected_nodes.clear();
                    state.selected_edges.clear();
                    state.active = None;
                }
            }

            InteractionTarget::NodeHeader { node_id } | InteractionTarget::Node { node_id } => {
                if modifiers.shift {
                    if state.selected_nodes.contains(&node_id) {
                        state.selected_nodes.remove(&node_id);
                        if state.active == Some(InteractionTarget::Node { node_id })
                            || state.active == Some(InteractionTarget::NodeHeader { node_id })
                        {
                            state.active = state
                                .selected_nodes
                                .iter()
                                .next()
                                .map(|&id| InteractionTarget::Node { node_id: id });
                        }
                    } else {
                        state.selected_nodes.insert(node_id);
                        state.active = Some(target.clone());
                    }
                } else {
                    state.selected_nodes.clear();
                    state.selected_nodes.insert(node_id);
                    state.active = Some(target.clone());
                }

                if state.selected_nodes.is_empty() {
                    state.active = None;
                }
            }

            InteractionTarget::Socket { node_id, socket_id } => {
                state.active = Some(InteractionTarget::Socket { node_id, socket_id });
            }

            InteractionTarget::Edge { edge_id } => {
                if modifiers.shift {
                    if state.selected_edges.contains(&edge_id) {
                        state.selected_edges.remove(&edge_id);
                    } else {
                        state.selected_edges.insert(edge_id);
                    }
                } else {
                    state.selected_edges.clear();
                    state.selected_edges.insert(edge_id);
                }
                state.active = Some(InteractionTarget::Edge { edge_id });
            }

            InteractionTarget::Overlay { .. } => {}
        }

        state
    }

    /// Finalize interaction on mouse up.
    /// Currently no special logic, but reserved for future drag/box-select completion.
    pub fn resolve_mouse_up(
        _hit: Option<&DrawItem>,
        _prev_state: &InteractionState,
    ) -> InteractionState {
        _prev_state.clone()
    }
}
