//! Forward-mode dual numbers — Q1's derivative-carrying instantiation of
//! [`Real`] (M0 PR 5).
//!
//! A [`Dual`] pairs a value with the derivative of that value with respect
//! to one scalar parameter: evaluating a recipe at
//! `Dual { value: x, deriv: 1 }` yields `Dual { value: f(x), deriv: f'(x) }`
//! — exact forward-mode differentiation, no finite differences, one pass.
//! The wrapper is generic over the base scalar: `Dual<f64>` ([`Dual64`])
//! gives fast sensitivities, `Dual<Interval>` ([`DualInterval`], behind the
//! `interval` cargo feature) gives *certified* derivative enclosures — the
//! same chain-rule code serves both, which is the point of writing the
//! chain rules in [`Real`]-surface operations.
//!
//! # In-house generic wrapper; num-dual demoted to a test oracle
//!
//! DESIGN.md's crate table originally slated `num-dual` for `Dual<f64>`.
//! That plan fails a load-bearing contract: num-dual routes its
//! transcendentals through the base scalar's `Float` implementation —
//! **std** `sin`/`cos`, not the `libm` crate D9 mandates — so a `Real`
//! written over `num_dual::Dual64` could not keep its value part
//! bit-identical to the plain-`f64` run of the same recipe. Wrapping
//! num-dual to fix the value channel means reimplementing every chain rule
//! anyway, at which point the dependency carries no weight. So: one
//! in-house `Dual<T>`, chain rules expressed in our own trait ops, and
//! num-dual demoted to a **dev-dependency oracle** — the test suite
//! compares our `Dual<f64>` derivatives against num-dual's within ulp
//! bounds (see the `num_dual_oracle` test module for why its *value*
//! channel legitimately differs from ours). Deviation flagged for
//! ratification in this PR's design conversation.
//!
//! # THE contract: the value channel *is* the plain-`T` computation
//!
//! For every operation, `Dual { value: x, .. }.op().value` is **bit-
//! identical** to `x.op()` at the base scalar. This holds by construction
//! — the value channel literally computes with `T`'s own operations, never
//! a rearranged variant — and it is what makes a dual replay trustworthy:
//! the value part of a `Dual<f64>` run *is* the `f64` run (poison
//! semantics included: `f64`'s NaN-at-`powi(_, 0)` guard, the interval
//! scalar's clamp-and-decorate behavior — all inherited verbatim), with
//! the derivative riding alongside. Under test across every operation.
//!
//! Derivative channels have no such cross-type identity — they are new
//! computations. Their association orders are fixed and documented per
//! method (D9 determinism), and [`Real::sin_cos`] is the primitive that
//! pays off here: one `sin_cos` of the value serves the value *and*
//! derivative channels of both trigonometric components.
//!
//! # Kink conventions (`abs`, `min`, `max`, `floor`, `copysign`)
//!
//! The non-smooth operations need sign/order decisions that [`Real`]
//! deliberately cannot express, so they live in a sealed crate-private
//! helper trait ([`KinkJacobian`]) implemented per base scalar. The
//! ratified conventions:
//!
//! - **`abs`**: derivative factor `sign(value)`. At `f64`: `+1` for
//!   `value ≥ 0` — *including both signed zeros* (the sign of a
//!   floating-point zero is a representation artifact; `+1` matches
//!   `abs'(0⁺)`, a fixed subgradient pick, ties-keep-self flavored) —
//!   `−1` for `value < 0`, NaN propagates. At `Interval`: `[1, 1]` for a
//!   value enclosure in `[0, ∞)`, `[−1, −1]` in `(−∞, 0]`, and the honest
//!   hull `[−1, 1]` for a zero-straddling enclosure — an *enclosure of the
//!   subgradient*, a value computation, never a decision (no
//!   `Indeterminate` here); empty/NaI propagates.
//! - **`min`/`max`**: the value channel is the base scalar's own
//!   `min`/`max` (f64: ties keep `self`, NaN propagates; interval: the
//!   envelope). The derivative follows the chosen argument at `f64` (ties
//!   keep `self`, so the derivative keeps `self`'s — the PR 2 contract);
//!   at `Interval`, when neither argument is strictly separated from the
//!   other the derivative is the **hull of both tangents** (the lattice
//!   reading generalizes: a tie-region's derivative is the hull of both
//!   branches).
//! - **`floor`** (M2 PR 1): derivative factor 0 — floor is locally
//!   constant. At `f64` the factor is 0 *including at integers*
//!   (branch-consistency: the program as evaluated sits on a plateau;
//!   floor is right-continuous, so the plateau at an integer `k` is
//!   `[k, k+1)`'s), with NaN propagating. At `Interval`: `[0, 0]` when
//!   the value enclosure certainly spans no integer step, and the honest
//!   jump enclosure `[0, +∞]` when it may (floor is nondecreasing, so
//!   difference quotients are ≥ 0 but unbounded across a jump — the
//!   step-function analogue of `abs`'s straddle hull; `[0, 0]` there
//!   would falsify the mean-value enclosure).
//! - **`copysign`** (M2 PR 1): `d copysign(x, s) = σ(s)·abs′(x)·dx`,
//!   with σ the transferred sign and `abs′` the `abs` convention above;
//!   the `s` tangent is **discarded** (σ is locally constant in `s` — the
//!   same locally-the-chosen-branch discard as `min`'s unchosen
//!   argument). At `Interval` a `sign` enclosure straddling zero poisons
//!   the tangent to the entire line: the sign flip is a jump of `2|x|`
//!   over zero width, so no finite enclosure is honest.
//!
//! # Derivatives never branch (the Q1 residue, resolved)
//!
//! [`Decide`] for `Dual<T>` classifies the **value part only**; the
//! derivative never influences a branch. Rationale: derivatives are
//! *tangent-space* data — the linearization of the map at a point — while
//! topology is decided in the *base space*, on values. A huge or even
//! poisoned derivative at a cleanly-classifiable value means the
//! *sensitivity* is bad news (M6's stackup machinery will surface it),
//! not that the point is somewhere else. Delegation settles Q1's
//! "comparison/signum semantics of `Dual<Interval>`" with no new design:
//! `Dual<f64>` inherits `f64`'s total-with-escalation semantics and
//! `Dual<Interval>` inherits the enclosure semantics, Indeterminate ⇒
//! subdivision included. Consistently with [`Real`], there is no `signum`.
//!
//! # [`crate::Bounds`] yes, [`crate::CertifiedEnclosure`] no
//!
//! A dual **carries a bracket** and **may not certify** — Wave 0 decision
//! **D1** of `docs/SMELL-SCAN-2026-08.md` (Evan, 2026-08-19: *a `Dual` may
//! not certify — at least for now — but it may have `Bounds`*), separable
//! only because #643 (W1c/S41) made those two traits instead of one.
//! [`crate::Bounds`] is implemented and is the **value channel's** bracket
//! with the tangent discarded; [`crate::CertifiedEnclosure`] is not. **The
//! argument for both halves lives at the `impl Bounds for Dual` below, not
//! here** — one home, so there is one text to keep in step.
//!
//! Reading the derivative channel is still a **driver/harvest** activity
//! through the public fields, at the same restricted scope as
//! [`crate::Bounds`] itself (L7). `Bounds` offers no route to it: `lo()`
//! and `hi()` never see a tangent.
//!
//! The refusing half is pinned as a **compiler fact**, here rather than at
//! the impl because there is no impl to hang it on — it goes red the day
//! someone writes the certification impl, rather than the day something
//! certifies wrongly:
//!
//! ```
//! fn brackets<T: geom_core::Bounds>(_t: T) {}
//! brackets(geom_core::Dual64::constant(1.0));
//! ```
//!
//! ```compile_fail,E0277
//! fn certifies<T: geom_core::CertifiedEnclosure>(_t: T) {}
//! certifies(geom_core::Dual64::constant(1.0));
//! ```
//!
//! # Multi-parameter gradients
//!
//! One derivative slot, one parameter. M6's stackups over parameter
//! *vectors* extend this as gradient arrays (`Dual<T, const N: usize>`
//! style) or by nesting — an extension, not a rewrite; nothing here
//! forecloses it structurally.

use core::ops::{Add, Div, Mul, Neg, Sub};

use crate::predicate::{Band, Decide, Indeterminate, Sign};
use crate::real::{Bounds, Real};

#[cfg(feature = "interval")]
use crate::interval::Interval;

/// A forward-mode dual number over the base scalar `T`: a value and the
/// derivative of that value with respect to one scalar parameter.
///
/// Implements [`Real`] for the kernel's base scalars (`f64` and, behind
/// the `interval` feature, [`Interval`]), so any evaluation code generic
/// over [`Real`] differentiates itself when instantiated here. See the
/// [module docs](self) for the value-channel contract, the kink
/// conventions, and the decide-by-value rule.
///
/// The fields are **public**: every `(value, deriv)` pair is a valid dual
/// — there is no invariant for privacy to protect, and accessors would be
/// pure ceremony. Reading the channels is a **driver/harvest** activity
/// (seed a parameter with [`Dual::variable`], evaluate, read `deriv` out),
/// the same restricted scope as [`crate::real::Bounds`] under the L7 style
/// rule — generic evaluation code cannot name these fields anyway (it sees
/// an opaque `T: Real`), so public fields open no door in the
/// evaluation-code discipline.
#[derive(Debug, Clone, Copy)]
pub struct Dual<T> {
    /// The value channel — bit-identical to the plain-`T` computation of
    /// the same recipe (the module-level contract).
    pub value: T,
    /// The derivative channel — d(value)/d(parameter) for the one scalar
    /// parameter seeded via [`Dual::variable`].
    pub deriv: T,
}

impl<T: Real> Dual<T> {
    /// A dual from explicit channels (struct-literal shorthand).
    pub fn new(value: T, deriv: T) -> Self {
        Self { value, deriv }
    }

    /// The differentiation seed: `value` with derivative one. Evaluating
    /// `f` at `Dual::variable(x)` yields `f(x)` paired with `f'(x)`.
    pub fn variable(value: T) -> Self {
        Self {
            value,
            deriv: T::one(),
        }
    }

    /// A constant: `value` with derivative zero (an input the derivative
    /// is *not* taken with respect to).
    pub fn constant(value: T) -> Self {
        Self {
            value,
            deriv: T::zero(),
        }
    }
}

/// The non-smooth selectors [`Dual`]'s chain rules need from their base
/// scalar — a **sealed, crate-private helper trait** (written with
/// [`Real`] as its supertrait) implemented for `f64` here and for
/// [`Interval`] in `crate::interval` (feature-gated). It carries exactly
/// the three kink selectors: the `abs` sign factor and the `min`/`max`
/// tangent choice.
///
/// [`Real`] deliberately has no comparisons, and `d|x|/dx` or "whose
/// tangent does `min` keep" are order decisions *by nature* — not
/// expressible in [`Real`] ops, by design. Raw comparisons are allowed
/// *inside scalar implementations* (established at `f64`'s
/// [`Real::min`]); this trait is where that allowance lives for duals:
/// each base scalar answers the kink questions in its own honest way
/// (selection at `f64`, subgradient hulls at `Interval` — the module-doc
/// conventions), and the generic chain rules consume the answers without
/// ever branching themselves.
///
/// `pub(crate)` seals it: downstream code cannot implement it, so
/// `Dual<T>` instantiates only at the kernel's own base scalars (nesting
/// duals is an M6 design question, not a backdoor left ajar).
pub(crate) trait KinkJacobian: Real {
    /// The Jacobian factor of `abs` at `self`: the value the tangent gets
    /// multiplied by, `d|x|/dx` evaluated (or enclosed) at the value.
    /// Conventions per scalar are in the [module docs](self): `f64` gives
    /// `±1` (`+1` at both signed zeros) with NaN propagating; `Interval`
    /// gives `[1, 1]` / `[−1, −1]` / the straddle hull `[−1, 1]` with
    /// empty/NaI propagating.
    fn abs_sign_factor(self) -> Self;

    /// The tangent of `min(self, other)` given both arguments' tangents.
    /// `f64`: NaN in either *value* poisons the result (mirroring
    /// [`Real::min`]'s value channel); otherwise the tangent follows the
    /// chosen argument, ties keep `self` — the unchosen argument's tangent
    /// is discarded entirely (poison included: `min` is locally the chosen
    /// branch, and the loud channel for the unchosen branch's troubles is
    /// its own value). `Interval`: strict separation selects, anything
    /// else hulls both tangents; empty/NaI values poison.
    fn min_deriv(self, self_deriv: Self, other: Self, other_deriv: Self) -> Self;

    /// The tangent of `max(self, other)` — [`KinkJacobian::min_deriv`]
    /// mirrored (ties still keep `self`).
    fn max_deriv(self, self_deriv: Self, other: Self, other_deriv: Self) -> Self;

    /// The Jacobian factor of `floor` at `self`: 0 on plateaus, per the
    /// module-doc conventions. `f64` gives 0 everywhere (including at
    /// integers — branch-consistency with the right-continuous plateau)
    /// with NaN propagating; `Interval` gives `[0, 0]` when the value
    /// enclosure certainly spans no integer step and the honest jump
    /// enclosure `[0, +∞]` when it may, with empty/NaI propagating.
    fn floor_jacobian_factor(self) -> Self;

