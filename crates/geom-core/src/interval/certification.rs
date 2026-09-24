//! The certification doors: the [`Certification`] trait, the surface
//! certification arithmetic builds and reads [`Interval`] brackets
//! through.
//!
//! A file opts into certification arithmetic by importing the trait by
//! name — `use geom_core::interval::certification::Certification;` — and
//! a file that does so has no [`crate::Real`] in scope in its production
//! code, so an `Interval` there reaches neither `Real::is_poison` (NaI or
//! empty, which answers `false` on a `Trv` bracket with real endpoints)
//! nor a transcendental (which certification does not call —
//! `crates/geom-brep/README.md` C9). `scripts/gates/certification-doors.sh`
//! holds that separation over its allowlist of importers. The trait is
//! not re-exported at the crate root or beside [`Interval`], so no glob
//! import carries it in, and this module is the one place it is defined
//! with `Real` in scope.
//!
//! What stays inherent on [`Interval`]: [`Interval::from_bounds`] (the
//! driver's door in), [`Interval::repr_bits`] (the identity channel),
//! [`Interval::is_certified`] (the refusal predicate, which evaluation's
//! [`crate::Decide::sign_within`] asks too), and
//! [`Interval::from_certified`] — the crossing from a lane scalar INTO
//! certification arithmetic, called from files that legitimately hold
//! a lane `T: Real`.

use interval_transcendentals::DInterval;

use super::Interval;
use crate::real::Real;

/// Seals [`Certification`]: implemented for [`Interval`] alone.
pub(crate) mod sealed {
    /// The sealing supertrait (pub-in-private: unnameable downstream).
    pub trait Sealed {}
    impl Sealed for super::Interval {}
}

/// # The certification doors
///
/// Certification arithmetic — de Boor over coefficient enclosures, hull
/// bounds, implicit residuals, the mass-property quadrature's
/// certificates — runs on [`Interval`], and these are the doors it builds
/// and reads brackets through. Every one of them treats a value that is
/// not [`Interval::is_certified`] as a **refusal**: it hulls to NaI,
/// clamps to NaI, contains nothing, and has a NaN width and magnitude.
/// The arithmetic operators need no door of their own — every backend
/// operation propagates the minimum decoration of its operands, so a
/// refusal flows through `+ − × ÷` and every power and stays one.
///
/// **A refusal is not a NaN pair.** A quotient by a divisor that touches
/// zero, a negative power of a zero-straddling base, and a crossing from
/// a scalar that may not certify all carry ORDINARY endpoints below
/// `Def`, so a site that reads one endpoint and compares it asks
/// [`Interval::is_certified`] first, by name. That is the refusal
/// predicate here, and it is deliberately not [`Real::is_poison`], which
/// at this scalar asks only whether the value is NaI or empty — the
/// evaluation scalar's poison, a strictly weaker question.
///
/// The endpoints are read through [`crate::Bounds`], the door out, and
/// its `lo`/`hi` are NaN only for NaI and the empty set: a refusal
/// carrying real endpoints reports them.
///
/// The constructors that restate a [`Real`] method (`point`, `zero`,
/// `one`, `powi`) delegate to it and add nothing: they are the spellings
/// a certification file, which has no `Real` in scope, builds brackets
/// with.
///
/// Sealed: implemented for [`Interval`] alone.
pub trait Certification: sealed::Sealed + Copy {
    /// The ill-formed interval (NaI): the permanent refusal. Never
    /// certifies, flows through every operation.
    #[must_use]
    fn refused() -> Self;

    /// The degenerate enclosure of an exactly-known value —
    /// [`Real::from_f64`]. A non-finite `x` (NaN **or** ±inf) is NaI:
    /// neither stands for a real number, and an infinite *point* would
    /// launder overflow into data.
    #[must_use]
    fn point(x: f64) -> Self;

    /// The exact zero enclosure `[0, 0]` — [`Real::zero`].
    #[must_use]
    fn zero() -> Self;

