//! [`ParamRange`]: a parameter region as a type.

use crate::interval::Interval;
use crate::real::CertifiedEnclosure;

/// A closed parameter range `[lo, hi]` with `lo ≤ hi` and neither end
/// NaN — the window every span reader takes.
///
/// The fields are private and the two constructors refuse a NaN or
/// inverted pair, so a window that names no region has no spelling, and
/// a reader cannot forget to ask whether its ends are numbers and
/// ordered. The ends may be infinite: an out-of-domain end clamps to an
/// end span exactly as a finite one does.
///
/// **The invariant is a bracket, not a certificate.** How a range was
/// minted says whether it is certified: [`ParamRange::certified`] mints
/// only from an enclosure that certifies, [`ParamRange::new`] from any
/// ordered pair of numbers.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ParamRange {
    lo: f64,
    hi: f64,
}

impl ParamRange {
    /// `[lo, hi]`, or `None` when either end is NaN or `hi < lo`.
    pub fn new(lo: f64, hi: f64) -> Option<Self> {
        // `<=` is false for a NaN on either side.
        (lo <= hi).then_some(Self { lo, hi })
    }

    /// The bracket of `x`, or `None` when `x` does not certify
    /// ([`CertifiedEnclosure::certified_bracket`]): a NaI, an empty
    /// enclosure and a `Trv` one all refuse.
    pub fn certified(x: Interval) -> Option<Self> {
        let (lo, hi) = x.certified_bracket()?;
        Self::new(lo, hi)
    }

    /// The lower end.
    pub fn lo(self) -> f64 {
        self.lo
    }

    /// The upper end.
    pub fn hi(self) -> f64 {
        self.hi
    }

    /// `lo.midpoint(hi)`. NaN only for the whole line `(−∞, +∞)`, which
    /// has no midpoint; [`ParamRange::clamp_to`] a finite domain first.
    pub fn mid(self) -> f64 {
        self.lo.midpoint(self.hi)
    }

    /// Each end clamped into `domain`. Clamping is monotone, so an
    /// ordered range stays ordered: a range wholly past one end of the
    /// domain becomes that end's point.
    #[must_use]
    pub fn clamp_to(self, domain: Self) -> Self {
        let clamp = |x: f64| x.max(domain.lo).min(domain.hi);
        Self {
            lo: clamp(self.lo),
            hi: clamp(self.hi),
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::real::Real;

    #[test]
    fn a_nan_or_inverted_pair_is_not_a_range() {
        assert!(ParamRange::new(f64::NAN, 1.0).is_none(), "NaN lo");
        assert!(ParamRange::new(0.0, f64::NAN).is_none(), "NaN hi");
        assert!(ParamRange::new(1.0, 0.0).is_none(), "inverted");
        let point = ParamRange::new(0.5, 0.5).expect("a point is a range");
        assert_eq!((point.lo(), point.hi()), (0.5, 0.5));
        let wide = ParamRange::new(f64::NEG_INFINITY, f64::INFINITY).expect("the whole line");
        assert!(wide.mid().is_nan(), "the whole line has no midpoint");
    }

    #[test]
    fn only_a_certified_enclosure_mints_a_certified_range() {
        let r = ParamRange::certified(Interval::from_bounds(0.25, 0.75)).expect("certifies");
        assert_eq!((r.lo(), r.hi()), (0.25, 0.75));
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
        let clamp = |lo, hi| {
            let r = ParamRange::new(lo, hi).unwrap().clamp_to(domain);
            (r.lo(), r.hi())
        };
        assert_eq!(clamp(-1.0, 1.0), (0.0, 1.0));
        assert_eq!(
            clamp(4.0, 5.0),
            (3.0, 3.0),
            "past the upper end: that end's point"
        );
        assert_eq!(clamp(-5.0, -4.0), (0.0, 0.0), "past the lower end");
        assert_eq!(clamp(f64::NEG_INFINITY, f64::INFINITY), (0.0, 3.0));
    }
}
