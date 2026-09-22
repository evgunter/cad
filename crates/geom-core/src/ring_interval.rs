//! The **C9 interval ring**: the certification substrate's enclosure
//! type, a newtype over the backend's decorated interval
//! (`interval_transcendentals::DInterval`). This is the arithmetic
//! every M5 fitted-cache certification stands on
//! (`crates/geom-brep/README.md` C9, C2.2).
//!
//! # Two interval roles, one arithmetic
//!
//! The kernel carries two interval-shaped types and they mean
//! different things, but the difference is no longer in the
//! arithmetic: the ring IS the backend, wearing the certification
//! substrate's surface. Both compile in every build, so nothing in the
//! build configuration separates them; what separates them is the role
//! each plays, and it stays that way until RING-3 dissolves this
//! newtype into [`Interval`](crate::interval::Interval).
//!
//! - [`Interval`](crate::interval::Interval) is a
//!   **[`Real`](crate::Real) instantiation** — an evaluation scalar.
//!   Geometry recipes are written generically over `Real` and
//!   *replayed* at that type; it carries transcendentals, decorations,
//!   and a [`Decide`](crate::Decide) impl. The `interval` cargo feature
//!   gates the lane-trait impls above this crate and the interval test
//!   files — not the type, and not the generic bodies that take it.
//! - [`RingInterval`] (this module) is **certification
//!   substrate**. It is *not* a `Real` instantiation and deliberately
//!   does not implement `Real`: no transcendentals, no `Decide`, no
//!   evaluation-generic code may name it. Certification code bounds
//!   quantities with it and reads the bracket through
//!   [`Enclosure`].
//!
//! Because it is certification substrate rather than evaluation code,
//! **raw `f64` comparisons inside this module are correct and
//! intended** (the no-comparison discipline of `real.rs` governs
//! evaluation code generic over `T`; there is no `T` here — the
//! endpoints are concrete `f64` structure).
//!
//! # Semantics: the decoration is the poison channel
//!
//! A `RingInterval` is **poison** exactly when its decoration is below
//! `Decoration::Def` — the same threshold
//! [`Interval::is_certified`](crate::interval::Interval::is_certified)
//! asks. That one test subsumes NaI and the empty set (both carry a
//! decoration below `Def`) and it also catches the shape a NaN-endpoint
//! rule cannot see: a **refusal carrying real endpoints**, which is
//! what a zero-touching divisor and a domain-clamped crossing produce.
//! Poison never certifies and flows through every operation, because
//! every operation propagates the minimum decoration of its operands.
//!
//! **A poisoned bracket's endpoints are not reliably NaN**, so a
//! consumer that reads one side and compares it must ask
//! [`RingInterval::is_poison`] first. An `f64` comparison against NaN
//! is not the refusal any more; the decoration is.
//!
//! Infinite endpoints are permitted as *bounds* (an honest
//! `[MAX, +inf]` after overflow beats a lie), but a non-finite
//! **point** is poison: `point(f64::INFINITY)` cannot stand for a real
//! number.
//!
//! # Rounding: the backend's, with its exactness witnesses
//!
//! Every arithmetic result is outward-rounded by the backend
//! (`interval-transcendentals/src/round.rs`), which pads only where the
//! operation was inexact. There is no unconditional one-ulp pad here
//! and no sign clamp: the backend's rules are the only rules, and where
//! the reals make a bound exact — `0` as the infimum of a
//! zero-straddling even power, `0 · x = 0` for a zero factor — the
//! backend says so from its own corner conventions rather than from a
//! clamp applied afterwards.
//!
//! `ci.yml`'s "interval-square `powi(2)` allowlist" step still gates
//! `x * x` over `crates/*/src`: [`RingInterval::sqr`] is the operation
//! for a square, because `x * x` treats the factors as independent and
//! gives a zero-straddling argument a spurious negative lower bound.
//!
//! # Determinism (D9)
//!
//! No rounding-mode state, no platform branches, no libm on this path:
//! the backend is directed padding around IEEE-required operations, so
//! results are bit-identical on every conforming platform, and every
//! reduction order is the written one.

use interval_transcendentals::{DInterval, Decoration};