    /// The exact one enclosure `[1, 1]` — [`Real::one`].
    #[must_use]
    fn one() -> Self;

    /// The smallest enclosure containing both arguments. A refusal in
    /// either argument makes the hull NaI — a hull that quietly dropped
    /// a refused member would certify geometry it never bounded, and the
    /// backend's `hull` treats the empty set as an identity, which is a
    /// different operation from this one. The evaluation scalar's hull,
    /// through which a refusal flows at the minimum decoration, is
    /// [`crate::SpanLocate::enclosure_hull`]: a different operation
    /// under a different name.
    #[must_use]
    fn hull(a: Self, b: Self) -> Self;

    /// The intersection with the window `[lo, hi]` — narrowing an
    /// enclosure by a fact known independently of the arithmetic that
    /// produced it (`cos` lies in `[−1, 1]` however the series was
    /// summed).
    ///
    /// **Refusal first**, and that is the whole reason this is a method
    /// rather than the two-line spelling at each call site: `f64::max`
    /// and `f64::min` return the *non*-NaN operand, so
    /// `from_bounds(x.lo().max(lo), x.hi().min(hi))` resurrects a
    /// refused enclosure as the window itself — a plausible,
    /// sound-looking bracket with no argument behind it, which is the
    /// laundering D4 ¶2 exists to prevent. A refused enclosure, a NaN
    /// window, and a window disjoint from the enclosure all yield NaI.
    ///
    /// Spelled over the endpoints, keeping the enclosure's own
    /// decoration, rather than through the backend's `intersection`,
    /// which caps its result at `Trv` — that would refuse every clamp,
    /// which is a change to what the door says rather than to what the
    /// arithmetic computes.
    #[must_use]
    fn clamped_to(self, lo: f64, hi: f64) -> Self;

    /// Whether `x` lies in the enclosure. False for a refusal and for a
    /// NaN `x` (nothing is known to lie in a bracket that may not
    /// certify).
    #[must_use]
    fn contains(self, x: f64) -> bool;

    /// The bracket's width, rounded **up** by one step (`hi − lo` is
    /// inexact in general); `0` for a point, `+inf` for an infinite
    /// side, and `NaN` for a refusal.
    #[must_use]
    fn width(self) -> f64;

    /// An upper bound on `|x|` over the enclosure (`NaN` for a refusal).
    /// This is the scalar sup-norm reading: `max(|lo|, |hi|)` is exact
    /// under negation (sign-bit only), so no widening is needed.
    #[must_use]
    fn mag(self) -> f64;

    /// The tight square, `x²` over the enclosure — **the operation to
    /// use instead of `x * x`** whenever the argument can straddle
    /// zero. `[-a, b]` squared is `[0, max(a², b²)]`, but the product
    /// of two *independent* enclosures `[-a, b] · [-a, b]` is
    /// `[-ab, …]`: a spurious negative lower bound that refuses a
    /// downstream `sqrt`. The backend's even power, whose zero lower
    /// bound for a zero-straddling argument is exact.
    #[must_use]
    fn sqr(self) -> Self;

    /// Integer powers — [`Real::powi`], the backend's: `n == 0` refuses
    /// a refused base (`NaN⁰` is not 1), a negative `n` is the
    /// reciprocal of the positive power (so a zero-straddling base
    /// refuses through the division), and every even power of a
    /// zero-straddling enclosure keeps the exact lower bound `0`.
    #[must_use]
    fn powi(self, n: i32) -> Self;
}

/// Raw `f64` comparisons inside these bodies are scalar-implementation
/// code (Q1's allowance, as in [`Real::min`] at `f64`): the endpoints are
/// concrete structure, and there is no `T` here.
impl Certification for Interval {
    fn refused() -> Self {
        Self(DInterval::nai())
    }

    fn point(x: f64) -> Self {
        <Self as Real>::from_f64(x)
    }

    fn zero() -> Self {
        <Self as Real>::zero()
    }

