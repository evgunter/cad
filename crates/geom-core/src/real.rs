//! The [`Real`] scalar trait and its `f64` implementation.
//!
//! `Real` is the scalar abstraction the entire evaluation layer is generic
//! over (Q1 in `docs/DESIGN.md`). Planned instantiations: `f64` (here),
//! `Interval` over `interval-transcendentals` (M0 PR 4, backend swapped
//! in M5 PR 1), `Dual<f64>` and `Dual<Interval>` (M0
//! PR 5). The trait surface is the *intersection* of what those types
//! support honestly — anything one of them cannot do honestly stays out.
//!
//! # What is deliberately absent
//!
//! - **No [`PartialOrd`] / [`PartialEq`] bounds and no comparison methods.**
//!   Q1's one day-one discipline is that every topology-determining branch
//!   goes through a named trilean predicate (M0 PR 3). Omitting comparisons
//!   from `Real` makes the *convenient* paths fail to typecheck: generic
//!   evaluation code cannot write raw `<`, `==`, `.sort()`, or numeric
//!   casts on a scalar. It is not an airtight cage — safe escape channels
//!   remain: adding an extra bound like `+ PartialOrd` to a type parameter
//!   (compiles at `f64`, dies at the interval instantiation), `Debug`
//!   format-string gadgets (the `Debug` supertrait is a deliberate
//!   diagnostic affordance and the main leak channel), and `Any`/`TypeId`
//!   type-branching enabled by the `'static` bound. These are closed by a
//!   named kernel style rule, **evaluation-code discipline**: type
//!   parameters in evaluation code carry no bounds beyond geom-core's
//!   scalar traits, and no `format!`/`Debug`-string inspection or
//!   `TypeId`/`Any` dispatch on scalar values. The residue channels are
//!   banned by this rule and are loud in review; CI greps for the
//!   extra-bound pattern as a tripwire. (Concrete-`f64` code still has `<`
//!   from std — that is fine; the rule governs generic evaluation code.)
//! - **No bound extraction** (`to_f64`, `lo`/`hi`, `midpoint`): evaluation
//!   code must not be able to silently collapse an interval to a number.
//!   Certification/driver code that legitimately needs bounds goes through
//!   the separate [`Bounds`] trait (landed with the interval scalar, M0
//!   PR 4), whose restricted scope is a named style rule — see its docs.
//! - **No `exp`/`ln`/`pow(float)`**: nothing in M0–M4 needs them (analytic
//!   geometry and NURBS are algebraic/trigonometric). Cheap to add later,
//!   impossible to remove.
//! - **No `hypot` or other fused convenience methods**: callers write
//!   `(a*a + b*b).sqrt()`. A more-accurate `f64`-only override would make a
//!   dual number's value part diverge from the plain-`f64` computation;
//!   cross-instantiation consistency (the same recipe evaluated at different
//!   scalar types must agree on the shared part) outranks last-ulp accuracy.
//!   For the same reason `mul_add`/FMA is excluded: hardware-fused multiply-
//!   add versus a soft multiply-then-add rounds differently across
//!   instantiations and platforms, breaking cross-instantiation consistency
//!   and D9 determinism.
//! - **No [`core::iter::Sum`] / [`core::iter::Product`] bounds**: D9 allows
//!   parallelism only in fixed reduction shapes, so summation order must be
//!   explicit (a fold) at call sites; the trait offers no order-implicit
//!   reductions.
//!
//! # Totality and NaN policy
//!
//! Every operation is **total**. Out-of-domain inputs produce the scalar
//! type's poison value (`sqrt(-1)` is NaN at `f64`, the empty interval at
//! `Interval`) rather than an error: fallible arithmetic (`Result` from
//! every `sqrt`) is unusable in evaluation code. Instead, *the predicate
//! layer is the single place numbers become decisions* (M0 PR 3): any
//! predicate whose inputs are NaN/empty returns the typed
//! indeterminate/invalid outcome, never a silent branch, and certified
//! residual checks (D4 ¶2) catch NaN in cached geometry (`NaN ≤ ε` is false,
//! so certification fails loudly). NaN may propagate through *values* but
//! never through *decisions* — which is why [`Real::min`] / [`Real::max`]
//! must propagate NaN rather than drop it (see their docs).

use core::fmt::Debug;
use core::ops::{Add, Div, Mul, Neg, Sub};

