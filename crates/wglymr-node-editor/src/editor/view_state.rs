use crate::document::commands::{EdgeId, NodeId, SocketId};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera {
    pub pan: [f32; 2],
    pub zoom: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rect {
    pub min: [f32; 2],
    pub max: [f32; 2],
}

#[derive(Debug, Clone, PartialEq)]
pub struct RenderSocket {
    pub socket_id: SocketId,
    pub center: [f32; 2],
}

#[derive(Debug, Clone, PartialEq)]
pub struct RenderNode {
    pub node_id: NodeId,
    pub rect: Rect,
    pub input_sockets: Vec<RenderSocket>,
    pub output_sockets: Vec<RenderSocket>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RenderEdge {
    pub edge_id: EdgeId,
    pub from: [f32; 2],
    pub to: [f32; 2],
}
