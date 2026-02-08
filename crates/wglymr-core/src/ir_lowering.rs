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

    #[error("optional input socket {0:?} missing default value")]
    OptionalInputMissingDefault(SocketId),
}

struct LoweringContext<'a> {
    view: &'a GraphView<'a>,
    instructions: Vec<IrInst>,
    socket_to_value: HashMap<SocketId, ValueId>,
    next_value_id: u32,
}

impl<'a> LoweringContext<'a> {
    fn new(view: &'a GraphView<'a>) -> Self {
        Self {
            view,
            instructions: Vec::new(),
            socket_to_value: HashMap::new(),
            next_value_id: 0,
        }
    }

    fn alloc_value_id(&mut self) -> ValueId {
        let id = ValueId(self.next_value_id);
        self.next_value_id += 1;
        id
    }

    fn emit_constant(&mut self, literal: Literal, ir_type: IrType) -> ValueId {
        let value_id = self.alloc_value_id();
        self.instructions.push(IrInst::Constant {
            value: literal,
            ty: ir_type,
        });
        value_id
    }

    fn resolve_input(&mut self, socket_id: SocketId) -> Result<ValueId, IrLoweringError> {
        if let Some(link) = self.view.graph.links_into(socket_id).next() {
            if let Some(&value_id) = self.socket_to_value.get(&link.from) {
                return Ok(value_id);
            }
        }

        let socket = self
            .view
            .graph
            .socket(socket_id)
            .expect("socket from node must exist");

        let config = socket.input_config.as_ref();
        let is_optional = config.map(|c| c.optional).unwrap_or(false);

        if is_optional {
            if let Some(default_literal) = config.and_then(|c| c.default.clone()) {
                let ir_type = value_type_to_ir_type(socket.value_type)?;
                let value_id = self.emit_constant(default_literal, ir_type);
                return Ok(value_id);
            } else {
                return Err(IrLoweringError::OptionalInputMissingDefault(socket_id));
            }
        }

        Err(IrLoweringError::MissingInput(socket_id))
    }
}

pub fn lower_to_ir(view: &GraphView, types: &TypeMap) -> Result<IrProgram, IrLoweringError> {
    let mut ctx = LoweringContext::new(view);

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
                let value_id = ctx.emit_constant(literal, ir_type);

                if let Some(&output_socket) = node.outputs.first() {
                    ctx.socket_to_value.insert(output_socket, value_id);
                }
            }

            NodeKind::Math(math_op) => {
                let mut input_values = Vec::new();
                for &socket_id in &node.inputs {
                    let value_id = ctx.resolve_input(socket_id)?;
                    input_values.push(value_id);
                }

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

                let value_id = ctx.alloc_value_id();

                ctx.instructions.push(IrInst::Binary {
                    op: math_op_to_binary_op(math_op.clone()),
                    lhs: input_values[0],
                    rhs: input_values[1],
                    ty: ir_type,
                });

                ctx.socket_to_value.insert(*output_socket, value_id);
            }

            NodeKind::Generic(_) => {
                if node.inputs.len() == 1 && node.outputs.len() == 1 {
                    let input_socket = node.inputs[0];
                    let output_socket = node.outputs[0];

                    let input_value = ctx.resolve_input(input_socket)?;
                    ctx.socket_to_value.insert(output_socket, input_value);
                } else {
                    return Err(IrLoweringError::UnsupportedNode);
                }
            }
        }
    }

    Ok(IrProgram {
        instructions: ctx.instructions,
    })
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