/// The scalar type the geometry evaluation layer is generic over.
///
/// See the [module docs](self) for the design rationale: the deliberately
/// omitted operations (comparisons, bound extraction) and the totality/NaN
/// policy are as much a part of the contract as the methods below.
///
/// # Contract for implementors
///
/// - All operations are **total** — they never panic and never return an
///   error; out-of-domain inputs yield the type's poison value (NaN, empty
///   interval).
/// - [`Real::from_f64`] must be an **exact embedding**: every `f64` is
///   exactly representable (a point interval, a constant dual number).
/// - Deterministic per D9: same build + same inputs → bit-identical
///   outputs, on every platform.
/// - Overriding a defaulted method ([`Real::sin`], [`Real::cos`]) is only
///   permitted if the override is bit-identical to the corresponding
///   projection of the required [`Real::sin_cos`] primitive; this is under
///   test. (The expected reason to override is scalar performance — skipping
///   the discarded component — not a different numeric result.)
///
/// The `Copy` bound is deliberate: all planned instantiations are `Copy`,
/// and evaluation code is arithmetic-dense and reference-noise hostile. The
/// cost — foreclosing arbitrary-precision scalars (e.g. `rug::Float` is not
/// `Copy`) under this exact trait — is accepted; arbitrary precision would
/// need its own design pass anyway.
pub trait Real:
    Copy
    + Clone
    + Debug
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
    + Send
    + Sync
    + 'static
{
    /// Embeds an `f64` exactly (a point interval, a constant dual number).
    fn from_f64(x: f64) -> Self;

    /// The additive identity.
    fn zero() -> Self;

    /// The multiplicative identity.
    fn one() -> Self;

    /// The circle constant π (for interval types: a tight enclosure of π).
    fn pi() -> Self;

    /// The circle constant τ = 2π (for interval types: a tight enclosure).
    fn tau() -> Self;

    /// The square root. Total: out-of-domain input (negative at `f64`)
    /// yields the poison value (NaN / empty interval), per the module-level
    /// totality policy.
    fn sqrt(self) -> Self;

    /// The absolute value.
    fn abs(self) -> Self;

    /// Raises `self` to an integer power by exponentiation by squaring;
    /// `n < 0` computes the reciprocal of `self.powi(|n|)`, and `n == 0`
    /// yields [`Real::one`] for every **non-poisoned** input. Poison
    /// propagates through every exponent, *including `n == 0`* — NaN at
    /// `f64`, empty/NaI at intervals: `x⁰ = 1` is a statement about
    /// numbers, and a poisoned value is not a number (the module-level
    /// policy — poison flows through values, and laundering it into an
    /// exact 1 would erase the upstream failure).
    ///
    /// The multiplication order is fixed by the squaring algorithm (see
    /// [`powi_by_squaring`]), so results are deterministic but — for
    /// `|n| ≥ 4` — not necessarily bit-identical to naive repeated
    /// multiplication (different rounding association). Interval
    /// instantiations may override this method with a dedicated integer
    /// power for a **tight enclosure of the true value** — their contract
    /// is containment of the real power, not reproduction of f64's
    /// multiplication association (squaring an enclosure that straddles
    /// zero is not tight: `[-1, 2]·[-1, 2] = [-2, 4]` but `x² ∈ [0, 4]`).
    ///
    /// For negative exponents the reciprocal-of-power rule means extreme
    /// magnitudes can overflow before inverting — e.g. `powi(2.0, -1074)`
    /// yields `0.0` where the true value is the minimum subnormal. This is
    /// harmless within the session-boxed model range (D4 ¶4); changing it
    /// is a design conversation, not a bugfix.
    fn powi(self, n: i32) -> Self;

    /// The sine and cosine together (argument in radians) — **the
    /// primitive**.
    ///
    /// This pair, not the individual projections, is the required operation:
    /// it is the point on the unit circle at angle `self`, the restriction to
    /// the reals of the complex exponential `e^{iθ}`. The planned scalars make
    /// the pair the natural unit of work — a dual number's `sin` needs `cos x`
    /// for its derivative part anyway, and an interval computes both
    /// enclosures independently either way — so requiring the pair and
    /// projecting `sin`/`cos` out of it costs those types nothing while
    /// keeping the two components mutually consistent by construction.
    fn sin_cos(self) -> (Self, Self);

    /// The sine (argument in radians) — the first projection of
    /// [`Real::sin_cos`].
    ///
    /// Defaults to `self.sin_cos().0`. Implementations may override **only
    /// bit-identically to that projection** (verified by test); the sole
    /// sanctioned reason is scalar performance — skipping the discarded
    /// cosine — never a different numeric result.
    fn sin(self) -> Self {
        self.sin_cos().0
    }

    /// The cosine (argument in radians) — the second projection of
    /// [`Real::sin_cos`].
    ///
    /// Defaults to `self.sin_cos().1`. Implementations may override **only
    /// bit-identically to that projection** (verified by test); the sole
    /// sanctioned reason is scalar performance — skipping the discarded sine —
    /// never a different numeric result.
    fn cos(self) -> Self {
        self.sin_cos().1
    }

    /// The tangent (argument in radians).
    fn tan(self) -> Self;

    /// The arcsine, in [−π/2, π/2]. Total: |x| > 1 yields the poison value.
    fn asin(self) -> Self;

    /// The arccosine, in [0, π]. Total: |x| > 1 yields the poison value.
    fn acos(self) -> Self;

    /// The arctangent, in (−π/2, π/2).
    fn atan(self) -> Self;

    /// The four-quadrant arctangent of `self` (= y) and `x`, in (−π, π].
    fn atan2(self, x: Self) -> Self;

    /// The smaller of two values — a **lattice operation for geometry
    /// values** (bounding-box corners), never control flow. It returns
    /// `Self`, not `bool`, so it cannot drive a branch — that is the actual
    /// enforcement; branching goes through named predicates (M0 PR 3).
    ///
    /// Semantics: `min(a, b)` is `a` if `a ≤ b`, else `b` (ties keep
    /// `self`; for dual numbers the derivative follows the chosen argument
    /// at ties — kink nondifferentiability is inherent). **NaN propagates**:
    /// if either input is NaN the result is NaN — IEEE `minNum`'s
    /// NaN-dropping would silently launder a poisoned value, defeating the
    /// module-level NaN policy. (Intervals propagate empty naturally.)
    fn min(self, other: Self) -> Self;

    /// The larger of two values. Same contract as [`Real::min`]: a lattice
    /// operation for geometry values, not control flow; ties keep `self`;
    /// **NaN propagates** (either input NaN ⇒ NaN).
    fn max(self, other: Self) -> Self;

    /// The largest integer ≤ `self` — the range-reduction primitive
    /// (M2 PR 1; the M0-watchlist `floor`/`rem` item).
    ///
    /// Floor is an **exact** operation: the result is always exactly
    /// representable and uniquely defined, so every conforming
    /// implementation is bit-identical (like `sqrt`/`abs`, no libm
    /// routing question arises). It is a *value* computation, never
    /// control flow — it returns `Self`, not an integer type, so it
    /// cannot drive a branch (the same enforcement shape as
    /// [`Real::min`]).
    ///
    /// Per-instantiation semantics:
    ///
    /// - `f64`: IEEE `roundTowardNegative` to integral; NaN propagates,
    ///   `±∞` stays `±∞` (not poison), `floor(-0.0) = -0.0`.
    /// - `Interval`: the hull `[floor(lo), floor(hi)]` — floor spans
    ///   integers ⇒ the hull is the honest enclosure (containment is the
    ///   contract). The decoration degrades to `Def` when the enclosure
    ///   spans a jump (defined everywhere, discontinuous on the box), so
    ///   a downstream decision sees the discontinuity honestly;
    ///   empty/NaI propagate.
    /// - `Dual<T>`: value channel is `T::floor` verbatim; the derivative
    ///   follows the ratified kink conventions — `floor` is locally
    ///   constant, so the f64 tangent factor is 0 *including at
    ///   integers* (branch-consistency: the derivative of the program as
    ///   evaluated — the plateau's), while the interval instantiation
    ///   carries the honest jump enclosure `[0, +∞]` over any box that
    ///   spans an integer step (floor is nondecreasing, so all
    ///   difference quotients are ≥ 0 and unbounded across a jump —
    ///   the certified-tier analogue of `abs`'s straddle hull).
    fn floor(self) -> Self;

    /// The value with `self`'s magnitude and `sign`'s sign — the
    /// branchless sign-transfer primitive (needed by the Pixar
    /// orthonormal-basis construction, `Vec3::orthonormal_basis`).
    ///
    /// **Poison propagates through BOTH arguments** — deliberately
    /// stricter than IEEE 754 `copySign`, which is a non-arithmetic bit
    /// operation that would return `±|self|` for a NaN `sign`: a
    /// poisoned sign means the sign is unknown, and laundering it into a
    /// definite choice would defeat the module-level NaN policy. Either
    /// input NaN ⇒ NaN (empty/NaI at intervals).
    ///
    /// Per-instantiation semantics:
    ///
    /// - `f64`: IEEE `copySign` behind the poison guard — an exact bit
    ///   operation, bit-identical everywhere. The sign of a zero `sign`
    ///   argument is its sign *bit*: `copysign(x, -0.0) = -|x|`.
    /// - `Interval`: a `sign` enclosure strictly positive (`lo > 0`)
    ///   yields `|self|`, strictly negative (`hi < 0`) yields `-|self|`;
    ///   an enclosure containing zero yields the honest two-sided hull
    ///   `[-sup|self|, sup|self|]` with the decoration capped at `Def`
    ///   (the function is defined everywhere but discontinuous in `sign`
    ///   at 0 — and the hull also covers f64's signed-zero behavior,
    ///   which a one-sided choice at `lo ≥ 0` would not).
    /// - `Dual<T>`: value channel is `T::copysign` verbatim; the
    ///   derivative is `σ(sign)·abs′(self)·self′` per the kink
    ///   conventions — the `sign` argument's own tangent is discarded
    ///   (σ is locally constant, the same discard rule as `min`'s
    ///   unchosen branch), and an interval `sign` straddling zero
    ///   poisons the tangent to the entire line (the jump in `sign` has
    ///   unbounded slope).
    fn copysign(self, sign: Self) -> Self;

    /// Range reduction into one period: `self − period·floor(self/period)`
    /// — for `period > 0`, the representative of `self` modulo `period`
    /// lying in `[0, period)` up to rounding (see below). The intended
    /// use is periodic-parameter reduction: `θ.reduce_periodic(T::tau())`.
    ///
    /// **The compositional body IS the definition.** This is deliberately
    /// a *projection* of [`Real::floor`] and the arithmetic ops — one
    /// fixed formula, evaluated in exactly the written association
    /// (divide, floor, multiply, subtract) — rather than a per-scalar
    /// primitive (`rem_euclid` style), because that is what makes the
    /// cross-instantiation contract hold with no per-type re-derivation:
    /// the `Dual` value channel is bit-identical to the plain-`T` run *by
    /// construction* (it executes the same four operations), the interval
    /// instantiation contains the true reduced value *by composition* of
    /// containments, and no comparison appears anywhere (`floor` is the
    /// only nonsmooth ingredient, and it is comparison-free). A
    /// remainder-based per-scalar definition would need all three
    /// properties re-established per instantiation. Implementations may
    /// override **only bit-identically** to this body (same clause as
    /// [`Real::sin`]/[`Real::cos`]).
    ///
    /// **Honest rounding statement** (the seam blur): in floating point
    /// the result can land a few ulps *outside* `[0, period)` — an input
    /// a hair below a period multiple can round `self/period` up to the
    /// integer, producing a slightly negative result; a hair above can
    /// round it down, producing a result a hair above `period`. No
    /// clamping is applied (that would be a comparison). Consumers that
    /// need a topology-grade statement about the seam go through the
    /// predicate layer, like every other decision.
    ///
    /// **What is promised across periods**: nothing bitwise. `2π` is not
    /// representable, so `θ + k·fl(τ)` is a *different real parameter*
    /// than `θ + k·τ`; reduced evaluations of `θ` and `θ + k·fl(τ)`
    /// agree to rounding (a few ulps scaled by `k` and the derivative),
    /// never bit-identically. At interval type the enclosure of the
    /// reduction contains the true reduced value whenever the inputs
    /// enclose theirs — the containment form of periodicity.
    ///
    /// Total: `period = 0` or poison in either operand poisons the
    /// result through the division (NaN at `f64`, empty/NaI at
    /// intervals); a negative `period` reduces into `(period, 0]`-ish by
    /// the same formula (documented behavior, not an intended use).
    fn reduce_periodic(self, period: Self) -> Self {
        self - period * (self / period).floor()
    }
}

