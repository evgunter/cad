//! The single global [`Tolerance`] value (D4 ¶1 in `docs/DESIGN.md`, as
//! revised 2026-07-16).
//!
//! One tolerance ε per run, shared by all bodies, never loosened mid-run —
//! per-model tolerances are deliberately rejected (any two bodies must be
//! boolean-combinable). D4 leaves "compile-time constant vs.
//! once-initialized" open as an implementation detail; this module chooses
//! **once-initialized** (a [`std::sync::OnceLock`]), which is what lets the
//! test suite run at several ε values (the multi-ε CI matrix) with zero
//! test-code cooperation.
//!
//! # One number, not two
//!
//! There is exactly one global tolerance: the linear ε. There is
//! deliberately **no** second global angular tolerance εₐ (D4 ¶1 as
//! revised 2026-07-16). An angle only acquires meaning through the
//! displacement d = r·θ it induces at some lever arm r, so a fixed εₐ
//! would silently privilege the hidden length scale ε/εₐ. Angular
//! thresholds are therefore never global — each predicate derives its own
//! as θ = ε/r, with the lever arm r named at the call site (see
//! `Band::angular_at` in `crate::predicate`). This module owns only ε.
//!
//! # Initialization protocol
//!
//! - [`Tolerance::init`] installs an explicit value, exactly once, with
//!   typed errors ([`ToleranceError`]) for invalid values or double
//!   initialization. There is no API to change the value afterwards —
//!   "never loosened mid-run" is structural.
//! - [`Tol::witness`] is **infallible and total**. On first use without a
//!   prior `init` it self-initializes from the environment ([`ENV_EPS`]):
//!   a well-formed value wins, an absent variable falls back to the
//!   compiled default, and a malformed or invalid value falls back to the
//!   default **while recording the failure**. The rejected variable is
//!   recorded, retrievable via [`Tolerance::env_init_errors`]. Loudness is
//!   restored structurally: a regular `#[test]` in this crate asserts the
//!   recorded slice is empty, so a typo'd ε in a CI matrix row fails
//!   visibly through the normal test mechanism (no library panic per D9,
//!   no silent config swallow per D4).
//! - [`Tolerance::init_document_eps`] commits a LOADED DOCUMENT's
//!   recorded ε. It outranks the env variable by being the first
//!   toucher of the lock — the env channel is process *bootstrap*, a
//!   document *states* its ε — and a disagreement with an
//!   already-committed value is the persistence layer's
//!   `ToleranceConflict` refusal, which commits nothing.
//!
//! # ε provenance (S22, ruled 2026-08-19)
//!
//! ε is a **declared run parameter**, not an implementation detail: the
//! model is a pure function of (parameter vector, ε). The `OnceLock` is
//! what makes "one ε per process" structural rather than documentary,
//! and it is kept for that reason — see `docs/SMELL-SCAN-2026-08.md`
//! S22. What it lacked was a way to say *where the committed value came
//! from*, which is why a stale `CAD_TOLERANCE_EPS` in a shell could
//! change what "coincident" means with no output line saying so
//! (issues #415, #497).
//!
//! [`EpsilonSource`] closes that: every commitment path records its
//! channel, [`Tolerance::eps_source`] reads it back, and
//! [`Tolerance::report`] / [`Tolerance::committed_report`] render the
//! committed value, its provenance, and any rejected env value as one
//! line for a run to print. **This is a channel, not a decision**: no
//! ranking, no refusal, and no predicate branches on it, and nothing in
//! the kernel reads `eps_source` at all.

use core::fmt;
use std::sync::OnceLock;

/// Environment variable consulted for ε on `get()` self-initialization
/// (name fixed by CI's multi-ε matrix; M0 L3).
pub const ENV_EPS: &str = "CAD_TOLERANCE_EPS";

/// Compiled default for ε: 1e-9 m (ratified, D4 ¶1) — micron-to-kilometer
/// coverage with ~4 orders of f64 headroom at km scale.
pub const DEFAULT_EPS: f64 = 1e-9;

/// Environment variable consulted for the ambiguity multiplier K on
/// `get()` self-initialization (Evan-directed at M2 PR 7: K joins ε as
/// an ε-style once-per-run configured value; previously a hard const).
pub const ENV_K: &str = "CAD_AMBIGUITY_K";

