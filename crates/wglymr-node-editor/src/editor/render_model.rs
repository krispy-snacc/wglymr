use crate::document::commands::{EdgeId, NodeId, SocketId};
use wglymr_color::Color;

use super::ui::SocketVisualType;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub min: [f32; 2],
    pub max: [f32; 2],
}

impl Rect {
    pub fn new(min: [f32; 2], max: [f32; 2]) -> Self {
        Self { min, max }
    }

    pub fn width(&self) -> f32 {
        self.max[0] - self.min[0]
    }

    pub fn height(&self) -> f32 {
        self.max[1] - self.min[1]
    }

    pub fn center(&self) -> [f32; 2] {
        [
            (self.min[0] + self.max[0]) * 0.5,
            (self.min[1] + self.max[1]) * 0.5,
        ]
    }

    pub fn intersects(&self, other: &Rect) -> bool {
        self.min[0] <= other.max[0]
            && self.max[0] >= other.min[0]
            && self.min[1] <= other.max[1]
            && self.max[1] >= other.min[1]
    }

    pub fn contains_point(&self, point: [f32; 2]) -> bool {
        point[0] >= self.min[0]
            && point[0] <= self.max[0]
            && point[1] >= self.min[1]
            && point[1] <= self.max[1]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RenderSocket {
    pub socket_id: SocketId,
    pub center: [f32; 2],
    pub visual: SocketVisualType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NodeColors {
    pub header: Color,
    pub body: Color,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RenderNode {
    pub node_id: NodeId,
    pub bounds: Rect,
    pub header_bounds: Rect,
    pub body_bounds: Rect,
    pub title: String,
    pub title_position: [f32; 2],
    pub colors: NodeColors,
    pub corner_radius: f32,
    pub input_sockets: Vec<RenderSocket>,
    pub output_sockets: Vec<RenderSocket>,
    pub z_index: i32,
    pub text_runs: Vec<crate::editor::text::GlyphRun>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RenderEdge {
    pub edge_id: EdgeId,
    pub from_socket: SocketId,
    pub to_socket: SocketId,
    pub from: [f32; 2],
    pub to: [f32; 2],
}
