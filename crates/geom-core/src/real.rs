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

    /// Is this value the type's **poison** (module docs: NaN at `f64`,
    /// NaI/empty at the interval scalar, a poisoned value channel at
    /// the dual scalar)?
    ///
    /// This is a *value-channel* question — the one every scalar can
    /// answer without a bracket — and it exists for **structure
    /// discrimination**, not for deciding geometry: the first consumer
    /// is `NurbsSurface::is_placeholder` (M6-3), which must tell the
    /// all-poison "no description yet" placeholder from a described
    /// control net at every evaluation scalar. Predicates on real
    /// margins keep going through `Decide`, whose poison arm carries
    /// the diagnostic; this method never replaces one.
    fn is_poison(self) -> bool;

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

    /// The index of the `period`-branch of `self` nearest zero:
    /// `⌊self/period + ½⌋`, the integer `k` for which `self − k·period`
    /// lies in `[−period/2, period/2)`.
    ///
    /// The **branch-pin** primitive: a consumer holding a raw periodic
    /// coordinate `raw` and a reference `near` shifts onto the branch
    /// nearest the reference by `raw + (near − raw).periodic_branch(p)·p`.
    /// Same construction rules as [`Real::reduce_periodic`] — a fixed
    /// composition of `÷`, `+` and [`Real::floor`], comparison-free, so
    /// the `Dual` value channel is bit-identical to the plain-`T` run
    /// and the interval instantiation contains the true index by
    /// composition of containments.
    ///
    /// **Where its enclosure is wide, and why that is honest**: at
    /// interval type the result spans two integers exactly when the box
    /// straddles a half-period offset — `self ≈ (k + ½)·period`, the
    /// configuration in which the two nearest branches are equidistant
    /// and the pin is a genuine tie. A consumer that must not be handed
    /// a tie classifies the distance-to-tie through the predicate layer
    /// first, like every other decision.
    fn periodic_branch(self, period: Self) -> Self {
        (self / period + Self::from_f64(0.5)).floor()
    }

    /// Range reduction into the period **centred on zero**: the
    /// representative of `self` modulo `period` lying in
    /// `[−period/2, period/2)`, up to the same rounding statement
    /// [`Real::reduce_periodic`] carries.
    ///
    /// This is the reduction for a **signed** periodic quantity — a
    /// difference of two angular coordinates, where "a little backward"
    /// must read as a small negative number and not as almost a whole
    /// period. [`Real::reduce_periodic`]'s `[0, period)` window is the
    /// reduction for an *extent*, which is forward by construction.
    ///
    /// # Fold the raw difference; never a `[0, period)` reduction first
    ///
    /// Both windows are discontinuous, and which reduction to use is
    /// decided by **where each one puts its jump**. `reduce_periodic`
    /// jumps at multiples of the period — at a difference of ZERO,
    /// which is precisely the value a coincidence gate is built to
    /// recognise. This reduction jumps at half-period offsets instead,
    /// so a difference near zero is in the window's interior and comes
    /// straight back: `⌊x/p + ½⌋` is `0` there, and the result is
    /// `x − p·0 = x`, exactly.
    ///
    /// **Where "there" ends, in floating point.** The identity holds
    /// for every `x` with `fl(fl(x/p) + ½) < 1`, which is very nearly
    /// but not exactly `(−p/2, p/2)`: rounding can carry the sum up to
    /// `1` a float or two BELOW the half period. At `p = τ` the top two
    /// floats of `[0, π]` do exactly that — `fl(π/τ)` is exactly `0.5`
    /// (τ is `2·fl(π)`, doubling is exact) and `nextbelow(π)/τ` rounds
    /// to `0.49999999999999994`, which `+ ½` rounds half-to-even back
    /// up to `1.0` — so both return `x − p` rather than `x`. State the
    /// rounding condition, not the mathematical interval, wherever a
    /// gate is made to depend on this identity. One further caveat:
    /// `x = −0.0` returns `−0.0` here and `+0.0` from
    /// [`Real::reduce_periodic`]; equal in value, different in bits.
    ///
    /// The consequence at interval type is the reason this helper is
    /// named rather than open-coded at each site: an argument box
    /// straddling zero reduces to a box of the SAME WIDTH, where
    /// composing this fold on top of a `[0, period)` reduction of the
    /// same quantity would hand the outer fold a box already widened to
    /// a whole period by the inner one. The composition is the shape to
    /// avoid; folding the raw difference once is the shape to write.
    ///
    /// Its own jump, at `±period/2`, is a real discontinuity of the
    /// signed representative and a box straddling it honestly encloses
    /// both signs — a consumer that cannot accept that classifies the
    /// distance to the half-period through the predicate layer.
    fn reduce_periodic_centred(self, period: Self) -> Self {
        self - period * self.periodic_branch(period)
    }
}