/// Compiled default for the ambiguity multiplier K = 10 (the ratified
/// M0 starting value; the M2 K report found no empirical pressure to
/// move it — `docs/K-REPORT.md`).
///
/// **The measurement behind that sentence is re-run on every build, and
/// gates** (issue #667's Q6 classification — a scheduled register, not
/// an excuse): `ci.yml`'s `k-lint (gate)` job runs
/// `scripts/k_probe_sweep.sh` over the Band-4 corpus and the demo tour
/// at ε ∈ {1e-6, 1e-9, 1e-12}, then lints the fresh margin distribution
/// against the thresholds and baseline pinned in `tools/k-lint`
/// (provenance: `tools/k-lint/src/lib.rs`). A margin that starts
/// crowding a decision boundary the committed baseline says is empty
/// FAILS the run. So "no empirical pressure to move K" is not a dated
/// observation this constant inherited — it is a claim something
/// re-measures per merge, and the honest reading of a fired lint is
/// evidence about the distribution, never a cue to nudge geometry until
/// it goes quiet (`tools/k-lint`'s own failure text says so).
///
/// **How much of that lint actually moves when K moves** — settled here
/// because it is not obvious from the tree, and a register credited
/// wider than it reads is the defect #667 was hunting. `escalate` is
/// K·`zero`, so:
///
/// * rule (1) (in-band outcomes) tracks K at **all three** ε rows — the
///   band `(ε, Kε)` is the kernel's own, so moving K directly moves
///   which samples come back `indeterminate`;
/// * rule (2)-above is `min(10²·Kε, BASELINE_FLOOR_MARGIN)`, so it
///   tracks K only while the cap is inert. At ε = 1e-9 and 1e-12 it is
///   (the cap would need K ≥ 400 / K ≥ 4e5); **at ε = 1e-6 the cap
///   already binds at the ratified K = 10** and keeps binding for any
///   K ≥ 0.4, so that row says nothing about K at all;
/// * rules (2)-below, (3) and (4) key off `zero` = ε and
///   `BASELINE_FLOOR_MARGIN`, and do not read K.
///
/// So the register is real and gating, and it is narrower than "the
/// distribution K was chosen against": it is K-sensitive at three ε
/// rows through rule (1) and at two of them through rule (2).
pub const DEFAULT_K: f64 = 10.0;

/// The kernel's global tolerance (D4 ¶1, as revised 2026-07-16): one ε per
/// run, shared by all bodies, never loosened mid-run.
///
/// A `struct` with a single field rather than a bare newtype: the extra
/// wrapping is deliberate room to grow — future run-global tolerance
/// configuration (should the M0 experiments call for any) extends this
/// struct without changing every call site. Angular thresholds are **not**
/// a candidate field: they are derived per predicate from ε and a named
/// lever arm (θ = ε/r), never carried globally (see the [module
/// docs](self)).
///
/// `eps` is `f64` regardless of the scalar type `T` in use — it bounds
/// *residuals and margins*, which are plain numbers even when the geometry
/// is evaluated at intervals or dual numbers. Units are the kernel's fixed
/// internal units (D4 ¶4): meters.
///
/// A `Tolerance` value is valid when `eps` is finite and strictly
/// positive; [`Tolerance::init`] enforces this (that documented range *is*
/// the sane range — no further restriction).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tolerance {
    /// Linear tolerance ε, in meters.
    pub eps: f64,
    /// The ambiguity multiplier K (dimensionless, > 1): a band's
    /// `escalate` threshold is K times its `zero` threshold (K·ε for
    /// linear bands, K·(ε/r) for angular ones). One value per run,
    /// never changed after commitment — exactly ε's invariant (D4 ¶1).
    /// Configured like ε: [`Tolerance::init`] or the [`ENV_K`] env var
    /// on first `get()`, defaulting to [`DEFAULT_K`] = 10 (Evan-directed
    /// at M2 PR 7; previously the hard `AMBIGUITY_K` const). Like ε, K
    /// is expected to become per-model persisted configuration with a
    /// recorded change operation in the document layer (the banked
    /// SetTolerance/change-ε principle extends to K) — future work,
    /// noted here only.
    pub k: f64,
}

/// Typed error from [`Tolerance::init`] (D9: every failure is a typed
/// error, never a panic).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToleranceError {
    /// The attempted `Tolerance`'s `eps` is not finite and strictly
    /// positive.
    InvalidValue {
        /// The rejected value.
        value: f64,
    },
    /// The attempted `Tolerance`'s `k` is not finite and strictly
    /// greater than 1 (K ≤ 1 would make every band's open ambiguity
    /// interval (ε, K·ε) empty over the reals — rejected here for the
    /// same reason [`crate::predicate::BandError::Empty`] rejects it
    /// per band).
    InvalidK {
        /// The rejected value.
        value: f64,
    },
    /// The global tolerance was already initialized (by an earlier `init`
    /// or by `get()`'s env self-initialization) and cannot change mid-run.
    AlreadyInitialized {
        /// The tolerance the run is already committed to.
        current: Tolerance,
        /// The value this rejected `init` call attempted to install.
        attempted: Tolerance,
    },
}

impl fmt::Display for ToleranceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidValue { value } => write!(
                f,
                "invalid tolerance: eps = {value:e} (must be finite and > 0)"
            ),
            Self::InvalidK { value } => write!(
                f,
                "invalid tolerance: k = {value:e} (must be finite and > 1)"
            ),
            Self::AlreadyInitialized { current, attempted } => write!(
                f,
                "tolerance already initialized to eps = {:e}, k = {}; attempted \
                 eps = {:e}, k = {} — one value per run (D4)",
                current.eps, current.k, attempted.eps, attempted.k
            ),
        }
    }
}

impl std::error::Error for ToleranceError {}

/// Why a tolerance environment variable was rejected.
#[derive(Debug, Clone, PartialEq)]
pub enum ToleranceEnvErrorKind {
    /// The value did not parse as an `f64` at all.
    Unparsable,
    /// The value parsed but is not finite and strictly positive.
    Invalid {
        /// The parsed (rejected) value.
        value: f64,
    },
}

