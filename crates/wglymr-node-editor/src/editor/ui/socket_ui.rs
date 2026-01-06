use crate::document::commands::SocketId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketVisualType {
    Circle,
    Diamond,
    Square,
    Triangle,
}

impl Default for SocketVisualType {
    fn default() -> Self {
        Self::Circle
    }
}

#[derive(Debug, Clone)]
pub struct SocketUIDefinition {
    pub id: SocketId,
    pub label: String,
    pub visual: SocketVisualType,
}

impl SocketUIDefinition {
    pub fn new(id: SocketId, label: String, visual: SocketVisualType) -> Self {
        Self { id, label, visual }
    }
}
