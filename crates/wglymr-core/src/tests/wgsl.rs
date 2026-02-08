use crate::ir::*;
use crate::wgsl::*;

#[test]
fn test_single_constant_emits_valid_wgsl() {
    let program = IrProgram {
        instructions: vec![IrInst::Constant {
            value: Literal::Float(1.0),
            ty: IrType::Float,
        }],
    };

    let wgsl = emit_wgsl(&program);

    assert!(wgsl.contains("fn main() -> f32"));
    assert!(wgsl.contains("let v0: f32 = 1;"));
    assert!(wgsl.contains("return v0;"));
}

#[test]
fn test_binary_expression_emits_correct_wgsl() {
    let program = IrProgram {
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

    let wgsl = emit_wgsl(&program);

    assert!(wgsl.contains("let v0: f32 = 1;"));
    assert!(wgsl.contains("let v1: f32 = 2;"));
    assert!(wgsl.contains("let v2: f32 = v0 + v1;"));
    assert!(wgsl.contains("return v2;"));
}

#[test]
fn test_type_names_are_correct() {
    assert_eq!(type_to_wgsl(IrType::Float), "f32");
    assert_eq!(type_to_wgsl(IrType::Vec2), "vec2<f32>");
    assert_eq!(type_to_wgsl(IrType::Vec3), "vec3<f32>");
    assert_eq!(type_to_wgsl(IrType::Vec4), "vec4<f32>");
    assert_eq!(type_to_wgsl(IrType::Color), "vec4<f32>");
    assert_eq!(type_to_wgsl(IrType::Bool), "bool");
    assert_eq!(type_to_wgsl(IrType::Int), "i32");
}

#[test]
fn test_value_naming_is_consistent() {
    assert_eq!(value_name(ValueId(0)), "v0");
    assert_eq!(value_name(ValueId(1)), "v1");
    assert_eq!(value_name(ValueId(42)), "v42");
    assert_eq!(value_name(ValueId(100)), "v100");
}

#[test]
fn test_all_binary_ops() {
    let program = IrProgram {
        instructions: vec![
            IrInst::Constant {
                value: Literal::Float(10.0),
                ty: IrType::Float,
            },
            IrInst::Constant {
                value: Literal::Float(5.0),
                ty: IrType::Float,
            },
            IrInst::Binary {
                op: BinaryOp::Sub,
                lhs: ValueId(0),
                rhs: ValueId(1),
                ty: IrType::Float,
            },
        ],
    };

    let wgsl = emit_wgsl(&program);
    assert!(wgsl.contains("v0 - v1"));
}

#[test]
fn test_vector_types_emit_correctly() {
    let program = IrProgram {
        instructions: vec![IrInst::Constant {
            value: Literal::Vec3([1.0, 2.0, 3.0]),
            ty: IrType::Vec3,
        }],
    };

    let wgsl = emit_wgsl(&program);

    assert!(wgsl.contains("fn main() -> vec3<f32>"));
    assert!(wgsl.contains("let v0: vec3<f32> = vec3<f32>(1, 2, 3);"));
}

#[test]
fn test_color_type_emits_as_vec4() {
    let program = IrProgram {
        instructions: vec![IrInst::Constant {
            value: Literal::Vec4([1.0, 0.5, 0.0, 1.0]),
            ty: IrType::Color,
        }],
    };

    let wgsl = emit_wgsl(&program);

    assert!(wgsl.contains("fn main() -> vec4<f32>"));
    assert!(wgsl.contains("let v0: vec4<f32> = vec4<f32>(1, 0.5, 0, 1);"));
}

#[test]
fn test_conversion_float_to_vec3() {
    let program = IrProgram {
        instructions: vec![
            IrInst::Constant {
                value: Literal::Float(2.0),
                ty: IrType::Float,
            },
            IrInst::Convert {
                from: ValueId(0),
                from_ty: IrType::Float,
                to_ty: IrType::Vec3,
            },
        ],
    };

    let wgsl = emit_wgsl(&program);

    assert!(wgsl.contains("fn main() -> vec3<f32>"));
    assert!(wgsl.contains("let v0: f32 = 2;"));
    assert!(wgsl.contains("let v1: vec3<f32> = vec3<f32>(v0);"));
    assert!(wgsl.contains("return v1;"));
}

#[test]
fn test_conversion_vec3_to_color() {
    let program = IrProgram {
        instructions: vec![
            IrInst::Constant {
                value: Literal::Vec3([1.0, 0.0, 0.5]),
                ty: IrType::Vec3,
            },
            IrInst::Convert {
                from: ValueId(0),
                from_ty: IrType::Vec3,
                to_ty: IrType::Color,
            },
        ],
    };

    let wgsl = emit_wgsl(&program);

    assert!(wgsl.contains("fn main() -> vec4<f32>"));
    assert!(wgsl.contains("let v0: vec3<f32> = vec3<f32>(1, 0, 0.5);"));
    assert!(wgsl.contains("let v1: vec4<f32> = vec4<f32>(v0.x, v0.y, v0.z, 1.0);"));
    assert!(wgsl.contains("return v1;"));
}
