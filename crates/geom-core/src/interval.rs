//! The [`Interval`] scalar over [inari] — Q1's certified instantiation of
//! [`Real`] and [`Decide`] (M0 PR 4, behind the `interval` cargo feature).
//!
//! An `Interval` is a machine-representable enclosure `[lo, hi]` of the
//! **true real value** of a computation: every operation returns an
//! interval guaranteed (IEEE 1788, MPFR-backed correct outward rounding)
//! to contain the exact result over all inputs in the operand enclosures.
//! Predicates decided at this type are therefore *certified*: a definite
//! [`Sign`] holds for the true value, not merely for one f64 evaluation
//! of it. An indeterminate outcome unwinds to Q1's subdivision driver,
//! which splits the parameter box and re-runs.
//!
//! # The decoration is the poison channel
//!
//! The newtype wraps [`inari::DecInterval`] — the *decorated* interval —
//! and not the bare [`inari::Interval`], deliberately. inari **clamps**
//! partially-out-of-domain inputs to the domain (`sqrt([-1, 4]) = [0, 2]`;
//! only a *fully* out-of-domain input goes empty) and records the
//! violation solely in the IEEE 1788 decoration, which degrades
//! monotonically through every subsequent operation. A bare-interval
//! wrapper would launder domain violations into plausible enclosures;
//! the decorated one carries the poison to the decision point, where
//! [`Decide::sign_within`] refuses to branch on any decoration below
//! [`Decoration::Def`] ("defined on the whole input box"): `Trv` means
//! "the domain was violated somewhere upstream" — exactly what must never
//! decide topology. Poison thus flows through *values* (clamped
//! enclosures keep computing) but never through *decisions*, the same
//! policy [`crate::real`] states for NaN at `f64`.
//!
//! # Certification semantics: truth containment, not f64 containment
//!
//! An enclosure brackets the TRUE value. A libm-computed `f64` of the same
//! expression can land *outside* a tight enclosure of a transcendental
//! result (libm makes no correct-rounding guarantee — its divergence from
//! std reaches 4 ulps in the census; no faithful-rounding violation was
//! found in 5.6M samples, but none is promised — while the enclosure's
//! endpoints are correctly rounded). Certification code therefore bounds *residual quantities*
//! evaluated at interval type and never asserts "f64 value ∈ enclosure"
//! for transcendental results; the same rule binds this module's tests.
//! Exact single operations (`+`, `−`, `·`, `/`, `sqrt`) are correctly
//! rounded at `f64`, so their f64 results *are* contained — those may be
//! asserted.
//!
//! # Determinism and the CPU floor
//!
//! inari's enclosures are bit-identical across CPUs and SIMD paths at
//! pinned dependency versions (probe findings, issue #4) — D9-compliant.
//! Its directed-rounding inline assembly requires AVX+FMA on x86-64; the
//! repo-wide `-C target-cpu=x86-64-v3` floor in `.cargo/config.toml`
//! guarantees it (aarch64 needs no flags). inari's own `unsafe`/asm is a
//! vetted-dependency exception under L4; this crate's `forbid(unsafe_code)`
//! is untouched.
//!
//! A `.cargo/config.toml` applies only to builds *rooted* in this repo, so
//! external crates enabling the `interval` feature do **not** inherit the
//! floor: each downstream consumer must replicate the
//! `-C target-cpu=x86-64-v3` rustflag in its own `.cargo/config.toml` (or
//! an equivalent `RUSTFLAGS`) to build the interval path on x86-64.
//!
//! # Non-real inputs
//!
//! `f64`'s NaN and ±∞ are not real numbers and have no enclosure;
//! [`Real::from_f64`] maps them to the ill-formed interval (NaI) — poison
//! from the moment of construction, never a silent clamp.

use core::ops::{Add, Div, Mul, Neg, Sub};

use inari::{DecInterval, Decoration};

use crate::dual::KinkJacobian;
use crate::predicate::{Band, Decide, Indeterminate, MarginDiag, Sign};
use crate::real::{Bounds, Real};

/// An enclosure of a true real value: the interval scalar over
/// [`inari::DecInterval`] (see the [module docs](self) for the poison and
/// certification semantics).
///
/// Deliberately **no** `PartialEq`: IEEE 1788's NaI is not equal to itself
/// (like NaN), and comparing enclosures for equality is not a meaningful
/// geometric question — classification goes through [`Decide`], bound
/// extraction through [`Bounds`].
#[derive(Debug, Clone, Copy)]
pub struct Interval(DecInterval);

impl Interval {
    /// Constructs the enclosure `[lo, hi]` from explicit bounds — the
    /// **subdivision driver's** constructor (the door *into* interval
    /// values, as [`Bounds`] is the door out), used to materialize
    /// parameter sub-boxes for interval replay. The same style-rule scope
    /// as [`Bounds`] applies (L7): driver/certification code only, never
    /// evaluation signatures — evaluation code receives already-built
    /// scalars generically.
    ///
    /// Total: an invalid pair (any NaN, `lo > hi`, or an infinite endpoint
    /// where IEEE 1788 admits none, e.g. `[+∞, +∞]`) yields NaI — poison,
    /// per the module-level policy, not an error value to branch around.
    /// Half-unbounded enclosures (`[0, +∞]`) are valid and carry
    /// decoration [`Decoration::Dac`] (`Com` requires boundedness).
    ///
    /// **Laundering warning.** A [`Bounds`]-extract → `from_bounds`-rebuild
    /// round trip mid-pipeline discards the decoration history: a `Trv`
    /// (domain-violated) enclosure re-enters as a fresh `Com`, scrubbed of
    /// its poison — the one genuine laundering door around the decoration
    /// channel. `from_bounds` exists to materialize the driver's parameter
    /// sub-boxes; it is never for reconstructing values that came out of
    /// [`Bounds`] mid-computation.
    pub fn from_bounds(lo: f64, hi: f64) -> Self {
        match DecInterval::try_from((lo, hi)) {
            Ok(x) => Self(x),
            Err(_) => Self(DecInterval::NAI),
        }
    }
}

impl Add for Interval {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Interval {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl Mul for Interval {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }
}

impl Div for Interval {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self(self.0 / rhs.0)
    }
}

impl Neg for Interval {
    type Output = Self;

    fn neg(self) -> Self {
        Self(-self.0)
    }
}

