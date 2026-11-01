//! `geom::nonrational_second_derivative_sup`: the bound on an exact
//! parabola, and each structure that denies a bound refusing by its own
//! cause.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{NurbsCurve3, SecondDerivativeUnbounded, nonrational_second_derivative_sup};
use geom_core::Point3;
use geom_core::spline::KnotVector;

fn curve(
    knots: Vec<f64>,
    degree: usize,
    control: Vec<Point3<f64>>,
    weights: Vec<f64>,
) -> NurbsCurve3<f64> {
    NurbsCurve3::new(
        KnotVector::clamped(knots, degree).unwrap(),
        control,
        weights,
    )
    .unwrap()
}

/// A quadratic Bézier has the constant `C″ = 2(P₀ − 2P₁ + P₂)`; here
/// `(0, −8, 0)`, so the hull is exact and the bound is `8` rounded up.
#[test]
fn a_quadratic_bezier_bounds_its_constant_second_derivative() {
    let c = curve(
        vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        2,
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 2.0, 0.0),
            Point3::new(2.0, 0.0, 0.0),
        ],
        vec![1.0; 3],
    );
    let bound = nonrational_second_derivative_sup(&c).unwrap();
    assert!(
        (8.0..=8.0 * (1.0 + 1e-12)).contains(&bound),
        "sup |C″| of a constant (0, −8, 0) is 8: got {bound}"
    );
}

/// A rational quarter circle: its second derivative is a quotient the
/// door does not assemble, so a weight other than `1` refuses rather
/// than hulling the control net as if it were polynomial.
#[test]
fn a_rational_curve_refuses_as_rational() {
    let w = core::f64::consts::FRAC_1_SQRT_2;
    let arc = curve(
        vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        2,
        vec![
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
        ],
        vec![1.0, w, 1.0],
    );
    assert_eq!(
        nonrational_second_derivative_sup(&arc),
        Err(SecondDerivativeUnbounded::Rational),
        "a rational arc is not this door's"
    );
}

/// Degree 1 has no second derivative; a degree-2 interior knot of
/// multiplicity 2 makes the first derivative discontinuous. Each
/// refuses by its own cause.
#[test]
fn each_denying_structure_refuses_by_its_cause() {
    let polyline = curve(
        vec![0.0, 0.0, 0.5, 1.0, 1.0],
        1,
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
            Point3::new(2.0, 0.0, 0.0),
        ],
        vec![1.0; 3],
    );
    assert_eq!(
        nonrational_second_derivative_sup(&polyline),
        Err(SecondDerivativeUnbounded::DegreeBelowTwo),
        "degree 1"
    );
    let kinked = curve(
        vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0],
        2,
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
            Point3::new(2.0, 0.0, 0.0),
            Point3::new(3.0, 1.0, 0.0),
            Point3::new(4.0, 0.0, 0.0),
        ],
        vec![1.0; 5],
    );
    assert_eq!(
        nonrational_second_derivative_sup(&kinked),
        Err(SecondDerivativeUnbounded::DerivativeKnotVector),
        "a degree-2 interior knot of multiplicity 2"
    );
}
