use crate::{IrInst, IrProgram, IrType, ValueId};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConversionError {
    #[error("no valid conversion from {from:?} to {to:?}")]
    NoConversion { from: IrType, to: IrType },
}

/// Check if conversion from one type to another is allowed
fn is_valid_conversion(from: IrType, to: IrType) -> bool {
    match (from, to) {
        // Float to vector types (splat)
        (IrType::Float, IrType::Vec2) => true,
        (IrType::Float, IrType::Vec3) => true,
        (IrType::Float, IrType::Vec4) => true,
        (IrType::Float, IrType::Color) => true,

        // Vec3 to Color (append alpha = 1.0)
        (IrType::Vec3, IrType::Color) => true,

        // All other conversions are not allowed
        _ => false,
    }
}

/// Insert explicit conversion instructions where types are incompatible but convertible
///
/// This pass walks the IR and detects type mismatches at use sites.
/// When a value of one type is used where another type is expected,
/// and a valid conversion exists, a Convert instruction is inserted.
///
/// Supported conversions:
/// - Float => Vec2/Vec3/Vec4/Color (splat)
/// - Vec3 => Color (append alpha = 1.0)
///
/// All other type mismatches result in an error.
pub fn insert_conversions(ir: IrProgram) -> Result<IrProgram, ConversionError> {
    let mut new_instructions = Vec::new();
    let mut value_types: Vec<IrType> = Vec::new();

    for inst in ir.instructions {
        match inst {
            IrInst::Constant { value, ty } => {
                value_types.push(ty);
                new_instructions.push(IrInst::Constant { value, ty });
            }

            IrInst::Binary { op, lhs, rhs, ty } => {
                // Check if operands need conversion
                let lhs_ty = value_types[lhs.0 as usize];
                let rhs_ty = value_types[rhs.0 as usize];

                // Convert lhs if needed
                let lhs_converted = if lhs_ty != ty {
                    if is_valid_conversion(lhs_ty, ty) {
                        let converted_id = ValueId(new_instructions.len() as u32);
                        new_instructions.push(IrInst::Convert {
                            from: lhs,
                            from_ty: lhs_ty,
                            to_ty: ty,
                        });
                        value_types.push(ty);
                        converted_id
                    } else {
                        return Err(ConversionError::NoConversion {
                            from: lhs_ty,
                            to: ty,
                        });
                    }
                } else {
                    lhs
                };

                // Convert rhs if needed
                let rhs_converted = if rhs_ty != ty {
                    if is_valid_conversion(rhs_ty, ty) {
                        let converted_id = ValueId(new_instructions.len() as u32);
                        new_instructions.push(IrInst::Convert {
                            from: rhs,
                            from_ty: rhs_ty,
                            to_ty: ty,
                        });
                        value_types.push(ty);
                        converted_id
                    } else {
                        return Err(ConversionError::NoConversion {
                            from: rhs_ty,
                            to: ty,
                        });
                    }
                } else {
                    rhs
                };

                value_types.push(ty);
                new_instructions.push(IrInst::Binary {
                    op,
                    lhs: lhs_converted,
                    rhs: rhs_converted,
                    ty,
                });
            }

            IrInst::Convert {
                from,
                from_ty,
                to_ty,
            } => {
                // Validate the conversion
                if !is_valid_conversion(from_ty, to_ty) {
                    return Err(ConversionError::NoConversion {
                        from: from_ty,
                        to: to_ty,
                    });
                }

                value_types.push(to_ty);
                new_instructions.push(IrInst::Convert {
                    from,
                    from_ty,
                    to_ty,
                });
            }
        }
    }

    Ok(IrProgram {
        instructions: new_instructions,
    })
}
