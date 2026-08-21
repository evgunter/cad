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
//! `Band::angular_at(tol, r)?` — where the operation's own richer error enum can
//! absorb a [`BandError`] (a misconfigured tolerance is the operation's
//! problem to report), and then threads the `Band` down into the
//! predicates it evaluates. Predicates and the classifier take the band as
//! given and only ever return [`Indeterminate`].
//!
//! [`Band::linear`] reads the run's global ε directly. [`Band::angular_at`]
//! takes a lever arm r and derives its coincidence threshold as the angle
//! ε/r — there is deliberately **no** global angular tolerance (D4 ¶1, as
//! revised 2026-07-16): an angle's tolerance is meaningless without the
//! length scale it acts through, so every angular threshold is derived per
//! predicate from ε and the arm the decision turns on.
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
//! [`MarginDiag::Invalid`] — a poisoned computation never takes a branch
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
/// Since M2 PR 7 (Evan-directed) K is an ε-style once-per-run
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
                "invalid band: {} = {value:e} (must be finite and > 0)",
                field.name()
            ),
            Self::InvalidLeverArm { value } => write!(
                f,
                "invalid band: lever arm = {value:e} (must be finite and > 0)"
            ),
            Self::Empty { zero, escalate } => write!(
                f,
                "invalid band: zero = {zero:e} must be strictly below escalate = {escalate:e} \
                 (the ambiguity band is a nonempty open interval)"
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
/// [`Tolerance`](crate::tolerance::Tolerance); derived scales (e.g. squared-distance comparisons) go
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
    /// [`BandError`] only when K·ε is not a valid escalation threshold —
    /// i.e. the run's ε is within a factor K of `f64::MAX`, so the
    /// product overflows to infinity. Unreachable for any physically
    /// meaningful tolerance (D4 ¶4's session box is meters, ε ≈ 1e-9),
    /// but [`Tolerance`](crate::tolerance::Tolerance) only guarantees ε finite and positive, and D9
    /// makes the residue a typed error rather than a silently invalid
    /// band or a panic.
    pub fn linear(tol: Tol) -> Result<Self, BandError> {
        Self::from_zero_threshold(tol, tol.eps())
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
    /// - Otherwise the [`BandError`] from [`Band::new`]: a `lever_arm` small
    ///   enough that K·(ε/`lever_arm`) overflows to infinity surfaces as
    ///   [`BandError::InvalidValue`] on `escalate` (the same overflow
    ///   residue [`Band::linear`] documents), a typed error rather than a
    ///   silently invalid band.
    pub fn angular_at(tol: Tol, lever_arm: f64) -> Result<Self, BandError> {
        if !(lever_arm.is_finite() && lever_arm > 0.0) {
            return Err(BandError::InvalidLeverArm { value: lever_arm });
        }
        Self::from_zero_threshold(tol, tol.eps() / lever_arm)
    }

    /// The shared part of [`Band::linear`] / [`Band::angular_at`]: the
    /// band (t, K·t) for a coincidence threshold t, with K the run's
    /// configured ambiguity multiplier ([`Tol::k`]
    /// — ε-style once-per-run configuration since M2 PR 7; default
    /// [`DEFAULT_K`] = 10). The pure scaling policy is
    /// [`Band::from_thresholds`], unit-testable without the global.
    fn from_zero_threshold(tol: Tol, zero: f64) -> Result<Self, BandError> {
        Self::from_thresholds(zero, tol.k())
    }

    /// The pure scaling policy: the band (t, k·t) for a coincidence
    /// threshold t and multiplier k (the funnel-test discipline in
    /// `crate::tolerance` — testable without the global `OnceLock`).
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
    /// per parameter unit (a certified speed lower bound, a chart's
    /// `param_rate`), so the product is the model-space length the span
    /// subtends. This is the levered door's shape with a rate arm — a
    /// separate constructor because the argument is clause (iii)'s
    /// (parameter space crosses to model space only through a per-kind
    /// metric door), not a dimensionless-times-length one.
    pub fn metered(span: T, rate: T) -> Self {
        Self(span * rate)
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
    /// What this door deliberately does NOT serve (Evan's #213
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

/// Diagnostic view of the margin inside an [`Indeterminate`]: what the
/// classifier saw, in a shape that stays honest across scalar types.
///
/// Closed enum by design (the set of scalar instantiations is closed, per
/// D3's closed-enum philosophy): [`MarginDiag::Value`] is what `f64`
/// classification saw, [`MarginDiag::Enclosure`] what interval
/// classification saw (M0 PR 4), and [`MarginDiag::Invalid`] the poison
/// outcome either scalar can produce.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MarginDiag {
    /// The classified `f64` margin, signed, exactly as submitted — it
    /// landed strictly inside the ambiguity band. It is here for error
    /// messages and margin telemetry, not to be branched on: recovering
    /// the margin to make the sign decision the classifier refused defeats
    /// the escalation contract (the whole point of the typed
    /// [`Indeterminate`] is that no sound branch exists here).
    Value(f64),
    /// The classified enclosure's bounds, exactly as the interval scalar
    /// held them: the enclosure straddles a decision boundary, or lies
    /// inside the ambiguity band, so no sign is certifiable at this width.
    /// Q1's subdivision driver responds by splitting the parameter box and
    /// re-evaluating at a tighter enclosure — with one **terminal** case:
    /// an enclosure lying wholly inside one open sliver band
    /// (`zero`, `escalate`), or its mirror on the negative side, is not
    /// refinable by subdivision at all (the band is semantically
    /// indeterminate at any width, even for a point — module docs); the
    /// driver escalates it as a genuine D4 ¶3 sliver, not a resolution
    /// failure. Like [`MarginDiag::Value`], this is diagnostic data, never
    /// something to branch on.
    Enclosure {
        /// The enclosure's lower bound (−∞ for a half-unbounded one).
        lo: f64,
        /// The enclosure's upper bound (+∞ for a half-unbounded one).
        hi: f64,
    },
    /// The margin was poisoned: NaN at `f64`, or — at the interval scalar
    /// (M0 PR 4) — an empty/ill-formed enclosure or a decoration recording
    /// a domain violation somewhere in the computation (see the
    /// totality/NaN policy in [`crate::real`]). A poisoned value carries
    /// no sign information at all — this is not "too close to call", it is
    /// "the question was never validly posed". For the subdivision driver
    /// the causes differ in curability: a domain-clamp `Invalid` (a `Trv`
    /// decoration from a partially out-of-domain enclosure) may cure under
    /// subdivision as the violating sub-box shrinks away, whereas a NaI
    /// `Invalid` (ill-formed from construction) never cures.
    Invalid,
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
/// margin, both thresholds, and the predicate that was being decided),
/// enough for actionable error messages and later margin telemetry
/// without any persisted decision log (dropped per Q1).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Indeterminate {
    /// What the classifier saw: the in-band margin, or the fact that the
    /// margin was invalid.
    pub margin: MarginDiag,
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

/// Borrowed margin-payload view of an [`Indeterminate`]: the predicate
/// name, the margin/enclosure data, and the band — WITHOUT the shared
/// recourse tail. For per-site Display impls that compose the
/// two-tolerance message themselves (site context + this payload +
/// [`COINCIDENCE_RECOURSE`]) and must not double the recourse; the
/// bare [`Indeterminate`] Display is this payload plus the shared
/// tail.
#[derive(Debug, Clone, Copy)]
pub struct IndeterminatePayload<'a>(&'a Indeterminate);

impl fmt::Display for IndeterminatePayload<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.predicate {
            Some(name) => write!(f, "predicate '{name}' indeterminate: ")?,
            None => f.write_str("sign indeterminate: ")?,
        }
        let (zero, escalate) = (self.0.band.zero, self.0.band.escalate);
        match self.0.margin {
            MarginDiag::Value(m) => write!(
                f,
                "margin {m:e} lies inside the ambiguity band (zero = {zero:e}, \
                 escalate = {escalate:e})"
            ),
            MarginDiag::Enclosure { lo, hi } => write!(
                f,
                "enclosure [{lo:e}, {hi:e}] cannot be classified against the band \
                 (zero = {zero:e}, escalate = {escalate:e})"
            ),
            MarginDiag::Invalid => write!(
                f,
                "margin is invalid (NaN or a poisoned enclosure); band \
                 (zero = {zero:e}, escalate = {escalate:e})"
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
        match self.margin {
            MarginDiag::Value(_) => write!(
                f,
                " — coincident at any precision you could care about, too close \
                 to build sound geometry from; {COINCIDENCE_RECOURSE} (D4)"
            ),
            MarginDiag::Enclosure { .. } => write!(
                f,
                " — it straddles a decision boundary or lies inside the ambiguity \
                 band; subdivide the parameter box for a tighter enclosure, or \
                 {COINCIDENCE_RECOURSE} (D4)"
            ),
            // Poison explains WHY the sign is indeterminate, but the
            // user's levers at a coincidence site are unchanged — the
            // Invalid arm carries the shared recourse like the others
            // (S6 review, MINOR-1).
            MarginDiag::Invalid => write!(
                f,
                " — a poisoned computation can never take a branch; check the \
                 operation's inputs upstream, then {COINCIDENCE_RECOURSE} (D4)"
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
    /// semantics in the [module docs](self).
    ///
    /// # Errors
    ///
    /// [`Indeterminate`] when the sign cannot be certified: the margin
    /// lies strictly inside the ambiguity band, or is invalid (NaN /
    /// empty enclosure). The error carries the margin diagnostic and the
    /// band; callers attach their predicate name via
    /// [`Indeterminate::with_predicate`].
    fn sign_within(self, band: Band) -> Result<Sign, Indeterminate>;
}

/// `f64` classification: |m| ≤ `zero` ⇒ `Zero`; |m| ≥ `escalate` ⇒ the
/// sign of m; strictly between ⇒ [`Indeterminate`]; NaN ⇒
/// [`Indeterminate`] with [`MarginDiag::Invalid`].
///
/// Deterministic per D9: built from IEEE comparisons and `abs` (exact,
/// bit-identical on every conforming platform). Raw comparison is
/// allowed *inside* scalar implementations — it is generic evaluation
/// code that must not branch on values (Q1); this impl is precisely the
/// place where comparisons are turned into certified decisions.
impl Decide for f64 {
    fn sign_within(self, band: Band) -> Result<Sign, Indeterminate> {
        if self.is_nan() {
            return Err(Indeterminate {
                margin: MarginDiag::Invalid,
                band,
                predicate: None,
            });
        }
        let magnitude = self.abs();
        if magnitude <= band.zero {
            // Includes both signed zeros (|±0.0| = +0.0 ≤ zero).
            Ok(Sign::Zero)
        } else if magnitude >= band.escalate {
            // Includes ±∞: an infinite margin is maximally definite.
            // `self` is non-NaN with |self| ≥ escalate > 0 here, so it is
            // strictly one-signed.
            Ok(if self > 0.0 {
                Sign::Positive
            } else {
                Sign::Negative
            })
        } else {
            Err(Indeterminate {
                margin: MarginDiag::Value(self),
                band,
                predicate: None,
            })
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use crate::tolerance::Tol;
    use super::*;
    use proptest::prelude::*;

    // Global-state discipline (see `crate::tolerance`'s test module):
    // everything in this module is pure — bands are built with explicit
    // thresholds via `Band::new`, never via `Band::linear`/`angular_at`,
    // which would force the global `Tolerance` and race the lib test
    // binary's single designated global-touching test. The tolerance-
    // coupled constructors are covered in their own integration-test
    // binary (`tests/band_tolerance.rs`), i.e. their own process.
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
            "invalid band: zero = -1e-9 (must be finite and > 0)"
        );
        assert_eq!(
            Band::new(1e-9, f64::INFINITY).unwrap_err().to_string(),
            "invalid band: escalate = inf (must be finite and > 0)"
        );
        assert_eq!(
            Band::new(1e-8, 1e-9).unwrap_err().to_string(),
            "invalid band: zero = 1e-8 must be strictly below escalate = 1e-9 \
             (the ambiguity band is a nonempty open interval)"
        );
        // The lever-arm variant (an invalid arm returns before the global
        // tolerance is read, so this stays pure).
        assert_eq!(
            Band::angular_at(Tol::witness(), f64::NEG_INFINITY).unwrap_err().to_string(),
            "invalid band: lever arm = -inf (must be finite and > 0)"
        );
    }

    /// The pure scaling policy behind `Band::linear`/`angular_at` (the
    /// global-coupled wrappers are tested in `tests/band_tolerance.rs`).
    #[test]
    fn from_zero_threshold_scales_by_ambiguity_k() {
        let band = Band::from_thresholds(2.5e-7, DEFAULT_K).unwrap();
        assert_eq!(band.zero(), 2.5e-7);
        assert_eq!(band.escalate(), DEFAULT_K * 2.5e-7);

        // The documented failure residue: a threshold within a factor K
        // of f64::MAX overflows the escalate product to infinity and is
        // rejected as a typed error, not a silently invalid band.
        assert_eq!(
            Band::from_zero_threshold(Tol::witness(), f64::MAX),
            Err(BandError::InvalidValue {
                field: BandField::Escalate,
                value: f64::INFINITY,
            })
        );
    }

    /// `Band::angular_at` validates the lever arm *before* it reads the
    /// global tolerance (early return), so a rejected arm never touches the
    /// global `OnceLock` — these ca(Tol::witness(), f64::MAX)re and safe alongside the
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
        let err = Band::angular_at(Tol::witness(), f64::NAN).expect_err("NaN lever arm must be rejected");
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
                margin: MarginDiag::Value(m),
                band,
                predicate: None,
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
            (f64::NAN,        Err(Indeterminate { margin: MarginDiag::Invalid, band, predicate: None }), "NaN margin is Invalid, not a near-miss"),
        ];

        for (margin, expected, why) in table {
            assert_eq!(
                margin.sign_within(band),
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
                "sign indeterminate: margin 5e-9 lies inside the ambiguity band \
                 (zero = 1e-9, escalate = 1e-8) — coincident at any precision \
                 you could care about, too close to build sound geometry from; \
                 {COINCIDENCE_RECOURSE} (D4)"
            )
        );

        let named = (-5e-9f64)
            .sign_within(band)
            .expect_err("mid-band margin must be indeterminate")
            .with_predicate("side_of_plane");
        assert_eq!(
            named.to_string(),
            format!(
                "predicate 'side_of_plane' indeterminate: margin -5e-9 lies inside \
                 the ambiguity band (zero = 1e-9, escalate = 1e-8) — coincident at \
                 any precision you could care about, too close to build sound \
                 geometry from; {COINCIDENCE_RECOURSE} (D4)"
            )
        );
        // The payload view is the same message minus the shared tail —
        // what a composing site embeds next to its own recourse.
        assert_eq!(
            named.payload().to_string(),
            "predicate 'side_of_plane' indeterminate: margin -5e-9 lies inside \
             the ambiguity band (zero = 1e-9, escalate = 1e-8)"
        );

        let invalid = f64::NAN
            .sign_within(band)
            .expect_err("NaN margin must be indeterminate")
            .with_predicate("transversality");
        assert_eq!(
            invalid.to_string(),
            format!(
                "predicate 'transversality' indeterminate: margin is invalid (NaN \
                 or a poisoned enclosure); band (zero = 1e-9, escalate = 1e-8) — a \
                 poisoned computation can never take a branch; check the \
                 operation's inputs upstream, then {COINCIDENCE_RECOURSE} (D4)"
            )
        );

        // The interval variant's wording (the variant itself is not
        // feature-gated — it is constructed here directly; the interval
        // scalar that produces it organically lives behind the `interval`
        // feature and has its own tests).
        let enclosure = Indeterminate {
            margin: MarginDiag::Enclosure {
                lo: -2e-9,
                hi: 5e-9,
            },
            band,
            predicate: Some("side_of_plane"),
        };
        assert_eq!(
            enclosure.to_string(),
            format!(
                "predicate 'side_of_plane' indeterminate: enclosure [-2e-9, 5e-9] \
                 cannot be classified against the band (zero = 1e-9, escalate = 1e-8) \
                 — it straddles a decision boundary or lies inside the ambiguity \
                 band; subdivide the parameter box for a tighter enclosure, or \
                 {COINCIDENCE_RECOURSE} (D4)"
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
            match (m.sign_within(band), (-m).sign_within(band)) {
                (Ok(s), Ok(s_neg)) => prop_assert_eq!(s_neg, s.flip()),
                (Err(e), Err(e_neg)) => {
                    prop_assert_eq!(e.band, band);
                    prop_assert_eq!(e_neg.band, band);
                    prop_assert_eq!(e.margin, MarginDiag::Value(m));
                    prop_assert_eq!(e_neg.margin, MarginDiag::Value(-m));
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
            if let (Ok(s_lo), Ok(s_hi)) = (m_lo.sign_within(band), m_hi.sign_within(band)) {
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
            prop_assert_eq!(m.sign_within(band), Ok(Sign::Zero));
            prop_assert_eq!((-m).sign_within(band), Ok(Sign::Zero));
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
            match m.sign_within(band) {
                Ok(Sign::Zero) => prop_assert!(m.abs() <= band.zero()),
                Ok(Sign::Positive) => prop_assert!(m >= band.escalate()),
                Ok(Sign::Negative) => prop_assert!(m <= -band.escalate()),
                Err(e) => {
                    prop_assert!(
                        m.abs() > band.zero() && m.abs() < band.escalate(),
                        "indeterminate for out-of-band margin {}", m
                    );
                    prop_assert_eq!(e.margin, MarginDiag::Value(m));
                    prop_assert_eq!(e.band, band);
                    prop_assert!(e.predicate.is_none());
                }
            }
        }
    }
}
