use crate::{
    insert_conversions, BinaryOp, ConversionError, IrInst, IrProgram, IrType, Literal, ValueId,
};

#[test]
fn test_float_to_vec3_conversion() {
    let ir = IrProgram {
        instructions: vec![
            IrInst::Constant {
                value: Literal::Float(1.0),
                ty: IrType::Float,
            },
            IrInst::Constant {
                value: Literal::Vec3([1.0, 2.0, 3.0]),
                ty: IrType::Vec3,
            },
            IrInst::Binary {
                op: BinaryOp::Add,
                lhs: ValueId(0),
                rhs: ValueId(1),
                ty: IrType::Vec3,
            },
        ],
    };

    let result = insert_conversions(ir).unwrap();

    assert_eq!(result.instructions.len(), 4);

    match &result.instructions[2] {
        IrInst::Convert {
            from,
            from_ty,
            to_ty,
        } => {
            assert_eq!(*from, ValueId(0));
            assert_eq!(*from_ty, IrType::Float);
            assert_eq!(*to_ty, IrType::Vec3);
        }
        _ => panic!("Expected Convert instruction"),
    }

    match &result.instructions[3] {
        IrInst::Binary { lhs, rhs, .. } => {
            assert_eq!(*lhs, ValueId(2));
            assert_eq!(*rhs, ValueId(1));
        }
        _ => panic!("Expected Binary instruction"),
    }
}

#[test]
fn test_vec3_to_color_conversion() {
    let ir = IrProgram {
        instructions: vec![
            IrInst::Constant {
                value: Literal::Vec3([1.0, 0.0, 0.0]),
                ty: IrType::Vec3,
            },
            IrInst::Constant {
                value: Literal::Vec4([0.0, 1.0, 0.0, 1.0]),
                ty: IrType::Color,
            },
            IrInst::Binary {
                op: BinaryOp::Mul,
                lhs: ValueId(0),
                rhs: ValueId(1),
                ty: IrType::Color,
            },
        ],
    };

    let result = insert_conversions(ir).unwrap();

    assert_eq!(result.instructions.len(), 4);

    match &result.instructions[2] {
        IrInst::Convert {
            from,
            from_ty,
            to_ty,
        } => {
            assert_eq!(*from, ValueId(0));
            assert_eq!(*from_ty, IrType::Vec3);
            assert_eq!(*to_ty, IrType::Color);
        }
        _ => panic!("Expected Convert instruction"),
    }
}

#[test]
fn test_unsupported_conversion_errors() {
    let ir = IrProgram {
        instructions: vec![
            IrInst::Constant {
                value: Literal::Vec3([1.0, 2.0, 3.0]),
                ty: IrType::Vec3,
            },
            IrInst::Constant {
                value: Literal::Float(5.0),
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

    let result = insert_conversions(ir);
    assert!(result.is_err());

    match result.unwrap_err() {
        ConversionError::NoConversion { from, to } => {
            assert_eq!(from, IrType::Vec3);
            assert_eq!(to, IrType::Float);
        }
    }
}

#[test]
fn test_no_conversion_needed() {
    let ir = IrProgram {
        instructions: vec![
            IrInst::Constant {
                value: Literal::Vec3([1.0, 2.0, 3.0]),
                ty: IrType::Vec3,
            },
            IrInst::Constant {
                value: Literal::Vec3([4.0, 5.0, 6.0]),
                ty: IrType::Vec3,
            },
            IrInst::Binary {
                op: BinaryOp::Add,
                lhs: ValueId(0),
                rhs: ValueId(1),
                ty: IrType::Vec3,
            },
        ],
    };

    let result = insert_conversions(ir).unwrap();

    assert_eq!(result.instructions.len(), 3);
}

#[test]
fn test_instruction_order_valid() {
    let ir = IrProgram {
        instructions: vec![
            IrInst::Constant {
                value: Literal::Float(1.0),
                ty: IrType::Float,
            },
            IrInst::Binary {
                op: BinaryOp::Add,
                lhs: ValueId(0),
                rhs: ValueId(0),
                ty: IrType::Float,
            },
        ],
    };

    let result = insert_conversions(ir).unwrap();

    for (idx, inst) in result.instructions.iter().enumerate() {
        match inst {
            IrInst::Binary { lhs, rhs, .. } => {
                assert!((lhs.0 as usize) < idx);
                assert!((rhs.0 as usize) < idx);
            }
            IrInst::Convert { from, .. } => {
                assert!((from.0 as usize) < idx);
            }
            _ => {}
        }
    }
}

#[test]
fn test_both_operands_need_conversion() {
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
                ty: IrType::Vec3,
            },
        ],
    };

    let result = insert_conversions(ir).unwrap();

    assert_eq!(result.instructions.len(), 5);

    match &result.instructions[2] {
        IrInst::Convert { from_ty, to_ty, .. } => {
            assert_eq!(*from_ty, IrType::Float);
            assert_eq!(*to_ty, IrType::Vec3);
        }
        _ => panic!("Expected first Convert"),
    }

    match &result.instructions[3] {
        IrInst::Convert { from_ty, to_ty, .. } => {
            assert_eq!(*from_ty, IrType::Float);
            assert_eq!(*to_ty, IrType::Vec3);
        }
        _ => panic!("Expected second Convert"),
    }
}

