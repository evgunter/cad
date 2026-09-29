//! Trilean sign classification — the single door from numbers to
//! decisions (Q1 in `docs/DESIGN.md`).
//!
//! Every topology-determining branch in the kernel is a *sign decision
//! about a margin*: a scalar quantity m (a signed distance, a dot
//! product, a discriminant) classified against the run's global
//! [`Tolerance`](crate::tolerance::Tolerance). This module owns that classification primitive; the
//! geometry layers build *named predicates* (side-of-plane,
//! transversality, …) on top of it and never compare scalars directly.
//! Evaluation code — generic over [`Real`](crate::real::Real) — can only compute: `Real`
//! deliberately carries no comparisons. Code that needs to branch takes
//! the separate [`Decide`] bound, and [`Decide::sign_within`] is the only
//! passage from scalar values to control flow.
//!
//! # Trichotomy, not boolean
//!
//! Q1 sketches predicates as `Result<bool, Indeterminate>`; the primitive
//! implemented here is the full sign trichotomy
//! `Result<Sign, Indeterminate>`. This is a deliberate generalization:
//! side-of-surface tests need all three outcomes, and every boolean
//! predicate is a projection of a sign ([`Sign::is_zero`] and friends
//! compose with [`Decide::sign_within`] via `?` and `map`). Flagged for
//! ratification in this PR's design conversation.
//!
//! # The ambiguity band is semantic, not numerical
//!
//! A [`Band`] carries two thresholds: `zero` (the coincidence threshold —
//! ε for a linear margin, or the derived angle ε/r for an angular margin at
//! lever arm r) and `escalate` (= K·`zero`, with K the run-configured multiplier [`Tol::k`](crate::tolerance::Tol), default [`DEFAULT_K`] = 10).
//! Classification at `f64`:
//!
//! - |m| ≤ `zero` — **coincident**: [`Sign::Zero`].
//! - |m| ≥ `escalate` — **definite**: [`Sign::Negative`] or
//!   [`Sign::Positive`] by the sign of m.
//! - `zero` < |m| < `escalate` — the **ambiguity band**: the typed
//!   [`Indeterminate`] outcome. Never a guess.
//!
//! The open band (ε, K·ε) is a *design statement*, not a noise model: a
//! margin inside it means "distinct, but too close to the coincidence
//! threshold to build sound geometry from" — sliver faces and
//! near-degenerate features, D4 ¶3's "almost always a modeling mistake".
//! The band is indeterminate *even under exact arithmetic*; consistently,
//! the interval instantiation (M0 PR 4) will treat an enclosure inside
//! the band as indeterminate even when it is a single point. Under this
//! reading, f64 evaluation noise is absorbed informally — f64 answers are
//! best-effort, and certification is the interval instantiation's job
//! (Q1's architecture). The rejected alternative reading — a thin
//! symmetric noise buffer around ε, K = 1 + η/ε for a per-predicate
//! evaluation-noise bound η — would accept some models this reading
//! rejects, but it rests on per-predicate conditioning claims that cannot
//! be verified until interval replay exists, and it makes the certified
//! Zero region subtly smaller than the *defined* coincidence region. In
//! fairness to that reading, its K = 1 + η/ε would be a *measurable*
//! quantity — derived from an actual per-predicate noise bound — whereas
//! the sliver band's K (default [`DEFAULT_K`] = 10; run-configurable since M2 PR 7) is a semantic choice, an
//! honest guess about how much clearance beyond ε sound geometry needs,
//! pending the M0 multi-ε experiments. We take the design statement over
//! the measured buffer here, but the buffer reading's K is the more
//! principled *number* and that is not a point in this reading's favor.
//! The consequence users see here: a model whose true margin lands in
//! (ε, K·ε) fails loudly rather than building; the fix is to widen the
//! feature or shrink ε.
//!
//! # Constructing bands, deciding predicates
//!
//! A [`Band`] is a parameter to [`Decide::sign_within`], not something a
//! predicate constructs on the fly. The idiom: an **operation** builds its
//! band(s) once, at operation entry — `Band::linear(tol)?` /
//! `Band::angular_at(tol, r)?` (a suite pinning its own scale has
//! `Band::linear_at(tol, eps)?` as well) — where the operation's own richer error enum can
//! absorb a [`BandError`] (a misconfigured tolerance is the operation's
//! problem to report), and then threads the `Band` down into the
//! predicates it evaluates. Predicates and the classifier take the band as
//! given and only ever return [`Indeterminate`].
//!
//! [`Band::linear`] takes the run's tolerance witness
//! ([`Tol`]) and reads ε through it — so a function
//! that builds a band says so in its own signature, and one that does not
//! is visibly ε-free. [`Band::linear_at`] takes the witness and an ε of
//! the caller's own — the run's K over a named scale, for a probe that
//! pins its ε and still wants the run's escalation; it is `#[doc(hidden)]`
//! because that is an instrument, not a production door. [`Band::angular_at`] takes the same witness plus a
//! lever arm r, and derives its coincidence threshold as the angle
//! ε/r — there is deliberately **no** global angular tolerance (D4 ¶1, as
//! revised 2026-07-16): an angle's tolerance is meaningless without the
//! length scale it acts through, so every angular threshold is derived per
//! predicate from ε and the arm the decision turns on.
//!
//! # The rate pair, beside the doors
//!
//! Parameter space crosses to model space only through a per-kind
//! metric rate, and the rate's **bound direction** is the semantic
//! content of that crossing, not a detail of how it was derived.
//! [`SupSpeed`] and [`InfSpeed`] — metres per parameter unit — carry
//! the direction in the type, so the two metric doors
//! ([`Margin::metered`], [`Margin::metered_sup`]) are told apart by
//! the compiler rather than by a paragraph. **This is the rule's one
//! home**; every door, type and conversion below points here rather
//! than restating it:
//!
//! - **inf** for a "definitely apart" claim, where a certified LOWER
//!   bound under-states the length and so cannot certify a sliver;
//! - **sup** for an overshoot or an escape, where a certified UPPER
//!   bound over-states the displacement and so can only refuse.
//!
//! Both are transparent tags rather than positivity witnesses: a rate
//! of zero, infinity or poison means something different at each site
//! and each keeps its own guard. Their conversions are one operation
//! each, so typing a site moves no margin's bits.
//!
//! There is deliberately **no** `From<BandError> for Indeterminate`: a
//! misconfigured band (K·ε overflowed, thresholds inverted) is not an
//! indeterminate margin. They are different failures with different types
//! — one is "this run's tolerance cannot form a band", the other is "this
//! margin is too close to call". Consequently a `Band::linear(tol)?` written
//! inside a `fn … -> Result<_, Indeterminate>` does *not* compile: that is
//! the type system pointing out that band construction belongs at the
//! operation layer, above the predicate, not inside a function whose only
//! error is the in-band verdict.
//!
//! Naming (see [`Indeterminate::with_predicate`]): a predicate's static
//! name identifies the *decision* that classified the margin, and the leaf
//! predicate function attaches it at its own definition site. Composite
//! predicates do not rename — higher layers add context through their own
//! typed error wrappers (D4 ¶3), never by overwriting the leaf's name.
//!
//! # Boundary and special-value semantics (f64)
//!
//! Both closures are chosen so the definite regions are exactly the
//! defined ones: `Zero` is closed at |m| = `zero` because D4 *defines*
//! coincidence as |m| ≤ ε, and the definite signs are closed at
//! |m| = `escalate` because a margin of exactly K·ε has the full designed
//! clearance. NaN margins yield [`Indeterminate`] with
//! [`MarginKind::Invalid`] — a poisoned computation never takes a branch
//! (the totality/NaN policy in [`crate::real`]). Infinite margins are
//! definite (an infinite margin is maximally clear of both thresholds),
//! and both signed zeros classify as `Zero` (the sign of a floating-point
//! zero is a representation artifact, not geometry).

use core::fmt;

use crate::spline::SpanLocate;
use crate::tolerance::Tol;

/// The **default** ambiguity multiplier K = 10, re-exported from the
/// tolerance module: a band's `escalate` threshold is K times its
/// `zero` threshold (K·ε for [`Band::linear`], K·(ε/r) for
/// [`Band::angular_at`] at lever arm r).
///
/// Since M2 PR 7 (Ev-directed) K is an ε-style once-per-run
/// configured value — [`Tol::k`](crate::tolerance::Tol), overridable
/// via [`crate::tolerance::ENV_K`] — exactly the growth path this
/// constant's original doc anticipated ("piggyback on the tolerance
/// env mechanism later only if the experiments demand per-run
/// variation"). The default remains the ratified 10 (the M2 K report
/// found no empirical pressure to move it — `docs/K-REPORT.md`), and
/// that finding is **re-measured on every build by `ci.yml`'s
/// `k-lint (gate)`** rather than left as a dated observation — see
/// [`crate::tolerance::DEFAULT_K`] for the register and what fires.
pub use crate::tolerance::DEFAULT_K;

/// The definite outcome of a sign classification: which side of zero a
/// margin certifiably lies on, at the tolerance's resolution.
///
/// `Zero` is a *positive* claim of coincidence (|m| ≤ ε), not a failure
/// to decide — the failure outcome is the typed [`Indeterminate`] error.
///
/// The variant order gives a derived [`Ord`]: `Negative < Zero <
/// Positive`. This is a *post-decision* ordering — it compares
/// already-made classifications, not the scalar values behind them. It is
/// exactly the natural sign order (below zero < at zero < above zero), and
/// it is the order the monotonicity property is stated against (a larger
/// margin never classifies strictly lower). Comparing `Sign`s is never a
/// back door to comparing scalars — the scalar has already passed through
/// the [`Decide`] door and been reduced to one of three decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Sign {
    /// The margin is certifiably negative: m ≤ −`escalate`.
    Negative,
    /// The margin is coincident with zero: |m| ≤ `zero`.
    Zero,
    /// The margin is certifiably positive: m ≥ `escalate`.
    Positive,
}

impl Sign {
    /// The sign of the negated margin: swaps `Negative` and `Positive`,
    /// fixes `Zero`. `sign_within(-m)` and `sign_within(m).flip()` agree
    /// whenever both are definite (under test).
    pub fn flip(self) -> Self {
        match self {
            Self::Negative => Self::Positive,
            Self::Zero => Self::Zero,
            Self::Positive => Self::Negative,
        }
    }

    /// Whether the sign is `Negative` — the boolean projection for
    /// strictly-below predicates.
    pub fn is_negative(self) -> bool {
        self == Self::Negative
    }

    /// Whether the sign is `Zero` — the boolean projection for
    /// coincidence predicates (Q1's `Result<bool, Indeterminate>` shape
    /// is `sign_within(..).map(Sign::is_zero)`).
    pub fn is_zero(self) -> bool {
        self == Self::Zero
    }

    /// Whether the sign is `Positive` — the boolean projection for
    /// strictly-above predicates.
    pub fn is_positive(self) -> bool {
        self == Self::Positive
    }
}

impl fmt::Display for Sign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Negative => "negative",
            Self::Zero => "zero",
            Self::Positive => "positive",
        })
    }
}

/// Which [`Band`] threshold a [`BandError::InvalidValue`] refers to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BandField {
    /// The coincidence threshold (`zero`).
    Zero,
    /// The escalation threshold (`escalate`).
    Escalate,
}

impl BandField {
    /// The field's name in error messages.
    fn name(self) -> &'static str {
        match self {
            Self::Zero => "zero",
            Self::Escalate => "escalate",
        }
    }
}

/// Typed error from [`Band`] construction (D9: every failure is a typed
/// error, never a panic).
#[derive(Debug, Clone, Copy, PartialEq)]
// The variant roster `topo`'s sample-coverage row reads (this
// crate's `test-support` feature, test builds only).
#[cfg_attr(
    feature = "test-support",
    derive(strum::EnumDiscriminants),
    strum_discriminants(name(BandErrorKind), derive(strum::EnumIter), doc(hidden))
)]
pub enum BandError {
    /// A threshold of the attempted `Band` is not finite and strictly
    /// positive.
    InvalidValue {
        /// The offending threshold.
        field: BandField,
        /// The rejected value.
        value: f64,
    },
    /// The lever arm handed to [`Band::angular_at`] is not finite and
    /// strictly positive. Reported directly against the caller's input —
    /// the call site named a lever arm, so the error names that lever arm
    /// rather than the derived angular threshold ε/r it would have produced
    /// (a direct, actionable diagnostic instead of a downstream one).
    InvalidLeverArm {
        /// The rejected lever arm.
        value: f64,
    },
    /// The thresholds are individually valid but `zero` ≥ `escalate`, so
    /// the open ambiguity band (`zero`, `escalate`) is empty *over the
    /// reals* and the definite regions would meet or overlap. The band
    /// must be a nonempty open interval of real numbers — a
    /// zero-or-negative-width band is a different design (no escalation at
    /// all, or inverted) and is rejected.
    ///
    /// The nonemptiness this enforces is a statement about *real numbers*,
    /// not representable ones. `Band::new(t, t.next_up())` is accepted: at
    /// f64 the open interval `(t, t.next_up())` contains no representable
    /// value — de-facto empty at f64, so no f64 margin ever classifies as
    /// indeterminate against it — yet it is a mathematically nonempty
    /// hairline band and a principled configuration. The interval
    /// instantiation (M0 PR 4) can still be indeterminate over it: an
    /// enclosure that straddles the hairline is not a single representable
    /// point. Only the *exactly*-empty band (`zero` ≥ `escalate`, where
    /// the reals themselves offer nothing between the thresholds) is
    /// rejected here.
    Empty {
        /// The attempted coincidence threshold.
        zero: f64,
        /// The attempted escalation threshold.
        escalate: f64,
    },
}

