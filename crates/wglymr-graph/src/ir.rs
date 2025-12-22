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

pub enum Literal {
    Float(f32),
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4([f32; 4]),
    Bool(bool),
    Int(i32),
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
