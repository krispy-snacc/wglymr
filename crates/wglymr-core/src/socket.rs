use serde::{Deserialize, Serialize};

use crate::{Literal, NodeId, ValueType};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SocketId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SocketDirection {
    Input,
    Output,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InputSocketConfig {
    pub optional: bool,
    pub default: Option<Literal>,
}

impl InputSocketConfig {
    pub fn required() -> Self {
        Self {
            optional: false,
            default: None,
        }
    }

    pub fn optional_with_default(default: Literal) -> Self {
        Self {
            optional: true,
            default: Some(default),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Socket {
    pub id: SocketId,
    pub node: NodeId,
    pub direction: SocketDirection,
    pub value_type: ValueType,
    pub name: String,
    pub input_config: Option<InputSocketConfig>,
}
