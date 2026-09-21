//! The **C9 interval ring**, as a newtype over the backend's decorated
//! interval — the RING-2 dry run. Not for merge: this branch exists to
//! name the consumers that depend on a semantic difference between the
//! ring's two-state poison and `DInterval`'s decoration channel.
//!
//! The surface is unchanged, so every caller compiles untouched. What
//! changes is the semantics underneath it:
//!
//! - poison is `dec < Def` (which subsumes NaI and the empty set)
//!   rather than a NaN endpoint pair;
//! - `+ − × ÷ neg powi sqr` are the backend's, so they carry its
//!   exactness witnesses and its `0 · inf := 0` and `inf / inf` corner
//!   conventions;
//! - the sign clamp and the zero annihilator are GONE: the backend's
//!   rules are the only rules;
//! - a divisor that touches or straddles zero is `Trv` in the backend,
//!   which this type's `is_poison` reads as poison — the one rule that
//!   survives verbatim.
//!
//! `from_certified` is unchanged in meaning; `hull` and `clamped_to`
//! keep the ring's refusing shape, because the backend's `hull` treats
//! the empty set as an identity and its `intersection` caps every
//! result at `Trv`, and either would change what the door says rather
//! than what the arithmetic computes.

use interval_transcendentals::{DInterval, Decoration};

use crate::real::{CertifiedEnclosure, Enclosure};

/// One representable step up, for the widths this type still rounds
/// itself.
#[inline]
fn up1(x: f64) -> f64 {
    x.next_up()
}

/// An enclosure of a real quantity: `[lo, hi]`, or poison.
#[derive(Clone, Copy, Debug)]
pub struct RingInterval(DInterval);

impl RingInterval {
    /// The poison value. Never certifies, flows through every
    /// operation.
    pub fn poison() -> Self {
        Self(DInterval::nai())
    }

    /// The degenerate enclosure of an exactly-known value; a non-finite
    /// `x` is poison.
    pub fn point(x: f64) -> Self {
        Self(DInterval::point(x))
    }

    /// The enclosure with the given endpoints; NaN endpoints, an
    /// inverted bracket and a closed side at infinity are poison.
    pub fn from_bounds(lo: f64, hi: f64) -> Self {
        Self(DInterval::from_bounds(lo, hi))
    }

    /// Reads a scalar's bracket into the ring through the **certified**
    /// door, poisoning a scalar that may not certify.
    pub fn from_certified<T: CertifiedEnclosure>(x: T) -> Self {
        match x.certified_bracket() {
            Some((lo, hi)) => Self::from_bounds(lo, hi),
            None => Self::poison(),
        }
    }

    /// The smallest enclosure containing both arguments. Poison in
    /// either argument poisons the hull — a hull that quietly dropped a
    /// poisoned member would certify geometry it never bounded, and the
    /// backend's `hull` treats the empty set as an identity.
    pub fn hull(a: Self, b: Self) -> Self {
        if a.is_poison() || b.is_poison() {
            return Self::poison();
        }
        Self(a.0.hull(b.0))
    }

    /// The intersection with the window `[lo, hi]`. Poison first, a NaN
    /// window is poison, and a window disjoint from the enclosure is
    /// poison.
    ///
    /// Spelled over the endpoints rather than through the backend's
    /// `intersection`, which caps its result at `Trv` and would poison
    /// every clamp.
    pub fn clamped_to(self, lo: f64, hi: f64) -> Self {
        if self.is_poison() || lo.is_nan() || hi.is_nan() {
            return Self::poison();
        }
        let narrowed = DInterval::from_bounds(self.0.lo().max(lo), self.0.hi().min(hi));
        Self(narrowed.with_dec_capped(self.0.decoration()))
    }

    /// The exact zero enclosure `[0, 0]`.
    pub fn zero() -> Self {
        Self(DInterval::point(0.0))
    }

    /// The exact one enclosure `[1, 1]`.
    pub fn one() -> Self {
        Self(DInterval::point(1.0))
    }