#[test]
fn test_float_to_vec2_conversion() {
    let ir = IrProgram {
        instructions: vec![
            IrInst::Constant {
                value: Literal::Float(5.0),
                ty: IrType::Float,
            },
            IrInst::Constant {
                value: Literal::Vec2([1.0, 2.0]),
                ty: IrType::Vec2,
            },
            IrInst::Binary {
                op: BinaryOp::Mul,
                lhs: ValueId(0),
                rhs: ValueId(1),
                ty: IrType::Vec2,
            },
        ],
    };

    let result = insert_conversions(ir).unwrap();

    assert_eq!(result.instructions.len(), 4);

    match &result.instructions[2] {
        IrInst::Convert {
            from,
            from_ty,
            to_ty,
        } => {
            assert_eq!(*from, ValueId(0));
            assert_eq!(*from_ty, IrType::Float);
            assert_eq!(*to_ty, IrType::Vec2);
        }
        _ => panic!("Expected Convert instruction"),
    }
}

#[test]
fn test_float_to_vec4_conversion() {
    let ir = IrProgram {
        instructions: vec![
            IrInst::Constant {
                value: Literal::Float(1.0),
                ty: IrType::Float,
            },
            IrInst::Constant {
                value: Literal::Vec4([1.0, 2.0, 3.0, 4.0]),
                ty: IrType::Vec4,
            },
            IrInst::Binary {
                op: BinaryOp::Mul,
                lhs: ValueId(0),
                rhs: ValueId(1),
                ty: IrType::Vec4,
            },
        ],
    };

    let result = insert_conversions(ir).unwrap();

    assert_eq!(result.instructions.len(), 4);

    match &result.instructions[2] {
        IrInst::Convert {
            from,
            from_ty,
            to_ty,
        } => {
            assert_eq!(*from, ValueId(0));
            assert_eq!(*from_ty, IrType::Float);
            assert_eq!(*to_ty, IrType::Vec4);
        }
        _ => panic!("Expected Convert instruction"),
    }
}

#[test]
fn test_float_to_color_conversion() {
    let ir = IrProgram {
        instructions: vec![
            IrInst::Constant {
                value: Literal::Float(0.5),
                ty: IrType::Float,
            },
            IrInst::Constant {
                value: Literal::Vec4([1.0, 0.0, 0.0, 1.0]),
                ty: IrType::Color,
            },
            IrInst::Binary {
                op: BinaryOp::Mul,
                lhs: ValueId(0),
                rhs: ValueId(1),
                ty: IrType::Color,
            },
        ],
    };

    let result = insert_conversions(ir).unwrap();

    assert_eq!(result.instructions.len(), 4);

    match &result.instructions[2] {
        IrInst::Convert {
            from,
            from_ty,
            to_ty,
        } => {
            assert_eq!(*from, ValueId(0));
            assert_eq!(*from_ty, IrType::Float);
            assert_eq!(*to_ty, IrType::Color);
        }
        _ => panic!("Expected Convert instruction"),
    }
}
