//! A carrier's knots held in its scalar: [`Knots`], and the span of
//! them an evaluator reads, [`KnotSpan`].
//!
//! The values are the definition, one per breakpoint, and the
//! multiplicities are structure. The `f64` [`KnotVector`] beside them
//! is their value channel, read off them: it carries the structure
//! (degree, multiplicities, which span a parameter lies in), and the
//! values carry the arithmetic. So a knot that is a function of a
//! document's variables stays that function wherever a basis reads it,
//! a repeated knot is one quantity read as often as it repeats, and the
//! span choice stays an `f64` read of the value channel.

use core::num::NonZeroUsize;

use super::knots::{KnotVector, Span, SplineError};
use super::locate::SpanLocate;
use crate::real::Real;

/// A clamped knot vector whose breakpoints are held in the scalar `T`
/// (module docs).
///
/// Its structure is always its values' value channel
/// ([`SpanLocate::point_value`]): [`Knots::new`] reads it off the
/// values, `From<KnotVector>` takes the vector's own values at `f64`,
/// and [`Knots::map`] re-reads the mapped values against it. No
/// constructor takes a vector and values to pair.
#[derive(Clone, Debug)]
pub struct Knots<T> {
    structure: KnotVector,
    /// One per knot of `structure`: each breakpoint's value copied
    /// across its run, so the run is one quantity. A vector converted at
    /// `f64` ([`From<KnotVector>`]) keeps its own knots bit for bit, so
    /// its run copies agree on `==` and may differ in a zero's sign.
    values: Vec<T>,
}

/// One span of a [`Knots`]: the [`Span`] its structure proves, and the
/// values that span's knot indices read. Both borrow one `Knots`, so
/// the pairing is the borrow's.
#[derive(Clone, Copy, Debug)]
pub struct KnotSpan<'a, T> {
    span: Span<'a>,
    values: &'a [T],
}

impl<T: SpanLocate> Knots<T> {
    /// These knots carried onto `structure`, the vector a knot-algebra
    /// plan made of this one: a breakpoint `structure` shares with this
    /// vector (bit-equal value channel) keeps its held value, so it
    /// stays one quantity through the operation, and a new one is
    /// `fresh(v)`, the caller's entry of the value it inserted.
    ///
    /// # Panics
    ///
    /// When `fresh` moves a new breakpoint's point: the structure would
    /// no longer be the values' value channel.
    #[must_use]
    pub fn restructure(&self, structure: KnotVector, fresh: impl Fn(f64) -> T) -> Self {
        let held: Vec<(f64, T)> = {
            let mut at = 0;
            self.structure
                .knot_runs()
                .map(|(v, m)| {
                    let value = self.values[at];
                    at += m;
                    (v, value)
                })
                .collect()
        };
        let mut values = Vec::with_capacity(structure.knots().len());
        let mut next = 0;
        for (v, m) in structure.knot_runs() {
            while next < held.len() && held[next].0 < v {
                next += 1;
            }
            let value = match held.get(next) {
                Some((h, value)) if h.to_bits() == v.to_bits() => *value,
                _ => {
                    let value = fresh(v);
                    assert!(
                        value.point_value().map(f64::to_bits) == Some(v.to_bits()),
                        "Knots::restructure entered breakpoint {v} as {:?}",
                        value.point_value()
                    );
                    value
                }
            };
            values.extend(core::iter::repeat_n(value, m));
        }
        Self { structure, values }
    }