    /// The lower end of the bracket (NaN if poisoned by a NaN shape).
    pub fn lo(self) -> f64 {
        self.0.lo()
    }

    /// The upper end of the bracket (NaN if poisoned by a NaN shape).
    pub fn hi(self) -> f64 {
        self.0.hi()
    }

    /// Whether this may not certify: the backend's decoration is below
    /// `Def`, which covers NaI and the empty set.
    pub fn is_poison(self) -> bool {
        self.0.decoration() < Decoration::Def
    }

    /// Whether `x` lies in the enclosure. False for poison and for NaN
    /// `x`.
    pub fn contains(self, x: f64) -> bool {
        !self.is_poison() && self.0.contains(x)
    }

    /// The bracket's width, rounded **up** (`NaN` for poison).
    pub fn width(self) -> f64 {
        if self.is_poison() {
            return f64::NAN;
        }
        let (lo, hi) = (self.0.lo(), self.0.hi());
        if lo == hi {
            return 0.0;
        }
        up1(hi - lo)
    }

    /// An upper bound on `|x|` over the enclosure (`NaN` for poison).
    pub fn mag(self) -> f64 {
        if self.is_poison() {
            return f64::NAN;
        }
        let (a, b) = (self.0.lo().abs(), self.0.hi().abs());
        if a > b { a } else { b }
    }

    /// The tight square — the backend's even power, which carries the
    /// same exact-zero rule for a zero-straddling input.
    pub fn sqr(self) -> Self {
        Self(self.0.powi(2))
    }

    /// Integer powers — the backend's.
    pub fn powi(self, n: i32) -> Self {
        Self(self.0.powi(n))
    }
}

/// The ring refuses exactly on poison.
impl crate::real::CertifiedEnclosure for RingInterval {
    fn certified_bracket(self) -> Option<(f64, f64)> {
        (!self.is_poison()).then_some((self.0.lo(), self.0.hi()))
    }
}

impl Enclosure for RingInterval {
    fn lo(self) -> f64 {
        self.0.lo()
    }

    fn hi(self) -> f64 {
        self.0.hi()
    }
}

