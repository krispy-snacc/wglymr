use crate::ir::{BinaryOp, IrInst, IrProgram, IrType, Literal, ValueId};
use crate::ir_debug::{pretty_print, validate_ir, IrValidationError};

#[test]
fn pretty_print_simple_program() {
    let ir = IrProgram {
        instructions: vec![
            IrInst::Constant {
                value: Literal::Float(1.0),
                ty: IrType::Float,
            },
            IrInst::Constant {
                value: Literal::Float(2.0),
                ty: IrType::Float,
            },
            IrInst::Binary {
                op: BinaryOp::Add,
                lhs: ValueId(0),
                rhs: ValueId(1),
                ty: IrType::Float,
            },
        ],
    };

    let output = pretty_print(&ir);
    assert!(output.contains("v0: f32 = const 1"));
    assert!(output.contains("v1: f32 = const 2"));
    assert!(output.contains("v2: f32 = add v0, v1"));
}

#[test]
fn pretty_print_with_different_types() {
    let ir = IrProgram {
        instructions: vec![
            IrInst::Constant {
                value: Literal::Vec2([1.0, 2.0]),
                ty: IrType::Vec2,
            },
            IrInst::Constant {
                value: Literal::Bool(true),
                ty: IrType::Bool,
            },
            IrInst::Constant {
                value: Literal::Int(42),
                ty: IrType::Int,
            },
        ],
    };

    let output = pretty_print(&ir);
    assert!(output.contains("v0: vec2"));
    assert!(output.contains("v1: bool"));
    assert!(output.contains("v2: i32"));
}

#[test]
fn validator_passes_valid_ir() {
    let ir = IrProgram {
        instructions: vec![
            IrInst::Constant {
                value: Literal::Float(1.0),
                ty: IrType::Float,
            },
            IrInst::Constant {
                value: Literal::Float(2.0),
                ty: IrType::Float,
            },
            IrInst::Binary {
                op: BinaryOp::Add,
                lhs: ValueId(0),
                rhs: ValueId(1),
                ty: IrType::Float,
            },
        ],
    };

    assert!(validate_ir(&ir).is_ok());
}

#[test]
fn validator_passes_constants_only() {
    let ir = IrProgram {
        instructions: vec![
            IrInst::Constant {
                value: Literal::Float(1.0),
                ty: IrType::Float,
            },
            IrInst::Constant {
                value: Literal::Float(2.0),
                ty: IrType::Float,
            },
        ],
    };

    assert!(validate_ir(&ir).is_ok());
}

#[test]
fn validator_fails_invalid_value_ref() {
    let ir = IrProgram {
        instructions: vec![
            IrInst::Constant {
                value: Literal::Float(1.0),
                ty: IrType::Float,
            },
            IrInst::Binary {
                op: BinaryOp::Add,
                lhs: ValueId(0),
                rhs: ValueId(999),
                ty: IrType::Float,
            },
        ],
    };

    let result = validate_ir(&ir);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        IrValidationError::InvalidValueRef(1, 999)
    ));
}

#[test]
fn validator_fails_future_reference() {
    let ir = IrProgram {
        instructions: vec![
            IrInst::Constant {
                value: Literal::Float(1.0),
                ty: IrType::Float,
            },
            IrInst::Binary {
                op: BinaryOp::Add,
                lhs: ValueId(0),
                rhs: ValueId(2),
                ty: IrType::Float,
            },
            IrInst::Constant {
                value: Literal::Float(3.0),
                ty: IrType::Float,
            },
        ],
    };

    let result = validate_ir(&ir);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        IrValidationError::FutureValueRef(1, 2)
    ));
}

#[test]
fn validator_fails_self_reference() {
    let ir = IrProgram {
        instructions: vec![
            IrInst::Constant {
                value: Literal::Float(1.0),
                ty: IrType::Float,
            },
            IrInst::Binary {
                op: BinaryOp::Add,
                lhs: ValueId(1),
                rhs: ValueId(1),
                ty: IrType::Float,
            },
        ],
    };

    let result = validate_ir(&ir);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        IrValidationError::FutureValueRef(1, 1)
    ));
}

#[test]
fn pretty_print_all_binary_ops() {
    let ir = IrProgram {
        instructions: vec![
            IrInst::Constant {
                value: Literal::Float(1.0),
                ty: IrType::Float,
            },
            IrInst::Constant {
                value: Literal::Float(2.0),
                ty: IrType::Float,
            },
            IrInst::Binary {
                op: BinaryOp::Add,
                lhs: ValueId(0),
                rhs: ValueId(1),
                ty: IrType::Float,
            },
            IrInst::Binary {
                op: BinaryOp::Sub,
                lhs: ValueId(0),
                rhs: ValueId(1),
                ty: IrType::Float,
            },
            IrInst::Binary {
                op: BinaryOp::Mul,
                lhs: ValueId(0),
                rhs: ValueId(1),
                ty: IrType::Float,
            },
            IrInst::Binary {
                op: BinaryOp::Div,
                lhs: ValueId(0),
                rhs: ValueId(1),
                ty: IrType::Float,
            },
        ],
    };

    let output = pretty_print(&ir);
    assert!(output.contains("add"));
    assert!(output.contains("sub"));
    assert!(output.contains("mul"));
    assert!(output.contains("div"));
}
