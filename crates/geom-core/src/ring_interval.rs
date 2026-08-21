//! The **C9 interval ring**: a small, MIT-clean, always-compiled
//! enclosure type with `±`, `×`, `÷` and integer powers, and nothing
//! else. This is the arithmetic every M5 fitted-cache certification
//! stands on (`docs/CURVED-DESIGN.md` C9, C2.2).
//!
//! # Two interval roles, deliberately distinct
//!
//! The kernel now carries two interval-shaped types, and confusing them
//! would be a design error:
//!
//! - `geom_core::interval::Interval` (behind the `interval` cargo
//!   feature — deliberately unlinked: the module does not exist in a
//!   default build, and a broken intra-doc link would) is a
//!   **[`Real`](crate::Real) instantiation** — an evaluation scalar. Geometry recipes are written generically over `Real` and
//!   *replayed* at that type; it carries transcendentals, decorations,
//!   and a [`Decide`](crate::Decide) impl.
//! - [`RingInterval`] (this module, **always compiled**, no feature
//!   gate — that is its reason to exist) is **certification
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
//! endpoints are concrete `f64` structure, and min/max/sign reasoning
//! on them is exactly the arithmetic's job).
//!
//! # Semantics: exactly two states
//!
//! An `RingInterval` is either an **enclosure** `[lo, hi]` with
//! `lo <= hi`, both non-NaN, or **poison** (both endpoints NaN). There
//! is no decoration channel and no empty set: poison is the single
//! failure state, it flows through every operation, and it can never
//! certify — `residual.hi() <= eps` is false for NaN under every
//! comparison direction, which is the [`Bounds`](crate::Bounds)
//! convention this type mirrors (D4 ¶2, fail-loud).
//!
//! Infinite endpoints are permitted as *bounds* (an honest
//! `[MAX, +inf]` after overflow beats a lie), but a non-finite
//! **point** is poison: `point(f64::INFINITY)` cannot stand for a real
//! number.
//!
//! # Rounding: outward ulp-widening, unconditional
//!
//! Portable Rust cannot set the FPU rounding mode, so every arithmetic
//! result is padded outward by one representable step around the
//! round-to-nearest result (`next_down` / `next_up`), the technique
//! `interval-transcendentals/src/round.rs` proves out (its Lemma P1:
//! for a correctly-rounded-to-nearest `c` of a real `t`,
//! `next_down(c) <= t <= next_up(c)`, because no representable number
//! lies strictly between `t` and `c`). `+`, `-`, `*`, `/` on `f64` are
//! correctly rounded by IEEE 754, so one step suffices.
//!
//! The padding is **unconditional**: v1 ships no exactness witnesses
//! (the "this operation was exact, do not widen" test that
//! `interval-transcendentals` implements for division). Widening every
//! op costs one ulp per operation and is the conservatism C9 sanctions
//! ("sound, slightly conservative"); witnesses carry their own proof
//! burden and belong to the crate that already owns that craft. **The
//! door is noted, not built** — a later revision may add them without
//! changing any signature here.
//!
//! Widening is nonetheless *clamped by sign reasoning*: a product of
//! two nonnegative enclosures has true infimum `>= 0`, so the widened
//! lower bound is raised back to `0.0`. That is a fact about the reals,
//! not a claim about floating-point exactness, and it is what keeps
//! even powers of zero-straddling quantities from acquiring a spurious
//! negative lower bound — the bug class that cost M2 three separate
//! occurrences, and that ci.yml's "interval-square powi(2) allowlist"
//! step now gates tree-wide.
//!
//! # Determinism (D9)
//!
//! No rounding-mode state, no platform branches, no libm: the ring is
//! `next_down`/`next_up` around IEEE-required operations, so results
//! are bit-identical on every conforming platform, and every reduction
//! order in this module is the written one.

use crate::real::{CertifiedEnclosure, Enclosure};

/// One representable step down; `next_down(-inf) = -inf`, the correct
/// lower bound for an already-unbounded side.
#[inline]
fn down1(x: f64) -> f64 {
    x.next_down()
}

/// One representable step up; mirror of [`down1`].
#[inline]
fn up1(x: f64) -> f64 {
    x.next_up()
}