impl fmt::Display for BandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidValue { field, value } => write!(
                f,
                "the band's {} threshold is {value:e}, and must be finite and positive. \
                 Recourse: set a finite, positive tolerance, from which a derived band \
                 takes both thresholds; a band built directly wants finite, positive ones",
                field.name()
            ),
            Self::InvalidLeverArm { value } => write!(
                f,
                "the band's lever arm is {value:e}, and must be finite and positive. \
                 Recourse: name the lever arm the decision turns on (a radius of curvature \
                 or an extent), or classify a linear margin instead"
            ),
            Self::Empty { zero, escalate } => write!(
                f,
                "the band's zero threshold {zero:e} is not below its escalate threshold \
                 {escalate:e}, so the band is empty. Recourse: raise the escalate threshold \
                 above the zero threshold; a band derived from the tolerance does, with its \
                 multiplier K > 1"
            ),
        }
    }
}

impl std::error::Error for BandError {}

/// The two thresholds a margin is classified against: `zero` (the
/// coincidence threshold — ε for a linear margin, or the derived angle ε/r
/// for an angular margin at lever arm r) and `escalate` (= K·`zero`, the
/// least clearance a definite sign requires). See the [module docs](self)
/// for what the open interval between them — the ambiguity band — means.
///
/// Thresholds are `f64` regardless of the scalar type being classified —
/// like [`Tolerance`](crate::tolerance::Tolerance), they bound margins, which are plain numbers even
/// when the geometry is evaluated at intervals or dual numbers. Units
/// are the kernel's fixed internal units (D4 ¶4): the same meters or
/// radians the margin is measured in.
///
/// The fields are private so a `Band` is valid by construction:
/// `0 < zero < escalate`, both finite, enforced by [`Band::new`] with a
/// typed [`BandError`]. Most callers want [`Band::linear`] or
/// [`Band::angular_at`], which derive the thresholds from the run's global
/// [`Tolerance`](crate::tolerance::Tolerance), or [`Band::linear_at`] for a
/// caller-named ε with the run's K; derived scales (e.g. squared-distance comparisons) go
/// through `Band::new` at the geometry layer — no convenience constructor
/// exists for them before a consumer does.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Band {
    /// Coincidence threshold: |m| ≤ `zero` classifies as [`Sign::Zero`].
    zero: f64,
    /// Escalation threshold: |m| ≥ `escalate` classifies as a definite
    /// sign.
    escalate: f64,
}

impl Band {
    /// Validates and constructs a band with the given thresholds.
    ///
    /// # Errors
    ///
    /// - [`BandError::InvalidValue`] if a threshold is not finite and
    ///   strictly positive (checked `zero` first, then `escalate`).
    /// - [`BandError::Empty`] if `zero` ≥ `escalate` — the ambiguity band
    ///   must be a nonempty open interval.
    pub fn new(zero: f64, escalate: f64) -> Result<Self, BandError> {
        if !(zero.is_finite() && zero > 0.0) {
            return Err(BandError::InvalidValue {
                field: BandField::Zero,
                value: zero,
            });
        }
        if !(escalate.is_finite() && escalate > 0.0) {
            return Err(BandError::InvalidValue {
                field: BandField::Escalate,
                value: escalate,
            });
        }
        if zero >= escalate {
            return Err(BandError::Empty { zero, escalate });
        }
        Ok(Self { zero, escalate })
    }

    /// The band for **linear** margins (meters): (ε, K·ε) from the run's
    /// global [`Tolerance`](crate::tolerance::Tolerance) (its ε and its K).
    ///
    /// Call this once at operation entry, not inside a predicate: the
    /// operation's error enum absorbs the [`BandError`] via `?`, and the
    /// resulting `Band` is passed down to [`Decide::sign_within`] (see the
    /// calling-convention section of the [module docs](self)).
    ///
    /// # Errors
    ///
    /// Two arms are reachable, at opposite ends of the range
    /// [`Tolerance`](crate::tolerance::Tolerance) admits (any finite
    /// ε > 0, any finite K > 1), and they want **opposite** repairs —
    /// which is why the returned [`BandError`] names which one it is:
    ///
    /// - [`BandError::InvalidValue`] on `escalate` when K·ε overflows to
    ///   infinity: the run's ε is within a factor K of `f64::MAX`.
    ///   Unreachable for any physically meaningful tolerance (D4 ¶4's
    ///   session box is meters, ε ≈ 1e-9). The repair is a smaller ε.
    /// - [`BandError::Empty`] when K·ε rounds back onto ε, so `zero` ==
    ///   `escalate` and the open interval between them is empty. Writing
    ///   a subnormal ε as n·2⁻¹⁰⁷⁴, this is exactly fl(K·n) == n — the
    ///   product is rounded onto the subnormal grid, so the increment
    ///   can round away entirely. It needs **both** knobs turned: a
    ///   subnormal ε (and in fact ε ≤ 2⁻¹⁰²³, since above that even the
    ///   least admissible K = 1 + 2⁻⁵² clears the half-ulp) **and** a K
    ///   near 1 — at the smallest ε (n = 1) every K below 1.5 collapses
    ///   the band, and the admitted K-region narrows as ε grows. **No
    ///   normal ε collapses at any admitted K**, and at the ratified
    ///   default K = 10 (`docs/K-REPORT.md`) no ε at all does. The
    ///   repair is to raise ε, or K, or both.
    ///
    /// Neither is a panic and neither is a silently invalid band: D9
    /// makes the residue a typed error. The arms are disjoint — no
    /// tolerance reaches both.
    ///
    /// **This is the one home of that derivation.** The refusals that
    /// carry a [`BandError`] outward cite it rather than restating it;
    /// a second copy is a second thing to keep true, and the arithmetic
    /// here is pinned by `geom-core`'s `tests/band_tolerance.rs`.
    pub fn linear(tol: Tol) -> Result<Self, BandError> {
        Self::linear_at(tol, tol.eps())
    }

    /// The band for a **linear** margin at an ε the caller names: (ε,
    /// K·ε) for the given ε and the run's K.
    ///
    /// The difference from [`Band::linear`] is which half comes from the
    /// run. `linear` takes both halves from the committed
    /// [`Tolerance`](crate::tolerance::Tolerance); this takes only the
    /// **K**, and the coincidence threshold is the one handed in. That is
    /// what a suite pinning its own ε — a bisection variable, an ε
    /// ladder, a fixed scale a row is about — needs while still
    /// escalating the way the run does: K is the run's escalation policy,
    /// not a property of the scale being probed, and `CAD_AMBIGUITY_K`
    /// admits any K > 1, so a literal multiplier beside a chosen ε pins
    /// the row to one K rather than to the run's.
    ///
    /// It is exactly `Band::new(eps, tol.k() * eps)` — same product, same
    /// bits — with the K-coupling named once instead of at each site. A
    /// band at a scale that is **not** K·ε (a squared-distance
    /// comparison, a deliberately fixed ratio) still goes through
    /// [`Band::new`]; this door is only for the run's K.
    ///
    /// **`#[doc(hidden)]`: an INSTRUMENT, not a door**, on
    /// `geom-brep`'s `offset_fit` precedent for the same `_at` shape —
    /// *"the `_at` form takes a chosen target and is `#[doc(hidden)]`,
    /// because it is an INSTRUMENT rather than a door"*. The reason
    /// carries across exactly: naming your own ε beside the run's is
    /// what D4 ¶1's witness rule exists to stop in production code, and
    /// a documented door invites it. Every consumer today is a suite
    /// pinning a scale its row is about — `sweep`'s `common/approx.rs`
    /// and `sf2b_r1_probes.rs`, `geom-brep`'s `pcurve_p1b_r2_probes.rs`
    /// and `curved_torus_arc_residual.rs` (twice). It stays `pub`
    /// because those suites are outside this crate; a production caller
    /// that wants a band wants [`Band::linear`].
    ///
    /// # Errors
    ///
    /// The two arms [`Band::linear`] documents, read at the given ε
    /// rather than the run's — [`BandError::InvalidValue`] on `escalate`
    /// if K·`eps` overflows, [`BandError::Empty`] if K·`eps` rounds back
    /// onto `eps` — plus one `Band::linear` cannot have:
    /// [`BandError::InvalidValue`] on `zero` if `eps` is not itself
    /// finite and strictly positive. The run's ε is validated at commit;
    /// this one is the caller's and is validated here.
    #[doc(hidden)]
    pub fn linear_at(tol: Tol, eps: f64) -> Result<Self, BandError> {
        Self::from_thresholds(eps, tol.k())
    }

    /// The band for an **angular** margin (radians) at a named lever arm
    /// `lever_arm` (meters). The coincidence threshold is the angle
    /// θ = ε/`lever_arm` — the angle whose induced displacement
    /// d = `lever_arm`·θ equals the run's linear tolerance ε — and
    /// `escalate` = K·θ as for every band.
    ///
    /// There is no global angular tolerance (D4 ¶1, as revised 2026-07-16):
    /// an angle only means something through the displacement it induces at
    /// a length scale, so the threshold is always derived per predicate from
    /// ε and the lever arm the decision actually turns on — name that arm at
    /// the call site. Canonical choices:
    ///
    /// - **Tangency classification** — the local radius of relative
    ///   curvature 1/κ_rel: the arm over which the surfaces' tangent
    ///   directions are being compared.
    /// - **Parallelism decisions** — the face extent: the largest in-plane
    ///   distance over which an angular error accumulates into a gap.
    /// - **Conservative universal arm** — the session-box extent (D4 ¶4):
    ///   the largest lever arm anything in the model can have, giving the
    ///   tightest angular threshold that is always safe.
    ///
    /// Constructed once at operation entry, like [`Band::linear`] — see
    /// that method and the calling-convention section of the [module
    /// docs](self).
    ///
    /// # Errors
    ///
    /// - [`BandError::InvalidLeverArm`] if `lever_arm` is not finite and
    ///   strictly positive — validated **first**, before ε is even read, so
    ///   the error names the actual input rather than a derived threshold.
    /// - Otherwise the [`BandError`] from [`Band::new`] over the derived
    ///   threshold θ = ε/`lever_arm`, in three reachable forms. The arm
    ///   is a **caller argument** supplied per predicate, not a
    ///   once-per-run configuration, so all three are reachable at an
    ///   ordinary run ε by naming an extreme arm:
    ///   - [`BandError::InvalidValue`] on `zero` if θ is not itself a
    ///     valid coincidence threshold. The rule is about the **pair**:
    ///     θ rounds to 0 exactly when `lever_arm` ≥ ε·2¹⁰⁷⁵ (equivalently
    ///     ε ≤ 2⁻¹⁰⁷⁵·`lever_arm`) — at ε = 1e-16, for instance, the
    ///     boundary arm is 4.0480450661462123e307, one ulp under which θ
    ///     is still a nonzero subnormal. It overflows to infinity at the
    ///     other extreme, a `lever_arm` below ε/`f64::MAX`, which is
    ///     reachable only for ε above about 8.9e-16. Either way the
    ///     repair is the arm, not ε.
    ///   - [`BandError::InvalidValue`] on `escalate` if θ is finite but
    ///     K·θ overflows — the same overflow residue [`Band::linear`]
    ///     documents, reached here through a tiny arm rather than a huge ε.
    ///   - [`BandError::Empty`] if K·θ rounds back onto θ, by
    ///     [`Band::linear`]'s condition applied to θ rather than to ε.
    ///     **A large arm reaches it at an ordinary ε**: ε = 1e-9 with
    ///     arm = 1e300 gives the subnormal θ = 1e-309, which every K
    ///     below about 1 + 2.5e-15 collapses. K still has to be near 1
    ///     — the default K = 10 collapses no θ — but ε need not be
    ///     extreme, and the session-box extent this method recommends as
    ///     the conservative universal arm is the documented road to a
    ///     large one.
    ///
    /// Each is a typed error rather than a silently invalid band.
    pub fn angular_at(tol: Tol, lever_arm: f64) -> Result<Self, BandError> {
        if !(lever_arm.is_finite() && lever_arm > 0.0) {
            return Err(BandError::InvalidLeverArm { value: lever_arm });
        }
        Self::linear_at(tol, tol.eps() / lever_arm)
    }