    /// The tangent factor of `powi(·, 0)` at `self`: the **exactly
    /// zero** derivative of the constant-one function, *tainted by the
    /// value's poison* — `0` (or `[0, 0]`) for every describable value
    /// (±∞ included: the function is constant there too), with `f64`
    /// NaN propagating and `Interval` empty/NaI propagating and the
    /// zero's decoration capped by the value's. This is what keeps
    /// `powi(0)` from laundering a poisoned value channel into a
    /// fresh, cleanly-classifying zero tangent (#126, option (a)).
    fn powi_zero_deriv_factor(self) -> Self;

    /// The tangent of `copysign(self, sign)` given `self`'s tangent:
    /// `σ(sign)·abs′(self)·self_deriv` per the module-doc conventions.
    /// The `sign` argument's own tangent is deliberately not a parameter
    /// — σ is locally constant in `sign`, so that tangent is discarded
    /// (the same discard rule as `min`'s unchosen branch). `f64`:
    /// either value NaN poisons; otherwise the signed `abs` factor
    /// selects. `Interval`: a `sign` enclosure straddling zero yields
    /// the entire line (the jump has unbounded slope); a sign-definite
    /// enclosure selects `±abs_sign_factor(self)·self_deriv`, with the
    /// result's decoration capped by the deciding values'.
    fn copysign_deriv(self, self_deriv: Self, sign: Self) -> Self;
}

/// `f64` kink selectors: IEEE comparisons, exact and deterministic (D9).
/// Raw comparison is allowed inside scalar implementations (Q1) — this is
/// the duals' single home for it at `f64`.
impl KinkJacobian for f64 {
    /// `+1` for `self ≥ 0` — including both signed zeros: the sign of a
    /// floating-point zero is a representation artifact (`-0.0 >= 0.0` is
    /// true in IEEE 754, so both zeros take the `+1` arm), and `+1` is the
    /// fixed subgradient pick matching `abs'(0⁺)`. `−1` for `self < 0`;
    /// NaN propagates.
    fn abs_sign_factor(self) -> Self {
        if self.is_nan() {
            f64::NAN
        } else if self >= 0.0 {
            1.0
        } else {
            -1.0
        }
    }

    /// Selection mirroring `f64`'s [`Real::min`] exactly: either value NaN
    /// ⇒ NaN; `self ≤ other` keeps `self`'s tangent (ties keep `self`),
    /// else `other`'s.
    fn min_deriv(self, self_deriv: Self, other: Self, other_deriv: Self) -> Self {
        if self.is_nan() || other.is_nan() {
            f64::NAN
        } else if self <= other {
            self_deriv
        } else {
            other_deriv
        }
    }

    /// Selection mirroring `f64`'s [`Real::max`] exactly: either value NaN
    /// ⇒ NaN; `self ≥ other` keeps `self`'s tangent (ties keep `self`),
    /// else `other`'s.
    fn max_deriv(self, self_deriv: Self, other: Self, other_deriv: Self) -> Self {
        if self.is_nan() || other.is_nan() {
            f64::NAN
        } else if self >= other {
            self_deriv
        } else {
            other_deriv
        }
    }

    /// 0 everywhere — floor is locally constant, and at an integer the
    /// program as evaluated sits on the right-continuous plateau
    /// (branch-consistency, module docs). NaN propagates.
    fn floor_jacobian_factor(self) -> Self {
        if self.is_nan() { f64::NAN } else { 0.0 }
    }

    /// 0 everywhere the value is describable (`x⁰` is the constant 1,
    /// ±∞ included); NaN propagates (trait docs — the #126 fix).
    fn powi_zero_deriv_factor(self) -> Self {
        if self.is_nan() { f64::NAN } else { 0.0 }
    }

    /// `σ(sign)·abs′(self)·self_deriv`, associations fixed as written:
    /// the transferred sign σ = `copysign(1, sign)` (a bit read — locally
    /// constant, including at `sign = ±0`), times the `abs` factor (the
    /// `+1`-at-zero convention), times the tangent. Either value NaN ⇒
    /// NaN; a NaN tangent rides through the multiplications.
    fn copysign_deriv(self, self_deriv: Self, sign: Self) -> Self {
        if self.is_nan() || sign.is_nan() {
            f64::NAN
        } else {
            f64::copysign(1.0, sign) * self.abs_sign_factor() * self_deriv
        }
    }
}

impl<T: Real> Add for Dual<T> {
    type Output = Self;

    /// `(a + b, a' + b')`.
    fn add(self, rhs: Self) -> Self {
        Self {
            value: self.value + rhs.value,
            deriv: self.deriv + rhs.deriv,
        }
    }
}

impl<T: Real> Sub for Dual<T> {
    type Output = Self;

    /// `(a − b, a' − b')`.
    fn sub(self, rhs: Self) -> Self {
        Self {
            value: self.value - rhs.value,
            deriv: self.deriv - rhs.deriv,
        }
    }
}

// The product/quotient rules necessarily mix operators inside `Mul`/`Div`
// bodies — the textbook dual-number false positive for this lint.
#[allow(clippy::suspicious_arithmetic_impl)]
impl<T: Real> Mul for Dual<T> {
    type Output = Self;

