//! The [`SpanLocate`] seam: per-instantiation span selection for
//! B-spline evaluation.
//!
//! Generic evaluation code cannot compare a scalar `t` against knots —
//! [`Real`] is comparison-free by design (Q1), and C12.8
//! forbids growing its surface. Span selection is therefore a
//! **structure-selection seam implemented per base scalar**, exactly
//! like `floor`/`copysign`'s per-instantiation kink handling in
//! [`crate::dual`]: each scalar answers "which spans does this value
//! overlap?" in its own honest way, and generic evaluation consumes the
//! answer (a validated span range — structure) without branching on
//! values.
//!
//! A **poison** scalar locates nothing: the locator answers `None`, and
//! every caller answers what its type already gives for poison (an
//! evaluator its NaN/NaI point). Poison flows through values, never
//! through the choice of a span.
//!
//! Per-instantiation behavior:
//!
//! - **`f64`**: binary search with the fixed tie-break of
//!   [`KnotVector::span_at`] — at a knot value, the span *starting*
//!   there; last span closed. One span; none at NaN.
//! - **`Probe`**: via its `f64` (it *is* an `f64` with a recorder).
//! - **`Interval`**: the inclusive hull of the
//!   spans overlapped by `[lo, hi]` — sound containment (evaluating
//!   every overlapped span's polynomial extension over the box and
//!   hulling contains the true image; each span's extension agrees
//!   with the curve on the span itself). It reads the **bracket**, not
//!   the certificate: a `Trv` enclosure is sound and still locates, and
//!   its decoration rides through to the reader that certifies. A NaI
//!   or empty enclosure has no bracket and locates nothing.
//! - **`Dual<T>`**: the value channel's spans, verbatim — the ratified
//!   kink convention ("the derivative of the program as evaluated"):
//!   at a knot, `Dual<f64>` differentiates the polynomial of the span
//!   the tie-break selects, exactly as `floor` differentiates the
//!   plateau it evaluates on. `Dual<Interval>` inherits the interval
//!   hull with **channel-independent** hulling (see
//!   [`SpanLocate::enclosure_hull`]) — the derivative channel gets the
//!   hull of the spans' derivative enclosures, never a min/max
//!   selection that could drop a branch.
//!
//! The trait is **sealed** (the private supertrait): it exists for the
//! kernel's own scalars, is not a public extension point, and carries
//! no API commitment beyond what the NURBS evaluators need. It is a
//! [`crate::Decide`] supertrait so that decision-capable generic code
//! (`T: Decide`, the topology layer's bound) can evaluate NURBS
//! carriers without naming it; evaluation-only generic code that needs
//! NURBS uses `T: SpanLocate` as the parameter's **sole bound** (the
//! same style rule as [`crate::Bounds`] — it is a `Real` subtrait, so
//! the ring operations come with it).

use super::algebra::RingRefinement;
use super::knots::{KnotVector, Span};
use crate::real::Real;

/// The answer an evaluator gives for the poison parameter `seed`: `seed`
/// times NaN, which is poison in every channel `seed` carries. A bare
/// `T::from_f64(f64::NAN)` is not that at the dual scalar — it is a dual
/// CONSTANT, whose derivative channel is a zero that reads as data.
pub fn poison_from<T: Real>(seed: T) -> T {
    seed * T::from_f64(f64::NAN)
}

/// Seals [`SpanLocate`]: implemented for exactly the kernel scalars.
pub(crate) mod sealed {
    /// The sealing supertrait (pub-in-private: unnameable downstream).
    pub trait Sealed {}
    impl Sealed for f64 {}
    #[cfg(feature = "probe")]
    impl Sealed for crate::k_stats::Probe {}
    impl<T: Sealed> Sealed for crate::dual::Dual<T> {}
    impl<T: Sealed> Sealed for crate::sym::Sym<T> {}
    impl Sealed for crate::interval::Interval {}
}

