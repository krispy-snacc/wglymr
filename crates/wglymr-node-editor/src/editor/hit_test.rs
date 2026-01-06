use crate::document::commands::{EdgeId, NodeId, SocketId};
use crate::document::descriptors::SocketDirection;
use crate::editor::render_model::{Rect, RenderEdge, RenderNode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HitTestContext {
    Hover,
    Click,
    Drag,
    BoxSelect,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HitResult {
    Node {
        node_id: NodeId,
        region: NodeRegion,
    },
    Socket {
        socket_id: SocketId,
        direction: SocketDirection,
    },
    Edge {
        edge_id: EdgeId,
    },
    Background,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeRegion {
    Header,
    Body,
}

fn point_in_rect(point: [f32; 2], rect: &Rect) -> bool {
    point[0] >= rect.min[0]
        && point[0] <= rect.max[0]
        && point[1] >= rect.min[1]
        && point[1] <= rect.max[1]
}

fn distance_squared(a: [f32; 2], b: [f32; 2]) -> f32 {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    dx * dx + dy * dy
}

fn distance_point_to_segment(p: [f32; 2], a: [f32; 2], b: [f32; 2]) -> f32 {
    let dx = b[0] - a[0];
    let dy = b[1] - a[1];
    let length_squared = dx * dx + dy * dy;

    if length_squared < 1e-8 {
        return distance_squared(p, a).sqrt();
    }

    let t = ((p[0] - a[0]) * dx + (p[1] - a[1]) * dy) / length_squared;
    let t = t.clamp(0.0, 1.0);

    let closest = [a[0] + t * dx, a[1] + t * dy];
    distance_squared(p, closest).sqrt()
}

fn hit_test_sockets(mouse_world: [f32; 2], nodes: &[RenderNode]) -> Option<HitResult> {
    let socket_radius = 6.0;
    let radius_squared = socket_radius * socket_radius;

    let mut closest_socket: Option<(SocketId, SocketDirection, f32)> = None;

    for node in nodes {
        for socket in &node.input_sockets {
            let dist_sq = distance_squared(mouse_world, socket.center);
            if dist_sq <= radius_squared {
                if let Some((_, _, current_dist)) = closest_socket {
                    if dist_sq < current_dist {
                        closest_socket = Some((socket.socket_id, SocketDirection::Input, dist_sq));
                    }
                } else {
                    closest_socket = Some((socket.socket_id, SocketDirection::Input, dist_sq));
                }
            }
        }

        for socket in &node.output_sockets {
            let dist_sq = distance_squared(mouse_world, socket.center);
            if dist_sq <= radius_squared {
                if let Some((_, _, current_dist)) = closest_socket {
                    if dist_sq < current_dist {
                        closest_socket = Some((socket.socket_id, SocketDirection::Output, dist_sq));
                    }
                } else {
                    closest_socket = Some((socket.socket_id, SocketDirection::Output, dist_sq));
                }
            }
        }
    }

    closest_socket.map(|(socket_id, direction, _)| HitResult::Socket {
        socket_id,
        direction,
    })
}

fn hit_test_nodes(mouse_world: [f32; 2], nodes: &[RenderNode]) -> Option<HitResult> {
    for node in nodes.iter().rev() {
        if point_in_rect(mouse_world, &node.header_bounds) {
            return Some(HitResult::Node {
                node_id: node.node_id,
                region: NodeRegion::Header,
            });
        }

        if point_in_rect(mouse_world, &node.body_bounds) {
            return Some(HitResult::Node {
                node_id: node.node_id,
                region: NodeRegion::Body,
            });
        }
    }

    None
}

fn hit_test_edges(mouse_world: [f32; 2], edges: &[RenderEdge]) -> Option<HitResult> {
    let tolerance = 5.0;

    for edge in edges {
        let distance = distance_point_to_segment(mouse_world, edge.from, edge.to);
        if distance <= tolerance {
            return Some(HitResult::Edge {
                edge_id: edge.edge_id,
            });
        }
    }

    None
}

pub fn hit_test(
    mouse_world: [f32; 2],
    context: HitTestContext,
    nodes: &[RenderNode],
    edges: &[RenderEdge],
) -> HitResult {
    if context != HitTestContext::BoxSelect {
        if let Some(result) = hit_test_sockets(mouse_world, nodes) {
            return result;
        }
    }

    if let Some(result) = hit_test_nodes(mouse_world, nodes) {
        return result;
    }

    if context != HitTestContext::BoxSelect {
        if let Some(result) = hit_test_edges(mouse_world, edges) {
            return result;
        }
    }

    HitResult::Background
}
