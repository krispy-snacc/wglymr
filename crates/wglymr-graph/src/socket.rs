use serde::{Deserialize, Serialize};

use crate::NodeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SocketId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SocketDirection {
    Input,
    Output,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValueType {
    Float,
    Vec2,
    Vec3,
    Vec4,
    Bool,
    Int,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Socket {
    pub id: SocketId,
    pub node: NodeId,
    pub direction: SocketDirection,
    pub value_type: ValueType,
    pub name: String,
}
