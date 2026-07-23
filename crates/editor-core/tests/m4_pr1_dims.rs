//! Dimension-checker refusals and evaluator behavior (spec D4 + D8):
//! each refusal is a TYPED error, pinned so the v1 restrictive lattice
//! (ratified F1) stays a contract — relaxation must be a deliberate,
//! additive change.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{Dimension, DimensionError, EvalError, Expr, ParamEnv, eval, eval_count};

fn len(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).unwrap()
}

fn ang(v: f64) -> Expr {
    Expr::literal(v, Dimension::Angle).unwrap()
}

fn scl(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).unwrap()
}

fn env() -> ParamEnv<f64> {
    ParamEnv::default()
}

#[test]
fn length_plus_angle_refused() {
    assert_eq!(
        Expr::add(len(1.0), ang(0.5)).unwrap_err(),
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
        Expr::mul(len(2.0), len(3.0)).unwrap_err(),
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
        Expr::div(len(1.0), len(2.0)).unwrap_err(),
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
        Expr::mul(Expr::count(3), len(1.0)).unwrap_err(),
        DimensionError::CountNeedsExplicitPromotion { op: "mul" }
    );
    assert_eq!(
        Expr::div(Expr::count(3), scl(2.0)).unwrap_err(),
        DimensionError::CountNeedsExplicitPromotion { op: "div" }
    );
    // …and eval refuses a Count expression outright.
    assert_eq!(
        eval(&Expr::count(3), &env()).unwrap_err(),
        EvalError::CountExprInContinuousEval
    );
    // The explicit promotion works.
    let promoted = Expr::count_to_scalar(Expr::count(3)).unwrap();
    let scaled = Expr::mul(promoted, len(2.0)).unwrap();
    assert_eq!(eval(&scaled, &env()).unwrap(), 6.0);
}

#[test]
fn count_literal_is_integer_only() {
    assert_eq!(
        Expr::literal(3.0, Dimension::Count).unwrap_err(),
        DimensionError::LiteralCountIsInteger
    );
}

#[test]
fn trig_needs_angle_and_produces_scalar() {
    assert_eq!(
        Expr::sin(len(1.0)).unwrap_err(),
        DimensionError::TrigNeedsAngle {
            op: "sin",
            found: Dimension::Length
        }
    );
    let s = Expr::sin(ang(0.0)).unwrap();
    assert_eq!(s.dim(), Dimension::Scalar);
    assert_eq!(eval(&s, &env()).unwrap(), 0.0);
}

#[test]
fn atan2_same_dimension_produces_angle() {
    let a = Expr::atan2(len(1.0), len(1.0)).unwrap();
    assert_eq!(a.dim(), Dimension::Angle);
    assert_eq!(
        Expr::atan2(len(1.0), scl(1.0)).unwrap_err(),
        DimensionError::Mismatch {
            op: "atan2",
            left: Dimension::Length,
            right: Dimension::Scalar
        }
    );
    assert_eq!(
        Expr::atan2(Expr::count(1), Expr::count(1)).unwrap_err(),
        DimensionError::CountNeedsExplicitPromotion { op: "atan2" }
    );
}

#[test]
fn count_arithmetic_exact_and_overflow_typed() {
    let sum = Expr::add(Expr::count(2), Expr::count(3)).unwrap();
    assert_eq!(sum.dim(), Dimension::Count);
    assert_eq!(eval_count(&sum, &env()).unwrap(), 5);
    let big = Expr::mul(Expr::count(i64::MAX), Expr::count(2)).unwrap();
    assert_eq!(
        eval_count(&big, &env()).unwrap_err(),
        EvalError::CountOverflow
    );
    // eval_count refuses continuous expressions.
    assert_eq!(
        eval_count(&len(1.0), &env()).unwrap_err(),
        EvalError::ContinuousExprInCountEval {
            found: Dimension::Length
        }
    );
}

#[test]
fn min_max_same_dimension_only() {
    assert!(Expr::min(len(1.0), len(2.0)).is_ok());
    assert!(Expr::max(Expr::count(1), Expr::count(2)).is_ok());
    assert_eq!(
        Expr::min(len(1.0), ang(1.0)).unwrap_err(),
        DimensionError::Mismatch {
            op: "min",
            left: Dimension::Length,
            right: Dimension::Angle
        }
    );
}
