//! A carrier's knots held in its scalar: [`Knots`], and the span of
//! them an evaluator reads, [`KnotSpan`].
//!
//! The values are the definition, one per breakpoint, and the runs and
//! multiplicities are structure ([`KnotStructure`]), certified for
//! every member of the family the values stand for (C6). A knot that is
//! a function of a document's variables stays that function wherever a
//! basis reads it, and a repeated knot is one quantity read as often as
//! it repeats. At an enclosure scalar a breakpoint may be wider than a
//! point; where every breakpoint is a point, the `f64` [`KnotVector`]
//! read off them sits beside the structure as C6's interim
//! ([`Knots::structure_f64_interim`]), and the span a parameter lies in
//! is that vector's tie-break, bit for bit as before.

use core::num::NonZeroUsize;

use std::borrow::Cow;

use super::knots::{KnotStructure, KnotVector, Span, SplineError};
use super::locate::SpanLocate;
use crate::real::Real;

/// A clamped knot vector whose breakpoints are held in the scalar `T`
/// (module docs).
///
/// Its structure is certified at construction ([`Knots::new`]):
/// `From<KnotVector>` takes the vector's own values at `f64`, and
/// [`Knots::map`] carries the structure over where it keeps every point
/// and certifies the mapped runs anew where it does not.
#[derive(Clone, Debug)]
pub struct Knots<T> {
    structure: KnotStructure,
    /// The `f64` vector, where every breakpoint is a point: the values'
    /// value channel, C6's interim.
    point: Option<KnotVector>,
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
    /// **The vector a knot-algebra plan is decided against** (C6 S5b):
    /// each run's value where it is a point, so at point knots it is
    /// the knots' own `f64` vector, bit for bit, borrowed; and where a
    /// run is wider than a point, the midpoint of its bracket, a
    /// stand-in that orders as the run does for every member: runs
    /// certified to ascend ([`Self::new`]) have disjoint brackets, so
    /// their midpoints ascend too. A plan made against it has the right
    /// schedule (its steps name the runs by index, and the appliers at a
    /// scalar read the held values); its `f64` reference values are a
    /// stand-in's, which the plan's holder marks (`plan_inputs`). Not
    /// public: outside the knot algebra's own decisions a stand-in is no
    /// value of the knots.
    ///
    /// # Errors
    ///
    /// [`SplineError::KnotIsPoison`] at a poison run, which
    /// [`Self::new`] refuses.
    pub(super) fn plan_proxy(&self) -> Result<Cow<'_, KnotVector>, SplineError> {
        if let Some(point) = &self.point {
            return Ok(Cow::Borrowed(point));
        }
        let mut knots = Vec::with_capacity(self.values.len());
        for (index, (value, m)) in self.runs().enumerate() {
            let (lo, hi) = value
                .structure_bracket()
                .ok_or(SplineError::KnotIsPoison { index })?;
            knots.extend(core::iter::repeat_n(f64::midpoint(lo, hi), m));
        }
        Ok(Cow::Owned(KnotVector::from_algebra(knots, self.degree())))
    }

    /// These knots carried onto `structure`, the vector a knot-algebra
    /// plan made of this one (`Knots::plan_proxy`'s): a breakpoint
    /// `structure` shares with that vector (bit-equal) keeps its held
    /// value, so it stays one quantity through the operation, and a new
    /// one is `fresh(v)`, the caller's entry of the value it inserted.
    ///
    /// # Panics
    ///
    /// When `fresh` moves a new breakpoint's point: the structure would
    /// no longer be the values' value channel. And on knots with a
    /// poison run, which [`Self::new`] refuses.
    #[must_use]
    #[track_caller]
    #[allow(clippy::panic)]
    pub fn restructure(&self, structure: KnotVector, fresh: impl Fn(f64) -> T) -> Self {
        let Ok(proxy) = self.plan_proxy() else {
            panic!("Knots::restructure: a poison run, which Knots::new refuses")
        };
        let held: Vec<(f64, T)> = {
            let mut at = 0;
            proxy
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
        // The vector is the values' own only where every one is a point;
        // over a wide run it is a stand-in's.
        let all_point = values.iter().all(|v| v.point_value().is_some());
        Self {
            structure: structure.structure().clone(),
            point: all_point.then_some(structure),
            values,
        }
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
    /// at an enclosure scalar two whose mapped enclosures overlap
    /// refuse undecided ([`SplineError::KnotRunsUndecided`]).
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
        let last = self.structure.multiplicities().len() - 1;
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
    /// `runs`, each a value and its multiplicity, ascending (C6): a
    /// value may be an enclosure wider than a point, and the runs'
    /// order is certified for every value they stand for. Where every
    /// value is a point, the `f64` vector is read off them through
    /// [`SpanLocate::point_value`] and validated by
    /// [`KnotVector::clamped`], as before.
    ///
    /// # Errors
    ///
    /// - [`SplineError::KnotIsPoison`] for the first run whose value
    ///   is poison;
    /// - [`SplineError::KnotVectorInvalid`] (`NonFinite`, at the run's
    ///   first knot) for one that reaches past the finite numbers, as
    ///   [`KnotVector::clamped`] refuses a non-finite knot;
    /// - [`SplineError::KnotRunsNotIncreasing`] for the first run that
    ///   is not above its predecessor for any value they stand for: two
    ///   breakpoints that read as one number would merge into one run,
    ///   and the structure would no longer be the runs written;
    /// - [`SplineError::KnotRunsUndecided`] for the first run whose
    ///   order against its predecessor holds for some values and not
    ///   others, carrying the step's enclosure as its margin;
    /// - otherwise the clamped contract's refusal (a zero degree, a
    ///   multiplicity the clamped form does not admit), which indexes
    ///   the knots, not the runs.
    pub fn new(runs: Vec<(T, NonZeroUsize)>, degree: usize) -> Result<Self, SplineError> {
        let mut read: Option<Vec<f64>> = Some(Vec::new());
        let mut values: Vec<T> = Vec::new();
        let mut multiplicities = Vec::with_capacity(runs.len());
        let mut last: Option<T> = None;
        let mut at = 0;
        for (index, (value, multiplicity)) in runs.into_iter().enumerate() {
            let multiplicity = multiplicity.get();
            if value.structure_bracket().is_none() {
                return Err(SplineError::KnotIsPoison { index });
            }
            if !value.is_bounded() {
                return Err(SplineError::KnotVectorInvalid {
                    reason: super::knots::KnotVectorIssue::NonFinite { index: at },
                });
            }
            at += multiplicity;
            if let Some(prev) = last {
                runs_ascend(prev, value, index)?;
            }
            last = Some(value);
            read = read.and_then(|mut r| {
                let v = value.point_value()?;
                r.extend(core::iter::repeat_n(v, multiplicity));
                Some(r)
            });
            values.extend(core::iter::repeat_n(value, multiplicity));
            multiplicities.push(multiplicity);
        }
        let point = read.map(|r| KnotVector::clamped(r, degree)).transpose()?;
        let structure = match &point {
            Some(kv) => kv.structure().clone(),
            None => KnotStructure::clamped(multiplicities, degree)?,
        };
        Ok(Self {
            structure,
            point,
            values,
        })
    }

    /// The first run whose value is wider than a point, where there is
    /// one: knots without an `f64` vector have one ([`Self::new`]).
    pub fn first_wide_run(&self) -> Option<usize> {
        self.runs()
            .position(|(value, _)| value.point_value().is_none())
    }

    /// The held value of the breakpoint whose value channel is `v`
    /// (bit-equal), or `None` when no breakpoint reads as `v`.
    ///
    /// # Errors
    ///
    /// [`SplineError::CarrierKnotNotAPoint`] at the first wide run, where
    /// the knots are not all points: they have no value channel to
    /// match `v` against, and answering `None` there would enter `v` as
    /// a fresh value beside a breakpoint it may name.
    pub fn value_at(&self, v: f64) -> Result<Option<T>, SplineError> {
        let Some(point) = self.point.as_ref() else {
            let index = self.first_wide_run().unwrap_or(0);
            return Err(SplineError::CarrierKnotNotAPoint { index });
        };
        // The runs ascend, so the one whose first knot can be `v` is the
        // first knot at or above it; a run's value is its first knot's.
        let knots = point.knots();
        let at = knots.partition_point(|&k| k < v);
        Ok(knots
            .get(at)
            .filter(|k| k.to_bits() == v.to_bits())
            .map(|_| self.values[at]))
    }

    /// The spans the parameter `t` overlaps: the first and the last, or
    /// `None` for a poison `t`. Point knots locate on their `f64`
    /// vector ([`SpanLocate::locate_spans`]); knots wider than a point
    /// on the held values, every member's span hulled
    /// ([`SpanLocate::locate_held`]).
    pub fn locate(&self, t: T) -> Option<(KnotSpan<'_, T>, KnotSpan<'_, T>)> {
        let set = match &self.point {
            Some(kv) => t.locate_spans(kv)?,
            None => t.locate_held(&self.structure, &self.values)?,
        };
        Some((self.paired(set.first), self.paired(set.last)))
    }

    /// The same knots at another scalar: `f` applied once per
    /// breakpoint. Where `f` keeps every breakpoint's point (a lift,
    /// `Real::from_computed`), the structure and the `f64` vector carry
    /// over as they are. Otherwise the mapped runs are certified anew,
    /// as [`Self::new`] certifies them, so mapped knots are held knots
    /// like any other: their order holds for every member, a poison one
    /// refuses, and they carry an `f64` vector exactly when every mapped
    /// breakpoint is a point.
    ///
    /// # Errors
    ///
    /// [`Self::new`]'s, on the mapped runs.
    ///
    /// # Panics
    ///
    /// When `f` moves a point breakpoint to another point: the caller's
    /// lift contract, broken.
    pub fn map<U: SpanLocate>(&self, f: impl Fn(T) -> U) -> Result<Knots<U>, SplineError> {
        let mut runs = Vec::with_capacity(self.structure.multiplicities().len());
        let mut kept = self.point.is_some();
        for (before, m) in self.runs() {
            let mapped = f(before);
            match (before.point_value(), mapped.point_value()) {
                (Some(was), Some(now)) => assert!(
                    was.to_bits() == now.to_bits(),
                    "Knots::map moved breakpoint {was} to {now:?}"
                ),
                _ => kept = false,
            }
            runs.push((mapped, NonZeroUsize::new(m).unwrap_or(NonZeroUsize::MIN)));
        }
        if !kept {
            return Knots::new(runs, self.structure.degree());
        }
        let values = runs
            .iter()
            .flat_map(|&(v, m)| core::iter::repeat_n(v, m.get()))
            .collect();
        Ok(Knots {
            structure: self.structure.clone(),
            point: self.point.clone(),
            values,
        })
    }
}

/// Whether `b` lies above `a` for every value they stand for (C6),
/// refused typed otherwise: [`SplineError::KnotRunsNotIncreasing`] where
/// it holds for none, [`SplineError::KnotRunsUndecided`] with the step's
/// enclosure where it holds for some.
fn runs_ascend<T: SpanLocate>(a: T, b: T, index: usize) -> Result<(), SplineError> {
    if super::locate::structure_ascends(a, b) {
        return Ok(());
    }
    let step = b - a;
    match step.structure_bracket() {
        Some((lo, hi)) if hi > 0.0 && lo != hi => Err(SplineError::KnotRunsUndecided {
            index,
            margin: crate::MarginDiag::enclosure(lo, hi),
        }),
        _ => Err(SplineError::KnotRunsNotIncreasing { index }),
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
        for &m in self.structure.multiplicities() {
            let mapped = f(self.values[values.len()]);
            values.extend(core::iter::repeat_n(mapped, m));
        }
        Knots {
            structure: self.structure.clone(),
            point: self.point.clone(),
            values,
        }
    }

    /// These knots reflected through 0, `k'_i = 0 − k_{m−i}` in `T`
    /// over the held values: negation does not round, so the structure
    /// is the reflected runs, and an `f64` vector beside it is
    /// [`KnotVector::negated`]'s.
    #[must_use]
    pub fn negated(&self) -> Self {
        let multiplicities = self
            .structure
            .multiplicities()
            .iter()
            .rev()
            .copied()
            .collect();
        let point = self.point.as_ref().map(KnotVector::negated);
        let structure = match &point {
            Some(kv) => kv.structure().clone(),
            None => KnotStructure::clamped(multiplicities, self.structure.degree())
                .unwrap_or_else(|e| unreachable!("a clamped structure reflected: {e}")),
        };
        Self {
            structure,
            point,
            values: self.values.iter().rev().map(|k| T::zero() - *k).collect(),
        }
    }

    /// The structure (C6): degree, runs and multiplicities, and no
    /// position.
    pub fn structure(&self) -> &KnotStructure {
        // The vector's own, where there is one: the spans of `span` and
        // `locate` borrow it, and a span compares by its structure.
        self.point
            .as_ref()
            .map_or(&self.structure, KnotVector::structure)
    }

    /// **C6's interim**: the `f64` vector read off these knots, where
    /// every breakpoint is a point; the value channel every `f64`
    /// reader of them reads. Knots wider than a point have none, and a
    /// reader reaching for it there is a kernel bug, refused loud with
    /// its location. Counted by `scripts/gates/knots-f64-interim.sh`;
    /// it goes in C6's S5.
    ///
    /// # Panics
    ///
    /// On knots that are not all points.
    #[track_caller]
    #[allow(clippy::panic)]
    pub fn structure_f64_interim(&self) -> &KnotVector {
        // A `match`, not `unwrap_or_else`: `Location::caller()` inside a
        // closure names this file, not the reader.
        match &self.point {
            Some(kv) => kv,
            None => panic!(
                "a reader at {} took the f64 knot vector of a carrier whose knots are not \
                 all points (C6: its positions are its held values)",
                core::panic::Location::caller()
            ),
        }
    }

    /// Whether every breakpoint is a point, so that
    /// [`Self::structure_f64_interim`] answers.
    pub fn is_point(&self) -> bool {
        self.point.is_some()
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
        self.structure.multiplicities().iter().map(move |&m| {
            let v = self.values[at];
            at += m;
            (v, m)
        })
    }

    /// **The creases as held**: the value of each interior run of
    /// multiplicity at least the degree
    /// ([`KnotStructure::crease_runs`](super::KnotStructure::crease_runs)),
    /// ascending. The one statement of where a spline has no derivative
    /// inside its domain, read as values.
    pub fn crease_values(&self) -> Vec<T> {
        let values: Vec<T> = self.runs().map(|(v, _)| v).collect();
        self.structure.crease_runs().map(|r| values[r]).collect()
    }

    /// The parameter domain as held: the end runs' values at the
    /// indices [`KnotVector::domain`] reads (`p` and `n − 1 − p`), so
    /// at `f64` it is that door's pair bit for bit, a clamp run that
    /// mixes `-0.0` and `0.0` included.
    pub fn domain(&self) -> (T, T) {
        let (n, p) = (self.values.len(), self.structure.degree());
        (self.values[p], self.values[n - 1 - p])
    }

    /// The degree `p`.
    pub fn degree(&self) -> usize {
        self.structure.degree()
    }

    /// The span at `index`, or `None` where the structure refuses it
    /// (outside the spans, or empty).
    pub fn span(&self, index: usize) -> Option<KnotSpan<'_, T>> {
        let span = match &self.point {
            Some(kv) => kv.span(index)?,
            None => self.structure.span(index)?,
        };
        Some(self.paired(span))
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
    fn from(vector: KnotVector) -> Self {
        Self {
            structure: vector.structure().clone(),
            values: vector.knots().to_vec(),
            point: Some(vector),
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
        assert_eq!(
            k.structure_f64_interim().knots(),
            k.values(),
            "the value channel"
        );
        assert_eq!(
            k.structure_f64_interim().knots(),
            &[0.0, 0.0, 0.0, 0.5, 2.0, 2.0, 2.0]
        );
        let (first, last) = k.locate(1.0).expect("a numeric parameter");
        assert_eq!((first.span().index(), last.span().index()), (3, 3));
        let runs: Vec<(f64, usize)> = k.runs().collect();
        assert_eq!(runs, vec![(0.0, 3), (0.5, 1), (2.0, 3)]);
    }

    /// **Poison names no place (C6): it refuses by run.** A NaN
    /// breakpoint at `f64` and an empty enclosure at `Interval` both
    /// refuse `KnotIsPoison` at their run.
    #[test]
    fn a_poison_breakpoint_refuses_by_run() {
        match Knots::new(vec![(0.0, m(2)), (f64::NAN, m(1)), (1.0, m(2))], 1) {
            Err(SplineError::KnotIsPoison { index }) => assert_eq!(index, 1),
            other => panic!("a NaN knot built a vector: {other:?}"),
        }
        let pt = |x: f64| Interval::from_bounds(x, x);
        let nai = Interval::from_f64(f64::NAN);
        match Knots::new(vec![(pt(0.0), m(2)), (nai, m(1)), (pt(1.0), m(2))], 1) {
            Err(SplineError::KnotIsPoison { index }) => assert_eq!(index, 1),
            other => panic!("a NaI knot built a vector: {other:?}"),
        }
    }

    /// **A breakpoint wider than a point is held (C6)**, its structure
    /// certified for every member: `[0.25, 0.75]` between the clamps of
    /// a degree-1 vector gives runs `2, 1, 2` and no `f64` vector. Its
    /// span read hulls every span a member's parameter can lie in:
    /// `0.5` may lie on either side of the wide knot, so both spans;
    /// `0.1` is below every member's knot, so the first alone; `0.9`
    /// above every member's, so the last alone. Red where a wide knot
    /// refuses, or where the read drops a span a member lies in.
    #[test]
    fn a_wide_breakpoint_is_held_and_its_span_read_hulls_every_member() {
        let pt = |x: f64| Interval::from_bounds(x, x);
        let wide = Interval::from_bounds(0.25, 0.75);
        let k = Knots::new(vec![(pt(0.0), m(2)), (wide, m(1)), (pt(1.0), m(2))], 1)
            .expect("a wide breakpoint is held");
        assert!(!k.is_point(), "no f64 vector names a wide knot");
        assert_eq!(k.structure().multiplicities(), &[2, 1, 2]);
        let spans = |t: f64| {
            let (first, last) = k.locate(pt(t)).expect("a numeric parameter");
            (first.span().index(), last.span().index())
        };
        assert_eq!(spans(0.5), (1, 2), "either side of the wide knot");
        assert_eq!(spans(0.1), (1, 1), "below every member's knot");
        assert_eq!(spans(0.9), (2, 2), "above every member's knot");
        // The boundaries. At the knot's low end, the member whose knot is
        // there puts `t` in span 2 (`k ≤ t`); at its high end no member
        // puts `t` below its knot, so span 1 is impossible; at the
        // domain's end the last span holds `t` though nothing lies above.
        assert_eq!(spans(0.25), (1, 2), "at the knot's low end");
        assert_eq!(spans(0.75), (2, 2), "at the knot's high end");
        assert_eq!(spans(1.0), (2, 2), "at the domain's end");
        assert_eq!(spans(0.0), (1, 1), "at the domain's start");
    }

    /// **An order some members break refuses undecided, with its
    /// margin (C6)**: `[0.4, 0.6]` then `[0.5, 0.7]` ascend for some
    /// members and meet or cross for others, so run 2 refuses
    /// `KnotRunsUndecided`, carrying the step's enclosure, about `[−0.1, 0.3]`.
    /// Narrowing the first to `[0.4, 0.45]` lifts it; `[0.6, 0.7]` then
    /// `[0.3, 0.4]` crosses for every member and refuses as not
    /// increasing. Red where an overlap is taken as ascending, or
    /// refused without its margin.
    #[test]
    fn an_order_some_members_break_refuses_undecided_with_its_margin() {
        let pt = |x: f64| Interval::from_bounds(x, x);
        let iv = Interval::from_bounds;
        let build = |a: Interval, b: Interval| {
            Knots::new(
                vec![(pt(0.0), m(2)), (a, m(1)), (b, m(1)), (pt(1.0), m(2))],
                1,
            )
        };
        match build(iv(0.4, 0.6), iv(0.5, 0.7)) {
            Err(SplineError::KnotRunsUndecided { index, margin }) => {
                assert_eq!(index, 2);
                let crate::ErrorTextReading::Enclosure { lo, hi } =
                    margin.diagnostic_f64_for_error_text()
                else {
                    panic!("an enclosure margin, got {margin:?}");
                };
                // The step's exact value between the f64 inputs, enclosed outward.
                assert!(lo <= 0.5 - 0.6 && (0.5 - 0.6) - lo < 1e-12, "{lo}");
                assert!(hi >= 0.7 - 0.4 && hi - (0.7 - 0.4) < 1e-12, "{hi}");
            }
            other => panic!("an overlapping order built a vector: {other:?}"),
        }
        assert!(
            build(iv(0.4, 0.45), iv(0.5, 0.7)).is_ok(),
            "a narrower box lifts it"
        );
        match build(iv(0.6, 0.7), iv(0.3, 0.4)) {
            Err(SplineError::KnotRunsNotIncreasing { index }) => assert_eq!(index, 2),
            other => panic!("a crossing order built a vector: {other:?}"),
        }
        // A step whose top is exactly zero: the two meet at 0.5 and cross
        // everywhere else, so no member ascends, and no narrower box
        // lifts it. Not increasing, not undecided.
        match build(iv(0.5, 0.6), iv(0.4, 0.5)) {
            Err(SplineError::KnotRunsNotIncreasing { index }) => assert_eq!(index, 2),
            other => panic!("an order that ascends for no member: {other:?}"),
        }
    }

    /// **Mapped knots are held knots (C6)**: `map` certifies what it
    /// does not keep, as [`Knots::new`] does. Point knots `0, 0.4, 0.5,
    /// 1` widened by `±0.3` overlap at run 1, and refuse undecided; a
    /// breakpoint mapped to poison refuses at its run; wide knots mapped
    /// to their midpoints are points, with their `f64` vector. Red where
    /// `map` carries the structure over unchecked.
    #[test]
    fn a_map_certifies_what_it_does_not_keep() {
        let k = Knots::new(vec![(0.0, m(2)), (0.4, m(1)), (0.5, m(1)), (1.0, m(2))], 1).unwrap();
        match k.map(|x| Interval::from_bounds(x - 0.3, x + 0.3)) {
            Err(SplineError::KnotRunsUndecided { index, .. }) => assert_eq!(index, 1),
            other => panic!("overlapping mapped runs: {other:?}"),
        }
        match k.map(|x| {
            if x == 0.4 {
                Interval::from_bounds(f64::NAN, f64::NAN)
            } else {
                Interval::from_f64(x)
            }
        }) {
            Err(SplineError::KnotIsPoison { index }) => assert_eq!(index, 1),
            other => panic!("a poison mapped breakpoint: {other:?}"),
        }
        let pt = |x: f64| Interval::from_bounds(x, x);
        let wide = Knots::new(
            vec![
                (pt(0.0), m(2)),
                (Interval::from_bounds(0.25, 0.75), m(1)),
                (pt(1.0), m(2)),
            ],
            1,
        )
        .unwrap();
        let mid = wide
            .map(|x| 0.5 * (crate::real::Bounds::lo(x) + crate::real::Bounds::hi(x)))
            .expect("midpoints ascend");
        assert!(mid.is_point(), "point knots hold their vector");
        assert_eq!(mid.values(), &[0.0, 0.0, 0.5, 1.0, 1.0]);
    }

    /// **`value_at` tells "no breakpoint reads as `v`" from "no value
    /// channel"**: on point knots a breakpoint's value answers and any
    /// other `v` is `None`, the caller's cue to enter it fresh; on knots
    /// with a wide breakpoint it refuses at that run, rather than a
    /// `None` that would enter `v` beside a breakpoint it may name. Red
    /// where wide knots answer `None`.
    #[test]
    fn value_at_refuses_wide_knots_rather_than_answering_none() {
        let pt = |x: f64| Interval::from_bounds(x, x);
        let point = Knots::new(vec![(pt(0.0), m(2)), (pt(0.5), m(1)), (pt(1.0), m(2))], 1).unwrap();
        assert_eq!(
            point
                .value_at(0.5)
                .map(|k| k.map(|v| (crate::real::Bounds::lo(v), crate::real::Bounds::hi(v)))),
            Ok(Some((0.5, 0.5)))
        );
        assert!(matches!(point.value_at(0.25), Ok(None)));
        let wide = Knots::new(
            vec![
                (pt(0.0), m(2)),
                (Interval::from_bounds(0.25, 0.75), m(1)),
                (pt(1.0), m(2)),
            ],
            1,
        )
        .unwrap();
        for v in [0.5, 0.0] {
            match wide.value_at(v) {
                Err(SplineError::CarrierKnotNotAPoint { index }) => assert_eq!(index, 1),
                other => panic!("{v}: wide knots answered {other:?}"),
            }
        }
    }

    /// **A breakpoint past the finite numbers refuses on every path**:
    /// `[1, +∞]` as the end run refuses `NonFinite` at its first knot,
    /// as the `f64` point `+∞` does. Red where the held path admits it.
    #[test]
    fn an_unbounded_breakpoint_refuses_as_a_point_one_does() {
        use super::super::knots::KnotVectorIssue;
        let pt = |x: f64| Interval::from_bounds(x, x);
        let wide = Knots::new(
            vec![
                (pt(0.0), m(2)),
                (pt(0.5), m(1)),
                (Interval::from_bounds(1.0, f64::INFINITY), m(2)),
            ],
            1,
        );
        let point = Knots::new(vec![(0.0, m(2)), (0.5, m(1)), (f64::INFINITY, m(2))], 1);
        for (what, got) in [("wide", wide.map(|_| ())), ("point", point.map(|_| ()))] {
            match got {
                Err(SplineError::KnotVectorInvalid {
                    reason: KnotVectorIssue::NonFinite { index },
                }) => assert_eq!(index, 3, "{what}"),
                other => panic!("{what}: an unbounded breakpoint: {other:?}"),
            }
        }
    }

    /// **C6's interim, both halves**: point knots read through their
    /// `f64` vector, bit for bit; a carrier with a wide knot refuses
    /// the read loud. Red where the interim answers for a wide carrier.
    #[test]
    fn the_interim_f64_vector_reads_points_and_refuses_a_wide_carrier() {
        use test_utils::panic_capture::caught;
        let k = Knots::new(vec![(0.0, m(2)), (0.5, m(1)), (1.0, m(2))], 1).unwrap();
        assert_eq!(
            k.structure_f64_interim().knots(),
            &[0.0, 0.0, 0.5, 1.0, 1.0]
        );
        let pt = |x: f64| Interval::from_bounds(x, x);
        let wide = Knots::new(
            vec![
                (pt(0.0), m(2)),
                (Interval::from_bounds(0.25, 0.75), m(1)),
                (pt(1.0), m(2)),
            ],
            1,
        )
        .unwrap();
        // The refusal names the reader: this file at the reading line,
        // not the accessor's body.
        let names_the_reader = |message: Option<String>, line: u32| {
            let message = message.expect("a wide carrier's f64 vector was read");
            let at = format!("a reader at {}:{line}:", file!());
            assert!(message.contains(&at), "{message:?} does not name {at}");
        };
        let line = line!() + 2;
        let read = caught(|| {
            wide.structure_f64_interim();
        });
        names_the_reader(read, line);
        let span = wide.span(1).expect("a nonempty span").span();
        let line = line!() + 2;
        let through_span = caught(|| {
            span.knots_f64_interim();
        });
        names_the_reader(through_span, line);
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
        let lifted = k
            .map(|x| {
                calls.set(calls.get() + 1);
                Interval::from_bounds(x, x)
            })
            .unwrap();
        assert_eq!(calls.get(), 3, "one call per breakpoint");
        assert_eq!(
            lifted.structure_f64_interim().knots(),
            k.structure_f64_interim().knots()
        );
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
            r.structure_f64_interim().knots(),
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
        assert_eq!(
            bits(got.structure_f64_interim().knots()),
            bits(want.knots())
        );
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