use crate::real::{CertifiedEnclosure, Enclosure};

/// One representable step up, for the width this type rounds itself.
#[inline]
fn up1(x: f64) -> f64 {
    x.next_up()
}

/// An enclosure of a real quantity: `[lo, hi]` with a decoration that
/// is the poison channel. See the module docs for the full contract —
/// poison is `dec < Def`, the backend's arithmetic, certification
/// substrate only.
///
/// No `PartialEq` / `PartialOrd` / `Hash`, matching the `linalg`
/// charter and the interval scalar: enclosure comparison is a
/// certification decision (`hi() <= eps`), never a bit pattern, and
/// `==` on a pair of brackets would silently mean the wrong thing.
#[derive(Clone, Copy, Debug)]
pub struct RingInterval(DInterval);

impl RingInterval {
    /// The poison value: NaI, the permanent refusal. Never certifies,
    /// flows through every operation.
    pub fn poison() -> Self {
        Self(DInterval::nai())
    }

    /// The degenerate enclosure of an exactly-known value. A non-finite
    /// `x` (NaN **or** ±inf) is poison: neither stands for a real
    /// number, and an infinite *point* would launder overflow into
    /// data.
    pub fn point(x: f64) -> Self {
        Self(DInterval::point(x))
    }

    /// The enclosure with the given endpoints. The refusal set is the
    /// backend's, and it is the ring's: NaN endpoints, an inverted
    /// bracket (`lo > hi`), and a bracket whose closed side sits at
    /// infinity (`[+inf, +inf]`, `[-inf, -inf]`, `lo == +inf`,
    /// `hi == -inf`) contain no real number at all and construct
    /// poison. An infinite *endpoint* on the open side is accepted (an
    /// honest unbounded side beats a lie) and caps the decoration at
    /// `Dac`.
    pub fn from_bounds(lo: f64, hi: f64) -> Self {
        Self(DInterval::from_bounds(lo, hi))
    }

    /// Reads a scalar's bracket into the ring through the **certified**
    /// door, carrying the refusal in the DECORATION rather than in the
    /// endpoints.
    ///
    /// This is the C9 ring's one body for a lane scalar taken whole:
    /// every crossing reaches it, whether it calls this door directly or
    /// through a private wrapper that adds a name over this same body and
    /// nothing else. (A crossing that reads only *one* end of the bracket,
    /// such as a symmetric pad built from `hi`, is a different operation
    /// and spells its own refusal.)
    ///
    /// A scalar that may not certify crosses as its own bracket capped
    /// at `Trv` — endpoints intact, the refusal recorded where the ring
    /// reads it. That is the whole point of the door: the scalar records
    /// a domain violation in its decoration, not in its endpoints
    /// (`sqrt([−1, 4])` is `[0, 2]` at `Trv`), so the violation has to be
    /// read HERE, while the decoration is still there to read, and
    /// carried on in the channel the ring's own refusal lives in.
    /// Whatever is built from the crossing is a certificate, so a scalar
    /// carrying a sound bracket its computation is not entitled to must
    /// stay poison, rather than become a plausible bound nothing
    /// downstream can question.
    ///
    /// A scalar that does certify crosses capped at `Def`, which is
    /// exactly what
    /// [`certified_bracket`](CertifiedEnclosure::certified_bracket)
    /// promises and no more: the door's verdict is a two-valued one, so
    /// reading a stronger decoration out of it would be a claim the
    /// scalar never made.
    ///
    /// **Why `Def` rather than the crossing scalar's own decoration**,
    /// which for the interval scalar would often be `Com` or `Dac`:
    /// the parameter is `T: CertifiedEnclosure`, and most of its
    /// implementors — `f64`, `Probe`, `Sym` — have no decoration to
    /// carry. Two of the three answers this door can give would then
    /// depend on which scalar the value crossed from rather than on
    /// what was proved about it, and a ring bound built from an `f64`
    /// would be weaker than the identical bound built from an
    /// `Interval`. `Def` is the strongest claim every implementor
    /// actually makes, so it is the one the crossing carries. The cap
    /// is invisible today — nothing downstream reads a ring
    /// decoration but [`Self::is_poison`] — and stops being a cap at
    /// all in RING-3, where the ring dissolves into the interval
    /// scalar and there is no crossing left to make.
    ///
    /// `Bounds::lo`/`Bounds::hi` into [`Self::from_bounds`] is the
    /// *driver's* spelling and stays available: reading a bracket is not
    /// certifying it, and that spelling mints a fresh `Com`/`Dac` with
    /// no claim about the computation behind it.
    pub fn from_certified<T: CertifiedEnclosure>(x: T) -> Self {
        let (lo, hi) = x.crossing_bracket();
        let cap = if x.certified_bracket().is_some() {
            Decoration::Def
        } else {
            Decoration::Trv
        };
        Self(DInterval::from_bounds(lo, hi).with_dec_capped(cap))
    }