    /// The same knots re-expressed on `[lo, hi]`, in `T`: the ends are
    /// `lo` and `hi` entered as constants, and each interior breakpoint
    /// `k` maps to `lo + (hi − lo)·((k − a)/(b − a))` over the held
    /// values, `[a, b]` the held domain — [`KnotVector::on_domain`]'s
    /// formula and association, so at `f64` the structure is that
    /// door's, bit for bit.
    ///
    /// # Errors
    ///
    /// [`SplineError::DomainInvalid`] as [`KnotVector::on_domain`];
    /// otherwise [`Self::new`]'s refusals. Two breakpoints that
    /// rounding collapses onto one value refuse
    /// ([`SplineError::KnotRunsNotIncreasing`]) rather than merge, and
    /// at an enclosure scalar an interior breakpoint whose map is
    /// inexact comes out wider than a point and refuses
    /// ([`SplineError::KnotNotAPoint`]).
    pub fn on_domain(&self, lo: f64, hi: f64) -> Result<Self, SplineError> {
        let span = hi - lo;
        if !(lo.is_finite() && hi.is_finite() && lo < hi && span.is_finite()) {
            return Err(SplineError::DomainInvalid { lo, hi });
        }
        let p = self.structure.degree();
        let n = self.values.len();
        let (a, b) = (self.values[p], self.values[n - 1 - p]);
        let width = super::knots::knot_difference::<T>(lo, hi);
        let (lo_t, hi_t) = (T::from_f64(lo), T::from_f64(hi));
        let mut runs: Vec<(T, NonZeroUsize)> = Vec::new();
        let last = self.structure.knot_runs().count() - 1;
        for (i, (value, m)) in self.runs().enumerate() {
            let mapped = if i == 0 {
                lo_t
            } else if i == last {
                hi_t
            } else {
                lo_t + width * ((value - a) / (b - a))
            };
            let m = NonZeroUsize::new(m).unwrap_or(NonZeroUsize::MIN);
            runs.push((mapped, m));
        }
        Self::new(runs, p)
    }

    /// The clamped vector of degree `degree` whose breakpoints are
    /// `runs`, each a value and its multiplicity, ascending. The `f64`
    /// vector is read off the values through
    /// [`SpanLocate::point_value`] and validated by
    /// [`KnotVector::clamped`].
    ///
    /// # Errors
    ///
    /// - [`SplineError::KnotNotAPoint`] for the first run whose value
    ///   is no single point (poison, or a wide enclosure);
    /// - [`SplineError::KnotRunsNotIncreasing`] for the first run whose
    ///   value channel is not strictly above its predecessor's: two
    ///   breakpoints that read as one number would merge into one run,
    ///   and the structure would no longer be the runs written;
    /// - otherwise [`KnotVector::clamped`]'s refusal (a zero degree, a
    ///   multiplicity the clamped form does not admit), which indexes
    ///   the knots, not the runs.
    pub fn new(runs: Vec<(T, NonZeroUsize)>, degree: usize) -> Result<Self, SplineError> {
        let mut read: Vec<f64> = Vec::new();
        let mut values: Vec<T> = Vec::new();
        let mut last: Option<f64> = None;
        for (index, (value, multiplicity)) in runs.into_iter().enumerate() {
            let multiplicity = multiplicity.get();
            let v = value
                .point_value()
                .ok_or(SplineError::KnotNotAPoint { index })?;
            if last.is_some_and(|l| v <= l) {
                return Err(SplineError::KnotRunsNotIncreasing { index });
            }
            last = Some(v);
            read.extend(core::iter::repeat_n(v, multiplicity));
            values.extend(core::iter::repeat_n(value, multiplicity));
        }
        Ok(Self {
            structure: KnotVector::clamped(read, degree)?,
            values,
        })
    }

    /// The spans the parameter `t` overlaps, located on the value
    /// channel ([`SpanLocate::locate_spans`]): the first and the last,
    /// or `None` for a poison `t`.
    pub fn locate(&self, t: T) -> Option<(KnotSpan<'_, T>, KnotSpan<'_, T>)> {
        let set = t.locate_spans(&self.structure)?;
        Some((self.paired(set.first), self.paired(set.last)))
    }

