//! IR lowering - convert typed GraphView into linear IrProgram
//!
//! Takes a topologically sorted graph with resolved types and produces
//! a flat sequence of IR instructions with explicit value dependencies.

use std::collections::HashMap;
use thiserror::Error;

use crate::{
    BinaryOp, GraphView, IrInst, IrProgram, IrType, Literal, MathOp, NodeKind, SocketId, TypeMap,
    ValueId, ValueType,
};

#[derive(Error, Debug)]
pub enum IrLoweringError {
    #[error("missing input value for socket {0:?}")]
    MissingInput(SocketId),

    #[error("missing type for socket {0:?}")]
    MissingType(SocketId),

    #[error("unsupported node kind")]
    UnsupportedNode,
}

pub fn lower_to_ir(view: &GraphView, types: &TypeMap) -> Result<IrProgram, IrLoweringError> {
    let mut instructions = Vec::new();
    let mut socket_to_value: HashMap<SocketId, ValueId> = HashMap::new();
    let mut next_value_id = 0u32;

    for &node_id in &view.topo_order {
        if !view.reachable.contains(&node_id) {
            continue;
        }

        let node = view
            .graph
            .node(node_id)
            .expect("node in topo_order must exist");

        match &node.kind {
            NodeKind::Value(value_type) => {
                let ir_type = value_type_to_ir_type(*value_type)?;

                let literal = create_default_literal(*value_type);

                let value_id = ValueId(next_value_id);
                next_value_id += 1;

                instructions.push(IrInst::Constant {
                    value: literal,
                    ty: ir_type,
                });

                if let Some(&output_socket) = node.outputs.first() {
                    socket_to_value.insert(output_socket, value_id);
                }
            }

            NodeKind::Math(math_op) => {
                let input_values: Vec<ValueId> = node
                    .inputs
                    .iter()
                    .map(|&socket_id| {
                        view.graph
                            .links_into(socket_id)
                            .next()
                            .and_then(|link| socket_to_value.get(&link.from).copied())
                            .ok_or(IrLoweringError::MissingInput(socket_id))
                    })
                    .collect::<Result<_, _>>()?;

                if input_values.len() != 2 {
                    return Err(IrLoweringError::UnsupportedNode);
                }

                let output_socket = node
                    .outputs
                    .first()
                    .ok_or(IrLoweringError::UnsupportedNode)?;

                let output_type = types
                    .get(*output_socket)
                    .ok_or(IrLoweringError::MissingType(*output_socket))?;

                let ir_type = value_type_to_ir_type(output_type)?;

                let value_id = ValueId(next_value_id);
                next_value_id += 1;

                instructions.push(IrInst::Binary {
                    op: math_op_to_binary_op(math_op.clone()),
                    lhs: input_values[0],
                    rhs: input_values[1],
                    ty: ir_type,
                });

                socket_to_value.insert(*output_socket, value_id);
            }

            NodeKind::Generic(_) => {
                if node.inputs.len() == 1 && node.outputs.len() == 1 {
                    let input_socket = node.inputs[0];
                    let output_socket = node.outputs[0];

                    if let Some(link) = view.graph.links_into(input_socket).next() {
                        if let Some(&input_value) = socket_to_value.get(&link.from) {
                            socket_to_value.insert(output_socket, input_value);
                        }
                    }
                } else {
                    return Err(IrLoweringError::UnsupportedNode);
                }
            }
        }
    }

    Ok(IrProgram { instructions })
}

fn value_type_to_ir_type(value_type: ValueType) -> Result<IrType, IrLoweringError> {
    Ok(match value_type {
        ValueType::Float => IrType::Float,
        ValueType::Vec2 => IrType::Vec2,
        ValueType::Vec3 => IrType::Vec3,
        ValueType::Vec4 => IrType::Vec4,
        ValueType::Bool => IrType::Bool,
        ValueType::Int => IrType::Int,
        ValueType::Color => IrType::Color,
    })
}

fn create_default_literal(value_type: ValueType) -> Literal {
    match value_type {
        ValueType::Float => Literal::Float(0.0),
        ValueType::Vec2 => Literal::Vec2([0.0, 0.0]),
        ValueType::Vec3 => Literal::Vec3([0.0, 0.0, 0.0]),
        ValueType::Vec4 => Literal::Vec4([0.0, 0.0, 0.0, 0.0]),
        ValueType::Bool => Literal::Bool(false),
        ValueType::Int => Literal::Int(0),
        ValueType::Color => Literal::Vec4([0.0, 0.0, 0.0, 1.0]),
    }
}

fn math_op_to_binary_op(math_op: MathOp) -> BinaryOp {
    match math_op {
        MathOp::Add => BinaryOp::Add,
        MathOp::Subtract => BinaryOp::Sub,
        MathOp::Multiply => BinaryOp::Mul,
        MathOp::Divide => BinaryOp::Div,
    }
}