    /// The smallest enclosure containing both arguments. Poison in
    /// either argument poisons the hull — a hull that quietly dropped a
    /// poisoned member would certify geometry it never bounded, and the
    /// backend's `hull` treats the empty set as an identity, which is a
    /// different operation from this one.
    pub fn hull(a: Self, b: Self) -> Self {
        if a.is_poison() || b.is_poison() {
            return Self::poison();
        }
        Self(a.0.hull(b.0))
    }

    /// The intersection with the window `[lo, hi]` — narrowing an
    /// enclosure by a fact known independently of the arithmetic that
    /// produced it (`cos` lies in `[−1, 1]` however the series was
    /// summed).
    ///
    /// **Poison first**, and that is the whole reason this is a method
    /// rather than the two-line spelling at each call site: `f64::max`
    /// and `f64::min` return the *non*-NaN operand, so
    /// `from_bounds(x.lo().max(lo), x.hi().min(hi))` resurrects a
    /// poisoned enclosure as the window itself — a plausible,
    /// sound-looking bracket with no argument behind it, which is the
    /// laundering D4 ¶2 exists to prevent. A poisoned enclosure, a NaN
    /// window, and a window disjoint from the enclosure all yield
    /// poison.
    ///
    /// Spelled over the endpoints rather than through the backend's
    /// `intersection`, which caps its result at `Trv` — that would
    /// poison every clamp in the tree, which is a change to what the
    /// door says rather than to what the arithmetic computes.
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

    /// The lower end of the bracket. **NaN only for the NaN-endpoint
    /// poison shapes** (NaI and the empty set); a refusal carrying real
    /// endpoints reports them, so a consumer comparing this value must
    /// ask [`Self::is_poison`] first.
    pub fn lo(self) -> f64 {
        self.0.lo()
    }

    /// The upper end of the bracket; see [`Self::lo`] for what a
    /// poisoned bracket reports.
    pub fn hi(self) -> f64 {
        self.0.hi()
    }

    /// Whether this may not certify: the backend's decoration is below
    /// `Def`, which covers NaI, the empty set, and every refusal that
    /// carries real endpoints.
    pub fn is_poison(self) -> bool {
        self.0.decoration() < Decoration::Def
    }

    /// Whether `x` lies in the enclosure. False for poison and for NaN
    /// `x` (nothing is known to lie in an unknown bracket).
    pub fn contains(self, x: f64) -> bool {
        !self.is_poison() && self.0.contains(x)
    }

    /// The bracket's width, rounded **up** (`NaN` for poison). An
    /// infinite side gives `+inf`.
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

    /// An upper bound on `|x|` over the enclosure, rounded **up**
    /// (`NaN` for poison). This is the scalar sup-norm reading:
    /// `max(|lo|, |hi|)` is exact under negation (sign-bit only), so no
    /// widening is needed.
    pub fn mag(self) -> f64 {
        if self.is_poison() {
            return f64::NAN;
        }
        let (a, b) = (self.0.lo().abs(), self.0.hi().abs());
        if a > b { a } else { b }
    }

    /// The tight square, `x²` over the enclosure — **the operation to
    /// use instead of `x * x`** whenever the argument can straddle
    /// zero. `[-a, b]` squared is `[0, max(a², b²)]`, but the product
    /// of two *independent* enclosures `[-a, b] · [-a, b]` is
    /// `[-ab, …]`: a spurious negative lower bound that poisons a
    /// downstream `sqrt`.
    ///
    /// The backend's even power, whose zero lower bound for a
    /// zero-straddling argument is exact — a fact about the reals.
    pub fn sqr(self) -> Self {
        Self(self.0.powi(2))
    }