/// [`Real`] over enclosures: every operation is IEEE 1788-tightest via
/// inari (MPFR-backed, correctly rounded outward for the transcendentals),
/// total (out-of-domain input clamps with a degraded decoration or goes
/// empty — the poison channel, see the [module docs](self)), and
/// deterministic per D9 (bit-identical enclosures across platforms at
/// pinned dependency versions).
impl Real for Interval {
    /// The point enclosure `[x, x]` with decoration `Com` — an exact
    /// embedding for every *finite* `f64`. NaN and ±∞ are not real
    /// numbers and have no enclosure: they map to NaI, explicitly —
    /// poison at construction rather than a silent clamp (the interval
    /// counterpart of `f64`'s NaN-in, NaN-through).
    fn from_f64(x: f64) -> Self {
        match DecInterval::try_from((x, x)) {
            Ok(p) => Self(p),
            Err(_) => Self(DecInterval::NAI),
        }
    }

    /// The exact point `[0, 0]`.
    fn zero() -> Self {
        Self::from_f64(0.0)
    }

    /// The exact point `[1, 1]`.
    fn one() -> Self {
        Self::from_f64(1.0)
    }

    /// inari's 1-ulp enclosure of π (the two adjacent `f64`s bracketing
    /// the true π), decoration `Com`.
    fn pi() -> Self {
        Self(DecInterval::PI)
    }

    /// inari's 1-ulp enclosure of τ = 2π, decoration `Com`.
    fn tau() -> Self {
        Self(DecInterval::TAU)
    }

    /// Tight enclosure of the square root over `self ∩ [0, ∞)`. A
    /// partially negative enclosure **clamps** and degrades the
    /// decoration to `Trv` (poisoning any decision downstream); a fully
    /// negative one is empty.
    fn sqrt(self) -> Self {
        Self(self.0.sqrt())
    }

    /// Tight enclosure of the absolute value.
    fn abs(self) -> Self {
        Self(self.0.abs())
    }

    /// **Overrides** the defaulted squaring algorithm with inari's
    /// dedicated integer power, per the override clause in
    /// [`Real::powi`]'s docs: the interval contract is a **tight
    /// enclosure of the true power**, and repeated interval
    /// multiplication is not tight for enclosures straddling zero
    /// (`[-1, 2]·[-1, 2] = [-2, 4]`, but `x² ∈ [0, 4]` — the dependency
    /// problem). Deterministic either way (D9).
    ///
    /// Poison is never laundered: NaI and the empty enclosure propagate
    /// through **every** exponent, *including `n == 0`* — in agreement
    /// with `f64`, whose `powi` guards `n == 0` to propagate NaN (the
    /// trait's poison-propagation clause; both scalars keep the poison
    /// flowing). The one remaining `n == 0` divergence is ±∞, and it is
    /// not powi's: `f64` keeps `(±∞)⁰ = 1` because infinity is not f64
    /// poison, while here `from_f64(±∞)` is already NaI — the ratified
    /// non-real mapping (module docs), decided at construction.
    fn powi(self, n: i32) -> Self {
        Self(self.0.powi(n))
    }

    /// The pair `(sin(self), cos(self))`, each component a correctly
    /// rounded tight enclosure (MPFR). No paired/fused override could do
    /// better: at interval type the two components are independent
    /// enclosures of the same input interval either way, and their mutual
    /// consistency is automatic — both bracket the truth over exactly
    /// `self` — so the pair *is* the two projections, by construction
    /// (the trait's bit-identity contract for the defaulted `sin`/`cos`
    /// projections holds trivially).
    fn sin_cos(self) -> (Self, Self) {
        (Self(self.0.sin()), Self(self.0.cos()))
    }

    /// Tight enclosure of the tangent; an enclosure containing a pole has
    /// unbounded range and a degraded decoration.
    fn tan(self) -> Self {
        Self(self.0.tan())
    }

    /// Tight enclosure of the arcsine over `self ∩ [-1, 1]`; clamping
    /// degrades the decoration (poison), a fully out-of-domain enclosure
    /// is empty.
    fn asin(self) -> Self {
        Self(self.0.asin())
    }

    /// Tight enclosure of the arccosine over `self ∩ [-1, 1]`; same
    /// clamp/poison behavior as [`Real::asin`].
    fn acos(self) -> Self {
        Self(self.0.acos())
    }

    /// Tight enclosure of the arctangent (total on ℝ).
    fn atan(self) -> Self {
        Self(self.0.atan())
    }

    /// Tight enclosure of the four-quadrant arctangent of `self` (= y)
    /// and `x`. An input box containing the origin (or straddling the
    /// branch cut on the negative x-axis) widens the range and degrades
    /// the decoration to `Trv` — the discontinuity is a domain-of-
    /// definition violation for a *continuous* branch, and the poison
    /// channel reports it.
    fn atan2(self, x: Self) -> Self {
        Self(self.0.atan2(x.0))
    }

    /// The interval **envelope** minimum `[min(a.lo, b.lo),
    /// min(a.hi, b.hi)]` — the tight enclosure of `min(x, y)` over the
    /// operand enclosures, the lattice-operation reading of [`Real::min`]
    /// generalized to sets. Empty and NaI inputs propagate, satisfying
    /// the trait contract's poison-propagation intent (there is no
    /// "tie-keeps-self" at interval type: ties are a point notion, and
    /// the envelope contains both choices).
    fn min(self, other: Self) -> Self {
        Self(self.0.min(other.0))
    }

    /// The interval envelope maximum — same contract as [`Real::min`]'s
    /// override above, mirrored.
    fn max(self, other: Self) -> Self {
        Self(self.0.max(other.0))
    }
}

/// Bound extraction (certification/driver scope — see [`Bounds`]):
/// the enclosure's exact endpoints. Poison surfaces honestly: NaI **and**
/// the empty enclosure both yield NaN from both accessors, so either
/// bracket fails every downstream `residual ≤ ε` certification loudly
/// (D4 ¶2) — `NaN ≤ ε` is false under every comparison direction.
///
/// The empty enclosure deliberately does NOT surface as IEEE 1788's
/// canonical reversed pair (+∞, −∞): that pair's *upper* accessor is −∞,
/// which would PASS a `residual.hi() ≤ ε` certification check for a
/// residual that was poisoned to empty — the exact laundering D4 ¶2
/// exists to prevent. Empty and NaI are therefore indistinguishable
/// through `Bounds`, on purpose: failing certification outranks
/// IEEE 1788 representational honesty. (Code that needs to tell them
/// apart is driver/diagnostic code, which sees the decoration through
/// [`Decide::sign_within`]'s [`MarginDiag::Invalid`], not through this
/// trait.)
impl Bounds for Interval {
    fn lo(self) -> f64 {
        if self.0.is_empty() {
            f64::NAN
        } else {
            self.0.inf()
        }
    }

    fn hi(self) -> f64 {
        if self.0.is_empty() {
            f64::NAN
        } else {
            self.0.sup()
        }
    }
}

