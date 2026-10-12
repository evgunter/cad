//! The PROJECTIVE knot algebra's scalars, and the homogeneous applier
//! at a generic scalar.
//!
//! [`crate::spline::algebra`] decides every knot-algebra plan as `f64`
//! structure. Its appliers combine control data off one schedule:
//! [`CurvePlan::apply_rational`] in the projective form, at the scalars
//! named by [`ProjectiveScalar`] only, over the knots and weights held
//! there; [`CurvePlan::apply_points`] the same form on the plan's own
//! `f64` channel; and [`CurvePlan::apply_certified`] in the homogeneous
//! form, in the certification ring.
//! [`CurvePlan::apply_homogeneous_in`] is the homogeneous form at a
//! caller's generic scalar, over the knots held there.

use super::algebra::{Coef, CurvePlan, Form, KnotAt};
use super::locate::SpanLocate;
use crate::real::Real;
use crate::{Point2, Point3};

/// The scalars the PROJECTIVE knot algebra means something at: `f64`,
/// `Probe`, and `Dual` and `Sym` over them — not `Interval`, and not
/// `Dual<Interval>`.
///
/// [`CurvePlan::apply_rational`] combines de-homogenized points with
/// a quotient `λ` it forms from the held weights. At an evaluation
/// scalar that is the fixed association the tree documents. In the
/// certification ring it would enclose a de-homogenized combination
/// of the plan's projective form, which is not how a certificate holds
/// a refinement (C6: a refinement inside a certificate is held only as
/// homogeneous enclosures `(w·P, w)`, [`CurvePlan::apply_certified`]).
/// So the applier, and every knot operation built on it, is bounded by
/// this trait, and a call at the certification scalar does not
/// typecheck:
///
/// ```compile_fail,E0277
/// use geom_core::Interval;
/// use geom_core::interval::certification::Certification;
/// use geom_core::spline::{KnotVector, algebra::refine_plan};
/// let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
/// let plan = refine_plan(&kv, &[1.0, 1.0, 1.0], &[0.5]).unwrap().remove(0);
/// let pt = |x: f64| Interval::from_bounds(x, x);
/// let knots: Vec<Interval> = kv.knots().iter().map(|k| pt(*k)).collect();
/// let old = [pt(0.0), pt(1.0), pt(2.0)];
/// let _ = plan.apply_rational(
///     &knots,
///     &old,
///     &[pt(1.0); 3],
///     (Interval::refused(), &pt),
///     |x, y, l: Interval| x + (y - x) * l,
/// );
/// ```
///
/// The points are bounded with it ([`ProjectivePoint`]), so the
/// scalar cannot stay at an evaluation scalar while the points it
/// combines are ring enclosures: the shape that lifts `λ` inside the
/// closure (an `f64` scalar over `Interval` points) does not typecheck
/// either:
///
/// ```compile_fail,E0277
/// use geom_core::Interval;
/// use geom_core::interval::certification::Certification;
/// use geom_core::spline::{KnotVector, algebra::refine_plan};
/// let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
/// let plan = refine_plan(&kv, &[1.0, 1.0, 1.0], &[0.5]).unwrap().remove(0);
/// let pt = |x: f64| Interval::from_bounds(x, x);
/// let old = [pt(0.0), pt(1.0), pt(2.0)];
/// let _ = plan.apply_rational::<f64, Interval>(
///     kv.knots(),
///     &old,
///     &[1.0; 3],
///     (Interval::refused(), &|v| v),
///     |x, y, l| x + (y - x) * Interval::from_bounds(l, l),
/// );
/// ```
///
/// Nor does [`CurvePlan::apply_points`], the plan's own `f64` reference,
/// combine ring enclosures with its `f64` `λ`:
///
/// ```compile_fail,E0277
/// use geom_core::Interval;
/// use geom_core::interval::certification::Certification;
/// use geom_core::spline::{KnotVector, algebra::refine_plan};
/// let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
/// let plan = refine_plan(&kv, &[1.0, 1.0, 1.0], &[0.5]).unwrap().remove(0);
/// let pt = |x: f64| Interval::from_bounds(x, x);
/// let old = [pt(0.0), pt(1.0), pt(2.0)];
/// let _ = plan.apply_points(&old, Interval::refused(), |x, y, l: f64| {
///     x + (y - x) * Interval::from_bounds(l, l)
/// });
/// ```
///
/// What the bound cannot see is a closure that computes in the ring
/// and carries the result out through a capture rather than its
/// return value: its inputs and output are at an evaluation scalar.
///
/// Its twin at an evaluation scalar compiles:
///
/// ```
/// use geom_core::spline::{KnotVector, algebra::refine_plan};
/// let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
/// let plan = refine_plan(&kv, &[1.0, 1.0, 1.0], &[0.5]).unwrap().remove(0);
/// let (refined, weights) = plan.apply_rational(
///     kv.knots(),
///     &[0.0, 1.0, 2.0],
///     &[1.0; 3],
///     (f64::NAN, &|v| v),
///     |x, y, l: f64| x + (y - x) * l,
/// );
/// assert_eq!((refined.len(), weights.len()), (4, 4));
/// ```
///
/// A methodless sealed marker over [`SpanLocate`]: it adds no
/// comparison surface, and it is a statement about the kernel's own
/// scalars, not an extension point.
pub trait ProjectiveScalar: SpanLocate + sealed::Sealed {}

