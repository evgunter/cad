//! The single global [`Tolerance`] value (D4 ¶1 in `docs/DESIGN.md`).
//!
//! One tolerance pair per run, shared by all bodies, never loosened
//! mid-run — per-model tolerances are deliberately rejected (any two bodies
//! must be boolean-combinable). D4 leaves "compile-time constant vs.
//! once-initialized" open as an implementation detail; this module chooses
//! **once-initialized** (a [`std::sync::OnceLock`]), which is what lets the
//! test suite run at several ε values (the multi-ε CI matrix) with zero
//! test-code cooperation.
//!
//! # Initialization protocol
//!
//! - [`Tolerance::init`] installs an explicit value, exactly once, with
//!   typed errors ([`ToleranceError`]) for invalid values or double
//!   initialization. There is no API to change the value afterwards —
//!   "never loosened mid-run" is structural.
//! - [`Tolerance::get`] is **infallible and total**. On first use without a
//!   prior `init` it self-initializes from the environment
//!   ([`ENV_EPS`] / [`ENV_EPS_ANGULAR`]): well-formed values win, absent
//!   variables fall back to the compiled defaults, and a malformed or
//!   invalid value falls back to the defaults **while recording the
//!   failure**, retrievable via [`Tolerance::env_init_error`]. Loudness is
//!   restored structurally: a regular `#[test]` in this crate asserts the
//!   recorded error is `None`, so a typo'd ε in a CI matrix row fails
//!   visibly through the normal test mechanism (no library panic per D9,
//!   no silent config swallow per D4).

use core::fmt;
use std::sync::OnceLock;

/// Environment variable consulted for ε on `get()` self-initialization
/// (name fixed by CI's multi-ε matrix, L3 in `docs/M0-LOG.md`).
pub const ENV_EPS: &str = "CAD_TOLERANCE_EPS";

/// Environment variable consulted for εₐ on `get()` self-initialization.
pub const ENV_EPS_ANGULAR: &str = "CAD_TOLERANCE_EPS_ANGULAR";

/// Compiled default for ε: 1e-9 m (ratified, D4 ¶1) — micron-to-kilometer
/// coverage with ~4 orders of f64 headroom at km scale.
pub const DEFAULT_EPS: f64 = 1e-9;

/// Compiled default for εₐ: 1e-8 rad. **Provisional**: DESIGN.md defers
/// εₐ's numeric value to M0 experiments; 1e-8 sits above f64
/// normal-computation noise (~1e-8 at unit scale). Revisit where the first
/// real consumers (transversality margins, M0 PRs 3–4) land.
pub const DEFAULT_EPS_ANGULAR: f64 = 1e-8;

/// The kernel's global tolerance pair (D4 ¶1): one value per run, shared by
/// all bodies, never loosened mid-run.
///
/// Both fields are `f64` regardless of the scalar type `T` in use — they
/// bound *residuals and margins*, which are plain numbers even when the
/// geometry is evaluated at intervals or dual numbers. Units are the
/// kernel's fixed internal units (D4 ¶4): meters and radians.
///
/// A `Tolerance` value is valid when both fields are finite and strictly
/// positive; [`Tolerance::init`] enforces this (that documented range *is*
/// the sane range — no further restriction).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tolerance {
    /// Linear tolerance ε, in meters.
    pub eps: f64,
    /// Angular tolerance εₐ, in radians.
    pub eps_angular: f64,
}

/// Which [`Tolerance`] field a [`ToleranceError::InvalidValue`] refers to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToleranceField {
    /// The linear tolerance ε.
    Eps,
    /// The angular tolerance εₐ.
    EpsAngular,
}

impl ToleranceField {
    /// The field's name in error messages.
    fn name(self) -> &'static str {
        match self {
            Self::Eps => "eps",
            Self::EpsAngular => "eps_angular",
        }
    }
}