/// The inclusive range of knot spans a scalar value overlaps —
/// **structure** output, consumed by span-restricted evaluation.
/// `first == last` for point scalars.
///
/// Both ends are [`Span`]s rather than bare indices: span validity
/// originates *here*, in the locator, and every locator route is
/// [`KnotVector::span_at`] or [`KnotVector::span_range`]. Carrying the proof out
/// means no consumer re-derives *nonemptiness* or the window base — an
/// evaluator taking a `usize` would have to. The set borrows the
/// `knots` it was located in, through both of its spans, so a consumer
/// has nothing to re-pair either: the vector each end is a proof about
/// travels with it. Iterate the interior with
/// `first.index() + 1 ..= last.index()` and [`KnotVector::span`],
/// which refuses the empty spans in between.
/// `Debug` comes from [`Span`]'s, which prints the borrow as an
/// address rather than following it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpanSet<'a> {
    /// The first overlapped span.
    pub first: Span<'a>,
    /// The last overlapped span (inclusive; `≥ first`).
    pub last: Span<'a>,
}

/// Per-instantiation span selection (module docs) — the seam that lets
/// enum-level NURBS evaluators run at every kernel scalar without
/// giving [`Real`] a comparison surface.
pub trait SpanLocate: sealed::Sealed + Real {
    /// The spans of `knots` this value overlaps, or `None` for a poison
    /// value (module docs for the per-scalar semantics;
    /// [`KnotVector::span_at`] for the f64 tie-break).
    ///
    /// The lifetime is on the method, not the trait: the answer
    /// borrows the vector it located in, and the scalar implementing
    /// this holds nothing.
    fn locate_spans<'a>(self, knots: &'a KnotVector) -> Option<SpanSet<'a>>;

    /// Combines two per-span evaluation results into one enclosure.
    /// Through the evaluators it is invoked only when
    /// [`SpanLocate::locate_spans`] returned more than one span
    /// (interval-natured scalars), so a point scalar reaches it there
    /// by no path. **It is not unreachable for a point scalar in
    /// general**, and nothing here may be read as saying so: callers
    /// hull two chart parameters directly — `topo`'s `chart_edge`
    /// mints an `Envelope` image that way on both of its arms — and at
    /// `f64`, `Probe` or `Dual<f64>` the answer is the poison value,
    /// which then propagates through `Real::min`/`max` by the NaN
    /// policy that `crate::real` pins. A hull of two distinct point
    /// results is not a point, and answering one would fabricate data,
    /// so poison is the honest reply at every reachable call.
    ///
    /// For `Interval` this is the convex hull with poison-first
    /// semantics (NaI/empty propagate, matching the tangent-hull
    /// convention in `crate::interval`); for `Dual<T>` it hulls the
    /// value and derivative channels **independently** (a min/max
    /// selection would follow one branch's tangent and drop the
    /// other's — unsound for a straddling enclosure).
    fn enclosure_hull(self, other: Self) -> Self;

    /// One homogeneous channel (`w`, or one coordinate of `w·P`) through
    /// `refinement` at this scalar, per scalar as the locator is. At
    /// `Interval` it is the certification ring's refinement
    /// ([`RingRefinement::refine`]), so the result encloses the
    /// described curve's refined channel through the ring's one step.
    /// At every other scalar it is [`CurvePlan::apply_homogeneous`](super::CurvePlan::apply_homogeneous) per
    /// plan, both Boehm ratios formed in the scalar. Either way a
    /// channel that is not one entry per source control point refuses:
    /// every entry of the refined length is poison.
    fn refine_homogeneous(refinement: &RingRefinement, channel: &[Self]) -> Vec<Self> {
        if channel.len() != refinement.source_count() {
            return vec![Self::from_f64(f64::NAN); refinement.knots().control_count()];
        }
        refinement
            .plans()
            .iter()
            .fold(channel.to_vec(), |c, plan| plan.apply_homogeneous(&c))
    }

    /// A rational curve's stored `weights` through `refinement` at this
    /// scalar, or `None` unless every refined weight is certainly
    /// positive: the weights a de-homogenization divides by, and the
    /// licence of a rational hull. One refinement at `f64` (the check
    /// reads the weights it returns) and at `Interval` (the ring's,
    /// checked by [`certainly_positive`](super::algebra::certainly_positive)). At every other scalar the
    /// check reads the `f64` refinement, the scalar's own value, and
    /// the weights are [`SpanLocate::refine_homogeneous`]'s.
    fn refine_weights(refinement: &RingRefinement, weights: &[f64]) -> Option<Vec<Self>> {
        f64::refine_weights(refinement, weights)?;
        let lifted: Vec<Self> = weights.iter().map(|w| Self::from_f64(*w)).collect();
        Some(Self::refine_homogeneous(refinement, &lifted))
    }
    /// The one `f64` this value locates by, or `None` when it is no
    /// single point: poison, or an enclosure wider than a point.
    ///
    /// This is how knot values held in the scalar give the `f64` knot
    /// vector its value channel ([`super::Knots::new`]): the read the
    /// locator itself makes, and nothing else. An interval answers only for a point
    /// bracket, because a wide one names no knot structure, and a
    /// midpoint would invent one.
    fn point_value(self) -> Option<f64>;
}

