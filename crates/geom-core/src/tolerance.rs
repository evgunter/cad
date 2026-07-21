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
//! - [`Tolerance::get`] is **infallible and total**. On first use without a
//!   prior `init` it self-initializes from the environment ([`ENV_EPS`]):
//!   a well-formed value wins, an absent variable falls back to the
//!   compiled default, and a malformed or invalid value falls back to the
//!   default **while recording the failure**. The rejected variable is
//!   recorded, retrievable via [`Tolerance::env_init_errors`]. Loudness is
//!   restored structurally: a regular `#[test]` in this crate asserts the
//!   recorded slice is empty, so a typo'd ε in a CI matrix row fails
//!   visibly through the normal test mechanism (no library panic per D9,
//!   no silent config swallow per D4).

use core::fmt;
use std::sync::OnceLock;

/// Environment variable consulted for ε on `get()` self-initialization
/// (name fixed by CI's multi-ε matrix, L3 in `docs/M0-LOG.md`).
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
/// This is *recorded*, not raised — [`Tolerance::get`] stays total and
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
            tolerance,
            env_errors,
        }
    })
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
    ///   already committed — by an earlier `init` or by [`Tolerance::get`]'s
    ///   env self-initialization — carrying both the current and the
    ///   attempted value. There is no way to change the tolerance after
    ///   commitment (D4 ¶1: never loosened mid-run).
    ///
    /// When `init` is used it should be called during single-threaded
    /// startup, before any [`Tolerance::get`]; racing `init` against a first
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

    /// The run's global tolerance. Infallible and total.
    ///
    /// On first call without a prior [`Tolerance::init`], self-initializes
    /// from the environment ([`ENV_EPS`]); an absent variable falls back to
    /// [`DEFAULT_EPS`], and a malformed or invalid value falls back to the
    /// default while recording the failure (see
    /// [`Tolerance::env_init_errors`]).
    pub fn get() -> Tolerance {
        global().tolerance
    }

    /// Every failure recorded by `get()`'s env self-initialization.
    ///
    /// At most one entry per tolerance env var — at most two at present
    /// ([`ENV_EPS`] and [`ENV_K`]); the slice shape is kept stable for
    /// future run-global config vars.
    ///
    /// Forces initialization (as if by [`Tolerance::get`]) if it has not
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
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
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
        let t = Tolerance::get();

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
        assert_eq!(Tolerance::get(), t);
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