/// Bound extraction for **certification and driver code** — deliberately a
/// separate trait, never folded into [`Real`].
///
/// [`Real`] omits bound extraction so evaluation code cannot silently
/// collapse an interval to a number (see the [module docs](self)); this
/// trait is the separate door those docs promised. Its scope is a named
/// style rule under the evaluation-code discipline (M0 L7):
/// `Bounds` may appear only in **certification and
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
/// (`topo::separation` — LIB-PLACEDUNION's placement certificate —
/// falls under this same 2026-07-29 amendment rather than a new one:
/// it is a certified box constructor beside its invariants plus a
/// query driver over the C10 tree, and decides no topology. It is on
/// the CI allowlist for that reason.)
///
/// **Correction (2026-08-19, D1's adversarial review).** That entry used
/// to end *"and its boxes only ever refuse"*. That is the wrong
/// direction and it matters now that `Dual: Bounds`. `Separation::of`,
/// `Separation::certify` and `image` are `T: Decide + Bounds` with **no**
/// [`CertifiedEnclosure`], and box NON-overlap is precisely a GRANT:
/// `certify`'s own doc says *"`Ok(())` is the certificate"*, and
/// `topo::graft_disjoint_all_keyed` asserts nothing about its operands
/// (#382), so nothing downstream re-checks it. The door is instantiable
/// at `Dual64` and at `Dual<Interval>` — verified by compilation.
///
/// It is nonetheless **sound at a dual, on a different justification**:
/// delegation. Every box endpoint a dual produces is its value channel's,
/// which is the plain-`T` run's bit-identically (D9), so a dual run's
/// certificate is the base scalar's certificate — the `f64` run's at
/// `Dual<f64>`, the `Interval` run's at `Dual<Interval>`. No wrong
/// certificate exists.
///
/// **Whether `separation` should carry [`CertifiedEnclosure`] was a
/// #643-completeness question left open here. Answered NO, and the
/// CALLER decides it, not the door.** `Separation::of`/`certify`'s one
/// production caller is `editor_core::eval::wire::wire_placed_union`,
/// beneath `evaluate<T>` — a MIXED pass whose node kinds are
/// overwhelmingly non-certifying, which a [`CertifiedBounds`] bound
/// here would reach by propagation. Doors tighten; passes keep their
/// lanes. That these three signatures return NON-generic types is true
/// and is why nothing a dual gets from them is WRONG, but it does not
/// decide it: `topo::chart_region_overlap` has that property and WAS
/// tightened, nothing generic calling it. Whether an answer is wrong at
/// a dual is a third question with its own home — `geom::projection`'s
/// `mid`, on freezing a selection (#874).
///
/// **Ratified extension (M5 PR 11, Evan's lane-split ruling):**
/// `topo::props`'s certified-quadrature plumbing joins the compound
/// allowlist. The quadrature lane simultaneously decides (its
/// `props_quad_*` funnel margins) and reads brackets into the C9 ring
/// (certification substrate), so `T: Decide + Bounds` is its honest
/// signature; the split from NON-CERTIFYING scalars (duals) is STATIC —
/// `topo::props::PropsQuadLane`'s explicit per-scalar impls are the
/// only entry, and the dual impl instantiates none of it.
/// (Written before #643 and before D1 as "the split from bracket-free
/// scalars". Since the **D1 ruling** of 2026-08-19 a dual is not
/// bracket-free: it carries the value channel's bracket. What it may
/// not do is certify. The split therefore stands on two things now, not
/// one — `PropsQuadLane` at the API, and, since #643, the
/// `quad_lane::*` signatures' own third term [`CertifiedEnclosure`],
/// which no `Dual` implements.)
///
/// **Extension (M5 PR 12, ORCHESTRATOR ruling 2026-08-03, applying
/// the PR 11 precedent; retroactive Evan review per the self-merge
/// convention):** the
/// **fillet-validity battery** — `sweep::fillet::battery`, the M6-1
/// in-place surgery (`sweep::fillet::surgery` — the same lane, the same
/// clearance-margin class; extended under the same ruling), and the
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
/// What differed from PR 11 was only the SPLIT, and it differed because
/// there was nothing to split: no dual-scalar path could reach this
/// code, since [`Bounds`] had no [`Dual`](crate::Dual) impl. A
/// `PropsQuadLane`-style static lane split would therefore have had an
/// EMPTY refusing side, so the seam was ratified instead.
///
/// The seam's signatures are satisfiable at a dual (`Bounds` has a
/// `Dual` impl since D1, and the doors are `pub` on an API-first
/// kernel), and this remains the one allowlisted `Decide + Bounds`
/// seam with no refusing lane. **The written reason it needs none is
/// the delegation rule below**, whose test the seam's reads were
/// enumerated under (twice, independently — the full enumeration is
/// PR #682's body): all fourteen `Bounds` reads across
/// `battery.rs`/`build.rs`/`surgery.rs` have every predicate's
/// `Ok`/`Err` coming from a `decide(...)` call; ten reads are
/// typed-error payloads, four are selections, and the two that feed a
/// classification or a mutation (`battery.rs` → `chain_g1`;
/// `surgery.rs` → `body.split_edge`) are sound by value-channel
/// delegation. Nothing mints a certificate object. A `FilletLane`
/// whose refusing side would be empty is dead code, not a guard.
/// Recorded here because this entry is what a reader consults;
/// `scripts/gates/bounds-allowlist.sh` points at it rather than
/// restating it.
///
/// **The delegation rule (DUAL-DESIGN DL5) — the standing criterion
/// for a lane-less `Bounds` seam.** A `Bounds` read is lane-exempt
/// when it (a) feeds an error payload or report, or (b) selects among
/// constructions whose classification is value-channel-decided AND
/// whose selected quantity is locally constant in the parameters —
/// sound by value-part delegation: a dual's bracket is its value
/// channel's ([`Dual`](crate::Dual)'s `Bounds` impl), so the read's
/// branch is the base scalar's branch. The locally-constant condition
/// is load-bearing, not decoration: a frozen `f64` choice is
/// tangent-sound only while the chosen quantity cannot move with a
/// seed — `geom::projection`'s `mid` freeze (issue 874's class, the
/// `separation` entry above and `dual.rs`'s harvest note both name
/// it) is the live counterexample shape when it is not. A read that
/// MINTS a certificate object or feeds a [`CertifiedEnclosure`]
/// consumer is never exempt: it needs a refusing lane in the
/// `PropsQuadLane` shape, and admitting one without a lane would be a
/// ratified REVERSAL of DL5 on its own evidence — not an entry this
/// rule can grow.
///
/// **Extension (M6-2, authorized under the PR 11/PR 12 precedent;
/// retroactive Evan review per the self-merge convention):** the **SSI rung-3 certificate** —
/// `geom_brep::ssi` (the `certify_rung3` door),
/// `geom_brep::ssi::certify` (the three limbs) and
/// `geom_brep::pcurve_cache`'s fitted lane — joins the compound
/// allowlist. It is the quadrature seam's class exactly: it
/// simultaneously DECIDES (its `ssi_on_locus`, `ssi_hull_sup`,
/// `ssi_tube_transversality`, `pcurve_*` funnel margins) and CONSUMES
/// ENCLOSURES — limb 2 is a control-hull bound and limb 3 a box-chain
/// enclosure, both computed in the C9 ring, which is reached from a
/// scalar only through its bracket. So `T: Decide + Bounds` is the
/// honest signature.
///
/// Two things distinguish it from the PR 12 seam, and both cut toward
/// ratifying it rather than against. First, the SPLIT is real and is
/// written: `geom_brep::PcurveFittedLane` has certified impls for
/// `f64`, [`Probe`](crate::Probe) and the interval scalar and a
/// **refusing** impl for [`Dual`](crate::Dual) — the `PropsQuadLane`
/// shape, with a non-empty refusing side, because dual bodies really do
/// validate and really cannot hold a fitted cache. Second, the
/// narrowest file set was taken: `geom_brep::ssi::enclose` — the ring
/// machinery itself — decides nothing and therefore takes the
/// **sole-bound** `T: Bounds` the rule already allows, and is not
/// allowlisted.
///
/// **Extension (M7-8, under Evan's #264 ruling):**
/// `geom_brep::edge_nurbs` — the plane × NURBS declare-and-check edge
/// lane — joins the allowlist as the narrowest possible extension of
/// the M6-2 seam. It adds no new obligation: it DELEGATES to the
/// already-listed `certify_rung3` door, handing it a **declared**
/// carrier instead of a marched one, and therefore inherits that
/// door's signature rather than widening the rule's reach. The split
/// is written in the ratified shape — `geom_brep::EdgeNurbsLane` has
/// certified impls for `f64`, [`Probe`](crate::Probe) and the interval
/// scalar and a **refusing** impl for [`Dual`](crate::Dual) — and it
/// is precisely what keeps `Bounds` off `topo`'s DEFAULT doors: the
/// lane is a SEPARATE door whose own impl block carries the lane
/// bound (`Body::set_edge_curve_nurbs_lane`), with `_via(…, f)`
/// parameterising the shared machinery behind it. Injection moves a
/// bound onto a narrower signature; it does not remove one.
///
/// **Extension (M9-2 PR-1, under the PR 11 precedent; retroactive
/// Evan review per the self-merge convention):** `topo::chart_region` — the chart-region overlap
/// predicate — joins the compound allowlist. It is the quadrature
/// seam's class exactly: it simultaneously DECIDES (its
/// `chart_region_*` funnel margins) and reads **exact-`f64`
/// structure** through the bracket — the spec-mandated C6 inventory
/// gate (a `Harmonic` trig channel is straight only when its bracket
/// is a point at exactly `0.0`; the `props.rs` rectangle-trim read)
/// plus the bit-identical-region fast path — so a compound bound is its
/// honest signature; a sole-bound form is unsatisfiable.
///
/// **The door and the lane guard different things, and both are
/// needed.** The door's bound is `Decide + `[`CertifiedBounds`], which
/// no [`Dual`](crate::Dual) satisfies, so the predicate is
/// uninstantiable at a dual however it is reached — including from
/// outside the crate, where the lane is never consulted; this seam is
/// no longer the exception among the four. `ChartRegionLane`'s refusing
/// `Dual` impl is NOT redundant with that: it is what lets the census,
/// a MIXED pass, decline this one arm and keep going, which no bound on
/// a whole function can express.
///
/// The tightening replaced an audit, not a wrong answer — `ChartOverlap`
/// is not generic in `T`, so a dual run's answer was the value
/// channel's exactly. **That is not what decided it**: `topo::separation`
/// shares the property and was NOT tightened, the difference being that
/// nothing generic calls this door (the `separation` entry above).
///
/// **Extension (2026-08-29, ratified by Evan in conversation):**
/// `editor_core::checks` — the advisory-check registry — joins the
/// compound allowlist as the **second production caller** of
/// `topo::separation`, alongside
/// `editor_core::eval::wire::wire_placed_union`. Its bound is
/// `Decide + `[`CertifiedBounds`], **not** `Decide + `[`Bounds`].
///
/// **What the ruling says the rule is FOR**, in Evan's words: the gate
/// exists "to avoid the dangerous pattern when not necessary, so if it
/// is necessary it's fine". That reading applies to every entry above
/// and every one that follows — the rule is not a budget on how many
/// seams may exist, and an extension is not earned by RESEMBLING one
/// already listed. What a candidate owes is a demonstration of
/// **necessity**, and the demonstration is the ratifiable artifact.
///
/// **Necessity is a filter on a candidate, never a licence.** What is
/// NEVER allowed, whatever the necessity argument, is the thing the
/// sole-bound rule exists to prevent and the 2026-07-29 amendment
/// restates in its last clause: **brackets never decide.** Every
/// topology-determining branch stays a
/// [`Decide`](crate::predicate::Decide) call site — a trilean, with
/// its in-band arm — and a bracket may prune a candidate set, drive a
/// subdivision, or be reported; it may not be read off and branched on
/// to reach an evaluated answer. A parameter that needs a bracket in
/// order to decide something OUTSIDE the trilean is not a weak seam
/// candidate, it is precisely the escape hatch the CI grep was written
/// to catch, and no demonstration of necessity redeems it — such a
/// candidate is refused rather than weighed. So an entry owes two
/// things, and the ORDER matters: first that its reads stay on the
/// prune/report side, then that the bound is unavoidable.
///
/// This row clears the first: `editor_core::checks` reads no bracket
/// at all — no `lo`/`hi` call appears in the file — and its
/// bracket-derived verdicts (`SolidSeparation::certify`'s, and
/// `classify_shells`' through the `props_quad_*` funnel) decide only
/// whether a FINDING is emitted. No body, no topology and no evaluated
/// value moves on them: the resident REPORTS and never gates, which is
/// `editor_core::checks`'s own ratified posture (DS6).
///
/// **On the second: this entry got it wrong once, and the correction
/// is the entry's most useful content.** It first carried
/// `Decide + `[`Bounds`] and argued necessity from two negative
/// results — that `topo::PropsQuadLane` does not imply [`Bounds`]
/// (true, and checked by deleting the term), and that a
/// `PropsQuadLane`-style lane would have an empty refusing side since
/// the D1 ruling (also true). **Neither reaches the question.** They
/// establish that SOME bracket bound is needed, never that the WEAK
/// one is, and the tighter bound was never tried. It compiles:
/// `Decide + PropsQuadLane + `[`CertifiedBounds`] builds the workspace
/// with zero errors, because nothing generic calls `run_checks` — its
/// callers are the viewer at a concrete `f64`, the tour, and tests.
///
/// A reviewer found that with a one-line experiment the entry itself
/// should have run. The lesson generalises past this row: a necessity
/// argument must name the WEAKEST bound that works and show the next
/// tighter one failing, or it is an argument that a bound suffices —
/// which is not what this rule asks.
///
/// **The tightening has a consequence that crosses a unit boundary,
/// recorded here because that is where it will be looked for.** No
/// [`Dual`](crate::Dual) implements [`CertifiedEnclosure`], so
/// `editor_core::run_checks` is no longer callable at one. M10-DI's
/// `r1_dual_probes` had been observing exactly that reachability and
/// calling it "a gap DL3 does not cover"; it is now CLOSED rather than
/// merely unobserved, and those rows say so. Closing it is DL1 holding
/// one door further out — the registry's separation resident GRANTS a
/// certificate (box non-overlap is a genuine separation claim), and a
/// dual never certifies, which is the sentence `topo::AtRestPolicy` is
/// itself built on.
///
/// **And the precedent had been read backwards.** The `separation`
/// entry's "passes keep their lanes" turns on its caller being a mixed
/// pass BENEATH `evaluate<T>`, which a [`CertifiedBounds`] bound would
/// reach by propagation; `run_checks` is not beneath `evaluate` and
/// propagates to nothing. The M9-2 entry states the actual
/// discriminator — `topo::chart_region` WAS tightened, "the difference
/// being that nothing generic calls this door" — and by that sentence
/// `run_checks` falls on the tighten side. It is now tightened, so the
/// two doors agree.
///
/// The allowlist row is owed either way: the gate's matcher is shaped
/// by the trait NAME and reads `Decide + CertifiedBounds` as a
/// compound bound in both operand orders, which is correct — that is a
/// parameter that decides AND brackets.
///
/// **Provenance.** Unlike the PR 11/PR 12/M6-2/M9-2 extensions above,
/// this one did not go through the self-merge convention: it arose in
/// conversation with Evan and he ruled on it directly, before the PR
/// carrying it merged. No retroactive review is owed.
///
/// **Not an extension — a spelling.** The pair
/// `Bounds + CertifiedEnclosure` — both bracket doors, no `Decide` — is
/// spelled [`CertifiedBounds`] and is therefore a **sole** bound by
/// construction, not an exception to this rule. It is outside the rule's
/// class rather than carved out of it: the rule catches an evaluation or
/// decision parameter that has also been handed bracket extraction, and
/// both halves of that pair are bracket-side doors. Adding
/// [`Decide`](crate::predicate::Decide) to
/// it is a compound bound again, and needs ratification — and
/// `scripts/gates/bounds-allowlist.sh` enforces that: its matcher is
/// shaped by the trait NAME, so it reads `Decide + CertifiedBounds` as a
/// compound bound in either operand order, and a **sole**
/// [`CertifiedBounds`] does not fire.
///
/// **Resolution of #990 (ratified 2026-08-27, Evan's ruling in the
/// issue conversation): the two non-decision shapes, and why neither
/// needs a seam.** VERBS-TUBEWALL met a gap twice: a new door outside
/// the seams had (1) no non-metered spelling for *request validity*
/// (refusing a nonsensical caller input like `thickness ≤ 0` without
/// minting a K-corpus row that meters the CALLER), and (2) no way to
/// echo the *caller's own offending value* in a refusal payload (no
/// `f64` out of a `T: Decide`). Both dissolve without touching this
/// rule's class, because neither is a kernel decision:
///
/// 1. **Request validity dissolves at the signature.** Caller-intent
///    magnitudes enter doors as plain `f64` — or, where the constraint
///    is expressible in the type, as a validating newtype whose
///    constructor is the single refusal site — and are lifted into `T`
///    only after validation. `topo::shell`'s `thickness` is the
///    pattern. No bracket is ever read, no row meters the caller, and
///    the invalid value cannot reach the door at all in the newtype
///    form. A door that takes its request in `T` is the thing to fix,
///    not to allowlist.
/// 2. **Refusal payloads need nothing new — (1) already covers them.**
///    With caller values validated at `f64` before lifting, a refusing
///    door still holds the caller's own number and echoes it freely;
///    #990's motivating case (TUBEWALL reporting the run's threshold
///    instead of the caller's number) cannot recur. What remains are
///    DERIVED quantities — margins, realized radii, values that exist
///    only at `T` — and those are deliberately NOT echoed outside the
///    ratified seams: an `f64` in an error payload is a branchable
///    channel, i.e. the same unmetered decision surface at one remove,
///    so the honest spelling outside a seam is the variant name plus
///    the run's threshold (TUBEWALL's posture — now the rule, not a
///    fallback). A door that wants to echo a derived margin is asking
///    to be a seam, ratified individually (the fillet battery is the
///    precedent). No general projection helper exists, by this ruling.
///
/// For genuine decisions nothing changes: the metered predicate layer
/// is the only spelling — it IS the "definite sign or indeterminate"
/// trilean, plus the two things a bare helper lacks (the NAME in the
/// verdict log, and escalation as the forced disposition of
/// indeterminate). A free-floating bounds-comparison helper would be
/// the #701 `Enclosure` evasion with better manners, and stays out.
///
/// **The direction rule for terminal grants (#571, A′'s design-owner
/// ruling; recorded here per the #1027 durable-home rule — it
/// previously lived only in the decision's history).** A terminal
/// grant from `Bounds` is legitimate only when the granted claim lies
/// in the bound's CONSERVATIVE direction: a box-disjointness answer IS
/// a sound disjointness certificate for the contents
/// (sufficient-not-necessary; touching-or-overlapping boxes with
/// disjoint contents refuse or escalate — the safe failure direction),
/// whereas a terminal grant of an EXISTENCE or overlap claim from
/// boxes would be the violation. The #990 shapes above never reach
/// this rule: neither branches on a bracket, so neither is a terminal
/// grant at all.
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
/// directly. A helper that only needs to READ a bracket takes
/// `T: Enclosure` and works for all three: an `f64` coefficient is a
/// degenerate bracket, an interval-scalar coefficient is the replayed
/// enclosure, a ring coefficient is the certification arithmetic's own.
///
/// **Not "certification helpers", which is what this said before #643 —
/// and the word matters more since D1 (2026-08-19).** `Enclosure` is a
/// bracket accessor; the certification door is [`CertifiedEnclosure`].
/// The spline hull bounds in [`crate::spline::hull`], cited here as the
/// first `Enclosure` consumer, moved to [`CertifiedEnclosure`] at #643,
/// and no `Enclosure`-bounded signature remains in `crates/*/src`. See
/// the blanket impl below for why that is worth saying: a `Dual` is an
/// `Enclosure`, and a new compound `T: Enclosure` bound is gated
/// exactly as a `Bounds` one.
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
/// keeps `f64` and the interval scalar usable by helpers written against
/// the smaller trait.
///
/// **This blanket impl means [`Dual`](crate::Dual) is an `Enclosure` too,
/// since the D1 ruling of 2026-08-19 gave it [`Bounds`].** A compound
/// `Enclosure` bound is therefore the same class of decide-and-bracket
/// parameter as a compound `Bounds` one, and it is gated the same way:
/// `scripts/gates/bounds-allowlist.sh` greps `Enclosure` exactly as it
/// greps `Bounds`, against the same file allowlist (DUAL-DESIGN DL4 —
/// the resolution of the issue-701 gap), so a new `T: Enclosure` bound
/// on certifying code fails CI until it is ratified into the `Bounds`
/// scope rule here.
impl<T: Bounds> Enclosure for T {
    fn lo(self) -> f64 {
        Bounds::lo(self)
    }