impl SpanLocate for f64 {
    fn locate_spans<'a>(self, knots: &'a KnotVector) -> Option<SpanSet<'a>> {
        let span = knots.span_at(self)?;
        Some(SpanSet {
            first: span,
            last: span,
        })
    }

    fn enclosure_hull(self, _other: Self) -> Self {
        // Unreachable through the evaluators (single-span locator);
        // total anyway — poison, never a fabricated point value.
        f64::NAN
    }

    fn refine_weights(refinement: &RingRefinement, weights: &[f64]) -> Option<Vec<Self>> {
        let w = Self::refine_homogeneous(refinement, weights);
        // `!(w > 0.0)` catches NaN too.
        #[allow(clippy::neg_cmp_op_on_partial_ord)]
        let refused = w.iter().any(|w| !(*w > 0.0) || !w.is_finite());
        (!refused).then_some(w)
    }

    fn point_value(self) -> Option<f64> {
        (!self.is_nan()).then_some(self)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::dual::{Dual, Dual64};

    fn kv() -> KnotVector {
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 2.0, 2.0, 2.0], 2).unwrap()
    }

    /// `Span`'s fields are private (that is the point), so the
    /// locator's answer is compared as the pair of indices it names.
    fn indices(set: Option<SpanSet<'_>>) -> (usize, usize) {
        let set = set.expect("a numeric value locates");
        (set.first.index(), set.last.index())
    }

    #[test]
    fn f64_locates_one_span_with_the_span_at_tie_break() {
        let k = kv();
        assert_eq!(indices(0.5.locate_spans(&k)), (2, 2));
        assert_eq!(indices(1.0.locate_spans(&k)), (3, 3));
        assert_eq!(indices(2.0.locate_spans(&k)), (3, 3));
        assert!(f64::NAN.locate_spans(&k).is_none(), "NaN located a span");
        assert!(0.5f64.enclosure_hull(0.6).is_nan());
        assert_eq!(0.5f64.point_value(), Some(0.5));
        assert_eq!(f64::NAN.point_value(), None, "NaN read as a point");
    }

    /// The `Probe` half is behind the `probe` feature (the recording
    /// scalar is opt-in); the `Dual` half is not, so this test keeps
    /// asserting the dual value-channel in a default build rather than
    /// disappearing with the feature.
    #[test]
    fn probe_and_dual_follow_their_value_channels() {
        let k = kv();
        #[cfg(feature = "probe")]
        {
            let p = crate::k_stats::Probe(1.5);
            assert_eq!(indices(p.locate_spans(&k)), (3, 3));
        }
        let d = Dual64::variable(1.0);
        assert_eq!(indices(d.locate_spans(&k)), (3, 3));
        assert_eq!(d.point_value(), Some(1.0), "the dual's value channel");
        assert_eq!(Dual::new(f64::NAN, 1.0).point_value(), None);
        assert!(
            Dual::new(f64::NAN, 1.0).locate_spans(&k).is_none(),
            "a poisoned value channel located a span"
        );
        // Dual hulls channel-wise: over f64 channels that is poison.
        let h = Dual::new(0.25, 1.0).enclosure_hull(Dual::new(0.75, -1.0));
        assert!(h.value.is_nan() && h.deriv.is_nan());
    }
}
