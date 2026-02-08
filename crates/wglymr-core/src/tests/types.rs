use crate::{types::*, TypeError, ValueType};

#[test]
fn test_scalar_classification() {
    assert!(ValueType::Float.is_scalar());
    assert!(ValueType::Bool.is_scalar());
    assert!(ValueType::Int.is_scalar());

    assert!(!ValueType::Vec2.is_scalar());
    assert!(!ValueType::Vec3.is_scalar());
    assert!(!ValueType::Vec4.is_scalar());
    assert!(!ValueType::Color.is_scalar());
}

#[test]
fn test_vector_classification() {
    assert!(ValueType::Vec2.is_vector());
    assert!(ValueType::Vec3.is_vector());
    assert!(ValueType::Vec4.is_vector());
    assert!(ValueType::Color.is_vector());

    assert!(!ValueType::Float.is_vector());
    assert!(!ValueType::Bool.is_vector());
    assert!(!ValueType::Int.is_vector());
}

#[test]
fn test_vector_widths() {
    assert_eq!(ValueType::Vec2.vector_width(), Some(2));
    assert_eq!(ValueType::Vec3.vector_width(), Some(3));
    assert_eq!(ValueType::Vec4.vector_width(), Some(4));
    assert_eq!(ValueType::Color.vector_width(), Some(4));

    assert_eq!(ValueType::Float.vector_width(), None);
    assert_eq!(ValueType::Bool.vector_width(), None);
    assert_eq!(ValueType::Int.vector_width(), None);
}

#[test]
fn test_exact_type_compatibility() {
    assert!(are_compatible(ValueType::Float, ValueType::Float));
    assert!(are_compatible(ValueType::Vec2, ValueType::Vec2));
    assert!(are_compatible(ValueType::Vec3, ValueType::Vec3));
    assert!(are_compatible(ValueType::Vec4, ValueType::Vec4));
    assert!(are_compatible(ValueType::Bool, ValueType::Bool));
    assert!(are_compatible(ValueType::Int, ValueType::Int));
    assert!(are_compatible(ValueType::Color, ValueType::Color));
}

#[test]
fn test_mismatched_types_incompatible() {
    assert!(!are_compatible(ValueType::Float, ValueType::Int));
    assert!(!are_compatible(ValueType::Vec2, ValueType::Vec3));
    assert!(!are_compatible(ValueType::Vec3, ValueType::Vec4));
    assert!(!are_compatible(ValueType::Bool, ValueType::Int));
}

#[test]
fn test_color_not_compatible_with_vec4() {
    assert!(!are_compatible(ValueType::Color, ValueType::Vec4));
    assert!(!are_compatible(ValueType::Vec4, ValueType::Color));
}

#[test]
fn test_unification_identical_types() {
    assert_eq!(
        unify(&[ValueType::Float, ValueType::Float, ValueType::Float]).unwrap(),
        ValueType::Float
    );
    assert_eq!(
        unify(&[ValueType::Vec3, ValueType::Vec3]).unwrap(),
        ValueType::Vec3
    );
    assert_eq!(unify(&[ValueType::Color]).unwrap(), ValueType::Color);
}

#[test]
fn test_unification_fails_mixed_types() {
    let result = unify(&[ValueType::Float, ValueType::Int]);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), TypeError::Mismatch { .. }));

    let result = unify(&[ValueType::Vec2, ValueType::Vec3, ValueType::Vec2]);
    assert!(result.is_err());
}

#[test]
fn test_color_does_not_unify_with_vec4() {
    let result = unify(&[ValueType::Color, ValueType::Vec4]);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), TypeError::Mismatch { .. }));
}

#[test]
fn test_unification_empty_fails() {
    let result = unify(&[]);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), TypeError::EmptyUnification));
}
