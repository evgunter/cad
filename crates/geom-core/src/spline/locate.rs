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
//! Per-instantiation behavior:
//!
//! - **`f64`**: binary search with the fixed tie-break of
//!   [`KnotVector::find_span`] — at a knot value, the span *starting*
//!   there; last span closed. One span, always.
//! - **`Probe`**: via its `f64` (it *is* an `f64` with a recorder).
//! - **`Interval`** (feature `interval`): the inclusive hull of the
//!   spans overlapped by `[lo, hi]` — sound containment (evaluating
//!   every overlapped span's polynomial extension over the box and
//!   hulling contains the true image; each span's extension agrees
//!   with the curve on the span itself).
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

use super::knots::{KnotVector, Span};
use crate::real::Real;

/// Seals [`SpanLocate`]: implemented for exactly the kernel scalars.
pub(crate) mod sealed {
    /// The sealing supertrait (pub-in-private: unnameable downstream).
    pub trait Sealed {}
    impl Sealed for f64 {}
    #[cfg(feature = "probe")]
    impl Sealed for crate::k_stats::Probe {}
    impl<T: Sealed> Sealed for crate::dual::Dual<T> {}
    #[cfg(feature = "interval")]
    impl Sealed for crate::interval::Interval {}
}

/// The inclusive range of knot spans a scalar value overlaps —
/// **structure** output, consumed by span-restricted evaluation.
/// `first == last` for point scalars.
///
/// Both ends are [`Span`]s rather than bare indices: span validity
/// originates *here*, in the locator, and every locator route is
/// [`KnotVector::span_at`], which is total. Carrying the proof out
/// means no consumer re-derives *nonemptiness* or the window base — an
/// evaluator taking a `usize` would have to. What a consumer does
/// still check is that the span belongs to the vector it is evaluating
/// against ([`KnotVector::admits`]), which a `SpanSet` cannot say
/// because it carries no borrow of the `knots` it was located in.
/// Iterate the interior with `first.index() + 1 ..= last.index()` and
/// [`KnotVector::span`], which refuses the empty spans in between.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpanSet {
    /// The first overlapped span.
    pub first: Span,
    /// The last overlapped span (inclusive; `≥ first`).
    pub last: Span,
}

/// Per-instantiation span selection (module docs) — the seam that lets
/// enum-level NURBS evaluators run at every kernel scalar without
/// giving [`Real`] a comparison surface.
pub trait SpanLocate: sealed::Sealed + Real {
    /// The spans of `knots` this value overlaps (module docs for the
    /// per-scalar semantics; [`KnotVector::find_span`] for the f64
    /// tie-break and NaN totalization).
    fn locate_spans(self, knots: &KnotVector) -> SpanSet;

    /// Combines two per-span evaluation results into one enclosure —
    /// invoked only when [`SpanLocate::locate_spans`] returned more
    /// than one span (interval-natured scalars). Point scalars
    /// (`f64`, `Probe`, `Dual<f64>`) never reach it by construction
    /// (single span); their implementation is the poison value — a
    /// hull of two distinct point results is not a point, and
    /// answering one would fabricate data.
    ///
    /// For `Interval` this is the convex hull with poison-first
    /// semantics (NaI/empty propagate, matching the tangent-hull
    /// convention in `crate::interval`); for `Dual<T>` it hulls the
    /// value and derivative channels **independently** (a min/max
    /// selection would follow one branch's tangent and drop the
    /// other's — unsound for a straddling enclosure).
    fn enclosure_hull(self, other: Self) -> Self;
}

impl SpanLocate for f64 {
    fn locate_spans(self, knots: &KnotVector) -> SpanSet {
        let span = knots.span_at(self);
        SpanSet {
            first: span,
            last: span,
        }
    }

    fn enclosure_hull(self, _other: Self) -> Self {
        // Unreachable through the evaluators (single-span locator);
        // total anyway — poison, never a fabricated point value.
        f64::NAN
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
    fn indices(set: SpanSet) -> (usize, usize) {
        (set.first.index(), set.last.index())
    }

    #[test]
    fn f64_locates_one_span_with_the_find_span_tie_break() {
        let k = kv();
        assert_eq!(indices(0.5.locate_spans(&k)), (2, 2));
        assert_eq!(indices(1.0.locate_spans(&k)), (3, 3));
        assert_eq!(indices(2.0.locate_spans(&k)), (3, 3));
        assert!(0.5f64.enclosure_hull(0.6).is_nan());
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
        // Dual hulls channel-wise: over f64 channels that is poison.
        let h = Dual::new(0.25, 1.0).enclosure_hull(Dual::new(0.75, -1.0));
        assert!(h.value.is_nan() && h.deriv.is_nan());
    }
}