    /// The pure scaling policy behind all three tolerance-coupled
    /// constructors: the band (t, k·t) for a coincidence threshold t and
    /// multiplier k. It takes k as a number rather than off a witness,
    /// which is what makes the scaling testable without the global
    /// `OnceLock` (the funnel-test discipline in `crate::tolerance`);
    /// [`Band::linear_at`] is this with k read off the run.
    fn from_thresholds(zero: f64, k: f64) -> Result<Self, BandError> {
        Self::new(zero, k * zero)
    }

    /// The coincidence threshold: margins with |m| ≤ `zero` classify as
    /// [`Sign::Zero`]. Finite and strictly positive by construction.
    pub fn zero(self) -> f64 {
        self.zero
    }

    /// The escalation threshold: margins with |m| ≥ `escalate` classify
    /// as a definite sign. Finite and strictly greater than
    /// [`Band::zero`] by construction.
    pub fn escalate(self) -> f64 {
        self.escalate
    }
}

/// A Margin is dimensionally a length, minted only through the blessed
/// doors.
///
/// The margin is a **length** in the kernel's internal metres — the
/// point deviation from specified geometry — *by signature* (D4's
/// margin dimensional convention, clause (i), RATIFIED 2026-08-05).
///
/// `#[repr(transparent)]` and erased at compile time: **no dimension
/// algebra, no generic dimension parameter**. Most kernel functions are
/// single-kind per argument, so the annotation is a signature fact, not
/// a genericity layer; the vector/linalg interior stays bare `T`. The
/// typed surface is exactly the classify seam ([`crate::k_stats::decide`]
/// and each crate's thin funnel wrappers): every margin the K-telemetry
/// recorder sees is a `Margin<T>` by construction.
///
/// The only constructors are the blessed doors below. Each door's doc
/// states the dimensional argument it makes explicit at the call site;
/// choosing the door IS the site's dimension proof (the per-row
/// arguments live in `docs/predicate-dimension-audit.md`). A margin no
/// door honestly fits is a **finding** — a ledger row — never a cast:
/// there is deliberately no raw construction door.
///
/// The doors compute nothing beyond the single operation they name
/// (a product, a norm, a quotient), so wrapping an existing margin
/// expression through its door is bit-identical to the bare
/// expression — the K-telemetry margin stream is unchanged by
/// construction (the rollout's acceptance).
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Margin<T>(T);

impl<T: crate::real::Real> Margin<T> {
    /// Door: a value that **is** a length already. The dimensional
    /// argument at the call site is one of: a coordinate or parameter
    /// difference on an arc-length-parameterized carrier (a line's `t`
    /// IS metres); a signed projection of a metre vector onto a **unit**
    /// direction (a plane residual `(p − o)·n̂` is the point's
    /// coordinate along the normal); a difference, sum, `min`/`max`, or negation of
    /// quantities that are individually lengths (a radius minus a gap,
    /// a signed-offset difference, folded curvature arms); a residual
    /// documented as normalized to metres (`implicit_residual`'s `/2r`
    /// form); or a distance computed by upstream geometry code whose
    /// contract states metres. What this door does NOT admit: anything
    /// whose metres-ness needs a lever, a root, or a quotient — those
    /// have their own doors, and the operation belongs inside the door.
    pub fn of(margin: T) -> Self {
        Self(margin)
    }

    /// Door: a **dimensionless** quantity levered by an **arm** —
    /// computes `x · arm`. The dimensional argument: `x` is a pure
    /// number (a sine or cosine of unit vectors, an angle or angular
    /// difference in radians, a curvature × length product such as
    /// `κ·L`), and `arm` is the length lever that converts it to the
    /// point deviation it implies (D4's θ·r form). The arm names the
    /// geometry that would move: a radius, an extent, a sector chord, a
    /// curvature arm. Multiplication is the door's one operation, so
    /// `levered(x, arm)` is bit-identical to the bare `x * arm`.
    pub fn levered(x: T, arm: T) -> Self {
        Self(x * arm)
    }

    /// Levered door, second-order (sagitta) form: a **relative
    /// curvature** `κ_rel` (1/m) levered by the **square** of the arm —
    /// computes `curvature_rel * arm.powi(2) * 0.5`, the sagitta bound
    /// κ·L²/2. The dimensional argument: the dimensionless quantity is
    /// the accumulated angle κ·arm, its lever the arm again, and the
    /// half is exact (a power of two). The op order (`powi(2)`, then
    /// the half last) is the kernel's canonical osculation shape — the
    /// interval-square rule requires `powi(2)` — so wrapping the
    /// shipped sites is bit-identical.
    pub fn sagitta(curvature_rel: T, arm: T) -> Self {
        Self(curvature_rel * arm.powi(2) * T::from_f64(0.5))
    }

    /// Levered door, reciprocal form: a quantity levered by the
    /// **reciprocal** of the divisor — computes `x / per_length`. The
    /// dimensional argument is the levered door's with the arm named by
    /// its reciprocal: dividing a dimensionless `sin θ` by a relative
    /// curvature (1/m) IS multiplying by the curvature arm (D4 ¶1's
    /// tangency lever, "normal-parallel within θ ⟺ within ε of the
    /// locus"); dividing a jet-weighted residual (m²/param) by the
    /// jet's speed (m/param) IS projecting onto the unit parameter
    /// direction. Both leave metres. One operation, bit-identical to
    /// the bare `x / per_length`.
    pub fn levered_inv(x: T, per_length: T) -> Self {
        Self(x / per_length)
    }

    /// Metric door (clause (iii) at the seam): a **parameter-space
    /// span** crossed to model space by its per-kind **metric rate** —
    /// computes `span * rate`. The dimensional argument: `span` is in
    /// the carrier's parameter units and `rate` is the kind's metres
    /// per parameter unit, so the product is the model-space length
    /// the span subtends. This is the levered door's shape with a rate
    /// arm — a separate constructor because the argument is clause
    /// (iii)'s (parameter space crosses to model space only through a
    /// per-kind metric door), not a dimensionless-times-length one.
    ///
    /// The rate is an [`InfSpeed`] **by signature**: this is the
    /// "definitely apart" door (module docs, *The rate pair*) — a
    /// forward span, an interior split parameter, a root clear of its
    /// endpoints. An overshoot or an escape takes
    /// [`Margin::metered_sup`]. One operation, bit-identical to the
    /// bare `span * rate`.
    pub fn metered(span: T, rate: InfSpeed<T>) -> Self {
        Self(rate.to_meters(span))
    }

    /// Metric door, sup side: a **parameter-space overshoot** crossed
    /// to model space by a certified UPPER bound on the rate —
    /// computes `span * rate`, the same single operation as
    /// [`Margin::metered`] over the other bound direction.
    ///
    /// The dimensional argument is [`Margin::metered`]'s and the
    /// direction argument is the module docs' (*The rate pair*): this
    /// is the overshoot-and-escape door — trim containment, an iso
    /// row's snap and drift slack, a loop-continuity gap. Reading the
    /// same product as a positive-extent claim would be unsound, which
    /// is why that direction has its own door and its own type.
    pub fn metered_sup(span: T, rate: SupSpeed<T>) -> Self {
        Self(rate.to_meters(span))
    }

    /// Norm door (3-vector form): the Euclidean norm of a
    /// metre-component vector — computes `v.norm()`. The dimensional
    /// argument: each component of `v` is a length (a point difference,
    /// a metre-scaled residual vector), and the Euclidean norm of
    /// lengths is a length.
    pub fn norm3(v: crate::linalg::Vec3<T>) -> Self {
        Self(v.norm())
    }

    /// Norm door (2-vector form): see [`Margin::norm3`]; the same
    /// argument for planar (sketch-plane) geometry.
    pub fn norm2(v: crate::linalg::Vec2<T>) -> Self {
        Self(v.norm())
    }

    /// Door: an oriented/certified **measure over the lever that
    /// scales it down to a length** — computes `measure / lever`. The
    /// dimensional argument: an area (m²) over the boundary length or
    /// lever that scales it — a ring run's Newell area over its
    /// perimeter (`2A/P`, the run's mean width), a chart-orientation
    /// area `a×b·n̂` over its radius, a crossing determinant over its
    /// straddle height — or a signed volume (m³) over the surface area
    /// that levers it (`V/A`, a body's mean thickness) — is the point
    /// displacement the measure subtends at that lever. Exactly zero
    /// when the measure is exactly zero, so non-strict pass directions
    /// are unmoved.
    ///
    /// What this door deliberately does NOT serve (Ev's #213
    /// layering ruling): the **consistency backstops** — inequalities
    /// between integral RESULTS, the `volume_backstop` family. Those
    /// are outside the length seam by design: they decide on bare `T`
    /// through the invariant lane
    /// ([`crate::k_stats::decide_invariant`]), and their firing is a
    /// kernel-invariant (Corrupt-class) error, not a validity refusal.
    pub fn over_lever(measure: T, lever: T) -> Self {
        Self(measure / lever)
    }

    /// The wrapped margin, for the classify seam and for diagnostics
    /// (refusal payloads that echo the margin they classified). This is
    /// an exit, not an entrance: reading the value back does not
    /// construct a `Margin`, so it cannot launder a dimension.
    pub fn value(self) -> T {
        self.0
    }
}

impl Margin<f64> {
    /// Lifts an `f64`-substrate length to the deciding scalar (the
    /// certification-substrate idiom: quadrature margins are computed
    /// in certified `f64` enclosure arithmetic and lifted so every lane
    /// records identically). Dimension-preserving, **not** a
    /// construction door — the door was chosen when the `Margin<f64>`
    /// was built.
    pub fn lift<T: crate::real::Real>(self) -> Margin<T> {
        Margin(T::from_f64(self.0))
    }
}

/// A certified **upper** bound on a speed — metres per parameter unit.
///
/// The sup half of the rate pair ([`InfSpeed`] is the other). Both are
/// `#[repr(transparent)]` newtypes over the run scalar and both are a
/// **dimension-and-direction tag, not a positivity witness**: `new`
/// takes any value, poison and zero included, because the sites
/// disagree about what a zero or infinite rate means and each keeps
/// its own guard (`plane_nurbs_ssi`'s finite-and-positive refusal,
/// `param_rate_gate`'s subtended-length gate,
/// `PatchRegularity::thinness`'s deliberate absence of one).
///
/// # Which bound a site needs
///
/// Which direction a site needs is the module docs' rule (*The rate
/// pair*), stated there and not again here. What this type adds is
/// that the answer is a TYPE fact: [`Margin::metered`] takes the inf
/// and [`Margin::metered_sup`] the sup, so a site that reaches for the
/// wrong one does not compile, and these two rows are what say so.
///
/// A sup handed to the inf door — an over-stated rate read as a
/// "definitely apart" claim, the unsound direction:
///
/// ```compile_fail,E0308
/// use geom_core::{Margin, SupSpeed};
/// let _ = Margin::metered(1.0_f64, SupSpeed::new(2.0_f64));
/// ```
///
/// Its twin differs in one respect — the door matches the tag — and
/// compiles:
///
/// ```
/// use geom_core::{Margin, SupSpeed};
/// let _ = Margin::metered_sup(1.0_f64, SupSpeed::new(2.0_f64));
/// ```
///
/// And the mirror, an inf handed to the sup door — an under-stated
/// escape, the other unsound direction:
///
/// ```compile_fail,E0308
/// use geom_core::{InfSpeed, Margin};
/// let _ = Margin::metered_sup(1.0_f64, InfSpeed::new(2.0_f64));
/// ```
///
/// with the twin that compiles:
///
/// ```
/// use geom_core::{InfSpeed, Margin};
/// let _ = Margin::metered(1.0_f64, InfSpeed::new(2.0_f64));
/// ```
///
/// Stable rustdoc checks only that a `compile_fail` block fails to
/// build; the `,E0308` beside it is not verified there, which is what
/// each twin is for — a typo shared by both would redden the twin.
/// The code was read off `rustc` on these snippets at the pinned
/// toolchain (1.97.0).
///
/// Neither type carries `PartialEq` or `PartialOrd`, so a rate cannot
/// be compared without saying `get()` — the [`Real`](crate::real::Real)
/// surface's rule. The rule it enforces is that **a tagged rate is
/// never `==`'d or `<`'d**, not that no ordering happens: a fold that
/// picks the larger of two sups ([`SupSpeed`] producers do this —
/// `speed_lever`, `nurbs_stretch_bounds`) orders the bare payloads and
/// mints the tag on the result, which is where such a fold belongs.
///
/// ```compile_fail,E0369
/// use geom_core::SupSpeed;
/// let _ = SupSpeed::new(1.0_f64) == SupSpeed::new(1.0_f64);
/// ```
///
/// ```
/// use geom_core::SupSpeed;
/// let _ = SupSpeed::new(1.0_f64).get() == SupSpeed::new(1.0_f64).get();
/// ```
///
/// # The two conversions
///
/// [`SupSpeed::to_meters`] and [`SupSpeed::to_param`] are **one
/// operation each** — `span * s` and `m / s` — and therefore
/// bit-identical to the bare arithmetic they replace (D9: the rate
/// pair moves no margin's bits). A surface's pair of rates is two
/// values, not a type: nothing here folds `u` and `v` together.
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct SupSpeed<T>(T);

