//! Parameters as types: [`Param`], a parameter value that is a number,
//! and [`ParamRange`], a parameter region. The one knot search,
//! [`last_at_or_below`], takes a [`Param`], so "the value is not NaN"
//! is its parameter's type rather than a caller's promise.

use crate::interval::Interval;
use crate::real::CertifiedEnclosure;

/// A parameter value that is not NaN. It may be infinite: an
/// out-of-domain value clamps to an end span exactly as a finite one
/// does. The field is private and [`Param::new`] is the only mint, so a
/// search that takes a `Param` has no NaN to place.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Param(f64);

impl Param {
    /// `t`, or `None` when it is NaN: a NaN names no place in a domain.
    pub fn new(t: f64) -> Option<Self> {
        (!t.is_nan()).then_some(Self(t))
    }

    /// The value.
    pub fn get(self) -> f64 {
        self.0
    }
}

/// The last index `i` with `sorted[i] ≤ t`, or `None` when every
/// element exceeds `t` — **the** knot search: every span and cell
/// locator over a sorted knot slice is this plus its own clamping.
///
/// `sorted` must be non-decreasing; a violation is not unsound (the
/// answer is in range) but it is arbitrary. At a run of equal values
/// the answer is the run's LAST index, which is what makes a located
/// span the nonempty one starting at a repeated knot.
pub fn last_at_or_below(sorted: &[f64], t: Param) -> Option<usize> {
    sorted.partition_point(|k| *k <= t.0).checked_sub(1)
}

/// A closed parameter range `[start, end]` with `start ≤ end` and
/// neither end NaN — the window every span reader takes.
///
/// The fields are private and the constructors refuse a NaN or inverted
/// pair, so a window that names no region has no spelling, and a reader
/// cannot forget to ask whether its ends are numbers and ordered. The
/// ends may be infinite: an out-of-domain end clamps to an end span
/// exactly as a finite one does.
///
/// **The invariant is a bracket, not a certificate.** How a range was
/// minted says whether it is certified: [`ParamRange::certified`] mints
/// only from an enclosure that certifies, [`ParamRange::new`] and
/// [`ParamRange::spanning`] from numbers.
///
/// Its ends are `start` and `end`, not `lo` and `hi`: a parameter range
/// is a region of a domain, not an enclosure, and `lo`/`hi` are the
/// bracket accessors ([`crate::Bounds`]) whose reads the certified
/// endpoint census counts.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ParamRange {
    start: Param,
    end: Param,
}

impl ParamRange {
    /// `[start, end]`, or `None` when either end is NaN or
    /// `end < start`.
    pub fn new(start: f64, end: f64) -> Option<Self> {
        // `<=` is false for a NaN on either side.
        (start <= end).then_some(Self {
            start: Param(start),
            end: Param(end),
        })
    }

    /// The range between `a` and `b` in either order, or `None` when
    /// either is NaN. The spelling for an unordered pair: a
    /// `new(a.min(b), a.max(b))` would mint a window from the other end
    /// alone, because `f64::min`/`max` drop a NaN operand.
    pub fn spanning(a: f64, b: f64) -> Option<Self> {
        let (a, b) = (Param::new(a)?, Param::new(b)?);
        Some(if a.0 <= b.0 {
            Self { start: a, end: b }
        } else {
            Self { start: b, end: a }
        })
    }

    /// The bracket of `x`, or `None` when `x` does not certify
    /// ([`CertifiedEnclosure::certified_bracket`]): a NaI, an empty
    /// enclosure and a `Trv` one all refuse.
    pub fn certified(x: Interval) -> Option<Self> {
        let (lo, hi) = x.certified_bracket()?;
        Self::new(lo, hi)
    }

    /// The lower end.
    pub fn start(self) -> f64 {
        self.start.0
    }

    /// The upper end.
    pub fn end(self) -> f64 {
        self.end.0
    }

    /// Both ends, as the values a search takes.
    pub fn ends(self) -> (Param, Param) {
        (self.start, self.end)
    }

    /// `start.midpoint(end)`. NaN only for the whole line `(−∞, +∞)`,
    /// which has no midpoint; [`ParamRange::clamp_to`] a finite domain
    /// first.
    pub fn mid(self) -> f64 {
        self.start.0.midpoint(self.end.0)
    }