mod sealed {
    /// The sealing supertrait (pub-in-private: unnameable downstream).
    pub trait Sealed {}

    /// [`super::ProjectivePoint`]'s sealing supertrait.
    pub trait SealedPoint<T> {}
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

/// **A computation with one half per side of the projective
/// partition**, run by [`SpanLocate::split_projective`] at the scalar
/// `S`: [`Self::projective`] where `S` is a [`ProjectiveScalar`], so the
/// half may call the projective knot algebra, and [`Self::enclosure`]
/// where `S`'s value channel is an enclosure (`Interval`, and `Dual`
/// and `Sym` over it), where that algebra does not typecheck. No value
/// is read to choose: the scalar's type answers.
pub trait ProjectiveSplit<S: SpanLocate> {
    /// What either half returns.
    type Out;

    /// The half run at a projective scalar.
    fn projective(self) -> Self::Out
    where
        S: ProjectiveScalar;

    /// The half run at an enclosure scalar.
    fn enclosure(self) -> Self::Out;
}

/// `Dual<T>`'s answer is its base's: `arm` runs at `Dual<T>` on the
/// half `T` takes (a projective `T` makes `Dual<T>` projective).
pub(crate) struct DualArm<A>(pub(crate) A);

impl<T, A> ProjectiveSplit<T> for DualArm<A>
where
    T: SpanLocate + crate::dual::KinkJacobian,
    A: ProjectiveSplit<crate::dual::Dual<T>>,
{
    type Out = A::Out;

    fn projective(self) -> A::Out
    where
        T: ProjectiveScalar,
    {
        self.0.projective()
    }

    fn enclosure(self) -> A::Out {
        self.0.enclosure()
    }
}

/// `Sym<T>`'s answer is its base's, as [`DualArm`].
pub(crate) struct SymArm<A>(pub(crate) A);

impl<T, A> ProjectiveSplit<T> for SymArm<A>
where
    T: SpanLocate,
    A: ProjectiveSplit<crate::sym::Sym<T>>,
{
    type Out = A::Out;

    fn projective(self) -> A::Out
    where
        T: ProjectiveScalar,
    {
        self.0.projective()
    }

    fn enclosure(self) -> A::Out {
        self.0.enclosure()
    }
}

/// The control data [`CurvePlan::apply_rational`] combines at the
/// projective scalar `T` (and [`CurvePlan::apply_points`] at `f64`):
/// `T` itself, and the points and coordinate
/// arrays over it. The applier's `λ` and its points share one scalar,
/// so neither can be a certification enclosure ([`ProjectiveScalar`]).
///
/// Sealed: a statement about the kernel's own point types, not an
/// extension point.
pub trait ProjectivePoint<T: ProjectiveScalar>: Copy + sealed::SealedPoint<T> {}

impl<T: ProjectiveScalar> sealed::SealedPoint<T> for T {}
impl<T: ProjectiveScalar> ProjectivePoint<T> for T {}
impl<T: ProjectiveScalar> sealed::SealedPoint<T> for Point2<T> {}
impl<T: ProjectiveScalar> ProjectivePoint<T> for Point2<T> {}
impl<T: ProjectiveScalar> sealed::SealedPoint<T> for Point3<T> {}
impl<T: ProjectiveScalar> ProjectivePoint<T> for Point3<T> {}
impl<T: ProjectiveScalar, const N: usize> sealed::SealedPoint<T> for [T; N] {}
impl<T: ProjectiveScalar, const N: usize> ProjectivePoint<T> for [T; N] {}

impl CurvePlan {
    /// **The same homogeneous schedule at the caller's scalar**, for a
    /// caller generic over [`Real`] that refines a homogeneous channel
    /// (`w`, or one coordinate of `w·P`): each step is
    /// `β·x + α·y` with `α = (u − U_j)/Δ` and `β = (U_{j+p} − u)/Δ`
    /// taken in `T` from `knots`, the knot values of the vector this
    /// plan was made against (one per knot), and the inserted value:
    /// a breakpoint already present is read there, so it stays one
    /// quantity, and a new one enters as the constant of the schedule
    /// it is. At `Interval` both ratios are outward-rounded
    /// enclosures and the result encloses the refined channel of the
    /// described curve, with no `f64` ratio standing in. It has no
    /// hull meet, which `Real` cannot spell, so it is not the ring's
    /// step: a caller that holds `Interval` channels refines through
    /// [`RingRefinement`](super::RingRefinement).
    ///
    /// Total: a ratio-less step (elevation, removal), a malformed plan,
    /// a short channel or a knot index past `knots` yields `T`'s poison
    /// (`from_f64(NaN)`) in the targets it reaches.
    pub fn apply_homogeneous_in<T: Real>(&self, knots: &[T], old: &[T]) -> Vec<T> {
        self.apply_ratios(old, T::from_f64(f64::NAN), |x, y, _, form| {
            let Form::Blend(Coef::Knots { u, lo, hi }) = form else {
                return None;
            };
            let u = match u {
                KnotAt::Index(i) => *knots.get(i)?,
                KnotAt::Fresh(v) => T::from_f64(v),
            };
            let (lo, hi) = (*knots.get(lo)?, *knots.get(hi)?);
            let span = hi - lo;
            Some(x * ((hi - u) / span) + y * ((u - lo) / span))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{ProjectiveScalar, ProjectiveSplit, SpanLocate};
    use crate::dual::Dual;
    use crate::{Interval, Sym};

    /// Names the half the dispatch ran.
    struct Which;

    impl<S: SpanLocate> ProjectiveSplit<S> for Which {
        type Out = &'static str;

        fn projective(self) -> &'static str
        where
            S: ProjectiveScalar,
        {
            "projective"
        }

        fn enclosure(self) -> &'static str {
            "enclosure"
        }
    }

    #[test]
    fn a_wrapper_takes_its_bases_half() {
        assert_eq!(
            [
                f64::split_projective(Which),
                Dual::<f64>::split_projective(Which),
                Sym::<f64>::split_projective(Which),
                Interval::split_projective(Which),
                Dual::<Interval>::split_projective(Which),
                Sym::<Interval>::split_projective(Which),
            ],
            [
                "projective",
                "projective",
                "projective",
                "enclosure",
                "enclosure",
                "enclosure",
            ]
        );
    }
}