/// A certified **lower** bound on a speed — metres per parameter unit.
///
/// The inf half of the rate pair; see [`SupSpeed`] for the direction
/// rule, the tag-not-a-witness contract and the one-operation
/// conversion. This half has ONE conversion and not two: `m / inf`
/// over-states a parameter reach, so the parameter side of the pair is
/// [`SupSpeed::to_param`]'s alone. An **exact** closed-form rate (a line's `1`, a circle's
/// radius, an ellipse's minor semi-axis) is an inf bound by being
/// exact, and is minted through this door.
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct InfSpeed<T>(T);

impl<T: crate::real::Real> SupSpeed<T> {
    /// Tags a rate as a certified upper bound. The tag is the caller's
    /// claim; nothing here checks it, exactly as choosing a
    /// [`Margin`] door is the call site's dimension proof.
    pub fn new(rate: T) -> Self {
        Self(rate)
    }

    /// The rate itself, for arithmetic no conversion door names — the
    /// exit, not an entrance.
    pub fn get(self) -> T {
        self.0
    }

    /// A parameter-space span crossed to metres: `span * s`, one
    /// operation, bit-identical to the bare product.
    pub fn to_meters(self, span: T) -> T {
        span * self.0
    }

    /// A model-space length crossed to parameter units: `m / s`, one
    /// operation, bit-identical to the bare quotient. Dividing by the
    /// SUP under-states the parameter reach, which is the safe side of
    /// a floor or a tube pad: a smaller chart-space region is claimed
    /// than the metre statement licenses.
    ///
    /// This is the pair's ONLY parameter-side conversion, and
    /// deliberately: `m / inf` over-states the reach, which is the
    /// unsafe side of every claim the pair serves, so [`InfSpeed`] has
    /// no `to_param` for a site to reach for.
    pub fn to_param(self, meters: T) -> T {
        meters / self.0
    }
}

impl<T: crate::real::Real> InfSpeed<T> {
    /// Tags a rate as a certified lower bound; see
    /// [`SupSpeed::new`] for what the tag is and is not.
    pub fn new(rate: T) -> Self {
        Self(rate)
    }

    /// The rate itself; see [`SupSpeed::get`].
    pub fn get(self) -> T {
        self.0
    }

    /// A parameter-space span crossed to metres: `span * s`, one
    /// operation, bit-identical to the bare product. Multiplying by
    /// the INF under-states the length, which is what a
    /// definitely-apart claim needs.
    ///
    /// The inf half has no parameter-side conversion: see
    /// [`SupSpeed::to_param`], which is the whole pair's.
    pub fn to_meters(self, span: T) -> T {
        span * self.0
    }
}

/// **The reporting margin**: what the classifier saw when it decided,
/// for error reporting only — on every outcome of
/// [`Decide::sign_within`], definite ([`Decided::margin`]) and
/// indeterminate ([`Indeterminate::margin`]) alike.
///
/// **Nothing decides on it, and the type is what says so.** The
/// decision is the [`Sign`] beside it, taken by the classifier against
/// the band; recovering the number to take a decision the classifier
/// did not take defeats the escalation contract (`crate::real`'s
/// `Bounds` scope rule, clause 2). So the type offers no value a
/// comparison could read in passing:
///
/// - its reading is private — no variant to match, no field, no
///   `PartialOrd`, no conversion to a number;
/// - what a refusal says with it is computed here and rendered: the
///   number itself through `Display`/`LowerExp`, and the conditional
///   tolerance offer of a sized decision through
///   [`MarginDiag::sized_recourse`];
/// - what it does expose carries no number: its [`MarginKind`], which
///   scalar lane classified it or that it was poisoned. Poison is the
///   one fact read off the margin that routes anything, because a
///   poisoned question has no number to decide on; whether an
///   escalation is a terminal sliver is the classifier's own verdict,
///   recorded when it is minted ([`Indeterminate::terminal_sliver`]).
///
/// The one way to the `f64`s is [`MarginDiag::diagnostic_f64_for_error_text`],
/// for a payload that must hand the numbers to another renderer (a
/// Python exception's fields). Its call sites in
/// production code are an allowlist a gate counts
/// (`scripts/gates/reporting-margin-door.sh`), so a decision taken on
/// the number is a visibly named call on that list, and a review that
/// sees the door anywhere but a message or a payload conversion has
/// found the misuse.
///
/// Equality is identity of the reading, for pinning payloads in tests.
/// A comparand has to be minted ([`MarginDiag::value`],
/// [`MarginDiag::enclosure`]), and the same gate counts the production
/// mint sites, so a decision by equality is a listed mint too.
///
/// ```compile_fail,E0599
/// use geom_core::{Band, Decide, MarginDiag, Sign};
/// let band = Band::new(1e-9, 1e-8).unwrap();
/// let e = 5e-9_f64.sign_within(band).unwrap_err();
/// // Deciding on the reporting margin in passing: it must not compile.
/// let _sign = match e.margin {
///     MarginDiag::Value(m) if m > 0.0 => Sign::Positive,
///     _ => Sign::Zero,
/// };
/// ```
///
/// ```compile_fail,E0369
/// use geom_core::{Band, Decide, MarginDiag};
/// let band = Band::new(1e-9, 1e-8).unwrap();
/// let d = 5e-10_f64.sign_within(band).unwrap();
/// // No ordering, and no number to order.
/// let _small = d.margin < MarginDiag::value(1e-9);
/// ```
///
/// ```compile_fail,E0277
/// use geom_core::{Band, Decide};
/// let band = Band::new(1e-9, 1e-8).unwrap();
/// let d = 5e-10_f64.sign_within(band).unwrap();
/// let _m: f64 = d.margin.into();
/// ```
///
/// The door is the one spelling that reaches the number, and it names
/// its only use:
///
/// ```
/// use geom_core::{Band, Decide, ErrorTextReading};
/// let band = Band::new(1e-9, 1e-8).unwrap();
/// let d = 5e-10_f64.sign_within(band).unwrap();
/// assert_eq!(d.margin.diagnostic_f64_for_error_text(), ErrorTextReading::Value(5e-10));
/// assert_eq!(format!("{:e}", d.margin), "5e-10");
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MarginDiag(Reading);

/// The private reading of a [`MarginDiag`].
#[derive(Debug, Clone, Copy, PartialEq)]
enum Reading {
    /// The classified `f64` margin, signed, exactly as submitted.
    Value(f64),
    /// The classified enclosure's bounds, exactly as the interval
    /// scalar held them.
    Enclosure { lo: f64, hi: f64 },
    /// The margin was poisoned.
    Invalid,
}

/// Which shape a [`MarginDiag`] has — and no number.
///
/// Closed by design (the set of scalar instantiations is closed, per
/// D3's closed-enum philosophy).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// The roster `topo`'s sample-coverage row reads (this crate's
// `test-support` feature, test builds only).
#[cfg_attr(feature = "test-support", derive(strum::EnumIter))]
pub enum MarginKind {
    /// What `f64` classification saw: a point margin.
    Value,
    /// What interval classification saw: an enclosure. When it rides an
    /// [`Indeterminate`], the enclosure straddles a decision boundary or
    /// lies inside the ambiguity band. Q1's subdivision driver responds
    /// by splitting the parameter box and re-evaluating at a tighter
    /// enclosure — with one **terminal** case: an enclosure lying wholly
    /// inside one open sliver band (`zero`, `escalate`), or its mirror on
    /// the negative side, is not refinable by subdivision at all (the
    /// band is semantically indeterminate at any width, even for a point
    /// — module docs); the driver escalates it as a genuine D4 ¶3
    /// sliver, not a resolution failure.
    Enclosure,
    /// The margin was poisoned: NaN at `f64`, or — at the interval
    /// scalar — an empty/ill-formed enclosure or a decoration recording
    /// a domain violation somewhere in the computation (see the
    /// totality/NaN policy in [`crate::real`]). A poisoned value carries
    /// no sign information at all — this is not "too close to call", it
    /// is "the question was never validly posed". For the subdivision
    /// driver the causes differ in curability: a domain-clamp `Invalid`
    /// (a `Trv` decoration from a partially out-of-domain enclosure) may
    /// cure under subdivision as the violating sub-box shrinks away,
    /// whereas a NaI `Invalid` (ill-formed from construction) never
    /// cures.
    Invalid,
}

impl From<&MarginDiag> for MarginKind {
    fn from(margin: &MarginDiag) -> Self {
        margin.kind()
    }
}

/// The numbers of a [`MarginDiag`], out of
/// [`MarginDiag::diagnostic_f64_for_error_text`] — for error text and
/// payload conversion only.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ErrorTextReading {
    /// The classified `f64` margin.
    Value(f64),
    /// The classified enclosure's bounds (±∞ for a half-unbounded one).
    Enclosure {
        /// The enclosure's lower bound.
        lo: f64,
        /// The enclosure's upper bound.
        hi: f64,
    },
    /// The margin was poisoned.
    Invalid,
}

impl ErrorTextReading {
    /// The point margin, where the classifier saw one.
    #[must_use]
    pub fn value(self) -> Option<f64> {
        match self {
            Self::Value(m) => Some(m),
            Self::Enclosure { .. } | Self::Invalid => None,
        }
    }
}

/// The pass set of a decision on a size the user may intend: one that
/// passes on a nonzero sign, the only kind a smaller tolerance can
/// decide passing (D4 ¶1 (i)).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SizedPass {
    /// A definitely positive margin.
    Positive,
    /// A positive or zero margin.
    NonNegative,
    /// A definitely positive or definitely negative margin: the decision
    /// passes on either side and refuses only at zero.
    NonZero,
}

impl SizedPass {
    /// Whether a zero verdict passes this decision.
    #[must_use]
    pub fn passes_zero(self) -> bool {
        match self {
            Self::Positive | Self::NonZero => false,
            Self::NonNegative => true,
        }
    }

    /// Whether a tolerance below `v`'s own size decides `v` passing: `v`
    /// is a nonzero margin on a side the decision accepts. A zero margin
    /// (either sign of it) leaves no size to tighten below.
    fn tightens(self, v: f64) -> bool {
        match self {
            Self::Positive | Self::NonNegative => v > 0.0,
            Self::NonZero => v != 0.0,
        }
    }

    /// The tolerance every margin in `[lo, hi]` is decided passing below,
    /// where both ends tighten on one side.
    fn below(self, lo: f64, hi: f64, k: f64) -> Option<f64> {
        (self.tightens(lo) && self.tightens(hi) && (lo > 0.0) == (hi > 0.0))
            .then(|| lo.abs().min(hi.abs()) / k)
    }
}

/// What an unreadable margin may mean, appended to the decision's lever
/// by [`MarginDiag::sized_recourse`].
pub const UNREADABLE_MARGIN_NOTE: &str =
    "an unreadable or collapsed margin may indicate a kernel bug worth reporting";

/// The words a sized decision's recourse table hands
/// [`MarginDiag::sized_recourse`]: everything but the number.
#[derive(Clone, Copy, Debug)]
pub struct SizedWords<'a> {
    /// The lever, after "Recourse: ".
    pub lever: &'a str,
    /// The noun the conditional names ("if this {size} is intended").
    pub size: &'a str,
    /// What the decision passes on.
    pub passes: SizedPass,
    /// Whether the door reading the refusal may name a tolerance at all.
    pub may_tighten: bool,
    /// Appended to the lever as "; {note}" where no smaller tolerance
    /// decides the margin passing.
    pub otherwise: Option<&'a str>,
}

impl MarginDiag {
    /// A poisoned margin.
    pub const INVALID: Self = Self(Reading::Invalid);

    /// The reading of a point margin, as `f64` classification mints it.
    #[must_use]
    pub const fn value(m: f64) -> Self {
        Self(Reading::Value(m))
    }

    /// The reading of an enclosure, as interval classification mints it.
    #[must_use]
    pub const fn enclosure(lo: f64, hi: f64) -> Self {
        Self(Reading::Enclosure { lo, hi })
    }

    /// Which shape this reading has.
    #[must_use]
    pub fn kind(self) -> MarginKind {
        match self.0 {
            Reading::Value(_) => MarginKind::Value,
            Reading::Enclosure { .. } => MarginKind::Enclosure,
            Reading::Invalid => MarginKind::Invalid,
        }
    }