    fn one() -> Self {
        <Self as Real>::one()
    }

    fn hull(a: Self, b: Self) -> Self {
        if !a.is_certified() || !b.is_certified() {
            return Self::refused();
        }
        Self(a.0.hull(b.0))
    }

    fn clamped_to(self, lo: f64, hi: f64) -> Self {
        if !self.is_certified() || lo.is_nan() || hi.is_nan() {
            return Self::refused();
        }
        let narrowed = DInterval::from_bounds(self.0.lo().max(lo), self.0.hi().min(hi));
        Self(narrowed.with_dec_capped(self.0.decoration()))
    }

    fn contains(self, x: f64) -> bool {
        self.is_certified() && self.0.contains(x)
    }

    fn width(self) -> f64 {
        if !self.is_certified() {
            return f64::NAN;
        }
        let (lo, hi) = (self.0.lo(), self.0.hi());
        if lo == hi {
            return 0.0;
        }
        (hi - lo).next_up()
    }

    fn mag(self) -> f64 {
        if !self.is_certified() {
            return f64::NAN;
        }
        let (a, b) = (self.0.lo().abs(), self.0.hi().abs());
        if a > b { a } else { b }
    }

    fn sqr(self) -> Self {
        Real::powi(self, 2)
    }

    fn powi(self, n: i32) -> Self {
        Real::powi(self, n)
    }
}

/// The certification doors' own rows: every door refuses a value that
/// is not certified, whatever its endpoints say.
#[cfg(test)]
mod certification_door_tests {
    use super::{Certification, Interval};
    use crate::real::Bounds;

    fn ri(lo: f64, hi: f64) -> Interval {
        Interval::from_bounds(lo, hi)
    }

    #[test]
    fn construction_poison_paths() {
        assert!(!Interval::point(f64::NAN).is_certified());
        assert!(!Interval::point(f64::INFINITY).is_certified());
        assert!(!Interval::point(f64::NEG_INFINITY).is_certified());
        assert!(!ri(1.0, 0.0).is_certified());
        assert!(!ri(f64::NAN, 1.0).is_certified());
        assert!(!ri(0.0, f64::NAN).is_certified());
        assert!(ri(f64::MAX, f64::INFINITY).is_certified());
        assert!(ri(f64::NEG_INFINITY, f64::MIN).is_certified());
        // Brackets that contain no real number at all.
        assert!(!ri(f64::INFINITY, f64::INFINITY).is_certified());
        assert!(!ri(f64::NEG_INFINITY, f64::NEG_INFINITY).is_certified());
        assert!(!ri(f64::INFINITY, f64::NEG_INFINITY).is_certified());
        assert!(ri(f64::NEG_INFINITY, f64::INFINITY).width().is_infinite());
        assert!(Interval::point(0.0).contains(0.0));
        assert!(!Interval::refused().contains(0.0));
        assert!(!Interval::point(0.0).contains(f64::NAN));
    }

    #[test]
    fn poison_flows_through_every_op() {
        let p = Interval::refused();
        let x = ri(1.0, 2.0);
        for r in [p + x, x + p, p - x, x - p, p * x, x * p, p / x, x / p, -p] {
            assert!(!r.is_certified());
        }
        assert!(!p.powi(0).is_certified(), "NaN^0 is not 1");
        assert!(!p.powi(3).is_certified());
        assert!(!p.sqr().is_certified());
        assert!(!Interval::hull(p, x).is_certified());
        assert!(p.width().is_nan());
        assert!(p.mag().is_nan());
    }