/// An enclosure of a real quantity: `[lo, hi]`, or poison (NaN
/// brackets). See the module docs for the full contract — two states,
/// no decorations, unconditional outward widening, certification
/// substrate only.
///
/// No `PartialEq` / `PartialOrd` / `Hash`, matching the `linalg`
/// charter and the interval scalar: enclosure comparison is a
/// certification decision (`hi() <= eps`), never a bit pattern, and
/// `==` on a pair of brackets would silently mean the wrong thing.
#[derive(Clone, Copy, Debug)]
pub struct RingInterval {
    lo: f64,
    hi: f64,
}

impl RingInterval {
    /// The poison value: NaN brackets. Never certifies, flows through
    /// every operation.
    pub fn poison() -> Self {
        Self {
            lo: f64::NAN,
            hi: f64::NAN,
        }
    }

    /// The degenerate enclosure of an exactly-known value. A non-finite
    /// `x` (NaN **or** ±inf) is poison: neither stands for a real
    /// number, and an infinite *point* would launder overflow into
    /// data.
    pub fn point(x: f64) -> Self {
        if x.is_finite() {
            Self { lo: x, hi: x }
        } else {
            Self::poison()
        }
    }

    /// The enclosure with the given endpoints. NaN endpoints or an
    /// inverted bracket (`lo > hi`) are poison; an infinite *endpoint*
    /// is accepted on the open side (an honest unbounded side beats a
    /// lie).
    ///
    /// `lo = +inf` or `hi = -inf` is **also poison**: `[+inf, +inf]`,
    /// `[-inf, -inf]`, or any bracket whose closed side sits at infinity
    /// contains no real number at all, so it cannot be an enclosure of
    /// one. Admitting it would let arithmetic proceed on a value that
    /// stands for nothing — and `[+inf, +inf]` would even pass `Div`'s
    /// away-from-zero test.
    // `!(lo <= hi)` is deliberate NaN-catching: `lo > hi` would let NaN
    // through (same idiom as spline weight validation).
    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    pub fn from_bounds(lo: f64, hi: f64) -> Self {
        if !(lo <= hi) || lo == f64::INFINITY || hi == f64::NEG_INFINITY {
            return Self::poison();
        }
        Self { lo, hi }
    }

    /// Reads a scalar's bracket into the ring through the **certified**
    /// door, poisoning a scalar that may not certify.
    ///
    /// This is the C9 ring's one body for a lane scalar taken whole.
    /// Every crossing S41 names reaches it: five call it directly, and
    /// the rest go through `geom-core`'s own `spline::hull::bracket` and
    /// `geom-brep`'s `ssi::enclose::ring`, which are one-line wrappers
    /// over it. (A crossing that reads only *one* end of the bracket,
    /// such as a symmetric pad built from `hi`, is a different operation
    /// and spells its own refusal.) It is a method rather than the
    /// three-line spelling at each call site
    /// for the same reason [`Self::clamped_to`] is: the hazard is the
    /// bracket door, which never refuses. `Interval` records a domain
    /// violation in its decoration, not its endpoints — `sqrt([−1, 4])`
    /// is `[0, 2]` at `Trv` — and the ring has two states and no
    /// decoration channel. Whatever is built from the crossing is a
    /// certificate, so a scalar that carries a sound bracket its
    /// computation is not entitled to must become poison here, where the
    /// decoration is still readable, rather than a plausible bound
    /// nothing downstream can question.
    ///
    /// [`Bounds::lo`](crate::Bounds::lo)/[`hi`](crate::Bounds::hi) into
    /// [`Self::from_bounds`] is the *driver's* spelling and stays
    /// available: reading a bracket is not certifying it.
    pub fn from_certified<T: CertifiedEnclosure>(x: T) -> Self {
        match x.certified_bracket() {
            Some((lo, hi)) => Self::from_bounds(lo, hi),
            None => Self::poison(),
        }
    }

    /// The smallest enclosure containing both arguments. Poison in
    /// either argument poisons the hull — a hull that quietly dropped a
    /// poisoned member would certify geometry it never bounded.
    pub fn hull(a: Self, b: Self) -> Self {
        if a.is_poison() || b.is_poison() {
            return Self::poison();
        }
        Self {
            lo: if a.lo < b.lo { a.lo } else { b.lo },
            hi: if a.hi > b.hi { a.hi } else { b.hi },
        }
    }