    /// Whether the margin was poisoned ([`MarginKind::Invalid`]).
    #[must_use]
    pub fn is_invalid(self) -> bool {
        self.kind() == MarginKind::Invalid
    }

    /// **The one door to the numbers, for error text only.** A call
    /// outside a message or a payload conversion is a decision on a
    /// reporting margin, which the type exists to prevent; production
    /// call sites are counted by `scripts/gates/reporting-margin-door.sh`.
    #[must_use]
    pub fn diagnostic_f64_for_error_text(self) -> ErrorTextReading {
        match self.0 {
            Reading::Value(m) => ErrorTextReading::Value(m),
            Reading::Enclosure { lo, hi } => ErrorTextReading::Enclosure { lo, hi },
            Reading::Invalid => ErrorTextReading::Invalid,
        }
    }

    /// **The recourse a sized decision's band-decided arm ends in**
    /// (D4 ¶1 (i)), with the value this margin gives: the lever, and —
    /// where the door may name a tolerance and a smaller one decides
    /// the margin passing — "or, if this {size} is intended, tighten
    /// the tolerance below `|m|/K`", `K` the band's multiplier. A point
    /// margin tightens where it is nonzero on a side the decision
    /// accepts; an enclosure where both ends do, below its nearer end.
    /// Otherwise the lever alone, with [`SizedWords::otherwise`] where
    /// given; a poisoned margin adds [`UNREADABLE_MARGIN_NOTE`].
    ///
    /// The choice is made here, from the number, and only a sentence
    /// leaves. Because the sentence is chosen from the number, reading
    /// it is reading the margin: its production callers are the
    /// sized-decision table alone, counted by
    /// `scripts/gates/reporting-margin-door.sh`.
    #[must_use]
    pub fn sized_recourse(self, band: Band, words: SizedWords<'_>) -> String {
        let SizedWords {
            lever,
            size,
            passes,
            may_tighten,
            otherwise,
        } = words;
        let k = band.escalate() / band.zero();
        let below = match self.0 {
            Reading::Value(m) => passes.tightens(m).then(|| m.abs() / k),
            Reading::Enclosure { lo, hi } => passes.below(lo, hi, k),
            Reading::Invalid => return format!("Recourse: {lever}; {UNREADABLE_MARGIN_NOTE}"),
        };
        match (below, otherwise) {
            (Some(v), _) if may_tighten => format!(
                "Recourse: {lever}, or, if this {size} is intended, tighten the tolerance below \
                 {v:e} m"
            ),
            (Some(_), _) | (None, None) => format!("Recourse: {lever}"),
            (None, Some(note)) => format!("Recourse: {lever}; {note}"),
        }
    }

    /// Writes the reading with `f`'s own number format applied to each
    /// number: `m`, `[lo, hi]`, or words for poison.
    fn render(
        self,
        f: &mut fmt::Formatter<'_>,
        num: fn(&f64, &mut fmt::Formatter<'_>) -> fmt::Result,
    ) -> fmt::Result {
        match self.0 {
            Reading::Value(m) => num(&m, f),
            Reading::Enclosure { lo, hi } => {
                f.write_str("[")?;
                num(&lo, f)?;
                f.write_str(", ")?;
                num(&hi, f)?;
                f.write_str("]")
            }
            Reading::Invalid => f.write_str("invalid (NaN or a poisoned enclosure)"),
        }
    }
}

/// The reading as text: `m`, `[lo, hi]`, or
/// `invalid (NaN or a poisoned enclosure)`.
impl fmt::Display for MarginDiag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.render(f, fmt::Display::fmt)
    }
}

/// As `Display`, each number in exponent form.
impl fmt::LowerExp for MarginDiag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.render(f, fmt::LowerExp::fmt)
    }
}

/// A definite outcome of [`Decide::sign_within`]: the sign the margin
/// was classified to, and the reporting margin it was classified on.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Decided {
    /// The decision.
    pub sign: Sign,
    /// What the classifier saw, for error reporting only
    /// ([`MarginDiag`]).
    pub margin: MarginDiag,
}

/// Typed outcome when a sign cannot be certified — the predicate layer's
/// D4 ¶3-style actionable error.
///
/// At `f64` this means the margin landed in the ambiguity band (or was
/// NaN — see [`MarginDiag`]); at the interval instantiation (M0 PR 4) it
/// means the enclosure straddles a decision boundary or was poisoned, and
/// Q1's subdivision driver responds by splitting the parameter box and
/// re-running. Construction code propagates it with `?`.
///
/// Fields are public: this is honest diagnostic data (the achieved
/// margin's reporting view, both thresholds, and the predicate that was
/// being decided), enough for actionable error messages and later
/// margin telemetry without any persisted decision log (dropped per
/// Q1).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Indeterminate {
    /// What the classifier saw — the in-band margin, or the fact that
    /// the margin was invalid — for error reporting only
    /// ([`MarginDiag`]).
    pub margin: MarginDiag,
    /// **The classifier's verdict: is subdivision futile?** Recorded
    /// where the escalation is minted, as a definite outcome records its
    /// sign: `true` when interval classification found the enclosure
    /// wholly inside one open sliver band, `(zero, escalate)` or its
    /// mirror, where no subdivision decides it — interval enclosures
    /// shrink monotonically, so a sub-box's stays inside the band its
    /// parent's was inside (DESIGN Q1: such an enclosure is *terminal*
    /// for the driver). `false` for every other escalation: a straddling
    /// enclosure, a point margin, a poisoned one, and every escalation
    /// minted outside the classifier. The driver reads this, never the
    /// margin.
    ///
    /// It answers that one question, not D9's row-1 terminal-vs-curable
    /// axis in full: a `Value`-kind escalation is terminal there (a
    /// statement about the input), and reads `false` here, because no
    /// box is being subdivided.
    ///
    /// The field is public like its neighbours, so what keeps a caller
    /// from writing `true` on a straddle is
    /// `scripts/gates/reporting-margin-door.sh`, which pins such
    /// literals in production code to none: gate-held, not type-held.
    pub terminal_sliver: bool,
    /// The band the margin was classified against.
    pub band: Band,
    /// The named predicate being decided, when a caller attached one via
    /// [`Indeterminate::with_predicate`]. `None` as produced by
    /// [`Decide::sign_within`] itself, which cannot know its caller.
    pub predicate: Option<&'static str>,
}

/// The unified sub-ε_input recourse sentence (the two-tolerance
/// principle, D4 ¶1 addendum, #129): below ε_input, "exactly on the
/// coincidence" and "in the ambiguity band" are ONE user situation —
/// coincident at any precision the user could care about — with one
/// three-lever recourse, phrased once, here. The margin rides the
/// error payload as data; kernel semantics keep the distinction.
///
/// Every site that refuses on a too-close-to-a-coincidence situation
/// composes this fragment into its Display output — directly for
/// definite arms (exactly-on refusals with no [`Indeterminate`]
/// payload), or through [`Indeterminate`]'s own Display for escalated
/// arms. Message-pinning tests pin the fragment with `contains`, never
/// with full-string pins that rot.
pub const COINCIDENCE_RECOURSE: &str =
    "declare the coincidence, move the geometry, or lower the tolerance";

/// The one recourse for a quantity the floating-point format cannot
/// hold — a length that overflows the norm or underflows to zero while
/// its direction is good. No tolerance reaches it, so it never rides
/// with [`COINCIDENCE_RECOURSE`]; every site that refuses on the
/// format's range composes this fragment, and message-pinning tests pin
/// it with `contains`.
pub const RANGE_RECOURSE: &str = "scale the geometry into the session's range";

/// [`COINCIDENCE_RECOURSE`] at a door that takes no declaration: the
/// two levers left, the geometry and the tolerance. The chord join
/// that a split and a Boolean share composes it, since the join cannot
/// know whether its caller declares; the Boolean's own wrapper adds the
/// declaration back (`topo::BooleanError::Join`).
pub const NO_DECLARATION_RECOURSE: &str = "move the geometry, or lower the tolerance";

/// [`NO_DECLARATION_RECOURSE`] at a split, whose plane is the first
/// lever: a split takes no declarations (`topo::split`'s signature).
pub const SPLIT_PLANE_RECOURSE: &str =
    "move the split plane or the geometry, or lower the tolerance";

/// The one ending of a refusal that only a kernel defect reaches:
/// nothing the user changes in the model is a way through, so the
/// sentence says so plainly and asks for the report.
///
/// **An ending, not a recourse: a whole sentence, marker included.**
/// The `*_RECOURSE` neighbours are repairs a site labels `Recourse:`;
/// a dead end is not a recourse and carries no
/// such label, and the words that say there is none ARE the marker a
/// refusal's shape is checked by (`test_utils::refusal::recourse_markers`
/// counts `There is no way through`). So a site appends this after its
/// own sentence and adds nothing: the ending is here, whole, once.
///
/// A site that must compose it into a `&'static str` reaches the same
/// literal through the hidden `geom_core::kernel_defect_ending!` macro.
pub const KERNEL_DEFECT_ENDING: &str = crate::kernel_defect_ending!();

/// [`KERNEL_DEFECT_ENDING`] where the thing refused may have been
/// READ rather than built: a body or record at rest, which a damaged
/// file reaches as surely as a defective operation does. The user's
/// report is the same; what it is about is not, so the sentence names
/// both. A refusal over something the kernel computed on the spot — a
/// description the constructors already validated — ends in
/// [`KERNEL_DEFECT_ENDING`], since no file stands between them.
pub const KERNEL_OR_FILE_DEFECT_ENDING: &str = crate::kernel_or_file_defect_ending!();

/// The subject a door states for an escalation whose decision it has no
/// words for — a name its table does not carry, or no name at all. One
/// phrase for every door, so the refusal-shape guard can read it as no
/// subject: a row that renders it is red unless admitted by name.
pub const UNNAMED_DECISION: &str = "an unnamed decision";

/// The one ending of a refusal at a shape or configuration the kernel
/// does not build or check yet: nothing the user changes gets through
/// today, and nothing is wrong with what they asked for.
pub const NOT_YET_ENDING: &str = "There is no way through yet";

/// The qualifier a refusal at a kernel approximation limit puts on its
/// one recourse, loosening the tolerance (D4 ¶1 (i)): a kernel
/// approximation limit — an offset fit that stalls, a quadrature budget
/// spent, a built curve's residual — leaves the user no lever but ε,
/// and the kernel falling short there may be a defect.
///
/// **It follows the lever, not stands alone.** It ends the message, but
/// only after the site's own lever and the value its payload gives
/// (`Recourse: loosen the tolerance to {best} m or more, `), appended
/// after a comma so the value stays in the one sentence the marker
/// labels. A site whose payload gives no value writes the whole
/// sentence [`KERNEL_LIMIT_RECOURSE`]. A refusal that names any other
/// recourse — a geometry lever such as splitting the face — names that
/// instead, and no loosening at all: this is for the site that would
/// otherwise name none.
///
/// A site that must compose it into a `&'static str` reaches the same
/// literal through the hidden `geom_core::kernel_limit_last_resort!`
/// macro.
pub const KERNEL_LIMIT_LAST_RESORT: &str = crate::kernel_limit_last_resort!();

/// The whole last-resort recourse for a site whose payload gives no
/// value to size the loosening to: the lever, then
/// [`KERNEL_LIMIT_LAST_RESORT`].
pub const KERNEL_LIMIT_RECOURSE: &str = concat!(
    "Recourse: loosen the tolerance, ",
    crate::kernel_limit_last_resort!()
);

/// [`KERNEL_DEFECT_ENDING`] as a literal, for `concat!` at a site
/// whose prose is a `&'static str` (a constant cannot be spliced into
/// one). The constant is defined through this macro, so the two are
/// one spelling.
#[doc(hidden)]
#[macro_export]
macro_rules! kernel_defect_ending {
    () => {
        "There is no way through: this is a kernel defect; report it"
    };
}

/// [`KERNEL_LIMIT_LAST_RESORT`] as a literal, for `concat!`; see
/// `kernel_defect_ending!`.
#[doc(hidden)]
#[macro_export]
macro_rules! kernel_limit_last_resort {
    () => {
        "as a last resort; this refusal may indicate a kernel bug worth reporting"
    };
}

/// [`KERNEL_OR_FILE_DEFECT_ENDING`] as a literal, for `concat!`; see
/// `kernel_defect_ending!`.
#[doc(hidden)]
#[macro_export]
macro_rules! kernel_or_file_defect_ending {
    () => {
        "There is no way through: this is a kernel defect or a damaged file; report it"
    };
}