/// Certified sign classification of an enclosure `[lo, hi]` against a
/// [`Band`] — the interval instantiation of the single door from numbers
/// to decisions.
///
/// **Poison first**: a decoration below [`Decoration::Def`] refuses to
/// classify at all, yielding [`MarginDiag::Invalid`]. This covers NaI
/// (`Ill`), the empty enclosure (`Trv`), and — the crucial case — a
/// *plausible-looking* enclosure whose computation violated a domain
/// somewhere (`Trv` via clamping, e.g. `sqrt([-1, 4]) = [0, 2]`). The
/// threshold is `Def` by design: `Def` and above certify the expression
/// was defined on the entire input box, which is exactly what a sound
/// branch needs; continuity (`Dac`) and boundedness (`Com`) are not
/// required for a sign decision.
///
/// For a healthy enclosure the classification is set inclusion against
/// the same closed regions the `f64` classifier uses:
///
/// - `[lo, hi] ⊆ [−zero, zero]` — every enclosed value is coincident —
///   [`Sign::Zero`];
/// - `lo ≥ escalate` — every enclosed value has full clearance —
///   [`Sign::Positive`]; mirrored for [`Sign::Negative`];
/// - anything else is [`Indeterminate`] with
///   [`MarginDiag::Enclosure`] carrying the exact bounds.
///
/// Consequences, both deliberate: an enclosure *straddling* a region
/// boundary is indeterminate (subdivide and re-run — Q1's driver), and a
/// **point** enclosure inside the open ambiguity band `(zero, escalate)`
/// is indeterminate *even though it is exact* — the sliver band is a
/// design statement about sound geometry, not a noise model (the ratified
/// reading in `crate::predicate`'s module docs), so certifying the point
/// changes nothing.
///
/// **Driver termination rule.** The second consequence generalizes: an
/// [`MarginDiag::Enclosure`] lying **wholly inside one open sliver band**
/// `(zero, escalate)` — or its mirror `(-escalate, -zero)` — is
/// **terminal**. No subdivision refines it: the band is semantically
/// indeterminate at *any* width, down to a point, so the driver escalates
/// it as a genuine D4 ¶3 sliver, never retries it as a resolution
/// failure. [`MarginDiag::Invalid`] outcomes split for the driver too: a
/// domain-clamp `Invalid` (`Trv` from partial clamping) may cure under
/// subdivision — the violating sub-box shrinks away — while a NaI
/// `Invalid` never cures.
impl Decide for Interval {
    fn sign_within(self, band: Band) -> Result<Sign, Indeterminate> {
        if self.0.decoration() < Decoration::Def {
            return Err(Indeterminate {
                margin: MarginDiag::Invalid,
                band,
                predicate: None,
            });
        }
        let (lo, hi) = (self.0.inf(), self.0.sup());
        if -band.zero() <= lo && hi <= band.zero() {
            Ok(Sign::Zero)
        } else if lo >= band.escalate() {
            Ok(Sign::Positive)
        } else if hi <= -band.escalate() {
            Ok(Sign::Negative)
        } else {
            Err(Indeterminate {
                margin: MarginDiag::Enclosure { lo, hi },
                band,
                predicate: None,
            })
        }
    }
}

/// Replaces `x`'s decoration with `min(x.decoration(), floor)` — the
/// derivative channel's honesty floor for the kink selectors below: a
/// tangent selected or hulled by comparing *values* is only as
/// trustworthy as those values, so their decoration caps it. Poison
/// shapes pass through unchanged (NaI stays NaI; an empty interval can
/// only carry `Trv`/`Ill`, which `set_dec` preserves).
fn cap_decoration(x: DecInterval, floor: Decoration) -> DecInterval {
    match x.interval() {
        Some(bare) => DecInterval::set_dec(bare, x.decoration().min(floor)),
        None => DecInterval::NAI,
    }
}

/// The convex hull of two decorated tangents, decorated with the *minimum*
/// of their decorations — deliberately NOT IEEE 1788's set-operation
/// convention (which would drop to `Trv` unconditionally): the hull here
/// is not a set operation on unrelated intervals but the subgradient
/// convention for a tie region (see [`KinkJacobian`]), and each branch's
/// tangent keeps its own computation history. Poison first: either
/// operand NaI ⇒ NaI, either empty ⇒ empty (1788's "empty absorbs into
/// the hull" would *drop* a poisoned tangent — the opposite of poison
/// propagation).
fn tangent_hull(x: DecInterval, y: DecInterval) -> DecInterval {
    if x.is_nai() || y.is_nai() {
        return DecInterval::NAI;
    }
    if x.is_empty() || y.is_empty() {
        return DecInterval::EMPTY;
    }
    match (x.interval(), y.interval()) {
        (Some(bare_x), Some(bare_y)) => DecInterval::set_dec(
            bare_x.convex_hull(bare_y),
            x.decoration().min(y.decoration()),
        ),
        // Unreachable (NaI was handled above), but total per D9 — poison,
        // never a panic.
        _ => DecInterval::NAI,
    }
}

/// Interval kink selectors (see [`KinkJacobian`] in `crate::dual` for the
/// contract, and `crate::dual`'s module docs for the ratified
/// conventions). All three are *value* computations — honest enclosures
/// of subgradients — never decisions: no `Indeterminate` is produced
/// here, and raw endpoint comparisons are allowed because this is scalar-
/// implementation code (Q1's allowance, same as [`Real::min`] at `f64`).
///
/// **Decoration convention (pinned under test, revisitable at M6):** each
/// selector's output decoration is capped by the decoration of the
/// *value(s)* that steered it — a sign factor read off a domain-violated
/// (`Trv`) value is itself only `Trv`-trustworthy, and a tangent chosen
/// by comparing `Trv` values likewise. In the clean (`Com`) case this is
/// a no-op. Nothing in M0 branches on derivative decorations ([`Decide`]
/// for duals classifies values only; `Bounds` is not implemented for
/// duals), so this convention is about honest bookkeeping, not behavior.
impl KinkJacobian for Interval {
    /// `[1, 1]` when the value enclosure lies in `[0, ∞)` (including the
    /// point zero — matching `f64`'s `+1`-at-zero subgradient pick; an
    /// endpoint `-0.0` compares `≥ 0`), `[−1, −1]` in `(−∞, 0]`, and the
    /// honest straddle hull `[−1, 1]` otherwise. Empty/NaI propagates.
    /// Decoration: the value's own (the factor is exact; its
    /// trustworthiness is exactly the value's).
    fn abs_sign_factor(self) -> Self {
        if self.0.is_nai() {
            return Self(DecInterval::NAI);
        }
        if self.0.is_empty() {
            return Self(DecInterval::EMPTY);
        }
        let factor = if self.0.inf() >= 0.0 {
            inari::const_interval!(1.0, 1.0)
        } else if self.0.sup() <= 0.0 {
            inari::const_interval!(-1.0, -1.0)
        } else {
            inari::const_interval!(-1.0, 1.0)
        };
        Self(DecInterval::set_dec(factor, self.0.decoration()))
    }