    /// Each end clamped into `domain`. Clamping is monotone, so an
    /// ordered range stays ordered: a range wholly past one end of the
    /// domain becomes that end's point.
    #[must_use]
    pub fn clamp_to(self, domain: Self) -> Self {
        let clamp = |x: Param| Param(x.0.max(domain.start.0).min(domain.end.0));
        Self {
            start: clamp(self.start),
            end: clamp(self.end),
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::real::Real;

    fn ends(r: ParamRange) -> (f64, f64) {
        (r.start(), r.end())
    }

    #[test]
    fn a_nan_or_inverted_pair_is_not_a_range() {
        assert!(ParamRange::new(f64::NAN, 1.0).is_none(), "NaN start");
        assert!(ParamRange::new(0.0, f64::NAN).is_none(), "NaN end");
        assert!(ParamRange::new(1.0, 0.0).is_none(), "inverted");
        let point = ParamRange::new(0.5, 0.5).expect("a point is a range");
        assert_eq!(ends(point), (0.5, 0.5));
        let wide = ParamRange::new(f64::NEG_INFINITY, f64::INFINITY).expect("the whole line");
        assert!(wide.mid().is_nan(), "the whole line has no midpoint");
        assert!(Param::new(f64::NAN).is_none(), "a NaN is no parameter");
    }

    /// An unordered pair orders itself, and a NaN on either side refuses
    /// where `f64::min`/`max` would have kept the other end.
    #[test]
    fn spanning_orders_a_pair_and_refuses_a_nan_on_either_side() {
        assert_eq!(
            ends(ParamRange::spanning(0.75, 0.25).unwrap()),
            (0.25, 0.75)
        );
        assert_eq!(
            ends(ParamRange::spanning(0.25, 0.75).unwrap()),
            (0.25, 0.75)
        );
        assert!(ParamRange::spanning(f64::NAN, 0.5).is_none(), "NaN first");
        assert!(ParamRange::spanning(0.5, f64::NAN).is_none(), "NaN second");
    }

    #[test]
    fn only_a_certified_enclosure_mints_a_certified_range() {
        let r = ParamRange::certified(Interval::from_bounds(0.25, 0.75)).expect("certifies");
        assert_eq!(ends(r), (0.25, 0.75));
        assert!(
            ParamRange::certified(Interval::from_f64(f64::NAN)).is_none(),
            "NaI"
        );
        let empty = Interval::from_bounds(-2.0, -1.0).sqrt();
        assert!(ParamRange::certified(empty).is_none(), "empty");
        // `sqrt([-1, 4]) = [0, 2]` is a bracket that does not certify.
        let trv = Interval::from_bounds(-1.0, 4.0).sqrt();
        assert!(!trv.is_certified(), "the fixture is a Trv enclosure");
        assert!(ParamRange::certified(trv).is_none(), "Trv");
    }

    #[test]
    fn clamping_keeps_the_range_ordered() {
        let domain = ParamRange::new(0.0, 3.0).unwrap();
        let clamp = |lo, hi| ends(ParamRange::new(lo, hi).unwrap().clamp_to(domain));
        assert_eq!(clamp(-1.0, 1.0), (0.0, 1.0));
        assert_eq!(
            clamp(4.0, 5.0),
            (3.0, 3.0),
            "past the upper end: that end's point"
        );
        assert_eq!(clamp(-5.0, -4.0), (0.0, 0.0), "past the lower end");
        assert_eq!(clamp(f64::NEG_INFINITY, f64::INFINITY), (0.0, 3.0));
    }

    /// The search against its definition, at a repeated value, below
    /// everything, above everything and between.
    #[test]
    fn the_search_is_the_last_index_at_or_below() {
        let sorted = [0.0, 0.0, 1.0, 1.0, 1.0, 2.0];
        let at = |t: f64| last_at_or_below(&sorted, Param::new(t).unwrap());
        assert_eq!(at(-1.0), None);
        assert_eq!(at(0.0), Some(1), "a run answers its last index");
        assert_eq!(at(0.5), Some(1));
        assert_eq!(at(1.0), Some(4));
        assert_eq!(at(2.0), Some(5));
        assert_eq!(at(f64::INFINITY), Some(5));
        assert_eq!(at(f64::NEG_INFINITY), None);
    }
}