    /// Integer powers, the ring's only non-`±×÷` operation — the
    /// backend's.
    ///
    /// `n == 0` is `[1, 1]` for every non-poisoned argument and poison
    /// for poison (the `Real::powi` poison-guard convention: `NaN⁰` is
    /// not 1). Negative `n` is the reciprocal of the positive power, so
    /// a zero-straddling base yields poison through the division.
    ///
    /// Every even power of a zero-straddling enclosure keeps the exact
    /// lower bound `0`.
    pub fn powi(self, n: i32) -> Self {
        Self(self.0.powi(n))
    }
}

/// The ring refuses exactly on poison. Its decoration IS its
/// domain-violation channel, so a bracket below `Def` stands for a
/// computation this ring is not entitled to certify, whatever its
/// endpoints say. Every other ring is a sound bracket with a claim
/// behind it.
///
/// The refusal is this door's own rather than a loan from
/// [`RingInterval::from_bounds`]. Handing back the endpoints and
/// leaving a downstream constructor to reject them is precisely what
/// the trait's method doc excludes: what a generic
/// `T: CertifiedEnclosure` consumer is promised is a bracket it may
/// certify with, not a pair it has to re-check.
impl CertifiedEnclosure for RingInterval {
    fn certified_bracket(self) -> Option<(f64, f64)> {
        (!self.is_poison()).then_some((self.0.lo(), self.0.hi()))
    }

    /// The ring's refusal lives in the decoration, so a refused ring
    /// still has endpoints to report — and a ring crossing into a ring
    /// keeps them rather than minting a NaN pair.
    fn crossing_bracket(self) -> (f64, f64) {
        (self.0.lo(), self.0.hi())
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

    /// **Not** the operation for squares: `x * x` treats the factors as
    /// independent (see [`sqr`](Self::sqr)).
    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }
}

impl core::ops::Div for RingInterval {
    type Output = Self;

    /// **A divisor that straddles or touches zero is poison**, and it is
    /// the backend that says so: the quotient's decoration drops to
    /// `Trv`, which [`RingInterval::is_poison`] reads as the refusal.
    /// Such a quotient carries REAL endpoints (the unbounded hull, or a
    /// finite one-sided bound), so it is the shape a consumer comparing
    /// an endpoint must guard against.
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

    /// The shape a NaN-endpoint poison rule cannot see: the refusal is
    /// in the decoration and the endpoints are real numbers, so every
    /// consumer comparing one side has to ask `is_poison()` first.
    #[test]
    fn a_refusal_can_carry_real_endpoints() {
        let q = ri(-2.0, -1.0) / ri(0.0, 5e-324);
        assert!(q.is_poison());
        assert!(q.lo().is_finite() || q.hi().is_finite(), "{q:?}");
        // The whole hazard in one line: the comparison a consumer writes
        // is TRUE on a value that refuses.
        assert!(q.hi() < 0.0, "{q:?}");
        // And a finite one, through a multiply that annihilates the
        // unbounded side.
        let f = (ri(-2.0, -1.0) / ri(-1.0, 1.0)) * RingInterval::zero();
        assert!(f.is_poison());
        assert!(f.lo() == 0.0 && f.hi() == 0.0, "{f:?}");
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
        // A zero factor annihilates exactly: the backend's corner
        // convention answers `0 · x = 0`, with no pad to clamp back.
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

    /// The crossing carries the scalar's refusal in the decoration and
    /// its endpoints unchanged: an uncertified scalar is poison here
    /// without becoming a NaN bracket.
    #[test]
    fn the_crossing_carries_the_refusal_in_the_decoration() {
        assert!(!RingInterval::from_certified(1.5).is_poison());
        assert!(RingInterval::from_certified(f64::NAN).is_poison());
        let crossed = RingInterval::from_certified(2.0f64);
        assert!(crossed.lo() == 2.0 && crossed.hi() == 2.0);
    }
}