    /// Strict separation selects (`self` certainly below keeps its own
    /// tangent, `other` certainly below takes the other's); any overlap —
    /// including a single shared endpoint, and in particular equal point
    /// values with different tangents — hulls both tangents
    /// ([`tangent_hull`]). Strictness is deliberate: with touching
    /// enclosures the minimum can sit *at* the tie, where the true
    /// one-sided derivatives are both branches' — only strict separation
    /// certifies a single branch. Empty/NaI **values** poison the tangent
    /// (both channels of a poisoned `min` are poison, mirroring `f64`'s
    /// NaN guard); the result's decoration is capped by the deciding
    /// values' ([`cap_decoration`]).
    fn min_deriv(self, self_deriv: Self, other: Self, other_deriv: Self) -> Self {
        if self.0.is_nai() || other.0.is_nai() {
            return Self(DecInterval::NAI);
        }
        if self.0.is_empty() || other.0.is_empty() {
            return Self(DecInterval::EMPTY);
        }
        let chosen = if self.0.sup() < other.0.inf() {
            self_deriv.0
        } else if other.0.sup() < self.0.inf() {
            other_deriv.0
        } else {
            tangent_hull(self_deriv.0, other_deriv.0)
        };
        Self(cap_decoration(
            chosen,
            self.0.decoration().min(other.0.decoration()),
        ))
    }

    /// [`KinkJacobian::min_deriv`] mirrored: strict separation selects the
    /// certainly-greater argument's tangent, any overlap hulls. Same
    /// poison and decoration conventions.
    fn max_deriv(self, self_deriv: Self, other: Self, other_deriv: Self) -> Self {
        if self.0.is_nai() || other.0.is_nai() {
            return Self(DecInterval::NAI);
        }
        if self.0.is_empty() || other.0.is_empty() {
            return Self(DecInterval::EMPTY);
        }
        let chosen = if self.0.inf() > other.0.sup() {
            self_deriv.0
        } else if other.0.inf() > self.0.sup() {
            other_deriv.0
        } else {
            tangent_hull(self_deriv.0, other_deriv.0)
        };
        Self(cap_decoration(
            chosen,
            self.0.decoration().min(other.0.decoration()),
        ))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use proptest::prelude::*;

    use super::*;
    use crate::real::powi_by_squaring;

    // Global-state discipline (see `crate::tolerance`'s test module): all
    // bands here are built purely via `Band::new`; the tolerance-coupled
    // path at interval type lives in its own integration binary
    // (`tests/interval_band.rs`), i.e. its own process.
    //
    // Truth-containment discipline (module docs): assertions about
    // transcendental results only ever check containment of *known true
    // values* (π/6's sine is exactly 1/2) or of identities (sin² + cos² =
    // 1) — never containment of libm-computed f64 results, which can
    // legitimately fall outside a tight enclosure. Exact single ops
    // (+, −, ·, /, sqrt) are correctly rounded at f64, so their f64
    // results ARE asserted contained.

    /// Shorthand: the enclosure [lo, hi].
    fn iv(lo: f64, hi: f64) -> Interval {
        Interval::from_bounds(lo, hi)
    }

    /// A named unary trait operation (for the monotonicity tables).
    type UnaryOp = fn(Interval) -> Interval;

    /// A named binary trait operation (for the monotonicity tables).
    type BinaryOp = fn(Interval, Interval) -> Interval;

    /// The fixed pure band (zero = 1e-9, escalate = 1e-8) used by the
    /// decision tables — identical to `predicate.rs`'s `band_1e9`.
    fn band_1e9() -> Band {
        Band::new(1e-9, 1e-8).unwrap()
    }

    /// Set inclusion on the underlying intervals (empty ⊆ anything;
    /// false if either side is NaI — the proptests never build NaI).
    fn subset(a: Interval, b: Interval) -> bool {
        a.0.subset(b.0)
    }

    /// The same generic consumer `real.rs`'s tests use — evaluation code
    /// written against `Real` alone, here instantiated at `Interval`.
    fn hypotenuse<T: Real>(a: T, b: T) -> T {
        (a * a + b * b).sqrt()
    }

    #[test]
    fn from_f64_embeds_finite_points_exactly() {
        for x in [1.0, -2.5, 1e-308, -1e300, f64::MAX] {
            let p = Interval::from_f64(x);
            assert_eq!(p.lo().to_bits(), x.to_bits(), "lo of point {x:e}");
            assert_eq!(p.hi().to_bits(), x.to_bits(), "hi of point {x:e}");
            assert_eq!(p.0.decoration(), Decoration::Com);
        }
        // Zero embeds exactly as a VALUE, but not bit-for-bit: inari
        // canonicalizes endpoint representation (it stores the lower bound
        // negated, so `inf()` of the point zero surfaces as -0.0). The
        // sign of a floating-point zero is a representation artifact, not
        // geometry (same stance as `crate::predicate`'s boundary table),
        // so value equality is the honest assertion here.
        for z in [0.0f64, -0.0] {
            let p = Interval::from_f64(z);
            assert_eq!(p.lo(), 0.0, "lo of point {z:?}");
            assert_eq!(p.hi(), 0.0, "hi of point {z:?}");
            assert_eq!(p.0.decoration(), Decoration::Com);
        }
    }

    #[test]
    fn from_f64_poisons_non_reals() {
        // NaN and ±∞ are not real numbers: NaI explicitly, not a clamp.
        for x in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(Interval::from_f64(x).0.is_nai(), "from_f64({x:e})");
        }
    }

    #[test]
    fn from_bounds_constructs_and_poisons() {
        let x = iv(-1.0, 2.0);
        assert_eq!(x.lo(), -1.0);
        assert_eq!(x.hi(), 2.0);
        assert_eq!(x.0.decoration(), Decoration::Com);

        // Half-unbounded is valid but only Dac (Com requires bounded).
        let half = iv(0.0, f64::INFINITY);
        assert_eq!(half.0.decoration(), Decoration::Dac);
        assert_eq!(half.lo(), 0.0);
        assert_eq!(half.hi(), f64::INFINITY);

        // Invalid pairs are NaI (total, poison — never a panic).
        assert!(iv(2.0, 1.0).0.is_nai(), "reversed bounds");
        assert!(iv(f64::NAN, 1.0).0.is_nai(), "NaN bound");
        assert!(iv(1.0, f64::NAN).0.is_nai(), "NaN bound");
        assert!(
            iv(f64::INFINITY, f64::INFINITY).0.is_nai(),
            "[+inf, +inf] encloses no real"
        );
    }

