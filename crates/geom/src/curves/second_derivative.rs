//! The certified second-derivative hull of a non-rational 3-D
//! B-spline — the one spelling the mesh chord schedule and the STEP
//! export's node schedule both read.
//!
//! A certification file (`scripts/gates/certification-doors.sh`'s
//! importer list): its brackets are built through the `Certification`
//! doors and its one endpoint read asks `is_certified()` first.

use geom_core::Bounds;
use geom_core::Point3;
use geom_core::interval::Interval;
use geom_core::interval::certification::Certification;
use geom_core::spline::{KnotVector, SplineCoeffs};

/// A certified `sup‖C″‖` over the whole domain of a **non-rational**
/// 3-D B-spline given by its knot vector and control net — the
/// iterated coefficient-difference hull
/// ([`geom_core::spline::SplineCoeffs::derivative_coeffs`] twice,
/// per component, in the C9 ring so every knot difference rounds
/// outward), combined as the Euclidean norm of the three per-component
/// hull magnitudes.
///
/// `Err(SecondDerivativeUnbounded)` when the structure cannot license
/// a bound, and the variant NAMES which structure — a caller's typed
/// refusal is only as good as the reason it can quote, and folding
/// three causes onto one `NaN` cost the mesh chord lane exactly that
/// when this body was extracted out of it. Rational nets are NOT this
/// door's: their quotient-rule assembly divides by a weight range and
/// lives with the consumer that owns the homogeneous form.
///
/// # Errors
///
/// [`SecondDerivativeUnbounded`] — the degree is below 2, the
/// derivative knot vector does not materialise, or the iterated hull
/// is poisoned.
pub fn nonrational_second_derivative_sup(
    knots: &KnotVector,
    control: &[Point3<f64>],
) -> Result<f64, SecondDerivativeUnbounded> {
    let p = knots.degree();
    if p < 2 {
        return Err(SecondDerivativeUnbounded::DegreeBelowTwo);
    }
    let Ok(kv1) = KnotVector::clamped(knots.derivative_knot_slice().to_vec(), p - 1) else {
        return Err(SecondDerivativeUnbounded::DerivativeKnotVector);
    };
    let mut sum_sq = <Interval as Certification>::zero();
    for comp in 0..3 {
        let coeffs: Vec<Interval> = control
            .iter()
            .map(|pt| {
                Interval::point(match comp {
                    0 => pt.x,
                    1 => pt.y,
                    _ => pt.z,
                })
            })
            .collect();
        let q1 = knots.difference_coeffs(&coeffs);
        // The hull of the SECOND-difference net through the geom-core
        // door: the second difference is the first difference of `q1`
        // against the derivative vector `kv1`, which is what
        // `derivative_domain_hull` answers. A length the mint refuses
        // arrives refused.
        let hull = kv1
            .with_coeffs(&q1)
            .map_or_else(Interval::refused, SplineCoeffs::derivative_domain_hull);
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
    /// The knot vector's degree is below 2, so the curve has no second
    /// derivative to hull.
    DegreeBelowTwo,
    /// The derivative knot vector does not materialise as a clamped
    /// vector of degree `p − 1`.
    DerivativeKnotVector,
    /// The iterated difference-coefficient hull is poisoned or
    /// unbounded (a non-finite control point, a zero knot difference).
    PoisonedHull,
}