    /// The intersection with the window `[lo, hi]` — narrowing an
    /// enclosure by a fact known independently of the arithmetic that
    /// produced it (`cos` lies in `[−1, 1]` however the series was
    /// summed).
    ///
    /// **Poison first**, and that is the whole reason this is a method
    /// rather than the two-line spelling at each call site. The hazard is
    /// not about decorations and does not depend on the interval scalar
    /// being in the build: NaI and the empty enclosure already surface as
    /// NaN endpoints from storage, and `RingInterval` poison is NaN by
    /// construction, so the swallow below is reachable in a plain `f64`
    /// build too. `f64::max` and `f64::min` return the *non*-NaN operand,
    /// so
    /// `from_bounds(x.lo().max(lo), x.hi().min(hi))` resurrects a
    /// poisoned enclosure as the window itself — a plausible, sound-
    /// looking bracket with no argument behind it, which is the
    /// laundering D4 ¶2 exists to prevent. A poisoned enclosure, a NaN
    /// window, and a window disjoint from the enclosure all yield
    /// poison.
    pub fn clamped_to(self, lo: f64, hi: f64) -> Self {
        if self.is_poison() || lo.is_nan() || hi.is_nan() {
            return Self::poison();
        }
        Self::from_bounds(self.lo.max(lo), self.hi.min(hi))
    }

    /// The exact zero enclosure `[0, 0]`.
    pub fn zero() -> Self {
        Self { lo: 0.0, hi: 0.0 }
    }

    /// The exact one enclosure `[1, 1]`.
    pub fn one() -> Self {
        Self { lo: 1.0, hi: 1.0 }
    }

    /// Whether this is the exact zero enclosure `[0, 0]` (either signed
    /// zero on either end). Used by the ring's zero-annihilator rule:
    /// `0 · x = 0` and `0 / x = 0` are **algebraic identities over the
    /// reals**, not floating-point exactness claims, so the result is
    /// the exact `[0, 0]` with no outward padding.
    fn is_exact_zero(self) -> bool {
        self.lo == 0.0 && self.hi == 0.0
    }

    /// The lower end of the bracket (NaN if poisoned).
    pub fn lo(self) -> f64 {
        self.lo
    }

    /// The upper end of the bracket (NaN if poisoned).
    pub fn hi(self) -> f64 {
        self.hi
    }

    /// Whether this is the poison value. Both endpoints are NaN
    /// together — the type has no half-poisoned state — so testing
    /// either is equivalent.
    pub fn is_poison(self) -> bool {
        self.lo.is_nan() || self.hi.is_nan()
    }

    /// Whether `x` lies in the enclosure. False for poison and for NaN
    /// `x` (nothing is known to lie in an unknown bracket).
    pub fn contains(self, x: f64) -> bool {
        self.lo <= x && x <= self.hi
    }

    /// The bracket's width, rounded **up** (`NaN` for poison). An
    /// infinite side gives `+inf`.
    pub fn width(self) -> f64 {
        if self.is_poison() {
            return f64::NAN;
        }
        if self.lo == self.hi {
            return 0.0;
        }
        up1(self.hi - self.lo)
    }

    /// An upper bound on `|x|` over the enclosure, rounded **up**
    /// (`NaN` for poison). This is the scalar sup-norm reading:
    /// `max(|lo|, |hi|)` is exact under negation (sign-bit only), so no
    /// widening is needed.
    pub fn mag(self) -> f64 {
        if self.is_poison() {
            return f64::NAN;
        }
        let (a, b) = (self.lo.abs(), self.hi.abs());
        if a > b { a } else { b }
    }
}

/// The ring refuses exactly on poison. It has two states and no
/// decorations, so [`RingInterval::is_poison`] IS its domain-violation
/// channel: a poisoned ring stands for no real, and a pair of NaN
/// endpoints is not a bracket of anything. Every other ring is a sound
/// bracket read straight off storage.
///
/// The refusal is this door's own rather than a loan from
/// [`RingInterval::from_bounds`]. Handing back `Some((NaN, NaN))` and
/// leaving a downstream constructor to reject it is precisely what the
/// trait's method doc excludes: what a generic `T: CertifiedEnclosure`
/// consumer is promised is a bracket, not a NaN it has to re-check, and
/// no consumer is obliged to funnel the pair through a constructor that
/// happens to catch it.
impl crate::real::CertifiedEnclosure for RingInterval {
    fn certified_bracket(self) -> Option<(f64, f64)> {
        (!self.is_poison()).then_some((self.lo, self.hi))
    }
}

