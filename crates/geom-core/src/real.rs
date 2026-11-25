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

use crate::tolerance::Tol;

/// **What the registered-identity door did with one registration**
/// ([`crate::Sym::register_equal`], ERROR-DESIGN E12's provenance reserve).
///
/// Every arm is a REFUSAL or a record, and none of them is silent:
/// the door answers what it did, so a registrant that wanted to be
/// loud can be and a pin can read it.
///
/// **Seven flat arms over two axes, and that is a decision** (a review
/// flagged the flattening; this is the call). The axes are the WITNESS
/// — did the value channel find the two values one real: witnessed,
/// refused by an exact witness, refused by an inexact one, or unable to
/// say — and the REGISTRY — recorded,
/// already there, refused as cyclic, or not consulted. A struct of two
/// fields would name them separately and would also make
/// `{Contradicted, Recorded}` spellable, which is a state the door must
/// never be in; a registrant would then match twice to learn one thing.
/// The seven arms are exactly the reachable combinations, so the
/// impossible ones cannot be written down, and a call site reads one
/// answer. The cost, stated: "was this refused?" is a two-arm match
/// rather than a field read, and every registrant pays it by hand
/// (`work/sym/sym-registration-flattens-two-axes`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SymRegistration {
    /// Recorded: from here on the two nodes denote one function of the
    /// parameters in this session's EARLY normal form, and a decision
    /// that rests on the record is counted `crate::SymCounts::registered`.
    Recorded,
    /// Already recorded — the same two nodes, or two nodes the registry
    /// already resolves to one. Idempotent, and cheap: nothing is
    /// invalidated.
    Already,
    /// **REFUSED by an EXACT witness: the two values are PROVED not one
    /// real.** Reserved for [`crate::Interval`], whose certified
    /// enclosures of the two sides are DISJOINT over the leaf's box.
    /// There is no slack in that test and nothing to lose at the scale,
    /// so the answer is a proof of a defect: either the constructor did
    /// not build what it claims, or an upstream enclosure does not
    /// contain its real. A registrant may therefore `debug_assert!` on
    /// this arm, and the swept registrants do. `f64` and
    /// [`crate::Probe`] never answer it — an inexact witness answers
    /// [`SymRegistration::Disputed`].
    ///
    /// **A registrant's assertion on this arm is LIVE IN RELEASE, and
    /// that is a deliberate change of behaviour.** This workspace ships
    /// `[profile.release] debug-assertions = true`, so a registrant that
    /// `debug_assert!`s here ABORTS in every profile, where before the
    /// arm split the same configuration was a counted refusal in all of
    /// them. It is the right shape: at an exact witness a refusal is a
    /// proof of a soundness defect somewhere upstream, and this codebase
    /// fails loud rather than recording a defect in a column. What is
    /// bought with it is that nothing downstream can rest on a
    /// registration the exact witness disproved.
    ///
    /// Nothing is recorded, the registry is unchanged, and every
    /// decision that would have rested on the record stays numeric.
    Contradicted,
    /// **REFUSED by an INEXACT witness: the two values are apart by more
    /// than its slack, and it cannot say which claim that is.** The
    /// answer of `f64` and [`crate::Probe`], which compare at a point
    /// with a slack of the run's ε relative to the larger magnitude
    /// ([`Real::register_equal`]'s `f64` impl). The registration may be
    /// a lie; it may equally be a theorem of the reals whose two sides
    /// separate because the arithmetic ran out of significand at this
    /// scale — an adversarial torus at a minor radius of 10¹⁸ with walls
    /// below one ULP reaches exactly that, and nothing is wrong there.
    /// **Never asserted on.** The gap cannot tell the two apart (at that
    /// torus the sides are many ULPs apart), which is why the arm is
    /// keyed to the KIND of witness and not to a second threshold.
    ///
    /// Nothing is recorded, and the refusal is counted exactly as
    /// [`SymRegistration::Contradicted`] is
    /// ([`crate::SymCounts::registrations_refused`]).
    Disputed,
    /// **REFUSED, typed: the registration would close a cycle** — the
    /// right node's expression already contains the left one, so
    /// aliasing them would make the normal form's walk non-terminating.
    /// (`form_in`'s termination argument is structural: a node's id is
    /// a hash of its children's, so a cycle needs a hash preimage. The
    /// registry is the one thing that could introduce one by hand, and
    /// this arm is what keeps that argument true.)
    Cyclic,
    /// **The claim was witnessed and nothing was recorded.** Either the
    /// scalar tracks no expressions (`f64`, `Interval`, `Probe`: there
    /// is nothing at a bare scalar to record an identity ABOUT), or
    /// there is one and nowhere to put it — no session installed, the
    /// tier off at a zero-term budget, or [`crate::SymRules::registered`]
    /// off.
    Witnessed,
    /// **This scalar's value channel cannot witness the claim**, so
    /// nothing is claimed and nothing is recorded — the default arm of
    /// [`Real::register_equal`] and what every scalar that tracks no
    /// expressions answers. A poisoned value answers this too: an
    /// expression with no value witnesses nothing.
    Unwitnessed,
}

impl SymRegistration {
    /// **What a registrant does with the door's answer**, by arm, in
    /// one place for every registrant.
    ///
    /// - [`SymRegistration::Contradicted`] is the EXACT witness's proof
    ///   that `what` is false on the values the registrant built, and is
    ///   loud: either the registrant's theorem is false for the
    ///   configuration it was handed, or an upstream enclosure does not
    ///   contain its real. Live in release (the arm's own doc).
    /// - [`SymRegistration::Disputed`] is an inexact witness's refusal,
    ///   counted in the session's receipt and never asserted on.
    /// - Every other arm is a record, a no-op, or nothing to record, and
    ///   none of them is a defect.
    ///
    /// Exhaustive by hand, so a new arm is a compile error here.
    pub fn handle(self, what: &str) {
        match self {
            Self::Contradicted => debug_assert!(
                !matches!(self, Self::Contradicted),
                "the EXACT witness separated {what}: either the registrant's theorem is \
                 false for the configuration it was handed, or an upstream enclosure does \
                 not contain its real"
            ),
            Self::Disputed
            | Self::Recorded
            | Self::Already
            | Self::Witnessed
            | Self::Unwitnessed
            | Self::Cyclic => {}
        }
    }
}

/// **What a comparison at this scalar PROVES** — the property that
/// decides, per lane scalar, whether a disagreement between the value
/// channel and a symbolic form is a soundness defect or a dispute.
///
/// It is a property of the SCALAR, fixed at compile time
/// ([`Real::WITNESS`]), and never a value read: nothing branches on a
/// number to obtain it, so D9 is untouched and it is not the
/// instrument [`crate::Decide::enclosure_probe`] is.
///
/// **Two contracts read it, and that is the point.** The
/// registered-identity door forwards the lane scalar's refusal arm —
/// [`SymRegistration::Contradicted`] from an exact witness,
/// [`SymRegistration::Disputed`] from an inexact one — and
/// `Sym<T>::sign_within` checks a symbolic `Zero` against its enclosure
/// at an exact witness only, where the enclosure is a proof. Both are
/// the same question about the same channel, so they read one marker
/// rather than two roster copies.
///
/// **That they cannot drift is a property of the PIN, not of the
/// marker**, and the difference is one a first cut got wrong: a const
/// read in two places still lets a scalar declare one thing and answer
/// another. What closes it is that
/// `geom-core/tests/sym11_witness_kind_rows.rs`'s `witness_agrees` is
/// GENERIC — one predicate instantiated at every `impl Real` in the
/// tree by name — so an impl whose const and whose refusal arm
/// disagree reds without anyone having written that pair down.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Witness {
    /// **A comparison here is a PROOF.** The value channel is a
    /// certified enclosure of the real ([`crate::Interval`]), so two
    /// enclosures that do not meet prove the two reals differ, and an
    /// enclosure that excludes zero proves the margin is not zero.
    /// There is no slack in either test and no scale at which the
    /// answer is the arithmetic giving up, so a disagreement with a
    /// form is a defect in one of the two channels: a proven zero whose
    /// enclosure excludes zero fails a `debug_assert!` where the tier
    /// returns it (`sym::discharge_checked`), and a zero the tier would
    /// prove under a definite sign, which the decision does not ask, is
    /// the test instrument's report (`sym::contradiction`).
    Exact,
    /// **A comparison here is a MEASUREMENT of one rounded number.**
    /// `f64` and [`crate::Probe`] evaluate at a point: at a far
    /// placement the rounding exceeds the band, under rule F a one-ulp
    /// error in a sign argument becomes a whole `2.0` at the margin,
    /// and at a pole the channel has no clause 1 to refuse with. None
    /// of those is a false claim by the form, so none of them may be
    /// charged to the form: the numeric answer is kept.
    Inexact,
}

/// **The plain formula `(q − o)·n`**, in [`crate::Vec3::dot`]'s order:
/// the reading a scalar falls back to where its posed one
/// ([`Real::difference_dot`]) does not apply.
#[must_use]
pub fn plain_difference_dot<T: Real>(q: [T; 3], o: [T; 3], n: [T; 3]) -> T {
    (q[0] - o[0]) * n[0] + (q[1] - o[1]) * n[1] + (q[2] - o[2]) * n[2]
}

/// **The plain formulas for a depressed quartic's `Δ` and
/// `D = 64s − 16p²`**: the reading a scalar falls back to where its posed
/// one ([`Real::quartic_sign_algebra`]) does not apply.
#[must_use]
pub fn plain_quartic_sign_algebra<T: Real>(p: T, q_hat: T, s: T) -> [T; 2] {
    let (p2, q2, s2) = (p.powi(2), q_hat.powi(2), s.powi(2));
    let disc = T::from_f64(256.0) * s2 * s - T::from_f64(128.0) * p2 * s2
        + T::from_f64(144.0) * p * q2 * s
        - T::from_f64(27.0) * q2.powi(2)
        + T::from_f64(16.0) * p2.powi(2) * s
        - T::from_f64(4.0) * p2 * p * q2;
    [disc, T::from_f64(64.0) * s - T::from_f64(16.0) * p2]
}

/// **A first harmonic's curve as it is stored**: the point `at` it
/// stores, its unit `axis` and seam `u_ref`, its semi-axes `(a, b)`, and
/// the phase `(c₀, s₀)` at which `at` lies on it, so that its point at
/// `θ` is `at + u_ref·a·(cos θ − c₀) + (axis × u_ref)·b·(sin θ − s₀)`.
/// A conic held at its centre states the phase `(0, 0)`; an ellipse held
/// on its arc, its anchor's. [`Real::harmonic_belly`] reads it.
#[derive(Clone, Copy, Debug)]
pub struct StoredHarmonic<T> {
    /// The stored point.
    pub at: [T; 3],
    /// The stored unit axis.
    pub axis: [T; 3],
    /// The stored unit seam.
    pub u_ref: [T; 3],
    /// The semi-axes along the seam and across it.
    pub semi: (T, T),
    /// Where on the curve `at` lies: `(0, 0)` at its centre.
    pub phase: (T, T),
}

impl<T: Copy> StoredHarmonic<T> {
    /// Every scalar of the placement, read through `f`.
    #[must_use]
    pub fn map<U>(self, f: impl Fn(T) -> U) -> StoredHarmonic<U> {
        StoredHarmonic {
            at: self.at.map(&f),
            axis: self.axis.map(&f),
            u_ref: self.u_ref.map(&f),
            semi: (f(self.semi.0), f(self.semi.1)),
            phase: (f(self.phase.0), f(self.phase.1)),
        }
    }
}