    /// The `Bounds` poison contract: empty and NaI both surface as NaN
    /// brackets — deliberately indistinguishable (doc on the impl) — and
    /// neither can ever pass a D4 ¶2 `residual.hi() ≤ ε` certification.
    #[test]
    fn bounds_poison_is_nan_and_never_certifies() {
        let empty = iv(-4.0, -1.0).sqrt();
        assert!(empty.0.is_empty());
        let nai = Interval::from_f64(f64::NAN);
        assert!(nai.0.is_nai());

        let eps = 1e-9;
        for (x, what) in [(empty, "empty"), (nai, "NaI")] {
            assert!(x.lo().is_nan(), "{what}: lo must be NaN");
            assert!(x.hi().is_nan(), "{what}: hi must be NaN");
            // The never-certifies pin: NOT the ordinary `hi > eps` check —
            // the point is that the certification comparison itself comes
            // back false, so `residual.hi() <= eps` can never pass.
            let certifies = x.hi() <= eps;
            assert!(!certifies, "{what}: a poisoned bracket must never certify");
            let certifies_lo = x.lo() <= eps;
            assert!(!certifies_lo, "{what}: nor through the lower bound");
        }
    }

    #[test]
    fn constants_are_exact_points_and_one_ulp_enclosures() {
        // Value equality for zero (the sign of a zero endpoint is a
        // representation artifact — see `from_f64_embeds_finite_points_
        // exactly`); bit equality for one.
        let zero = Interval::zero();
        assert_eq!(zero.lo(), 0.0);
        assert_eq!(zero.hi(), 0.0);
        assert_eq!(zero.0.decoration(), Decoration::Com);
        let one = Interval::one();
        assert_eq!(one.lo().to_bits(), 1.0f64.to_bits());
        assert_eq!(one.hi().to_bits(), 1.0f64.to_bits());
        assert_eq!(one.0.decoration(), Decoration::Com);

        // fl(π) rounds DOWN (π = 3.14159265358979323846… > fl(π) =
        // 3.141592653589793), so the unique 1-ulp enclosure of the true π
        // is exactly [fl(π), next_up(fl(π))] — same for τ, whose rounding
        // direction matches π's because fl(2x) = 2·fl(x) exactly.
        let pi = Interval::pi();
        assert_eq!(pi.lo(), core::f64::consts::PI);
        assert_eq!(pi.hi(), core::f64::consts::PI.next_up());
        let tau = Interval::tau();
        assert_eq!(tau.lo(), core::f64::consts::TAU);
        assert_eq!(tau.hi(), core::f64::consts::TAU.next_up());
    }

    #[test]
    fn exact_ops_have_exact_or_containing_enclosures() {
        // sqrt(4) = 2 exactly: the enclosure is the exact point.
        let two = Interval::from_f64(4.0).sqrt();
        assert_eq!(two.lo(), 2.0);
        assert_eq!(two.hi(), 2.0);

        // 0.1 + 0.2: a single correctly rounded op — the f64 result is
        // contained (allowed for exact ops, module docs) and the
        // enclosure is at most 1 ulp wide.
        let s = Interval::from_f64(0.1) + Interval::from_f64(0.2);
        assert!(s.0.contains(0.1 + 0.2));
        assert!(s.hi() <= s.lo().next_up());
    }

    #[test]
    fn sqrt_domain_table() {
        // Fully in-domain (0 included): tight, decoration intact.
        let x = iv(0.0, 4.0).sqrt();
        assert_eq!((x.lo(), x.hi()), (0.0, 2.0));
        assert_eq!(x.0.decoration(), Decoration::Com);

        // Partially out of domain: CLAMPS to [0, 2] — a plausible
        // enclosure — and records the violation only in the decoration.
        let clamped = iv(-1.0, 4.0).sqrt();
        assert_eq!((clamped.lo(), clamped.hi()), (0.0, 2.0));
        assert_eq!(clamped.0.decoration(), Decoration::Trv);

        // Fully out of domain: empty.
        let empty = iv(-4.0, -1.0).sqrt();
        assert!(empty.0.is_empty());
        assert_eq!(empty.0.decoration(), Decoration::Trv);

        // NaI propagates.
        assert!(Interval::from_f64(f64::NAN).sqrt().0.is_nai());
    }

    #[test]
    fn powi_is_tight_across_zero() {
        // The headline case: [-1, 2]² must be [0, 4] (the true range),
        // not repeated multiplication's [-2, 4].
        let x = iv(-1.0, 2.0);
        let tight = x.powi(2);
        assert_eq!((tight.lo(), tight.hi()), (0.0, 4.0));

        // What the defaulted squaring algorithm would have produced —
        // still a *valid* enclosure (the override refines, never
        // contradicts), just not tight.
        let squared = powi_by_squaring(x, 2);
        assert_eq!((squared.lo(), squared.hi()), (-2.0, 4.0));
        assert!(subset(tight, squared));

        // Odd powers keep sign structure: [-1, 2]³ = [-1, 8].
        let cubed = x.powi(3);
        assert_eq!((cubed.lo(), cubed.hi()), (-1.0, 8.0));

        // Negative exponents: 2⁻² = 1/4 exactly.
        let quarter = Interval::from_f64(2.0).powi(-2);
        assert_eq!((quarter.lo(), quarter.hi()), (0.25, 0.25));
    }

    #[test]
    fn powi_zero_exponent_does_not_launder_poison() {
        // Healthy input: x⁰ = [1, 1].
        let one = iv(-1.0, 2.0).powi(0);
        assert_eq!((one.lo(), one.hi()), (1.0, 1.0));

        // Poison propagates through n = 0 — in agreement with f64, whose
        // powi guards n = 0 to propagate NaN (both scalars refuse to
        // launder poison into an exact 1).
        let empty = iv(-4.0, -1.0).sqrt();
        assert!(empty.powi(0).0.is_empty());
        assert!(Interval::from_f64(f64::NAN).powi(0).0.is_nai());
    }

    /// The subtler n = 0 laundering channel: a *clamped* (Trv) value has a
    /// plausible enclosure, and x⁰ collapses it to the exact point [1, 1] —
    /// if pown reset the decoration, the upstream domain violation would
    /// vanish into the most innocent-looking value there is. inari
    /// preserves Trv through pown; pin it, decision refusal included.
    #[test]
    fn trv_clamp_survives_zero_exponent_and_still_refuses() {
        let x = iv(-1.0, 4.0).sqrt().powi(0);
        assert_eq!((x.lo(), x.hi()), (1.0, 1.0));
        assert_eq!(x.0.decoration(), Decoration::Trv);

        let band = band_1e9();
        assert_eq!(
            x.sign_within(band),
            Err(Indeterminate {
                margin: MarginDiag::Invalid,
                band,
                predicate: None,
            })
        );
    }

