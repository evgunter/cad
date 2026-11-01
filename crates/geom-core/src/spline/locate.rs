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

use super::knots::{KnotVector, Span};
use crate::real::Real;

/// The answer an evaluator gives for the poison parameter `seed`: `seed`
/// times NaN, which is poison in every channel `seed` carries. A bare
/// `T::from_f64(f64::NAN)` is not that at the dual scalar — it is a dual
/// CONSTANT, whose derivative channel is a zero that reads as data.
pub fn poison_from<T: Real>(seed: T) -> T {
    seed * T::from_f64(f64::NAN)
}

/// Whether `b` is above `a` for every value they stand for, as a
/// structure decision (C6): at points, `a < b` on their value channels,
/// so no arithmetic is formed at a point scalar; otherwise
/// [`SpanLocate::structure_positive`] of `b − a`.
pub fn structure_ascends<T: SpanLocate>(a: T, b: T) -> bool {
    match (a.point_value(), b.point_value()) {
        (Some(a), Some(b)) => a < b,
        _ => (b - a).structure_positive(),
    }
}

/// Whether `x` is nonzero for every value it stands for, as a structure
/// decision (C6): at a point, `x != 0` on its value channel; otherwise
/// [`SpanLocate::structure_positive`] of `x` or of `−x`. Poison is not
/// nonzero.
pub fn structure_nonzero<T: SpanLocate>(x: T) -> bool {
    match x.point_value() {
        Some(v) => v != 0.0,
        None => x.structure_positive() || (-x).structure_positive(),
    }
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
/// [`KnotVector::span_at`] or [`KnotVector::span_range`] over point
/// knots, or [`SpanLocate::locate_held`] over held knots wider than a
/// point. Carrying the proof out means no consumer re-derives
/// *nonemptiness* or the window base — an evaluator taking a `usize`
/// would have to. The set borrows the structure it was located in,
/// through both of its spans, so a consumer has nothing to re-pair
/// either: the structure each end is a proof about travels with it. Iterate the interior with
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
    /// hull two chart parameters directly, and at
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

    /// The Bernstein coefficients over `[a, b]` of the piece one
    /// coefficient channel defines on `span`, read from `window`, the
    /// channel's `p + 1` coefficients on that span: coefficient `k` is
    /// the blossom at `a` taken `p − k` times and `b` taken `k` times.
    /// `held` is the curve's knot values in the scalar (its
    /// [`Knots::values`](super::Knots::values)), and `held_ends` the ends `ends` in the scalar
    /// (a breakpoint's held value, any other end its constant).
    ///
    /// Per scalar, as the locator is. At `Interval` it is the window's
    /// certified restriction ([`super::SpanRow::restrict`]), whose every
    /// coefficient encloses the true one. It reads the knots' `f64`
    /// vector (C6's interim), so it runs on point knots, which every
    /// carrier's are.
    /// At every other scalar it is de Boor's recurrence over the held
    /// knots and ends, the ratio `α` formed in the scalar and the fixed
    /// association `x·(1 − α) + y·α` (D9).
    ///
    /// **Every scalar refuses the same inputs**, read on the `f64`
    /// structure: a window that is not `p + 1` long, an end outside the
    /// span's closure, and held knots that are not the vector's length
    /// answer poison coefficients — the piece's polynomial past its span
    /// is not the curve, and an extension answered at one scalar and
    /// refused at another hides a caller that crosses a knot. The poison
    /// is the window's own ([`poison_from`] its first entry), or a bare
    /// NaN for an empty window, which has no channel to carry.
    fn restrict_window(
        window: &[Self],
        span: Span<'_>,
        held: &[Self],
        ends: (f64, f64),
        held_ends: (Self, Self),
    ) -> Vec<Self> {
        let p = span.degree();
        let knots = span.knots_f64_interim().knots();
        // Indexing justified: `p ≤ index ≤ last_span() = len − p − 2`.
        let (lo, hi) = (knots[span.index()], knots[span.index() + 1]);
        let inside = |x: f64| lo <= x && x <= hi;
        if window.len() != p + 1 || held.len() != knots.len() || !inside(ends.0) || !inside(ends.1)
        {
            let poison = window
                .first()
                .map_or_else(|| Self::from_f64(f64::NAN), |&c| poison_from(c));
            return vec![poison; p + 1];
        }
        let first = span.first_control();
        (0..=p)
            .map(|k| {
                let mut d = window.to_vec();
                for r in 1..=p {
                    let u = if r <= p - k { held_ends.0 } else { held_ends.1 };
                    for i in (r..=p).rev() {
                        // `first + i + p + 1 − r ≤ index + p`, inside the vector.
                        let (lo, hi) = (held[first + i], held[first + i + p + 1 - r]);
                        let alpha = (u - lo) / (hi - lo);
                        d[i] = d[i - 1] * (Self::one() - alpha) + d[i] * alpha;
                    }
                }
                d[p]
            })
            .collect()
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

    /// The `f64` this value IS, with nothing else carried, or `None`.
    /// Stricter than [`SpanLocate::point_value`], which reads only the
    /// value channel: a `Dual` answers only with a derivative of exactly
    /// zero, and a `Sym` only when its node is the literal of its value,
    /// so a value that carries a parameter or a symbol in this scalar is
    /// never read as a constant. (A value read out to `f64` and entered
    /// again through `from_f64` IS a literal, at `Sym` as everywhere:
    /// what it carried was dropped at that read, not here.) A structure
    /// decision that holds only for genuinely equal values (equal plain
    /// constant NURBS weights cancel out of the curve) reads this, not
    /// the value channel. `None` is always the conservative answer.
    /// Declared, never defaulted: a wrapper has to say what it carries
    /// beyond its value.
    fn plain_constant(self) -> Option<f64>;

    /// **The closed bracket of every value this scalar stands for**, or
    /// `None` for poison: `(v, v)` at a point, the enclosure's ends at
    /// `Interval`, the value part's at the wrappers. A structure
    /// refusal carries it as its margin (C6), so a refinement of the
    /// inputs that narrows the bracket can lift the refusal.
    fn structure_bracket(self) -> Option<(f64, f64)> {
        self.point_value().map(|v| (v, v))
    }

    /// **The spans of a carrier whose knots are held in the scalar** and
    /// may be wider than a point (C6): every nonempty span some member's
    /// parameter can lie in, for some member's knots, hulled as the
    /// first and the last. Span `i` can hold the parameter when its
    /// lower knot's bracket starts at or below the parameter's top and
    /// the parameter's bottom lies below its upper knot's bracket top;
    /// the first span is closed below and the last above, as
    /// [`KnotVector::span_at`] clamps. At point knots and a point
    /// parameter that is the `f64` tie-break, the span starting at a
    /// knot. `None` for a poison parameter or a poison knot.
    ///
    /// The spans it answers are over `structure` alone, so their
    /// [`Span::knots_f64_interim`] refuses: their positions are `held`.
    fn locate_held<'a>(
        self,
        structure: &'a super::knots::KnotStructure,
        held: &[Self],
    ) -> Option<SpanSet<'a>> {
        let (t_lo, t_hi) = self.structure_bracket()?;
        let mut brackets = Vec::with_capacity(held.len());
        for k in held {
            brackets.push(k.structure_bracket()?);
        }
        let (first_span, last_span) = (structure.first_span(), structure.last_span());
        let possible = (first_span..=last_span).filter(|&i| {
            structure.span_is_nonempty(i)
                && (i == first_span || brackets[i].0 <= t_hi)
                && (i == last_span || t_lo < brackets[i + 1].1)
        });
        let (mut first, mut last) = (None, None);
        for i in possible {
            first = first.or(Some(i));
            last = Some(i);
        }
        Some(SpanSet {
            first: structure.span(first?)?,
            last: structure.span(last?)?,
        })
    }

    /// [`SpanLocate::point_value`], or NaN where there is none: the
    /// `f64` a structure decision reads, so that a value with no point
    /// (poison, an enclosure wider than a point) takes every
    /// NaN-catching refusal arm.
    fn value_channel(self) -> f64 {
        self.point_value().unwrap_or(f64::NAN)
    }

    /// Whether every value this scalar stands for is above zero: the
    /// positivity a structure decision asks of a value (C6), such as a
    /// chord that is not empty or a parameter step that ascends. At a
    /// point scalar it is `value_channel() > 0.0`, so poison answers
    /// `false`. At `Interval` it reads the bracket, as the locator does:
    /// a wide value answers by its lower end. The wrappers answer by
    /// their value part.
    ///
    /// `false` is "not every value", not "no value": a caller that
    /// branches on it rather than refusing tells a point or poison
    /// apart from a wide value first, since a wide value's members may
    /// take both branches.
    fn structure_positive(self) -> bool {
        self.value_channel() > 0.0
    }

    /// **The homogeneous knot algebra's step**, `x + (y − x)·t`
    /// ([`CurvePlan::apply_homogeneous`](super::CurvePlan::apply_homogeneous),
    /// C6 S4): `t` is an insertion's or an elevation's blend coefficient,
    /// in `[0, 1]`, or a removal's unblend `1/a` or `1/(1 − a)`, above
    /// `1`. The expression, at every scalar but `Interval`. There, a
    /// blend's `t` inside `[0, 1]` takes the hull of the step at its
    /// box's corners, which meets the inputs' widths, so a chain of
    /// blends does not multiply the widths it inherits (the expression
    /// does: `x` enters twice); an unblend's `t` takes the expression, as
    /// the ring applier does.
    fn homogeneous_step(x: Self, y: Self, t: Self) -> Self {
        x + (y - x) * t
    }

    /// **Which side of the projective partition this scalar is on**,
    /// answered by running one of `arm`'s two halves: its
    /// [`projective`](super::ProjectiveSplit::projective) half, which
    /// may call the projective knot algebra, at `f64`, `Probe`, and
    /// `Dual` and `Sym` over them; its
    /// [`enclosure`](super::ProjectiveSplit::enclosure) half at
    /// `Interval` and the wrappers over it. A wrapper answers by its
    /// base, so the dispatch is the type's and no value is read.
    /// This value narrowed to `[lo, hi]`, where the caller knows every
    /// member it is read for lies there (a parameter on one span of held
    /// knots lies between that span's knots, C6 S5c): an enclosure of
    /// those members no wider than `self`. Its other channels (a
    /// derivative's tangent) are `self`'s, since on those members the
    /// narrowed value is `self`'s value. The default answers `self`,
    /// which encloses them already: a point scalar has nothing to
    /// narrow, and a scalar that cannot narrow soundly gives up only
    /// tightness. At `Interval` it is the intersection, decorated as
    /// `self` is; where `self` is poison or misses `[lo, hi]`, `self`.
    ///
    /// An evaluation-lane narrowing, not the certification lane's clamp,
    /// which refuses an uncertified value and an empty intersection and
    /// takes its endpoints (and its choice between zeros of opposite
    /// sign) from the backend's intersection. Here an uncertified value
    /// stays as uncertified as it came, and a miss, which no caller with
    /// a member on `[lo, hi]` can meet, gives up only tightness.
    fn clip_to(self, _lo: f64, _hi: f64) -> Self {
        self
    }

    /// Declared, never defaulted: a new scalar says which half it runs.
    fn split_projective<A: super::ProjectiveSplit<Self>>(arm: A) -> A::Out;
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

    fn point_value(self) -> Option<f64> {
        (!self.is_nan()).then_some(self)
    }

    fn plain_constant(self) -> Option<f64> {
        self.point_value()
    }

    fn split_projective<A: super::ProjectiveSplit<Self>>(arm: A) -> A::Out {
        arm.projective()
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

    /// A structure positivity holds of every value a scalar stands for:
    /// a point by its value channel, an enclosure only when its whole
    /// bracket is above zero, and a wrapper by its value part.
    #[test]
    fn structure_positive_holds_of_every_value_the_scalar_stands_for() {
        use crate::{Interval, Sym};
        let wide = |lo: f64, hi: f64| Interval::from_bounds(lo, hi);
        let cases = [
            ("a positive point", 0.5f64.structure_positive(), true),
            ("zero", 0.0f64.structure_positive(), false),
            ("NaN", f64::NAN.structure_positive(), false),
            (
                "a wide positive",
                wide(0.25, 0.75).structure_positive(),
                true,
            ),
            (
                "a straddling bracket",
                wide(-0.25, 0.75).structure_positive(),
                false,
            ),
            (
                "a bracket touching zero",
                wide(0.0, 0.75).structure_positive(),
                false,
            ),
            (
                "NaI",
                Interval::from_f64(f64::NAN).structure_positive(),
                false,
            ),
            (
                "a dual over a wide positive",
                Dual::new(wide(0.25, 0.75), wide(-1.0, 1.0)).structure_positive(),
                true,
            ),
            (
                "a dual over a straddling bracket",
                Dual::new(wide(-0.25, 0.75), wide(1.0, 1.0)).structure_positive(),
                false,
            ),
            (
                "a sym over a wide positive",
                Sym::<Interval>::from_f64(0.5).structure_positive(),
                true,
            ),
            (
                "a negative dual point",
                Dual64::variable(-1.0).structure_positive(),
                false,
            ),
        ];
        for (what, got, want) in cases {
            assert_eq!(got, want, "{what}");
        }
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

    /// `plain_constant` at each scalar: a constant answers its value;
    /// poison, a wide enclosure, a derivative and a non-literal symbol
    /// each answer `None`, whatever the value channel says.
    #[test]
    fn plain_constant_reads_only_what_is_carried_as_a_constant() {
        use crate::interval::Interval;
        use crate::sym::Sym;
        assert_eq!(2.5f64.plain_constant(), Some(2.5));
        assert_eq!(f64::NAN.plain_constant(), None);
        assert_eq!(
            <Interval as Real>::from_f64(2.5).plain_constant(),
            Some(2.5)
        );
        assert_eq!(Interval::from_bounds(1.0, 2.0).plain_constant(), None);
        assert_eq!(Dual64::constant(2.5).plain_constant(), Some(2.5));
        assert_eq!(Dual::new(2.5, 1.0).point_value(), Some(2.5));
        assert_eq!(Dual::new(2.5, 1.0).plain_constant(), None);
        let lit = <Sym<f64> as Real>::from_f64;
        assert_eq!(lit(2.5).plain_constant(), Some(2.5));
        assert_eq!(<Sym<f64> as Real>::one().plain_constant(), Some(1.0));
        let computed = lit(1.25) + lit(1.25);
        assert_eq!(computed.point_value(), Some(2.5));
        assert_eq!(computed.plain_constant(), None);
    }
}
