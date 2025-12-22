use crate::ir::{BinaryOp, IrInst, IrProgram, IrType, Literal, ValueId};

pub(crate) fn value_name(id: ValueId) -> String {
    format!("v{}", id.0)
}

pub(crate) fn type_to_wgsl(ty: IrType) -> &'static str {
    match ty {
        IrType::Float => "f32",
        IrType::Vec2 => "vec2<f32>",
        IrType::Vec3 => "vec3<f32>",
        IrType::Vec4 => "vec4<f32>",
        IrType::Color => "vec4<f32>",
        IrType::Bool => "bool",
        IrType::Int => "i32",
    }
}

pub fn emit_wgsl(ir: &IrProgram) -> String {
    let mut output = String::new();

    output.push_str("fn main() -> ");
    let return_type = get_return_type(ir);
    output.push_str(&type_to_wgsl(return_type));
    output.push_str(" {\n");

    for (index, inst) in ir.instructions.iter().enumerate() {
        let value_id = ValueId(index as u32);
        output.push_str(&emit_instruction(inst, value_id));
    }

    let last_value = ValueId((ir.instructions.len() - 1) as u32);
    output.push_str("    return ");
    output.push_str(&value_name(last_value));
    output.push_str(";\n");

    output.push_str("}\n");

    output
}

fn get_return_type(ir: &IrProgram) -> IrType {
    match ir.instructions.last() {
        Some(IrInst::Constant { ty, .. }) => *ty,
        Some(IrInst::Binary { ty, .. }) => *ty,
        None => panic!("Empty IR program"),
    }
}

fn emit_instruction(inst: &IrInst, value_id: ValueId) -> String {
    match inst {
        IrInst::Constant { value, ty } => {
            let mut line = String::from("    let ");
            line.push_str(&value_name(value_id));
            line.push_str(": ");
            line.push_str(&type_to_wgsl(*ty));
            line.push_str(" = ");
            line.push_str(&literal_to_wgsl(value, *ty));
            line.push_str(";\n");
            line
        }
        IrInst::Binary { op, lhs, rhs, ty } => {
            let mut line = String::from("    let ");
            line.push_str(&value_name(value_id));
            line.push_str(": ");
            line.push_str(&type_to_wgsl(*ty));
            line.push_str(" = ");
            line.push_str(&value_name(*lhs));
            line.push_str(" ");
            line.push_str(binop_to_wgsl(*op));
            line.push_str(" ");
            line.push_str(&value_name(*rhs));
            line.push_str(";\n");
            line
        }
    }
}

fn literal_to_wgsl(lit: &Literal, ty: IrType) -> String {
    match lit {
        Literal::Float(f) => format!("{}", f),
        Literal::Vec2([x, y]) => format!("vec2<f32>({}, {})", x, y),
        Literal::Vec3([x, y, z]) => format!("vec3<f32>({}, {}, {})", x, y, z),
        Literal::Vec4([x, y, z, w]) => match ty {
            IrType::Color => format!("vec4<f32>({}, {}, {}, {})", x, y, z, w),
            _ => format!("vec4<f32>({}, {}, {}, {})", x, y, z, w),
        },
        Literal::Bool(b) => format!("{}", b),
        Literal::Int(i) => format!("{}", i),
    }
}

fn binop_to_wgsl(op: BinaryOp) -> &'static str {
    match op {
        BinaryOp::Add => "+",
        BinaryOp::Sub => "-",
        BinaryOp::Mul => "*",
        BinaryOp::Div => "/",
    }
}