    #[test]
    fn min_max_are_interval_envelopes_and_propagate_poison() {
        // Disjoint: min/max select whole intervals.
        let a = iv(1.0, 3.0);
        let b = iv(4.0, 6.0);
        assert_eq!((Real::min(a, b).lo(), Real::min(a, b).hi()), (1.0, 3.0));
        assert_eq!((Real::max(a, b).lo(), Real::max(a, b).hi()), (4.0, 6.0));

        // Overlapping: the pointwise envelope, not either operand.
        let c = iv(0.0, 5.0);
        assert_eq!((Real::min(a, c).lo(), Real::min(a, c).hi()), (0.0, 3.0));
        assert_eq!((Real::max(a, c).lo(), Real::max(a, c).hi()), (1.0, 5.0));

        // Empty propagates (the interval reading of NaN propagation).
        let empty = iv(-4.0, -1.0).sqrt();
        assert!(Real::min(empty, a).0.is_empty());
        assert!(Real::max(a, empty).0.is_empty());

        // NaI propagates.
        let nai = Interval::from_f64(f64::NAN);
        assert!(Real::min(nai, a).0.is_nai());
        assert!(Real::max(a, nai).0.is_nai());
    }

    #[test]
    fn sin_of_pi_sixth_contains_the_true_half() {
        // sin(π/6) = 1/2 EXACTLY, and 1/2 is representable: a rare case
        // where truth containment for a transcendental is checkable
        // exactly. (Note what is NOT asserted: containment of
        // libm::sin(fl(π)/6.0) — an f64 evaluation may fall outside a
        // tight enclosure of the truth; module docs.)
        let sixth = Interval::pi() / Interval::from_f64(6.0);
        let s = sixth.sin();
        assert!(s.0.contains(0.5), "sin(π/6) enclosure must contain 1/2");
        // And the enclosure is tight: a few ulps around 0.5.
        assert!(s.hi() - s.lo() <= 1e-15);
    }

    #[test]
    fn generic_hypotenuse_at_interval_is_exact_on_3_4_5() {
        // 3² + 4² = 25 and sqrt(25) = 5, all exact: the generic consumer
        // instantiated at Interval produces the exact point [5, 5].
        let h = hypotenuse(Interval::from_f64(3.0), Interval::from_f64(4.0));
        assert_eq!((h.lo(), h.hi()), (5.0, 5.0));
    }

    #[test]
    fn sin_cos_projections_are_bit_identical_to_the_pair() {
        // Interval keeps the defaulted sin/cos projections, so this is
        // trivially true; it pins the trait contract all the same.
        for x in [-2.5, 0.0, 0.7, 3.9] {
            let v = Interval::from_f64(x);
            let (s, c) = v.sin_cos();
            assert_eq!(v.sin().lo().to_bits(), s.lo().to_bits());
            assert_eq!(v.sin().hi().to_bits(), s.hi().to_bits());
            assert_eq!(v.cos().lo().to_bits(), c.lo().to_bits());
            assert_eq!(v.cos().hi().to_bits(), c.hi().to_bits());
        }
    }

    /// THE decoration-channel test: a domain violation that leaves a
    /// perfectly plausible — even decisively positive — enclosure must
    /// still refuse to branch, because the violation lives only in the
    /// decoration.
    #[test]
    fn domain_clamp_poisons_the_decision_despite_a_plausible_enclosure() {
        let band = band_1e9();

        // sqrt([-1, 4]) clamps to [0, 2] with decoration Trv.
        let clamped = iv(-1.0, 4.0).sqrt();
        assert_eq!((clamped.lo(), clamped.hi()), (0.0, 2.0));

        // Push it decisively positive: [1, 3] — by value alone this
        // would classify Positive (lo = 1 ≫ escalate = 1e-8)...
        let shifted = clamped + Interval::one();
        assert_eq!((shifted.lo(), shifted.hi()), (1.0, 3.0));
        assert_eq!(shifted.0.decoration(), Decoration::Trv);

        // ...but the decoration says the domain was violated upstream,
        // and a poisoned computation never takes a branch.
        for poisoned in [clamped, shifted] {
            assert_eq!(
                poisoned.sign_within(band),
                Err(Indeterminate {
                    margin: MarginDiag::Invalid,
                    band,
                    predicate: None,
                })
            );
        }
    }

    #[test]
    fn full_domain_miss_stays_empty_downstream_and_never_branches() {
        let band = band_1e9();
        let empty = Interval::from_f64(-1.0).sqrt();
        assert!(empty.0.is_empty());

        // Every downstream operation preserves emptiness...
        let downstream = (empty * Interval::from_f64(2.0) + Interval::one()).abs();
        assert!(downstream.0.is_empty());

        // ...and the decision refuses, with the poison diagnostic.
        assert_eq!(
            downstream.sign_within(band),
            Err(Indeterminate {
                margin: MarginDiag::Invalid,
                band,
                predicate: None,
            })
        );
    }

