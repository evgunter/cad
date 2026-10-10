//! The PROJECTIVE knot algebra's scalars, and the homogeneous applier
//! at a generic scalar.
//!
//! [`crate::spline::algebra`] computes every knot-algebra plan as `f64`
//! structure. Its two appliers combine control data off one schedule:
//! [`CurvePlan::apply_points`] in the projective form, at the scalars
//! named by [`ProjectiveScalar`] only, and [`CurvePlan::apply_certified`]
//! in the homogeneous form, in the certification ring.
//! [`CurvePlan::apply_homogeneous`] is the homogeneous form at a
//! caller's generic scalar.

use super::algebra::CurvePlan;
use super::locate::SpanLocate;
use crate::real::Real;

/// The scalars the PROJECTIVE knot algebra means something at: `f64`,
/// `Probe`, and `Dual` and `Sym` over them — not `Interval`, and not
/// `Dual<Interval>`.
///
/// [`CurvePlan::apply_points`] combines de-homogenized points with
/// [`CurvePlan::weights`]' stored `f64` quotient `λ`. At an evaluation
/// scalar that is the fixed association the tree documents. In the
/// certification ring it would enclose `x + (y − x)·fl(λ)` under the
/// rounded `f64` weights the plan minted, which is a neighbour of the
/// described carrier and not that carrier (C6: a refinement inside a
/// certificate is held only as homogeneous enclosures `(w·P, w)`,
/// [`CurvePlan::apply_certified`]). So the applier, and every knot
/// operation built on it, is bounded by this trait, and a call at the
/// certification scalar does not typecheck:
///
/// ```compile_fail,E0277
/// use geom_core::Interval;
/// use geom_core::spline::{KnotVector, algebra::refine_plan};
/// let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
/// let plan = refine_plan(&kv, &[1.0, 1.0, 1.0], &[0.5]).unwrap().remove(0);
/// let old = [Interval::point(0.0), Interval::point(1.0), Interval::point(2.0)];
/// let _ = plan.apply_points(&old, Interval::refused(), |x, y, l: Interval| x + (y - x) * l);
/// ```
///
/// Its twin at an evaluation scalar compiles:
///
/// ```
/// use geom_core::spline::{KnotVector, algebra::refine_plan};
/// let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
/// let plan = refine_plan(&kv, &[1.0, 1.0, 1.0], &[0.5]).unwrap().remove(0);
/// let refined = plan.apply_points(&[0.0, 1.0, 2.0], f64::NAN, |x, y, l: f64| x + (y - x) * l);
/// assert_eq!(refined.len(), 4);
/// ```
///
/// A methodless sealed marker over [`SpanLocate`]: it adds no
/// comparison surface, and it is a statement about the kernel's own
/// scalars, not an extension point.
pub trait ProjectiveScalar: SpanLocate + sealed::Sealed {}

mod sealed {
    /// The sealing supertrait (pub-in-private: unnameable downstream).
    pub trait Sealed {}
}

impl sealed::Sealed for f64 {}
impl ProjectiveScalar for f64 {}
#[cfg(feature = "probe")]
impl sealed::Sealed for crate::k_stats::Probe {}
#[cfg(feature = "probe")]
impl ProjectiveScalar for crate::k_stats::Probe {}
impl<T: ProjectiveScalar> sealed::Sealed for crate::dual::Dual<T> {}
impl<T: ProjectiveScalar> ProjectiveScalar for crate::dual::Dual<T> where
    crate::dual::Dual<T>: SpanLocate
{
}
impl<T: ProjectiveScalar> sealed::Sealed for crate::sym::Sym<T> {}
impl<T: ProjectiveScalar> ProjectiveScalar for crate::sym::Sym<T> {}

impl CurvePlan {
    /// **The same homogeneous schedule at the caller's scalar**, for a
    /// caller generic over [`Real`] that refines a homogeneous channel
    /// (`w`, or one coordinate of `w·P`): each step is
    /// `β·x + α·y` with `α = (u − U_j)/Δ` and `β = (U_{j+p} − u)/Δ`
    /// taken in `T` from the knots, so at `Interval` both ratios are
    /// outward-rounded enclosures and the result encloses the refined
    /// channel of the described curve, with no `f64` ratio standing in.
    /// It has no hull meet, which `Real` cannot spell, so at `Interval`
    /// it is the looser of the two: a caller that holds `Interval`
    /// channels uses [`CurvePlan::apply_certified`].
    ///
    /// Total: a ratio-less step (elevation, removal), a malformed plan
    /// or a short channel yields `T`'s poison (`from_f64(NaN)`) in the
    /// targets it reaches.
    pub fn apply_homogeneous<T: Real>(&self, old: &[T]) -> Vec<T> {
        self.apply_ratios(old, T::from_f64(f64::NAN), |x, y, r| {
            let (lo, hi, u) = (
                T::from_f64(r.lo),
                T::from_f64(r.hi),
                T::from_f64(r.inserted),
            );
            let span = hi - lo;
            x * ((hi - u) / span) + y * ((u - lo) / span)
        })
    }
}
