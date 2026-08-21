//! The tolerance-coupled band constructors ([`Band::linear`] /
//! [`Band::angular_at`]) against the run's global [`Tolerance`].
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
//! the environment on the first `Band::linear(Tol::witness())` call, so the multi-ε CI
//! matrix (`CAD_TOLERANCE_EPS`) genuinely exercises *different bands*
//! through this test — every assertion is written relative to the run's
//! ε, not to a fixed value.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use geom_core::{Band, BandError, BandField, Decide, MarginDiag, Sign};

#[test]
fn bands_track_the_global_tolerance() {
    let band =
        Band::linear(Tol::witness()).expect("the run's eps is sane, so K*eps cannot overflow");
    let tolerance = Tol::witness().get();

    // linear() is exactly (eps, K*eps) of the committed tolerance...
    assert_eq!(band.zero(), tolerance.eps);
    assert_eq!(band.escalate(), tolerance.k * tolerance.eps);

    // ...and angular_at derives its threshold per lever arm as eps/r —
    // there is no global angular tolerance (D4 ¶1, revised 2026-07-16).
    // At the unit lever arm (r = 1) the derived angle is exactly eps, so
    // the angular band coincides with the linear one.
    let unit = Band::angular_at(Tol::witness(), 1.0)
        .expect("the run's eps is sane, so eps/1 forms a band");
    assert_eq!(unit.zero(), tolerance.eps);
    assert_eq!(unit.escalate(), tolerance.k * tolerance.eps);
    assert_eq!(unit, band);

    // A curvature-style lever arm r = 1/kappa_rel scales the threshold:
    // zero = eps/r, escalate = K*(eps/r).
    let kappa_rel = 4.0;
    let arm = 1.0 / kappa_rel;
    let curved = Band::angular_at(Tol::witness(), arm).expect("eps/arm is a sane finite threshold");
    assert_eq!(curved.zero(), tolerance.eps / arm);
    assert_eq!(curved.escalate(), tolerance.k * (tolerance.eps / arm));

    // Overflow residue: a lever arm tiny enough that eps/arm is finite but
    // K*(eps/arm) overflows surfaces as the existing InvalidValue-on-
    // escalate error (routed through Band::new), not a silently bad band.
    // arm = eps / (MAX/2) makes eps/arm ~ MAX/2 — finite, well above the
    // MAX/K threshold — so escalate = K*(MAX/2) overflows to +inf.
    let tiny_arm = tolerance.eps / (f64::MAX / 2.0);
    assert_eq!(
        Band::angular_at(Tol::witness(), tiny_arm),
        Err(BandError::InvalidValue {
            field: BandField::Escalate,
            value: f64::INFINITY,
        })
    );

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