/// A recorded failure from `get()`'s env self-initialization.
///
/// This is *recorded*, not raised — [`Tol::witness`] stays total and
/// falls back to the compiled default; retrieve it via
/// [`Tolerance::env_init_errors`]. A test in this crate asserts the
/// recorded slice is empty, which is what makes a malformed CI env value
/// fail loudly.
#[derive(Debug, Clone, PartialEq)]
pub struct ToleranceEnvError {
    /// The environment variable that was rejected.
    pub var: &'static str,
    /// Its raw string value (non-Unicode values appear lossily).
    pub raw: String,
    /// Why it was rejected.
    pub kind: ToleranceEnvErrorKind,
}

impl fmt::Display for ToleranceEnvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ToleranceEnvErrorKind::Unparsable => write!(
                f,
                "tolerance env var {} = {:?} does not parse as a number; \
                 the compiled default was used instead",
                self.var, self.raw
            ),
            ToleranceEnvErrorKind::Invalid { value } => write!(
                f,
                "tolerance env var {} = {:?} (= {value:e}) must be finite and > 0; \
                 the compiled default was used instead",
                self.var, self.raw
            ),
        }
    }
}

impl std::error::Error for ToleranceEnvError {}

/// Which channel supplied the run's committed ε (S22, ruled
/// 2026-08-19).
///
/// Purely a record: the commitment ORDER and the refusals are exactly
/// as they were before this type existed (a document's ε outranks the
/// env bootstrap by committing first; a disagreement refuses at the
/// persistence layer). Nothing in the kernel branches on this value —
/// it exists so a run can *state* the ε it is deciding at.
///
/// K's provenance is deliberately NOT tracked: K is resolved from
/// [`ENV_K`] on every commitment path including the document one, so
/// there is only ever one answer for it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EpsilonSource {
    /// [`DEFAULT_EPS`], the compiled default — nothing supplied a
    /// value.
    ///
    /// Also the answer when [`ENV_EPS`] was present but **rejected**
    /// (unparsable, or not finite and > 0): a rejected value never
    /// reaches the tolerance, so the compiled default is what the run
    /// is actually deciding at. The rejection itself rides
    /// [`Tolerance::env_init_errors`] and is printed by
    /// [`ToleranceReport`].
    Default,
    /// The process environment's [`ENV_EPS`], parsed and accepted.
    ///
    /// The env channel is process **bootstrap**. This variant means
    /// nothing more authoritative overrode it — in particular, no
    /// document stated an ε before the lock was touched.
    Env,
    /// An explicit [`Tolerance::init`] — the embedding program stated
    /// ε in code. This path never consults the environment.
    Init,
    /// A loaded document's recorded ε, via
    /// [`Tolerance::init_document_eps`] — the document was the first
    /// toucher of the lock, so its value is what the run decides at,
    /// whether or not [`ENV_EPS`] was also set (it was never read).
    ///
    /// A document that *agrees* with an already-committed value is a
    /// benign reload and does NOT change the provenance: the earlier
    /// channel is still the one that supplied the number.
    Document,
}

impl fmt::Display for EpsilonSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default => write!(f, "the compiled default"),
            Self::Env => write!(f, "the environment ({ENV_EPS})"),
            Self::Init => write!(f, "an explicit Tolerance::init"),
            Self::Document => write!(f, "a loaded document"),
        }
    }
}

/// One line stating what the run committed to and where it came from:
/// ε, its [`EpsilonSource`], K, and any env value that was rejected on
/// the way.
///
/// Built by [`Tolerance::report`] (which commits the ambient bootstrap
/// if nothing has, exactly as [`Tol::witness`] does) or by
/// [`Tolerance::committed_report`] (which never commits, so a program
/// may report at any point without *deciding* ε by asking).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ToleranceReport {
    /// The tolerance the run is committed to.
    pub tolerance: Tolerance,
    /// Which channel supplied `tolerance.eps`.
    pub eps_source: EpsilonSource,
    /// Every env value rejected on the way to the commitment — the
    /// same slice [`Tolerance::env_init_errors`] returns.
    pub env_errors: &'static [ToleranceEnvError],
}

impl fmt::Display for ToleranceReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "tolerance: eps = {:e} m from {}, K = {}",
            self.tolerance.eps, self.eps_source, self.tolerance.k
        )?;
        for e in self.env_errors {
            write!(f, "; REJECTED: {e}")?;
        }
        Ok(())
    }
}

/// The once-per-run global state: the committed tolerance plus, when the
/// commitment came from `get()`'s env path, every recorded env failure
/// (empty when nothing was rejected).
///
/// `env_errors` holds **at most one entry per tolerance env var** — at
/// most two at present ([`ENV_EPS`] and [`ENV_K`]). The `Vec` shape is a
/// stable surface for future run-global config vars, so adding one never
/// changes [`Tolerance::env_init_errors`]'s signature.
#[derive(Debug)]
struct Global {
    tolerance: Tolerance,
    env_errors: Vec<ToleranceEnvError>,
    /// Which channel supplied `tolerance.eps` — written by whichever
    /// path won the `get_or_init` race, read by nothing in the kernel.
    eps_source: EpsilonSource,
}

static GLOBAL: OnceLock<Global> = OnceLock::new();

