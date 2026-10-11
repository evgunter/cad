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
    /// across its run, so the run is one quantity.
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
    ///   multiplicity the clamped form does not admit).
    pub fn new(runs: Vec<(T, usize)>, degree: usize) -> Result<Self, SplineError> {
        let mut read: Vec<f64> = Vec::new();
        let mut values: Vec<T> = Vec::new();
        let mut last: Option<f64> = None;
        for (index, (value, multiplicity)) in runs.into_iter().enumerate() {
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
    /// mapped value's own point is the breakpoint's, and a map that
    /// moves one is a caller's error the debug build refuses.
    #[must_use]
    pub fn map<U: SpanLocate>(&self, f: impl Fn(T) -> U) -> Knots<U> {
        let mut values = Vec::with_capacity(self.values.len());
        for (v, m) in self.structure.knot_runs() {
            let mapped = f(self.values[values.len()]);
            debug_assert!(
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

    #[test]
    fn the_structure_is_read_off_the_runs() {
        let k = Knots::new(vec![(0.0, 3), (0.5, 1), (2.0, 3)], 2).unwrap();
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
        match Knots::new(vec![(pt(0.0), 2), (wide, 1), (pt(1.0), 2)], 1) {
            Err(SplineError::KnotNotAPoint { index }) => assert_eq!(index, 1),
            other => panic!("a wide knot built a vector: {other:?}"),
        }
        match Knots::new(vec![(0.0, 2), (f64::NAN, 1), (1.0, 2)], 1) {
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
        match Knots::new(vec![(c(0.0), 2), (a, 1), (b, 1), (c(1.0), 2)], 1) {
            Err(SplineError::KnotRunsNotIncreasing { index }) => assert_eq!(index, 2),
            other => panic!("two breakpoints at one value merged: {other:?}"),
        }
        match Knots::new(vec![(0.0, 2), (0.6, 1), (0.4, 1), (1.0, 2)], 1) {
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
            vec![(c(0.0), 3), (Dual64::variable(0.4), 2), (c(1.0), 3)],
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

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "Knots::map moved breakpoint")]
    fn a_map_that_moves_a_value_refuses_in_debug() {
        let k = Knots::new(vec![(0.0, 2), (1.0, 2)], 1).unwrap();
        let _ = k.map(|x| 3.0 * x);
    }
}