impl Enclosure for RingInterval {
    fn lo(self) -> f64 {
        self.lo
    }

    fn hi(self) -> f64 {
        self.hi
    }
}

/// Assembles a result from already-widened endpoints, poisoning on any
/// NaN (an indeterminate form such as `0 · inf` or `inf − inf` reached
/// an endpoint) or on an inverted bracket. Every op funnels through
/// here, so no operation can return a malformed value.
// `!(lo <= hi)` is the NaN-catching form; see `from_bounds`.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
fn finish(lo: f64, hi: f64) -> RingInterval {
    if !(lo <= hi) {
        return RingInterval::poison();
    }
    RingInterval { lo, hi }
}

/// The minimum/maximum of the four corner values, NaN-propagating (a
/// NaN corner is an indeterminate form and must reach [`finish`]).
fn corner_min_max(c: [f64; 4]) -> (f64, f64) {
    let (mut lo, mut hi) = (c[0], c[0]);
    // Fixed ascending order (D9). A NaN corner short-circuits: `<` and
    // `>` are both false for NaN, so it would otherwise be silently
    // dropped from the min/max instead of poisoning the result.
    for v in c {
        if v.is_nan() {
            return (f64::NAN, f64::NAN);
        }
        if v < lo {
            lo = v;
        }
        if v > hi {
            hi = v;
        }
    }
    (lo, hi)
}

impl RingInterval {
    /// The tight square, `x²` over the enclosure — **the operation to
    /// use instead of `x * x`** whenever the argument can straddle
    /// zero. `[-a, b]` squared is `[0, max(a², b²)]`, but the product
    /// of two *independent* enclosures `[-a, b] · [-a, b]` is
    /// `[-ab, …]`: a spurious negative lower bound that poisons a
    /// downstream `sqrt`.
    ///
    /// The zero lower bound of a straddling square is exact — a fact
    /// about the reals — so it is not widened.
    pub fn sqr(self) -> Self {
        if self.is_poison() {
            return Self::poison();
        }
        if self.is_exact_zero() {
            return Self::zero();
        }
        if self.lo >= 0.0 {
            finish(down1(self.lo * self.lo).max(0.0), up1(self.hi * self.hi))
        } else if self.hi <= 0.0 {
            finish(down1(self.hi * self.hi).max(0.0), up1(self.lo * self.lo))
        } else {
            let (a, b) = (self.lo * self.lo, self.hi * self.hi);
            finish(0.0, up1(if a > b { a } else { b }))
        }
    }

    /// Integer powers by repeated squaring, the ring's only non-`±×÷`
    /// operation.
    ///
    /// `n == 0` is `[1, 1]` for every non-poisoned argument and poison
    /// for poison (the `Real::powi` poison-guard convention: `NaN⁰` is
    /// not 1). Negative `n` is the reciprocal of the positive power, so
    /// a zero-straddling base yields poison through [`core::ops::Div`].
    ///
    /// The squaring step is [`sqr`](Self::sqr), so **every even power
    /// of a zero-straddling enclosure keeps the exact lower bound 0**:
    /// the accumulator is only ever squared, and for even `n` every
    /// factor entering the product is itself a square (sign-clamped
    /// multiplication preserves nonnegativity).
    ///
    /// Association is fixed (D9): ascending bit order, accumulator
    /// squared after each bit, the first set bit seeding the result
    /// directly rather than multiplying by `[1, 1]` (which would widen
    /// an exact 0 lower bound back below zero).
    pub fn powi(self, n: i32) -> Self {
        if self.is_poison() {
            return Self::poison();
        }
        if n == 0 {
            return Self::one();
        }
        let mut result = Self::one();
        let mut seeded = false;
        let mut acc = self;
        // unsigned_abs handles n == i32::MIN without overflow.
        let mut e = n.unsigned_abs();
        while e > 0 {
            if e & 1 == 1 {
                result = if seeded { result * acc } else { acc };
                seeded = true;
            }
            e >>= 1;
            if e > 0 {
                acc = acc.sqr();
            }
        }
        if n < 0 { Self::one() / result } else { result }
    }
}

