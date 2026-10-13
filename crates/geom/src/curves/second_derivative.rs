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
/// ([`crate::NurbsCurve3::accel_upper_bound`]), so a weight other than
/// `1` refuses here.
///
/// # Errors
///
/// [`SecondDerivativeUnbounded`] — the curve is rational, the degree
/// is below 2, the first derivative is discontinuous, or the iterated
/// hull is poisoned.
pub fn nonrational_second_derivative_sup(
    curve: &NurbsCurve3<f64>,
) -> Result<f64, SecondDerivativeUnbounded> {
    if curve.weights().iter().any(|w| *w != 1.0) {
        return Err(SecondDerivativeUnbounded::Rational);
    }
    let control = curve.control();
    let mut sum_sq = <Interval as Certification>::zero();
    for comp in 0..3 {
        let hull = curve.knots().with_coeffs_from_fn(
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
                Derived::Discontinuous { .. } => {
                    Err(SecondDerivativeUnbounded::DerivativeKnotVector)
                }
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
    /// A weight is not `1`: the curve is rational, and its second
    /// derivative is a quotient this door does not assemble.
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