impl core::ops::Neg for RingInterval {
    type Output = Self;

    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl core::ops::Add for RingInterval {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl core::ops::Sub for RingInterval {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl core::ops::Mul for RingInterval {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }
}

impl core::ops::Div for RingInterval {
    type Output = Self;

    /// A divisor that straddles or touches zero is `Trv` in the
    /// backend, which [`RingInterval::is_poison`] reads as poison.
    fn div(self, rhs: Self) -> Self {
        Self(self.0 / rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ri(lo: f64, hi: f64) -> RingInterval {
        RingInterval::from_bounds(lo, hi)
    }

    #[test]
    fn construction_poison_paths() {
        assert!(RingInterval::point(f64::NAN).is_poison());
        assert!(RingInterval::point(f64::INFINITY).is_poison());
        assert!(RingInterval::point(f64::NEG_INFINITY).is_poison());
        assert!(ri(1.0, 0.0).is_poison());
        assert!(ri(f64::NAN, 1.0).is_poison());
        assert!(ri(0.0, f64::NAN).is_poison());
        assert!(!ri(f64::MAX, f64::INFINITY).is_poison());
        assert!(!ri(f64::NEG_INFINITY, f64::MIN).is_poison());
        // Brackets that contain no real number at all.
        assert!(ri(f64::INFINITY, f64::INFINITY).is_poison());
        assert!(ri(f64::NEG_INFINITY, f64::NEG_INFINITY).is_poison());
        assert!(ri(f64::INFINITY, f64::NEG_INFINITY).is_poison());
        assert!(ri(f64::NEG_INFINITY, f64::INFINITY).width().is_infinite());
        assert!(RingInterval::point(0.0).contains(0.0));
        assert!(!RingInterval::poison().contains(0.0));
        assert!(!RingInterval::point(0.0).contains(f64::NAN));
    }

    #[test]
    fn poison_flows_through_every_op() {
        let p = RingInterval::poison();
        let x = ri(1.0, 2.0);
        for r in [p + x, x + p, p - x, x - p, p * x, x * p, p / x, x / p, -p] {
            assert!(r.is_poison());
        }
        assert!(p.powi(0).is_poison(), "NaN^0 is not 1");
        assert!(p.powi(3).is_poison());
        assert!(p.sqr().is_poison());
        assert!(RingInterval::hull(p, x).is_poison());
        assert!(p.width().is_nan());
        assert!(p.mag().is_nan());
    }

    #[test]
    fn division_refuses_zero_straddling_divisors() {
        let x = ri(1.0, 2.0);
        for d in [ri(-1.0, 1.0), ri(0.0, 1.0), ri(-1.0, 0.0), ri(0.0, 0.0)] {
            assert!((x / d).is_poison(), "divisor touching zero must poison");
        }
        assert!(!(x / ri(1.0, 2.0)).is_poison());
        assert!(!(x / ri(-2.0, -1.0)).is_poison());
        // Negative powers inherit the refusal.
        assert!(ri(-1.0, 1.0).powi(-1).is_poison());
    }

    #[test]
    fn ops_contain_the_true_result_and_widen_outward() {
        // 0.1 + 0.2 is inexact at f64; the bracket must contain both the
        // RN result and the exact sum.
        let s = RingInterval::point(0.1) + RingInterval::point(0.2);
        assert!(s.contains(0.1 + 0.2));
        assert!(s.lo() < 0.1 + 0.2 && 0.1 + 0.2 < s.hi());
        let q = RingInterval::point(1.0) / RingInterval::point(3.0);
        assert!(q.contains(1.0 / 3.0) && q.lo() < q.hi());
        let m = ri(-2.0, 3.0) * ri(-5.0, 7.0);
        assert!(m.lo() <= -15.0 && m.hi() >= 21.0);
    }

    #[test]
    fn even_powers_of_straddling_enclosures_keep_lower_bound_zero() {
        // The zero-straddling-square lesson, at the ring.
        let x = ri(-3.0, 2.0);
        assert!(x.sqr().lo() == 0.0 && x.sqr().hi() >= 9.0);
        for n in [2, 4, 6, 8, 10, 12] {
            let p = x.powi(n);
            assert!(p.lo() == 0.0, "x^{n} lower bound {} != 0", p.lo());
            assert!(p.hi() >= 3.0f64.powi(n));
        }
        // The independent-factor product is exactly the trap avoided.
        assert!((x * x).lo() < 0.0, "plain Mul is the trap sqr avoids");
        // Odd powers keep the sign structure.
        assert!(x.powi(3).lo() <= -27.0 && x.powi(3).hi() >= 8.0);
        // Zero is exact under the sign clamp.
        let z = RingInterval::zero() * ri(-1e300, 1e300);
        assert!(z.lo() == 0.0 && z.hi() == 0.0);
    }

    #[test]
    fn powi_edges() {
        let x = ri(2.0, 3.0);
        assert!(x.powi(0).contains(1.0) && x.powi(0).width() == 0.0);
        assert!(x.powi(1).lo() == 2.0 && x.powi(1).hi() == 3.0);
        assert!(x.powi(-2).contains(1.0 / 9.0) && x.powi(-2).contains(0.25));
        // i32::MIN must not overflow the exponent: the magnitude goes
        // through `unsigned_abs`, and the overflowed positive power is
        // the honest `[MAX, +inf]` rather than poison, so its
        // reciprocal is a finite underflowed bracket of zero.
        let tiny = x.powi(i32::MIN);
        assert!(!tiny.is_poison(), "{tiny:?}");
        assert!(tiny.contains(0.0) && tiny.hi() < 1e-300, "{tiny:?}");
    }
}
