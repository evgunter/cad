//! The certified second-derivative hull of a non-rational 3-D
//! B-spline — the one spelling the mesh chord schedule and the STEP
//! export's node schedule both read.
//!
//! A certification file (`scripts/gates/certification-doors.sh`'s
//! importer list): its brackets are built through the `Certification`
//! doors and its one endpoint read asks `is_certified()` first.

use geom_core::Bounds;
use geom_core::interval::Interval;
use geom_core::interval::certification::Certification;
use geom_core::spline::Derived;

use crate::curves::NurbsCurve3;

/// A certified `sup‖C″‖` over the whole domain of a **non-rational**
/// 3-D B-spline — the iterated coefficient-difference hull
/// ([`geom_core::spline::SplineCoeffs::derived`] twice, per
/// component, in the C9 ring so every knot difference rounds
/// outward), combined as the Euclidean norm of the three per-component
/// hull magnitudes.
///
/// `Err(SecondDerivativeUnbounded)` when the structure cannot license
/// a bound, and the variant NAMES which structure — a caller's typed
/// refusal is only as good as the reason it can quote. Rational curves
/// are NOT this door's: their `C″ = M/W³` is the quotient statement's
/// ([`crate::NurbsCurve3::accel_upper_bound`]), so weights that are not
/// one constant refuse here ([`NurbsCurve3::constant_weight`]).
///
/// # Errors
///
/// [`SecondDerivativeUnbounded`] — the curve is rational, the degree
/// is below 2, the first derivative is discontinuous, or the iterated
/// hull is poisoned.
pub fn nonrational_second_derivative_sup(
    curve: &NurbsCurve3<f64>,
) -> Result<f64, SecondDerivativeUnbounded> {
    if curve.constant_weight().is_none() {
        return Err(SecondDerivativeUnbounded::Rational);
    }
    let control = curve.control();
    let mut sum_sq = <Interval as Certification>::zero();
    for comp in 0..3 {
        let hull = curve.knots_f64_interim().with_coeffs_from_fn(
            |i| {
                let pt = control[i];
                Interval::point(match comp {
                    0 => pt.x,
                    1 => pt.y,
                    _ => pt.z,
                })
            },
            |pair| match pair.derived() {
                // The second level's hull is the first level's
                // derivative hull against its own vector.
                Derived::Vector(first) => Ok(first.pair().derivative_domain_hull()),
                Derived::SpanConstant(_) => Err(SecondDerivativeUnbounded::DegreeBelowTwo),
                Derived::Discontinuous(_) => Err(SecondDerivativeUnbounded::DerivativeKnotVector),
            },
        )?;
        sum_sq = sum_sq + hull.sqr();
    }
    // A refused hull carries its refusal in the decoration rather than
    // in the endpoints, so it is asked by name.
    if !sum_sq.is_certified() {
        return Err(SecondDerivativeUnbounded::PoisonedHull);
    }
    let bound = sum_sq.hi().sqrt().next_up();
    if bound.is_finite() {
        Ok(bound)
    } else {
        Err(SecondDerivativeUnbounded::PoisonedHull)
    }
}

/// Why [`nonrational_second_derivative_sup`] can state no bound — one
/// variant per structure that denies one, so a caller's refusal can
/// name what it met rather than reporting a poisoned number.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecondDerivativeUnbounded {
    /// The weights are not one constant: the curve is rational, and its
    /// second derivative is a quotient this door does not assemble.
    Rational,
    /// The knot vector's degree is below 2, so the curve has no second
    /// derivative to hull ([`Derived::SpanConstant`]).
    DegreeBelowTwo,
    /// An interior knot of multiplicity equal to the degree: the first
    /// derivative is discontinuous there ([`Derived::Discontinuous`]).
    DerivativeKnotVector,
    /// The iterated difference-coefficient hull is poisoned or
    /// unbounded (a non-finite control point, a zero knot difference).
    PoisonedHull,
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::{SecondDerivativeUnbounded, nonrational_second_derivative_sup};
    use crate::curves::NurbsCurve3;
    use geom_core::Point3;
    use geom_core::spline::KnotVector;

    /// **A constant weight is this door's**: the cubic with every weight
    /// `2`, `0.5`, `3` or `0.1` is the polynomial on its net, and its
    /// bound is the unit-weight curve's to the bit (the hull reads the control net
    /// alone); a varying weight vector refuses as rational.
    #[test]
    fn a_constant_weight_bounds_as_its_polynomial_and_a_varying_one_refuses() {
        let sup = |weights: Vec<f64>| {
            let curve = NurbsCurve3::new(
                KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0, 1.0], 3).unwrap(),
                vec![
                    Point3::new(0.0, 0.0, 0.0),
                    Point3::new(1.0, 2.0, 0.5),
                    Point3::new(2.0, -1.0, 0.0),
                    Point3::new(3.0, 1.0, -0.5),
                    Point3::new(4.0, 0.0, 0.0),
                ],
                weights,
            )
            .unwrap();
            nonrational_second_derivative_sup(&curve)
        };
        let unit = sup(vec![1.0; 5]).unwrap();
        for c in [2.0, 0.5, 3.0, 0.1] {
            assert_eq!(
                sup(vec![c; 5]).unwrap().to_bits(),
                unit.to_bits(),
                "weights {c}"
            );
        }
        assert_eq!(
            sup(vec![1.0, 1.0, 2.0, 1.0, 1.0]),
            Err(SecondDerivativeUnbounded::Rational)
        );
    }
}
