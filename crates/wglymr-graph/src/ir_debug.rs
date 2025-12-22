use crate::ir::{BinaryOp, IrInst, IrProgram, IrType, Literal};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IrValidationError {
    #[error("instruction {0} references invalid value v{1}")]
    InvalidValueRef(usize, u32),

    #[error("instruction {0} references future value v{1}")]
    FutureValueRef(usize, u32),
}

pub fn pretty_print(ir: &IrProgram) -> String {
    let mut output = String::new();

    for (idx, inst) in ir.instructions.iter().enumerate() {
        let value_id = format!("v{}", idx);
        let line = match inst {
            IrInst::Constant { value, ty } => {
                let type_str = format_type(*ty);
                let value_str = format_literal(value);
                format!("{}: {} = const {}\n", value_id, type_str, value_str)
            }
            IrInst::Binary { op, lhs, rhs, ty } => {
                let type_str = format_type(*ty);
                let op_str = format_binary_op(*op);
                format!(
                    "{}: {} = {} v{}, v{}\n",
                    value_id, type_str, op_str, lhs.0, rhs.0
                )
            }
        };
        output.push_str(&line);
    }

    output
}

pub fn validate_ir(ir: &IrProgram) -> Result<(), IrValidationError> {
    for (idx, inst) in ir.instructions.iter().enumerate() {
        let referenced_values = match inst {
            IrInst::Constant { .. } => vec![],
            IrInst::Binary { lhs, rhs, .. } => vec![*lhs, *rhs],
        };

        for value_id in referenced_values {
            let value_idx = value_id.0 as usize;

            if value_idx >= ir.instructions.len() {
                return Err(IrValidationError::InvalidValueRef(idx, value_id.0));
            }

            if value_idx >= idx {
                return Err(IrValidationError::FutureValueRef(idx, value_id.0));
            }
        }
    }

    Ok(())
}

fn format_type(ty: IrType) -> &'static str {
    match ty {
        IrType::Float => "f32",
        IrType::Vec2 => "vec2",
        IrType::Vec3 => "vec3",
        IrType::Vec4 => "vec4",
        IrType::Color => "color",
        IrType::Bool => "bool",
        IrType::Int => "i32",
    }
}

fn format_literal(lit: &Literal) -> String {
    match lit {
        Literal::Float(f) => format!("{}", f),
        Literal::Vec2(v) => format!("vec2({}, {})", v[0], v[1]),
        Literal::Vec3(v) => format!("vec3({}, {}, {})", v[0], v[1], v[2]),
        Literal::Vec4(v) => format!("vec4({}, {}, {}, {})", v[0], v[1], v[2], v[3]),
        Literal::Bool(b) => format!("{}", b),
        Literal::Int(i) => format!("{}", i),
    }
}

fn format_binary_op(op: BinaryOp) -> &'static str {
    match op {
        BinaryOp::Add => "add",
        BinaryOp::Sub => "sub",
        BinaryOp::Mul => "mul",
        BinaryOp::Div => "div",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ValueId;

    #[test]
    fn test_pretty_print_simple_program() {
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
        let expected = "v0: f32 = const 1\nv1: f32 = const 2\nv2: f32 = add v0, v1\n";
        assert_eq!(output, expected);
    }

    #[test]
    fn test_validator_passes_valid_ir() {
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
    fn test_validator_fails_invalid_value_ref() {
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
    fn test_validator_fails_future_reference() {
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
}
