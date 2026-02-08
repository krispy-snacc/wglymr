//! Type lattice for node graph type resolution.
//!
//! Defines value types and their relationships without performing type propagation
//! or graph traversal. This module provides pure type theory for the node system.

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ValueType {
    Float,
    Vec2,
    Vec3,
    Vec4,
    Bool,
    Int,
    Color,
}

impl ValueType {
    /// Check if type is a scalar value
    pub fn is_scalar(&self) -> bool {
        matches!(self, ValueType::Float | ValueType::Bool | ValueType::Int)
    }

    /// Check if type is a vector value
    pub fn is_vector(&self) -> bool {
        matches!(
            self,
            ValueType::Vec2 | ValueType::Vec3 | ValueType::Vec4 | ValueType::Color
        )
    }

    /// Get vector width, None for scalars
    pub fn vector_width(&self) -> Option<u8> {
        match self {
            ValueType::Vec2 => Some(2),
            ValueType::Vec3 => Some(3),
            ValueType::Vec4 | ValueType::Color => Some(4),
            _ => None,
        }
    }
}

/// Check if two types are compatible
///
/// Current rules: only exact matches are compatible.
/// Color is NOT compatible with Vec4.
/// No implicit conversions or promotions.
pub fn are_compatible(a: ValueType, b: ValueType) -> bool {
    a == b
}

/// Unify a set of types into a single type
///
/// Current rules: all types must be identical.
/// Returns error if types differ or input is empty.
pub fn unify(types: &[ValueType]) -> Result<ValueType, TypeError> {
    if types.is_empty() {
        return Err(TypeError::EmptyUnification);
    }

    let first = types[0];
    for &ty in &types[1..] {
        if ty != first {
            return Err(TypeError::Mismatch {
                expected: first,
                found: ty,
            });
        }
    }

    Ok(first)
}

use crate::SocketId;

#[derive(Error, Debug)]
pub enum TypeError {
    #[error("type mismatch: expected {expected:?}, found {found:?}")]
    Mismatch {
        expected: ValueType,
        found: ValueType,
    },

    #[error("cannot unify empty type set")]
    EmptyUnification,

    #[error("optional input socket {socket:?} missing default value")]
    OptionalInputMissingDefault { socket: SocketId },

    #[error("default literal type mismatch for socket {socket:?}: expected {expected:?}, found {found:?}")]
    DefaultLiteralTypeMismatch {
        socket: SocketId,
        expected: ValueType,
        found: ValueType,
    },

    #[error("required input socket {socket:?} is not connected")]
    UnconnectedRequiredInput { socket: SocketId },
}
