use thiserror::Error;

use crate::{NodeId, SocketDirection, SocketId, ValueType};

#[derive(Debug, Error)]
pub enum GraphError {
    #[error("Socket {socket:?} not found")]
    SocketNotFound { socket: SocketId },

    #[error("Node {node:?} not found")]
    NodeNotFound { node: NodeId },

    #[error("Socket has wrong direction (expected {expected:?}, got {found:?})")]
    WrongDirection {
        expected: SocketDirection,
        found: SocketDirection,
    },

    #[error("Type mismatch: cannot connect {from:?} to {to:?}")]
    TypeMismatch { from: ValueType, to: ValueType },

    #[error("Input socket already has an incoming connection")]
    InputAlreadyConnected,

    #[error("Graph contains cycles, cannot compute topological order")]
    CycleDetected,

    #[error("Optional input socket {socket:?} missing default value")]
    OptionalInputMissingDefault { socket: SocketId },

    #[error("Default literal type mismatch for socket {socket:?}: expected {expected:?}, found {found:?}")]
    DefaultLiteralTypeMismatch {
        socket: SocketId,
        expected: ValueType,
        found: ValueType,
    },
}