/// Typed error from [`Tolerance::init`] (D9: every failure is a typed
/// error, never a panic).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToleranceError {
    /// A field of the attempted `Tolerance` is not finite and strictly
    /// positive.
    InvalidValue {
        /// The offending field.
        field: ToleranceField,
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
            Self::InvalidValue { field, value } => write!(
                f,
                "invalid tolerance: {} = {value:e} (must be finite and > 0)",
                field.name()
            ),
            Self::AlreadyInitialized { current, attempted } => write!(
                f,
                "tolerance already initialized to (eps = {:e}, eps_angular = {:e}); \
                 attempted (eps = {:e}, eps_angular = {:e}) — one value per run (D4)",
                current.eps, current.eps_angular, attempted.eps, attempted.eps_angular
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
/// falls back to the compiled defaults; retrieve it via
/// [`Tolerance::env_init_error`]. A test in this crate asserts it is
/// `None`, which is what makes a malformed CI env value fail loudly.
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
/// commitment came from `get()`'s env path, any recorded env failure.
#[derive(Debug)]
struct Global {
    tolerance: Tolerance,
    env_error: Option<ToleranceEnvError>,
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
        let (tolerance, env_error) = resolve_from_env(env_lookup);
        Global {
            tolerance,
            env_error,
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
) -> (f64, Option<ToleranceEnvError>) {
    let Some(raw) = lookup(var) else {
        return (default, None);
    };
    match raw.parse::<f64>() {
        Ok(value) if value.is_finite() && value > 0.0 => (value, None),
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
/// Each variable resolves independently (a bad ε does not discard a good
/// εₐ). If both variables are bad, the recorded error is the ε one
/// (resolution order: ε, then εₐ) — one recorded error is enough to fail
/// the run loudly.
fn resolve_from_env(
    lookup: impl Fn(&str) -> Option<String>,
) -> (Tolerance, Option<ToleranceEnvError>) {
    let (eps, eps_err) = resolve_var(&lookup, ENV_EPS, DEFAULT_EPS);
    let (eps_angular, angular_err) = resolve_var(&lookup, ENV_EPS_ANGULAR, DEFAULT_EPS_ANGULAR);
    (Tolerance { eps, eps_angular }, eps_err.or(angular_err))
}

impl Tolerance {
    /// Validates that both fields are finite and strictly positive.
    fn validate(self) -> Result<(), ToleranceError> {
        if !(self.eps.is_finite() && self.eps > 0.0) {
            return Err(ToleranceError::InvalidValue {
                field: ToleranceField::Eps,
                value: self.eps,
            });
        }
        if !(self.eps_angular.is_finite() && self.eps_angular > 0.0) {
            return Err(ToleranceError::InvalidValue {
                field: ToleranceField::EpsAngular,
                value: self.eps_angular,
            });
        }
        Ok(())
    }

    /// Installs `tolerance` as the run's global tolerance, exactly once.
    ///
    /// # Errors
    ///
    /// - [`ToleranceError::InvalidValue`] if a field is not finite and
    ///   strictly positive (checked before touching the global state, so a
    ///   rejected call leaves initialization for later).
    /// - [`ToleranceError::AlreadyInitialized`] if the global tolerance was
    ///   already committed — by an earlier `init` or by [`Tolerance::get`]'s
    ///   env self-initialization — carrying both the current and the
    ///   attempted value. There is no way to change the tolerance after
    ///   commitment (D4 ¶1: never loosened mid-run).
    pub fn init(tolerance: Tolerance) -> Result<(), ToleranceError> {
        tolerance.validate()?;
        let mut installed = false;
        let global = GLOBAL.get_or_init(|| {
            installed = true;
            Global {
                tolerance,
                env_error: None,
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
    /// from the environment ([`ENV_EPS`], [`ENV_EPS_ANGULAR`]); absent
    /// variables fall back to [`DEFAULT_EPS`] / [`DEFAULT_EPS_ANGULAR`],
    /// and malformed or invalid values fall back to the defaults while
    /// recording the failure (see [`Tolerance::env_init_error`]).
    pub fn get() -> Tolerance {
        global().tolerance
    }

    /// Any failure recorded by `get()`'s env self-initialization.
    ///
    /// Forces initialization (as if by [`Tolerance::get`]) if it has not
    /// happened yet, so the answer is definitive. `None` means the
    /// committed tolerance involved no rejected env value — in particular,
    /// initialization via [`Tolerance::init`] never consults the
    /// environment and records nothing.
    ///
    /// The crate's test suite asserts this is `None`, which is what makes
    /// a typo'd tolerance in a CI matrix row fail loudly (through the
    /// normal test mechanism rather than a library panic, per D9).
    pub fn env_init_error() -> Option<&'static ToleranceEnvError> {
        global().env_error.as_ref()
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
    // (`get` / `env_init_error` / a necessarily-failing late `init`). The
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
    fn env_absent_yields_defaults_without_error() {
        let (t, err) = resolve_from_env(table(&[]));
        assert_eq!(
            t,
            Tolerance {
                eps: DEFAULT_EPS,
                eps_angular: DEFAULT_EPS_ANGULAR
            }
        );
        assert_eq!(err, None);
    }

    #[test]
    fn env_well_formed_values_win() {
        let (t, err) = resolve_from_env(table(&[(ENV_EPS, "1e-6")]));
        assert_eq!(err, None);
        assert_eq!(
            t,
            Tolerance {
                eps: 1e-6,
                eps_angular: DEFAULT_EPS_ANGULAR
            }
        );

        let (t, err) = resolve_from_env(table(&[
            (ENV_EPS, "2.5e-8"),
            (ENV_EPS_ANGULAR, "0.0000001"),
        ]));
        assert_eq!(err, None);
        assert_eq!(
            t,
            Tolerance {
                eps: 2.5e-8,
                eps_angular: 1e-7
            }
        );
    }

    #[test]
    fn env_malformed_value_defaults_and_records() {
        let (t, err) = resolve_from_env(table(&[(ENV_EPS, "bogus")]));
        assert_eq!(
            t,
            Tolerance {
                eps: DEFAULT_EPS,
                eps_angular: DEFAULT_EPS_ANGULAR
            }
        );
        assert_eq!(
            err,
            Some(ToleranceEnvError {
                var: ENV_EPS,
                raw: "bogus".to_string(),
                kind: ToleranceEnvErrorKind::Unparsable,
            })
        );

        // Empty string and stray whitespace are malformed too (parsed
        // as-is, no trimming — a config anomaly worth surfacing).
        for raw in ["", " 1e-6", "1e-6 "] {
            let (t, err) = resolve_from_env(table(&[(ENV_EPS_ANGULAR, raw)]));
            assert_eq!(t.eps_angular, DEFAULT_EPS_ANGULAR);
            let err = err.expect("malformed value must be recorded");
            assert_eq!(err.var, ENV_EPS_ANGULAR);
            assert_eq!(err.kind, ToleranceEnvErrorKind::Unparsable);
        }
    }

    #[test]
    fn env_non_finite_value_defaults_and_records() {
        for raw in ["inf", "-inf", "NaN"] {
            let (t, err) = resolve_from_env(table(&[(ENV_EPS, raw)]));
            assert_eq!(t.eps, DEFAULT_EPS);
            let err = err.expect("non-finite value must be recorded");
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
            let err = err.expect("non-positive value must be recorded");
            assert_eq!(err.var, ENV_EPS);
            assert!(matches!(err.kind, ToleranceEnvErrorKind::Invalid { .. }));
        }
    }

    #[test]
    fn env_fields_resolve_independently_and_eps_error_wins() {
        // A bad eps does not discard a good eps_angular.
        let (t, err) = resolve_from_env(table(&[(ENV_EPS, "junk"), (ENV_EPS_ANGULAR, "1e-7")]));
        assert_eq!(
            t,
            Tolerance {
                eps: DEFAULT_EPS,
                eps_angular: 1e-7
            }
        );
        assert_eq!(err.expect("error must be recorded").var, ENV_EPS);

        // Both bad: the recorded error is the eps one (resolution order).
        let (t, err) = resolve_from_env(table(&[(ENV_EPS, "junk"), (ENV_EPS_ANGULAR, "junk")]));
        assert_eq!(
            t,
            Tolerance {
                eps: DEFAULT_EPS,
                eps_angular: DEFAULT_EPS_ANGULAR
            }
        );
        assert_eq!(err.expect("error must be recorded").var, ENV_EPS);
    }

    // `init` with an *invalid* value is rejected before the global
    // `OnceLock` is touched, so these are safe outside the funnel test.
    #[test]
    fn init_rejects_invalid_values_without_initializing() {
        let cases = [
            (
                Tolerance {
                    eps: -1e-9,
                    eps_angular: 1e-8,
                },
                ToleranceField::Eps,
                -1e-9,
            ),
            (
                Tolerance {
                    eps: f64::INFINITY,
                    eps_angular: 1e-8,
                },
                ToleranceField::Eps,
                f64::INFINITY,
            ),
            (
                Tolerance {
                    eps: 1e-9,
                    eps_angular: 0.0,
                },
                ToleranceField::EpsAngular,
                0.0,
            ),
        ];
        for (t, field, value) in cases {
            assert_eq!(
                Tolerance::init(t),
                Err(ToleranceError::InvalidValue { field, value })
            );
        }
        // NaN separately (NaN != NaN defeats assert_eq on the error).
        let err = Tolerance::init(Tolerance {
            eps: f64::NAN,
            eps_angular: 1e-8,
        })
        .expect_err("NaN eps must be rejected");
        assert!(matches!(
            err,
            ToleranceError::InvalidValue {
                field: ToleranceField::Eps,
                value,
            } if value.is_nan()
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

        // Env sanity: a malformed/invalid tolerance env var fails the run
        // here, loudly, with the recorded error in the message.
        let env_error = Tolerance::env_init_error();
        assert!(
            env_error.is_none(),
            "tolerance environment override was rejected: {}",
            env_error.map_or_else(String::new, ToString::to_string)
        );

        // Consistency: the global value is exactly what the pure resolver
        // computes from the real environment.
        let (expected, _) = resolve_from_env(env_lookup);
        assert_eq!(t, expected);

        // init after get: typed AlreadyInitialized carrying current and
        // attempted (D4: the tolerance cannot change mid-run).
        let attempted = Tolerance {
            eps: 123.0,
            eps_angular: 4.0,
        };
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
}