    /// The interval boundary table, mirroring `predicate.rs`'s f64 table:
    /// every region-inclusion choice against the fixed band
    /// (zero = 1e-9, escalate = 1e-8), including enclosures straddling
    /// each boundary.
    #[test]
    fn interval_boundary_table() {
        let band = band_1e9();
        let indeterminate = |lo: f64, hi: f64| {
            Err(Indeterminate {
                margin: MarginDiag::Enclosure { lo, hi },
                band,
                predicate: None,
            })
        };
        let invalid = Err(Indeterminate {
            margin: MarginDiag::Invalid,
            band,
            predicate: None,
        });

        // First representable values beyond the thresholds.
        let above_zero = 1e-9f64.next_up();
        let below_escalate = 1e-8f64.next_down();

        #[rustfmt::skip]
        let table: &[(Interval, Result<Sign, Indeterminate>, &str)] = &[
            // -- Zero: the enclosure must be a SUBSET of [-zero, zero].
            (Interval::zero(),      Ok(Sign::Zero), "the exact point 0"),
            (iv(-1e-9, 1e-9),       Ok(Sign::Zero), "the entire coincidence region: closed inclusion"),
            (iv(0.0, 1e-9),         Ok(Sign::Zero), "up to and including +zero"),
            (Interval::from_f64(-1e-9), Ok(Sign::Zero), "the point at -zero exactly"),
            // -- Straddling the zero threshold: indeterminate.
            (iv(-above_zero, 0.0),  indeterminate(-above_zero, 0.0), "pokes just past -zero"),
            (iv(0.0, above_zero),   indeterminate(0.0, above_zero), "pokes just past +zero"),
            (iv(-2e-9, 2e-9),       indeterminate(-2e-9, 2e-9), "symmetric straddle of the coincidence region"),
            // -- Inside the ambiguity band: indeterminate EVEN FOR POINTS
            //    (the sliver band is semantic — ratified reading (b)).
            (Interval::from_f64(5e-9), indeterminate(5e-9, 5e-9), "a point in the sliver band stays indeterminate"),
            (iv(2e-9, 3e-9),        indeterminate(2e-9, 3e-9), "an enclosure wholly inside the band"),
            (Interval::from_f64(-5e-9), indeterminate(-5e-9, -5e-9), "sliver point, negative side"),
            // -- Straddling the escalate threshold: indeterminate.
            (iv(below_escalate, 1.0), indeterminate(below_escalate, 1.0), "lo one ulp short of escalate"),
            (iv(-1.0, -below_escalate), indeterminate(-1.0, -below_escalate), "hi one ulp short of -escalate"),
            (iv(5e-9, 1.0),         indeterminate(5e-9, 1.0), "from mid-band into definite territory"),
            // -- Wide enclosures spanning everything: indeterminate.
            (iv(-1.0, 1.0),         indeterminate(-1.0, 1.0), "spans all three regions"),
            (iv(0.0, f64::INFINITY), indeterminate(0.0, f64::INFINITY), "half-unbounded (Dac) still classifies by inclusion"),
            // -- Definite: lo ≥ escalate (closed), mirrored for negative.
            (Interval::from_f64(1e-8), Ok(Sign::Positive), "the point at +escalate: closed toward definite"),
            (iv(1e-8, 2e-8),        Ok(Sign::Positive), "lo exactly at escalate"),
            (iv(1.0, 2.0),          Ok(Sign::Positive), "far outside the band"),
            (iv(2.0, f64::INFINITY), Ok(Sign::Positive), "unbounded above but certifiably clear (Dac decides)"),
            (Interval::from_f64(-1e-8), Ok(Sign::Negative), "the point at -escalate"),
            (iv(-2e-8, -1e-8),      Ok(Sign::Negative), "hi exactly at -escalate"),
            (iv(f64::NEG_INFINITY, -2.0), Ok(Sign::Negative), "unbounded below but certifiably clear"),
            // -- Poison: never a branch.
            (Interval::from_f64(f64::NAN), invalid, "NaI"),
            (Interval::from_f64(-1.0).sqrt(), invalid, "empty enclosure"),
            (iv(-1.0, 4.0).sqrt(),  invalid, "clamped (Trv) enclosure — the decoration channel"),
        ];

        for (x, expected, why) in table {
            assert_eq!(
                x.sign_within(band),
                *expected,
                "enclosure [{:e}, {:e}]: {why}",
                x.lo(),
                x.hi()
            );
        }
    }

    #[test]
    fn evaluation_is_bit_identical_across_repetitions() {
        // D9 at interval type: the same pipeline, re-run, yields
        // bit-identical enclosure bounds every time (the probe verified
        // this across CPUs; this pins it within one process).
        fn pipeline(seed: f64) -> Interval {
            let x = Interval::from_f64(seed);
            let (s, c) = (x * Interval::pi() + Interval::one()).sin_cos();
            let h = hypotenuse(s, c);
            Real::atan2(h, x.abs() + Interval::from_f64(0.5)).powi(3) / Interval::from_f64(7.0)
        }

        for seed in [0.0, 1.5e-9, -3.25, 1234.5] {
            let first = pipeline(seed);
            let key = (first.lo().to_bits(), first.hi().to_bits());
            for _ in 0..16 {
                let again = pipeline(seed);
                assert_eq!(
                    (again.lo().to_bits(), again.hi().to_bits()),
                    key,
                    "pipeline({seed}) must be bit-identical on re-evaluation"
                );
            }
        }
    }

    /// The kink selectors' decoration convention (see the
    /// [`KinkJacobian`] impl above): outputs are capped by the deciding
    /// values' decorations, poison propagates shape-correctly. These pins
    /// live here (not in `crate::dual`'s tests) because they read the
    /// wrapped `DecInterval` directly.
    #[test]
    fn kink_selector_decorations() {
        let trv_value = iv(-1.0, 4.0).sqrt(); // [0, 2], Trv (clamped)
        let clean = iv(1.0, 2.0);
        let tangent = iv(3.0, 3.0);

        // abs factor: the value's own decoration rides on the factor.
        let factor = trv_value.abs_sign_factor();
        assert_eq!((factor.lo(), factor.hi()), (1.0, 1.0));
        assert_eq!(factor.0.decoration(), Decoration::Trv);
        let factor = clean.abs_sign_factor();
        assert_eq!(factor.0.decoration(), Decoration::Com);
        // Straddle keeps a clean input's decoration too (the hull is a
        // convention, not a domain violation).
        let factor = iv(-1.0, 2.0).abs_sign_factor();
        assert_eq!((factor.lo(), factor.hi()), (-1.0, 1.0));
        assert_eq!(factor.0.decoration(), Decoration::Com);
        // Poison shapes: NaI in, NaI out; empty in, empty out.
        assert!(Interval::from_f64(f64::NAN).abs_sign_factor().0.is_nai());
        let empty = iv(-4.0, -1.0).sqrt();
        assert!(empty.abs_sign_factor().0.is_empty());
        assert!(!empty.abs_sign_factor().0.is_nai());

        // min_deriv: a Trv value on either side caps the selected tangent.
        let selected = trv_value.min_deriv(tangent, iv(10.0, 11.0), iv(9.0, 9.0));
        assert_eq!((selected.lo(), selected.hi()), (3.0, 3.0));
        assert_eq!(selected.0.decoration(), Decoration::Trv);
        // Clean separation keeps the chosen tangent's decoration.
        let selected = clean.min_deriv(tangent, iv(10.0, 11.0), iv(9.0, 9.0));
        assert_eq!(selected.0.decoration(), Decoration::Com);
        // The hull's decoration is the min of the tangents' (deliberately
        // NOT 1788's unconditional Trv for set ops — doc on tangent_hull):
        // a Trv tangent hulled with a Com one yields Trv.
        let trv_tangent = iv(-1.0, 4.0).sqrt();
        let hulled = clean.min_deriv(trv_tangent, iv(1.0, 3.0), tangent);
        assert_eq!((hulled.lo(), hulled.hi()), (0.0, 3.0));
        assert_eq!(hulled.0.decoration(), Decoration::Trv);
        let hulled = clean.min_deriv(tangent, iv(1.0, 3.0), tangent);
        assert_eq!(hulled.0.decoration(), Decoration::Com);
        // Poisoned VALUES poison the tangent outright (both-channel
        // poison, mirroring f64's NaN guard in the value channel's min).
        assert!(
            Interval::from_f64(f64::NAN)
                .min_deriv(tangent, clean, tangent)
                .0
                .is_nai()
        );
        assert!(empty.min_deriv(tangent, clean, tangent).0.is_empty());
        // A poisoned TANGENT inside the hull region propagates poison.
        assert!(clean.min_deriv(empty, iv(1.0, 3.0), tangent).0.is_empty());
        assert!(
            clean
                .min_deriv(Interval::from_f64(f64::NAN), iv(1.0, 3.0), tangent)
                .0
                .is_nai()
        );

        // max_deriv mirrors: certainly-greater selects.
        let selected = iv(5.0, 6.0).max_deriv(tangent, clean, iv(9.0, 9.0));
        assert_eq!((selected.lo(), selected.hi()), (3.0, 3.0));
        let other_selected = clean.max_deriv(tangent, iv(5.0, 6.0), iv(9.0, 9.0));
        assert_eq!((other_selected.lo(), other_selected.hi()), (9.0, 9.0));
        // Overlap hulls.
        let hulled = iv(1.0, 3.0).max_deriv(tangent, iv(2.0, 4.0), iv(9.0, 9.0));
        assert_eq!((hulled.lo(), hulled.hi()), (3.0, 9.0));
    }