/// Bound extraction for **certification and driver code** — deliberately a
/// separate trait, never folded into [`Real`].
///
/// [`Real`] omits bound extraction so evaluation code cannot silently
/// collapse an interval to a number (see the [module docs](self)); this
/// trait is the separate door those docs promised. Its scope is a named
/// style rule under the evaluation-code discipline (L7 in
/// `docs/M0-LOG.md`): `Bounds` may appear only in **certification and
/// driver code** — residual certification, the subdivision driver,
/// rendering/telemetry — never in evaluation signatures. Code that needs
/// it writes `T: Bounds` as the parameter's sole bound (it is a subtrait,
/// so [`Real`]'s operations come with it); an *extra* bound tacked onto an
/// evaluation type parameter is exactly the escape hatch the discipline's
/// CI grep exists to catch.
///
/// **Ratified amendment (2026-07-29, M5 PR 8):** spatial-index /
/// candidate-pruning DRIVER code — the C10 `bvh` crate and the
/// certified box constructors beside their invariants — and the
/// boolean-sweep + evaluation-service seams that feed it may write
/// `T: Decide + Bounds`: the sweep is simultaneously decision code
/// (`Decide`) and the subdivision driver (`Bounds`), so the sole-bound
/// form is unsatisfiable there and the compound bound is the honest
/// signature. The sole-bound rule stands everywhere else, the CI grep
/// enforces a per-file allowlist of exactly these seams, and brackets
/// still never decide topology — every topology-determining branch
/// remains a [`Decide`](crate::predicate::Decide) call site; boxes
/// only ever prune.
///
/// **Ratified extension (M5 PR 11, Evan's lane-split ruling):**
/// `topo::props`'s certified-quadrature plumbing joins the compound
/// allowlist. The quadrature lane simultaneously decides (its
/// `props_quad_*` funnel margins) and reads brackets into the C9 ring
/// (certification substrate), so `T: Decide + Bounds` is its honest
/// signature; the split from bracket-free scalars (duals) is STATIC —
/// `topo::props::PropsQuadLane`'s explicit per-scalar impls are the
/// only entry, and the dual impl instantiates none of it.
///
/// **Extension (M5 PR 12, ORCHESTRATOR ruling 2026-08-03, applying
/// the PR 11 precedent; retroactive Evan review per the self-merge
/// convention):** the
/// **fillet-validity battery** — `sweep::fillet::battery` and the
/// assembly it licenses, `sweep::fillet::build` — joins the compound
/// allowlist. It is the same class as the quadrature seam on both
/// counts. It simultaneously decides (its six `fillet3_*` funnel
/// margins) and CONSUMES ENCLOSURES: the quantities it classifies are
/// certified metric bounds — a support's sup-normal-curvature hull
/// through `curvature_lever_arm`, a blend's setback bound off the
/// analytic arm — and every refusal reports the offending margin as an
/// `f64` payload, which is a bracket read. So `T: Decide + Bounds` is
/// its honest signature.
///
/// What differs from PR 11 is only the SPLIT, and it differs because
/// there is nothing to split: no dual-scalar path can reach this code.
/// [`Bounds`] is implemented for `f64`, the interval scalar, and the
/// telemetry probe — never for [`Dual`](crate::Dual), which has no
/// bracket to offer — and the one production caller
/// (`editor_core::eval`'s fillet wiring) already sits beneath an
/// evaluation service whose own signature carries `Bounds` and which
/// instantiates at `f64` and `Interval` only. A `PropsQuadLane`-style
/// static lane split would therefore have had an EMPTY refusing side:
/// a dual impl that refuses a call no dual scalar can make. The seam
/// is ratified instead, and the day a dual lane wants fillets is the
/// day the static split earns its keep.
///
/// # Semantics
///
/// `[lo(), hi()]` brackets every real number the scalar stands for. For
/// `f64` the bracket is the value itself (`lo` = `hi`); for the interval
/// scalar it is the enclosure of the **true** value of the computation —
/// not of any particular `f64` evaluation of it. A libm-computed `f64` can
/// land *outside* a tight enclosure of a transcendental result (libm makes
/// no correct-rounding guarantee — its divergence from std reaches 4 ulps
/// in the census; no faithful-rounding violation was found in 5.6M
/// samples, but none is promised — while the enclosure is correctly
/// rounded), so certification code
/// bounds *residual quantities* computed at interval type and never
/// asserts "f64 value ∈ enclosure" for transcendental results (exact
/// operations — `+`, `·`, `sqrt` — are correctly rounded at `f64` and may
/// be asserted contained).
///
/// Poison surfaces honestly rather than narrowing: a poisoned `f64` yields
/// NaN from both accessors, and the interval scalar yields NaN from both
/// accessors for **both** the ill-formed interval (NaI) and the empty one.
/// Empty and NaI are deliberately indistinguishable through this trait:
/// IEEE 1788's canonical empty pair (+∞, −∞) would let `hi() ≤ ε` PASS for
/// a poisoned-to-empty residual, and failing certification outranks
/// representational honesty. A NaN bracket fails every downstream
/// `residual ≤ ε` certification loudly (`NaN ≤ ε` is false under every
/// comparison direction — the D4 ¶2 fail-loud path): certification treats
/// such a bracket as failed, never as data.
pub trait Bounds: Real {
    /// The lower end of the bracket (the value itself at `f64`; the
    /// enclosure's infimum at the interval scalar).
    fn lo(self) -> f64;

