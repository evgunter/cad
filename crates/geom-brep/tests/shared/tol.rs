//! The run's committed tolerance, as the suites of this crate ask for
//! it: the linear band and ε itself.
//!
//! Both are one call to a `geom-core` door with no argument to vary, so
//! there is no derivation here to merge — what was being duplicated was
//! the three-line wrapper, once per suite, forty-eight times.
//!
//! **What this module does NOT absorb.** Four suites build a band that
//! is deliberately NOT the run's: `decoration_plane_mint.rs`
//! (a fixed 1e-9 .. 1e-8), `pcurve_p1a_meter.rs` (twice — `ROW_EPS`
//! and a fixed 1e-9 .. 1e-8) and `r2_probes.rs` (`4·DRIFT ..
//! 40·DRIFT`). Each says at its own site why it is pinned to the row's
//! scale rather than to ε, and a shared home must not make them follow
//! ε by accident.

use geom_core::{Band, Tol};

/// The run's **linear** band, (ε, K·ε) from the committed tolerance.
///
/// This is [`Band::linear`] and nothing else. Three suites
/// (`m5_pr12_circle_certificate.rs`, `pcurve_p1b_r2_probes.rs`,
/// `review_pr12_meridian_probe.rs`) used to spell its body out as
/// `Band::new(tol.eps, tol.k * tol.eps)`; that is the same value and
/// the same bits — `Band::linear(tol)` is `from_zero_threshold(tol,
/// tol.eps())`, which is `Band::new(zero, k * zero)` with `zero =
/// tol.eps()` and `k = tol.k()` (`geom-core/src/predicate.rs`) — so
/// pointing them here changed a spelling, not a threshold.
///
/// # Panics
///
/// Only if the run's ε is within a factor K of `f64::MAX`, which no
/// tolerance a session can commit reaches. A test wants to hear about
/// that loudly rather than to carry a `Result` through every row.
#[allow(clippy::expect_used)]
pub(crate) fn band() -> Band {
    Band::linear(Tol::witness()).expect("the run's linear band")
}

/// The run's committed linear tolerance ε, in meters.
pub(crate) fn eps() -> f64 {
    Tol::witness().get().eps
}
