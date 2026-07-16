//! The tolerance-coupled band constructors ([`Band::linear`] /
//! [`Band::angular`]) against the run's global [`Tolerance`].
//!
//! This lives in its own integration-test binary — i.e. its own process —
//! per the funnel-test discipline (see `src/tolerance.rs`'s test module):
//! the global tolerance commits exactly once per process, the lib test
//! binary's single global-touching test owns that binary's commitment,
//! and `tests/tolerance_init.rs` owns the explicit-`init` path. This file
//! must contain exactly ONE `#[test]` (a second would race it for first
//! touch of the global).
//!
//! Deliberately NO explicit `init` here: the global self-initializes from
//! the environment on the first `Band::linear()` call, so the multi-ε CI
//! matrix (`CAD_TOLERANCE_EPS`) genuinely exercises *different bands*
//! through this test — every assertion is written relative to the run's
//! ε, not to a fixed value.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{AMBIGUITY_K, Band, Decide, MarginDiag, Sign, Tolerance};

#[test]
fn bands_track_the_global_tolerance() {
    let band = Band::linear().expect("the run's eps is sane, so K*eps cannot overflow");
    let tolerance = Tolerance::get();

    // linear() is exactly (eps, K*eps) of the committed tolerance...
    assert_eq!(band.zero(), tolerance.eps);
    assert_eq!(band.escalate(), AMBIGUITY_K * tolerance.eps);

    // ...and angular() is exactly (eps_a, K*eps_a).
    let angular = Band::angular().expect("the run's eps_angular is sane");
    assert_eq!(angular.zero(), tolerance.eps_angular);
    assert_eq!(angular.escalate(), AMBIGUITY_K * tolerance.eps_angular);

    // Classification tracks the run's eps: margins placed relative to
    // eps land in the same region at every matrix value. (0.5, 3, 20 are
    // safely interior to their regions under fp rounding — no product
    // here can cross the exact thresholds eps and 10*eps.)
    let eps = tolerance.eps;
    assert_eq!((0.5 * eps).sign_within(band), Ok(Sign::Zero));
    assert_eq!((-0.5 * eps).sign_within(band), Ok(Sign::Zero));
    assert_eq!((20.0 * eps).sign_within(band), Ok(Sign::Positive));
    assert_eq!((-20.0 * eps).sign_within(band), Ok(Sign::Negative));
    let sliver = (3.0 * eps)
        .sign_within(band)
        .expect_err("3*eps lies inside the ambiguity band (eps, 10*eps)");
    assert_eq!(sliver.margin, MarginDiag::Value(3.0 * eps));
    assert_eq!(sliver.band, band);
}