    /// The upper end of the bracket (the value itself at `f64`; the
    /// enclosure's supremum at the interval scalar).
    fn hi(self) -> f64;
}

/// `f64` brackets itself exactly: `lo` = `hi` = the value. NaN stays NaN —
/// poison surfaces through the bracket, never silently narrows away.
impl Bounds for f64 {
    fn lo(self) -> f64 {
        self
    }

    fn hi(self) -> f64 {
        self
    }
}

/// **Bracket access without the `Real` obligation** — the certification
/// seam (M5 PR 2, `docs/CURVED-DESIGN.md` C9).
///
/// [`Bounds`] is a subtrait of [`Real`], which is right for *evaluation
/// scalars* that also carry a bracket (`f64`, the interval scalar): they
/// are things geometry recipes are replayed at. But the C9 interval ring
/// ([`crate::ring_interval::RingInterval`]) is deliberately **not** an
/// evaluation scalar — it has no transcendentals and must never appear in
/// an evaluation signature — so it cannot implement `Real`, and therefore
/// cannot implement `Bounds`.
///
/// `Enclosure` is the smaller trait both sides can meet at: just the two
/// bracket readers, no arithmetic obligation at all. Every `Bounds`
/// implementor gets it by blanket impl (so `f64` and the interval scalar
/// are covered without a line of change), and the ring implements it
/// directly. Certification helpers — the spline hull bounds in
/// [`crate::spline::hull`] are the first — take `T: Enclosure` and work
/// for all three: an `f64` coefficient is a degenerate bracket, an
/// interval-scalar coefficient is the replayed enclosure, a ring
/// coefficient is the certification arithmetic's own.
///
/// # Semantics
///
/// Identical to [`Bounds`]: `[lo(), hi()]` brackets every real number the
/// value stands for, and **poison surfaces as NaN from both accessors**
/// rather than narrowing — a NaN bracket fails every `residual <= eps`
/// check loudly (D4 ¶2). Implementors owe that convention.
///
/// # Style note (method-name shadowing)
///
/// The two traits share method names `lo`/`hi`. Generic code bounded by
/// `T: Bounds` resolves through its own bound and is unaffected, but
/// calling `x.lo()` on a **concrete** `Bounds` type with both traits in
/// scope is ambiguous (E0034). Import one trait, or disambiguate with
/// `Enclosure::lo(x)`. The names are worth the friction: a second spelling
/// for "the bottom of the bracket" would be worse.
pub trait Enclosure: Copy {
    /// The lower end of the bracket (NaN if poisoned).
    fn lo(self) -> f64;

    /// The upper end of the bracket (NaN if poisoned).
    fn hi(self) -> f64;
}

/// Every [`Bounds`] scalar is an [`Enclosure`] — the one-line seam that
/// keeps `f64` and the interval scalar usable by certification helpers
/// written against the smaller trait.
impl<T: Bounds> Enclosure for T {
    fn lo(self) -> f64 {
        Bounds::lo(self)
    }

    fn hi(self) -> f64 {
        Bounds::hi(self)
    }
}

/// Exponentiation by squaring over any [`Real`], the shared implementation
/// of [`Real::powi`]: `n < 0` via the reciprocal of `base.powi(|n|)`,
/// `n == 0` yields one **unconditionally** — the generic default takes the
/// shortcut without inspecting the base (it cannot: [`Real`] deliberately
/// exposes no poison test), so poison-aware implementations guard `n == 0`
/// before delegating (f64's NaN guard; the interval scalar overrides the
/// whole method). Total for every input; the multiplication order is fixed
/// (deterministic per D9).
pub(crate) fn powi_by_squaring<T: Real>(base: T, n: i32) -> T {
    let mut result = T::one();
    let mut acc = base;
    // unsigned_abs handles n == i32::MIN without overflow.
    let mut e = n.unsigned_abs();
    while e > 0 {
        if e & 1 == 1 {
            result = result * acc;
        }
        e >>= 1;
        if e > 0 {
            acc = acc * acc;
        }
    }
    if n < 0 { T::one() / result } else { result }
}