    /// The same knots at another scalar: `f` applied once per
    /// breakpoint, the structure carried over.
    ///
    /// `f` re-reads each value (a lift, `Real::from_computed`): the
    /// mapped value's own point is the breakpoint's.
    ///
    /// # Panics
    ///
    /// When `f` moves a breakpoint's point: the structure would no
    /// longer be the values' value channel.
    #[must_use]
    pub fn map<U: SpanLocate>(&self, f: impl Fn(T) -> U) -> Knots<U> {
        let mut values = Vec::with_capacity(self.values.len());
        for (v, m) in self.structure.knot_runs() {
            let mapped = f(self.values[values.len()]);
            assert!(
                mapped.point_value().map(f64::to_bits) == Some(v.to_bits()),
                "Knots::map moved breakpoint {v} to {:?}",
                mapped.point_value()
            );
            values.extend(core::iter::repeat_n(mapped, m));
        }
        Knots {
            structure: self.structure.clone(),
            values,
        }
    }
}

impl<T: Real> Knots<T> {
    /// The same knots at another scalar under a LIFT `f`, one that
    /// keeps each breakpoint's point: `f` applied once per breakpoint,
    /// the structure carried over. That `f` keeps the point is the
    /// caller's contract, the one every `map_scalar` door states; a
    /// caller at a scalar that reads its point takes [`Self::map`],
    /// which checks it.
    #[must_use]
    pub fn lift<U: Real>(&self, f: impl Fn(T) -> U) -> Knots<U> {
        let mut values = Vec::with_capacity(self.values.len());
        for (_, m) in self.structure.knot_runs() {
            let mapped = f(self.values[values.len()]);
            values.extend(core::iter::repeat_n(mapped, m));
        }
        Knots {
            structure: self.structure.clone(),
            values,
        }
    }

    /// These knots reflected through 0, `k'_i = 0 − k_{m−i}` in `T`
    /// over the held values, on [`KnotVector::negated`]'s structure:
    /// negation does not round, so the structure is still the values'
    /// value channel.
    #[must_use]
    pub fn negated(&self) -> Self {
        Self {
            structure: self.structure.negated(),
            values: self.values.iter().rev().map(|k| T::zero() - *k).collect(),
        }
    }

    /// The value channel: the structure every `f64` reader of these
    /// knots reads.
    pub fn structure(&self) -> &KnotVector {
        &self.structure
    }

    /// The knot values, one per knot of [`Self::structure`]; a run's
    /// knots are copies of its one breakpoint.
    pub fn values(&self) -> &[T] {
        &self.values
    }

    /// The breakpoints and their multiplicities, ascending, clamps
    /// included.
    pub fn runs(&self) -> impl Iterator<Item = (T, usize)> + '_ {
        let mut at = 0;
        self.structure.knot_runs().map(move |(_, m)| {
            let v = self.values[at];
            at += m;
            (v, m)
        })
    }

    /// The parameter domain as held: the end runs' values at the
    /// indices [`KnotVector::domain`] reads (`p` and `n − 1 − p`), so
    /// at `f64` it is that door's pair bit for bit, a clamp run that
    /// mixes `-0.0` and `0.0` included.
    pub fn domain(&self) -> (T, T) {
        let (n, p) = (self.values.len(), self.structure.degree());
        (self.values[p], self.values[n - 1 - p])
    }

    /// The held value of the breakpoint whose value channel is `v`
    /// (bit-equal), or `None` when no breakpoint reads as `v`.
    pub fn value_at(&self, v: f64) -> Option<T> {
        self.runs()
            .zip(self.structure.knot_runs())
            .find(|(_, (k, _))| k.to_bits() == v.to_bits())
            .map(|((value, _), _)| value)
    }

    /// The degree `p`.
    pub fn degree(&self) -> usize {
        self.structure.degree()
    }

    /// The span at `index`, or `None` where [`KnotVector::span`]
    /// refuses it (outside the spans, or empty).
    pub fn span(&self, index: usize) -> Option<KnotSpan<'_, T>> {
        Some(self.paired(self.structure.span(index)?))
    }

    /// A span of this structure paired with these values.
    fn paired<'a>(&'a self, span: Span<'a>) -> KnotSpan<'a, T> {
        KnotSpan {
            span,
            values: &self.values,
        }
    }
}

impl From<KnotVector> for Knots<f64> {
    fn from(structure: KnotVector) -> Self {
        Self {
            values: structure.knots().to_vec(),
            structure,
        }
    }
}