/// The process-environment lookup used by `get()`'s self-initialization.
/// Non-Unicode values are read lossily so they surface as a recorded parse
/// error rather than silently reading as absent (fail loud, D4).
fn env_lookup(key: &str) -> Option<String> {
    std::env::var_os(key).map(|v| v.to_string_lossy().into_owned())
}

fn global() -> &'static Global {
    GLOBAL.get_or_init(|| {
        let (tolerance, env_errors) = resolve_from_env(env_lookup);
        Global {
            eps_source: env_eps_source(env_lookup, &env_errors),
            tolerance,
            env_errors,
        }
    })
}

/// Which channel supplied the ε that `resolve_from_env` just returned:
/// [`EpsilonSource::Env`] when [`ENV_EPS`] was present AND accepted,
/// [`EpsilonSource::Default`] otherwise — absent, or present and
/// rejected (a rejected value falls back to [`DEFAULT_EPS`] and is
/// recorded, so the run really is deciding at the compiled default).
///
/// Split out rather than folded into `resolve_from_env` so the env
/// policy above keeps its shape and its tests: this reads the same
/// injectable lookup and the errors that policy produced.
fn env_eps_source(
    lookup: impl Fn(&str) -> Option<String>,
    env_errors: &[ToleranceEnvError],
) -> EpsilonSource {
    let rejected = env_errors.iter().any(|e| e.var == ENV_EPS);
    if !rejected && lookup(ENV_EPS).is_some() {
        EpsilonSource::Env
    } else {
        EpsilonSource::Default
    }
}

/// Resolves one tolerance field from the environment: absent ⇒ default,
/// well-formed ⇒ the env value, malformed/invalid ⇒ default plus a
/// recorded error. The raw string is parsed as-is (no trimming): a value
/// that needs cleanup is a config anomaly worth surfacing.
fn resolve_var(
    lookup: &impl Fn(&str) -> Option<String>,
    var: &'static str,
    default: f64,
    valid: impl Fn(f64) -> bool,
) -> (f64, Option<ToleranceEnvError>) {
    let Some(raw) = lookup(var) else {
        return (default, None);
    };
    match raw.parse::<f64>() {
        Ok(value) if value.is_finite() && valid(value) => (value, None),
        Ok(value) => (
            default,
            Some(ToleranceEnvError {
                var,
                raw,
                kind: ToleranceEnvErrorKind::Invalid { value },
            }),
        ),
        Err(_) => (
            default,
            Some(ToleranceEnvError {
                var,
                raw,
                kind: ToleranceEnvErrorKind::Unparsable,
            }),
        ),
    }
}

/// Pure resolution of a [`Tolerance`] from an injectable environment
/// lookup — the whole env policy lives here, unit-testable without
/// touching the process environment or the global `OnceLock`; the
/// `OnceLock` wrapper above is a thin shell over this.
///
/// ε ([`ENV_EPS`], finite and > 0) and K ([`ENV_K`], finite and > 1)
/// are consulted; the returned `Vec` holds at most one error per var.
fn resolve_from_env(
    lookup: impl Fn(&str) -> Option<String>,
) -> (Tolerance, Vec<ToleranceEnvError>) {
    let (eps, eps_err) = resolve_var(&lookup, ENV_EPS, DEFAULT_EPS, |v| v > 0.0);
    let (k, k_err) = resolve_var(&lookup, ENV_K, DEFAULT_K, |v| v > 1.0);
    let env_errors = eps_err.into_iter().chain(k_err).collect();
    (Tolerance { eps, k }, env_errors)
}

impl Tolerance {
    /// A tolerance with the given ε and the default K
    /// ([`DEFAULT_K`]) — the pre-PR 7 construction shape, kept as the
    /// ergonomic constructor for callers who only care about ε.
    pub fn with_eps(eps: f64) -> Self {
        Self { eps, k: DEFAULT_K }
    }

    /// Validates that `eps` is finite and strictly positive and `k`
    /// finite and strictly greater than 1.
    fn validate(self) -> Result<(), ToleranceError> {
        if !(self.eps.is_finite() && self.eps > 0.0) {
            return Err(ToleranceError::InvalidValue { value: self.eps });
        }
        if !(self.k.is_finite() && self.k > 1.0) {
            return Err(ToleranceError::InvalidK { value: self.k });
        }
        Ok(())
    }

    /// Installs `tolerance` as the run's global tolerance, exactly once.
    ///
    /// # Errors
    ///
    /// - [`ToleranceError::InvalidValue`] if `eps` is not finite and
    ///   strictly positive (checked before touching the global state, so a
    ///   rejected call leaves initialization for later).
    /// - [`ToleranceError::AlreadyInitialized`] if the global tolerance was
    ///   already committed — by an earlier `init` or by [`Tol::witness`]'s
    ///   env self-initialization — carrying both the current and the
    ///   attempted value. There is no way to change the tolerance after
    ///   commitment (D4 ¶1: never loosened mid-run).
    ///
    /// When `init` is used it should be called during single-threaded
    /// startup, before any [`Tol::witness`]; racing `init` against a first
    /// `get` is safe (a single commit wins, the loser gets a typed error),
    /// but which value wins is scheduling-dependent.
    pub fn init(tolerance: Tolerance) -> Result<(), ToleranceError> {
        tolerance.validate()?;
        let mut installed = false;
        let global = GLOBAL.get_or_init(|| {
            installed = true;
            Global {
                tolerance,
                env_errors: Vec::new(),
                eps_source: EpsilonSource::Init,
            }
        });
        if installed {
            Ok(())
        } else {
            Err(ToleranceError::AlreadyInitialized {
                current: global.tolerance,
                attempted: tolerance,
            })
        }
    }