    /// Product rule, association fixed as `a'·b + a·b'` (left factor's
    /// tangent first; each product left-associated) — deterministic per
    /// D9.
    fn mul(self, rhs: Self) -> Self {
        Self {
            value: self.value * rhs.value,
            deriv: self.deriv * rhs.value + self.value * rhs.deriv,
        }
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl<T: Real> Div for Dual<T> {
    type Output = Self;

    /// Quotient rule, association fixed as `(a'·b − a·b') / b.powi(2)` —
    /// the classic form, numerator fully evaluated before the single
    /// division. The denominator square is `powi(2)`, not `b·b`: at `f64`
    /// that is bit-identical (`powi_by_squaring` reduces `powi(_, 2)` to
    /// `1·(b·b)`, and `×1` is exact), while at `Dual<Interval>` it is
    /// the interval backend's tight integer power — squaring a
    /// zero-straddling denominator by
    /// dependent multiplication needlessly loses the `[0, ·]` lower bound
    /// (the dependency problem), so the tight square keeps the enclosure
    /// honest. `b = 0` poisons the derivative channel through the division
    /// (±∞/NaN at `f64`, empty at intervals) exactly as it does the value
    /// channel.
    fn div(self, rhs: Self) -> Self {
        Self {
            value: self.value / rhs.value,
            deriv: (self.deriv * rhs.value - self.value * rhs.deriv) / rhs.value.powi(2),
        }
    }
}

impl<T: Real> Neg for Dual<T> {
    type Output = Self;

    /// `(−a, −a')`.
    fn neg(self) -> Self {
        Self {
            value: -self.value,
            deriv: -self.deriv,
        }
    }
}

/// [`Real`] for duals over any sealed base scalar: value channel = the
/// plain-`T` computation (bit-identical, the module-level contract),
/// derivative channel = the chain rule in fixed, documented association
/// orders. Total like every `Real`: out-of-domain inputs poison the value
/// channel via `T`'s own totality policy, and the derivative channel
/// poisons alongside through its arithmetic (division by a poisoned or
/// zero denominator, multiplication by a poisoned factor).
impl<T: KinkJacobian> Real for Dual<T> {
    /// A constant embed: `(T::from_f64(x), 0)`. Exact because `T`'s
    /// embedding is; the derivative of a constant is exactly zero.
    fn from_f64(x: f64) -> Self {
        Self::constant(T::from_f64(x))
    }

    /// `(0, 0)`.
    fn zero() -> Self {
        Self::constant(T::zero())
    }

    /// `(1, 0)`.
    fn one() -> Self {
        Self::constant(T::one())
    }

    /// `(π, 0)` — a constant of the run, not a function of the parameter.
    fn pi() -> Self {
        Self::constant(T::pi())
    }

    /// `(τ, 0)`.
    fn tau() -> Self {
        Self::constant(T::tau())
    }

    /// `(√a, a' / (r + r))` where `r = √a` — the value-channel root is
    /// computed once and reused; the doubling `r + r` is exact at `f64`
    /// and tight at intervals. At the kink `a = 0` the derivative channel
    /// poisons itself (±∞ or NaN at `f64` via division by zero, empty at
    /// intervals) — the domain-edge kink, consistent with totality; a
    /// negative value poisons both channels through `T::sqrt`'s own
    /// poison.
    fn sqrt(self) -> Self {
        let r = self.value.sqrt();
        Self {
            value: r,
            deriv: self.deriv / (r + r),
        }
    }

    /// `(|a|, sign(a)·a')` with the sign factor from
    /// [`KinkJacobian::abs_sign_factor`] (the module-doc kink convention:
    /// `+1` at zero for `f64`, the hull `[−1, 1]` for a straddling
    /// interval). Association fixed as `factor · a'`.
    fn abs(self) -> Self {
        Self {
            value: self.value.abs(),
            deriv: self.value.abs_sign_factor() * self.deriv,
        }
    }

    /// The VALUE channel decides (D8: tangents never decide anything —
    /// a poisoned derivative with a describable value is a poisoned
    /// *tangent*, not a poisoned number).
    fn is_poison(self) -> bool {
        self.value.is_poison()
    }

    /// `(⌊a⌋, floor′(a)·a′)` with the factor from
    /// [`KinkJacobian::floor_jacobian_factor`] (the module-doc step
    /// convention: 0 on plateaus at `f64`, the jump enclosure `[0, +∞]`
    /// over a step-spanning box at `Interval`). Association fixed as
    /// `factor · a′`. The value channel is `T::floor` verbatim.
    fn floor(self) -> Self {
        Self {
            value: self.value.floor(),
            deriv: self.value.floor_jacobian_factor() * self.deriv,
        }
    }

    /// `(copysign(a, s), σ(s)·abs′(a)·a′)` via
    /// [`KinkJacobian::copysign_deriv`] (the module-doc convention: the
    /// `s` tangent is discarded — σ is locally constant in `s`; an
    /// interval `s` straddling zero poisons the tangent to the entire
    /// line). The value channel is `T::copysign` verbatim, poison guard
    /// included.
    fn copysign(self, sign: Self) -> Self {
        Self {
            value: self.value.copysign(sign.value),
            deriv: self.value.copysign_deriv(self.deriv, sign.value),
        }
    }

    /// `(aⁿ, n·aⁿ⁻¹·a')` — the value channel is `T::powi` verbatim, so it
    /// inherits `T`'s poison contract *exactly*: `f64`'s NaN-at-`n = 0`
    /// guard and the interval scalar's tight, decoration-preserving power
    /// come along for free (the module-level value-channel contract).
    ///
    /// Derivative: `n = 0` yields the **exactly zero** tangent of the
    /// constant-one function, but *not* a fresh constant: the zero is
    /// `KinkJacobian::powi_zero_deriv_factor(a) · a'` — exactly 0
    /// whenever the value is describable (±∞ included; forcing `0·a⁻¹`
    /// instead would manufacture NaN at `a = 0`, where `a⁰ = 1` has
    /// honest derivative 0), while a poisoned VALUE channel poisons the
    /// tangent too (#126 option (a): poison-in-poison-out per channel
    /// pair — a fresh clean zero would launder a NaI/`Trv`/NaN input
    /// into a classifiable derivative) and a poisoned tangent rides
    /// through the multiplication as in `floor`. Otherwise
    /// `n·aⁿ⁻¹·a'`, association fixed as `(n · aⁿ⁻¹) · a'` with the power
    /// from `T::powi(n − 1)` — except at the one unrepresentable exponent
    /// `n = i32::MIN`, where `aⁿ⁻¹` is computed as `aⁿ / a` (reusing the
    /// value channel; documented association for that single edge, which
    /// no session-boxed model can reach with a finite result anyway).
    fn powi(self, n: i32) -> Self {
        let value = self.value.powi(n);
        let deriv = if n == 0 {
            self.value.powi_zero_deriv_factor() * self.deriv
        } else {
            let power = if n == i32::MIN {
                value / self.value
            } else {
                self.value.powi(n - 1)
            };
            T::from_f64(f64::from(n)) * power * self.deriv
        };
        Self { value, deriv }
    }

    /// The pair-primitive payoff: **one** `sin_cos` of the value serves
    /// all four channels — `sin` is `(s, c·a')` and `cos` is
    /// `(c, −(s·a'))`. Associations fixed as `c · a'` and the negation of
    /// `s · a'` (negation is exact at both base scalars, so the placement
    /// is a documented convention, not a rounding choice). The value pair
    /// is `T::sin_cos` verbatim; since [`Real`] requires any overridden
    /// `T::sin`/`T::cos` to be bit-identical to the pair's projections,
    /// the value-channel contract holds against those too.
    fn sin_cos(self) -> (Self, Self) {
        let (s, c) = self.value.sin_cos();
        (
            Self {
                value: s,
                deriv: c * self.deriv,
            },
            Self {
                value: c,
                deriv: -(s * self.deriv),
            },
        )
    }

    /// `(tan a, (1 + t.powi(2))·a')` where `t` is the value-channel
    /// tangent — the `1 + tan²` form of `sec²` reuses the already-computed
    /// value instead of spending a second transcendental on `cos²`.
    /// Association fixed as `(1 + t.powi(2)) · a'`. The square is `powi(2)`,
    /// not `t·t`: bit-identical at `f64` (`powi_by_squaring` gives
    /// `1·(t·t)`, `×1` exact) but the tight `pown` at `Dual<Interval>`,
    /// where a zero-straddling tangent enclosure would otherwise lose its
    /// `[0, ·]` floor under dependent multiplication (the dependency
    /// problem). Near a pole the derivative grows with `t²`, honestly.
    fn tan(self) -> Self {
        let t = self.value.tan();
        Self {
            value: t,
            deriv: (T::one() + t.powi(2)) * self.deriv,
        }
    }

    /// `(asin a, a' / √(1 − a.powi(2)))`, association fixed as written. The
    /// square is `powi(2)`, not `a·a`: bit-identical at `f64`
    /// (`powi_by_squaring` gives `1·(a·a)`, `×1` exact) but the tight
    /// `pown` at `Dual<Interval>`, where a zero-straddling value enclosure
    /// squared by dependent multiplication would lose its `[0, ·]` floor
    /// (the dependency problem) and slacken `1 − a²` — and hence the whole
    /// derivative enclosure — for no reason. At the domain edges `a = ±1`
    /// the denominator is zero — derivative channel poison (the kink at the
    /// domain edge); beyond them `T::asin` and the square root both poison,
    /// so both channels carry it.
    fn asin(self) -> Self {
        let denom = (T::one() - self.value.powi(2)).sqrt();
        Self {
            value: self.value.asin(),
            deriv: self.deriv / denom,
        }
    }

    /// `(acos a, −(a' / √(1 − a.powi(2))))` — [`Real::asin`]'s derivative
    /// negated (negation exact, placement documented). Same tight-square
    /// reasoning as [`Real::asin`]: `powi(2)` is bit-identical to `a·a` at
    /// `f64` but keeps `1 − a²` tight for a zero-straddling enclosure at
    /// `Dual<Interval>`.
    fn acos(self) -> Self {
        let denom = (T::one() - self.value.powi(2)).sqrt();
        Self {
            value: self.value.acos(),
            deriv: -(self.deriv / denom),
        }
    }

    /// `(atan a, a' / (1 + a.powi(2)))`, association fixed as written.
    /// Total on all of ℝ like the value channel. The square is `powi(2)`,
    /// not `a·a`: bit-identical at `f64` (`powi_by_squaring` gives
    /// `1·(a·a)`, `×1` exact) but the tight `pown` at `Dual<Interval>` —
    /// this is the load-bearing case for the stackup loop this type exists
    /// for. Over a value enclosure straddling zero, dependent `a·a` returns
    /// a lower bound below `0` (the dependency problem), so `1 + a²` can
    /// straddle zero, the division blows the derivative enclosure to
    /// `[−∞, ∞]`; the tight `pown` keeps `a² ≥ 0`, hence `1 + a² ≥ 1`, and
    /// the enclosure stays bounded.
    fn atan(self) -> Self {
        Self {
            value: self.value.atan(),
            deriv: self.deriv / (T::one() + self.value.powi(2)),
        }
    }

    /// `(atan2(y, x), (x·y' − y·x') / (x.powi(2) + y.powi(2)))` with
    /// `self` = y, association fixed as written (numerator fully evaluated,
    /// then one division). The denominator squares are `powi(2)`, not
    /// `x·x`/`y·y`: bit-identical at `f64` (`powi_by_squaring` gives
    /// `1·(x·x)`, `×1` exact) but the tight `pown` at `Dual<Interval>`.
    /// This is load-bearing for the stackup loop: with dependent
    /// multiplication, `x·x` and `y·y` each dip below zero over a
    /// zero-straddling enclosure (the dependency problem), so their sum
    /// `x² + y²` can straddle zero and blow the derivative enclosure to
    /// `[−∞, ∞]`; the tight `pown` keeps each square `≥ 0`, so the sum
    /// stays `≥ 0` and (away from the origin) bounded away from it. At the
    /// origin the denominator is zero — derivative poison, matching the
    /// value channel's own origin behavior (`f64` returns the IEEE
    /// conventional angle but the derivative is honestly undefined;
    /// intervals degrade their decoration).
    fn atan2(self, x: Self) -> Self {
        Self {
            value: self.value.atan2(x.value),
            deriv: (x.value * self.deriv - self.value * x.deriv)
                / (x.value.powi(2) + self.value.powi(2)),
        }
    }

    /// Value channel: `T::min` verbatim (f64 ties keep `self` and NaN
    /// propagates; intervals take the envelope). Derivative channel:
    /// [`KinkJacobian::min_deriv`] — follows the chosen argument at `f64`,
    /// hulls both tangents in an interval tie region (module-doc kink
    /// conventions).
    fn min(self, other: Self) -> Self {
        Self {
            value: self.value.min(other.value),
            deriv: self.value.min_deriv(self.deriv, other.value, other.deriv),
        }
    }

    /// Value channel: `T::max` verbatim; derivative channel:
    /// [`KinkJacobian::max_deriv`]. Same contract as [`Real::min`],
    /// mirrored.
    fn max(self, other: Self) -> Self {
        Self {
            value: self.value.max(other.value),
            deriv: self.value.max_deriv(self.deriv, other.value, other.deriv),
        }
    }
}

/// Span selection by **value channel**, verbatim — the ratified kink
/// convention applied to the knot lattice (see
/// [`crate::spline::locate`]'s module docs): at a knot, `Dual` selects
/// the span the base scalar's tie-break selects and differentiates the
/// program as evaluated (the same branch-consistency rule as `floor`'s
/// plateau derivative and `copysign`'s locally-constant σ). Multi-span
/// enclosure combination hulls the two channels **independently** —
/// each channel's hull is that channel's honest enclosure; a `min`/
/// `max`-style selection would follow one branch's tangent and drop the
/// other's.
impl<T> crate::spline::SpanLocate for Dual<T>
where
    T: crate::spline::SpanLocate + KinkJacobian,
{
    fn locate_spans(self, knots: &crate::spline::KnotVector) -> crate::spline::SpanSet {
        self.value.locate_spans(knots)
    }

    fn enclosure_hull(self, other: Self) -> Self {
        Self {
            value: self.value.enclosure_hull(other.value),
            deriv: self.deriv.enclosure_hull(other.deriv),
        }
    }
}

/// Decide-by-value: classification delegates to the **value part only**;
/// the derivative never influences a branch (the module-doc rationale:
/// derivatives are tangent-space data, topology is decided in the base
/// space). This is the resolution of Q1's residue item on
/// `Dual<Interval>` comparison semantics — `Dual<f64>` inherits `f64`'s
/// total-with-escalation classification, `Dual<Interval>` inherits the
/// certified enclosure classification (decoration poison, straddle ⇒
/// [`Indeterminate`] ⇒ subdivision), with no dual-specific decision
/// design at all. A poisoned or astronomically large derivative on a
/// cleanly classifiable value classifies cleanly (under test): bad
/// sensitivity is M6's stackup machinery's news to report, not a
/// topology question.
impl<T> Decide for Dual<T>
where
    T: Decide + KinkJacobian,
{
    fn sign_within(self, band: Band) -> Result<Sign, Indeterminate> {
        self.value.sign_within(band)
    }
}

/// **The bracket is the value channel; the tangent is not in it.**
///
/// Wave 0 decision **D1** of `docs/SMELL-SCAN-2026-08.md`, ruled by Evan
/// 2026-08-19: *a `Dual` may not certify — at least for now — but it may
/// have [`Bounds`].* The two halves are separable only because #643
/// (W1c/S41) split them into two traits; this takes the first and leaves
/// the second alone.
///
/// # The definition is read off an existing contract
///
/// Since #643, [`Bounds`] asks one question: *what bracket does this
/// value carry?* The module contract above already answers it — the value
/// channel of a `Dual<T>` build **is** the plain-`T` build, bit-identically
/// (D9's dual contract) — so whatever brackets the plain run brackets the
/// dual run's value, with nothing to re-establish per operation. Hence
/// **delegation** rather than a restated bracket: the degenerate
/// `lo = hi = value` at `T = f64`, the value enclosure's own endpoints at
/// `T = Interval`, and poison surfacing as NaN from both accessors because
/// the base scalar's accessors do.
///
/// # Why the tangent is excluded — an EXTENSION of E9, not a reading of it
///
/// Labelled an extension deliberately. `docs/ERROR-DESIGN.md` **E9**
/// (*tangent poison never refuses*) is scoped to **leaf refusal** —
/// "refusal is decided solely by value-channel predicates and
/// W-certificates" — and says nothing about [`Bounds::lo`]/[`Bounds::hi`],
/// which did not exist for a dual when it was written. Calling that a
/// ratification would be the exact move S44 exists to complain about.
///
/// The extension: E9's principle is that the derivative channel must never
/// make the value channel's verdict worse. Hulling the tangent into
/// `[lo, hi]` does precisely that — a NaN or unbounded tangent, which E4
/// expects at a kink and `copysign`'s straddle rule mints deliberately,
/// would poison a bracket the value channel is entitled to, and every
/// `Decide + Bounds` seam that refuses on a bracket would then refuse a
/// dual run its `f64` run passes. Same failure mode, different accessor,
/// so the same answer is the consistent one. [`Decide`] for `Dual` settled
/// the identical question by value-part delegation (Q1 residue, PR
/// #9/#10) and `is_poison` is value-only; a tangent-reading `Bounds` would
/// be the one accessor of three that disagrees. The derivative stays where
/// it is read: the public fields, under the same L7 scope as [`Bounds`].
///
/// **The cost, said out loud because E4 would otherwise rediscover it.**
/// E9 pairs "never refuses" with a **forfeiture** half (`per_param`/`rss`
/// entries report `UnavailableBecause`, E5). `Bounds` carries **no signal**
/// that the tangent is degraded — `lo()`/`hi()` are the value channel's
/// whether the tangent is `1.0` or NaN. Intended, but it means E4 must
/// read the tangent through the public fields; degradation cannot be
/// recovered from a bracket after the fact.
///
/// # This grants no certification right
///
/// [`crate::CertifiedEnclosure`] is deliberately unimplemented for `Dual`
/// and this impl does not change that: every C9-ring door is bounded by it
/// and stays uninstantiable at a dual. What opens is the bracket half —
/// boxes, pruning, the `f64` margin payloads a typed refusal reports,
/// and **selections**, which are the ones with a condition on them: a
/// frozen `f64` choice is free of the tangent only while the quantity
/// chosen is locally constant in the input. `geom::projection::mid` is
/// where that condition currently fails, and is issue **#874**.
/// One `Decide + Bounds` door *grants* without that guard
/// (`topo::separation`, sound at a dual by delegation); the scope rule in
/// `real.rs` is the home for both that and the fillet seam's obligation.
///
/// # On the spelling
///
/// `Self: Real` (the supertrait, which for a dual means
/// `T: KinkJacobian`) plus a **sole** `Bounds` on the base scalar, rather
/// than the file's usual `T: X + KinkJacobian`. It says the obligation
/// directly: a dual brackets iff it is a `Real` over a bracket-carrying
/// scalar.
///
/// It is **not** thereby "satisfied by construction": in the equivalent
/// spelling `T: Bounds + KinkJacobian` this fires
/// `scripts/gates/bounds-allowlist.sh`, which does not allowlist this
/// file. It is satisfied by the RULE, not by the grep. That is written up
/// once, in the gate's header (KNOWN GAP 2), which declares this the
/// sanctioned spelling of this one impl and pins the evasion with a
/// self-test case.
impl<T> Bounds for Dual<T>
where
    Self: Real,
    T: Bounds,
{
    fn lo(self) -> f64 {
        self.value.lo()
    }

    fn hi(self) -> f64 {
        self.value.hi()
    }
}

/// Forward-mode dual over the default scalar: fast, uncertified
/// sensitivities.
pub type Dual64 = Dual<f64>;

/// Forward-mode dual over the certified interval scalar: derivative
/// *enclosures* riding on enclosure values (the instantiation that never
/// existed off the shelf — see the module-doc deviation note).
#[cfg(feature = "interval")]
pub type DualInterval = Dual<Interval>;

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use proptest::prelude::*;

    use super::*;
    use crate::predicate::MarginDiag;

    // Global-state discipline (see `crate::tolerance`'s test module): all
    // bands here are built purely via `Band::new`, never via the
    // tolerance-coupled constructors, so nothing in this module touches
    // the global `Tolerance`.

    /// The fixed pure band (zero = 1e-9, escalate = 1e-8) used by the
    /// Decide-delegation tests — identical to `predicate.rs`'s.
    fn band_1e9() -> Band {
        Band::new(1e-9, 1e-8).unwrap()
    }

    /// Distance in ulps between two finite f64s (same monotone integer
    /// mapping as `real.rs`'s test helper; +0.0 and -0.0 both map to 0).
    fn ulp_dist(a: f64, b: f64) -> u64 {
        fn ord(x: f64) -> i64 {
            let i = x.to_bits() as i64;
            if i < 0 { i64::MIN.wrapping_sub(i) } else { i }
        }
        u64::try_from((i128::from(ord(a)) - i128::from(ord(b))).unsigned_abs())
            .expect("ulp distance exceeds u64 — inputs were not comparable finite values")
    }

    /// The same generic consumer `real.rs` and `interval.rs` use —
    /// evaluation code written against `Real` alone, here instantiated at
    /// dual types.
    fn hypotenuse<T: Real>(a: T, b: T) -> T {
        (a * a + b * b).sqrt()
    }

    /// The composition-smoke pipeline: distance from the point at angle
    /// `theta` on the circle of radius `r` (centered at the origin) to the
    /// fixed point `(cx, cy)` — a small but real evaluation recipe
    /// (sin_cos, mul, sub, sqrt) generic over `Real`.
    fn circle_distance<T: Real>(theta: T, r: T, cx: T, cy: T) -> T {
        let (s, c) = theta.sin_cos();
        let dx = r * c - cx;
        let dy = r * s - cy;
        (dx * dx + dy * dy).sqrt()
    }

    /// The analytic d/dθ of [`circle_distance`], written generically so
    /// the f64 test compares against the libm-backed scalar computation
    /// and the interval test gets an independent enclosure of the truth:
    /// D' = ((r·c − cx)·(−r·s) + (r·s − cy)·(r·c)) / D.
    fn circle_distance_dtheta<T: Real>(theta: T, r: T, cx: T, cy: T) -> T {
        let (s, c) = theta.sin_cos();
        let dx = r * c - cx;
        let dy = r * s - cy;
        let d = (dx * dx + dy * dy).sqrt();
        (dx * (-(r * s)) + dy * (r * c)) / d
    }

    /// Central difference (f(x+h) − f(x−h)) / 2h with h scaled to x.
    ///
    /// Error model (the justification for the loose bounds below):
    /// truncation is h²·|f‴|/6 and rounding is ~ε·|f|/h; with h = 1e-5
    /// times the argument scale both terms sit near 1e-10–1e-11 for the
    /// O(1)-conditioned functions and domains these tests use, so a
    /// tolerance of 1e-6·(1 + |expected|) is ~4 orders of magnitude of
    /// headroom while still catching any wrong chain rule (which is off
    /// by O(1) or by a factor).
    fn central_diff(f: impl Fn(f64) -> f64, x: f64) -> f64 {
        let h = 1e-5 * x.abs().max(1.0);
        (f(x + h) - f(x - h)) / (2.0 * h)
    }

    /// Loose agreement bound for central-difference checks (see
    /// [`central_diff`]'s error model).
    fn close_to_cd(dual: f64, cd: f64) -> bool {
        (dual - cd).abs() <= 1e-6 * (1.0 + cd.abs())
    }

    // ------------------------------------------------------------------
    // THE contract: value-channel bit-identity with plain f64
    // ------------------------------------------------------------------

    /// Named unary ops as (f64 computation, dual computation) pairs. Every
    /// unary `Real` operation appears; `sin_cos` is handled separately
    /// (tuple result) and `powi` separately (extra parameter).
    type UnaryPair = (&'static str, fn(f64) -> f64, fn(Dual64) -> Dual64);

    fn unary_ops() -> Vec<UnaryPair> {
        vec![
            ("neg", |x| -x, |x| -x),
            ("sqrt", Real::sqrt, Real::sqrt),
            ("abs", Real::abs, Real::abs),
            ("sin", Real::sin, Real::sin),
            ("cos", Real::cos, Real::cos),
            ("tan", Real::tan, Real::tan),
            ("asin", Real::asin, Real::asin),
            ("acos", Real::acos, Real::acos),
            ("atan", Real::atan, Real::atan),
            ("floor", Real::floor, Real::floor),
        ]
    }

    /// Named binary ops, same shape.
    type BinaryPair = (
        &'static str,
        fn(f64, f64) -> f64,
        fn(Dual64, Dual64) -> Dual64,
    );

    /// A central-difference case: name, f64 computation, dual computation,
    /// sample point.
    type CdCase = (&'static str, fn(f64) -> f64, fn(Dual64) -> Dual64, f64);

    fn binary_ops() -> Vec<BinaryPair> {
        vec![
            ("add", |x, y| x + y, |x, y| x + y),
            ("sub", |x, y| x - y, |x, y| x - y),
            ("mul", |x, y| x * y, |x, y| x * y),
            ("div", |x, y| x / y, |x, y| x / y),
            ("atan2", Real::atan2, Real::atan2),
            ("min", Real::min, Real::min),
            ("max", Real::max, Real::max),
            ("copysign", Real::copysign, Real::copysign),
            (
                "reduce_periodic",
                Real::reduce_periodic,
                Real::reduce_periodic,
            ),
        ]
    }

    /// Bit-identity across poison and special values: NaN, ±0, ±∞ through
    /// every unary op. NaN payload comparison via `to_bits` is sound here
    /// because both sides make the *same* underlying call on the same
    /// input — that is the point of the contract.
    #[test]
    fn value_channel_bit_identity_special_values() {
        let specials = [0.0, -0.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY];
        for &x in &specials {
            for (name, f, fd) in unary_ops() {
                assert_eq!(
                    fd(Dual::variable(x)).value.to_bits(),
                    f(x).to_bits(),
                    "{name}({x:?})"
                );
            }
            let (s, c) = Real::sin_cos(x);
            let (ds, dc) = Dual::variable(x).sin_cos();
            assert_eq!(ds.value.to_bits(), s.to_bits(), "sin_cos.0({x:?})");
            assert_eq!(dc.value.to_bits(), c.to_bits(), "sin_cos.1({x:?})");
            for n in [-3, -1, 0, 1, 2, 7] {
                assert_eq!(
                    Dual::variable(x).powi(n).value.to_bits(),
                    <f64 as Real>::powi(x, n).to_bits(),
                    "powi({x:?}, {n})"
                );
            }
            for &y in &specials {
                for (name, f, fd) in binary_ops() {
                    assert_eq!(
                        fd(Dual::variable(x), Dual::variable(y)).value.to_bits(),
                        f(x, y).to_bits(),
                        "{name}({x:?}, {y:?})"
                    );
                }
            }
        }
    }

    proptest! {
        /// THE contract, property-tested: for every operation the dual's
        /// value part is bit-identical to the plain-f64 computation,
        /// regardless of what rides in the derivative slot.
        #[test]
        fn value_channel_bit_identity(
            x in -1.0e3..1.0e3f64,
            y in -1.0e3..1.0e3f64,
            dx in -10.0..10.0f64,
            dy in -10.0..10.0f64,
            n in -8..=8i32,
        ) {
            let a = Dual::new(x, dx);
            let b = Dual::new(y, dy);
            for (name, f, fd) in unary_ops() {
                prop_assert_eq!(fd(a).value.to_bits(), f(x).to_bits(), "{}", name);
            }
            let (s, c) = Real::sin_cos(x);
            let (ds, dc) = a.sin_cos();
            prop_assert_eq!(ds.value.to_bits(), s.to_bits());
            prop_assert_eq!(dc.value.to_bits(), c.to_bits());
            prop_assert_eq!(
                a.powi(n).value.to_bits(),
                <f64 as Real>::powi(x, n).to_bits()
            );
            for (name, f, fd) in binary_ops() {
                prop_assert_eq!(fd(a, b).value.to_bits(), f(x, y).to_bits(), "{}", name);
            }
        }

        /// The trait's projection contract at `Dual64`: `sin`/`cos` (the
        /// inherited defaults) are bit-identical to the components of
        /// `sin_cos`, in both channels.
        #[test]
        fn sin_cos_projections_bit_identical(
            x in -1.0e3..1.0e3f64,
            d in -10.0..10.0f64,
        ) {
            let a = Dual::new(x, d);
            let (s, c) = a.sin_cos();
            prop_assert_eq!(Real::sin(a).value.to_bits(), s.value.to_bits());
            prop_assert_eq!(Real::sin(a).deriv.to_bits(), s.deriv.to_bits());
            prop_assert_eq!(Real::cos(a).value.to_bits(), c.value.to_bits());
            prop_assert_eq!(Real::cos(a).deriv.to_bits(), c.deriv.to_bits());
        }
    }

    // ------------------------------------------------------------------
    // Derivative correctness against central differences (loose bounds)
    // ------------------------------------------------------------------

    proptest! {
        /// Smooth unary chain rules vs central differences, on domains
        /// where each function is well-conditioned (see [`central_diff`]
        /// for the error model behind the bound). `tan` stays inside
        /// (−1.4, 1.4) — away from the ±π/2 poles where f‴ explodes —
        /// and `asin`/`acos` inside (−0.9, 0.9) for the same reason.
        ///
        /// The seed `d` is a random nonzero tangent (not the unit seed):
        /// the chain rule multiplies the slope by `d`, so the check is
        /// `deriv ≈ d·slope`. This is what catches a mutant that drops the
        /// `·a'` seed factor from an op — a unit seed makes `d·slope` and
        /// `slope` identical and hides it.
        #[test]
        fn unary_derivatives_match_central_differences(
            x_trig in -1.35..1.35f64,
            x_pos in 0.1..100.0f64,
            x_wide in -50.0..50.0f64,
            d in prop_oneof![-1000.0..-0.001f64, 0.001..1000.0f64],
        ) {
            let cases: &[CdCase] = &[
                ("sin", Real::sin, Real::sin, x_wide),
                ("cos", Real::cos, Real::cos, x_wide),
                ("tan", Real::tan, Real::tan, x_trig),
                ("asin", Real::asin, Real::asin, 0.6 * x_trig),
                ("acos", Real::acos, Real::acos, 0.6 * x_trig),
                ("atan", Real::atan, Real::atan, x_wide),
                ("sqrt", Real::sqrt, Real::sqrt, x_pos),
                ("abs", Real::abs, Real::abs, x_pos + 1.0),
            ];
            for &(name, f, fd, x) in cases {
                let dual = fd(Dual::new(x, d)).deriv;
                let cd = central_diff(f, x);
                prop_assert!(
                    close_to_cd(dual, d * cd),
                    "{}'({}) with seed {} = {} but d·(central difference) = {}",
                    name, x, d, dual, d * cd
                );
            }
        }

        /// powi's `n·xⁿ⁻¹` rule vs central differences, positive and
        /// negative exponents, base away from zero (negative exponents
        /// are ill-conditioned there). The random nonzero seed `d` makes
        /// the check `deriv ≈ d·slope`, catching a mutant that drops the
        /// `·a'` seed factor (a unit seed would hide it).
        #[test]
        fn powi_derivative_matches_central_differences(
            x in 0.2..3.0f64,
            neg in any::<bool>(),
            n in -6..=6i32,
            d in prop_oneof![-1000.0..-0.001f64, 0.001..1000.0f64],
        ) {
            let x = if neg { -x } else { x };
            let dual = Dual::new(x, d).powi(n).deriv;
            let cd = central_diff(|t| <f64 as Real>::powi(t, n), x);
            // close_to_cd's relative-to-|expected| bound already scales
            // with magnitude: |x|⁻⁷ at x = 0.2 reaches ~8e4, and the
            // 1e-6·(1 + |d·cd|) floor grows with the seeded expected value
            // — the same headroom as everywhere else, so reuse the helper.
            prop_assert!(
                close_to_cd(dual, d * cd),
                "d/dx x^{} at {} with seed {} = {} but d·(central difference) = {}",
                n, x, d, dual, d * cd
            );
        }

        /// Both partial derivatives of atan2 vs central differences
        /// (seeding y then x as the variable), quadrant-independent.
        #[test]
        fn atan2_partials_match_central_differences(
            y in 0.1..10.0f64,
            x in 0.1..10.0f64,
            sy in any::<bool>(),
            sx in any::<bool>(),
        ) {
            let y = if sy { -y } else { y };
            let x = if sx { -x } else { x };
            let d_dy = Real::atan2(Dual::variable(y), Dual::constant(x)).deriv;
            let cd_y = central_diff(|t| Real::atan2(t, x), y);
            prop_assert!(close_to_cd(d_dy, cd_y), "∂/∂y atan2({y}, {x})");
            let d_dx = Real::atan2(Dual::constant(y), Dual::variable(x)).deriv;
            let cd_x = central_diff(|t| Real::atan2(y, t), x);
            prop_assert!(close_to_cd(d_dx, cd_x), "∂/∂x atan2({y}, {x})");
        }

        /// Product, quotient, and composite rules through a nontrivial
        /// rational-trig pipeline vs central differences. The random
        /// nonzero seed `d` rides the whole chain (`deriv ≈ d·slope`), so
        /// a mutant dropping the `·a'` seed factor from the `sin_cos`
        /// inside is caught (a unit seed would hide it).
        #[test]
        fn composite_derivative_matches_central_differences(
            x in -3.0..3.0f64,
            d in prop_oneof![-1000.0..-0.001f64, 0.001..1000.0f64],
        ) {
            fn pipeline<T: Real>(x: T) -> T {
                let (s, _) = x.sin_cos();
                (x * x + T::one()) / (x * s + T::from_f64(2.5))
            }
            // Keep the denominator away from zero (it is ≥ 2.5 − |x·sin x|
            // and can vanish near |x| ≈ 2.97): condition on margin.
            prop_assume!((x * Real::sin(x) + 2.5).abs() > 0.5);
            let dual = pipeline(Dual::new(x, d)).deriv;
            let cd = central_diff(pipeline::<f64>, x);
            prop_assert!(
                close_to_cd(dual, d * cd),
                "pipeline'({}) with seed {} = {} vs d·cd {}",
                x, d, dual, d * cd
            );
        }
    }

    // ------------------------------------------------------------------
    // Kink semantics at f64
    // ------------------------------------------------------------------

    /// abs at and around zero: the ratified subgradient convention —
    /// factor +1 at both signed zeros (representation artifact), ±1 by
    /// sign elsewhere, NaN propagating through the tangent.
    #[test]
    fn abs_kink_convention_at_f64() {
        // Both signed zeros take the +1 arm; value is +0.0 bitwise (abs
        // clears the sign bit).
        for z in [0.0f64, -0.0] {
            let d = Dual::new(z, 3.0).abs();
            assert_eq!(d.value.to_bits(), 0.0f64.to_bits(), "abs({z:?}) value");
            assert_eq!(d.deriv, 3.0, "abs'({z:?}) keeps the +1 factor");
        }
        assert_eq!(Dual::new(2.0, 3.0).abs().deriv, 3.0);
        assert_eq!(Dual::new(-2.0, 3.0).abs().deriv, -3.0);
        // NaN value poisons both channels.
        let poisoned = Dual::new(f64::NAN, 3.0).abs();
        assert!(poisoned.value.is_nan());
        assert!(poisoned.deriv.is_nan());
        // A NaN tangent stays NaN through the factor multiply.
        assert!(Dual::new(2.0, f64::NAN).abs().deriv.is_nan());
    }

    /// min/max ties keep `self` in BOTH channels — the PR 2 contract
    /// ("ties keep self; the derivative follows the chosen argument").
    #[test]
    fn min_max_ties_keep_self_and_derivative_follows() {
        let a = Dual::new(1.0, 7.0);
        let b = Dual::new(1.0, -7.0);
        // Ties keep self: value a.value, tangent a.deriv — for BOTH ops.
        assert_eq!(Real::min(a, b).deriv, 7.0);
        assert_eq!(Real::max(a, b).deriv, 7.0);
        // Strict order: the winning argument's tangent, either position.
        let lo = Dual::new(1.0, 10.0);
        let hi = Dual::new(2.0, 20.0);
        assert_eq!(Real::min(lo, hi).deriv, 10.0);
        assert_eq!(Real::min(hi, lo).deriv, 10.0);
        assert_eq!(Real::max(lo, hi).deriv, 20.0);
        assert_eq!(Real::max(hi, lo).deriv, 20.0);
        // Signed-zero ties are position-decided (value channel keeps
        // self's zero bitwise) and the tangent follows the same choice.
        let pz = Dual::new(0.0, 1.0);
        let nz = Dual::new(-0.0, 2.0);
        assert_eq!(Real::min(pz, nz).value.to_bits(), 0.0f64.to_bits());
        assert_eq!(Real::min(pz, nz).deriv, 1.0);
        assert_eq!(Real::min(nz, pz).value.to_bits(), (-0.0f64).to_bits());
        assert_eq!(Real::min(nz, pz).deriv, 2.0);
    }

    /// NaN semantics of min/max: a NaN *value* poisons both channels; a
    /// NaN tangent on the UNCHOSEN argument is discarded (min is locally
    /// the chosen branch — the loud channel for the unchosen branch's
    /// troubles is its own value), while a NaN tangent on the chosen
    /// argument rides through.
    #[test]
    fn min_max_nan_semantics() {
        let clean = Dual::new(1.0, 5.0);
        let nan_value = Dual::new(f64::NAN, 0.0);
        for (name, result) in [
            ("min", Real::min(clean, nan_value)),
            ("min-flipped", Real::min(nan_value, clean)),
            ("max", Real::max(clean, nan_value)),
            ("max-flipped", Real::max(nan_value, clean)),
        ] {
            assert!(result.value.is_nan(), "{name}: value");
            assert!(result.deriv.is_nan(), "{name}: deriv");
        }
        // Unchosen NaN tangent discarded: min picks the clean branch.
        let nan_tangent = Dual::new(2.0, f64::NAN);
        let picked = Real::min(clean, nan_tangent);
        assert_eq!(picked.value, 1.0);
        assert_eq!(picked.deriv, 5.0);
        // Chosen NaN tangent rides through.
        let picked = Real::max(clean, nan_tangent);
        assert_eq!(picked.value, 2.0);
        assert!(picked.deriv.is_nan());
    }

    /// floor's step convention at f64: derivative exactly 0 on plateaus
    /// AND at integers (branch-consistency — the program as evaluated
    /// sits on the right-continuous plateau), NaN propagating.
    #[test]
    fn floor_step_convention_at_f64() {
        // Plateau interior: value floors, tangent is an exact zero
        // whatever the seed.
        let d = Dual::new(2.7, 5.0).floor();
        assert_eq!(d.value, 2.0);
        assert_eq!(d.deriv.to_bits(), 0.0f64.to_bits());
        // AT an integer: still the plateau's 0 (the ratified pick).
        let at_int = Dual::new(3.0, 5.0).floor();
        assert_eq!(at_int.value, 3.0);
        assert_eq!(at_int.deriv.to_bits(), 0.0f64.to_bits());
        // Negative side: floor(-2.3) = -3, tangent 0.
        let neg = Dual::new(-2.3, 5.0).floor();
        assert_eq!(neg.value, -3.0);
        assert_eq!(neg.deriv.to_bits(), 0.0f64.to_bits());
        // NaN value poisons both channels.
        let poisoned = Dual::new(f64::NAN, 5.0).floor();
        assert!(poisoned.value.is_nan());
        assert!(poisoned.deriv.is_nan());
        // A NaN tangent stays NaN through the 0-factor multiply (0·NaN).
        assert!(Dual::new(2.5, f64::NAN).floor().deriv.is_nan());
    }

    /// copysign's kink conventions at f64: σ(s)·abs′(x) selects the
    /// tangent factor; the sign argument's tangent is discarded (σ is
    /// locally constant); NaN in either VALUE poisons both channels.
    #[test]
    fn copysign_kink_convention_at_f64() {
        // Positive x, negative s: factor σ·abs′ = (−1)(+1) = −1.
        let d = Real::copysign(Dual::new(3.0, 7.0), Dual::new(-2.0, 100.0));
        assert_eq!(d.value, -3.0);
        assert_eq!(d.deriv, -7.0);
        // Negative x, negative s: (−1)(−1) = +1 — no flip.
        let d = Real::copysign(Dual::new(-3.0, 7.0), Dual::new(-2.0, 0.0));
        assert_eq!(d.value, -3.0);
        assert_eq!(d.deriv, 7.0);
        // Sign tangent discarded even when poisoned: σ is locally
        // constant (the min-unchosen-branch discard rule).
        let d = Real::copysign(Dual::new(3.0, 7.0), Dual::new(2.0, f64::NAN));
        assert_eq!(d.value, 3.0);
        assert_eq!(d.deriv, 7.0);
        // x = 0 takes abs's +1-at-zero pick: factor σ·(+1).
        let d = Real::copysign(Dual::new(0.0, 7.0), Dual::new(-1.0, 0.0));
        assert_eq!(d.value.to_bits(), (-0.0f64).to_bits());
        assert_eq!(d.deriv, -7.0);
        // s = −0.0 transfers its sign BIT (documented) and σ = −1.
        let d = Real::copysign(Dual::new(3.0, 7.0), Dual::new(-0.0, 0.0));
        assert_eq!(d.value, -3.0);
        assert_eq!(d.deriv, -7.0);
        // NaN sign VALUE poisons both channels (the trait's both-argument
        // poison guard, inherited verbatim in the value channel).
        let d = Real::copysign(Dual::new(3.0, 7.0), Dual::new(f64::NAN, 0.0));
        assert!(d.value.is_nan());
        assert!(d.deriv.is_nan());
        let d = Real::copysign(Dual::new(f64::NAN, 0.0), Dual::new(1.0, 0.0));
        assert!(d.value.is_nan());
        assert!(d.deriv.is_nan());
    }

    /// reduce_periodic differentiates through its compositional body:
    /// away from period boundaries the derivative w.r.t. the input is
    /// exactly the seed (slope 1 — floor contributes 0), and the
    /// derivative w.r.t. the period is −floor(x/p) (the plateau index).
    #[test]
    fn reduce_periodic_derivatives() {
        use core::f64::consts::TAU;
        // d/dθ: slope 1 scaled by the seed.
        let d = Real::reduce_periodic(Dual::new(1.5 + TAU, 3.0), Dual::constant(TAU));
        assert!((d.value - 1.5).abs() <= 1e-15);
        assert_eq!(d.deriv, 3.0);
        // d/dp of (x mod p) = −⌊x/p⌋ away from kinks: x = 7, p = 2 ⇒ −3.
        let d = Real::reduce_periodic(Dual::constant(7.0), Dual::variable(2.0));
        assert_eq!(d.value, 1.0);
        assert_eq!(d.deriv, -3.0);
    }

    /// sqrt's domain-edge kink: value 0 keeps the exact zero while the
    /// tangent poisons itself to +∞ (division by zero); negative values
    /// poison both channels.
    #[test]
    fn sqrt_kink_at_zero() {
        let at_zero = Dual::variable(0.0).sqrt();
        assert_eq!(at_zero.value.to_bits(), 0.0f64.to_bits());
        assert_eq!(at_zero.deriv, f64::INFINITY);
        let negative = Dual::variable(-1.0).sqrt();
        assert!(negative.value.is_nan());
        assert!(negative.deriv.is_nan());
    }

    /// asin/acos at exactly the domain edges ±1: both channels pinned. The
    /// value channel lands on ±π/2 / {0, π} (within a couple ulps of the
    /// libm result), while the derivative channel poisons to an *infinite*
    /// tangent — the denominator √(1 − a²) is an EXACT zero (`a.powi(2)` is
    /// exactly 1 at a = ±1, 1 − 1 = 0, √0 = +0), so a'/0 = ±∞. This is the
    /// totality policy at a domain-edge kink: the vertical tangent is an
    /// honest infinity, not an error (asin rises to a vertical slope at
    /// both ends ⇒ +∞; acos falls to one ⇒ −∞), exactly as sqrt's edge
    /// kink poisons its own tangent rather than trapping.
    #[test]
    fn asin_acos_at_unit_edges() {
        use core::f64::consts::{FRAC_PI_2, PI};
        // asin: value ≈ ±π/2, derivative +∞ at both edges (increasing,
        // vertical tangent). deriv seed +1 divided by exact +0.
        let hi = Dual::variable(1.0).asin();
        assert!(ulp_dist(hi.value, FRAC_PI_2) <= 1, "asin(1) value ≈ π/2");
        assert_eq!(hi.deriv, f64::INFINITY, "asin'(1) = +∞");
        let lo = Dual::variable(-1.0).asin();
        assert!(ulp_dist(lo.value, -FRAC_PI_2) <= 1, "asin(-1) value ≈ -π/2");
        assert_eq!(lo.deriv, f64::INFINITY, "asin'(-1) = +∞");
        // acos: value ≈ 0 / π, derivative −∞ at both edges (decreasing) —
        // asin's tangent negated.
        let hi = Dual::variable(1.0).acos();
        assert!(ulp_dist(hi.value, 0.0) <= 1, "acos(1) value ≈ 0");
        assert_eq!(hi.deriv, f64::NEG_INFINITY, "acos'(1) = -∞");
        let lo = Dual::variable(-1.0).acos();
        assert!(ulp_dist(lo.value, PI) <= 1, "acos(-1) value ≈ π");
        assert_eq!(lo.deriv, f64::NEG_INFINITY, "acos'(-1) = -∞");
    }

    // ------------------------------------------------------------------
    // powi: the n = 0 cases and the i32::MIN edge
    // ------------------------------------------------------------------

    /// The n = 0 clauses together: the tangent is an EXACT zero (the
    /// derivative of the constant-one function) for every describable
    /// value, while poison in EITHER channel is conserved — the value
    /// channel keeps f64's poison guard (NaN⁰ is NaN, not 1) and the
    /// tangent is the value-tainted zero of `powi_zero_deriv_factor`
    /// times the input tangent (#126 option (a)).
    #[test]
    fn powi_zero_exponent_channels() {
        // Healthy input: (x⁰, 0·) — value exactly 1, tangent exactly +0.
        let healthy = Dual::new(2.5, 7.0).powi(0);
        assert_eq!(healthy.value.to_bits(), 1.0f64.to_bits());
        assert_eq!(healthy.deriv.to_bits(), 0.0f64.to_bits());
        // x = 0: value 0⁰ = 1 (not poison), tangent exactly zero — the
        // naive 0·x⁻¹ would have manufactured NaN here.
        let at_zero = Dual::variable(0.0).powi(0);
        assert_eq!(at_zero.value, 1.0);
        assert_eq!(at_zero.deriv.to_bits(), 0.0f64.to_bits());
        // ±∞: value (±∞)⁰ = 1 (not f64 poison), tangent exactly zero —
        // the factor is 0 for every describable value.
        let at_inf = Dual::variable(f64::INFINITY).powi(0);
        assert_eq!(at_inf.value, 1.0);
        assert_eq!(at_inf.deriv.to_bits(), 0.0f64.to_bits());
        // Poisoned input: BOTH channels propagate NaN (#126 option (a),
        // fixed at M5 PR 4 — the zero tangent is tainted by the value's
        // poison through `powi_zero_deriv_factor`, never a fresh
        // constant that would launder a poisoned value into a
        // classifiable derivative).
        let poisoned = Dual::variable(f64::NAN).powi(0);
        assert!(poisoned.value.is_nan());
        assert!(poisoned.deriv.is_nan());
        // A poisoned TANGENT with a clean value also stays poisoned
        // (0·NaN — same conservation as `floor`'s factor product).
        let bad_tan = Dual::new(2.0, f64::NAN).powi(0);
        assert_eq!(bad_tan.value, 1.0);
        assert!(bad_tan.deriv.is_nan());
    }

    /// The one unrepresentable exponent-minus-one: n = i32::MIN must not
    /// panic (D9 totality) and uses the documented `xⁿ / x` association.
    #[test]
    fn powi_i32_min_is_total() {
        // |x| > 1: x^MIN underflows to 0, tangent n·(0/x)·1 = ±0.
        let d = Dual::variable(2.0).powi(i32::MIN);
        assert_eq!(d.value, 0.0);
        assert_eq!(d.deriv, 0.0);
        // |x| < 1: x^MIN overflows to +∞, tangent −∞ scaled — infinite,
        // not NaN, and above all: no panic.
        let d = Dual::variable(0.5).powi(i32::MIN);
        assert_eq!(d.value, f64::INFINITY);
        assert!(d.deriv.is_infinite());
    }

    // ------------------------------------------------------------------
    // Decide delegation (the Q1 residue at f64)
    // ------------------------------------------------------------------

    /// Classification is the value part's — the derivative never
    /// influences it, adversarial channels included.
    #[test]
    fn decide_classifies_value_only() {
        let band = band_1e9();
        // Clean definite value with adversarial tangents: still definite.
        for adversarial in [f64::NAN, f64::INFINITY, 1e308, -1e308] {
            assert_eq!(
                Dual::new(1.0, adversarial).sign_within(band),
                Ok(Sign::Positive),
                "deriv = {adversarial:?}"
            );
            assert_eq!(
                Dual::new(-1.0, adversarial).sign_within(band),
                Ok(Sign::Negative),
                "deriv = {adversarial:?}"
            );
            assert_eq!(
                Dual::new(0.0, adversarial).sign_within(band),
                Ok(Sign::Zero),
                "deriv = {adversarial:?}"
            );
        }
        // In-band value: Indeterminate carrying the VALUE part as the
        // margin diagnostic, whatever the tangent.
        let err = Dual::new(5e-9, f64::NAN)
            .sign_within(band)
            .expect_err("sliver-band value must be indeterminate");
        assert_eq!(err.margin, MarginDiag::Value(5e-9));
        // Poisoned value: Invalid, even with a perfectly clean tangent.
        let err = Dual::new(f64::NAN, 1.0)
            .sign_within(band)
            .expect_err("NaN value must be indeterminate");
        assert_eq!(err.margin, MarginDiag::Invalid);
    }

    // ------------------------------------------------------------------
    // Constructors, generic consumption, composition smoke
    // ------------------------------------------------------------------

    #[test]
    fn constructors_seed_the_channels() {
        let v = Dual::variable(3.0f64);
        assert_eq!((v.value, v.deriv), (3.0, 1.0));
        let c = Dual::constant(3.0f64);
        assert_eq!((c.value, c.deriv), (3.0, 0.0));
        let e = <Dual64 as Real>::from_f64(3.0);
        assert_eq!((e.value, e.deriv), (3.0, 0.0));
        // Constants of the trait: value = T's constant, tangent = 0.
        assert_eq!(<Dual64 as Real>::pi().value, core::f64::consts::PI);
        assert_eq!(<Dual64 as Real>::pi().deriv, 0.0);
        assert_eq!(<Dual64 as Real>::tau().value, core::f64::consts::TAU);
        assert_eq!(<Dual64 as Real>::one().value, 1.0);
        assert_eq!(<Dual64 as Real>::zero().value, 0.0);
    }

    /// The existing generic consumer at `Dual64`: 3-4-5 is exact in both
    /// channels (9 + 16 = 25 exactly, √25 = 5 exactly, and the tangent
    /// 2·3·1 / (2·5) = 0.6 rounds exactly).
    #[test]
    fn generic_hypotenuse_at_dual64() {
        let h = hypotenuse(Dual::variable(3.0), Dual::constant(4.0));
        assert_eq!(h.value, 5.0);
        assert_eq!(h.deriv, 0.6);
    }

    proptest! {
        /// Composition smoke at f64: the dual derivative of the
        /// circle-point → distance pipeline matches the analytic d/dθ.
        /// Both sides are libm-backed computations of algebraically
        /// equal expressions differing only in rounding order, so the
        /// agreement bound is tight-ish (1e-10 relative) rather than
        /// central-difference loose.
        #[test]
        fn circle_distance_derivative_matches_analytic(
            theta in -3.0..3.0f64,
            r in 0.5..3.0f64,
            cx in -2.0..2.0f64,
            cy in -2.0..2.0f64,
            d in prop_oneof![-1000.0..-0.001f64, 0.001..1000.0f64],
        ) {
            // Keep the distance (the quotient's denominator) away from 0,
            // where D is nondifferentiable and the quotient ill-conditioned.
            prop_assume!(circle_distance(theta, r, cx, cy) > 0.1);
            // Seed θ with a random nonzero tangent d (not the unit seed):
            // the chain rule scales the analytic dD/dθ by d, so a mutant
            // dropping a `·a'` seed factor anywhere on the θ path is caught.
            let dual = circle_distance(
                Dual::new(theta, d),
                Dual::constant(r),
                Dual::constant(cx),
                Dual::constant(cy),
            );
            let analytic = d * circle_distance_dtheta(theta, r, cx, cy);
            prop_assert_eq!(dual.value.to_bits(),
                circle_distance(theta, r, cx, cy).to_bits());
            prop_assert!(
                (dual.deriv - analytic).abs() <= 1e-10 * (1.0 + analytic.abs()),
                "dD/dθ dual = {} vs d·analytic = {}",
                dual.deriv, analytic
            );
        }
    }

    // ------------------------------------------------------------------
    // num-dual oracle (dev-dependency, Dual<f64> derivatives only)
    // ------------------------------------------------------------------

    /// Cross-checks our chain rules against the independent `num-dual`
    /// implementation — the oracle role the crate was demoted to (module
    /// docs).
    ///
    /// **Why the value channels legitimately differ:** num-dual routes
    /// transcendentals through std's `Float` (the platform libm), ours
    /// through the `libm` crate (D9). The census in `real.rs`
    /// (`libm_vs_std_divergence_census`) measures their divergence at
    /// ≤ 4 ulps over 20k samples and *asserts* that bound rather than
    /// only reporting it — but its scope is `sin`/`cos` over −1000..1000
    /// only. The other transcendentals compared here (`tan`, `asin`,
    /// `acos`, `atan`, `atan2`, `powi`) are unmeasured, and neither side
    /// promises a bound on any of them — so transcendental *value*
    /// comparisons here are
    /// tolerance-based (generous ulp allowances) and correctly-rounded
    /// operations (`sqrt`, `+`, `−`, `·`) are asserted bit-identical.
    ///
    /// **Derivative bounds:** a chain rule multiplies the seed by factors
    /// built from those same transcendental values (sin/cos/tan of the
    /// argument), so the two implementations' derivatives differ by the
    /// few-ulp value divergence amplified by a handful of arithmetic
    /// roundings on each side — comfortably inside 64 ulps (~1e-14
    /// relative), which is still ~8 orders of magnitude tighter than the
    /// central-difference checks. Ops with structurally different
    /// derivative *routes* get wider documented allowances: num-dual
    /// computes atan2 as `atan(y/x)` with the value patched (division
    /// noise ⇒ 128 ulps) and tan as the dual quotient `sin/cos` (two
    /// std calls + a division ⇒ 128 ulps for the value too). `abs`,
    /// `min`, and `max` are deliberately absent: the kink conventions are
    /// ours (module docs), not something to outsource to an oracle with
    /// different tie/zero choices.
    mod num_dual_oracle {
        use num_dual::DualNum;

        use super::*;

        type Nd = num_dual::Dual64;

        /// Assert `ours` and `theirs` agree within `bound` ulps, treating
        /// NaN-vs-NaN as agreement (both poisoned the same input).
        fn assert_ulps(
            name: &str,
            ours: f64,
            theirs: f64,
            bound: u64,
        ) -> Result<(), TestCaseError> {
            if ours.is_nan() && theirs.is_nan() {
                return Ok(());
            }
            prop_assert!(
                !ours.is_nan() && !theirs.is_nan(),
                "{}: ours = {}, num-dual = {} (one-sided NaN)",
                name,
                ours,
                theirs
            );
            let d = ulp_dist(ours, theirs);
            prop_assert!(
                d <= bound,
                "{}: ours = {}, num-dual = {} — {} ulps apart (allowed {})",
                name,
                ours,
                theirs,
                d,
                bound
            );
            Ok(())
        }

        proptest! {
            /// Correctly-rounded ops: value channels bit-identical (both
            /// sides use the same IEEE-exact operations), derivatives
            /// within a few ulps (num-dual's quotient rule divides by a
            /// reciprocal-multiplication, ours by a single division).
            #[test]
            fn exact_ops_agree(
                x in -100.0..100.0f64,
                y in -100.0..100.0f64,
                dx in -10.0..10.0f64,
                dy in -10.0..10.0f64,
            ) {
                let (a, b) = (Dual::new(x, dx), Dual::new(y, dy));
                let (na, nb) = (Nd::new(x, dx), Nd::new(y, dy));
                let cases: &[(&str, Dual64, Nd)] = &[
                    ("add", a + b, na + nb),
                    ("sub", a - b, na - nb),
                    ("mul", a * b, na * nb),
                ];
                for (name, ours, theirs) in cases {
                    prop_assert_eq!(ours.value.to_bits(), theirs.re.to_bits(), "{} value", name);
                    // add/sub tangents are the same expressions; mul's
                    // differ only by operand order (exact in IEEE).
                    prop_assert_eq!(ours.deriv.to_bits(), theirs.eps.to_bits(), "{} deriv", name);
                }
                prop_assume!(y.abs() > 1e-3);
                let (ours, theirs) = (a / b, na / nb);
                // num-dual divides via recip-and-multiply (two roundings)
                // — bit-identity is not expected in either channel.
                assert_ulps("div value", ours.value, theirs.re, 2)?;
                assert_ulps("div deriv", ours.deriv, theirs.eps, 8)?;
                prop_assume!(x > 1e-3);
                let (ours, theirs) = (Real::sqrt(Dual::new(x, dx)), Nd::new(x, dx).sqrt());
                prop_assert_eq!(ours.value.to_bits(), theirs.re.to_bits(), "sqrt value");
                // num-dual: √x·(1/x)·0.5 (three roundings); ours: d/(r+r).
                assert_ulps("sqrt deriv", ours.deriv, theirs.eps, 8)?;
            }

            /// Transcendentals: tolerance-based value comparison (std vs
            /// libm), tight-but-nonzero derivative bounds — see the
            /// module docs for the per-op rationale. Both duals carry the
            /// same random nonzero seed `d` (not the unit seed), so the
            /// derivatives compared are `d·slope` on each side — a mutant
            /// dropping the `·a'` seed factor would break the agreement.
            #[test]
            fn transcendental_derivatives_agree(
                x in -20.0..20.0f64,
                d in prop_oneof![-1000.0..-0.001f64, 0.001..1000.0f64],
            ) {
                let a = Dual::new(x, d);
                let na = Nd::new(x, d);
                let cases: &[(&str, Dual64, Nd, u64, u64)] = &[
                    ("sin", Real::sin(a), na.sin(), 16, 64),
                    ("cos", Real::cos(a), na.cos(), 16, 64),
                    ("tan", Real::tan(a), na.tan(), 128, 128),
                    ("atan", Real::atan(a), na.atan(), 16, 64),
                ];
                for (name, ours, theirs, value_bound, deriv_bound) in cases {
                    assert_ulps(&format!("{name} value"), ours.value, theirs.re, *value_bound)?;
                    assert_ulps(&format!("{name} deriv"), ours.deriv, theirs.eps, *deriv_bound)?;
                }
            }

            /// asin/acos on a domain bounded away from ±1 (both sides'
            /// derivative denominators √(1−x²) are ill-conditioned at the
            /// edges, in different ways). The shared random nonzero seed
            /// `d` makes the compared derivatives `d·slope` on each side,
            /// so a mutant dropping the `·a'` seed factor is caught.
            #[test]
            fn inverse_trig_derivatives_agree(
                x in -0.95..0.95f64,
                d in prop_oneof![-1000.0..-0.001f64, 0.001..1000.0f64],
            ) {
                let a = Dual::new(x, d);
                let na = Nd::new(x, d);
                assert_ulps("asin value", Real::asin(a).value, na.asin().re, 16)?;
                assert_ulps("asin deriv", Real::asin(a).deriv, na.asin().eps, 64)?;
                assert_ulps("acos value", Real::acos(a).value, na.acos().re, 16)?;
                assert_ulps("acos deriv", Real::acos(a).deriv, na.acos().eps, 64)?;
            }

            /// atan2 in all four quadrants; num-dual's derivative routes
            /// through atan(y/x), hence the wider allowance.
            #[test]
            fn atan2_derivatives_agree(
                y in 0.1..10.0f64,
                x in 0.1..10.0f64,
                sy in any::<bool>(),
                sx in any::<bool>(),
            ) {
                let y = if sy { -y } else { y };
                let x = if sx { -x } else { x };
                let ours = Real::atan2(Dual::variable(y), Dual::constant(x));
                let theirs = Nd::new(y, 1.0).atan2(Nd::new(x, 0.0));
                assert_ulps("atan2 value", ours.value, theirs.re, 16)?;
                assert_ulps("atan2 deriv", ours.deriv, theirs.eps, 128)?;
            }

            /// powi across positive and negative exponents; both sides
            /// use exponentiation-by-squaring shapes with different
            /// associations (ours pure squaring, num-dual `x^(n−3)·x·x·x`
            /// over std's powi), so ulp allowances not bit-identity. The
            /// shared random nonzero seed `d` makes the compared
            /// derivatives `d·n·xⁿ⁻¹` on each side, so a mutant dropping
            /// the `·a'` seed factor is caught.
            #[test]
            fn powi_derivatives_agree(
                x in 0.2..3.0f64,
                neg in any::<bool>(),
                n in -8..=8i32,
                d in prop_oneof![-1000.0..-0.001f64, 0.001..1000.0f64],
            ) {
                let x = if neg { -x } else { x };
                let ours = Dual::new(x, d).powi(n);
                let theirs = Nd::new(x, d).powi(n);
                assert_ulps("powi value", ours.value, theirs.re, 32)?;
                assert_ulps("powi deriv", ours.deriv, theirs.eps, 64)?;
            }

            /// The composition-smoke pipeline against num-dual end to
            /// end: many chained ops, so the divergences accumulate —
            /// relative bound rather than ulps.
            #[test]
            fn circle_distance_agrees_with_num_dual(
                theta in -3.0..3.0f64,
                r in 0.5..3.0f64,
                cx in -2.0..2.0f64,
                cy in -2.0..2.0f64,
                d in prop_oneof![-1000.0..-0.001f64, 0.001..1000.0f64],
            ) {
                prop_assume!(circle_distance(theta, r, cx, cy) > 0.1);
                // Both sides seed θ with the same random nonzero d (not
                // the unit seed): a mutant dropping a `·a'` seed factor on
                // the θ path breaks the end-to-end agreement.
                let ours = circle_distance(
                    Dual::new(theta, d),
                    Dual::constant(r),
                    Dual::constant(cx),
                    Dual::constant(cy),
                );
                let (s, c) = Nd::new(theta, d).sin_cos();
                let (nr, ncx, ncy) = (Nd::from_re(r), Nd::from_re(cx), Nd::from_re(cy));
                let (dx, dy) = (nr * c - ncx, nr * s - ncy);
                let theirs = (dx * dx + dy * dy).sqrt();
                prop_assert!(
                    (ours.deriv - theirs.eps).abs() <= 1e-12 * (1.0 + theirs.eps.abs()),
                    "dD/dθ ours = {} vs num-dual = {}",
                    ours.deriv, theirs.eps
                );
            }
        }
    }

    // ------------------------------------------------------------------
    // Dual<Interval> (feature-gated)
    // ------------------------------------------------------------------

    /// `Dual<Interval>` through the public API only ([`crate::real::Bounds`]
    /// on the channels); the decoration-level pins for the interval kink
    /// selectors live in `crate::interval`'s test module, which can see
    /// the wrapped `DecInterval`.
    #[cfg(feature = "interval")]
    mod interval_duals {
        use super::*;
        use crate::interval::Interval;
        use crate::real::Bounds;

        /// Shorthand: a dual with enclosure channels `[vlo, vhi]` and
        /// `[dlo, dhi]`.
        fn di(vlo: f64, vhi: f64, dlo: f64, dhi: f64) -> DualInterval {
            Dual::new(
                Interval::from_bounds(vlo, vhi),
                Interval::from_bounds(dlo, dhi),
            )
        }

        fn bounds_of(x: Interval) -> (f64, f64) {
            (x.lo(), x.hi())
        }

        /// The straddle convention: a value enclosure containing zero
        /// gets the subgradient hull [−1, 1] as its abs tangent factor.
        #[test]
        fn abs_straddle_takes_the_hull_derivative() {
            let straddling = di(-1.0, 2.0, 1.0, 1.0).abs();
            assert_eq!(bounds_of(straddling.value), (0.0, 2.0));
            assert_eq!(bounds_of(straddling.deriv), (-1.0, 1.0));
            // One-signed enclosures select ±1 exactly.
            let positive = di(1.0, 2.0, 3.0, 3.0).abs();
            assert_eq!(bounds_of(positive.deriv), (3.0, 3.0));
            let negative = di(-2.0, -1.0, 3.0, 3.0).abs();
            assert_eq!(bounds_of(negative.deriv), (-3.0, -3.0));
            // The point zero takes the +1 arm (matching f64's convention).
            let at_zero = di(0.0, 0.0, 5.0, 5.0).abs();
            assert_eq!(bounds_of(at_zero.deriv), (5.0, 5.0));
            // The hull factor scales an interval tangent outward
            // honestly: [−1, 1] · [2, 3] = [−3, 3].
            let scaled = di(-1.0, 1.0, 2.0, 3.0).abs();
            assert_eq!(bounds_of(scaled.deriv), (-3.0, 3.0));
        }

        /// min/max tangents: strict separation selects, overlap hulls —
        /// including the equal-points tie, where the hull of both
        /// branches is the honest subgradient enclosure.
        #[test]
        fn min_max_select_or_hull_tangents() {
            // Strictly separated: the winning side's tangent, verbatim.
            let a = di(1.0, 2.0, 5.0, 5.0);
            let b = di(3.0, 4.0, 7.0, 7.0);
            assert_eq!(bounds_of(Real::min(a, b).deriv), (5.0, 5.0));
            assert_eq!(bounds_of(Real::min(b, a).deriv), (5.0, 5.0));
            assert_eq!(bounds_of(Real::max(a, b).deriv), (7.0, 7.0));
            assert_eq!(bounds_of(Real::max(b, a).deriv), (7.0, 7.0));
            // Overlapping values: envelope value, hulled tangents.
            let a = di(1.0, 3.0, 1.0, 1.0);
            let b = di(2.0, 4.0, 9.0, 9.0);
            let m = Real::min(a, b);
            assert_eq!(bounds_of(m.value), (1.0, 3.0));
            assert_eq!(bounds_of(m.deriv), (1.0, 9.0));
            // Equal point values with different tangents — the tie kink:
            // the hull, not a pick (there is no "self" preference at
            // interval type; ties are a point notion).
            let a = di(2.0, 2.0, 1.0, 1.0);
            let b = di(2.0, 2.0, 5.0, 5.0);
            assert_eq!(bounds_of(Real::min(a, b).deriv), (1.0, 5.0));
            assert_eq!(bounds_of(Real::max(a, b).deriv), (1.0, 5.0));
            // Touching enclosures (shared endpoint) also hull — only
            // STRICT separation certifies a single branch.
            let a = di(1.0, 2.0, 1.0, 1.0);
            let b = di(2.0, 3.0, 9.0, 9.0);
            assert_eq!(bounds_of(Real::min(a, b).deriv), (1.0, 9.0));
        }

        /// floor at `Dual<Interval>`: `[0, 0]` tangents on plateaus
        /// (including a point exactly at an integer), the honest jump
        /// enclosure `[0, +∞]` scaled by the tangent over step-spanning
        /// boxes.
        #[test]
        fn floor_tangent_plateau_vs_jump() {
            // Plateau interior: exact-zero tangent.
            let plateau = di(2.2, 2.8, 5.0, 5.0).floor();
            assert_eq!(bounds_of(plateau.value), (2.0, 2.0));
            assert_eq!(bounds_of(plateau.deriv), (0.0, 0.0));
            // Point at an integer: still the plateau (f64-consistent).
            let at_int = di(3.0, 3.0, 5.0, 5.0).floor();
            assert_eq!(bounds_of(at_int.value), (3.0, 3.0));
            assert_eq!(bounds_of(at_int.deriv), (0.0, 0.0));
            // Step-spanning box: value hulls the two plateaus, tangent is
            // the jump enclosure [0, +∞]·[5, 5] = [0, +∞].
            let jump = di(2.5, 3.5, 5.0, 5.0).floor();
            assert_eq!(bounds_of(jump.value), (2.0, 3.0));
            assert_eq!(bounds_of(jump.deriv), (0.0, f64::INFINITY));
            // A negative tangent flips the jump enclosure to [−∞, 0].
            let jump_neg = di(2.5, 3.5, -5.0, -5.0).floor();
            assert_eq!(bounds_of(jump_neg.deriv), (f64::NEG_INFINITY, 0.0));
        }

        /// copysign at `Dual<Interval>`: sign-definite enclosures select
        /// `±abs′·x′`; a zero-containing sign hulls the value to
        /// `[−sup|x|, sup|x|]` and poisons the tangent to the entire
        /// line (the sign flip has unbounded slope).
        #[test]
        fn copysign_tangent_definite_vs_straddle() {
            // Certainly negative sign: value −|x|, tangent −(+1)·x′.
            let s_neg = Real::copysign(di(2.0, 3.0, 7.0, 7.0), di(-2.0, -1.0, 0.0, 0.0));
            assert_eq!(bounds_of(s_neg.value), (-3.0, -2.0));
            assert_eq!(bounds_of(s_neg.deriv), (-7.0, -7.0));
            // Certainly positive sign, negative x: |x|, (+1)(−1)·x′.
            let x_neg = Real::copysign(di(-3.0, -2.0, 7.0, 7.0), di(1.0, 2.0, 0.0, 0.0));
            assert_eq!(bounds_of(x_neg.value), (2.0, 3.0));
            assert_eq!(bounds_of(x_neg.deriv), (-7.0, -7.0));
            // Zero-containing sign (including the exact point [0, 0]):
            // two-sided value hull, entire-line tangent.
            let straddle = Real::copysign(di(2.0, 3.0, 7.0, 7.0), di(-1.0, 1.0, 0.0, 0.0));
            assert_eq!(bounds_of(straddle.value), (-3.0, 3.0));
            assert_eq!(
                bounds_of(straddle.deriv),
                (f64::NEG_INFINITY, f64::INFINITY)
            );
            let at_zero = Real::copysign(di(2.0, 3.0, 7.0, 7.0), di(0.0, 0.0, 0.0, 0.0));
            assert_eq!(bounds_of(at_zero.value), (-3.0, 3.0));
            // Straddling x with definite sign: abs′ hulls to [−1, 1].
            let x_straddle = Real::copysign(di(-1.0, 2.0, 7.0, 7.0), di(1.0, 1.0, 0.0, 0.0));
            assert_eq!(bounds_of(x_straddle.value), (0.0, 2.0));
            assert_eq!(bounds_of(x_straddle.deriv), (-7.0, 7.0));
        }

        /// Poison propagates through BOTH channels: a fully-out-of-domain
        /// sqrt empties value and tangent alike; a NaI input stays NaI
        /// through any chain rule.
        #[test]
        fn empty_and_nai_propagate_through_both_channels() {
            // sqrt of a certainly-negative enclosure: value empty, and
            // the tangent d/(r+r) is empty through the division.
            let empty = Dual::variable(Interval::from_f64(-1.0)).sqrt();
            assert!(empty.value.lo().is_nan(), "value lo must be NaN bracket");
            assert!(empty.value.hi().is_nan());
            assert!(empty.deriv.lo().is_nan(), "deriv lo must be NaN bracket");
            assert!(empty.deriv.hi().is_nan());
            // NaI from construction: every op keeps both channels NaI.
            let nai = Dual::variable(Interval::from_f64(f64::NAN));
            let (s, c) = nai.sin_cos();
            for (name, d) in [("sin", s), ("cos", c), ("tan", nai.tan())] {
                assert!(d.value.lo().is_nan(), "{name} value");
                assert!(d.deriv.lo().is_nan(), "{name} deriv");
            }
            // The kink ops too: an empty value poisons min's both
            // channels, and abs of NaI stays NaI in both.
            let clean = di(1.0, 2.0, 1.0, 1.0);
            let poisoned = Real::min(clean, empty);
            assert!(poisoned.value.lo().is_nan());
            assert!(poisoned.deriv.lo().is_nan());
            let poisoned = nai.abs();
            assert!(poisoned.value.lo().is_nan());
            assert!(poisoned.deriv.lo().is_nan());
        }

        /// powi at interval: the value channel is the tight,
        /// poison-preserving pown verbatim ([−1, 2]² = [0, 4], not
        /// repeated multiplication's [−2, 4]) and the n = 0 tangent is
        /// the exact zero point for describable values while poison in
        /// the value channel now poisons BOTH channels (#126 option
        /// (a): the zero is tainted through `powi_zero_deriv_factor`,
        /// never a fresh clean constant).
        #[test]
        fn powi_value_channel_is_interval_pown() {
            let x = di(-1.0, 2.0, 1.0, 1.0);
            let sq = x.powi(2);
            assert_eq!(bounds_of(sq.value), (0.0, 4.0));
            // Tangent: 2·[−1, 2]·[1, 1] = [−2, 4].
            assert_eq!(bounds_of(sq.deriv), (-2.0, 4.0));
            // n = 0: exact zero tangent for a describable value.
            let zero_exp = x.powi(0);
            assert_eq!(bounds_of(zero_exp.value), (1.0, 1.0));
            assert_eq!(bounds_of(zero_exp.deriv), (0.0, 0.0));
            // A poisoned value channel poisons the tangent too (the
            // #126 fix; the pre-fix laundering pinned a clean [0, 0]
            // here — see review_m5_pr1_launder.rs for the flipped pin).
            let empty = Dual::variable(Interval::from_f64(-1.0)).sqrt();
            let poisoned = empty.powi(0);
            assert!(poisoned.value.lo().is_nan(), "empty value survives n = 0");
            assert!(
                poisoned.deriv.lo().is_nan(),
                "empty value must taint the n = 0 tangent"
            );
        }

        /// Decide delegation at `Dual<Interval>`: enclosure semantics on
        /// the value part — sliver-band points stay indeterminate,
        /// decoration poison refuses (even reached through dual
        /// arithmetic), and adversarial tangents (huge, even NaI) never
        /// affect a cleanly classifiable value.
        #[test]
        fn decide_delegates_to_the_value_enclosure() {
            let band = band_1e9();
            // Definite value, adversarial tangents.
            let nai_tangent = Dual::new(
                Interval::from_bounds(1.0, 2.0),
                Interval::from_f64(f64::NAN),
            );
            assert_eq!(nai_tangent.sign_within(band), Ok(Sign::Positive));
            let huge_tangent = di(1.0, 2.0, -1e300, 1e300);
            assert_eq!(huge_tangent.sign_within(band), Ok(Sign::Positive));
            // A sliver-band point value is indeterminate whatever rides
            // along, carrying the ENCLOSURE diagnostic of the value part.
            let sliver = di(5e-9, 5e-9, 1e300, 1e300);
            let err = sliver
                .sign_within(band)
                .expect_err("sliver point must stay indeterminate");
            assert_eq!(err.margin, MarginDiag::Enclosure { lo: 5e-9, hi: 5e-9 });
            // A straddling value is indeterminate (subdivision's cue).
            let straddle = di(-1.0, 1.0, 0.0, 0.0);
            assert!(straddle.sign_within(band).is_err());
            // Decoration poison in the VALUE channel refuses to branch,
            // even after flowing through dual arithmetic to a decisively
            // positive enclosure — the decoration channel works through
            // the wrapper.
            let clamped = Dual::variable(Interval::from_bounds(-1.0, 4.0)).sqrt()
                + Dual::constant(Interval::one());
            assert_eq!(bounds_of(clamped.value), (1.0, 3.0));
            let err = clamped
                .sign_within(band)
                .expect_err("Trv-decorated value must refuse to classify");
            assert_eq!(err.margin, MarginDiag::Invalid);
        }

        /// THE contract at interval type: the dual's value channel is
        /// bit-identical (as an enclosure) to the plain-Interval run of
        /// the same pipeline.
        #[test]
        fn value_channel_bit_identical_to_plain_interval() {
            fn pipeline<T: Real>(x: T) -> T {
                let (s, c) = (x * T::pi() + T::one()).sin_cos();
                let h = (s * s + c * c).sqrt();
                Real::atan2(h, x.abs() + T::from_f64(0.5)).powi(3) / T::from_f64(7.0)
            }
            for seed in [0.0, 1.5e-9, -3.25, 1234.5] {
                let plain = pipeline(Interval::from_f64(seed));
                let dual = pipeline(Dual::variable(Interval::from_f64(seed)));
                assert_eq!(
                    (dual.value.lo().to_bits(), dual.value.hi().to_bits()),
                    (plain.lo().to_bits(), plain.hi().to_bits()),
                    "pipeline({seed})"
                );
            }
        }

        /// Composition smoke at `Dual<Interval>`: the derivative
        /// enclosure and an independently-computed analytic-derivative
        /// enclosure both bracket the true d/dθ, so they must intersect
        /// (a wrong chain rule shifts the dual enclosure by O(1) and
        /// breaks this); the centered case has exactly-zero true
        /// derivative, which the enclosure must contain.
        #[test]
        fn circle_distance_derivative_enclosure() {
            for theta in [0.3, 1.2, -2.5] {
                let t = Interval::from_f64(theta);
                let (r, cx, cy) = (
                    Interval::from_f64(2.0),
                    Interval::from_f64(0.5),
                    Interval::from_f64(-0.3),
                );
                let dual = circle_distance(
                    Dual::variable(t),
                    Dual::constant(r),
                    Dual::constant(cx),
                    Dual::constant(cy),
                );
                let analytic = circle_distance_dtheta(t, r, cx, cy);
                let (dlo, dhi) = bounds_of(dual.deriv);
                let (alo, ahi) = bounds_of(analytic);
                assert!(
                    dlo <= ahi && alo <= dhi,
                    "θ = {theta}: dual deriv [{dlo}, {dhi}] and analytic \
                     [{alo}, {ahi}] must both bracket the true derivative"
                );
                assert!(dhi - dlo < 1e-9, "θ = {theta}: enclosure too wide");
            }
            // Distance from the circle's own center is constantly r:
            // the true derivative is exactly 0 and must be enclosed.
            let dual = circle_distance(
                Dual::variable(Interval::from_f64(0.7)),
                Dual::constant(Interval::from_f64(2.0)),
                Dual::constant(Interval::zero()),
                Dual::constant(Interval::zero()),
            );
            let (dlo, dhi) = bounds_of(dual.deriv);
            assert!(
                dlo <= 0.0 && 0.0 <= dhi,
                "constant-distance derivative enclosure [{dlo}, {dhi}] must contain 0"
            );
            assert!(dhi - dlo < 1e-12);
        }

        /// Regression (review: the dependency problem this whole type
        /// exists to survive). Over a value enclosure that straddles zero,
        /// atan's derivative denominator `1 + a²` must be squared *tightly*
        /// (`powi(2)`, the backend's dedicated integer power), not by
        /// dependent multiplication:
        /// `a·a` over `[−1, 1.1]` returns `[−1.1, 1.21]`, so `1 + a·a`
        /// straddles zero and the division blows the derivative enclosure
        /// to `[−∞, ∞]`. With the tight square `a² = [0, 1.21] ≥ 0`, hence
        /// `1 + a² ≥ 1`, so d/da atan over the box `[−1, 1.1]` with unit
        /// tangent is a bounded enclosure ⊂ `[0.4, 1.1]` (true range
        /// `[1/2.21, 1] ≈ [0.4525, 1]`). atan2's `x² + y²` shares the fix;
        /// this pins the representative case.
        #[test]
        fn atan_derivative_enclosure_bounded_over_straddling_box() {
            let a = Dual::new(Interval::from_bounds(-1.0, 1.1), Interval::one());
            let (dlo, dhi) = bounds_of(a.atan().deriv);
            assert!(
                dlo.is_finite() && dhi.is_finite(),
                "atan' enclosure [{dlo}, {dhi}] must be finite \
                 (dependent a·a would give [−∞, ∞])"
            );
            assert!(
                0.4 <= dlo && dhi <= 1.1,
                "atan' enclosure [{dlo}, {dhi}] must lie within [0.4, 1.1]"
            );
        }

        /// The generic consumer at `DualInterval`: 3-4-5 exact in the
        /// value channel, tangent 0.6 enclosed tightly.
        #[test]
        fn generic_hypotenuse_at_dual_interval() {
            let h = hypotenuse(
                Dual::variable(Interval::from_f64(3.0)),
                Dual::constant(Interval::from_f64(4.0)),
            );
            assert_eq!(bounds_of(h.value), (5.0, 5.0));
            let (dlo, dhi) = bounds_of(h.deriv);
            assert!(
                dlo <= 0.6 && 0.6 <= dhi,
                "tangent [{dlo}, {dhi}] must enclose 0.6"
            );
            assert!(dhi - dlo < 1e-15);
        }
    }

    /// **What the D1 ruling actually returns** (Evan, 2026-08-19; Wave 0
    /// decision D1 of `docs/SMELL-SCAN-2026-08.md`). Every row here pins
    /// the *value* of the bracket, not its existence: each one fails if
    /// [`Bounds`] for [`Dual`] is ever widened to see the tangent, or
    /// narrowed away from the base scalar's own bracket.
    mod bracket {
        use super::*;
        use crate::real::Bounds;

        /// Tangents chosen to be maximally hostile to any implementation
        /// that reads them: poison, both infinities, both zeros, and a
        /// value that would dominate any hull.
        const HOSTILE_TANGENTS: [f64; 7] = [
            f64::NAN,
            f64::INFINITY,
            f64::NEG_INFINITY,
            0.0,
            -0.0,
            1e308,
            -1e308,
        ];

        /// The bracket IS the value channel, bit for bit, whatever the
        /// tangent is. Red under any tangent-reading definition: a hull
        /// of the two channels returns NaN for the `f64::NAN` tangent and
        /// ±∞ for the infinite ones, none of which is the value.
        #[test]
        fn bracket_is_the_value_channel_bitwise_and_ignores_the_tangent() {
            for value in [0.0, -0.0, 1.0, -2.5, f64::MIN_POSITIVE, 1e308] {
                for deriv in HOSTILE_TANGENTS {
                    let d = Dual64::new(value, deriv);
                    assert_eq!(
                        Bounds::lo(d).to_bits(),
                        value.to_bits(),
                        "lo() must be the value channel bit for bit \
                         (value {value}, tangent {deriv})"
                    );
                    assert_eq!(
                        Bounds::hi(d).to_bits(),
                        value.to_bits(),
                        "hi() must be the value channel bit for bit \
                         (value {value}, tangent {deriv})"
                    );
                }
            }
        }

        /// A poisoned value channel surfaces as NaN from BOTH accessors —
        /// the [`Bounds`] poison convention, inherited from `f64` rather
        /// than re-implemented. A clean tangent does not launder it.
        #[test]
        fn poisoned_value_surfaces_as_nan_from_both_accessors() {
            let d = Dual64::new(f64::NAN, 1.0);
            assert!(Bounds::lo(d).is_nan(), "lo() must surface value poison");
            assert!(Bounds::hi(d).is_nan(), "hi() must surface value poison");
        }

        /// At `Dual<f64>` the bracket is DEGENERATE: `lo == hi`. Red if
        /// the impl ever widens the dual's bracket by any amount.
        #[test]
        fn dual64_bracket_is_a_point() {
            for value in [0.0, 1.0, -3.75, 1e-300, 1e300] {
                for deriv in HOSTILE_TANGENTS {
                    let d = Dual64::new(value, deriv);
                    assert_eq!(
                        Bounds::lo(d).to_bits(),
                        Bounds::hi(d).to_bits(),
                        "the dual bracket must be a point (value {value}, \
                         tangent {deriv})"
                    );
                }
            }
        }

        /// The justification, executed: after a real `Real`-generic
        /// computation, the dual run's bracket is bit-identical to the
        /// `f64` run's — which is the ONLY reason this impl is allowed to
        /// exist (D9's value-channel contract). Red if the impl stops
        /// delegating to the base scalar, and red if the value channel
        /// ever stops being the plain-`f64` computation.
        #[test]
        fn bracket_after_a_computation_is_the_f64_runs_bracket() {
            let plain = hypotenuse(3.0_f64, 4.0_f64);
            let dual = hypotenuse(Dual64::variable(3.0), Dual64::constant(4.0));
            assert_eq!(
                Bounds::lo(dual).to_bits(),
                Bounds::lo(plain).to_bits(),
                "the dual run's bracket must be the f64 run's, bit for bit"
            );
            assert_eq!(Bounds::hi(dual).to_bits(), Bounds::hi(plain).to_bits());
            assert!(
                (dual.deriv - 0.6).abs() < 1e-15,
                "and the tangent is still there to be read off the field"
            );
        }

        /// At `Dual<Interval>` the bracket is the VALUE ENCLOSURE's own
        /// endpoints — not a point, and not touched by an unbounded
        /// tangent (E9: tangent poison never refuses). Red if the impl
        /// hulls the channels together: `[−∞, ∞]` would swallow `[−1, 2]`.
        #[cfg(feature = "interval")]
        #[test]
        fn dual_interval_bracket_is_the_value_enclosure() {
            use crate::interval::Interval;
            let d = Dual::new(
                Interval::from_bounds(-1.0, 2.0),
                Interval::from_bounds(f64::NEG_INFINITY, f64::INFINITY),
            );
            assert_eq!(
                (Bounds::lo(d), Bounds::hi(d)),
                (-1.0, 2.0),
                "the bracket must be the value enclosure, tangent ignored"
            );
        }
    }
}
