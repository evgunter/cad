//! A carrier's knots held in its scalar: [`Knots`], and the span of
//! them an evaluator reads, [`KnotSpan`].
//!
//! The values are the definition. The `f64` [`KnotVector`] beside them
//! is their value channel, derived from them at construction and
//! never set apart from them: it carries the structure (degree,
//! multiplicities, which span a parameter lies in), and the values
//! carry the arithmetic. So a knot that is a function of a document's
//! variables stays that function wherever a basis reads it, and the
//! span choice stays an `f64` read of the value channel.

use super::knots::{KnotVector, Span, SplineError};
use super::locate::SpanLocate;
use crate::real::Real;

/// A clamped knot vector whose values are held in the scalar `T`
/// (module docs).
///
/// No constructor takes both a [`KnotVector`] and values, so the two
/// cannot disagree: [`Knots::new`] derives the vector from the values,
/// `From<KnotVector>` takes the vector's own values at `f64`, and
/// [`Knots::map`] carries the vector over a structural map of the
/// values.
#[derive(Clone, Debug)]
pub struct Knots<T> {
    structure: KnotVector,
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
    /// The knots `values` at `degree`, the `f64` vector read off them
    /// through [`SpanLocate::point_value`] and validated by
    /// [`KnotVector::clamped`].
    ///
    /// # Errors
    ///
    /// [`SplineError::KnotNotAPoint`] for the first value that is no
    /// single point (poison, or a wide enclosure), and otherwise
    /// [`KnotVector::clamped`]'s refusal of the vector read off them.
    pub fn new(values: Vec<T>, degree: usize) -> Result<Self, SplineError> {
        let read = values
            .iter()
            .enumerate()
            .map(|(index, v)| v.point_value().ok_or(SplineError::KnotNotAPoint { index }))
            .collect::<Result<Vec<f64>, SplineError>>()?;
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
}

impl<T: Real> Knots<T> {
    /// The value channel: the structure every `f64` reader of these
    /// knots reads.
    pub fn structure(&self) -> &KnotVector {
        &self.structure
    }

    /// The knot values, one per knot of [`Self::structure`].
    pub fn values(&self) -> &[T] {
        &self.values
    }

    /// The degree `p`.
    pub fn degree(&self) -> usize {
        self.structure.degree()
    }

    /// The same knots at another scalar: `f` applied to every value,
    /// the structure carried over verbatim. A structural map (a lift,
    /// a re-reading of the same number), so the structure still reads
    /// the mapped values' value channel.
    #[must_use]
    pub fn map<U: Real>(&self, f: impl Fn(T) -> U) -> Knots<U> {
        Knots {
            structure: self.structure.clone(),
            values: self.values.iter().map(|v| f(*v)).collect(),
        }
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
    use crate::interval::Interval;

    #[test]
    fn the_structure_is_read_off_the_values() {
        let k = Knots::new(vec![0.0, 0.0, 0.0, 0.5, 2.0, 2.0, 2.0], 2).unwrap();
        assert_eq!(k.structure().knots(), k.values(), "the value channel");
        let (first, last) = k.locate(1.0).expect("a numeric parameter");
        assert_eq!((first.span().index(), last.span().index()), (3, 3));
    }

    #[test]
    fn a_value_that_is_no_point_refuses_by_index() {
        let wide = Interval::from_bounds(0.25, 0.75);
        let pt = |x: f64| Interval::from_bounds(x, x);
        let vals = vec![pt(0.0), pt(0.0), wide, pt(1.0), pt(1.0)];
        match Knots::new(vals, 1) {
            Err(SplineError::KnotNotAPoint { index }) => assert_eq!(index, 2),
            other => panic!("a wide knot built a vector: {other:?}"),
        }
        let poisoned = vec![0.0, 0.0, f64::NAN, 1.0, 1.0];
        match Knots::new(poisoned, 1) {
            Err(SplineError::KnotNotAPoint { index }) => assert_eq!(index, 2),
            other => panic!("a NaN knot built a vector: {other:?}"),
        }
    }

    #[test]
    fn point_intervals_read_their_point() {
        let vals = [0.0, 0.0, 0.5, 1.0, 1.0]
            .map(|x| Interval::from_bounds(x, x))
            .to_vec();
        let k = Knots::new(vals, 1).unwrap();
        assert_eq!(k.structure().knots(), &[0.0, 0.0, 0.5, 1.0, 1.0]);
    }
}