    fn hi(self) -> f64 {
        Bounds::hi(self)
    }
}

/// **"May this value enter certified code?"** — the other half of what
/// [`Bounds`] used to mean, given a name of its own.
///
/// [`Bounds`] and [`Enclosure`] answer *"what bracket does this value
/// carry?"*: `[lo(), hi()]` is a superset of every real the value stands
/// for, read off storage, and it stays a sound bracket even when the
/// computation that produced it left a domain somewhere — interval
/// arithmetic still brackets the values the expression *was* defined on.
/// Certification asks a strictly stronger question: *was the expression
/// defined on the whole input box?* A bracket can be sound and still fail
/// that, and code that must not certify a domain violation needs to be
/// able to tell.
///
/// So this trait is not "a better [`Enclosure`]" and does not replace it.
/// It is the access-control half, split out, so that the two questions
/// have separate doors and a caller has to say which one it is asking.
/// It deliberately carries **one method and no supertrait**: a body that
/// needs the raw bracket too holds both doors, and says so with the
/// **sole** bound [`CertifiedBounds`] — still an honest inventory of the
/// doors it uses, which is the point of the alias: the inventory is
/// spelled as one name rather than as a compound bound the
/// compound-`Bounds` gate would have to special-case. Write
/// `T: CertifiedBounds`, not `T: Bounds + CertifiedEnclosure`. Making
/// this a subtrait of
/// [`Enclosure`] would re-bundle exactly what is being split, and would
/// put a third `lo`/`hi` in scope wherever a compound bound is written —
/// the ambiguity this module's style note already warns about for the
/// [`Bounds`]/[`Enclosure`] pair.
/// Certification entry points bound by `CertifiedEnclosure` cannot be
/// handed a value that merely *has* a bracket; containment checks bounded
/// by [`Bounds`] keep working on values certification would refuse, which
/// is exactly right — a `Trv` enclosure still contains what it claims to.
///
/// # Implementors
///
/// - `f64` — refuses on NaN, which is this lane's poison (D4's Q1
///   residue: *∞ is not f64 poison*, so an infinity still certifies the
///   degenerate bracket it is). The bracket is the value, so the value
///   being poison IS the domain-violation channel; there is no second
///   one to consult, and every finite or infinite `f64` certifies.
/// - [`crate::Interval`] — refuses below `Decoration::Def`, the same
///   threshold [`crate::predicate::Decide::sign_within`] refuses at, and
///   for the same reason. Empty and NaI sit below it, so the NaN
///   brackets they store never leave the door.
/// - [`crate::RingInterval`] — refuses on poison. The ring has two
///   states and no decorations, so `is_poison` is its whole
///   domain-violation channel.
/// - `k_stats::Probe` (feature `probe`) — refuses on NaN, byte-for-byte
///   as `f64` does; D9 forbids the recording lane diverging.
///
/// Every one of them therefore honours one postcondition, which is what
/// a generic `T: CertifiedEnclosure` body may rely on: **a `Some` never
/// carries a NaN end**. An infinite end is still possible and is not
/// poison — `[−∞, ∞]` is a sound (useless) bracket of a real, and
/// `Interval` certifies it at `Def`.
///
/// **[`crate::Dual`] is deliberately absent, and the absence is
/// permanent** (`docs/DUAL-DESIGN.md` DL1, closing D1's hedge): a dual
/// is tangent transport and never certifies. `Dual` implements
/// [`Bounds`] and does **not** implement this trait, which is what
/// holds the line — the only door between a dual and certified code.
/// Reopening it would be a ratified reversal of DL1 on its own
/// evidence, never an impl someone can add in passing.
pub trait CertifiedEnclosure: Copy {
    /// The bracket, or `None` if this value carries a domain violation.
    ///
    /// `Some([lo, hi])` promises both things at once: the pair brackets
    /// every real the value stands for **and** the computation behind it
    /// was defined on the whole input box. `None` is the refusal, and it
    /// is a refusal rather than a NaN bracket on purpose — NaN would be
    /// indistinguishable from arithmetic poison and would travel silently
    /// through `f64` combinators (`f64::max` returns the non-NaN operand),
    /// whereas a `None` the caller must destructure cannot be ignored.
    fn certified_bracket(self) -> Option<(f64, f64)>;
}

