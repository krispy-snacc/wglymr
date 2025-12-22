use serde::{Deserialize, Serialize};

use crate::ValueType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValueId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IrType {
    Float,
    Vec2,
    Vec3,
    Vec4,
    Color,
    Bool,
    Int,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Literal {
    Float(f32),
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4([f32; 4]),
    Bool(bool),
    Int(i32),
}

impl Literal {
    pub fn value_type(&self) -> ValueType {
        match self {
            Literal::Float(_) => ValueType::Float,
            Literal::Vec2(_) => ValueType::Vec2,
            Literal::Vec3(_) => ValueType::Vec3,
            Literal::Vec4(_) => ValueType::Vec4,
            Literal::Bool(_) => ValueType::Bool,
            Literal::Int(_) => ValueType::Int,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

pub enum IrInst {
    Constant {
        value: Literal,
        ty: IrType,
    },

    Binary {
        op: BinaryOp,
        lhs: ValueId,
        rhs: ValueId,
        ty: IrType,
    },
}

pub struct IrProgram {
    pub instructions: Vec<IrInst>,
}