/// **The plain formula for [`Real::harmonic_belly`]**: `√(A² + B²) − |D|`
/// with every term rounded as it is formed: the reading a scalar falls
/// back to where its posed one does not apply.
#[must_use]
pub fn plain_harmonic_belly<T: Real>(curve: StoredHarmonic<T>, o: [T; 3], n: [T; 3]) -> T {
    let StoredHarmonic {
        at,
        axis: w,
        u_ref: u,
        semi: (a, b),
        phase: (c0, s0),
    } = curve;
    let v = [
        w[1] * u[2] - w[2] * u[1],
        w[2] * u[0] - w[0] * u[2],
        w[0] * u[1] - w[1] * u[0],
    ];
    let dot = |p: [T; 3]| p[0] * n[0] + p[1] * n[1] + p[2] * n[2];
    let (big_a, big_b) = (a * dot(u), b * dot(v));
    let d = plain_difference_dot(at, o, n) - c0 * big_a - s0 * big_b;
    (big_a.powi(2) + big_b.powi(2)).sqrt() - d.abs()
}

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
    /// **What a comparison at this scalar proves** ([`Witness`]) —
    /// declared, never defaulted, because the two things that read it
    /// both fail in the same direction when a scalar under-declares:
    /// [`Real::register_equal`]'s refusal arm and the theorem-vs-numeric
    /// charge at `Sym<T>::sign_within`. **No default, so the compiler
    /// is what asks**: a new lane scalar does not compile until it says
    /// which side of the partition it is on, and nothing has to notice
    /// that it did not.
    ///
    /// Its [`Real::register_equal`] arm then has to AGREE, which the
    /// compiler cannot ask — `witness_agrees` in
    /// `geom-core/tests/sym11_witness_kind_rows.rs` does, generically,
    /// at every `impl Real` in the tree: `Exact` ⇔ a separated pair is
    /// refused `Contradicted`, `Inexact` ⇔ `Disputed`.
    const WITNESS: Witness;

    /// **This scalar's name, as a refusal names it** — prose written to
    /// sit inside "at the … scalar" (`"interval"`, `"dual"`), and the
    /// one home for it: every refusal that names the scalar it ran at
    /// reads it off its own type parameter, so two refusals cannot
    /// spell one scalar two ways. Declared, never defaulted,
    /// so a new scalar is asked by the compiler; a wrapper states its
    /// own name rather than composing its base's.
    const NAME: &'static str;

    /// **Whether `other` is this value, so that a certificate whose
    /// support holds it may be read rather than derived again** — the
    /// one knob of reading by support (`geom_brep::LocusCertificate`,
    /// a pcurve row's support): every comparison of a support reads its
    /// values through this, so a scalar that answers `false` reads no
    /// certificate and derives each where it is asked.
    ///
    /// `true` only where a derivation is a pure function of its support
    /// down to the bits of what it returns and records nothing a read
    /// would drop: the same bits at a value scalar, the same enclosure
    /// and decoration at an interval. `false`, the default, is always
    /// sound: a dual holds no certificate, and a scalar whose
    /// derivations are its record (the symbolic tier's decision log, the
    /// recording scalar's K samples) answers `false` by its own impl.
    fn same_value(self, other: Self) -> bool {
        let _ = (self, other);
        false
    }

    /// **What a certifying door's record is held as at this scalar**:
    /// the record itself where the scalar certifies, and
    /// [`Uncertified`] where it never does (a dual, DL1). An edge a
    /// door attaches carries its certificate in this shape, so a body
    /// at a tangent-transport scalar carries none by type (DL3), and a
    /// reader at a certifying scalar reads the record with no arm for
    /// its absence.
    ///
    /// Declared, never defaulted, as [`Real::WITNESS`] is: a new scalar
    /// says which side of the partition it is on. A record is held only
    /// through [`CertifiedEnclosure::carried`], so a scalar that holds
    /// records and cannot certify is not constructible from this side.
    type Carries<C: Copy + Debug + Send + Sync + 'static>: Copy + Debug + Send + Sync + 'static;

    /// **The expression this scalar carries**: at [`crate::Sym`] its
    /// node in the document's expression graph, and at every numeric
    /// scalar a type with no values ([`NoExpression`]). What is asked
    /// of an expression is asked of this type ([`Expression`]), so a
    /// question about the expression cannot be put to a number.
    ///
    /// One sentence per scalar: `Sym<T>` carries its node, the
    /// expression its value was computed as, or the identity a margin
    /// was given in its place ([`crate::Decide::free_identity`]); `f64`,
    /// [`crate::Interval`], [`crate::Dual`] and [`crate::Probe`] carry a
    /// value and no expression.
    type Expression: Expression;

    /// This value's expression: the node at [`crate::Sym`], and nothing
    /// at a numeric scalar, where `Option<NoExpression>` has the one
    /// value.
    fn expression(self) -> Option<Self::Expression>;

    /// **This scalar with its tangent removed**: the scalar a place is
    /// evaluated at and a decision reads, which DL5.2 (the crate README)
    /// calls the value channel. It is not [`crate::SpanLocate`]'s
    /// `value_channel`, an `f64` a structure decision reads (NaN where a
    /// value is wider than a point). What it is at each scalar:
    ///
    /// - `f64`, [`crate::Interval`], [`crate::Probe`]: the scalar itself,
    ///   which carries no tangent.
    /// - `Dual<f64>`, `Dual<Interval>`: the base scalar, the value part.
    /// - [`crate::Sym`]`<T>`: `Sym<T::Value>`, node for node: the symbols
    ///   are an identity over the document's unknowns, which a decision
    ///   reads, not a tangent. Over `f64` or `Interval` that is the
    ///   scalar itself.
    ///
    /// It carries no tangent of its own, so a second read is the
    /// identity. `T: Real<Value = T>` (or `Decide<Value = T>`) is the
    /// bound "a scalar that carries no tangent", which no dual meets.
    type Value: crate::Decide<Value = Self::Value>;

    /// The read down to [`Real::Value`]: a dual's value part, bit for
    /// bit; the `Sym` over its numeric channel's read, its node kept;
    /// the scalar itself elsewhere.
    fn without_tangent(self) -> Self::Value;

    /// **`x` without its tangent**: [`Real::without_tangent`] of every
    /// scalar it holds, through `F`'s map. Borrowed where the scalar
    /// carries no tangent; a copy at a dual, and at a `Sym` over one.
    /// Plumbing for a geometry type's `value_view()`, which is what a
    /// caller names.
    #[doc(hidden)]
    fn view<F: crate::value_view::OverScalar>(
        x: &F::At<Self>,
    ) -> crate::value_view::ValueView<'_, F::At<Self::Value>>;

    /// Embeds an `f64` exactly (a point interval, a constant dual number).
    ///
    /// **For a constant only**: a literal, a count, a tolerance, data
    /// that is a constant of the model. At [`crate::Sym`] it is an exact
    /// rational, so a value the kernel COMPUTED and re-entered here is a
    /// different function of the parameters than the one it came from,
    /// and a symbolic `Zero` over it is a theorem about the wrong
    /// function (ERROR-DESIGN E12). A computed value stays in `Self`, or
    /// enters through [`Real::from_computed`].
    fn from_f64(x: f64) -> Self;

    /// Embeds an `f64` that a NUMERICAL routine computed: the value is
    /// [`Real::from_f64`]'s, and at [`crate::Sym`] each call is its own
    /// unknown ([`crate::Sym::opaque`]), so no identity is proved over
    /// it. Declared, never defaulted: a scalar that defaulted it to
    /// `from_f64` would re-enter every computed value as a constant.
    ///
    /// Which code may call it is DL5.2 (the crate README, "Reading a
    /// bracket where nothing certifies"): the solved-value door
    /// ([`crate::solved::decide_admit`]), a scalar's own impl, and a
    /// place-taking evaluator's private step. The calls the rule does not
    /// yet allow are owed, each listed with the work item that retires it
    /// (`scripts/gates/from-computed-sites.sh`).
    fn from_computed(x: f64) -> Self;

    /// **What this scalar reads as in a sentence**: its value, its
    /// enclosure, or nothing ([`crate::told::Reading`]). The one
    /// projection a refusal payload takes of a scalar
    /// ([`crate::Told`]); the reading has no accessor, so nothing
    /// decides on it. Declared, never defaulted: a new scalar says what
    /// a person reads of it.
    fn told_reading(&self) -> crate::told::Reading;

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
    /// discrimination**, not for deciding geometry. Predicates on real
    /// margins keep going through `Decide`, whose poison arm carries
    /// the diagnostic; this method never replaces one.
    fn is_poison(self) -> bool;

    /// **Is every value this scalar carries a finite number?** At a
    /// point scalar, whether the value is finite (`x − x` is not the
    /// poison, [`is_finite_length`]); at the interval scalar, whether
    /// both endpoints are finite and the enclosure is not poison — an
    /// exact fact about the representation, as [`Self::is_poison`] is,
    /// not a margin read against a band. The value channel answers at
    /// the dual and symbolic scalars.
    ///
    /// It is [`is_finite_length`] with the enclosure's case decided
    /// rather than waved through. That predicate keeps waving it
    /// through on purpose (an overflowed norm still ENCLOSES the true
    /// length); this one is for a value that must itself be finite to
    /// be what its type claims — a rigid motion's angle and
    /// translation (`linalg::RigidMotion`'s mints and `f64` door), and
    /// a length every rung of a door levers by (`topo`'s
    /// `chart_region::edge_lengths`), where an enclosure reaching `∞`
    /// would divide a finite height to zero at its lower end.
    ///
    /// It is also how a generic body asks which binade a value is in
    /// without a comparison surface: `x · 2ᵏ` is unbounded exactly when
    /// `|x| ≥ 2¹⁰²⁴⁻ᵏ`, so `normalizing_scale` (`linalg/vec.rs`) reads
    /// its cuts as overflows of the witness and of its reciprocal. That
    /// use is sound only where every answer selects the same real
    /// function — a choice of spelling, never of value.
    ///
    /// Required, with no default: `is_finite_length` is the waving
    /// answer at an enclosure, so a scalar that inherited it would
    /// pass an unbounded enclosure silently. Each scalar states its
    /// own reading.
    fn is_bounded(self) -> bool;

    /// **An enclosure of the rotation the stated frame `m` names**, where
    /// this scalar's value is a plain enclosure and nothing else: the
    /// columns of the polar factor `R` of `m` (the rotation nearest it),
    /// each entry enclosed. `m` is a frame stated at a point scalar
    /// (columns `u`, `v`, `w`, read here), which claims the exact
    /// rotation it is the rounding of; this is the lift of that claim
    /// into an enclosure ([`crate::RigidMotion::map_scalar`] across
    /// kinds). A scalar with no such enclosure answers `None` (the
    /// default): a point scalar has none to give, and a scalar that also
    /// carries derivatives or symbolic forms holds the rotation in those
    /// channels too, which an enclosure of its value does not reach.
    /// Decides nothing; a frame whose rotation it cannot enclose is
    /// answered poisoned.
    #[must_use]
    fn enclose_rotation(m: [[Self; 3]; 3]) -> Option<[[Self; 3]; 3]> {
        let _ = m;
        None
    }

    /// **An enclosure of the unit direction a stated `v` names**,
    /// [`Real::enclose_rotation`]'s one-column case: the direction
    /// `v/|v|`, each component enclosed, where this scalar's value is a
    /// plain enclosure; `None` (the default) where it is not.
    #[must_use]
    fn enclose_unit(v: [Self; 3]) -> Option<[Self; 3]> {
        let _ = v;
        None
    }

    /// **`(q − o)·n`, posed so its digits are the result's own**: the
    /// offset of `q` from `o` along `n`, which cancels whenever `q` and
    /// `o` lie far apart across `n` (a plane's stored origin far along
    /// the plane from the point read against it). At a value scalar each
    /// difference and product is error-free and their sum compensated
    /// (Ogita, Rump and Oishi's `Dot2`), so the value is the exact one's
    /// rounding up to a term of order `u²·Σ|nᵢ(qᵢ − oᵢ)|`, whatever the
    /// cancellation; at an enclosure stated at points it is the exact sum
    /// enclosed. A scalar that carries `T`'s value channel computes it
    /// through `T`'s own. There is no default: the plain formula
    /// ([`plain_difference_dot`]) is what this posing exists to replace,
    /// so a scalar states its reading.
    #[must_use]
    fn difference_dot(q: [Self; 3], o: [Self; 3], n: [Self; 3]) -> Self;

    /// **The belly `√(A² + B²) − |D|` of a first harmonic read across a
    /// plane, posed so its digits are the belly's own**: the height
    /// `D + A cos θ + B sin θ` of the stored curve `curve`
    /// ([`StoredHarmonic`]) above the plane through `o` with unit normal
    /// `n` ([`crate::exact::harmonic_belly`] names `A`, `B` and `D`). Its
    /// sign is whether the curve reaches the plane. Where the curve is
    /// far larger than its belly (an arc of a far-centred conic grazing
    /// the plane), `√(A² + B²)` and `|D|` are each of the curve's size and
    /// their difference cancels to the belly: read as that difference,
    /// it would be off by `u` of the curve's size.
    ///
    /// **At a value scalar** it is read as
    /// `(A² + B² − D²)/(√(A² + B²) + |D|)`, the numerator an exact
    /// expansion of the stored terms rounded once, and `A`, `B` and `D`
    /// each its exact value rounded once. A double-double reading of the
    /// same terms answers first where its own error bound shows it
    /// already holds that ([`crate::exact::harmonic_belly_near`]); one
    /// that cancels is read from the expansions. The numerator is within `2u` of
    /// itself ([`crate::exact::Expansion::estimate`]); `√(A² + B²)` is
    /// within `3u` (each square's `2u` halved by the root, and the
    /// root's `u`); the denominator sums two non-negative terms, `4u`;
    /// the quotient's own rounding adds `u`. So the belly is **within
    /// `7u` of itself, relative to the belly and not to the curve**, to
    /// first order; the row pins `8u`, judged exactly. Where the inputs'
    /// digits do not fit an exact expansion (an input whose lowest bit
    /// is so fine that `D²`'s deepest monomial, `(s₀·b·w·u·n)²`, falls
    /// below the least subnormal, or an overflow), it is poison: the
    /// belly cannot be read to its digits there, and a decision on it
    /// escalates rather than answering on a plain reading.
    ///
    /// **At an enclosure** stated at points the same expansions are
    /// enclosed, each step widened outward by an ulp or two (the row pins
    /// the width at `32u`); anywhere else, the plain enclosure, which is
    /// sound but carries the curve's size. A frame lifted from `f64` the
    /// production way (`Curve3::map_scalar(Interval::from_f64)`) states
    /// its axis and seam as enclosures of the polar factor, not as
    /// points, so it reads the plain enclosure, about `10¹⁶u` of the
    /// belly wide at a far scale, and a decision on it escalates
    /// (`work/germ/a-lifted-frames-interval-belly-reads-the-plain-enclosure.md`).
    /// A scalar that carries `T`'s value channel computes it through
    /// `T`'s own. There is no default ([`plain_harmonic_belly`] is the
    /// fallback a scalar names), as for [`Real::difference_dot`].
    #[must_use]
    fn harmonic_belly(curve: StoredHarmonic<Self>, o: [Self; 3], n: [Self; 3]) -> Self;

    /// **The discriminant `Δ` and the depth `D = 64s − 16p²` of the
    /// depressed monic quartic `y⁴ + p y² + q̂ y + s`, posed so their
    /// digits are their own**: the two signs its real-root count is read
    /// off. Near a repeated root `Δ` is a sum of terms orders larger than
    /// it (two roots `δ` apart put `Δ ≈ δ²·L¹⁰` beside terms of `L¹²` and
    /// more, `L` the roots' spread), and `D` cancels where `16s ≈ 4p²`,
    /// so the plain formulas keep only their terms' rounding there.
    ///
    /// **At a value scalar** each is its exact expansion over the stored
    /// coefficients ([`crate::exact::quartic_sign_algebra`]) rounded once,
    /// within `2u` of itself whatever the cancellation; poison where the
    /// coefficients' digits do not fit an exact expansion (a monomial's
    /// lowest bit below the least subnormal, or an overflow), so a
    /// decision on it escalates rather than answering on a plain reading.
    /// **At an enclosure** stated at points, the exact expansions
    /// enclosed; anywhere else the plain enclosure, which is sound. A
    /// scalar that carries `T`'s value channel computes it through `T`'s
    /// own. There is no default ([`plain_quartic_sign_algebra`] is the
    /// fallback a scalar names), as for [`Real::difference_dot`].
    #[must_use]
    fn quartic_sign_algebra(p: Self, q_hat: Self, s: Self) -> [Self; 2];

    /// **The `f64` this value states**, where it is a plain enclosure
    /// that is a certified point (its two ends one `f64`); `None` (the
    /// default) elsewhere: a point scalar's value is a rounding, not a
    /// statement, and a scalar that also carries derivatives or forms
    /// carries more than its value. A frame stated by such points is a
    /// choice, minted at `f64` and read into the lane by its enclosure
    /// ([`crate::RigidMotion::squared`]).
    #[must_use]
    fn stated_point(self) -> Option<f64> {
        None
    }

    /// **The meet of two readings of one real**, where `first` and the
    /// value `second` builds enclose the same real by independent
    /// spellings: at a scalar whose value is a plain enclosure, their
    /// intersection (the tighter enclosure, sound because whatever lies
    /// outside either is not the real); elsewhere `first` alone (the
    /// default), and `second` is never built, so a point or symbolic
    /// scalar reads exactly the one spelling and records no other. A
    /// caller reaches for it where each spelling widens in its own regime
    /// and neither bounds the other (`turn_arms`, geom-brep). Decides
    /// nothing.
    ///
    /// **Only for two readings of one real**, each a sound enclosure of
    /// it by its own argument: never a general narrowing of an enclosure
    /// by a window or a guess, which is `Certification::clamped_to`'s
    /// and `Certification::meet`'s business. Two such readings cannot be
    /// disjoint, so a disjoint meet is a kernel defect: it fails a debug
    /// assertion and answers poison.
    #[must_use]
    fn meet_readings(first: Self, second: impl FnOnce() -> Self) -> Self {
        let _ = second;
        first
    }

    /// **The registered-identity door's hook** (M10-9; ERROR-DESIGN
    /// E12's "kept in reserve — discharge by provenance"): a
    /// constructor generic over `T: Real` states that the two values
    /// it holds are ONE real, because the construction it just
    /// performed guarantees it — a swept arc's `‖q − c‖` and its
    /// radius, say. The scalar decides what, if anything, that is worth
    /// to it.
    ///
    /// **The default records nothing and claims nothing**
    /// ([`crate::sym::SymRegistration::Unwitnessed`]), which is what
    /// every scalar that tracks no expressions wants; the value channel
    /// is untouched at every scalar, including the one that does. Two
    /// jobs are behind the one method and the split is per scalar:
    ///
    /// - a scalar whose value channel can WITNESS the claim answers
    ///   [`crate::sym::SymRegistration::Witnessed`] or, when the two
    ///   values are not the same real, one of the two REFUSAL arms,
    ///   chosen by the KIND of witness: `f64` and [`crate::Probe`]
    ///   compare at a point with the run's ε as slack, taken as `tol`
    ///   ([`Real::register_equal`]'s `f64` impl argues the spelling),
    ///   and answer [`crate::sym::SymRegistration::Disputed`];
    ///   [`crate::Interval`] asks whether the two certified enclosures
    ///   MEET — an exact test — and answers
    ///   [`crate::sym::SymRegistration::Contradicted`]. Neither records
    ///   anything: there is no expression at a bare scalar to record it
    ///   about;
    /// - [`crate::Sym`] asks its own lane scalar that question first
    ///   and, on a witness, RECORDS the identity in the installed
    ///   session, where the symbolic tier's early normal form consults
    ///   it (`crate::sym`'s module docs).
    ///
    /// **WHAT THE WITNESS IS WORTH, exactly** — the contract this
    /// method's first cut overstated, corrected by two reviews taken by
    /// execution. A registration is an AXIOM; its soundness rests on
    /// the REGISTRANT'S PROOF and on nothing here. The witness refuses
    /// only a lie visible AT THE POINT (`f64`, `Probe`, and there only
    /// as `Disputed`, which may also be the arithmetic giving up) or one
    /// whose two certified enclosures are DISJOINT over the box
    /// (`Interval`, where the refusal is a proof) — and "the enclosures
    /// meet" is satisfied by every
    /// coincidence, so `x² ≡ x` over `[0.9, 1.1]` is recorded, and a
    /// registration false by a geometric amount is recorded as soon as
    /// the box is wide enough for the two enclosures to overlap. **The
    /// door cannot tell an identity from a coincidence.** That is why
    /// the spec requires each registrant to carry a theorem in its doc
    /// comment (claim 9), and why the witness is loosest exactly where
    /// the numeric-first shield is weakest — a residual whose two sides
    /// are dependency-widened straddles zero for the same reason its
    /// two enclosures meet. The rows that establish this are
    /// `geom-core/tests/m10_9_r1_sym_probes.rs`
    /// (`r1_a_coincidence_at_one_point_of_the_box_registers_and_decides_zero`
    /// and `r1_a_geometric_lie_the_f64_witness_refuses_is_recorded_at_interval`)
    /// and `geom-core/tests/m10_9_r2_sym_probes.rs`
    /// (`r2_the_interval_witness_lets_a_geometric_lie_through_over_a_wide_box`).
    ///
    /// **`tol` is the run's ε, and it ARRIVES.** The inexact witnesses
    /// (`f64`, [`crate::Probe`]) compare at a slack, and a slack that
    /// does not move with the run's ε is loosest exactly where the
    /// numeric-first shield is tightest. The run's ε is an entry-point
    /// commitment, so it is taken as a parameter and passed down from
    /// whichever registrant holds one — kernel library code may not
    /// mint a tolerance witness (`scripts/gates/witness-not-ambient.sh`).
    /// A scalar whose witness is EXACT ([`crate::Interval`]'s meet of
    /// two certified enclosures) ignores it, and says so at its impl.
    ///
    /// **WHERE IT MAY BE CALLED.** This method hands every generic
    /// `T: Real` body a value COMPARISON — the capability
    /// evaluation-code discipline exists to keep out of that position,
    /// and one that adds no bound for `no-extra-real-bounds` to see. It
    /// is therefore allowlisted by SITE:
    /// `scripts/gates/register-equal-allowlist.sh` names the
    /// constructors that may call it, and a new call site fails that
    /// gate until it is ratified there with its theorem. It is not a
    /// general-purpose equality and must never stand in for one.
    ///
    /// `#[must_use]`: a refusal a caller drops is a lie nobody sees.
    /// The registrant handles the typed answer; a refusal is also
    /// counted in the session's receipt
    /// ([`crate::SymCounts::registrations_refused`]).
    #[must_use]
    fn register_equal(self, _other: Self, _tol: Tol) -> SymRegistration {
        SymRegistration::Unwitnessed
    }

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

    /// **`self · k` for `k` an exact power of two** — the scale a
    /// subdivision's `2⁻ʳ` applies to a sum it formed unscaled. The
    /// value is the product's, `self * Self::from_f64(k)`, and that is
    /// the default: exact at `f64` wherever the result is normal, and
    /// the same expression at the symbolic and dual scalars.
    ///
    /// At the interval scalar the product by a point power of two is
    /// itself the exponent shift: `[lo·k, hi·k]`, unpadded wherever both
    /// results are strictly above the smallest normal (a product just
    /// under it rounds up onto it), and the general product everywhere
    /// else — a result in the subnormal range, an overflow, a poisoned
    /// operand, or a `k` that is not a normal power of two
    /// (`interval_transcendentals::DInterval::scale_pow2`). A `k` that
    /// is not one is never wrong, only not exact.
    fn scale_pow2(self, k: f64) -> Self {
        self * Self::from_f64(k)
    }

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

    /// **`self − sin(self)`, conditioned at every `self`**: twice the area
    /// a unit circle's arc of that signed span cuts off its chord. The
    /// difference as written cancels to `self³/6` for a small `self` and
    /// keeps `self`'s rounding, so a scalar reads it there off the
    /// alternating series `x³/3! − x⁵/5! + …`, whose terms only shrink
    /// below [`LESS_SIN_SERIES_BELOW`], and past that as the difference,
    /// which loses nothing there.
    ///
    /// Declared, never defaulted: each scalar keeps its own contract
    /// (an enclosure bounds the series' dropped tail; a recording scalar
    /// records the one function, re-valued through this door).
    fn less_sin(self) -> Self;

    /// **The sine of a twelfth of a turn**, exactly `1/2`. Defaults to
    /// the literal. A scalar that records structure keeps the sine of
    /// the angle instead, valued `1/2`, so what reads the structure sees
    /// the angle and not only its value.
    fn sin_of_a_twelfth() -> Self {
        Self::from_f64(0.5)
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

    /// The value with `self`'s magnitude and `sign`'s sign — sign
    /// transfer where the sign is a genuinely signed quantity. It is
    /// NOT the way to choose between two candidates: the enclosure arm
    /// must hull at any zero-containing sign, so a choice keyed on it
    /// hulls wherever the quantity it reads can be zero
    /// ([`Real::select_le_zero`] is that door).
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

    /// **The exact unit sign of `self`**: `±1`, `self`'s sign carried
    /// onto one — [`Real::copysign`] of one, so every one of its
    /// per-instantiation semantics holds: an `f64` zero answers by its
    /// sign bit, an enclosure holding a zero answers the hull `[−1, 1]`,
    /// and poison answers poison. The one home of a `±1` read off a
    /// value's sign; a two-way CHOICE keyed on a sign is
    /// [`Real::select_le_zero`]'s.
    fn unit_sign(self) -> Self {
        Self::one().copysign(self)
    }

    /// **The value-level decision door**: `when_le` if `self ≤ 0`, else
    /// `when_gt` — a two-way selection keyed on the sign of a
    /// difference, with the TIE (`self == 0`) on the `when_le` side.
    ///
    /// This is how comparison-free evaluation code picks between two
    /// candidate computations. It is a *value* operation, not control
    /// flow — it returns `Self`, so it cannot drive a branch, and no
    /// topology-determining question passes through it (those go to a
    /// named predicate, per Q1). Its consumers include
    /// [`crate::Vec3::orthonormal_basis`], which chooses which world
    /// axis to cross the normal with, and `profile`'s
    /// `sugar::split_half_tan`, which chooses which half-angle
    /// quotient of a chord split does not cancel.
    ///
    /// **The tie is keyed on the VALUE zero, never on a zero's sign
    /// bit** — `-0.0 ≤ 0` and `+0.0 ≤ 0` take the same arm. That is
    /// what makes the tie decidable over an enclosure: a point
    /// enclosure `[0, 0]` names one real, the tie-break is the same for
    /// every point of it, so `Interval` DECIDES there. This is exactly
    /// what [`Real::copysign`] cannot do — its zero-containing arm must
    /// hull at a POINT zero, because an `f64` zero's sign bit is
    /// invisible in an enclosure and a one-sided choice would fail to
    /// contain an `f64` replay that saw `-0.0`. A construction that
    /// spells a choice as `copysign` on the difference is therefore a
    /// branch in disguise that hulls where a decision exists; this door
    /// is the one that does not.
    ///
    /// **Poison: `self` propagates; a candidate propagates only where
    /// it is read.** A poisoned decision value poisons the result — the
    /// choice is unknown, and picking an arm would launder it. A
    /// poisoned CANDIDATE poisons the result only when that candidate
    /// is selected, or when both are (the straddle arm below). The
    /// asymmetry is deliberate and load-bearing: the door exists to
    /// pick the well-conditioned member of a candidate set whose other
    /// members are degenerate at exactly the input being chosen away
    /// from — `orthonormal_basis` at `n = ±e_z` evaluates
    /// `normalize(e_z × n)`, which is `normalize(0)`, all poison, and
    /// the axis order is what guarantees it is never the one read.
    ///
    /// Per-instantiation semantics:
    ///
    /// - `f64`: the IEEE comparison `self <= 0.0` behind the poison
    ///   guard — a total order, exact and bit-deterministic (D9).
    /// - `Interval`: DECIDED when the comparison holds for every point
    ///   of the enclosure — `hi ≤ 0` (which includes the point tie
    ///   `[0, 0]`) selects `when_le`, `lo > 0` selects `when_gt` — with
    ///   the decoration capped by the deciding enclosure's (the choice
    ///   is only as trustworthy as the enclosure that made it).
    ///   UNDECIDED (`lo ≤ 0 < hi`, necessarily of positive width) is
    ///   the **hull of both candidates**, decoration capped at `Def`:
    ///   the question is real at every point of the box and the honest
    ///   answer is both branches, never `Trv`, never empty
    ///   (`docs/DUAL-DESIGN.md` DL6). The hull is what buys the
    ///   enclosure property: an `f64` value inside the decision's
    ///   enclosure lands on one of the two arms, and both are enclosed.
    /// - `Dual<T>`: value channel is `T`'s door verbatim (the
    ///   value-channel bit-identity contract); the tangent follows the
    ///   chosen candidate's, ties keeping the tie-break's choice (the
    ///   `min`/`max` kink convention), and hulls both tangents wherever
    ///   the value channel hulls.
    fn select_le_zero(self, when_le: Self, when_gt: Self) -> Self;

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
    /// **What a reader owes at the jump: nothing.** An exact multiple of
    /// `period` reduces to `0`, and an input a hair below one to a hair
    /// under `period` (above), so which end of the jump a coincidence
    /// lands on is rounding, not information. A caller reads the result
    /// only where its answer is invariant under that branch, and a whole
    /// turn comes from structure it holds (a closed carrier, a self-loop,
    /// a decided full span), never from this value.
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

/// `2^e`, exactly, built from its bits: subnormal for `e < -1022`, `0`
/// below `-1074`, `∞` above `1023`. The one spelling of an exact power
/// of two — `2f64.powi(e)` rounds past the exponent range (its own
/// docs' `powi(2.0, -1074) = 0`) — and the argument every
/// [`Real::scale_pow2`] call takes.
#[must_use]
#[allow(clippy::cast_sign_loss)]
pub const fn pow2(e: i32) -> f64 {
    if e > 1023 {
        f64::INFINITY
    } else if e >= -1022 {
        f64::from_bits(((e + 1023) as u64) << 52)
    } else if e >= -1074 {
        f64::from_bits(1u64 << (e + 1074))
    } else {
        0.0
    }
}

/// **Is `x` a finite number at this scalar?** — asked through the
/// value channel every [`Real`] has, with no bracket read and no
/// threshold invented.
///
/// A finite value less itself is exactly zero; `∞ − ∞` and `NaN − NaN`
/// are the scalar's poison ([`Real::is_poison`]). So the
/// self-difference IS the question, which is why the equal-operands
/// lint is allowed here and nowhere near it. An enclosure answers YES
/// however wide it is (a finite interval's self-difference is a finite
/// interval around zero, and an enclosure whose upper end overflowed
/// still contains its truth) — the honest scope: this catches the
/// point scalars, which is where an infinite length turns into a
/// definite wrong answer.
///
/// **It is a statement about a SCALAR, and it sits at the bottom
/// because every crate that has directions has to be able to ask it.**
/// The arithmetic whose failure mode it names is a norm's: components
/// beyond ~1e154 overflow [`Vec3::norm`](crate::Vec3::norm) to ∞, and
/// the ∞ length reads maximally DEFINITE to
/// [`Decide`](crate::Decide). A home in
/// a geometry crate above this one puts the single spelling out of
/// reach of the crates below it — `profile` depends on this crate
/// alone — and a rule with two spellings is two rules.
///
/// **Every decide-then-normalize direction door in the workspace asks
/// this first**, and the claim is the enumeration, not a generality:
///
/// - [`decide_unit_direction`](crate::decide_unit_direction) — the one
///   [`Margin::norm3`](crate::Margin::norm3) spelling, behind the
///   datum door, the evaluation layer's `unit()`, and every one of
///   [`linalg::frame`](crate::linalg::frame)'s normalizing sites (the
///   roll offset and the ladder's rungs reach it through
///   [`RigidMotion::from_aim`](crate::RigidMotion::from_aim)), and
///   [`UnitVec3::new`](crate::UnitVec3::new), which `sweep` reaches
///   for the revolve's sketch axis (`revolve::axis::AxisFrame::build`)
///   and for a path sweep's station tangents and chords
///   (`sweep_path_tangent`, `sweep_path_chord`);
/// - `topo`'s `sector_shape`, which asks it of each bounding chord
///   separately because its arm is their `min` and [`Real::min`]
///   propagates `NaN` but not infinity, so a `min` hides an
///   OVERFLOWED chord behind a finite one;
/// - `profile`'s two 2-D director doors (`unit_from_components`,
///   `arc_fillet::carrier_tangent`).
///
/// Each refuses with its own typed arm, naming the recourse its own
/// caller can reach: the question is one question, but what a caller
/// can DO about an overflowed direction differs by door — a spelled
/// component pair is divided through for free, a derived displacement
/// is not.
///
/// **This roster is hand-kept and nothing enforces it.** It is a
/// five-door cross-crate claim living in a leaf crate that cannot see
/// four of them, so it is exactly as current as the last person to
/// edit it. Treat it as a reading aid, not as a census; the census
/// that is checked is `docs/K-REPORT.md`'s, and it is about decided
/// names rather than about this question.
///
/// **The scalar's poison**, built through the totality policy rather
/// than spelled per type: [`Real::sqrt`] of a negative is poison on
/// every implementor (NaN at `f64`, the empty enclosure at `Interval`,
/// a poisoned value channel at a wrapper). The one spelling a generic
/// reader answers "no value" with.
#[must_use]
pub fn poison<T: Real>() -> T {
    (T::zero() - T::one()).sqrt()
}

/// **The declined half of the family: normalize-without-deciding.**
/// A site that normalizes a vector whose length it never decides at
/// all is a different shape — there is no sign question in front of
/// it to put this one before — and it needs none while the length is
/// read nowhere else: [`Vec3::normalize`](crate::Vec3::normalize)
/// prescales, so a finite vector yields its direction at every
/// magnitude and an infinite one yields poison. `geom-brep`'s
/// `enters::enters_material` is that case, and its tests pin it. The
/// shape is a defect where the un-asked length is ALSO read — as a
/// lever, a denominator, a span — or where the normalization is
/// spelled `v / v.norm()`, which the prescale never sees. Known
/// instances, a floor rather than a total: those listed in
/// `work/scalar/two-step-normalize-and-unasked-lengths`.
/// `profile`'s `path::chord_director` is one by design: a leg's length
/// is decided by the gates that read it as a lever and as a segment, so
/// the chord door decides none and refuses nothing. It asks this of the
/// chord's largest component only to choose how to divide (a component
/// past the range keeps `atan2`'s heading), and otherwise reads
/// `Vec2::normalize`, whose prescale no finite chord overflows or
/// flushes the squares of.
pub fn is_finite_length<T: Real>(x: T) -> bool {
    !width(x).is_poison()
}

/// **A scalar's own width about zero**, `x − x`: zero at a point scalar
/// (any finite value), the enclosure `[−w, w]` at `Interval`, `w` its
/// width, and poison where `x` is not finite. It names the one reading
/// that subtracting a value from itself is: no arithmetic on a value
/// gives its own spread otherwise, and a site that spells `x − x` reads
/// as a mistake. Its callers are what `grep -rn "geom_core::width\|real::width"
/// crates` finds: [`is_finite_length`], and `geom_brep::curve_pair`'s
/// touch spread, a closed form's angle read as its enclosure.
#[allow(clippy::eq_op)]
pub fn width<T: Real>(x: T) -> T {
    x - x
}

/// **Did this length UNDERFLOW to zero?** — the other end of
/// [`is_finite_length`]'s arithmetic, asked through the same value
/// channel, with no bracket read and no threshold invented.
///
/// `len` is a vector's computed norm and `witness` the largest
/// absolute value among the components it was computed from. **That
/// pairing is the contract**, and it is what makes the two divisions
/// below a decision: the norm is never smaller than the largest
/// component and never larger than `√n` times it, so `len / witness`
/// and `witness / len` are both bounded ratios — unless `len` came
/// out exactly zero. Then `witness / len` is `±∞` if the vector has
/// any nonzero component, and the scalar's poison (`0/0`) if it does
/// not, which is exactly the two cases this question separates:
///
/// - **the length underflowed** — components below ~1e-162 at `f64`
///   square to zero, so `norm_squared` and the norm are exactly zero
///   for a vector that has a perfectly good direction. Dividing by
///   that norm blows the direction up to `±∞`, which is why
///   [`Vec3::normalize`](crate::Vec3::normalize) prescales first;
/// - **the vector is the zero vector**, which really does name no
///   direction.
///
/// No tolerance separates them and no tolerance recovers the first:
/// the squared norm is zero at every ε, so a door that decides the
/// length's sign answers `Zero` either way. The only recourse for an
/// underflowed direction is the overflow end's — scale the geometry
/// into the session's range — which is why the two are different
/// facts about the input and get different refusals.
///
/// **Ask it AFTER [`is_finite_length`], never instead of it.** A
/// poisoned or overflowed length makes both ratios non-finite for
/// reasons that have nothing to do with underflow, so this question
/// is only meaningful once the length is known to be a finite number.
///
/// **The witness is derived, never invented**:
/// [`Vec2::norm_witness`](crate::Vec2::norm_witness) and
/// [`Vec3::norm_witness`](crate::Vec3::norm_witness) are the two
/// doors that spell it, and a caller that reaches past them is making
/// a claim this signature cannot check.
///
/// **Which doors ask it**: a door that refuses a vector whose length
/// leaves the format asks it through
/// [`Vec3::checked_direction_and_length`](crate::Vec3::checked_direction_and_length),
/// [`Vec3::checked_direction`](crate::Vec3::checked_direction), their
/// length halves or their 2-D twins
/// ([`LengthOutOfFormat`](crate::LengthOutOfFormat)), so the doors are
/// those readers' callers rather than a roster here. A door refusing
/// several vectors asks per vector, never of a length combined from
/// them (a `min` has no witness of its own, and an underflowed term
/// always wins it).
///
/// `profile`'s OTHER 2-D director door, `unit_from_components`, asks
/// [`is_finite_length`] and deliberately not this: the pair it refuses
/// is SPELLED by the caller rather than derived, so its existing
/// `ZeroDirection` sentence is true of an underflowed pair and its
/// recourse — scale the components up, which costs nothing because only
/// their ratio is read — is already the one that works. A new arm there
/// would separate two inputs that want the same answer.
///
/// **It bites at the point scalars, exactly as the finiteness
/// question does.** At an interval scalar a norm whose lower end
/// underflowed still ENCLOSES the true length — `[0, 3.1e-162]` for a
/// `1e-180` component, not `[0, 0]` — so `witness / len` is an
/// unbounded enclosure rather than poison, the answer is `false`, and
/// the enclosure lane goes on deciding against the band as before.
pub fn is_underflowed_length<T: Real>(len: T, witness: T) -> bool {
    !is_zero_length(len, witness) && !is_finite_length(witness / len)
}

/// **Is this the ZERO vector?** — the other case
/// [`is_underflowed_length`] separates out, and the one spelling of
/// that half of its arithmetic (the underflow door is written over this
/// one), asked with the same pairing through the value channel, with no bracket
/// read and no threshold.
///
/// `len` and `witness` are exactly [`is_underflowed_length`]'s pair,
/// and so is the precondition: **ask it AFTER [`is_finite_length`] of
/// `len`, never instead of it.** Of a finite length, `len / witness` is
/// the scalar's poison (`0/0`) exactly when both are zero — the vector
/// has no nonzero component — and a finite ratio otherwise, `0`
/// included (an underflowed length, whose direction is fine). An
/// OVERFLOWED length breaks that: `∞ / witness` is not finite for a
/// vector with perfectly finite components, which is why the
/// precondition is not optional — a caller that skips it names a large
/// direction the zero vector.
///
/// It bites at the point scalars and at the interval scalar alike for
/// an exactly-zero vector: `[0, 0] / [0, 0]` is the empty interval,
/// which is poison.
///
/// **Which doors ask it** — hand-kept, like its siblings' rosters:
///
/// - `topo`'s tier-3 check 1, of every stored direction of an analytic
///   surface or edge carrier — a plane's `normal`, the `axis` and
///   `u_ref` of the axisymmetric kinds, a line's `dir` — where a zero
///   one is a datum that describes no locus. It does not DECIDE the
///   length (that is [`decide_unit_direction`](crate::decide_unit_direction)'s
///   job, metered and band-relative): a datum is asked whether it is
///   the zero vector, not whether it is short.
pub fn is_zero_length<T: Real>(len: T, witness: T) -> bool {
    !is_finite_length(len / witness)
}

/// **A measured norm less a locus's radius**, `norm − radius`, spelled
/// `(norm_sq − radius²) / (norm + radius)`: the same real wherever
/// `norm + radius ≠ 0`, its one precondition. `norm_sq` is the measured
/// vector's own dot product and `norm` its root, so the numerator is a
/// polynomial in the vector's and the radius's data: for a vector built
/// at the radius's length it is the zero form with nothing known of the
/// radius's sign, where `√(radius²) − radius` is not. The sign is the
/// difference's; a caller that wants the distance takes the `abs` of
/// the whole quotient.
///
/// `radius` is the locus's positive radius. A caller that holds a
/// signed radius passes its magnitude: the numerator folds either way
/// (`|r|² = r²`), and with the signed value `norm + radius` is zero at
/// the locus's own points.
///
/// A caller holding the vector reads [`crate::Vec3::radius_gap`] or
/// [`crate::Vec2::radius_gap`], which form `norm_sq` and `norm` from it.
///
/// Per scalar:
/// - `f64`: `norm − radius` to rounding wherever `norm + radius` is not
///   zero in rounding; there it is a quotient of two roundings and no
///   value is claimed.
/// - `Interval`: an enclosure of `norm − radius` wherever the
///   denominator's enclosure excludes zero; where it contains zero the
///   quotient is invalid, and a decision on it refuses.
/// - `Dual`: the quotient's tangent is the difference's.
/// - `Sym<T>`: the value channel is `T`'s sentence; a zero form over an
///   invalid value refuses.
pub fn radius_gap<T: Real>(norm_sq: T, norm: T, radius: T) -> T {
    (norm_sq - radius.powi(2)) / (norm + radius)
}

/// Whether `x` is EXACTLY zero, of either sign: at the point scalars
/// `x == 0.0` on the value, at the interval scalar only the degenerate
/// hull `[−0, 0]`. An enclosure that merely contains zero, and poison,
/// are not zero.
///
/// [`Real`] compares nothing, so the read is arithmetic: `x · 0` is
/// poison exactly when `x` is not finite, and for a finite `x`,
/// `x · (1/x)` is poison exactly when `x` is zero (0 · ∞; at `Interval`
/// the reciprocal of `[0, 0]` is empty). It is structure read as
/// structure, not a tolerance decision, so a caller that takes the
/// `false` branch for a nearly-zero value must be the one that refuses
/// or routes more.
///
/// Its callers are what `grep -rn is_exact_zero crates` finds; each
/// says at the call what an exact zero means there.
pub fn is_exact_zero<T: Real>(x: T) -> bool {
    !(x * T::zero()).is_poison() && (x * (T::one() / x)).is_poison()
}

/// `x` with a zero of either sign stated as `+0.0`; every other value,
/// NaN included, unchanged. It moves no value (`-0.0 == 0.0`) and fixes
/// the bit pattern, for a reader that would tell the two zeros apart: a
/// hash paired with IEEE equality, a bitwise comparison, a printer.
#[must_use]
pub fn fold_signed_zero(x: f64) -> f64 {
    if x == 0.0 { 0.0 } else { x }
}

/// Bracket extraction off a scalar — deliberately a separate trait, never
/// folded into [`Real`], and **not** the certification door.
///
/// Carrying a bracket is not permission to certify. Since the D1 ruling of
/// 2026-08-19 a [`Dual`](crate::Dual) implements this trait, and it still
/// does not implement [`CertifiedEnclosure`], which is the certification
/// door; the two questions have separate traits so that a caller has to
/// say which one it is asking. [`Real`] omits bracket extraction
/// altogether, so evaluation code cannot silently collapse an interval to
/// a number (see the [module docs](self)); this trait is the separate door
/// those docs promised.
///
/// # The scope rule (M0 L7; the crate README, DL5.3)
///
/// **A bound states which scalars the code means something at.** Write
/// that bound, and never a wider one because it compiles. The standard
/// this replaces asked for the weakest bound that works; Ev, on PR 7155:
/// it "should be updated to have more of a focus on semantic
/// correctness", and it was never "broaden it to anything that doesn't
/// error". Code that certifies says so with [`Certifies`], and no work
/// is done to support a scalar the code has no meaning for.
///
/// **Excluding a scalar is free.** [`Certifies`] grants no bracket read,
/// so writing it needs no entry and no ratification, alone or beside a
/// bound that reads no bracket. Ev: "the exclusion is free and only the
/// bounds reading needs the extra check".
///
/// **What needs an entry is a bracket read in code that also decides.**
/// A `Bounds` bound appears only in **certification and driver code** —
/// residual certification, the subdivision driver, rendering/telemetry —
/// never in an evaluation signature; and code that only reads brackets
/// writes its bracket door as the parameter's **sole** bound (`T:
/// Bounds`, or `T: CertifiedBounds` where the code has no meaning at a
/// dual; each is a subtrait, so [`Real`]'s operations come with it). A
/// bracket door BESIDE a deciding bound on one parameter (`T: Decide +
/// Bounds`, `T: Certifies + CertifiedBounds`; [`Certifies`] has `Decide`
/// as its supertrait) is the compound form
/// `scripts/gates/bounds-allowlist.sh` reads for, and it is written only
/// in a file with an entry in [`bounds_allowlist`]. The entry shows the
/// read is necessary and that it decides nothing.
///
/// **A NAMED compound bound is the same obligation as the literal
/// spelling.** A trait that gathers a decision door and a bracket door
/// under one name — `trait ArcCarrierScalar: Decide + Bounds`,
/// `trait EvalScalar: Decide + … + Bounds` — hands every `T: ThatName`
/// exactly the parameter this rule refuses, written so that no
/// `+ Bounds` appears at the use site. The rule reads through the name:
/// declaring one IS writing a compound bound and needs a ratification
/// of its own, at whichever home that ruling has — this ledger, or the
/// declaring file's own module docs, which is where
/// `ArcCarrierScalar`'s LIB-G2 LB3 lives — and every use of the name
/// carries the obligation the name gathers.
///
/// **Brackets never decide**, and that is checked before any necessity
/// argument is weighed. Every topology-determining branch stays a
/// [`Decide`](crate::predicate::Decide) call site — a trilean, with its
/// in-band arm — while a bracket may prune a candidate set, drive a
/// subdivision, or be reported. A parameter that needs a bracket in order
/// to decide something OUTSIDE the trilean is not a weak seam candidate,
/// it is the escape hatch itself, and no demonstration of necessity
/// redeems it: such a candidate is refused rather than weighed.
///
/// **The ratified exceptions have their own home**, [`bounds_allowlist`]:
/// one entry per seam, with the argument that earned it: the read is
/// necessary, and it decides nothing. It
/// lives beside this trait rather than inside this doc because it is the
/// part that grows — see that module's own header.
///
/// **Extension (SEAT-4, authorized under the M7-8 precedent;
/// retroactive Ev review per the self-merge convention):**
/// `verbs::run` — the verb vocabulary's dispatch site — joins
/// the compound allowlist as the narrowest possible extension of the
/// PR 12 edge-blend seam, on the M7-8 argument verbatim: it adds no
/// obligation because it DELEGATES to already-listed doors
/// (`sweep::blend::build`'s `fillet_edges`/`chamfer_edges`; since the
/// boolean's migration also `topo::boolean_op_with`, itself listed
/// under the M5 PR 8 BVH candidate-generation allowance; since the
/// sweeps' and the split's migrations also `sweep::extrude`,
/// `sweep::revolve` and `topo::split`, the last of which asks for no
/// [`Bounds`] at all), passing its operands and parameters through
/// unchanged, and therefore inherits their signatures rather than
/// widening the rule's reach.
///
/// **It clears the first thing an entry owes** — that its reads stay
/// on the prune/report side — vacuously and checkably: the file
/// contains no [`Bounds`] read at all. No `lo`, no `hi`, no
/// comparison; the bound is spelled once, as the supertraits of the
/// `verbs::DoorScalar` alias (and its blanket impl), which each run
/// door's `impl<T: DoorScalar>` header names, purely so the callee's
/// bound is satisfiable. Nothing there decides anything,
/// in or out of the trilean.
///
/// **That "no read at all" clearance is a REVIEW-TIME MEASUREMENT, not
/// a guarded invariant**, and the distinction is worth the sentence
/// because the entry's whole force rests on it. The gate script
/// (`scripts/gates/bounds-allowlist.sh`) checks that a compound
/// `Bounds` bound appears only in allowlisted FILES; it does not check
/// that an allowlisted file abstains from reading brackets. So a later
/// edit could add a `lo()`/`hi()` comparison to this file and no row
/// anywhere would redden — the abstention was verified by reading the
/// file at ratification and holds only as long as someone keeps
/// reading it. Every entry in this allowlist carries that same
/// exposure; this one states it rather than leaving a reader to assume
/// the gate is stronger than it is.
///
/// **On the second — there is no read here to show necessary.** The
/// header names [`Bounds`] only so that its callees' bounds are
/// satisfiable, so this entry rests on theirs (the delegation rule) and
/// argues nothing of its own about a bracket. Which scalars the door
/// admits is the scope rule's question, and its answer is that the
/// seat is construction code: a blend, a boolean or a sweep built at a
/// [`Dual`](crate::Dual) carries its tangents, so the door means
/// something there and does not write [`Certifies`]. (That a dual
/// build exists is evidence of that, not the argument:
/// `editor_core::eval::wire`'s blend lowering runs beneath
/// `evaluate<T>`, which the dual corpus instantiates.)
///
/// The seam did not widen here; it acquired a file. The refusing-lane
/// question is answered where it was already answered: the PR 12 entry
/// above, under the delegation rule, for the doors this one calls.
///
/// **The file carries ONE compound header, and the second seam
/// SEAT-9 allows for has nothing in it** — the allowance stands and
/// covers nothing. What it was written for was the shell door's own
/// header: `verbs::run`'s shell arm delegates to `topo::shell_open`,
/// which is `CertifiedPolicy + `[`CertifiedBounds`] on its finished
/// body's scalar (already
/// allowlisted, at `topo/src/shell.rs`, under the 2026-09-02
/// certified at-rest entry), and a bound that names the callee's
/// rights cannot be merged into the first header: the paragraph above
/// records that the first header's doors mean something at a dual,
/// which a certifying bound would deny. That right is now a VALUE the
/// caller passes — the scalar's `topo::AtRestPolicy::ShellDoor`, which
/// is `topo::ShellDoor` where the scalar certifies and a type with no
/// values at a dual — so the seat's shell arm takes the door as a
/// parameter and rides the first header like every other verb, with
/// the delegation rule covering it on the same terms: it passes its
/// operand, its thickness and its designation through unchanged,
/// reads no bracket, and decides nothing in or out of the trilean. A
/// door whose right cannot be carried in a value is what would put a
/// second header back here, and that is what this allowance is for.
///
/// The "no bracket read at all" clearance above still describes the
/// whole file, and it is still a review-time measurement rather than a
/// guarded invariant, for the reason that paragraph gives.
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
/// rounded), so certification code bounds *residual quantities* computed
/// at interval type and never asserts "f64 value ∈ enclosure" for
/// transcendental results (exact operations — `+`, `·`, `sqrt` — are
/// correctly rounded at `f64` and may be asserted contained).
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
///
/// # The two shapes that are not decisions (#990, ratified 2026-08-27)
///
/// A door outside the seams has no non-metered spelling for *request
/// validity*, and no `f64` out of a `T: Decide` for a refusal payload.
/// Neither needs a seam, because neither is a kernel decision:
///
/// 1. **Request validity dissolves at the signature.** Caller-intent
///    magnitudes enter doors as plain `f64` — or, where the constraint is
///    expressible in the type, as a validating newtype whose constructor
///    is the single refusal site — and are lifted into `T` only after
///    validation (`topo::shell`'s `thickness` is the pattern). No bracket
///    is read and no row meters the caller. A door that takes its request
///    in `T` is the thing to fix, not to allowlist.
/// 2. **Refusal payloads need nothing new**, since (1) leaves a refusing
///    door holding the caller's own number. What remains are DERIVED
///    quantities — margins, realized radii, values that exist only at `T` —
///    and those never leave a door as a number: an `f64` in an error
///    payload is a branchable channel, the same unmetered decision surface
///    at one remove. **No projection to a number exists, by this ruling**
///    — a `fn margin_of<T: Bounds>(…) -> f64` for everyone's payloads is
///    precisely what was asked for and refused. A derived quantity leaves
///    as text and only as text (the crate README, "A number told
///    to a person"): the one projection is [`Real::told_reading`], into
///    a told quantity ([`crate::Told`]), which renders and offers
///    nothing a comparison could read. The classify seam's own diagnostic has that
///    shape for a decided margin: `MarginDiag`, minted inside each
///    scalar's `Decide` impl on every outcome of a verdict the funnel
///    records under its name (definite and indeterminate alike), is the
///    one projection of a decided margin. A refusal payload may echo it,
///    for error reporting only: a recourse table or a door's message
///    chooses its words from it, and nothing decides on it. Its type
///    makes that structural, not a convention — it offers no value a
///    comparison could read in passing, so deciding on it takes an
///    obviously wrong, visibly named step that review and a gate can
///    see.
///
/// For genuine decisions nothing changes: the metered predicate layer is
/// the only spelling — it IS the "definite sign or indeterminate" trilean,
/// plus the two things a bare helper lacks (the NAME in the verdict log,
/// and escalation as the forced disposition of indeterminate). A
/// free-floating bounds-comparison helper would be the #701 evasion (a
/// bracket reader under a name the gate does not read) with better
/// manners, and stays out.
///
/// # The direction rule for terminal grants (#571, A′'s ruling)
///
/// A terminal grant from `Bounds` is legitimate only when the granted
/// claim lies in the bound's CONSERVATIVE direction: a box-disjointness
/// answer IS a sound disjointness certificate for the contents
/// (sufficient-not-necessary; touching-or-overlapping boxes with disjoint
/// contents refuse or escalate — the safe failure direction), whereas a
/// terminal grant of an EXISTENCE or overlap claim from boxes would be the
/// violation. The two #990 shapes never reach this rule: neither branches
/// on a bracket, so neither is a terminal grant at all.
pub trait Bounds: Real {
    /// The infimum of the bracket the scalar carries — the value itself
    /// where the bracket is a point, NaN where the value is poison.
    fn lo(self) -> f64;

    /// The supremum of the bracket the scalar carries — the value itself
    /// where the bracket is a point, NaN where the value is poison.
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

pub mod bounds_allowlist {
    //! The ratified compound-bound seams, one entry per ratification.
    //!
    //! **A module and not a section of [`Bounds`](super::Bounds)'s own doc, because this is
    //! the part that GROWS.** The rule, the contract and the two standing
    //! rulings above are finished text; this ledger gains an entry every time
    //! a seam is ratified, and while it sat inside the trait's doc block that
    //! growth pushed the rule further from the top on every landing — which
    //! is the shape the finding behind this move was about. Here it can grow
    //! without moving anything a reader is looking for, and it is still in
    //! `real.rs`: every pointer into it names an entry by its ruling's LABEL
    //! and DATE (`scripts/gates/bounds-allowlist.sh`'s header, `topo`'s
    //! `boolean/reduce.rs`, `chart_region.rs`, `separation.rs`), and a label
    //! survives a move that a line number would not.
    //!
    //! Nothing here is a separate rule from [`Bounds`](super::Bounds)'s: the scope rule, the
    //! *brackets never decide* clause and the necessity standard are stated
    //! there and are what this ledger's entries are ratified AGAINST.
    //!
    //! Every seam ratified to write a compound bracket bound, in the
    //! order they were ratified, each with the necessity argument that
    //! earned it. **New entries are appended here**, and nothing checks
    //! that — it is a convention kept by hand, like the entries
    //! themselves; what the module buys is that keeping it costs the
    //! rule above nothing.
    //!
    //! **What the rule is FOR**, in Ev's words: the gate exists "to avoid
    //! the dangerous pattern when not necessary, so if it is necessary it's
    //! fine". The rule is not a budget on how many seams may exist, and an
    //! extension is not earned by RESEMBLING one already listed. What a
    //! candidate owes is a demonstration that the bracket READ is
    //! **necessary** — what the code takes from the bracket and cannot
    //! have without it — and that the read decides nothing. That
    //! demonstration is the ratifiable artifact. An argument that the
    //! bound *suffices*, or that a tighter one breaks a caller, is not
    //! it: which scalars a function admits is the scope rule's question,
    //! answered by what the function means and before any entry is
    //! weighed. That a dropped bracket term stops the code compiling is
    //! not the demonstration either: it shows the bound is needed to
    //! name a callee, not that a read is. An entry that says what is
    //! read and what the read is for has made the argument; an entry
    //! that reads no bracket itself and carries the bound to call one
    //! that does rests on its callee's entry (the delegation rule,
    //! below) and says so; an entry that does neither is OWED its
    //! argument, not discharged, and
    //! `work/certread/ledger-entries-still-argued-as-the-weakest-bound-that-works.md`
    //! lists the ones found.
    //! `scripts/gates/bounds-allowlist.sh` holds the file list and points here
    //! for the reasons rather than restating them.
    //!
    //! **2026-07-29 (M5 PR 8) — the driver amendment.** Spatial-index /
    //! candidate-pruning DRIVER code — the C10 `bvh` crate and the certified
    //! box constructors beside their invariants — and the boolean-sweep +
    //! evaluation-service seams that feed it may write `T: Decide + Bounds`:
    //! that code is simultaneously decision code (`Decide`) and the
    //! subdivision driver (`Bounds`), so the sole-bound form is unsatisfiable
    //! there. `topo::separation` — LIB-PLACEDUNION's placement certificate —
    //! falls under this entry rather than a new one: a certified box
    //! constructor beside its invariants plus a query driver over the C10
    //! tree, deciding no topology.
    //!
    //! `topo::census` — the at-rest census's BVH pre-filter — falls under
    //! this entry on the same terms: `census::Trees` builds the C10 tree's
    //! item boxes through the certified box constructors (`face_box`,
    //! `edge_box`, a vertex point widened by the sweep pad) and
    //! `census::Candidates` drives the tree's queries, deciding nothing —
    //! the exact sweeps that follow decide. The read is necessary: the
    //! tree's boxes are `f64` brackets read through `Bounds::lo`/`hi`
    //! (`bvh::Aabb::from_points`), and without them there is no tree to
    //! query; the census is decision code on the same scalar, so the
    //! bracket door stands beside `Decide`. The read decides nothing: a
    //! box only prunes, and the sweeps decide through the funnel. The
    //! bound rides the three `validate` doors that reach the census
    //! (`validate_pseudomanifold`, `validate_pseudomanifold_certificate`,
    //! and the `via` they share) as a REACHABILITY ride only: those
    //! doors are validators, not driver code, and carry `T: Bounds`
    //! because the census cannot be reached except through them — the
    //! first time this amendment's bound rides onto a non-driver public
    //! door, said plainly so the next ride is argued rather than
    //! inherited.
    //!
    //! `topo::boolean::carrier_touch` — the crossing layer's reading of a
    //! root set its door could not settle, against the face — falls under
    //! this entry on the same terms. It bisects an edge's span, reading a
    //! piece's half-length bracket only to stop bisecting (the subdivision
    //! driver), and it prunes the face's boundary edges by their certified
    //! `edge_box` against a touch's ball before a distance bound is decided
    //! (the box constructors' pruning side). Its verdicts are `Decide`
    //! calls, the second-order bound's premise among them, with one
    //! exception: `edge_clear_of_ball` answers clear at once when the
    //! edge's box misses the ball's, a terminal `Bounds` grant. It is the
    //! grant the box constructors' pruning makes, in the disjointness
    //! direction the #571 rule allows: a box miss can only clear an edge
    //! of the ball, never place one in it. The read is necessary: sole
    //! `Decide` reads no box and drives no bisection, and sole `Bounds`
    //! decides nothing. The bound is not a certifying one, because the
    //! reading is the public boolean's and the boolean builds at a dual
    //! (`crates/topo/tests/inside_out_operand.rs`): there the box prune
    //! and the bisection stop read the base scalar's brackets (the crate
    //! README, DL5.1), and every verdict is a `Decide` call. Three of `reduce`'s
    //! crossing-layer functions (`curved_face_arm`, `wall_crossing`,
    //! `settle_deferred`) carry the bound as a reachability ride to it and
    //! read no bracket of their own.
    //!
    //! `topo::validate`'s `ring_pairs` — check 9's broad phase over the
    //! rings of one face — falls under this entry on the same terms as
    //! `census::Trees`: it boxes each ring through the certified box
    //! constructor (`edge_box`, hulled over the ring's edges at the sweep
    //! pad), builds the C10 tree over those boxes and drives its queries,
    //! deciding nothing; the contact arms that follow decide every pair the
    //! boxes do not clear, through `Decide`. Its one box-level answer is
    //! the pruning grant, in the disjointness direction the #571 rule
    //! allows: a box miss can only clear a pair, never make one meet. The
    //! read is necessary (the ring boxes are brackets read through
    //! `Bounds::lo`/`hi`) and stands beside `Decide` because the arms
    //! that follow decide on the same scalar. It reads no bound
    //! its callers do not already carry (`tier3_local_checks_marked`, under
    //! the at-rest validator's own entry).
    //!
    //! `topo::face_boxes` — `FaceBoxes::of`, the blend's candidate prune
    //! for predicate 2's reach meter — falls under this entry on the
    //! census's terms: it boxes every face through `face_box` and builds
    //! the C10 tree, deciding nothing. Sole `Bounds` fails (`face_box`'s
    //! cylinder rule decides the axis length through the funnel) and sole
    //! `Decide` fails (the boxes are `f64` brackets, read through
    //! `Bounds::lo`/`hi`).
    //!
    //! `Separation::of`, `Separation::certify` and `image` carry **no**
    //! [`CertifiedEnclosure`](super::CertifiedEnclosure), and their box NON-overlap answer is a GRANT
    //! (`certify`'s own doc: *"`Ok(())` is the certificate"*, and
    //! `topo::graft_disjoint_all_keyed` asserts nothing about its operands,
    //! #382). They are instantiable at `Dual64` and at `Dual<Interval>`, and
    //! sound there **by delegation**: a dual's box endpoints are its value
    //! channel's, bit-identically the plain-`T` run's (D9), so a dual run's
    //! certificate is the base scalar's. **Whether they should carry
    //! [`CertifiedEnclosure`](super::CertifiedEnclosure) anyway: answered NO, and the CALLER decides it,
    //! not the door** — the one production caller,
    //! `editor_core::eval::wire::images_apart`, sits beneath
    //! `evaluate<T>`, a MIXED pass whose node kinds are overwhelmingly
    //! non-certifying and which a [`CertifiedBounds`](super::CertifiedBounds) bound here would reach
    //! by propagation. **Doors tighten; passes keep their lanes.** (The
    //! NON-generic return type is not the discriminator; the M9-2 entry
    //! states it. Whether an answer is *wrong* at a dual is a third question,
    //! homed at `geom::projection_policy`'s `mid`, #874.)
    //!
    //! **M5 PR 11 (Ev's lane-split ruling) — `topo::props`'s
    //! certified-quadrature plumbing**, which decides (its `props_quad_*`
    //! funnel margins) and reads brackets into certification arithmetic
    //! (`Interval::from_certified`). Its split from
    //! scalars that may not certify is STATIC and stands on two things:
    //! `topo::QuadLane::certified`, at `Certifies + CertifiedBounds`, is the only
    //! entry from the reporting walks, and the `quad_lane::*` signatures
    //! carry [`CertifiedEnclosure`](super::CertifiedEnclosure) as a third term, which no `Dual` has.
    //!
    //! **Re-counted when the certified half grew a LEVEL** (tier 3's +V
    //! check certifying a sign rather than a precision). Nothing about the
    //! seam's scope moved: the certified quadrature is entered over a round
    //! WINDOW now, so where one function took the compound bound there are
    //! a hook, a windowed walk and the certificate type that walk hands
    //! back — every one of them the same lane, the same bound and the same
    //! argument, and the `Decide` half is still what the `props_quad_*`
    //! funnel needs while the bracket half is still what the crossing into
    //! certification arithmetic reads.
    //! The read is unchanged: the windowed walk crosses each stored
    //! value into certification arithmetic through
    //! `Interval::from_certified`, which is the bracket read, and
    //! `Decide` is what accepts a round. No bracket of `T` is branched
    //! on. The certificate type is
    //! PUBLIC and carries the bound for the same reason its walk does —
    //! it resumes that walk, so what it may read and what it may decide
    //! are the walk's, scalar for scalar.
    //!
    //! **The clearance engine's door rides the same seam** (CLEAR's
    //! symbolic-tier ruling, 2026-10-27, applying the M5 PR 11
    //! precedent): `topo::clearance` (`Side::windows`, `Sweep::run`,
    //! `min_separation` and the window and region helpers beneath them)
    //! and `topo::cell_region`'s two doors take a `Body<T>` at
    //! `Decide + CertifiedBounds`. `QuadLane::certified` stays the only
    //! entry from the props reporting walks; these are further doors of
    //! the same kind, outside them. The argument is the same one. The
    //! bracket half is what lifts each surface, curve and point the door
    //! reads into interval arithmetic (`Interval::from_certified`), where
    //! the cells, the enclosures, the funnelled `d − c` and the bracket
    //! are computed. The `Decide` half is what places a cell's or a
    //! witness station's point at `T` through the point-in-face doors.
    //! No bracket of `T` is branched on. The read is the lift into
    //! interval arithmetic and nothing else; `Decide` is what places a
    //! station.
    //!
    //! **M5 PR 12 (orchestrator ruling 2026-08-03 applying the PR 11
    //! precedent; retroactive Ev review per the self-merge convention) — the
    //! edge-blend battery**, `sweep::blend::{battery, surgery, build}`: it
    //! decides (its six `fillet3_*` funnel margins) and CONSUMES ENCLOSURES —
    //! a support's sup-normal-curvature hull through `curvature_lever_arm`, a
    //! blend's setback bound off the analytic arm — reporting each offending
    //! margin as an `f64` payload, which is a bracket read. Since
    //! VERBS-CHAMFER both edge-blend front doors sit here: `chamfer_edges` is
    //! written inside these same three files deliberately, so the
    //! ratification covers the shared lane rather than a fourth file.
    //! Re-scoped 2026-09-05 (FILLET-SPLIT, under Ev's ruling on PR 1916 that
    //! a move with no design implication needs no ask): the two open bands'
    //! carves left `surgery.rs` for `blend/open/planar.rs` and
    //! `blend/open/ruled.rs` unchanged, and the cut-off both open bands
    //! end in later left `ruled.rs` for `blend/open/end_face.rs`, so this
    //! one seam is now spelled over six files — the file list is the
    //! entry's spelling, the seam is the ratified thing, and nothing about
    //! its scope was extended.
    //!
    //! It is the one allowlisted seam with **no refusing lane**, and the
    //! written reason it needs none is the delegation rule below: every
    //! predicate's `Ok`/`Err` in those files comes from a `decide(...)` call,
    //! every `Bounds` read is a typed-error payload or a selection —
    //! including the two that feed a classification or a mutation rather than
    //! sitting after one (`battery.rs` → `chain_g1`, `surgery.rs` →
    //! `body.split_edge`), sound by value-channel delegation — and nothing
    //! there mints a certificate object. A lane whose refusing side would be
    //! empty is dead code, not a guard.
    //!
    //! **The delegation rule (DUAL-DESIGN DL5) — the standing criterion for a
    //! lane-less `Bounds` seam.** Its text is the crate README's "Reading a
    //! bracket where nothing certifies": a `Bounds` read is lane-exempt when
    //! its `f64` (a) leaves, as an error payload or report, or (b) is only
    //! compared, a pick among constructions (DL5.1). An `f64` that comes
    //! back as a `T` is neither, and answers to DL5.2 at
    //! [`Real::from_computed`](super::Real::from_computed):
    //! `geom::projection_policy`'s `mid` (issue 874's class) is that shape. A read that MINTS a certificate
    //! object or feeds a [`CertifiedEnclosure`](super::CertifiedEnclosure) consumer is never exempt: it
    //! needs a lane door in the `topo::QuadLane` shape, and admitting one
    //! without a lane would be a ratified REVERSAL of DL5 on its own
    //! evidence — not an entry this rule can grow.
    //!
    //! **M6-2 (under the PR 11/PR 12 precedent; retroactive Ev review) — the
    //! SSI rung-3 certificate**: `geom_brep::ssi` (the `certify_rung3` door),
    //! `geom_brep::ssi::certify` (the three limbs) and
    //! `geom_brep::pcurve_cache`'s fitted lane, deciding (its `ssi_on_locus`,
    //! `ssi_hull_sup`, `ssi_tube_transversality`, `pcurve_*` funnel margins)
    //! and consuming certification enclosures (limb 2 a control-hull bound, limb 3 a
    //! box-chain enclosure). Its refusing side is **not** empty:
    //! the fitted door (`geom_brep::FittedLane`, answered by
    //! `topo::AtRestPolicy::fitted_lane`) is held at `f64`,
    //! [`Probe`](crate::Probe), the interval scalar and `Sym` over any of
    //! them, and absent at [`Dual`](crate::Dual), dual bodies really
    //! validating and really not holding a fitted cache. That door's module, `geom_brep::fitted_lane`,
    //! is the fitted lane's own seam and not a widening of it: its
    //! constructor (`FittedLane::certified`) and that constructor's
    //! pointer-identity helper carry the certification RIGHT the value
    //! stands for, hold the three `pcurve_cache` bodies by pointer, and read
    //! no bracket.
    //! `geom_brep::ssi::enclose` is deliberately absent: the enclosure machinery
    //! decides nothing and takes the sole bound the rule already allows.
    //!
    //! **M7-8 (under Ev's #264 ruling) — `geom_brep::edge_nurbs`**, the
    //! plane × NURBS declare-and-check edge lane and the narrowest possible
    //! extension of M6-2: it DELEGATES to the already-listed `certify_rung3`
    //! door with a **declared** carrier instead of a marched one, inheriting
    //! that door's signature rather than widening the rule's reach. It
    //! is what keeps `Bounds` off `topo`'s doors: the lane is a sealed
    //! VALUE (`geom_brep::NurbsLane`) whose one constructor carries the
    //! lane bound, handed to the certifying doors as their `lane`
    //! argument, and `topo`'s doors reach those doors through the
    //! scalar's policy (`AtRestPolicy::attach_edge`). Injection moves a
    //! bound onto a narrower signature; it does not remove one.
    //!
    //! **2026-09-02, amending the entry above rather than adding a row — the
    //! lane's split is a BOUND, not a trait.** This lane's static split was
    //! spelled as a `Decide` subtrait with three forwarding impls and a
    //! refusing `Dual` one. The trait is deleted: the shared certified body
    //! is the free function `geom_brep::plane_nurbs_limbs`, at
    //! `Decide + `[`Bounds`](super::Bounds)` + `[`CertifiedEnclosure`](super::CertifiedEnclosure)
    //! exactly as before, and the one DOOR that names it carries
    //! `Certifies + `[`CertifiedBounds`](super::CertifiedBounds) — the lane's
    //! sealed value, `geom_brep::certify`'s `NurbsLane::certified`. That
    //! file joins this allowlist for that reason and no other; the
    //! per-file scope consequence is real and is the price of writing the
    //! obligation where a grep can read it, which is the whole point of
    //! retiring the trait name. **The compound is forced rather than
    //! preferred**: [`Decide`](crate::Decide) descends from `SpanLocate` and
    //! [`Bounds`](super::Bounds) from [`Real`](super::Real), so no sole bound in
    //! the tree spells "decides AND may certify" — the next tighter spelling
    //! is not a weaker term but a missing one, and dropping either term stops
    //! the door compiling. **What changed is the mechanism, not the
    //! strictness**: a dual reached the trait and got
    //! `PlaneNurbsRefusal::LaneUnsupported` at run time; now it cannot form
    //! the call, and the refusal variant is retired with the impl that raised
    //! it. **What a mixed pass does instead** is the edge door split
    //! below (2026-10-25): no door takes the lane as an option.
    //! **The symbolic tier needs no arm of its own and gains none**:
    //! `Sym<T>` implements [`Bounds`](super::Bounds),
    //! [`CertifiedEnclosure`](super::CertifiedEnclosure) and
    //! [`Decide`](crate::Decide) exactly when its base scalar does, so it
    //! satisfies the free function's signature for every certifying base —
    //! which is what the deleted trait's symbolic impl said with an impl.
    //!
    //! **2026-10-25 (Ev's ruling on PR 5689; `geom-brep` README C2) — the
    //! edge door split as a BOUND.** An edge door is two functions: the
    //! mint (`geom_brep::EdgeCurve::mint`), construction at any deciding
    //! scalar, and the certification (`EdgeCurve::certify`,
    //! `certify_stated`, `recertify`, `recertify_stated` and the engine
    //! they share), bounded on `Decide + `[`CertifiedBounds`](super::CertifiedBounds)
    //! and taking the lane as a plain argument. `geom_brep::certify`
    //! carries those doors for that reason, and `topo::policy_lane` the
    //! certifying scalars' door bodies their policies run
    //! (`certifying_attach`, `certifying_reattach`), and nothing else: the
    //! compound is forced for the reason the entry above gives, and what it
    //! buys is that no code path at a dual can name a certifying function —
    //! a compile error, not a runtime `None`. What an edge carries is the
    //! scalar's to say ([`Real::Carries`](super::Real::Carries)): the
    //! certificate where it certifies, [`Uncertified`](super::Uncertified)
    //! at a dual.
    //! The same ruling's C2 (as ratified on PR 5652: limbs 1 and 2 owed
    //! by every carrier) puts a closed-form carrier's limb 2 at the rung-3
    //! limb's own bound, so `geom_brep::edge_nurbs`'s `analytic_edge_limbs`
    //! and `geom_brep::pcurve_cache`'s `limb2_offset_sup` ride the
    //! M6-2/M7-8 entries above with nothing wider.
    //!
    //! **2026-11 (PR 6532; under the M7-8 / C2 precedent; retroactive Ev
    //! review) — `geom_brep::edge_nurbs`'s `general_chart_image`**, the
    //! pcurve mint's image of every carrier on a spline chart, rides at
    //! `Decide + `[`Bounds`](super::Bounds)` + `[`CertifiedEnclosure`](super::CertifiedEnclosure),
    //! `chart_image`'s bound and nothing wider. Necessity: its first rung
    //! IS `chart_image` at that bound, and whether it climbs, its rungs and
    //! its breaks are structure read in `f64` (the wall's held knot lines,
    //! the miss at the scalar's midpoint), which [`Real`](super::Real) offers no read for
    //! but `Bounds::lo`/`hi`. Those reads place rungs and breaks and decide
    //! nothing; `CertifiedEnclosure` is `chart_image`'s, which the first
    //! rung calls.
    //!
    //! **2026-11-12 (under the M7-8 / C2 precedent; retroactive Ev review)
    //! — `geom_brep::edge_nurbs`'s `curved_nurbs_limbs`**, the lane's
    //! certificate for a sphere, cylinder or torus beside a spline chart,
    //! and `residual_nurbs_limbs`, the body it shares with
    //! `plane_nurbs_limbs`, ride at
    //! `Decide + `[`Bounds`](super::Bounds)` + `[`CertifiedEnclosure`](super::CertifiedEnclosure),
    //! `plane_nurbs_limbs`'s bound and nothing wider. Necessity: the shared
    //! body is the plane lane's own, which calls `chart_image` and the
    //! rung-3 door at that bound, and the twin is a second door onto it;
    //! drop either term and neither call compiles.
    //!
    //! **M9-2 PR-1 (under the PR 11 precedent; retroactive Ev review) —
    //! `topo::chart_region`**, the chart-region overlap predicate: it decides
    //! (its `chart_region_*` funnel margins) and reads exact-`f64` STRUCTURE
    //! through the bracket — the spec-mandated C6 inventory gate (a `Harmonic`
    //! trig channel is straight only when its bracket is a point at exactly
    //! `0.0`; the `props.rs` rectangle-trim read) plus the
    //! bit-identical-region fast path — so a sole-bound form is unsatisfiable.
    //!
    //! **The door and the hook guard different things, and both are needed.**
    //! The door's bound is `Decide + `[`CertifiedBounds`](super::CertifiedBounds), which no `Dual`
    //! satisfies, so the predicate is uninstantiable at one however it is
    //! reached — including from outside the crate, where no census is
    //! running. The census's `Option<T::RegionLane>` parameter (the
    //! scalar's `topo::AtRestPolicy::RegionLane`: the lane where the
    //! scalar certifies, a type with no values at a dual; `None` from the
    //! `_structural` twin) is not redundant with that: its `None` is what
    //! lets the census, a MIXED pass, decline this one arm and keep going,
    //! which no bound on a whole function can express.
    //! The tightening replaced an audit rather than a wrong answer, and
    //! **the discriminator is that nothing generic calls this door** — which
    //! is why `topo::separation`, whose caller is a mixed pass, was not
    //! tightened.
    //!
    //! **2026-08-29 (ratified by Ev in conversation) — `editor_core::checks`,
    //! the advisory-check registry**, the **second production caller** of
    //! `topo::separation`. Its bound is `Decide + `[`CertifiedBounds`](super::CertifiedBounds),
    //! **not** `Decide + `[`Bounds`](super::Bounds): nothing generic calls `run_checks` (its
    //! callers are the viewer at a concrete `f64`, the tour, and tests), so by
    //! the M9-2 discriminator it falls on the tighten side and the tighter
    //! bound builds the workspace with zero errors. It clears "brackets never
    //! decide" first — no `lo`/`hi` call appears in the file, and its
    //! bracket-derived verdicts (`SolidSeparation::certify`'s, and
    //! `classify_shells`' through the `props_quad_*` funnel) decide only
    //! whether a FINDING is emitted: the resident REPORTS and never gates
    //! (DS6). One consequence crosses a unit boundary and is recorded here: no
    //! `Dual` implements [`CertifiedEnclosure`](super::CertifiedEnclosure), so `editor_core::run_checks`
    //! is no longer callable at one — DL1 holding one door further out, since
    //! the registry's residents GRANT certificates. The suite
    //! that pins it is `editor-core/tests/r1_dual_probes.rs`, whose
    //! end-to-end rows build the dual body and the closure that would reach
    //! the registry and record that the call cannot be written; that is the
    //! only shape in which an unwritable call can be checked.
    //!
    //! **2026-09-02 — `topo::validate_geometric` and the two `topo::shell`
    //! verbs**, the certified at-rest validator and the one verb that
    //! validates what it built. Tier 3's battery is nine checks, eight of
    //! which any deciding scalar answers and one of which — the +V global
    //! orientation invariant — READS A CERTIFIED VOLUME ENCLOSURE when it
    //! is made through the certified quadrature. The entry is therefore
    //! two private functions, a structural phase at
    //! `T: Decide + Bounds + AtRestPolicy` and a certified half at
    //! `Decide + `[`CertifiedBounds`](super::CertifiedBounds), and the public entry is their
    //! composition, so its bound is the union and IS the compound one this
    //! file ratifies. Its twin `validate_geometric_structural`, at the
    //! structural phase's bound, holds no certified lane and makes the
    //! orientation check through the closed form. `shell`/`shell_open` take the same bound because their
    //! last act is that entry.
    //!
    //! **What it owes "brackets never decide", stated at the substance and
    //! not at the grep.** ONE `lo` and ONE `hi` call appear in
    //! `validate.rs`, and they are disclosed here rather than left to be
    //! discovered: check 1's
    //! [`Bounds::lo`](super::Bounds::lo) of each representability margin an
    //! analytic surface's or edge carrier's conventions state (`geom`'s
    //! `Surface::representability_margins` — a cylinder's, sphere's or
    //! torus tube's radius, and a cone half-angle's distance from each end
    //! of `(0, π/2)` — and `Curve3::representability_margins`, a circle's
    //! radius, an ellipse's semi-axes and a spiric's tube radius; a
    //! carrier's frame has no margin, being a `geom::Frame`, rigid by its
    //! type, so no bracket of it is read), the representability read. Its `hi` decides nothing either: the datum
    //! passes on `lo` alone, and `hi` only says which refusal a margin not
    //! passed is reported as, outside the convention where the whole
    //! bracket is, undecided where it straddles zero (ONEHOME fork 259).
    //! The certified half's own
    //! bracket read is `props`' certified quadrature, already ratified at
    //! the `props.rs` seam; this one compares a STORED DATUM's margin inside
    //! its convention with zero — a radius that is zero, negative or poison
    //! does not describe a small cylinder, sphere or torus, it fails to
    //! describe one — so the read is about whether the datum lies inside
    //! the convention its variant states and not about where geometry lies,
    //! and the value never crosses into a certificate. It takes no `k_stats` name and
    //! no band of its own precisely because it meters nothing — the chamfer's
    //! `NonpositiveSize` precedent; where a convention is itself stated to
    //! within the run's ε (the frame's), the ε is inside the margin `geom`
    //! computes and the read still compares with zero — and the geometric
    //! question beside it (`R - r`) does go through `decide`. `S88`'s named blind spot (a
    //! bracket read behind a renamed accessor) has no instance here any
    //! more: the accessor is gone and the read is spelled `Bounds::lo`
    //! where a grep sees it. What this entry discloses is a different
    //! shape — a bracket read that DECIDES NOTHING — with its one in-fence
    //! instance written down, and the count is not left to prose:
    //! `scripts/gates/bounds-allowlist.sh` pins compound BOUNDS per file,
    //! not reads, so `crates/topo/tests/r1_lane1_bracket_read_census.rs`
    //! counts the file's bracket reads over the code view against the
    //! two disclosed here and reds on a third. The door itself carries
    //! [`Bounds`](super::Bounds) beside [`CertifiedEnclosure`](super::CertifiedEnclosure)
    //! for its callee: the certified quadrature it hands the body to
    //! reads both, under the M5 PR 11 entry.
    //!
    //! **Why the obligation had to become literal at all**, since the door
    //! was already closed to a dual before this row and by construction: it
    //! was not closed. The quadrature lane TRAIT admitted a dual — its arm answered
    //! "no certified quadrature" and the +V check reported that refusal at
    //! run time — so the certification duty rode a trait name that spells
    //! none of this gate's, invisible to every instrument. What changed is
    //! the mechanism, not the strictness: the refusal is retired and the
    //! call a dual cannot honour is unwritable instead. A dual keeps the
    //! `_structural` twin, which holds no certified lane and is where every
    //! certificate its bit-identity rows compare is produced.
    //!
    //! **What a future row owes instead of citing this one.** Two negative
    //! results carried its first draft and neither reaches the question: that
    //! the quadrature lane trait did not imply [`Bounds`](super::Bounds) (true, checked by
    //! deleting the term), and that a refusing lane in that shape would have an
    //! empty refusing side since D1 (also true). Both establish that SOME
    //! bracket bound is needed, never that the WEAK one is.
    //!
    //! **Not an extension — a spelling.** The pair
    //! `Bounds + CertifiedEnclosure` — both bracket doors, no `Decide` — is
    //! spelled [`CertifiedBounds`](super::CertifiedBounds) and is a **sole** bound by construction,
    //! outside the rule's class rather than carved out of it: the rule catches
    //! an evaluation or decision parameter that has also been handed bracket
    //! extraction, and both halves of that pair are bracket-side doors. Adding
    //! [`Decide`](crate::predicate::Decide) to it is a compound bound again
    //! and needs ratification, which the gate enforces: its matcher is shaped
    //! by the trait NAME, so it reads `Decide + CertifiedBounds` as compound
    //! in either operand order, and a **sole** [`CertifiedBounds`](super::CertifiedBounds) does not
    //! fire.
}

/// **"May this value enter certified code?"** — the other half of what
/// [`Bounds`] used to mean, given a name of its own.
///
/// [`Bounds`] answers *"what bracket does this value carry?"*: `[lo(), hi()]` is a superset of every real the value stands
/// for, read off storage, and it stays a sound bracket even when the
/// computation that produced it left a domain somewhere — interval
/// arithmetic still brackets the values the expression *was* defined on.
/// Certification asks a strictly stronger question: *was the expression
/// defined on the whole input box?* A bracket can be sound and still fail
/// that, and code that must not certify a domain violation needs to be
/// able to tell.
///
/// So this trait is not "a better [`Bounds`]" and does not replace it.
/// It is the access-control half, split out, so that the two questions
/// have separate doors and a caller has to say which one it is asking.
/// It carries **no supertrait**: a body that needs a bracket accessor
/// too holds both doors, and says so with the
/// **sole** bound [`CertifiedBounds`] — still an honest inventory of the
/// doors it uses, which is the point of the alias: the inventory is
/// spelled as one name rather than as a compound bound the
/// compound-`Bounds` gate would have to special-case. Write
/// `T: CertifiedBounds`, not `T: Bounds + CertifiedEnclosure`. Making
/// this a subtrait of [`Bounds`] would re-bundle exactly what is being
/// split. Certification entry points bound by `CertifiedEnclosure` cannot be
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
/// - `k_stats::Probe` (feature `probe`) — refuses on NaN, byte-for-byte
///   as `f64` does; D9 forbids the recording lane diverging.
/// - [`crate::Sym`] — delegates to its numeric channel, so it refuses
///   exactly where the scalar it wraps does.
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
///
/// A scalar that certifies carries no tangent: it is its own
/// [`Real::Value`], so certification code hands a place straight to a
/// place-taking evaluator (`geom`'s `eval_at`).
pub trait CertifiedEnclosure: Real<Value = Self> + enclosure_seal::Sealed {
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

    /// `record`, held as this scalar holds a certifying door's record
    /// ([`Real::Carries`]): the one constructor of a held record, on the
    /// trait a dual does not implement.
    fn carried<C: Copy + Debug + Send + Sync + 'static>(record: C) -> <Self as Real>::Carries<C>
    where
        Self: Real;

    /// The record `held` holds.
    fn held<C: Copy + Debug + Send + Sync + 'static>(held: &<Self as Real>::Carries<C>) -> &C
    where
        Self: Real;
}

/// The seal of [`CertifiedEnclosure`], and through it of [`Certifies`]:
/// the scalars that certify are this crate's to name, so each impl of
/// the trait below has its line here, and no other crate can add one.
mod enclosure_seal {
    pub trait Sealed {}
    impl Sealed for f64 {}
    impl Sealed for crate::Interval {}
    #[cfg(feature = "probe")]
    impl Sealed for crate::k_stats::Probe {}
    impl<T: super::CertifiedBounds> Sealed for crate::Sym<T> {}
}

/// **What a tangent-transport scalar holds where a certifying scalar
/// holds a record**: nothing ([`Real::Carries`]). A dual never certifies
/// (DUAL-DESIGN DL1), and a dual body carries no certificate (DL3).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Uncertified;

/// **What a scalar that does not certify holds where a certifying one
/// holds a lane**: a type with no values (the crate README, DL5.3). A
/// pass that runs at every scalar takes its lane as a type that is the
/// lane at a scalar that [`Certifies`] and this at a dual, so the arm
/// that enters certification is not reachable there by type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NoLane {}

/// **What a scalar's expression answers** ([`Real::Expression`]): the
/// symbolic tier's node, or [`NoExpression`].
pub trait Expression: Copy + Debug + Send + Sync + 'static + expression_seal::Sealed {
    /// **Is this expression zero as a function of the document's
    /// variables?** A proof where the symbolic tier's normal form shows
    /// it (ERROR-DESIGN E12, the structural question), and `None` where
    /// it shows nothing: the value may still be zero.
    ///
    /// Not a decision: it counts nothing, records nothing, defers
    /// nothing, and reads no value, so a graph built inside its session
    /// gets one answer at every valuation and in every session. Its
    /// licence is to choose between two spellings of one real value,
    /// and the proof is spent on the spelling it chooses.
    ///
    /// It is a method of the expression, so it cannot be asked of a
    /// number or of a scalar that may be one:
    ///
    /// ```compile_fail,E0599
    /// use geom_core::Expression;
    /// let _ = 0.0_f64.proves_zero();
    /// ```
    ///
    /// ```compile_fail,E0599
    /// use geom_core::Expression;
    /// let _ = geom_core::Interval::from_bounds(0.0, 0.0).proves_zero();
    /// ```
    ///
    /// ```compile_fail,E0599
    /// use geom_core::Expression;
    /// fn asks<T: geom_core::Real>(x: T) -> bool {
    ///     x.proves_zero().is_some()
    /// }
    /// ```
    ///
    /// ```
    /// use geom_core::{Expression, Real};
    /// fn asks<T: Real>(x: T) -> bool {
    ///     x.expression().and_then(Expression::proves_zero).is_some()
    /// }
    /// assert!(!asks(0.0_f64));
    /// ```
    fn proves_zero(self) -> Option<ProvedZero>;
}

pub(crate) mod expression_seal {
    pub trait Sealed {}
    impl Sealed for super::NoExpression {}
}

/// **The expression of a scalar that carries none**: a type with no
/// values, so nothing is asked of it and no impl answers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NoExpression {}

impl Expression for NoExpression {
    fn proves_zero(self) -> Option<ProvedZero> {
        match self {}
    }
}

/// **A proof that an expression is zero** wherever the build takes the
/// decisions it took ([`Expression::proves_zero`]). Only the symbolic
/// tier makes one, and it is neither copied nor cloned: the caller that
/// chooses a spelling on it gives it up there.
#[derive(Debug)]
#[must_use]
pub struct ProvedZero(());

impl ProvedZero {
    /// The tier's proof.
    pub(crate) fn by_the_tier() -> Self {
        Self(())
    }

    /// **The closed spelling of the value proved zero**: the literal
    /// zero, for which the proof is given up.
    #[must_use]
    pub fn zero<T: Real>(self) -> T {
        T::zero()
    }
}

/// **A scalar that transports tangents and never certifies** — the
/// dual's side of [`Real::Carries`]' partition: its doors construct and
/// never check, and what they hold in a record's place is
/// [`Uncertified`]. No scalar implements both this and
/// [`CertifiedEnclosure`].
pub trait TangentTransport: Real {
    /// What this scalar holds in a record's place.
    fn uncertified<C: Copy + Debug + Send + Sync + 'static>() -> Self::Carries<C>;
}

/// `f64` refuses on NaN and only on NaN: the bracket is the value, so
/// the value being poison is the whole of its domain-violation channel
/// (see the trait docs, and D4's Q1 residue for why ∞ is not poison
/// here).
impl CertifiedEnclosure for f64 {
    fn certified_bracket(self) -> Option<(f64, f64)> {
        (!self.is_nan()).then_some((self, self))
    }

    fn carried<C: Copy + Debug + Send + Sync + 'static>(record: C) -> C {
        record
    }

    fn held<C: Copy + Debug + Send + Sync + 'static>(held: &C) -> &C {
        held
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
/// code routinely asks both: it builds certification enclosures through
/// the certified door ([`Interval::from_certified`](crate::Interval::from_certified),
/// whose bound is this one) and reads raw endpoints for the containment and padding
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

/// **A scalar that certifies**, and nothing else: the bound code writes
/// to say it has no meaning at a tangent-transport scalar (the crate
/// README, DL5.3).
///
/// It grants no bracket read and no method, so it is free to write: no
/// list, no ratification, and `scripts/gates/bounds-allowlist.sh` does
/// not read it. A body that also reads a bracket names the bracket door
/// beside it, and that read is what the [`Bounds`] scope rule asks about.
///
/// Implemented once, for every scalar that decides and has a
/// [`CertifiedEnclosure`], and closed because that trait is: its seal
/// (this crate's `enclosure_seal`) names the scalars, and no crate
/// outside can implement it, so none can make a scalar certify: `f64`, [`crate::Interval`], `k_stats::Probe`
/// (feature `probe`) and [`crate::Sym`] over a scalar with both bracket
/// doors. At `Sym<Sym<f64>>` it means what it means at `Sym<f64>`: the
/// symbolic tier certifies as the scalar under it does and changes only
/// how an identically-zero margin decides, however deep it is nested.
/// A scalar that certifies is its own [`Real::Value`] (it carries no
/// tangent), so code at `T: Certifies` hands a place straight to a
/// place-taking evaluator.
/// A [`crate::Dual`] is absent for [`CertifiedEnclosure`]'s reason
/// (`docs/DUAL-DESIGN.md` DL1), over any base:
///
/// ```compile_fail,E0277
/// fn certifies<T: geom_core::Certifies>() {}
/// certifies::<geom_core::Dual64>();
/// ```
///
/// ```compile_fail,E0277
/// fn certifies<T: geom_core::Certifies>() {}
/// certifies::<geom_core::DualInterval>();
/// ```
///
/// ```compile_fail,E0277
/// fn certifies<T: geom_core::Certifies>() {}
/// certifies::<geom_core::Sym<geom_core::Dual64>>();
/// ```
///
/// ```
/// fn certifies<T: geom_core::Certifies>() {}
/// certifies::<f64>();
/// certifies::<geom_core::Interval>();
/// certifies::<geom_core::Sym<f64>>();
/// certifies::<geom_core::Sym<geom_core::Interval>>();
/// ```
pub trait Certifies: crate::predicate::Decide<Value = Self> + certifies_seal::Sealed {}

mod certifies_seal {
    pub trait Sealed {}
}

/// One roster, not two: a scalar certifies exactly when it decides and
/// its computations answer a certified bracket ([`CertifiedEnclosure`],
/// which is sealed). The bound still grants neither: code at
/// `T: Certifies` can name no method of either trait's bracket doors.
impl<T: CertifiedEnclosure> certifies_seal::Sealed for T {}
impl<T: crate::predicate::Decide<Value = T> + certifies_seal::Sealed> Certifies for T {}

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

/// The `|x|` up to which [`Real::less_sin`] reads the series. Past it
/// the difference is more than half of `x`, so it keeps `x`'s digits; up
/// to it each term of the series is at most a fifth of the one before.
pub const LESS_SIN_SERIES_BELOW: f64 = 2.0;

/// The last `k` of the series' kept terms `x^(2k+3)/(2k+3)!`, so the
/// last power kept is `x²³`.
const LESS_SIN_LAST_TERM: u16 = 10;

/// The series of `x − sin x` through `x²³`, for `|x| ≤`
/// [`LESS_SIN_SERIES_BELOW`], nested as
/// `x³/6 · (1 − x²/20 · (1 − x²/42 · (…)))`: each factor is one term's
/// ratio to the one before, `x²/((2k + 2)(2k + 3))`. `tail` stands for
/// the factor the dropped terms would contribute, which lies in
/// `[1 − x²/600, 1]`. Read as `1` it leaves out under `2e-18` of the
/// value at the cutoff, a hundredth of a unit in the last place, and an
/// enclosure passes the whole range.
pub(crate) fn less_sin_series<T: Real>(x: T, tail: T) -> T {
    let q = x.powi(2);
    let nested = (1..=LESS_SIN_LAST_TERM).rev().fold(tail, |inner, k| {
        let ratio = f64::from((2 * k + 2) * (2 * k + 3));
        T::one() - q / T::from_f64(ratio) * inner
    });
    x * q / T::from_f64(6.0) * nested
}

/// `f64` is the kernel's default scalar (D9 determinism notes per method).
///
/// Transcendentals go through the pure-Rust [`libm`] crate: system libm
/// `sin`/`cos` differ across platforms in the last ulp — enough to flip a
/// marginal predicate (D9). `sqrt` and `abs` use the std/hardware
/// operations because IEEE 754 *requires* them to be exact/correctly
/// rounded, so they are already bit-identical everywhere (and faster).
impl Real for f64 {
    fn same_value(self, other: Self) -> bool {
        self.to_bits() == other.to_bits()
    }

    /// **INEXACT**: `f64` compares one rounded number against another.
    /// Its [`Real::register_equal`] refuses with
    /// [`SymRegistration::Disputed`] for that reason, and the same
    /// reason is why a symbolic zero is not checked against this
    /// channel's reading.
    const WITNESS: Witness = Witness::Inexact;

    const NAME: &'static str = "f64";

    type Carries<C: Copy + Debug + Send + Sync + 'static> = C;

    type Expression = NoExpression;

    fn expression(self) -> Option<NoExpression> {
        None
    }

    type Value = f64;

    fn without_tangent(self) -> f64 {
        self
    }

    fn view<F: crate::value_view::OverScalar>(
        x: &F::At<Self>,
    ) -> crate::value_view::ValueView<'_, F::At<Self>> {
        crate::value_view::ValueView::Same(x)
    }

    /// `Dot2` over the error-free differences; where that is not finite
    /// the plain formula's poison or overflow is the answer.
    fn difference_dot(q: [Self; 3], o: [Self; 3], n: [Self; 3]) -> Self {
        use crate::exact::{two_product, two_sum};
        let (mut hi, mut lo) = (0.0, 0.0);
        for i in 0..3 {
            let (d, de) = two_sum(q[i], -o[i]);
            let (p, pe) = two_product(d, n[i]);
            let (s, se) = two_sum(hi, p);
            hi = s;
            lo += se + pe + de * n[i];
        }
        let v = hi + lo;
        if v.is_finite() {
            v
        } else {
            plain_difference_dot(q, o, n)
        }
    }

    /// The double-double reading where it already holds the belly
    /// ([`crate::exact::harmonic_belly_near`]), else the exact expansions,
    /// each rounded once; poison where they cannot be exact (the trait's
    /// docs).
    fn harmonic_belly(curve: StoredHarmonic<Self>, o: [Self; 3], n: [Self; 3]) -> Self {
        if !crate::exact::kept_exactly(curve, o, n) {
            return f64::NAN;
        }
        if let Some(belly) = crate::exact::harmonic_belly_near(curve, o, n) {
            return belly;
        }
        let Some((squares, parts)) = crate::exact::harmonic_belly(curve, o, n) else {
            return f64::NAN;
        };
        let [big_a, big_b, d] = parts.map(|e| e.estimate());
        let rise = big_a.hypot(big_b) + d.abs();
        // A zero denominator is `A = B = D = 0`, whose belly is zero.
        if rise == 0.0 {
            return 0.0;
        }
        squares.estimate() / rise
    }

    /// The exact expansions, each rounded once; poison where they cannot
    /// be exact (the trait's docs).
    fn quartic_sign_algebra(p: Self, q_hat: Self, s: Self) -> [Self; 2] {
        crate::exact::quartic_sign_algebra(p, q_hat, s)
            .map_or([f64::NAN; 2], |parts| parts.map(|e| e.estimate()))
    }

    /// The identity — every `f64` embeds as itself, exactly.
    fn from_f64(x: f64) -> Self {
        x
    }

    fn from_computed(x: f64) -> Self {
        x
    }

    fn told_reading(&self) -> crate::told::Reading {
        crate::told::Reading::value(*self)
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

    /// A point: the value is a finite number.
    fn is_bounded(self) -> bool {
        self.is_finite()
    }

    /// **The witness at a point** ([`Real::register_equal`]): `f64`
    /// tracks no expression, so there is nothing to record — what it
    /// can do is say whether the two values ARE the same real, which is
    /// the half of the door's contract that keeps a constructor from
    /// stating something it did not build.
    ///
    /// **The slack is the run's ε, RELATIVE to the larger magnitude and
    /// floored at one**: `|a − b| ≤ tol.eps() · max(|a|, |b|, 1)`. Each
    /// of the three parts is a decision.
    ///
    /// - **The run's ε**, because this gates whether a registration is
    ///   RECORDED, and a fixed constant is loosest exactly where the
    ///   numeric-first shield is tightest: at ε = 1e-12 a constant of
    ///   1e-9 admits two sides a thousand band-widths apart. It arrives
    ///   as `tol` rather than being read ([`Real::register_equal`]).
    /// - **Relative**, because ε is a length in metres and `f64`
    ///   rounding at coordinates of 10⁹ is ~10⁻⁷: an absolute ε refuses
    ///   a TRUE identity far from the origin. Measured, not reasoned —
    ///   the absolute spelling turns `mesh`'s far-placement ball probe
    ///   and `sweep`'s tube-wall radii probe red in this lane.
    /// - **Floored at one**, so a pair near the origin is compared
    ///   absolutely at ε and the relative form does not shrink to zero
    ///   slack where the magnitudes do.
    ///
    /// **The cost, stated rather than implied.** Tying the slack to ε
    /// LOOSENS it at a loose ε: at ε = 1e-6 this witness admits two
    /// sides a thousand times further apart than the retired 1e-9
    /// constant did, so a lie between those two scales is now recorded
    /// where it was refused. That is the same trade in the other
    /// direction as the tightening at 1e-12, and it is the one a run
    /// asked for by choosing its ε: a registration is an axiom the
    /// registrant's proof carries, and this witness only ever refused
    /// what was visible at the point. Measured: no additional
    /// registration is recorded on any measured document at 1e-6
    /// (`m10_9_no_registrant_lies_on_any_measured_document` pins
    /// `registered` per document, identical at all three rows).
    ///
    /// **This witness is INEXACT** — [`Real::WITNESS`] is
    /// [`Witness::Inexact`] at this scalar, and this arm is that const
    /// spelled as a refusal — **so its refusal is
    /// [`SymRegistration::Disputed`] and never
    /// [`SymRegistration::Contradicted`]**: a comparison at a slack
    /// cannot tell a false claim from a true one the arithmetic lost at
    /// this scale, and `Contradicted` is reserved for the exact witness
    /// that can ([`crate::Interval`]'s meet). The same const is what
    /// keeps a symbolic zero unchecked against this channel's reading;
    /// a row pins the two together, so the door and the tier cannot
    /// drift apart.
    ///
    /// A poisoned value witnesses nothing: NaN is not a real, so no
    /// claim about it is checkable.
    ///
    /// It catches a lie AT THE POINT and nothing else — a claim true at
    /// the nominal and false elsewhere passes here as it passes at
    /// every scalar ([`Real::register_equal`]'s contract).
    fn register_equal(self, other: Self, tol: Tol) -> SymRegistration {
        if self.is_nan() || other.is_nan() {
            return SymRegistration::Unwitnessed;
        }
        let scale = self.abs().max(other.abs()).max(1.0);
        if (self - other).abs() <= tol.eps() * scale {
            SymRegistration::Witnessed
        } else {
            SymRegistration::Disputed
        }
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

    fn less_sin(self) -> Self {
        if self.abs() <= LESS_SIN_SERIES_BELOW {
            less_sin_series(self, 1.0)
        } else {
            self - libm::sin(self)
        }
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

    /// The IEEE comparison `self <= 0.0` behind the trait's poison
    /// guard: a NaN decision cannot choose, so it poisons. Both signed
    /// zeros take the `when_le` arm (`-0.0 <= 0.0` is true in IEEE
    /// 754), which is what makes the tie sign-blind. An unread
    /// candidate's NaN does not propagate — the trait docs give the
    /// reason. Raw comparison is allowed inside scalar implementations
    /// (Q1), as in [`Real::min`].
    fn select_le_zero(self, when_le: Self, when_gt: Self) -> Self {
        if self.is_nan() {
            f64::NAN
        } else if self <= 0.0 {
            when_le
        } else {
            when_gt
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    /// **A scalar's width about zero** ([`width`]): zero at a finite
    /// point, the enclosure's own width either side of zero at
    /// `Interval`, and poison for poison.
    #[test]
    fn width_is_a_points_zero_and_an_enclosures_spread() {
        assert_eq!(width(3.5_f64), 0.0);
        assert!(width(f64::NAN).is_nan() && width(f64::INFINITY).is_nan());
        let w = width(crate::Interval::from_bounds(1.0, 1.5));
        assert!(
            w.lo() <= -0.5 && w.hi() >= 0.5 && w.hi() <= 0.5 + 1e-15,
            "{w:?}"
        );
        assert!(width(crate::Interval::from_bounds(2.0, 2.0)).hi() == 0.0);
    }

    /// The finiteness predicate answers the VALUE channel at every
    /// scalar, and the interval arm is the one that would be wrong if
    /// the self-difference were replaced by a bracket read: an
    /// enclosure whose upper end overflowed still contains its truth,
    /// so it is finite-as-a-value and only poison is not.
    #[test]
    fn is_finite_length_reads_the_value_channel() {
        for x in [0.0, 1.0, -1e300, f64::MAX, f64::MIN_POSITIVE] {
            assert!(is_finite_length(x), "{x} is a finite number");
        }
        for x in [f64::INFINITY, f64::NEG_INFINITY, f64::NAN] {
            assert!(!is_finite_length(x), "{x} is not a finite number");
        }
        // The dual scalar answers about its VALUE: a finite value with
        // any tangent is finite, a poisoned value is not.
        assert!(is_finite_length(crate::Dual64::variable(3.0)));
        assert!(!is_finite_length(crate::Dual64::constant(f64::INFINITY)));
        // The overflow the predicate exists to catch, executed: the
        // norm of a 1e200 component is not a number a direction can be
        // divided by.
        assert!(!is_finite_length(
            crate::Vec3::new(1e200_f64, 0.0, 0.0).norm()
        ));
    }

    /// The underflow predicate separates the two ways a norm comes
    /// out zero, and the rows that matter are the ones a length
    /// comparison alone cannot tell apart: a direction whose squared
    /// norm fell out of the format, and the zero vector.
    ///
    /// The witness is always the largest |component|, which is the
    /// pairing the predicate's contract names.
    #[test]
    fn is_underflowed_length_separates_underflow_from_the_zero_vector() {
        fn ask(v: crate::Vec3<f64>) -> bool {
            let w = Real::max(Real::max(v.x.abs(), v.y.abs()), v.z.abs());
            is_underflowed_length(v.norm(), w)
        }
        // Underflowed: a direction, no length. The last two are
        // SUBNORMAL components, where a predicate that tested
        // `< f64::MIN_POSITIVE` instead of the value channel would
        // have to choose a threshold.
        for v in [
            crate::Vec3::new(1e-180_f64, 0.0, 0.0),
            crate::Vec3::new(1e-180_f64, 1e-180, 1e-180),
            crate::Vec3::new(0.0, -1e-200_f64, 0.0),
            crate::Vec3::new(1e-320_f64, 0.0, 0.0),
            crate::Vec3::new(f64::from_bits(1), 0.0, 0.0),
        ] {
            assert!(ask(v), "{v:?} has a direction and an underflowed norm");
            // …and the bare division by that norm is the blown-up one.
            assert!(!(v / v.norm()).norm().is_finite());
        }
        // Not underflowed: the zero vector really has no direction,
        // and every length that is merely SMALL is still a length —
        // 1e-30 squares to 1e-60, which the format holds.
        for v in [
            crate::Vec3::new(0.0_f64, 0.0, 0.0),
            crate::Vec3::new(1e-30_f64, 0.0, 0.0),
            crate::Vec3::new(1e-160_f64, 1e-170, 0.0),
            crate::Vec3::new(1.0_f64, 2.0, 3.0),
            crate::Vec3::new(1e200_f64, 0.0, 0.0),
        ] {
            assert!(!ask(v), "{v:?} did not underflow");
        }
    }

    /// The zero-vector door answers the zero vector and nothing else,
    /// at `f64` and at the interval scalar, once its precondition has
    /// held — and the precondition row shows why it is one: a finite
    /// vector whose NORM overflowed would be named zero without it.
    #[test]
    fn is_zero_length_names_only_the_zero_vector_after_the_finiteness_question() {
        fn ask<T: Real>(v: crate::Vec3<T>) -> Option<bool> {
            let len = v.norm();
            is_finite_length(len).then(|| is_zero_length(len, v.norm_witness()))
        }
        assert_eq!(ask(crate::Vec3::new(0.0_f64, 0.0, 0.0)), Some(true));
        for v in [
            crate::Vec3::new(1.0_f64, 2.0, 3.0),
            crate::Vec3::new(1e-200_f64, 0.0, 0.0),
            crate::Vec3::new(f64::from_bits(1), 0.0, 0.0),
            crate::Vec3::new(1e150_f64, 0.0, 0.0),
        ] {
            assert_eq!(ask(v), Some(false), "{v:?} is a direction");
        }
        // Overflowed norms: the precondition refuses to ask, and the
        // door asked anyway would answer the wrong thing.
        for v in [
            crate::Vec3::new(1e160_f64, 0.0, 0.0),
            crate::Vec3::new(1e200_f64, 0.0, 0.0),
        ] {
            assert_eq!(ask(v), None, "{v:?}'s norm overflows at f64");
            assert!(is_zero_length(v.norm(), v.norm_witness()));
        }
        {
            use crate::Interval;
            let iv = |x: f64, y: f64, z: f64| {
                crate::Vec3::new(
                    Interval::from_f64(x),
                    Interval::from_f64(y),
                    Interval::from_f64(z),
                )
            };
            assert_eq!(ask(iv(0.0, 0.0, 0.0)), Some(true), "[0,0]/[0,0] is empty");
            assert_eq!(ask(iv(0.0, 0.0, 1.0)), Some(false));
            assert_eq!(ask(iv(1e200, 0.0, 0.0)), Some(false));
        }
    }

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

    proptest! {
        /// **A plane's offset read across a far stored origin keeps its
        /// own digits** ([`Real::difference_dot`]): `q` near the world
        /// origin, `o` on a plane through it a megametre and more along
        /// that plane. The `f64` value is within an ulp or two of the
        /// exact sum and `Dot2`'s second-order term, where the plain
        /// formula rounds at `ulp(far)`; the
        /// enclosure stated at points holds the exact sum within a few
        /// ulps of it.
        #[test]
        fn a_far_origins_offset_is_read_to_its_own_digits(
            far in 1e3f64..1e9,
            c in prop::array::uniform3(-10.0f64..10.0),
            n in prop::array::uniform3(-1.0f64..1.0),
            t in prop::array::uniform3(-1.0f64..1.0),
            gap in -1.0f64..1.0,
        ) {
            use crate::exact::Expansion;
            let n = crate::Vec3::new(n[0], n[1], n[2]);
            prop_assume!(n.norm() > 0.1);
            let n = n.normalize();
            let w = n.cross(crate::Vec3::new(t[0], t[1], t[2]));
            prop_assume!(w.norm() > 0.1);
            let w = w.normalize();
            let q = crate::Point3::new(c[0], c[1], c[2]);
            let o = q - n * gap + w * far;
            let (q, o, n) = ([q.x, q.y, q.z], [o.x, o.y, o.z], [n.x, n.y, n.z]);
            let exact: f64 = Expansion::difference_dot(q, o, n).components().iter().sum();
            let ulp = |x: f64| f64::EPSILON * x.abs().max(f64::MIN_POSITIVE);
            let got = f64::difference_dot(q, o, n);
            // `Dot2`'s bound: the result's rounding and a second-order term.
            let terms: f64 = (0..3).map(|i| (n[i] * (q[i] - o[i])).abs()).sum();
            let u = f64::EPSILON / 2.0;
            prop_assert!(
                (got - exact).abs() <= 2.0 * ulp(exact) + 16.0 * u * u * terms,
                "f64: {got:e} against the exact {exact:e} (far {far:e})"
            );
            let point = |a: [f64; 3]| a.map(crate::Interval::from_f64);
            let enclosed = crate::Interval::difference_dot(point(q), point(o), point(n));
            // `exact` is the expansion's sum rounded, so the true sum is
            // within an ulp of it.
            prop_assert!(
                enclosed.lo() <= exact + ulp(exact) && exact - ulp(exact) <= enclosed.hi(),
                "interval: [{:e}, {:e}] misses the exact {exact:e}", enclosed.lo(), enclosed.hi()
            );
            prop_assert!(
                enclosed.hi() - enclosed.lo() <= 8.0 * ulp(exact),
                "interval: [{:e}, {:e}] is wider than a few ulps of {exact:e}",
                enclosed.lo(), enclosed.hi()
            );
        }
    }

    /// One draw of the far-centred belly row
    /// ([`a_far_centred_harmonics_belly_is_read_to_its_own_digits`]).
    #[derive(Debug)]
    struct BellyDraw {
        far: f64,
        k: f64,
        by: [f64; 3],
        axis: [f64; 3],
        seam: [f64; 3],
        graze: f64,
        anchor: f64,
        arc_held: bool,
        tilt: f64,
        across: [f64; 3],
        depth: f64,
        below: bool,
        slide: f64,
    }

    /// **The draw read and judged**: `Ok(false)` for a degenerate draw,
    /// `Ok(true)` where every reading holds, the failure otherwise.
    fn belly_draw(d: &BellyDraw) -> Result<bool, String> {
        use crate::test_support::stored_belly;
        let vec = |a: [f64; 3]| crate::Vec3::new(a[0], a[1], a[2]);
        let arr = |p: crate::Vec3<f64>| [p.x, p.y, p.z];
        if vec(d.axis).norm() <= 0.1 {
            return Ok(false);
        }
        let axis = vec(d.axis).normalize();
        let u = axis.cross(vec(d.seam));
        if u.norm() <= 0.1 {
            return Ok(false);
        }
        let u = u.normalize();
        let v = axis.cross(u);
        let (a, b) = (d.far, d.k * d.far);
        // The stored point is by the world origin on the arc, or the
        // centre `far` from it.
        let by = crate::Point3::new(d.by[0], d.by[1], d.by[2]);
        let on_arc = |t: f64| u * (a * t.cos()) + v * (b * t.sin());
        let centre = by - on_arc(d.anchor);
        let (at, phase) = if d.arc_held {
            (by, (d.anchor.cos(), d.anchor.sin()))
        } else {
            (centre, (0.0, 0.0))
        };
        let outward = u * (b * d.graze.cos()) + v * (a * d.graze.sin());
        let n = (outward.normalize() * d.tilt.cos() + axis * d.tilt.sin()).normalize();
        let w = n.cross(vec(d.across));
        if w.norm() <= 0.1 {
            return Ok(false);
        }
        let w = w.normalize();
        // From a tenth of the curve's size down to its last digits, so
        // both the double-double reading and the expansions answer.
        let gap = d.far * 10f64.powf(-d.depth);
        let gap = if d.below { -gap } else { gap };
        let o = centre + on_arc(d.graze) + n * gap + w * d.slide;
        let curve = StoredHarmonic {
            at: [at.x, at.y, at.z],
            axis: arr(axis),
            u_ref: arr(u),
            semi: (a, b),
            phase,
        };
        let (o, n) = ([o.x, o.y, o.z], arr(n));
        let u_ = f64::EPSILON / 2.0;
        let got = f64::harmonic_belly(curve, o, n);
        if !stored_belly::within(
            curve,
            o,
            n,
            (got - 8.0 * u_ * got.abs(), got + 8.0 * u_ * got.abs()),
        ) {
            return Err(format!(
                "f64: {got:e} is not within 8u of the belly (the plain formula reads {:e}): {d:?}",
                plain_harmonic_belly(curve, o, n)
            ));
        }
        let enclosed = crate::Interval::harmonic_belly(
            curve.map(crate::Interval::from_f64),
            o.map(crate::Interval::from_f64),
            n.map(crate::Interval::from_f64),
        );
        if !stored_belly::within(curve, o, n, (enclosed.lo(), enclosed.hi())) {
            return Err(format!(
                "interval: [{:e}, {:e}] misses the belly: {d:?}",
                enclosed.lo(),
                enclosed.hi()
            ));
        }
        if enclosed.hi() - enclosed.lo() > 32.0 * u_ * got.abs() {
            return Err(format!(
                "interval: [{:e}, {:e}] is wider than 32u of {got:e}: {d:?}",
                enclosed.lo(),
                enclosed.hi()
            ));
        }
        Ok(true)
    }

    proptest! {
        /// **A far-centred harmonic's belly is read to its own digits**
        /// ([`Real::harmonic_belly`]): a curve of semi-axes `far` and
        /// `k·far`, held at its centre (`far` from the world origin) or on
        /// its arc at a point by the origin, against a plane grazing it at
        /// any angle round the turn, its normal tilted out of the curve's
        /// plane, so `√(A² + B²)` and `|D|` are each about `far`, and the
        /// belly from a tenth of `far` down to its last digits. Judged
        /// exactly on the stored data
        /// ([`crate::test_support::stored_belly`]): the `f64` value is
        /// within `8u` of the belly, relatively, and the enclosure stated
        /// at points holds it and is no wider than `32u` of it.
        #[test]
        fn a_far_centred_harmonics_belly_is_read_to_its_own_digits(
            far in 1e3f64..1e9,
            k in 0.1f64..1.0,
            by in prop::array::uniform3(-10.0f64..10.0),
            axis in prop::array::uniform3(-1.0f64..1.0),
            seam in prop::array::uniform3(-1.0f64..1.0),
            graze in 0.0f64..core::f64::consts::TAU,
            anchor in 0.0f64..core::f64::consts::TAU,
            arc_held in any::<bool>(),
            tilt in -1.4f64..1.4,
            across in prop::array::uniform3(-1.0f64..1.0),
            depth in 1.0f64..16.0,
            below in any::<bool>(),
            slide in -10.0f64..10.0,
        ) {
            let read = belly_draw(&BellyDraw {
                far, k, by, axis, seam, graze, anchor, arc_held, tilt, across, depth, below, slide,
            });
            prop_assume!(read != Ok(false));
            prop_assert!(read.is_ok(), "{}", read.unwrap_err());
        }
    }

    /// **A belly at the stored data's own rounding is read from the
    /// expansions, not the double-double** ([`Real::harmonic_belly`]'s
    /// filter): a draw of the row above at semi-axis 8.9e8, where the
    /// belly is of the order of `u` of the curve's size. A double-double
    /// reading errs there by more than `8u` of the belly, so it holds only
    /// if the filter hands it to the expansions. Red against the filter
    /// accepting every double-double reading.
    #[test]
    fn a_belly_at_the_datas_own_rounding_is_read_from_the_expansions() {
        let draw = BellyDraw {
            far: 888128454.2190003,
            k: 0.47112202864400915,
            by: [0.0, 0.0, -6.179153848113178],
            axis: [
                -0.8602968179554397,
                -0.1678607116122417,
                0.11944456902113416,
            ],
            seam: [0.41905906378434005, 0.5317759491401944, -0.8204683221011131],
            graze: 2.454010139702463,
            anchor: 5.21957173226654,
            arc_held: false,
            tilt: -0.14796864220551767,
            across: [
                -0.21743992587270297,
                -0.12484256262884375,
                -0.6085511504449894,
            ],
            depth: 15.904608451721286,
            below: true,
            slide: 3.9130836524939507,
        };
        assert_eq!(belly_draw(&draw), Ok(true));
    }

    /// **A belly whose digits no exact expansion keeps is poison, not a
    /// plain reading** ([`Real::harmonic_belly`]): a normal component of
    /// `1e-200` puts `D²`'s deepest monomial below the least subnormal, so
    /// the `f64` reading is NaN (a decision on it escalates), and the
    /// enclosure falls back to the plain one, which still holds the belly.
    #[test]
    fn a_belly_past_the_exact_range_is_poison() {
        let curve = StoredHarmonic {
            at: [-1e6, 0.0, 0.0],
            axis: [0.0, 0.0, 1.0],
            u_ref: [1.0, 0.0, 0.0],
            semi: (1e6, 3e5),
            phase: (0.0, 0.0),
        };
        let (o, n) = ([1e-9, 0.0, 0.0], [1.0, 1e-200, 0.0]);
        assert!(f64::harmonic_belly(curve, o, n).is_nan());
        let enclosed = crate::Interval::harmonic_belly(
            curve.map(crate::Interval::from_f64),
            o.map(crate::Interval::from_f64),
            n.map(crate::Interval::from_f64),
        );
        assert!(crate::test_support::stored_belly::within(
            curve,
            o,
            n,
            (enclosed.lo(), enclosed.hi())
        ));
    }

    /// The door at `f64`: a total order, sign-blind at the tie, and
    /// poisoned only by the decision.
    #[test]
    fn select_le_zero_is_a_total_order_with_a_sign_blind_tie() {
        let sel = <f64 as Real>::select_le_zero;
        assert_eq!(sel(-1.0, 7.0, 9.0), 7.0);
        assert_eq!(sel(1.0, 7.0, 9.0), 9.0);
        // The tie takes `when_le`, and BOTH zeros are the same tie —
        // this is the whole difference from `copysign`, which reads the
        // zero's sign bit.
        assert_eq!(sel(0.0, 7.0, 9.0), 7.0);
        assert_eq!(sel(-0.0, 7.0, 9.0), 7.0);
        assert_eq!(<f64 as Real>::copysign(1.0, -0.0), -1.0);
        // Totality on the extremes; ±∞ is not poison.
        assert_eq!(sel(f64::NEG_INFINITY, 7.0, 9.0), 7.0);
        assert_eq!(sel(f64::INFINITY, 7.0, 9.0), 9.0);
        assert_eq!(sel(f64::MIN_POSITIVE, 7.0, 9.0), 9.0);
        assert_eq!(sel(-5.0e-324, 7.0, 9.0), 7.0);
        // The chosen arm is returned BITWISE — a selection, not an
        // arithmetic combination.
        assert_eq!(sel(-1.0, -0.0, 9.0).to_bits(), (-0.0f64).to_bits());
        // A poisoned decision cannot choose. An UNREAD candidate's
        // poison does not propagate: that is what lets a construction
        // select away from a degenerate branch.
        assert!(sel(f64::NAN, 7.0, 9.0).is_nan());
        assert_eq!(sel(-1.0, 7.0, f64::NAN), 7.0);
        assert_eq!(sel(1.0, f64::NAN, 9.0), 9.0);
        assert!(sel(-1.0, f64::NAN, 9.0).is_nan());
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

        /// abs is even, and exact at its argument: `abs(x)` is `x` with
        /// the sign removed, which is non-negative by construction — so
        /// non-negativity is the pinned value's own consequence and not a
        /// separate property. Evenness is a claim about the *reflected*
        /// argument, which a pin at `x` does not make, so it is asserted.
        #[test]
        fn abs_properties(x in -1.0e12..1.0e12f64) {
            prop_assert_eq!(Real::abs(-x), Real::abs(x));
            prop_assert_eq!(Real::abs(x), if x < 0.0 { -x } else { x });
        }
    }
}