impl<'a, T> KnotSpan<'a, T> {
    /// The structural span.
    pub fn span(self) -> Span<'a> {
        self.span
    }

    /// Every knot value of the vector, indexed as
    /// [`KnotVector::knots`] is.
    pub fn values(self) -> &'a [T] {
        self.values
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::dual::Dual64;
    use crate::interval::Interval;

    fn m(n: usize) -> NonZeroUsize {
        NonZeroUsize::new(n).unwrap()
    }

    #[test]
    fn the_structure_is_read_off_the_runs() {
        let k = Knots::new(vec![(0.0, m(3)), (0.5, m(1)), (2.0, m(3))], 2).unwrap();
        assert_eq!(k.structure().knots(), k.values(), "the value channel");
        assert_eq!(k.structure().knots(), &[0.0, 0.0, 0.0, 0.5, 2.0, 2.0, 2.0]);
        let (first, last) = k.locate(1.0).expect("a numeric parameter");
        assert_eq!((first.span().index(), last.span().index()), (3, 3));
        let runs: Vec<(f64, usize)> = k.runs().collect();
        assert_eq!(runs, vec![(0.0, 3), (0.5, 1), (2.0, 3)]);
    }

    #[test]
    fn a_value_that_is_no_point_refuses_by_run() {
        let pt = |x: f64| Interval::from_bounds(x, x);
        let wide = Interval::from_bounds(0.25, 0.75);
        match Knots::new(vec![(pt(0.0), m(2)), (wide, m(1)), (pt(1.0), m(2))], 1) {
            Err(SplineError::KnotNotAPoint { index }) => assert_eq!(index, 1),
            other => panic!("a wide knot built a vector: {other:?}"),
        }
        match Knots::new(vec![(0.0, m(2)), (f64::NAN, m(1)), (1.0, m(2))], 1) {
            Err(SplineError::KnotNotAPoint { index }) => assert_eq!(index, 1),
            other => panic!("a NaN knot built a vector: {other:?}"),
        }
    }

    /// Two runs that read as one number refuse rather than merge: the
    /// vector they would make has a double knot where two breakpoints
    /// were written, a structure nobody chose. Two distinct variables
    /// at one value are the case that matters, so the row uses those.
    #[test]
    fn equal_adjacent_runs_refuse_rather_than_merge() {
        let c = Dual64::constant;
        let (a, b) = (Dual64::variable(0.5), Dual64::new(0.5, -1.0));
        match Knots::new(
            vec![(c(0.0), m(2)), (a, m(1)), (b, m(1)), (c(1.0), m(2))],
            1,
        ) {
            Err(SplineError::KnotRunsNotIncreasing { index }) => assert_eq!(index, 2),
            other => panic!("two breakpoints at one value merged: {other:?}"),
        }
        match Knots::new(vec![(0.0, m(2)), (0.6, m(1)), (0.4, m(1)), (1.0, m(2))], 1) {
            Err(SplineError::KnotRunsNotIncreasing { index }) => assert_eq!(index, 2),
            other => panic!("a decreasing run built a vector: {other:?}"),
        }
    }

    /// A repeated knot is one quantity: every knot of its run is the
    /// breakpoint's value, carrying the breakpoint's channel.
    #[test]
    fn a_run_is_one_quantity() {
        let c = Dual64::constant;
        let k = Knots::new(
            vec![
                (c(0.0), m(3)),
                (Dual64::variable(0.4), m(2)),
                (c(1.0), m(3)),
            ],
            2,
        )
        .unwrap();
        assert_eq!(k.values()[3].deriv, 1.0);
        assert_eq!(k.values()[4].deriv, 1.0);
        assert_eq!(k.values()[3].value.to_bits(), k.values()[4].value.to_bits());
    }

    #[test]
    fn a_lift_maps_each_breakpoint_once_onto_the_same_structure() {
        let k = Knots::from(
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.3, 0.3, 1.0, 1.0, 1.0], 2).unwrap(),
        );
        let calls = core::cell::Cell::new(0);
        let lifted = k.map(|x| {
            calls.set(calls.get() + 1);
            Interval::from_bounds(x, x)
        });
        assert_eq!(calls.get(), 3, "one call per breakpoint");
        assert_eq!(lifted.structure().knots(), k.structure().knots());
    }

    /// A breakpoint the new structure shares keeps its held value, so a
    /// variable knot stays one quantity through an insertion; only the
    /// value brought in is entered fresh.
    #[test]
    fn a_restructure_keeps_shared_breakpoints_and_enters_new_ones() {
        let c = Dual64::constant;
        let k = Knots::new(
            vec![
                (c(0.0), m(3)),
                (Dual64::variable(0.4), m(1)),
                (c(1.0), m(3)),
            ],
            2,
        )
        .unwrap();
        let inserted =
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.25, 0.4, 0.4, 1.0, 1.0, 1.0], 2).unwrap();
        let r = k.restructure(inserted, Dual64::constant);
        assert_eq!(
            r.structure().knots(),
            &[0.0, 0.0, 0.0, 0.25, 0.4, 0.4, 1.0, 1.0, 1.0]
        );
        let derivs: Vec<f64> = r.values().iter().map(|v| v.deriv).collect();
        assert_eq!(derivs, vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0]);
    }

    /// `on_domain` is `KnotVector::on_domain`'s formula at `f64`, bit
    /// for bit, and carries a variable breakpoint's tangent through the
    /// affine map in `T`; constants enter at `from_f64`.
    #[test]
    fn on_domain_and_constant_agree_with_the_vector_doors() {
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.3, 0.3, 0.7, 2.0, 2.0, 2.0], 2).unwrap();
        let want = kv.on_domain(-1.5, 4.25).unwrap();
        let got = Knots::from(kv.clone()).on_domain(-1.5, 4.25).unwrap();
        let bits = |v: &[f64]| v.iter().map(|x| x.to_bits()).collect::<Vec<_>>();
        assert_eq!(bits(got.structure().knots()), bits(want.knots()));
        assert_eq!(bits(got.values()), bits(want.knots()));
        let c = Dual64::constant;
        let k = Knots::new(
            vec![
                (c(0.0), m(3)),
                (Dual64::variable(0.5), m(1)),
                (c(2.0), m(3)),
            ],
            2,
        )
        .unwrap();
        let moved = k.on_domain(0.0, 4.0).unwrap();
        assert_eq!(moved.values()[3].value, 1.0);
        assert_eq!(moved.values()[3].deriv, 2.0, "d/dk of 4·k/2");
        assert!(matches!(
            Knots::from(kv).on_domain(1.0, 1.0),
            Err(SplineError::DomainInvalid { .. })
        ));
    }

    /// `domain` is `KnotVector::domain` bit for bit at `f64`, a clamp
    /// run that mixes `-0.0` and `0.0` included (the vector reads its
    /// ends at `p` and `n − 1 − p`, and a converted vector keeps its own
    /// bits).
    #[test]
    fn the_held_domain_is_the_vectors_at_f64_signed_zeros_included() {
        for knots in [
            vec![-0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
            vec![0.0, -0.0, -0.0, 0.5, 2.0, 2.0, 2.0],
            vec![-1.0, -1.0, -1.0, -0.0, -0.0, -0.0],
        ] {
            let kv = KnotVector::clamped(knots.clone(), 2).unwrap();
            let (a, b) = kv.domain();
            let (ha, hb) = Knots::from(kv).domain();
            assert_eq!(
                (ha.to_bits(), hb.to_bits()),
                (a.to_bits(), b.to_bits()),
                "{knots:?}"
            );
        }
    }

    #[test]
    #[should_panic(expected = "Knots::map moved breakpoint")]
    fn a_map_that_moves_a_value_refuses() {
        let k = Knots::new(vec![(0.0, m(2)), (1.0, m(2))], 1).unwrap();
        let _ = k.map(|x| 3.0 * x);
    }
}
