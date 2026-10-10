//! Dimension-checker refusals and evaluator behavior (spec D4 + D8):
//! each refusal is a TYPED error, pinned so the v1 restrictive lattice
//! (ratified F1) stays a contract — relaxation must be a deliberate,
//! additive change.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{ang, len, scl};
use editor_core::{Dimension, DimensionError, EvalError, Formula, VarEnv, eval, eval_count};
use topo::BooleanOp;

fn env() -> VarEnv<f64> {
    VarEnv::default()
}

#[test]
fn length_plus_angle_refused() {
    assert_eq!(
        Formula::add(len(1.0), ang(0.5)).unwrap_err(),
        DimensionError::Mismatch {
            op: "add",
            left: Dimension::Length,
            right: Dimension::Angle
        }
    );
}

#[test]
fn length_times_length_refused() {
    // Dimension-changing products are OUT of the v1 lattice (F1);
    // the full rational-exponent lattice would be purely additive.
    assert_eq!(
        Formula::mul(len(2.0), len(3.0)).unwrap_err(),
        DimensionError::MulNeedsScalar {
            left: Dimension::Length,
            right: Dimension::Length
        }
    );
}

#[test]
fn length_over_length_refused() {
    // Same-dimension ratios (Length/Length → Scalar) are REFUSED in
    // v1 per spec D4 — this test pins the refusal; relaxing it later
    // is additive and must flip this test deliberately.
    assert_eq!(
        Formula::div(len(1.0), len(2.0)).unwrap_err(),
        DimensionError::DivNeedsScalarDivisor {
            left: Dimension::Length,
            right: Dimension::Length
        }
    );
}

#[test]
fn implicit_count_to_scalar_refused() {
    // Count never promotes implicitly (spec D4): mixing a Count with
    // a continuous operand is a typed construction error…
    assert_eq!(
        Formula::mul(Formula::count(3), len(1.0)).unwrap_err(),
        DimensionError::CountNeedsExplicitPromotion { op: "mul" }
    );
    assert_eq!(
        Formula::div(Formula::count(3), scl(2.0)).unwrap_err(),
        DimensionError::CountNeedsExplicitPromotion { op: "div" }
    );
    // …and eval refuses a Count expression outright.
    assert_eq!(
        eval(&Clone::clone(&Formula::count(3)), &env()).unwrap_err(),
        EvalError::CountExprInContinuousEval
    );
    // The explicit promotion works.
    let promoted = Formula::count_to_scalar(Formula::count(3)).unwrap();
    let scaled = Formula::mul(promoted, len(2.0)).unwrap();
    assert_eq!(eval(&Clone::clone(&scaled), &env()).unwrap(), 6.0);
}

#[test]
fn count_literal_is_integer_only() {
    assert_eq!(
        Formula::literal(3.0, Dimension::Count).unwrap_err(),
        DimensionError::LiteralCountIsInteger
    );
}

#[test]
fn trig_needs_angle_and_produces_scalar() {
    assert_eq!(
        Formula::sin(len(1.0)).unwrap_err(),
        DimensionError::TrigNeedsAngle {
            op: "sin",
            found: Dimension::Length
        }
    );
    let s = Formula::sin(ang(0.0)).unwrap();
    assert_eq!(s.dim(), Dimension::Scalar);
    assert_eq!(eval(&Clone::clone(&s), &env()).unwrap(), 0.0);
}

#[test]
fn atan2_same_dimension_produces_angle() {
    let a = Formula::atan2(len(1.0), len(1.0)).unwrap();
    assert_eq!(a.dim(), Dimension::Angle);
    assert_eq!(
        Formula::atan2(len(1.0), scl(1.0)).unwrap_err(),
        DimensionError::Mismatch {
            op: "atan2",
            left: Dimension::Length,
            right: Dimension::Scalar
        }
    );
    assert_eq!(
        Formula::atan2(Formula::count(1), Formula::count(1)).unwrap_err(),
        DimensionError::CountNeedsExplicitPromotion { op: "atan2" }
    );
}

#[test]
fn count_arithmetic_exact_and_overflow_typed() {
    let sum = Formula::add(Formula::count(2), Formula::count(3)).unwrap();
    assert_eq!(sum.dim(), Dimension::Count);
    assert_eq!(eval_count(&Clone::clone(&sum), &env()).unwrap(), 5);
    let big = Formula::mul(Formula::count(i64::MAX), Formula::count(2)).unwrap();
    assert_eq!(
        eval_count(&Clone::clone(&big), &env()).unwrap_err(),
        EvalError::CountOverflow
    );
    // eval_count refuses continuous expressions.
    assert_eq!(
        eval_count(&Clone::clone(&len(1.0)), &env()).unwrap_err(),
        EvalError::ContinuousExprInCountEval {
            found: Dimension::Length
        }
    );
}

#[test]
fn min_max_same_dimension_only() {
    assert!(Formula::min(len(1.0), len(2.0)).is_ok());
    assert!(Formula::max(Formula::count(1), Formula::count(2)).is_ok());
    assert_eq!(
        Formula::min(len(1.0), ang(1.0)).unwrap_err(),
        DimensionError::Mismatch {
            op: "min",
            left: Dimension::Length,
            right: Dimension::Angle
        }
    );
}

/// **[`Dimension::ALL`] holds each dimension once, and a dimension
/// added to the lattice cannot reach a release without someone reading
/// this row** — the idiom `BooleanOp::ALL` (`crates/topo/src/boolean`)
/// and `CheckId::ALL` (`dsc_checks::the_registry_order_is_every_check`)
/// are held to.
///
/// **What is forced**: the match below is exhaustive with no wildcard,
/// so a dimension added to the enum fails this file until it is
/// visited here. And the no-repeats half is what makes the count a
/// census rather than a length: with every entry distinct, a `len`
/// equal to `dims` means `ALL` holds each of them exactly once.
///
/// **What is NOT forced, measured**: `dims` itself. Every arm names the
/// same total so that visiting means re-deciding it — but nothing
/// checks that number against the enum, and the arm an author adds is
/// the arm they copied. A fifth variant with the arm `Mass => 4`
/// compiles and passes GREEN with `Mass` absent from `ALL`. The row
/// forces the visit, not the edit. That is the idiom's hole and not
/// this row's alone — it is inherited from the censuses cited above —
/// so it is filed as
/// `work/census/all-census-idiom-forces-the-visit-not-the-update` rather
/// than patched here in one of four places.
#[test]
fn all_is_every_dimension() {
    let dims = match Dimension::Length {
        Dimension::Length => 4,
        Dimension::Angle => 4,
        Dimension::Count => 4,
        Dimension::Scalar => 4,
    };
    for (i, dimension) in Dimension::ALL.iter().enumerate() {
        assert!(
            !Dimension::ALL[..i].contains(dimension),
            "{dimension:?} appears twice in Dimension::ALL"
        );
    }
    assert_eq!(
        Dimension::ALL.len(),
        dims,
        "Dimension::ALL has drifted from the declaration — it holds {} dimensions, the enum has {dims}",
        Dimension::ALL.len()
    );
}