    /// Commits a loaded DOCUMENT's recorded ε as the run's global
    /// tolerance (M4 PR 6 spec D4): ε comes from the document (the
    /// recorded value wins over an unread [`ENV_EPS`] — the ambient
    /// env mechanism is process BOOTSTRAP, and a document that states
    /// its ε outranks it), while K still resolves from the
    /// environment ([`ENV_K`], with any malformed value recorded
    /// exactly as `get()` records it).
    ///
    /// # Errors
    ///
    /// - [`ToleranceError::InvalidValue`] if `eps` is not finite and
    ///   strictly positive (checked before touching the global).
    /// - [`ToleranceError::AlreadyInitialized`] if the run's
    ///   tolerance was already committed. The CALLER decides whether
    ///   the committed ε matches the document (bit compare) — a match
    ///   is a benign reload, a mismatch is the one-process-one-ε
    ///   refusal (the persistence layer's `ToleranceConflict`).
    pub fn init_document_eps(eps: f64) -> Result<(), ToleranceError> {
        let (k, k_err) = resolve_var(&env_lookup, ENV_K, DEFAULT_K, |v| v > 1.0);
        let tolerance = Tolerance { eps, k };
        tolerance.validate()?;
        let mut installed = false;
        let global = GLOBAL.get_or_init(|| {
            installed = true;
            Global {
                tolerance,
                env_errors: k_err.into_iter().collect(),
                eps_source: EpsilonSource::Document,
            }
        });
        if installed {
            Ok(())
        } else {
            Err(ToleranceError::AlreadyInitialized {
                current: global.tolerance,
                attempted: tolerance,
            })
        }
    }

    /// Every failure recorded by `get()`'s env self-initialization.
    ///
    /// At most one entry per tolerance env var — at most two at present
    /// ([`ENV_EPS`] and [`ENV_K`]); the slice shape is kept stable for
    /// future run-global config vars.
    ///
    /// Forces initialization (as if by [`Tol::witness`]) if it has not
    /// happened yet, so the answer is definitive. An empty slice means the
    /// committed tolerance involved no rejected env value — in particular,
    /// initialization via [`Tolerance::init`] never consults the
    /// environment and records nothing.
    ///
    /// The crate's test suite asserts this is empty, which is what makes
    /// a typo'd tolerance in a CI matrix row fail loudly (through the
    /// normal test mechanism rather than a library panic, per D9).
    pub fn env_init_errors() -> &'static [ToleranceEnvError] {
        &global().env_errors
    }

    /// Which channel supplied the run's committed ε (S22, ruled
    /// 2026-08-19): the environment, an explicit [`Tolerance::init`],
    /// a loaded document, or the compiled default.
    ///
    /// Forces initialization (as if by [`Tol::witness`]) if it has
    /// not happened yet, so the answer is definitive — use
    /// [`Tolerance::committed_report`] to ask without committing.
    ///
    /// Reporting only. No ranking, no refusal, and no predicate reads
    /// this.
    pub fn eps_source() -> EpsilonSource {
        global().eps_source
    }

    /// The run's tolerance and the provenance of its ε, as one
    /// printable line.
    ///
    /// Forces initialization exactly as [`Tol::witness`] does — so a
    /// program that may later load a document should report through
    /// [`Tolerance::committed_report`] instead, which never decides ε
    /// by asking about it.
    pub fn report() -> ToleranceReport {
        let g = global();
        ToleranceReport {
            tolerance: g.tolerance,
            eps_source: g.eps_source,
            env_errors: &g.env_errors,
        }
    }

    /// The same report, or `None` when the run has not committed an ε
    /// yet.
    ///
    /// The **non-committing** door: unlike every other accessor here it
    /// does not `get_or_init`, so printing it at the top of a program
    /// cannot pre-empt a document that is about to state its own ε (an
    /// ambient bootstrap committed by the act of reporting would turn
    /// every subsequent load into a `ToleranceConflict`). Report at a
    /// point where the answer is interesting, and `None` truthfully
    /// means "nothing has decided yet".
    pub fn committed_report() -> Option<ToleranceReport> {
        GLOBAL.get().map(|g| ToleranceReport {
            tolerance: g.tolerance,
            eps_source: g.eps_source,
            env_errors: &g.env_errors,
        })
    }
}