/// `f64` refuses on NaN and only on NaN: the bracket is the value, so
/// the value being poison is the whole of its domain-violation channel
/// (see the trait docs, and D4's Q1 residue for why ∞ is not poison
/// here).
impl CertifiedEnclosure for f64 {
    fn certified_bracket(self) -> Option<(f64, f64)> {
        (!self.is_nan()).then_some((self, self))
    }
}

/// **Both bracket doors, for code that reads both** — the pair
/// [`Bounds`] + [`CertifiedEnclosure`] under one name, so a body that
/// needs the stored bracket *and* the fallible certified one writes a
/// **sole** bound.
///
/// The two doors answer different questions — `[lo(), hi()]` is the
/// bracket read off storage, `certified_bracket()` additionally promises
/// the computation was defined on the whole input box — and certification
/// code routinely asks both: it builds C9 ring enclosures through the
/// certified door and reads raw endpoints for the containment and padding
/// arithmetic around them. Spelling that `T: Bounds + CertifiedEnclosure`
/// is honest but is a *compound* bound in every mechanical sense, and the
/// compound-`Bounds` rule on [`Bounds`] exists to catch a specific thing
/// this is not (see that rule's `CertifiedBounds` paragraph). Named, it
/// is a sole bound by construction.
///
/// # What this is not
///
/// It is **not** a general-purpose "give me brackets" bound. A parameter
/// that only needs to read endpoints says `T: Bounds`; one that only needs
/// the certified door says `T: CertifiedEnclosure`. Reach for this name
/// only when the body genuinely uses both, and it does not license adding
/// bracket access to evaluation code that had none — the reason to hold
/// brackets out of evaluation signatures is unchanged by giving the pair a
/// shorter spelling.
///
/// It is **not** an escape from the compound-`Bounds` rule.
/// `T: Decide + CertifiedBounds` is a compound bound, fires
/// `scripts/gates/bounds-allowlist.sh`, and needs ratification —
/// correctly so, because that is exactly the thing the rule targets: one
/// parameter that both DECIDES and reads brackets. So the guidance above
/// — write the alias, not the pair — is safe to follow at a `Decide`
/// site: it changes the spelling and not what is ratified.
/// `geom_brep::ssi::certify`'s `probe_tube_chart` is that shape, writes
/// the long form today, and is allowlisted by file on its own
/// justification either way.
pub trait CertifiedBounds: Bounds + CertifiedEnclosure {}

/// Every scalar with both doors has the pair; the alias adds no obligation
/// an implementor must opt into.
impl<T: Bounds + CertifiedEnclosure> CertifiedBounds for T {}

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

    /// `f64`'s one poison value is NaN.
    fn is_poison(self) -> bool {
        self.is_nan()
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
