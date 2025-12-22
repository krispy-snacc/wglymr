use crate::ir::*;

#[test]
fn test_value_ids_are_monotonic() {
    let v0 = ValueId(0);
    let v1 = ValueId(1);
    let v2 = ValueId(2);

    assert!(v0.0 < v1.0);
    assert!(v1.0 < v2.0);
    assert!(v0.0 < v2.0);
}

#[test]
fn test_constant_instruction_type_correctness() {
    let inst = IrInst::Constant {
        value: Literal::Float(1.0),
        ty: IrType::Float,
    };

    match inst {
        IrInst::Constant { ty, .. } => {
            assert_eq!(ty, IrType::Float);
        }
        _ => panic!("Expected constant instruction"),
    }
}

#[test]
fn test_binary_instruction_references_value_ids() {
    let v0 = ValueId(0);
    let v1 = ValueId(1);

    let inst = IrInst::Binary {
        op: BinaryOp::Add,
        lhs: v0,
        rhs: v1,
        ty: IrType::Float,
    };

    match inst {
        IrInst::Binary { lhs, rhs, .. } => {
            assert_eq!(lhs, v0);
            assert_eq!(rhs, v1);
        }
        _ => panic!("Expected binary instruction"),
    }
}

#[test]
fn test_ir_program_preserves_insertion_order() {
    let mut program = IrProgram {
        instructions: Vec::new(),
    };

    program.instructions.push(IrInst::Constant {
        value: Literal::Float(1.0),
        ty: IrType::Float,
    });

    program.instructions.push(IrInst::Constant {
        value: Literal::Float(2.0),
        ty: IrType::Float,
    });

    program.instructions.push(IrInst::Binary {
        op: BinaryOp::Add,
        lhs: ValueId(0),
        rhs: ValueId(1),
        ty: IrType::Float,
    });

    assert_eq!(program.instructions.len(), 3);

    match &program.instructions[0] {
        IrInst::Constant { .. } => {}
        _ => panic!("First instruction should be constant"),
    }

    match &program.instructions[1] {
        IrInst::Constant { .. } => {}
        _ => panic!("Second instruction should be constant"),
    }

    match &program.instructions[2] {
        IrInst::Binary { .. } => {}
        _ => panic!("Third instruction should be binary"),
    }
}