/// Evidence that the run's tolerance is committed, and the kernel's only
/// door to reading it (D4 ¶1, "the witness, not the value" — ratified
/// 2026-08-21).
///
/// # What it is for
///
/// ε has to be four things at once, and before this type the fourth
/// fought the other three: **runtime-configurable**, **immutable**, **one
/// source of truth**, and **named in the signature of everything that
/// depends on it**. The [`OnceLock`] gives the first three and forfeits
/// the fourth. Threading a [`Tolerance`] gives the fourth and erodes the
/// third — [`Tolerance`]'s fields are public, so `tol: Tolerance` says
/// "decides against *a* tolerance", which is exactly true and exactly the
/// problem (S22 row 1's 2026-08-19 ruling rejected threading on that
/// ground, and `profile`'s 256 call sites all passing the same global are
/// the evidence it rested on).
///
/// `Tol` resolves it by carrying **evidence instead of the value**. It is
/// zero-sized with a private field, and [`Tol::witness`] — which commits
/// the ambient bootstrap exactly as the removed `Tolerance::get` did — is
/// its only constructor. So the type has exactly **one inhabitant**: two
/// `Tol` values are the same `Tol`, and a second ε is not a discipline
/// problem but something the type cannot express. The value never leaves
/// the `OnceLock`; passing a `Tol` passes the right to read it.
///
/// A `tol: Tol` parameter therefore says something a value parameter
/// cannot: not "takes a tolerance" but *"decides against the run's ε,
/// which the caller has already committed"*. The ε-**free** functions —
/// the exact predicates, the combinatorial layer — are identifiable by
/// the absence of the parameter, which is the half of the signal that
/// carries information.
///
/// # Cost
///
/// None. A zero-sized argument compiles away, and [`Tol::get`] is the
/// same acquire-load on an initialized [`OnceLock`] that every ambient
/// read was.
///
/// # Obtaining one
///
/// [`Tol::witness`] is confined by CI gate to the `pncad` door, binaries
/// and tests: kernel `src` receives a `Tol` from its caller and never
/// mints one, which is what keeps an ambient read from quietly returning.
/// Because witnessing **commits** ε, a program that may later load a
/// document should not witness before the load — see
/// [`Tolerance::committed_report`], the non-committing door, for the same
/// hazard in its reporting form.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tol(());

impl Tol {
    /// Commits the run's tolerance if nothing has yet, and returns the
    /// witness.
    ///
    /// Committing is exactly what the removed `Tolerance::get` did: with
    /// no prior [`Tolerance::init`] this self-initializes from the
    /// environment ([`ENV_EPS`]); an absent variable falls back to
    /// [`DEFAULT_EPS`], and a malformed or invalid value falls back to
    /// the default while recording the failure (see
    /// [`Tolerance::env_init_errors`]). Infallible and total.
    ///
    /// Call it at a program's entry point, or in a test — **not** inside
    /// kernel `src`, where the parameter is the point (see the [type
    /// docs](Tol)).
    pub fn witness() -> Self {
        let _ = global();
        Self(())
    }

    /// The run's committed tolerance — ε and K.
    pub fn get(self) -> Tolerance {
        global().tolerance
    }

    /// The run's committed linear tolerance ε, in meters.
    pub fn eps(self) -> f64 {
        self.get().eps
    }