/// The one answer a refusal gives when the table that routes its
/// recourse by predicate name does not carry the name that escalated:
/// it states the hole. The name is routing, so it rides `Debug` (this
/// value's field) rather than the sentence. Never a category asserted over the unknown name,
/// never silence — both read as a statement about the escalation, and
/// neither is one anybody made.
///
/// Every `Display` that routes a recourse by predicate name composes
/// this on its fall-through arm. The home is here because the crates
/// that route are `profile` and `sweep`, and this crate is the only
/// ancestor they share: `sweep` depends on `profile`, `profile` on
/// `geom-core` alone, so a sentence held in either of them is out of
/// reach of the other.
///
/// **It reports an absence, never a denial.** The refusal it tails has
/// already rendered [`Indeterminate`]'s own Display, which ends in
/// [`COINCIDENCE_RECOURSE`] — real advice. So this says the TABLE holds
/// nothing further, not that the advice above is not advice; a door
/// that has DECIDED a predicate needs nothing further says so itself
/// rather than reaching this sentence.
#[derive(Debug, Clone, Copy)]
pub struct MissingRecourse<'a>(pub Option<&'a str>);

impl fmt::Display for MissingRecourse<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Whether the escalation carried a name is the one fact about
        // the field a person can use: an unnamed one is not a gap in a
        // table but a decision nobody labelled.
        match self.0 {
            Some(_) => f.write_str("no recourse is recorded for this decision"),
            None => f.write_str("no recourse is recorded for an unnamed decision"),
        }
    }
}

/// Borrowed margin-payload view of an [`Indeterminate`]: the
/// margin/enclosure data and the band — WITHOUT the shared recourse
/// tail, and without the predicate's name, which is routing a developer
/// reads in `Debug` rather than anything the person holding the mouse
/// can act on. For per-site Display impls that compose the
/// two-tolerance message themselves (site context + this payload +
/// [`COINCIDENCE_RECOURSE`]) and must not double the recourse; the
/// bare [`Indeterminate`] Display is this payload plus the shared
/// tail.
#[derive(Debug, Clone, Copy)]
pub struct IndeterminatePayload<'a>(&'a Indeterminate);

impl fmt::Display for IndeterminatePayload<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (zero, escalate) = (self.0.band.zero, self.0.band.escalate);
        let margin = self.0.margin;
        match margin.0 {
            Reading::Value(_) => write!(
                f,
                "margin {margin:e} lies inside the ambiguity band ({zero:e}, {escalate:e})"
            ),
            Reading::Enclosure { .. } => write!(
                f,
                "enclosure {margin:e} cannot be classified against the ambiguity \
                 band ({zero:e}, {escalate:e})"
            ),
            Reading::Invalid => write!(
                f,
                "margin is invalid (NaN or a poisoned enclosure) against the ambiguity \
                 band ({zero:e}, {escalate:e})"
            ),
        }
    }
}

impl Indeterminate {
    /// Attaches a predicate's static name, so an escalation names the
    /// *decision* that classified the margin rather than just the numbers.
    /// A leaf predicate attaches its own name at its definition site:
    /// `m.sign_within(band).map_err(|e| e.with_predicate("side_of_plane"))`.
    ///
    /// By convention the name is the leaf's and stays the leaf's. A
    /// composite predicate built on top of `side_of_plane` does *not*
    /// rename the escalation — it adds its context through its own typed
    /// error wrapper (D4 ¶3), leaving the innermost decision's name intact.
    /// Mechanically this method replaces any name already present (so a
    /// second `with_predicate` would let an outer layer win), but the
    /// convention is that no outer layer calls it: overwriting the leaf's
    /// name would erase which decision actually went indeterminate.
    pub fn with_predicate(self, name: &'static str) -> Self {
        Self {
            predicate: Some(name),
            ..self
        }
    }

    /// The margin-payload view (name + margin data + band, no recourse
    /// tail) — see [`IndeterminatePayload`].
    pub fn payload(&self) -> IndeterminatePayload<'_> {
        IndeterminatePayload(self)
    }
}

impl fmt::Display for Indeterminate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.payload())?;
        match self.margin.kind() {
            MarginKind::Value => write!(f, " — a near-coincidence; {COINCIDENCE_RECOURSE}"),
            MarginKind::Enclosure => write!(
                f,
                " — subdivide the parameter box for a tighter enclosure, or \
                 {COINCIDENCE_RECOURSE}"
            ),
            // Poison explains WHY the sign is indeterminate, but the
            // user's levers at a coincidence site are unchanged — the
            // Invalid arm carries the shared recourse like the others
            // (S6 review, MINOR-1).
            MarginKind::Invalid => write!(
                f,
                " — check the operation's inputs upstream, then {COINCIDENCE_RECOURSE}"
            ),
        }
    }
}

impl std::error::Error for Indeterminate {}

/// Scalars that can classify their sign against a [`Band`] — the single
/// door from numbers to decisions.
///
/// Deliberately a **separate trait** from [`Real`](crate::real::Real) (a supertrait, not an
/// extra bound at use sites): evaluation code that merely computes stays
/// generic over `Real` alone and *cannot* branch on values; only code
/// that genuinely decides — predicate definitions, classification steps —
/// takes `T: Decide`. The split keeps the no-comparison discipline
/// structural (see the evaluation-code discipline in [`crate::real`]) and
/// makes decision points findable: every topology-determining branch is a
/// `sign_within` call site.
///
/// Implemented for `f64` here; the interval instantiation lands in M0
/// PR 4 (enclosure-based classification, indeterminate when the enclosure
/// straddles a boundary), and dual numbers in M0 PR 5 classify their
/// value part only — a derivative never influences a branch.
///
/// [`SpanLocate`] (M5 PR 3) is a supertrait (which brings
/// [`Real`](crate::real::Real) with
/// it): every decision-capable scalar has an authoritative
/// value/enclosure channel, and knot-span selection reads exactly that —
/// so `T: Decide` code (the topology layer's bound) can evaluate NURBS
/// carriers without naming the sealed span seam. Purely additive:
/// `SpanLocate` grants structure *selection* (span indices), never value
/// comparison or bound extraction.
pub trait Decide: SpanLocate {
    /// Classifies this value's sign against `band`, per the boundary
    /// semantics in the [module docs](self). Every outcome carries the
    /// reporting margin the classifier saw ([`Decided::margin`],
    /// [`Indeterminate::margin`]), minted here and nowhere else.
    ///
    /// # Errors
    ///
    /// [`Indeterminate`] when the sign cannot be certified: the margin
    /// lies strictly inside the ambiguity band, or is invalid (NaN /
    /// empty enclosure). The error carries the margin diagnostic and the
    /// band; callers attach their predicate name via
    /// [`Indeterminate::with_predicate`].
    fn sign_within(self, band: Band) -> Result<Decided, Indeterminate>;

    /// **The certified enclosure this value would be classified on**,
    /// for the shape report's use only ([`crate::sym::report`]'s
    /// `DecisionShape::enclosure`). `None` at every scalar that has no
    /// enclosure, which is the default and the only implementation
    /// outside [`crate::Interval`].
    ///
    /// It exists because "what bounds this document" is not answerable
    /// from predicate NAMES: several predicates can be over the band at
    /// once, and which one a drive reports is evaluation order. Reading
    /// the SET with its enclosures is what makes the bound a
    /// measurement (M10-9's fix pass; adopted from a review probe).
    ///
    /// It is an instrument, not a decision channel: nothing in the
    /// funnel may branch on it, and it is read at the report's call
    /// sites only. The read itself is two `f64` copies at `Interval`,
    /// so it is not guarded — a guard would cost what it saves.
    fn enclosure_probe(self) -> Option<(f64, f64)> {
        None
    }
}

/// `f64` classification: |m| ≤ `zero` ⇒ `Zero`; |m| ≥ `escalate` ⇒ the
/// sign of m; strictly between ⇒ [`Indeterminate`]; NaN ⇒
/// [`Indeterminate`] with an invalid [`MarginDiag`]. Every outcome
/// reports the margin as submitted ([`MarginDiag::value`]).
///
/// Deterministic per D9: built from IEEE comparisons and `abs` (exact,
/// bit-identical on every conforming platform). Raw comparison is
/// allowed *inside* scalar implementations — it is generic evaluation
/// code that must not branch on values (Q1); this impl is precisely the
/// place where comparisons are turned into certified decisions.
impl Decide for f64 {
    fn sign_within(self, band: Band) -> Result<Decided, Indeterminate> {
        if self.is_nan() {
            return Err(Indeterminate {
                margin: MarginDiag::INVALID,
                band,
                predicate: None,
                terminal_sliver: false,
            });
        }
        let margin = MarginDiag::value(self);
        let magnitude = self.abs();
        let sign = if magnitude <= band.zero {
            // Includes both signed zeros (|±0.0| = +0.0 ≤ zero).
            Sign::Zero
        } else if magnitude >= band.escalate {
            // Includes ±∞: an infinite margin is maximally definite.
            // `self` is non-NaN with |self| ≥ escalate > 0 here, so it is
            // strictly one-signed.
            if self > 0.0 {
                Sign::Positive
            } else {
                Sign::Negative
            }
        } else {
            // A point margin says nothing about a box.
            return Err(Indeterminate {
                margin,
                band,
                predicate: None,
                terminal_sliver: false,
            });
        };
        Ok(Decided { sign, margin })
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::interval::certification::Certification;
    use crate::tolerance::Tol;
    use proptest::prelude::*;

    // Global-state discipline (see `crate::tolerance`'s test module):
    // bands here are built with explicit thresholds via `Band::new` or
    // the pure `Band::from_thresholds`, never via `Band::linear` /
    // `Band::angular_at`, which derive ε from the global `Tolerance`
    // and would race the lib test binary's single designated
    // global-touching test. The one exception is the `linear_at`
    // agreement row, which READS the witness (`Tol::witness().k()`) and
    // never commits a value: every run reaches the same committed
    // tolerance, so there is nothing to race. Anything that needs a
    // PARTICULAR tolerance — both arms of the constructors' `# Errors`
    // included — lives in `tests/band_tolerance.rs`, which re-execs a
    // child process per row.
    //
    // The spec-pinning test here is `f64_boundary_table`: it nails every
    // closure choice at the exact thresholds. The proptests below are
    // property checks over continuous generators, which (almost surely)
    // never land a margin on an exact boundary — so they corroborate the
    // structure but do not, and cannot, pin the boundary semantics.

    /// The fixed band used by the boundary table: exactly the default
    /// tolerance's linear band, but constructed purely.
    fn band_1e9() -> Band {
        Band::new(1e-9, 1e-8).unwrap()
    }

    /// **Every outcome carries the margin it was classified on**: a
    /// definite `f64` verdict reports the margin as submitted, an
    /// interval one its enclosure, and the in-band and poisoned outcomes
    /// keep theirs.
    #[test]
    fn every_outcome_reports_its_margin() {
        let band = band_1e9();
        for m in [0.0, -0.0, 5e-10, -1e-9, 1e-8, -1.0] {
            let got = m.sign_within(band).unwrap();
            assert_eq!(got.margin, MarginDiag::value(m), "f64 {m:e}");
            let got = crate::Interval::point(m).sign_within(band).unwrap();
            assert_eq!(got.margin, MarginDiag::enclosure(m, m), "Interval {m:e}");
        }
        assert_eq!(
            5e-9.sign_within(band).unwrap_err().margin,
            MarginDiag::value(5e-9)
        );
        assert!(f64::NAN.sign_within(band).unwrap_err().margin.is_invalid());
    }

    /// Which margins a smaller tolerance decides passing, per pass set:
    /// a one-sided set tightens positive margins only, the two-sided set
    /// any nonzero one, and none tightens zero.
    #[test]
    fn each_pass_set_tightens_the_margins_it_accepts() {
        use SizedPass::{NonNegative, NonZero, Positive};
        let rows = [
            (Positive, 5e-9, true),
            (Positive, -5e-9, false),
            (NonNegative, 5e-9, true),
            (NonNegative, -5e-9, false),
            (NonZero, 5e-9, true),
            (NonZero, -5e-9, true),
            (Positive, 0.0, false),
            (NonNegative, -0.0, false),
            (NonZero, 0.0, false),
            (NonZero, -0.0, false),
        ];
        for (pass, v, want) in rows {
            assert_eq!(pass.tightens(v), want, "{pass:?} at {v:e}");
        }
        assert!(!Positive.passes_zero() && !NonZero.passes_zero() && NonNegative.passes_zero());
    }

    /// An enclosure is decided below its nearer end's `|m|/K` only when
    /// both ends sit on one accepted side; one across zero is decided
    /// passing by no tolerance.
    #[test]
    fn an_enclosure_tightens_only_with_both_ends_on_one_side() {
        use SizedPass::{NonZero, Positive};
        let k = 10.0;
        let rows = [
            (NonZero, 2e-9, 5e-9, Some(2e-10)),
            (NonZero, -5e-9, -2e-9, Some(2e-10)),
            (NonZero, -2e-9, 3e-9, None),
            (NonZero, 0.0, 3e-9, None),
            (Positive, 2e-9, 5e-9, Some(2e-10)),
            (Positive, -5e-9, -2e-9, None),
            (Positive, -2e-9, 3e-9, None),
        ];
        for (pass, lo, hi, want) in rows {
            assert_eq!(
                pass.below(lo, hi, k),
                want,
                "{pass:?} over [{lo:e}, {hi:e}]"
            );
        }
    }

    /// The sentence the reporting margin gives a sized decision: the
    /// valued offer where a smaller tolerance decides the margin passing
    /// and the door may name one, else the lever with the caller's note,
    /// or with what an unreadable margin may mean.
    #[test]
    fn a_sized_recourse_quotes_the_value_or_names_the_lever() {
        let band = band_1e9();
        let words = |may_tighten, otherwise| SizedWords {
            lever: "L",
            size: "length",
            passes: SizedPass::Positive,
            may_tighten,
            otherwise,
        };
        let offer = "Recourse: L, or, if this length is intended, tighten the tolerance below \
                     5e-11 m";
        let rows = [
            (
                MarginDiag::value(5e-10),
                words(true, None),
                offer.to_owned(),
            ),
            (
                MarginDiag::enclosure(5e-10, 8e-10),
                words(true, None),
                offer.to_owned(),
            ),
            (
                MarginDiag::value(5e-10),
                words(false, Some("n")),
                "Recourse: L".to_owned(),
            ),
            (
                MarginDiag::value(0.0),
                words(true, Some("n")),
                "Recourse: L; n".to_owned(),
            ),
            (
                MarginDiag::value(-5e-10),
                words(true, None),
                "Recourse: L".to_owned(),
            ),
            (
                MarginDiag::enclosure(-2e-10, 3e-10),
                words(true, Some("n")),
                "Recourse: L; n".to_owned(),
            ),
            (
                MarginDiag::INVALID,
                words(true, Some("n")),
                format!("Recourse: L; {UNREADABLE_MARGIN_NOTE}"),
            ),
        ];
        for (margin, words, want) in rows {
            assert_eq!(margin.sized_recourse(band, words), want, "{margin}");
        }
    }

    /// The reading renders in the formatter's own number format, and the
    /// door hands back exactly the numbers the classifier saw.
    #[test]
    fn the_reading_renders_and_the_door_returns_it_exactly() {
        let e = MarginDiag::enclosure(-2e-9, 3e-9);
        assert_eq!(format!("{e:e}"), "[-2e-9, 3e-9]");
        assert_eq!(format!("{}", MarginDiag::value(0.5)), "0.5");
        assert_eq!(
            MarginDiag::INVALID.to_string(),
            "invalid (NaN or a poisoned enclosure)"
        );
        assert_eq!(
            e.diagnostic_f64_for_error_text(),
            ErrorTextReading::Enclosure {
                lo: -2e-9,
                hi: 3e-9
            }
        );
        assert_eq!(e.kind(), MarginKind::Enclosure);
    }

    /// **The terminal sliver is the classifier's verdict**, recorded on
    /// the escalation it mints: an enclosure wholly inside one open
    /// sliver band, on either side, is terminal; touching a threshold or
    /// straddling zero is curable; a point margin and a poisoned one are
    /// never terminal.
    #[test]
    fn the_classifier_records_whether_an_escalation_is_a_terminal_sliver() {
        use crate::Interval;
        let band = band_1e9();
        let at = |lo: f64, hi: f64| {
            Interval::hull(Interval::point(lo), Interval::point(hi))
                .sign_within(band)
                .unwrap_err()
                .terminal_sliver
        };
        let rows = [
            (2e-9, 5e-9, true),
            (-5e-9, -2e-9, true),
            (1e-9, 5e-9, false),
            (2e-9, 1e-8, false),
            (-2e-9, 5e-9, false),
        ];
        for (lo, hi, want) in rows {
            assert_eq!(at(lo, hi), want, "[{lo:e}, {hi:e}]");
        }
        assert!(
            !5e-9.sign_within(band).unwrap_err().terminal_sliver,
            "a point"
        );
        assert!(
            !f64::NAN.sign_within(band).unwrap_err().terminal_sliver,
            "poison"
        );
    }

    #[test]
    fn sign_flip_is_an_involution_and_swaps_definites() {
        assert_eq!(Sign::Negative.flip(), Sign::Positive);
        assert_eq!(Sign::Positive.flip(), Sign::Negative);
        assert_eq!(Sign::Zero.flip(), Sign::Zero);
        for s in [Sign::Negative, Sign::Zero, Sign::Positive] {
            assert_eq!(s.flip().flip(), s);
        }
    }

    #[test]
    fn sign_projections_partition() {
        // Each sign satisfies exactly one projection.
        for s in [Sign::Negative, Sign::Zero, Sign::Positive] {
            let hits = [s.is_negative(), s.is_zero(), s.is_positive()];
            assert_eq!(hits.iter().filter(|&&b| b).count(), 1, "{s:?}");
        }
        assert!(Sign::Negative.is_negative());
        assert!(Sign::Zero.is_zero());
        assert!(Sign::Positive.is_positive());
    }

    #[test]
    fn sign_display() {
        assert_eq!(Sign::Negative.to_string(), "negative");
        assert_eq!(Sign::Zero.to_string(), "zero");
        assert_eq!(Sign::Positive.to_string(), "positive");
    }

    #[test]
    fn band_new_accepts_and_exposes_thresholds() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        assert_eq!(band.zero(), 1e-9);
        assert_eq!(band.escalate(), 1e-8);
        // Extremes of validity: subnormal zero, near-MAX escalate.
        assert!(Band::new(5e-324, 1e-300).is_ok());
        assert!(Band::new(1.0, f64::MAX).is_ok());
    }