    proptest! {
        /// Inclusion monotonicity, unary: x ⊆ y ⇒ f(x) ⊆ f(y) — the
        /// defining property of interval extensions, over every unary
        /// trait operation (including ones with partial domains, where
        /// clamping preserves inclusion).
        #[test]
        fn unary_ops_are_inclusion_monotone(
            raw in proptest::array::uniform4(-50.0..50.0f64),
        ) {
            let mut s = raw;
            s.sort_by(f64::total_cmp);
            let outer = iv(s[0], s[3]);
            let inner = iv(s[1], s[2]);

            let ops: &[(&str, UnaryOp)] = &[
                ("sqrt", Real::sqrt),
                ("abs", Real::abs),
                ("sin", Real::sin),
                ("cos", Real::cos),
                ("tan", Real::tan),
                ("asin", Real::asin),
                ("acos", Real::acos),
                ("atan", Real::atan),
                ("sq", |x| x.powi(2)),
                ("cube", |x| x.powi(3)),
                ("recip", |x| x.powi(-1)),
            ];
            for (name, f) in ops {
                prop_assert!(
                    subset(f(inner), f(outer)),
                    "{name}: f([{:e}, {:e}]) ⊄ f([{:e}, {:e}])",
                    inner.lo(), inner.hi(), outer.lo(), outer.hi()
                );
            }
        }

        /// Inclusion monotonicity, binary: x₁ ⊆ y₁ and x₂ ⊆ y₂ ⇒
        /// x₁ ∘ x₂ ⊆ y₁ ∘ y₂ for every binary trait operation.
        #[test]
        fn binary_ops_are_inclusion_monotone(
            raw_a in proptest::array::uniform4(-50.0..50.0f64),
            raw_b in proptest::array::uniform4(-50.0..50.0f64),
        ) {
            let (mut a, mut b) = (raw_a, raw_b);
            a.sort_by(f64::total_cmp);
            b.sort_by(f64::total_cmp);
            let (outer_a, inner_a) = (iv(a[0], a[3]), iv(a[1], a[2]));
            let (outer_b, inner_b) = (iv(b[0], b[3]), iv(b[1], b[2]));

            let ops: &[(&str, BinaryOp)] = &[
                ("add", |x, y| x + y),
                ("sub", |x, y| x - y),
                ("mul", |x, y| x * y),
                ("div", |x, y| x / y),
                ("min", Real::min),
                ("max", Real::max),
                ("atan2", Real::atan2),
            ];
            for (name, f) in ops {
                prop_assert!(
                    subset(f(inner_a, inner_b), f(outer_a, outer_b)),
                    "{name} not inclusion-monotone"
                );
            }
        }

        /// Exact single operations at points contain the f64 result:
        /// +, −, ·, /, sqrt are correctly rounded in IEEE 754, so the
        /// f64 value is one of the enclosure's endpoints' neighbors —
        /// containment is guaranteed. (Transcendentals are deliberately
        /// NOT tested this way; module docs.)
        #[test]
        fn exact_point_ops_contain_the_f64_result(
            a in -1.0e6..1.0e6f64,
            b in -1.0e6..1.0e6f64,
        ) {
            let (pa, pb) = (Interval::from_f64(a), Interval::from_f64(b));
            prop_assert!((pa + pb).0.contains(a + b));
            prop_assert!((pa - pb).0.contains(a - b));
            prop_assert!((pa * pb).0.contains(a * b));
            prop_assume!(b.abs() > 1e-3); // keep a/b finite
            prop_assert!((pa / pb).0.contains(a / b));
        }

        /// sqrt at nonnegative points contains the (exactly rounded)
        /// f64 sqrt, and the enclosure is at most 1 ulp wide.
        #[test]
        fn sqrt_point_containment(x in 0.0..1.0e12f64) {
            let r = Interval::from_f64(x).sqrt();
            prop_assert!(r.0.contains(x.sqrt()));
            prop_assert!(r.hi() <= r.lo().next_up());
        }

        /// The Pythagorean identity as a truth-containment statement:
        /// sin²x + cos²x − 1 encloses 0 with tiny width, for every x —
        /// the interval way of testing transcendentals (no f64 value
        /// containment involved).
        #[test]
        fn pythagorean_identity_encloses_zero(x in -1.0e3..1.0e3f64) {
            let (s, c) = Interval::from_f64(x).sin_cos();
            let residual = s * s + c * c - Interval::one();
            prop_assert!(residual.0.contains(0.0));
            prop_assert!(residual.0.wid() < 1e-13);
        }

        /// Classification at interval type agrees with f64 classification
        /// on healthy POINT enclosures — the two instantiations of
        /// `Decide` implement the same decision table.
        #[test]
        fn point_classification_agrees_with_f64(
            zero in 1.0e-12..1.0e-3f64,
            ratio in 1.5..100.0f64,
            t in -3.0..3.0f64,
        ) {
            let band = Band::new(zero, zero * ratio).unwrap();
            let m = t * band.escalate();
            let at_f64 = m.sign_within(band);
            let at_interval = Interval::from_f64(m).sign_within(band);
            match (at_f64, at_interval) {
                (Ok(a), Ok(b)) => prop_assert_eq!(a, b),
                (Err(e_f), Err(e_i)) => {
                    prop_assert_eq!(e_f.margin, MarginDiag::Value(m));
                    prop_assert_eq!(e_i.margin, MarginDiag::Enclosure { lo: m, hi: m });
                    prop_assert_eq!(e_i.band, band);
                }
                (a, b) => prop_assert!(
                    false,
                    "instantiations disagree at m = {}: f64 → {:?}, interval → {:?}",
                    m, a, b
                ),
            }
        }
    }
}