    /// The run's committed ambiguity multiplier K.
    pub fn k(self) -> f64 {
        self.get().k
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use crate::tolerance::Tol;
    use super::*;

    // Process-global state discipline: tests share one process, so
    // everything here goes through the *pure* `resolve_from_env` /
    // `validate` paths — except the single test at the bottom, which is the
    // only test in this binary allowed to touch the global `OnceLock`
    // (`get` / `env_init_errors` / a necessarily-failing late `init`). The
    // successful-`init` path lives in its own integration-test binary
    // (`tests/tolerance_init.rs`), i.e. its own process.

    /// An injectable lookup over a fixed table (no process env involved).
    fn table<'a>(pairs: &'a [(&'a str, &'a str)]) -> impl Fn(&str) -> Option<String> + 'a {
        move |key| {
            pairs
                .iter()
                .find(|(k, _)| *k == key)
                .map(|(_, v)| (*v).to_string())
        }
    }

    #[test]
    fn env_absent_yields_default_without_error() {
        let (t, err) = resolve_from_env(table(&[]));
        assert_eq!(t, Tolerance::with_eps(DEFAULT_EPS));
        assert!(err.is_empty());
    }

    #[test]
    fn env_well_formed_value_wins() {
        let (t, err) = resolve_from_env(table(&[(ENV_EPS, "1e-6")]));
        assert!(err.is_empty());
        assert_eq!(t, Tolerance::with_eps(1e-6));

        let (t, err) = resolve_from_env(table(&[(ENV_EPS, "2.5e-8")]));
        assert!(err.is_empty());
        assert_eq!(t, Tolerance::with_eps(2.5e-8));
    }

    #[test]
    fn env_malformed_value_defaults_and_records() {
        let (t, err) = resolve_from_env(table(&[(ENV_EPS, "bogus")]));
        assert_eq!(t, Tolerance::with_eps(DEFAULT_EPS));
        assert_eq!(
            err,
            vec![ToleranceEnvError {
                var: ENV_EPS,
                raw: "bogus".to_string(),
                kind: ToleranceEnvErrorKind::Unparsable,
            }]
        );

        // Empty string and stray whitespace are malformed too (parsed
        // as-is, no trimming — a config anomaly worth surfacing).
        for raw in ["", " 1e-6", "1e-6 "] {
            let (t, err) = resolve_from_env(table(&[(ENV_EPS, raw)]));
            assert_eq!(t.eps, DEFAULT_EPS);
            let [err] = &err[..] else {
                panic!("malformed value must be recorded (exactly one error)");
            };
            assert_eq!(err.var, ENV_EPS);
            assert_eq!(err.kind, ToleranceEnvErrorKind::Unparsable);
        }
    }

    #[test]
    fn env_non_finite_value_defaults_and_records() {
        for raw in ["inf", "-inf", "NaN"] {
            let (t, err) = resolve_from_env(table(&[(ENV_EPS, raw)]));
            assert_eq!(t.eps, DEFAULT_EPS);
            let [err] = &err[..] else {
                panic!("non-finite value must be recorded (exactly one error)");
            };
            assert_eq!(err.var, ENV_EPS);
            assert!(
                matches!(err.kind, ToleranceEnvErrorKind::Invalid { .. }),
                "expected Invalid, got {:?} for {raw:?}",
                err.kind
            );
        }
    }

    #[test]
    fn env_non_positive_value_defaults_and_records() {
        for raw in ["-1e-9", "0", "-0.0"] {
            let (t, err) = resolve_from_env(table(&[(ENV_EPS, raw)]));
            assert_eq!(t.eps, DEFAULT_EPS);
            let [err] = &err[..] else {
                panic!("non-positive value must be recorded (exactly one error)");
            };
            assert_eq!(err.var, ENV_EPS);
            assert!(matches!(err.kind, ToleranceEnvErrorKind::Invalid { .. }));
        }
    }

    // `init` with an *invalid* value is rejected before the global
    // `OnceLock` is touched, so these are safe outside the funnel test.
    #[test]
    fn init_rejects_invalid_values_without_initializing() {
        let cases = [
            (Tolerance::with_eps(-1e-9), -1e-9),
            (Tolerance::with_eps(f64::INFINITY), f64::INFINITY),
            (Tolerance::with_eps(0.0), 0.0),
        ];
        for (t, value) in cases {
            assert_eq!(
                Tolerance::init(t),
                Err(ToleranceError::InvalidValue { value })
            );
        }
        // NaN separately (NaN != NaN defeats assert_eq on the error).
        let err =
            Tolerance::init(Tolerance::with_eps(f64::NAN)).expect_err("NaN eps must be rejected");
        assert!(matches!(
            err,
            ToleranceError::InvalidValue { value } if value.is_nan()
        ));
    }

    /// THE single test in this binary that touches the global `OnceLock` —
    /// env sanity, get/env consistency, init-after-get, and get stability
    /// all funnel through here so the outcome is deterministic under any
    /// test interleaving.
    ///
    /// The first assertion is the designed loud-failure path for the CI
    /// multi-ε matrix: `CAD_TOLERANCE_EPS=bogus cargo test` must fail
    /// exactly here.
    #[test]
    fn global_get_env_sanity_and_once_semantics() {
        let t = Tol::witness().get();

        // Env sanity: any malformed/invalid tolerance env var fails the run
        // here, loudly, with the recorded error in the message.
        let env_errors = Tolerance::env_init_errors();
        assert!(
            env_errors.is_empty(),
            "tolerance environment override(s) were rejected:\n{}",
            env_errors
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("\n")
        );

        // Consistency: the global value is exactly what the pure resolver
        // computes from the real environment.
        let (expected, _) = resolve_from_env(env_lookup);
        assert_eq!(t, expected);

        // init after get: typed AlreadyInitialized carrying current and
        // attempted (D4: the tolerance cannot change mid-run).
        let attempted = Tolerance::with_eps(123.0);
        assert_eq!(
            Tolerance::init(attempted),
            Err(ToleranceError::AlreadyInitialized {
                current: t,
                attempted
            })
        );

        // get is stable: same value on every subsequent call.
        assert_eq!(Tol::witness().get(), t);
    }
    #[test]
    fn env_k_well_formed_value_wins() {
        let (t, err) = resolve_from_env(table(&[(ENV_K, "30")]));
        assert!(err.is_empty());
        assert_eq!(
            t,
            Tolerance {
                eps: DEFAULT_EPS,
                k: 30.0
            }
        );

        let (t, err) = resolve_from_env(table(&[(ENV_EPS, "1e-6"), (ENV_K, "3.5")]));
        assert!(err.is_empty());
        assert_eq!(t, Tolerance { eps: 1e-6, k: 3.5 });
    }

    #[test]
    fn env_k_invalid_or_malformed_defaults_and_records() {
        // K must be strictly greater than 1: 1.0, 0.5, and negatives
        // fall back to the default and record.
        for raw in ["1.0", "0.5", "-3"] {
            let (t, err) = resolve_from_env(table(&[(ENV_K, raw)]));
            assert_eq!(t.k, DEFAULT_K);
            let [e] = &err[..] else {
                panic!("expected exactly one recorded error for {raw:?}");
            };
            assert_eq!(e.var, ENV_K);
        }
        let (t, err) = resolve_from_env(table(&[(ENV_K, "ten")]));
        assert_eq!(t.k, DEFAULT_K);
        assert_eq!(
            err,
            vec![ToleranceEnvError {
                var: ENV_K,
                raw: "ten".to_string(),
                kind: ToleranceEnvErrorKind::Unparsable,
            }]
        );
    }

    // ---- ε provenance (S22, 2026-08-19) ----
    //
    // The PURE half: `env_eps_source` reads the same injectable lookup
    // the env policy does, so the whole env-bootstrap side of the
    // channel is decidable without touching the process env or the
    // global `OnceLock`. The `Init` / `Document` variants need a fresh
    // process each and live in `tests/eps_provenance.rs`.

    /// Resolve, then classify — exactly the pair `global()` performs.
    fn resolve_and_source<'a>(
        pairs: &'a [(&'a str, &'a str)],
    ) -> (Tolerance, EpsilonSource, Vec<ToleranceEnvError>) {
        let (t, errs) = resolve_from_env(table(pairs));
        let source = env_eps_source(table(pairs), &errs);
        (t, source, errs)
    }

    #[test]
    fn eps_source_is_default_when_env_is_absent() {
        let (t, source, errs) = resolve_and_source(&[]);
        assert_eq!(t.eps, DEFAULT_EPS);
        assert_eq!(source, EpsilonSource::Default);
        assert!(errs.is_empty());
    }

    #[test]
    fn eps_source_is_env_when_env_supplies_the_value() {
        let (t, source, errs) = resolve_and_source(&[(ENV_EPS, "1e-6")]);
        assert_eq!(t.eps, 1e-6);
        assert_eq!(source, EpsilonSource::Env);
        assert!(errs.is_empty());
    }

    #[test]
    fn a_rejected_env_eps_reports_default_not_env() {
        // The value the run DECIDES at is the compiled default, so the
        // provenance must say so — reporting `Env` here would name a
        // number that never reached a predicate. The rejection is still
        // carried, and the report prints it.
        for raw in ["bogus", "", "0", "-1e-9", "NaN"] {
            let (t, source, errs) = resolve_and_source(&[(ENV_EPS, raw)]);
            assert_eq!(t.eps, DEFAULT_EPS, "{raw:?}");
            assert_eq!(source, EpsilonSource::Default, "{raw:?}");
            assert_eq!(errs.len(), 1, "{raw:?}");
        }
    }

    #[test]
    fn a_rejected_k_does_not_disturb_eps_provenance() {
        // K's rejection is recorded against ENV_K; the ε channel is
        // unaffected, and the ε provenance must not read the wrong var.
        let (t, source, errs) = resolve_and_source(&[(ENV_EPS, "1e-6"), (ENV_K, "ten")]);
        assert_eq!(t.eps, 1e-6);
        assert_eq!(t.k, DEFAULT_K);
        assert_eq!(source, EpsilonSource::Env);
        assert_eq!(errs.len(), 1);
        assert_eq!(errs[0].var, ENV_K);

        let (_, source, _) = resolve_and_source(&[(ENV_K, "ten")]);
        assert_eq!(source, EpsilonSource::Default);
    }

    #[test]
    fn the_report_line_names_the_value_and_the_channel() {
        // Reporting is the whole point of the channel (S22 / the
        // no-ambient-env rule's clause 3), so the rendered line is
        // pinned: it must carry the number, the source, and any
        // rejected value.
        let report = ToleranceReport {
            tolerance: Tolerance { eps: 1e-6, k: 10.0 },
            eps_source: EpsilonSource::Env,
            env_errors: &[],
        };
        let line = report.to_string();
        assert!(line.contains("1e-6"), "{line}");
        assert!(line.contains(ENV_EPS), "{line}");
        assert!(line.contains("K = 10"), "{line}");
        assert!(!line.contains("REJECTED"), "{line}");

        for (source, needle) in [
            (EpsilonSource::Default, "compiled default"),
            (EpsilonSource::Init, "Tolerance::init"),
            (EpsilonSource::Document, "loaded document"),
        ] {
            let line = ToleranceReport {
                tolerance: Tolerance::with_eps(DEFAULT_EPS),
                eps_source: source,
                env_errors: &[],
            }
            .to_string();
            assert!(line.contains(needle), "{source:?}: {line}");
        }
    }

    #[test]
    fn the_report_line_prints_a_rejected_env_value() {
        // Issue #497's shape: the run is at the compiled default
        // because the ambient value was garbage, and the line has to
        // say both halves.
        static REJECTED: std::sync::OnceLock<Vec<ToleranceEnvError>> = std::sync::OnceLock::new();
        let errs = REJECTED.get_or_init(|| {
            let (_, errs) = resolve_from_env(table(&[(ENV_EPS, "bogus")]));
            errs
        });
        let line = ToleranceReport {
            tolerance: Tolerance::with_eps(DEFAULT_EPS),
            eps_source: EpsilonSource::Default,
            env_errors: errs,
        }
        .to_string();
        assert!(line.contains("compiled default"), "{line}");
        assert!(line.contains("REJECTED"), "{line}");
        assert!(line.contains("bogus"), "{line}");
    }

    #[test]
    fn invalid_k_is_rejected_by_validate() {
        for k in [1.0, 0.0, -2.0, f64::NAN, f64::INFINITY] {
            let attempted = Tolerance {
                eps: DEFAULT_EPS,
                k,
            };
            match attempted.validate() {
                Err(ToleranceError::InvalidK { value }) => {
                    assert!(value.is_nan() || value == k);
                }
                other => panic!("k = {k}: expected InvalidK, got {other:?}"),
            }
        }
    }
}