    /// The hull's refusing guard, where it differs from the backend's
    /// hull. A `Trv` operand is refused either way (the backend's hull
    /// carries the minimum decoration); an EMPTY one is not — the backend
    /// treats the empty set as a hull identity, so without the guard
    /// `hull(∅, x)` is `x`, certified, and a refused member has vanished.
    #[test]
    fn hull_refuses_an_empty_operand_the_backend_would_absorb() {
        let x = ri(1.0, 2.0);
        let empty = x / Interval::zero();
        assert!(!empty.is_certified(), "{empty:?}");
        assert!(!Interval::hull(empty, x).is_certified());
        assert!(!Interval::hull(x, empty).is_certified());
        let trv = ri(-2.0, -1.0) / ri(0.0, 1.0);
        assert!(!Interval::hull(trv, x).is_certified());
    }

    #[test]
    fn division_refuses_zero_straddling_divisors() {
        let x = ri(1.0, 2.0);
        for d in [ri(-1.0, 1.0), ri(0.0, 1.0), ri(-1.0, 0.0), ri(0.0, 0.0)] {
            assert!(!(x / d).is_certified(), "divisor touching zero must refuse");
        }
        assert!((x / ri(1.0, 2.0)).is_certified());
        assert!((x / ri(-2.0, -1.0)).is_certified());
        // Negative powers inherit the refusal.
        assert!(!ri(-1.0, 1.0).powi(-1).is_certified());
    }

    /// The shape a NaN-endpoint poison rule cannot see: the refusal is
    /// in the decoration and the endpoints are real numbers, so every
    /// consumer comparing one side has to ask `is_certified()` first.
    #[test]
    fn a_refusal_can_carry_real_endpoints() {
        let q = ri(-2.0, -1.0) / ri(0.0, 5e-324);
        assert!(!q.is_certified());
        assert!(q.lo().is_finite() || q.hi().is_finite(), "{q:?}");
        // The whole hazard in one line: the comparison a consumer writes
        // is TRUE on a value that refuses.
        assert!(q.hi() < 0.0, "{q:?}");
        // And a finite one, through a multiply that annihilates the
        // unbounded side.
        let f = (ri(-2.0, -1.0) / ri(-1.0, 1.0)) * Interval::zero();
        assert!(!f.is_certified());
        assert!(f.lo() == 0.0 && f.hi() == 0.0, "{f:?}");
    }

    #[test]
    fn ops_contain_the_true_result_and_widen_outward() {
        // 0.1 + 0.2 is inexact at f64; the bracket must contain both the
        // RN result and the exact sum.
        let s = Interval::point(0.1) + Interval::point(0.2);
        assert!(s.contains(0.1 + 0.2));
        assert!(s.lo() < 0.1 + 0.2 && 0.1 + 0.2 < s.hi());
        let q = Interval::point(1.0) / Interval::point(3.0);
        assert!(q.contains(1.0 / 3.0) && q.lo() < q.hi());
        let m = ri(-2.0, 3.0) * ri(-5.0, 7.0);
        assert!(m.lo() <= -15.0 && m.hi() >= 21.0);
    }

    #[test]
    fn even_powers_of_straddling_enclosures_keep_lower_bound_zero() {
        // The zero-straddling-square lesson.
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
        // A zero factor annihilates exactly: the backend's corner
        // convention answers `0 · x = 0`, with no pad to clamp back.
        let z = Interval::zero() * ri(-1e300, 1e300);
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
        // the honest `[MAX, +inf]` rather than a refusal, so its
        // reciprocal is a finite underflowed bracket of zero.
        let tiny = x.powi(i32::MIN);
        assert!(tiny.is_certified(), "{tiny:?}");
        assert!(tiny.contains(0.0) && tiny.hi() < 1e-300, "{tiny:?}");
    }

    /// The crossing carries the scalar's refusal in the decoration and
    /// its endpoints unchanged: an uncertified scalar is refused here
    /// without becoming a NaN bracket.
    #[test]
    fn the_crossing_carries_the_refusal_in_the_decoration() {
        assert!(Interval::from_certified(1.5).is_certified());
        assert!(!Interval::from_certified(f64::NAN).is_certified());
        let crossed = Interval::from_certified(2.0f64);
        assert!(crossed.lo() == 2.0 && crossed.hi() == 2.0);
    }
}