    #[test]
    fn band_new_rejects_invalid_thresholds() {
        // Non-positive / non-finite `zero`.
        for zero in [0.0, -0.0, -1e-9, f64::INFINITY, f64::NEG_INFINITY] {
            assert_eq!(
                Band::new(zero, 1e-8),
                Err(BandError::InvalidValue {
                    field: BandField::Zero,
                    value: zero,
                }),
                "zero = {zero:?}"
            );
        }
        // Non-positive / non-finite `escalate`.
        for escalate in [0.0, -1e-8, f64::INFINITY] {
            assert_eq!(
                Band::new(1e-9, escalate),
                Err(BandError::InvalidValue {
                    field: BandField::Escalate,
                    value: escalate,
                }),
                "escalate = {escalate:?}"
            );
        }
        // NaN separately (NaN != NaN defeats assert_eq on the error).
        let err = Band::new(f64::NAN, 1e-8).expect_err("NaN zero must be rejected");
        assert!(matches!(
            err,
            BandError::InvalidValue {
                field: BandField::Zero,
                value,
            } if value.is_nan()
        ));
        let err = Band::new(1e-9, f64::NAN).expect_err("NaN escalate must be rejected");
        assert!(matches!(
            err,
            BandError::InvalidValue {
                field: BandField::Escalate,
                value,
            } if value.is_nan()
        ));
        // A bad `zero` is reported first even when both are bad.
        assert_eq!(
            Band::new(-1.0, f64::INFINITY),
            Err(BandError::InvalidValue {
                field: BandField::Zero,
                value: -1.0,
            })
        );
    }

    #[test]
    fn band_new_rejects_empty_band() {
        // zero == escalate: the open band would be empty.
        assert_eq!(
            Band::new(1e-9, 1e-9),
            Err(BandError::Empty {
                zero: 1e-9,
                escalate: 1e-9,
            })
        );
        // zero > escalate: inverted.
        assert_eq!(
            Band::new(1e-8, 1e-9),
            Err(BandError::Empty {
                zero: 1e-8,
                escalate: 1e-9,
            })
        );
    }

    #[test]
    fn band_error_display() {
        assert_eq!(
            Band::new(-1e-9, 1e-8).unwrap_err().to_string(),
            "the band's zero threshold is -1e-9, and must be finite and positive. Recourse: \
             set a finite, positive tolerance, from which a derived band takes both \
             thresholds; a band built directly wants finite, positive ones"
        );
        assert_eq!(
            Band::new(1e-9, f64::INFINITY).unwrap_err().to_string(),
            "the band's escalate threshold is inf, and must be finite and positive. Recourse: \
             set a finite, positive tolerance, from which a derived band takes both \
             thresholds; a band built directly wants finite, positive ones"
        );
        assert_eq!(
            Band::new(1e-8, 1e-9).unwrap_err().to_string(),
            "the band's zero threshold 1e-8 is not below its escalate threshold 1e-9, so the \
             band is empty. Recourse: raise the escalate threshold above the zero threshold; \
             a band derived from the tolerance does, with its multiplier K > 1"
        );
        // The lever-arm variant (an invalid arm returns before the global
        // tolerance is read, so this stays pure).
        assert_eq!(
            Band::angular_at(Tol::witness(), f64::NEG_INFINITY)
                .unwrap_err()
                .to_string(),
            "the band's lever arm is -inf, and must be finite and positive. Recourse: name \
             the lever arm the decision turns on (a radius of curvature or an extent), or \
             classify a linear margin instead"
        );
    }

    /// **`BandError`'s half of the recourse claim, made enforceable.**
    /// Every arm states a condition; until this row, none of them said
    /// what to do about it, and the consumers that render a `BandError`
    /// whole — tier 3's `Band` arm is literally `"tier 3: {error}"` —
    /// contributed no recourse of their own, so the message a user read
    /// stopped at the condition.
    ///
    /// **This is a floor, not a proof**, on the same terms as
    /// `topo`'s `every_chart_region_arm_names_a_recourse`: a vocabulary
    /// check cannot tell a recourse from a sentence containing a verb,
    /// and a new arm whose recourse uses a word not on this list fails
    /// it honestly — extend the list in the same change. What it
    /// catches is the arm added with no second clause at all.
    #[test]
    fn every_band_error_arm_names_a_recourse() {
        const RECOURSE_VERBS: &[&str] = &["lower", "raise", "name", "classify", "wants"];
        let arms = [
            BandError::InvalidValue {
                field: BandField::Zero,
                value: -1e-9,
            },
            BandError::InvalidValue {
                field: BandField::Escalate,
                value: f64::INFINITY,
            },
            BandError::InvalidLeverArm { value: 0.0 },
            BandError::Empty {
                zero: 1e-8,
                escalate: 1e-9,
            },
        ];
        // Three variants; `InvalidValue` is rendered at both of its
        // fields, because the field name is interpolated into the message.
        assert_eq!(arms.len(), 4, "an arm was added without a row here");
        for arm in &arms {
            let msg = arm.to_string();
            let lower = msg.to_lowercase();
            assert!(
                RECOURSE_VERBS.iter().any(|v| lower.contains(v)),
                "no recourse in: {msg}"
            );
        }
    }

    /// The pure scaling policy the three tolerance-coupled constructors
    /// share (the constructors themselves, and both reachable arms of
    /// their `# Errors`, are tested in `tests/band_tolerance.rs`).
    #[test]
    fn from_thresholds_scales_by_ambiguity_k() {
        let band = Band::from_thresholds(2.5e-7, DEFAULT_K).unwrap();
        assert_eq!(band.zero(), 2.5e-7);
        assert_eq!(band.escalate(), DEFAULT_K * 2.5e-7);

        // The documented failure residue: a threshold within a factor K
        // of f64::MAX overflows the escalate product to infinity and is
        // rejected as a typed error, not a silently invalid band.
        assert_eq!(
            Band::from_thresholds(f64::MAX, DEFAULT_K),
            Err(BandError::InvalidValue {
                field: BandField::Escalate,
                value: f64::INFINITY,
            })
        );
    }

    /// **The door and the spelling it replaces are the same band.**
    /// `Band::linear_at(tol, eps)` is `Band::new(eps, tol.k() * eps)`,
    /// bit for bit, at whatever K the run is configured at — which is
    /// what lets the suites that open-coded the product point here
    /// without moving a threshold.
    ///
    /// Reads the committed tolerance through `Tol::witness` and never
    /// initializes it: K is read off the run rather than written here,
    /// so the row holds at every point of the CI's `CAD_TOLERANCE_EPS`
    /// matrix, and `tests/ambiguity_k_env.rs` is what exercises a K
    /// other than the default at all — it re-execs at K = 25, and
    /// nothing in `.github/` or `scripts/` varies K.
    #[test]
    fn linear_at_agrees_with_the_inline_spelling() {
        let tol = Tol::witness();
        let k = tol.k();
        for eps in [1e-12, 1e-9, 1e-6, 1.0, 2.5e-7, tol.eps()] {
            assert_eq!(
                Band::linear_at(tol, eps),
                Band::new(eps, k * eps),
                "eps = {eps:e}"
            );
            let band = Band::linear_at(tol, eps).expect("a band at a sane eps and the run's K");
            assert_eq!(band.zero(), eps);
            assert_eq!(band.escalate(), k * eps);
        }
        // At the run's own ε it IS the run's linear band — the door
        // differs from `Band::linear` in where ε comes from, nothing
        // else.
        assert_eq!(Band::linear_at(tol, tol.eps()), Band::linear(tol));

        // The ε is the CALLER's, so unlike `Band::linear` this door can
        // be handed one no tolerance would have committed — refused on
        // `zero`, and refused identically by the spelling it replaces.
        for bad in [0.0, -1.0, f64::INFINITY] {
            assert_eq!(
                Band::linear_at(tol, bad),
                Err(BandError::InvalidValue {
                    field: BandField::Zero,
                    value: bad,
                }),
                "eps = {bad:?}"
            );
            assert_eq!(Band::linear_at(tol, bad), Band::new(bad, k * bad));
        }
    }

