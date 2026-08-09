//! Tests for the Python-independent half of the crate.
//!
//! These run on the DEFAULT build path — no `python` feature, no
//! interpreter — which is the point: hosted CI executes them without a
//! Python toolchain present.

use crate::errors::{
    DimensionError, ErrorClass, LiteralRefusal, canonical_unit, check_literal, dimension_tag,
};
use pncad::document::Dimension;

#[test]
fn dimension_tags_are_stable() {
    assert_eq!(dimension_tag(Dimension::Length), "length");
    assert_eq!(dimension_tag(Dimension::Angle), "angle");
    assert_eq!(dimension_tag(Dimension::Count), "count");
    assert_eq!(dimension_tag(Dimension::Scalar), "scalar");
}

#[test]
fn canonical_units_match_the_gq5_ratification() {
    // GQ5 / §L4: canonical metres and radians underneath.
    assert_eq!(canonical_unit(Dimension::Length), Some("m"));
    assert_eq!(canonical_unit(Dimension::Angle), Some("rad"));
    assert_eq!(canonical_unit(Dimension::Count), None);
    assert_eq!(canonical_unit(Dimension::Scalar), None);
}

#[test]
fn dimension_error_carries_structure_not_prose() {
    let err = DimensionError::new("+", Dimension::Length, Dimension::Angle);
    assert_eq!(err.op, "+");
    assert_eq!(err.left, Dimension::Length);
    assert_eq!(err.right, Dimension::Angle);
    // The message exists for humans, but the fields above are the
    // contract (§L4: never strings).
    assert_eq!(err.to_string(), "cannot apply `+` to length and angle");
}

#[test]
fn error_classes_name_the_python_hierarchy() {
    assert_eq!(ErrorClass::Edit.class_name(), "EditError");
    assert_eq!(ErrorClass::Evaluation.class_name(), "EvaluationError");
    assert_eq!(ErrorClass::Validation.class_name(), "ValidationError");
    assert_eq!(ErrorClass::Dimension.class_name(), "DimensionError");
    assert_eq!(ErrorClass::Literal.class_name(), "LiteralError");
}

#[test]
fn literal_gate_refuses_exactly_what_the_kernel_refuses() {
    assert_eq!(check_literal(1.5, Dimension::Length), Ok(1.5));
    // NaN != NaN, so this arm is matched structurally rather than
    // compared by value.
    assert!(matches!(
        check_literal(f64::NAN, Dimension::Length),
        Err(LiteralRefusal::NonFinite { value }) if value.is_nan()
    ));
    assert!(matches!(
        check_literal(f64::INFINITY, Dimension::Length),
        Err(LiteralRefusal::NonFinite { .. })
    ));
    assert_eq!(
        check_literal(3.0, Dimension::Count),
        Err(LiteralRefusal::CountIsInteger)
    );
}