impl core::ops::Neg for RingInterval {
    type Output = Self;

    /// Exact — negation only flips the sign bit, so no widening.
    fn neg(self) -> Self {
        if self.is_poison() {
            return Self::poison();
        }
        finish(-self.hi, -self.lo)
    }
}

impl core::ops::Add for RingInterval {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        if self.is_poison() || rhs.is_poison() {
            return Self::poison();
        }
        finish(down1(self.lo + rhs.lo), up1(self.hi + rhs.hi))
    }
}

impl core::ops::Sub for RingInterval {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        if self.is_poison() || rhs.is_poison() {
            return Self::poison();
        }
        finish(down1(self.lo - rhs.hi), up1(self.hi - rhs.lo))
    }
}

/// Raises/lowers widened endpoints back to `0` when the sign rule
/// `sign(a ∘ b) = sign(a) · sign(b)` (`∘ ∈ {×, ÷}`) proves the true
/// result cannot cross zero.
///
/// This is **not** an exactness witness: it makes no claim about
/// floating-point rounding, only about the reals — the product of two
/// nonnegative quantities is nonnegative, so an outward step that
/// pushed the lower bound to `-5e-324` may be pulled back to `0`
/// without losing containment. (The `[0, 0]` factor itself never reaches
/// here — `Mul`/`Div` answer it with the zero annihilator first.)
fn sign_clamp(a: RingInterval, b: RingInterval, lo: f64, hi: f64) -> (f64, f64) {
    let (mut lo, mut hi) = (lo, hi);
    let nonneg = (a.lo() >= 0.0 && b.lo() >= 0.0) || (a.hi() <= 0.0 && b.hi() <= 0.0);
    let nonpos = (a.lo() >= 0.0 && b.hi() <= 0.0) || (a.hi() <= 0.0 && b.lo() >= 0.0);
    if nonneg && lo < 0.0 {
        lo = 0.0;
    }
    if nonpos && hi > 0.0 {
        hi = 0.0;
    }
    (lo, hi)
}

impl core::ops::Mul for RingInterval {
    type Output = Self;

    /// Four-corner min/max, widened outward, then sign-clamped. An
    /// indeterminate corner (`0 · ±inf`, only reachable from an
    /// infinite endpoint) poisons the result.
    ///
    /// **Not** the operation for squares: `x * x` treats the factors as
    /// independent (see [`sqr`](Self::sqr)).
    fn mul(self, rhs: Self) -> Self {
        if self.is_poison() || rhs.is_poison() {
            return Self::poison();
        }
        if self.is_exact_zero() || rhs.is_exact_zero() {
            return Self::zero();
        }
        // Fixed corner order (D9).
        let (lo, hi) = corner_min_max([
            self.lo * rhs.lo,
            self.lo * rhs.hi,
            self.hi * rhs.lo,
            self.hi * rhs.hi,
        ]);
        let (lo, hi) = sign_clamp(self, rhs, down1(lo), up1(hi));
        finish(lo, hi)
    }
}

impl core::ops::Div for RingInterval {
    type Output = Self;

    /// Four-corner min/max of `a / b`, widened outward, then
    /// sign-clamped.
    ///
    /// **A divisor that straddles or touches zero is poison.** The
    /// half-line/two-component semantics of IEEE 1788 extended division
    /// buy certification nothing (a residual bound that runs to
    /// infinity certifies nothing either), and refusing loudly is the
    /// honest conservative answer: the caller learns its denominator
    /// was not proven away from zero instead of receiving an unbounded
    /// bracket that silently passes through later arithmetic.
    fn div(self, rhs: Self) -> Self {
        if self.is_poison() || rhs.is_poison() {
            return Self::poison();
        }
        // Divisor must be proven strictly one side of zero.
        if !(rhs.lo > 0.0 || rhs.hi < 0.0) {
            return Self::poison();
        }
        if self.is_exact_zero() {
            return Self::zero();
        }
        // Fixed corner order (D9).
        let (lo, hi) = corner_min_max([
            self.lo / rhs.lo,
            self.lo / rhs.hi,
            self.hi / rhs.lo,
            self.hi / rhs.hi,
        ]);
        let (lo, hi) = sign_clamp(self, rhs, down1(lo), up1(hi));
        finish(lo, hi)
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