    /// `Band::angular_at` validates the lever arm *before* it reads the
    /// global tolerance (early return), so a rejected arm never touches the
    /// global `OnceLock` — these cases are pure and safe alongside the
    /// funnel discipline. The valid-arm paths (θ = ε/r, the κ-scaled case,
    /// and the escalate-overflow residue) are global-coupled and live in
    /// `tests/band_tolerance.rs`.
    #[test]
    fn angular_at_rejects_invalid_lever_arm() {
        for arm in [0.0, -0.0, -1.0, f64::INFINITY, f64::NEG_INFINITY] {
            assert_eq!(
                Band::angular_at(Tol::witness(), arm),
                Err(BandError::InvalidLeverArm { value: arm }),
                "arm = {arm:?}"
            );
        }
        // NaN separately (NaN != NaN defeats assert_eq on the error).
        let err =
            Band::angular_at(Tol::witness(), f64::NAN).expect_err("NaN lever arm must be rejected");
        assert!(matches!(
            err,
            BandError::InvalidLeverArm { value } if value.is_nan()
        ));
    }

    /// The boundary table: every closure choice at f64, spelled out
    /// against the fixed band (zero = 1e-9, escalate = 1e-8).
    #[test]
    fn f64_boundary_table() {
        let band = band_1e9();
        // First representable values beyond each threshold.
        let above_zero = 1e-9f64.next_up();
        let below_escalate = 1e-8f64.next_down();

        let indeterminate = |m: f64| {
            Err(Indeterminate {
                margin: MarginDiag::value(m),
                band,
                predicate: None,
                terminal_sliver: false,
            })
        };

        #[rustfmt::skip]
        let table: &[(f64, Result<Sign, Indeterminate>, &str)] = &[
            // -- Coincidence region: CLOSED at |m| = zero (D4 defines
            //    coincidence as |m| <= eps, and the classifier's Zero
            //    region matches the definition exactly).
            (0.0,             Ok(Sign::Zero), "true zero"),
            (-0.0,            Ok(Sign::Zero), "negative zero — the sign of a fp zero is a representation artifact"),
            (1e-9,            Ok(Sign::Zero), "exactly +zero: boundary closed toward Zero"),
            (-1e-9,           Ok(Sign::Zero), "exactly -zero: boundary closed toward Zero"),
            (5e-324,          Ok(Sign::Zero), "minimum positive subnormal is deep inside the coincidence region"),
            (-5e-324,         Ok(Sign::Zero), "minimum negative subnormal likewise"),
            // -- Ambiguity band: the OPEN interval (zero, escalate).
            (above_zero,      indeterminate(above_zero), "first value above +zero: already indeterminate"),
            (-above_zero,     indeterminate(-above_zero), "first value below -zero: already indeterminate"),
            (5e-9,            indeterminate(5e-9), "mid-band"),
            (-5e-9,           indeterminate(-5e-9), "mid-band, negative"),
            (below_escalate,  indeterminate(below_escalate), "last value below +escalate: still indeterminate"),
            (-below_escalate, indeterminate(-below_escalate), "last value above -escalate: still indeterminate"),
            // -- Definite regions: CLOSED at |m| = escalate (a margin of
            //    exactly K*eps has the full designed clearance).
            (1e-8,            Ok(Sign::Positive), "exactly +escalate: boundary closed toward definite"),
            (-1e-8,           Ok(Sign::Negative), "exactly -escalate: boundary closed toward definite"),
            (1.0,             Ok(Sign::Positive), "far outside the band"),
            (-1.0,            Ok(Sign::Negative), "far outside the band, negative"),
            (f64::INFINITY,   Ok(Sign::Positive), "an infinite margin is maximally definite"),
            (f64::NEG_INFINITY, Ok(Sign::Negative), "likewise toward -inf"),
            // -- Poison: NaN never takes a branch.
            (f64::NAN,        Err(Indeterminate { margin: MarginDiag::INVALID, band, predicate: None, terminal_sliver: false }), "NaN margin is Invalid, not a near-miss"),
        ];

        for (margin, expected, why) in table {
            assert_eq!(
                margin.sign_within(band).map(|d| d.sign),
                *expected,
                "margin {margin:e}: {why}"
            );
        }
    }

    #[test]
    fn with_predicate_attaches_and_replaces_the_name() {
        let err = 5e-9f64
            .sign_within(band_1e9())
            .expect_err("mid-band margin must be indeterminate");
        assert_eq!(err.predicate, None);

        let named = err.with_predicate("side_of_plane");
        assert_eq!(named.predicate, Some("side_of_plane"));
        // Margin and band pass through untouched.
        assert_eq!(named.margin, err.margin);
        assert_eq!(named.band, err.band);

        // Re-attaching replaces: the outermost predicate wins.
        assert_eq!(
            named.with_predicate("transversality").predicate,
            Some("transversality")
        );
    }

    /// Golden strings: the Display output is the D4 ¶3 actionable error a
    /// user sees, so its exact wording is under test.
    #[test]
    fn indeterminate_display_golden_strings() {
        let band = band_1e9();

        let bare = 5e-9f64
            .sign_within(band)
            .expect_err("mid-band margin must be indeterminate");
        assert_eq!(
            bare.to_string(),
            format!(
                "margin 5e-9 lies inside the ambiguity band (1e-9, 1e-8) — a \
                 near-coincidence; {COINCIDENCE_RECOURSE}"
            )
        );

        let named = (-5e-9f64)
            .sign_within(band)
            .expect_err("mid-band margin must be indeterminate")
            .with_predicate("side_of_plane");
        assert_eq!(
            named.to_string(),
            format!(
                "margin -5e-9 lies inside the ambiguity band (1e-9, 1e-8) — a \
                 near-coincidence; {COINCIDENCE_RECOURSE}"
            )
        );
        // The payload view is the same message minus the shared tail —
        // what a composing site embeds next to its own recourse.
        assert_eq!(
            named.payload().to_string(),
            "margin -5e-9 lies inside the ambiguity band (1e-9, 1e-8)"
        );

        let invalid = f64::NAN
            .sign_within(band)
            .expect_err("NaN margin must be indeterminate")
            .with_predicate("transversality");
        assert_eq!(
            invalid.to_string(),
            format!(
                "margin is invalid (NaN or a poisoned enclosure) against the ambiguity \
                 band (1e-9, 1e-8) — check the operation's inputs upstream, then {COINCIDENCE_RECOURSE}"
            )
        );

        // The interval variant's wording; it is constructed here
        // directly, because the interval scalar that produces it
        // organically has its own tests.
        let enclosure = Indeterminate {
            margin: MarginDiag::enclosure(-2e-9, 5e-9),
            band,
            predicate: Some("side_of_plane"),
            terminal_sliver: false,
        };
        assert_eq!(
            enclosure.to_string(),
            format!(
                "enclosure [-2e-9, 5e-9] cannot be classified against the ambiguity \
                 band (1e-9, 1e-8) — subdivide the parameter box for a tighter enclosure, or \
                 {COINCIDENCE_RECOURSE}"
            )
        );
    }

    proptest! {
        /// Negation antisymmetry: classification commutes with negation.
        /// Definite outcomes flip; indeterminate stays indeterminate with
        /// the margin mirrored and the band unchanged. The band is
        /// randomized (zero over nine decades, ratio K' in [1.5, 100])
        /// and the margin is generated *relative to escalate* so all
        /// three regions are hit at every band scale.
        #[test]
        fn negation_antisymmetry(
            zero in 1.0e-12..1.0e-3f64,
            ratio in 1.5..100.0f64,
            t in -3.0..3.0f64,
        ) {
            let band = Band::new(zero, zero * ratio).unwrap();
            let m = t * band.escalate();
            match (m.sign_within(band).map(|d| d.sign), (-m).sign_within(band).map(|d| d.sign)) {
                (Ok(s), Ok(s_neg)) => prop_assert_eq!(s_neg, s.flip()),
                (Err(e), Err(e_neg)) => {
                    prop_assert_eq!(e.band, band);
                    prop_assert_eq!(e_neg.band, band);
                    prop_assert_eq!(e.margin, MarginDiag::value(m));
                    prop_assert_eq!(e_neg.margin, MarginDiag::value(-m));
                    prop_assert!(e.predicate.is_none() && e_neg.predicate.is_none());
                }
                (a, b) => prop_assert!(
                    false,
                    "definiteness must be symmetric under negation: \
                     sign_within({}) = {:?} but sign_within({}) = {:?}",
                    m, a, -m, b
                ),
            }
        }

        /// Monotonicity: under the classification order (`Sign`'s derived
        /// `Ord`, Negative < Zero < Positive) a larger margin never
        /// classifies strictly lower — so two definite-and-different
        /// outcomes can never invert. (Indeterminate outcomes carry no
        /// order and are skipped.)
        ///
        /// This is *implied* by `outcomes_respect_the_band`: that property
        /// pins each outcome to one of three ordered, disjoint margin
        /// regions (Negative below −escalate, Zero within ±zero, Positive
        /// above escalate), and three ordered disjoint regions cannot
        /// produce an inversion. It is kept as executable documentation of
        /// the ordering, not as independent coverage.
        #[test]
        fn classification_is_monotone(
            zero in 1.0e-12..1.0e-3f64,
            ratio in 1.5..100.0f64,
            t1 in -3.0..3.0f64,
            t2 in -3.0..3.0f64,
        ) {
            let band = Band::new(zero, zero * ratio).unwrap();
            let (lo, hi) = if t1 <= t2 { (t1, t2) } else { (t2, t1) };
            let (m_lo, m_hi) = (lo * band.escalate(), hi * band.escalate());
            if let (Ok(s_lo), Ok(s_hi)) = (m_lo.sign_within(band).map(|d| d.sign), m_hi.sign_within(band).map(|d| d.sign)) {
                prop_assert!(
                    s_lo <= s_hi,
                    "inversion: sign_within({}) = {:?} > sign_within({}) = {:?}",
                    m_lo, s_lo, m_hi, s_hi
                );
            }
        }

        /// The Zero region contains a symmetric neighborhood of 0: any
        /// |t| < 1 scaled by the zero threshold classifies as Zero from
        /// both sides. (|t| < 1 implies fl(|t|·zero) <= zero: the true
        /// product is < zero and rounding a value below zero cannot exceed
        /// zero, since zero itself is representable.) This proves one
        /// direction only — margins below the threshold are Zero. The
        /// converse (Zero *implies* |m| <= zero, so the region is no
        /// larger than the coincidence interval) is the Zero arm of
        /// `outcomes_respect_the_band`; this property does not characterize
        /// the region exactly on its own.
        #[test]
        fn zero_region_is_symmetric(
            zero in 1.0e-12..1.0e-3f64,
            ratio in 1.5..100.0f64,
            t in -1.0..1.0f64,
        ) {
            let band = Band::new(zero, zero * ratio).unwrap();
            let m = t * zero;
            prop_assert_eq!(m.sign_within(band).map(|d| d.sign), Ok(Sign::Zero));
            prop_assert_eq!((-m).sign_within(band).map(|d| d.sign), Ok(Sign::Zero));
        }

        /// Band-respecting: every outcome implies the margin's location.
        /// Definite outcomes only occur outside the open band; the
        /// indeterminate outcome only inside it, carrying the exact
        /// margin, the exact band, and no predicate name.
        #[test]
        fn outcomes_respect_the_band(
            zero in 1.0e-12..1.0e-3f64,
            ratio in 1.5..100.0f64,
            t in -3.0..3.0f64,
        ) {
            let band = Band::new(zero, zero * ratio).unwrap();
            let m = t * band.escalate();
            match m.sign_within(band).map(|d| d.sign) {
                Ok(Sign::Zero) => prop_assert!(m.abs() <= band.zero()),
                Ok(Sign::Positive) => prop_assert!(m >= band.escalate()),
                Ok(Sign::Negative) => prop_assert!(m <= -band.escalate()),
                Err(e) => {
                    prop_assert!(
                        m.abs() > band.zero() && m.abs() < band.escalate(),
                        "indeterminate for out-of-band margin {}", m
                    );
                    prop_assert_eq!(e.margin, MarginDiag::value(m));
                    prop_assert_eq!(e.band, band);
                    prop_assert!(e.predicate.is_none());
                }
            }
        }
    }
}