/// `f64` is the kernel's default scalar (D9 determinism notes per method).
///
/// Transcendentals go through the pure-Rust [`libm`] crate: system libm
/// `sin`/`cos` differ across platforms in the last ulp — enough to flip a
/// marginal predicate (D9). `sqrt` and `abs` use the std/hardware
/// operations because IEEE 754 *requires* them to be exact/correctly
/// rounded, so they are already bit-identical everywhere (and faster).
impl Real for f64 {
    /// The identity — every `f64` embeds as itself, exactly.
    fn from_f64(x: f64) -> Self {
        x
    }

    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }

    fn pi() -> Self {
        core::f64::consts::PI
    }

    fn tau() -> Self {
        core::f64::consts::TAU
    }

    /// Std/hardware sqrt, not libm: IEEE 754 requires `sqrt` to be
    /// correctly rounded, so it is bit-identical on every conforming
    /// platform — D9-compliant and faster than a soft implementation.
    /// `sqrt(x) = NaN` for `x < 0` (totality policy; `sqrt(-0.0) = -0.0`
    /// per IEEE 754).
    fn sqrt(self) -> Self {
        f64::sqrt(self)
    }

    /// Std abs, not libm: it only clears the sign bit — exact on every
    /// platform, trivially D9-compliant.
    fn abs(self) -> Self {
        f64::abs(self)
    }

    /// [`powi_by_squaring`] behind a poison guard: `NaN⁰` is NaN, not 1 —
    /// the generic `n == 0` shortcut would launder f64's only poison
    /// representation into an exact 1 (the trait's poison-propagation
    /// clause). `(±∞)⁰` stays 1: ±∞ is not f64 poison — infinite margins
    /// are maximally definite (PR 3) — so only NaN takes the guard.
    fn powi(self, n: i32) -> Self {
        if n == 0 && self.is_nan() {
            return f64::NAN;
        }
        powi_by_squaring(self, n)
    }

    fn sin_cos(self) -> (Self, Self) {
        // Deliberately two separate libm calls, not libm::sincos: the
        // bit-identity-with-the-projections contract is what matters, and
        // `(libm::sin, libm::cos)` satisfies it by construction. libm has a
        // fused `sincos`, but revisit only with evidence that a fused
        // override is both bit-identical and worth the audit burden.
        (libm::sin(self), libm::cos(self))
    }

    /// Overrides the [`Real::sin`] projection so scalar sine does not pay for
    /// a discarded cosine. Trivially bit-identical to the projection: f64's
    /// [`Real::sin_cos`] *is* the two libm calls `(libm::sin, libm::cos)`, so
    /// this is exactly its first component (under test).
    fn sin(self) -> Self {
        libm::sin(self)
    }

    /// Overrides the [`Real::cos`] projection so scalar cosine does not pay
    /// for a discarded sine. Trivially bit-identical to the projection: f64's
    /// [`Real::sin_cos`] *is* the two libm calls `(libm::sin, libm::cos)`, so
    /// this is exactly its second component (under test).
    fn cos(self) -> Self {
        libm::cos(self)
    }

    fn tan(self) -> Self {
        libm::tan(self)
    }

    fn asin(self) -> Self {
        libm::asin(self)
    }

    fn acos(self) -> Self {
        libm::acos(self)
    }

    fn atan(self) -> Self {
        libm::atan(self)
    }

    fn atan2(self, x: Self) -> Self {
        libm::atan2(self, x)
    }

    /// NaN-propagating minimum: either input NaN ⇒ NaN. Deliberately NOT
    /// `f64::min` (IEEE `minNum`), which would return the non-NaN operand
    /// and silently drop a poisoned value. Ties (including `0.0` vs
    /// `-0.0`) keep `self`. Raw comparison is allowed *inside* scalar
    /// implementations; it is generic evaluation code that must not branch
    /// on values (Q1).
    fn min(self, other: Self) -> Self {
        if self.is_nan() || other.is_nan() {
            f64::NAN
        } else if self <= other {
            self
        } else {
            other
        }
    }

    /// NaN-propagating maximum: either input NaN ⇒ NaN (see [`Real::min`]
    /// on why `f64::max` is not used). Ties keep `self`.
    fn max(self, other: Self) -> Self {
        if self.is_nan() || other.is_nan() {
            f64::NAN
        } else if self >= other {
            self
        } else {
            other
        }
    }

    /// Std/hardware floor, not libm: IEEE 754 `roundTowardNegative` to
    /// integral is an exact operation (the result is uniquely defined and
    /// exactly representable), so it is bit-identical on every conforming
    /// platform — D9-compliant, same posture as `sqrt`/`abs`. NaN
    /// propagates; `±∞` stays (not poison); `floor(-0.0) = -0.0`.
    fn floor(self) -> Self {
        f64::floor(self)
    }

    /// IEEE `copySign` behind the trait's poison guard: either input NaN
    /// ⇒ NaN (IEEE's own `copySign` is a non-arithmetic bit operation
    /// that would launder a NaN `sign` into a definite choice — see the
    /// trait docs). Otherwise an exact bit operation: the sign of a zero
    /// `sign` argument is its sign bit. Raw `is_nan` inspection is
    /// allowed inside scalar implementations (Q1), as in [`Real::min`].
    fn copysign(self, sign: Self) -> Self {
        if self.is_nan() || sign.is_nan() {
            f64::NAN
        } else {
            f64::copysign(self, sign)
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // f64 has *inherent* sin/min/... (std) that shadow the trait methods on
    // method-call syntax, so tests invoke the trait explicitly (`Real::sin`)
    // or through generic helpers — otherwise we would be testing std, not
    // our libm-backed impl.

    /// Distance in ulps between two finite f64s (monotone integer mapping;
    /// +0.0 and -0.0 both map to 0).
    fn ulp_dist(a: f64, b: f64) -> u64 {
        fn ord(x: f64) -> i64 {
            let i = x.to_bits() as i64;
            if i < 0 { i64::MIN.wrapping_sub(i) } else { i }
        }
        u64::try_from((i128::from(ord(a)) - i128::from(ord(b))).unsigned_abs())
            .expect("ulp distance exceeds u64 — inputs were not comparable finite values")
    }

    /// A generic consumer of the trait, exercising that real evaluation
    /// code can be written against `Real` alone.
    fn hypotenuse<T: Real>(a: T, b: T) -> T {
        (a * a + b * b).sqrt()
    }

    fn naive_powi(x: f64, n: i32) -> f64 {
        let mut r = 1.0;
        for _ in 0..n.unsigned_abs() {
            r *= x;
        }
        if n < 0 { 1.0 / r } else { r }
    }

    #[test]
    fn from_f64_is_exact_identity() {
        for x in [0.0, -0.0, 1.0, -1.0, 2.5, 1e-308, -1e300, f64::INFINITY] {
            assert_eq!(<f64 as Real>::from_f64(x).to_bits(), x.to_bits());
        }
        assert!(<f64 as Real>::from_f64(f64::NAN).is_nan());
    }

    #[test]
    fn constants_are_exact() {
        assert_eq!(<f64 as Real>::zero().to_bits(), 0.0f64.to_bits());
        assert_eq!(<f64 as Real>::one().to_bits(), 1.0f64.to_bits());
        assert_eq!(<f64 as Real>::pi(), core::f64::consts::PI);
        assert_eq!(<f64 as Real>::tau(), core::f64::consts::TAU);
        // Doubling is exact in binary floating point and rounding commutes
        // with scaling by 2, so fl(2π) == 2·fl(π) exactly.
        assert_eq!(<f64 as Real>::tau(), 2.0 * <f64 as Real>::pi());
    }

    #[test]
    fn identity_laws_hold_exactly_on_samples() {
        for x in [0.0, 1.0, -2.5, 1e-9, 1e12, -7.25e-3] {
            assert_eq!((<f64 as Real>::zero() + x).to_bits(), x.to_bits());
            assert_eq!((<f64 as Real>::one() * x).to_bits(), x.to_bits());
        }
    }

    #[test]
    fn generic_code_compiles_and_computes() {
        // 3-4-5 triangle: 9 + 16 = 25 exactly, sqrt(25) = 5 exactly.
        assert_eq!(hypotenuse(3.0f64, 4.0f64), 5.0);
    }

    #[test]
    fn sqrt_totality_policy() {
        assert!(<f64 as Real>::sqrt(-1.0).is_nan());
        assert!(<f64 as Real>::sqrt(f64::NAN).is_nan());
        // IEEE 754: sqrt(-0.0) is -0.0, and sqrt is correctly rounded.
        assert_eq!(<f64 as Real>::sqrt(-0.0).to_bits(), (-0.0f64).to_bits());
        assert_eq!(<f64 as Real>::sqrt(4.0), 2.0);
        assert_eq!(<f64 as Real>::sqrt(f64::INFINITY), f64::INFINITY);
    }

    #[test]
    fn abs_basics() {
        assert_eq!(<f64 as Real>::abs(-2.5), 2.5);
        assert_eq!(<f64 as Real>::abs(2.5), 2.5);
        // abs clears the sign bit, so abs(-0.0) is +0.0 bitwise.
        assert_eq!(<f64 as Real>::abs(-0.0).to_bits(), 0.0f64.to_bits());
        assert!(<f64 as Real>::abs(f64::NAN).is_nan());
    }

    #[test]
    fn powi_zero_exponent_is_one_except_for_poison() {
        // The ±∞ rows are the deliberate carve-out: infinity is not f64
        // poison (infinite margins are maximally definite, PR 3), so
        // (±∞)⁰ = 1 like every other non-poisoned input.
        for x in [0.0, -0.0, 2.5, f64::INFINITY, f64::NEG_INFINITY] {
            assert_eq!(<f64 as Real>::powi(x, 0), 1.0);
        }
        // NaN — f64's poison — propagates through n = 0 instead of
        // laundering into an exact 1 (trait poison-propagation clause).
        assert!(<f64 as Real>::powi(f64::NAN, 0).is_nan());
    }

    #[test]
    fn powi_small_cases() {
        assert_eq!(<f64 as Real>::powi(2.0, 10), 1024.0);
        assert_eq!(<f64 as Real>::powi(2.0, -2), 0.25);
        assert_eq!(<f64 as Real>::powi(-3.0, 3), -27.0);
        assert_eq!(<f64 as Real>::powi(0.0, -1), f64::INFINITY);
        assert!(<f64 as Real>::powi(f64::NAN, 1).is_nan());
    }

    #[test]
    fn bounds_for_f64_is_the_identity_bracket() {
        for x in [0.0, -0.0, 1.5, -1e300, f64::INFINITY, f64::NEG_INFINITY] {
            assert_eq!(Bounds::lo(x).to_bits(), x.to_bits());
            assert_eq!(Bounds::hi(x).to_bits(), x.to_bits());
        }
        // Poison surfaces: a NaN value yields a NaN bracket, which every
        // downstream certification comparison fails loudly (D4 ¶2).
        assert!(Bounds::lo(f64::NAN).is_nan());
        assert!(Bounds::hi(f64::NAN).is_nan());
    }

    #[test]
    fn min_max_nan_propagation() {
        let n = f64::NAN;
        assert!(Real::min(n, 1.0).is_nan());
        assert!(Real::min(1.0, n).is_nan());
        assert!(Real::min(n, n).is_nan());
        assert!(Real::max(n, 1.0).is_nan());
        assert!(Real::max(1.0, n).is_nan());
        assert!(Real::max(n, n).is_nan());
        // Contrast: std f64::min would drop the NaN — the behavior we reject.
        assert_eq!(f64::min(n, 1.0), 1.0);
    }

    #[test]
    fn min_max_ties_keep_self() {
        // Signed-zero ties are decided by argument position, not value.
        assert_eq!(Real::min(0.0f64, -0.0).to_bits(), 0.0f64.to_bits());
        assert_eq!(Real::min(-0.0f64, 0.0).to_bits(), (-0.0f64).to_bits());
        assert_eq!(Real::max(0.0f64, -0.0).to_bits(), 0.0f64.to_bits());
        assert_eq!(Real::max(-0.0f64, 0.0).to_bits(), (-0.0f64).to_bits());
    }

    #[test]
    fn floor_exactness_and_poison() {
        // Exact integral rounding toward −∞ on every kind of input.
        assert_eq!(<f64 as Real>::floor(2.7), 2.0);
        assert_eq!(<f64 as Real>::floor(-2.3), -3.0);
        assert_eq!(<f64 as Real>::floor(2.0), 2.0);
        assert_eq!(<f64 as Real>::floor(-2.0), -2.0);
        // Signed zeros are preserved (floor is exact, no sign laundering).
        assert_eq!(<f64 as Real>::floor(0.5), 0.0);
        assert_eq!(<f64 as Real>::floor(-0.0).to_bits(), (-0.0f64).to_bits());
        assert_eq!(<f64 as Real>::floor(0.0).to_bits(), 0.0f64.to_bits());
        // ±∞ are not poison and pass through; NaN propagates.
        assert_eq!(<f64 as Real>::floor(f64::INFINITY), f64::INFINITY);
        assert_eq!(<f64 as Real>::floor(f64::NEG_INFINITY), f64::NEG_INFINITY);
        assert!(<f64 as Real>::floor(f64::NAN).is_nan());
        // Above 2^52 every f64 is integral: floor is the identity there.
        assert_eq!(<f64 as Real>::floor(9.1e15), 9.1e15);
    }

    #[test]
    fn copysign_transfers_sign_and_propagates_poison() {
        assert_eq!(<f64 as Real>::copysign(3.0, -1.0), -3.0);
        assert_eq!(<f64 as Real>::copysign(-3.0, 1.0), 3.0);
        assert_eq!(<f64 as Real>::copysign(3.0, 1.0), 3.0);
        // The sign of a zero `sign` argument is its sign BIT (documented).
        assert_eq!(<f64 as Real>::copysign(3.0, -0.0), -3.0);
        assert_eq!(<f64 as Real>::copysign(3.0, 0.0), 3.0);
        // Zero magnitude takes the transferred sign bitwise.
        assert_eq!(
            <f64 as Real>::copysign(0.0, -1.0).to_bits(),
            (-0.0f64).to_bits()
        );
        // ±∞ magnitude is not poison.
        assert_eq!(
            <f64 as Real>::copysign(f64::INFINITY, -1.0),
            f64::NEG_INFINITY
        );
        // Poison propagates through BOTH arguments — the deliberate
        // deviation from IEEE copySign (which would return ±3.0 here).
        assert!(<f64 as Real>::copysign(f64::NAN, 1.0).is_nan());
        assert!(<f64 as Real>::copysign(3.0, f64::NAN).is_nan());
        // Contrast: the IEEE bit operation launders the NaN sign.
        assert_eq!(f64::copysign(3.0, f64::NAN).abs(), 3.0);
    }

    #[test]
    fn reduce_periodic_basics_and_poison() {
        use core::f64::consts::TAU;
        // In-range inputs are fixed points (floor(x/p) = 0 ⇒ x − p·0 = x,
        // bit-exact for non-negative x).
        assert_eq!(
            <f64 as Real>::reduce_periodic(1.5, TAU).to_bits(),
            1.5f64.to_bits()
        );
        // One period up/down reduces to within rounding of the in-range
        // representative (fl(τ) arithmetic — value closeness, never
        // bit-identity; see the trait docs).
        assert!((<f64 as Real>::reduce_periodic(1.5 + TAU, TAU) - 1.5).abs() <= 1e-15);
        assert!((<f64 as Real>::reduce_periodic(1.5 - TAU, TAU) - 1.5).abs() <= 1e-15);
        // Many periods out: still lands within rounding of 1.5, with the
        // documented k-scaled blur.
        assert!((<f64 as Real>::reduce_periodic(1.5 + 1000.0 * TAU, TAU) - 1.5).abs() <= 1e-11);
        // Exact-period multiples of an exactly representable period.
        assert_eq!(<f64 as Real>::reduce_periodic(6.0, 2.0), 0.0);
        assert_eq!(<f64 as Real>::reduce_periodic(-6.0, 2.0), 0.0);
        assert_eq!(<f64 as Real>::reduce_periodic(-1.5, 2.0), 0.5);
        // Poison: zero period (0·∞ NaN through the formula), NaN operands.
        assert!(<f64 as Real>::reduce_periodic(1.0, 0.0).is_nan());
        assert!(<f64 as Real>::reduce_periodic(f64::NAN, TAU).is_nan());
        assert!(<f64 as Real>::reduce_periodic(1.0, f64::NAN).is_nan());
    }

    #[test]
    fn atan2_axis_cases() {
        use core::f64::consts::{FRAC_PI_2, PI};
        assert_eq!(Real::atan2(0.0f64, 1.0), 0.0);
        assert_eq!(Real::atan2(0.0f64, -1.0), PI);
        assert_eq!(Real::atan2(-0.0f64, -1.0), -PI);
        assert_eq!(Real::atan2(1.0f64, 0.0), FRAC_PI_2);
        assert_eq!(Real::atan2(-1.0f64, 0.0), -FRAC_PI_2);
        // IEEE convention: atan2(±0, +0) = ±0.
        assert_eq!(Real::atan2(0.0f64, 0.0), 0.0);
    }

    /// Census of libm-vs-std divergence over a fixed, deterministic sample
    /// set. This test COUNTS differing results (it does not assert the
    /// count is zero): the counts document *why* D9 mandates the libm crate
    /// — std routes to the platform libm, whose sin/cos are not correctly
    /// rounded and differ across platforms in the last ulp, enough to flip
    /// a marginal predicate. The only assertion is a sanity bound: libm
    /// stays within a few ulps of std everywhere on the samples.
    #[test]
    fn libm_vs_std_divergence_census() {
        const N: u32 = 20_000;
        let mut sin_diff = 0u32;
        let mut cos_diff = 0u32;
        let mut max_ulp = 0u64;
        for i in 0..=N {
            let x = -1000.0 + f64::from(i) * (2000.0 / f64::from(N));
            let (ls, ss) = (Real::sin(x), f64::sin(x));
            let (lc, sc) = (Real::cos(x), f64::cos(x));
            if ls.to_bits() != ss.to_bits() {
                sin_diff += 1;
            }
            if lc.to_bits() != sc.to_bits() {
                cos_diff += 1;
            }
            let d = ulp_dist(ls, ss).max(ulp_dist(lc, sc));
            max_ulp = max_ulp.max(d);
            assert!(
                d <= 4,
                "libm and std diverge by {d} ulps at x = {x} — beyond sanity bound"
            );
        }
        println!(
            "libm vs std over {} samples in [-1000, 1000]: sin differs on {} \
             ({:.3}%), cos differs on {} ({:.3}%), max divergence {} ulp(s)",
            N + 1,
            sin_diff,
            100.0 * f64::from(sin_diff) / f64::from(N + 1),
            cos_diff,
            100.0 * f64::from(cos_diff) / f64::from(N + 1),
            max_ulp
        );
    }

    proptest! {
        #[test]
        fn pythagorean_identity(x in -1.0e3..1.0e3f64) {
            let s = Real::sin(x);
            let c = Real::cos(x);
            prop_assert!((s * s + c * c - 1.0).abs() < 1e-14);
        }

        /// f64's *overridden* `sin`/`cos` must be bit-identical to the
        /// projections of the required `sin_cos` primitive — the tested
        /// direction of the override contract. This is not tautological: f64
        /// overrides `sin`/`cos` (separate scalar libm calls) rather than
        /// inheriting the projection defaults, so the test genuinely verifies
        /// those overrides match `sin_cos`'s components, guarding against an
        /// override or an edit to `sin_cos` that breaks bit-identity.
        #[test]
        fn sin_cos_bit_identical_to_components(x in -1.0e6..1.0e6f64) {
            let (s, c) = Real::sin_cos(x);
            prop_assert_eq!(s.to_bits(), Real::sin(x).to_bits());
            prop_assert_eq!(c.to_bits(), Real::cos(x).to_bits());
        }

        /// Quadrant correctness of atan2. Magnitudes are bounded within
        /// [1e-6, 1e6] so the ratio y/x stays ≥ 1e-12 away from the axes and
        /// the strict inequalities cannot be defeated by rounding to a
        /// boundary value (e.g. atan2 rounding up to fl(π)).
        #[test]
        fn atan2_quadrants(y in 1.0e-6..1.0e6f64, x in 1.0e-6..1.0e6f64) {
            use core::f64::consts::{FRAC_PI_2, PI};
            let q1 = Real::atan2(y, x);
            prop_assert!(q1 > 0.0 && q1 < FRAC_PI_2);
            let q2 = Real::atan2(y, -x);
            prop_assert!(q2 > FRAC_PI_2 && q2 < PI);
            let q3 = Real::atan2(-y, -x);
            prop_assert!(q3 > -PI && q3 < -FRAC_PI_2);
            let q4 = Real::atan2(-y, x);
            prop_assert!(q4 > -FRAC_PI_2 && q4 < 0.0);
        }

        /// sin(asin(x)) recovers x to ~1e-15 absolute: asin is accurate to
        /// ~1 ulp of a value ≤ π/2 (≈3.5e-16) and |d sin| ≤ 1, so the
        /// round-trip error is a few 1e-16.
        #[test]
        fn asin_roundtrip(x in -1.0..1.0f64) {
            prop_assert!((Real::sin(Real::asin(x)) - x).abs() <= 1e-15);
        }

        /// cos(acos(x)) recovers x to ~1e-15 absolute: acos is accurate to
        /// ~1 ulp of a value ≤ π (≈4.4e-16) and |d cos| = |sin| ≤ 1.
        #[test]
        fn acos_roundtrip(x in -1.0..1.0f64) {
            prop_assert!((Real::cos(Real::acos(x)) - x).abs() <= 1e-15);
        }

        /// tan(atan(x)) recovers x. Slack: an ~1-ulp error e in
        /// atan (|e| ≤ ~2.2e-16) is amplified by tan' = 1 + x², so the
        /// absolute error is O((1 + x²)·2.2e-16) ≈ 2.3e-12 at |x| = 100;
        /// the mixed bound below covers both the tiny-x and large-x ends.
        #[test]
        fn atan_roundtrip(x in -1.0e2..1.0e2f64) {
            let t = Real::tan(Real::atan(x));
            prop_assert!((t - x).abs() <= 1e-11 + 1e-13 * x.abs());
        }

        /// For |n| ≤ 3 exponentiation by squaring performs the *same*
        /// multiplications as naive repeated multiplication (up to
        /// commutativity, which is exact in IEEE arithmetic), so results
        /// are bit-identical. From |n| = 4 the association differs
        /// ((x²)·(x²) vs ((x²)·x)·x) and only closeness is guaranteed —
        /// see `powi_close_to_naive_medium_n`.
        #[test]
        fn powi_exact_vs_naive_small_n(
            x in 0.01..100.0f64,
            neg in any::<bool>(),
            n in -3..=3i32,
        ) {
            let x = if neg { -x } else { x };
            prop_assert_eq!(
                <f64 as Real>::powi(x, n).to_bits(),
                naive_powi(x, n).to_bits()
            );
        }

        /// For |n| in 4..=12, squaring and naive association may round
        /// differently; both accumulate ≤ ~n/2 ulps of relative error, so
        /// they agree to well within 1e-13 relative.
        #[test]
        fn powi_close_to_naive_medium_n(
            x in 0.01..100.0f64,
            neg in any::<bool>(),
            n in 4..=12i32,
            invert in any::<bool>(),
        ) {
            let x = if neg { -x } else { x };
            let n = if invert { -n } else { n };
            let p = <f64 as Real>::powi(x, n);
            let q = naive_powi(x, n);
            prop_assert!((p - q).abs() <= 1e-13 * q.abs());
        }

        /// Lattice laws for min/max on finite values (NaN propagation is
        /// covered by a dedicated unit test). Value equality (==) is the
        /// right comparison here: signed-zero ties differ bitwise by
        /// design (ties keep `self`).
        #[test]
        fn min_max_lattice_laws(
            a in -1.0e9..1.0e9f64,
            b in -1.0e9..1.0e9f64,
            c in -1.0e9..1.0e9f64,
        ) {
            // Idempotence.
            prop_assert_eq!(Real::min(a, a), a);
            prop_assert_eq!(Real::max(a, a), a);
            // Commutativity (as values).
            prop_assert_eq!(Real::min(a, b), Real::min(b, a));
            prop_assert_eq!(Real::max(a, b), Real::max(b, a));
            // Associativity.
            prop_assert_eq!(
                Real::min(Real::min(a, b), c),
                Real::min(a, Real::min(b, c))
            );
            prop_assert_eq!(
                Real::max(Real::max(a, b), c),
                Real::max(a, Real::max(b, c))
            );
            // Absorption.
            prop_assert_eq!(Real::max(a, Real::min(a, b)), a);
            prop_assert_eq!(Real::min(a, Real::max(a, b)), a);
            // Selection and ordering.
            let lo = Real::min(a, b);
            let hi = Real::max(a, b);
            prop_assert!(lo == a || lo == b);
            prop_assert!(hi == a || hi == b);
            prop_assert!(lo <= a && lo <= b);
            prop_assert!(hi >= a && hi >= b);
        }

        /// sqrt(x)² recovers x to ~2 ulps relative (sqrt and the squaring
        /// each contribute ≤ half an ulp of correctly rounded error).
        #[test]
        fn sqrt_square_roundtrip(x in 1.0e-12..1.0e12f64) {
            let r = Real::sqrt(x);
            prop_assert!(r >= 0.0);
            prop_assert!((r * r - x).abs() <= 1e-15 * x);
        }

        /// floor postconditions: integral, ≤ x, within 1 of x — and the
        /// defining bracket floor(x) ≤ x < floor(x) + 1.
        #[test]
        fn floor_bracket(x in -1.0e9..1.0e9f64) {
            let f = <f64 as Real>::floor(x);
            prop_assert_eq!(f, f.trunc());
            prop_assert!(f <= x && x < f + 1.0);
        }

        /// copysign postconditions: |result| = |x| bitwise in the
        /// magnitude bits, sign = sign of the sign argument.
        #[test]
        fn copysign_magnitude_and_sign(
            x in -1.0e9..1.0e9f64,
            s in -1.0e9..1.0e9f64,
        ) {
            let r = <f64 as Real>::copysign(x, s);
            prop_assert_eq!(Real::abs(r).to_bits(), Real::abs(x).to_bits());
            prop_assert_eq!(r.is_sign_negative(), s.is_sign_negative());
        }

        /// reduce_periodic lands within the documented seam blur of
        /// [0, period), and the input differs from the result by an
        /// integer number of periods up to rounding.
        #[test]
        fn reduce_periodic_lands_in_period(
            x in -1.0e6..1.0e6f64,
            p in 1.0e-3..1.0e3f64,
        ) {
            let r = <f64 as Real>::reduce_periodic(x, p);
            // Seam blur: a few roundings of the largest intermediate
            // (p·floor(x/p) ≈ |x|), per the trait's honest statement.
            let blur = 4.0 * f64::EPSILON * (x.abs() + p);
            prop_assert!(r >= -blur, "r = {} below the blurred seam", r);
            prop_assert!(r < p + blur, "r = {} above the blurred period", r);
            // (x − r)/p is an integer up to the same rounding scale.
            let k = (x - r) / p;
            prop_assert!((k - k.round()).abs() <= 1e-6, "k = {}", k);
        }

        /// abs is even, non-negative, and value-preserving in magnitude.
        #[test]
        fn abs_properties(x in -1.0e12..1.0e12f64) {
            prop_assert_eq!(Real::abs(-x), Real::abs(x));
            prop_assert!(Real::abs(x) >= 0.0);
            prop_assert_eq!(Real::abs(x), if x < 0.0 { -x } else { x });
        }
    }
}
