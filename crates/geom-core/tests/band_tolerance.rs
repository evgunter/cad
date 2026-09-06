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
//! ε **and its K**, never to a fixed value of either. Both halves of the
//! band are configurable (`CAD_AMBIGUITY_K` admits any K > 1), so a
//! multiplier written as a literal would pin this file to the default
//! K = 10 while still reading as ε-relative.

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
    // Reaching it needs a zero threshold in (MAX/K, MAX], which depends on
    // the run's K: the target below is MAX*(1+K)/(2K), a fraction in
    // (1/2, 1] that is under MAX for every K > 1 and over MAX/K for every
    // K > 1, so the residue stays reachable at whatever K the session
    // committed instead of only at a large one. (The factor is formed
    // before scaling MAX: MAX*(1+K) would itself overflow.) The arm is
    // necessarily subnormal — eps/arm has to land near MAX — so at the
    // tightest matrix eps the construction runs out of significand for a
    // K within ~1e-4 of 1; a band that narrow is degenerate anyway.
    let overflow_zero = f64::MAX * ((1.0 + tolerance.k) / (2.0 * tolerance.k));
    let tiny_arm = tolerance.eps / overflow_zero;
    assert_eq!(
        Band::angular_at(Tol::witness(), tiny_arm),
        Err(BandError::InvalidValue {
            field: BandField::Escalate,
            value: f64::INFINITY,
        })
    );

    // Classification tracks the run's band: margins placed relative to
    // eps AND K land in the same region at every matrix value. Every
    // multiplier is derived from the run's own K rather than assuming
    // one, so this row holds at each K a session can commit (K > 1) and
    // not only at the default 10; both margins are safely interior to
    // their regions under fp rounding.
    let eps = tolerance.eps;
    assert_eq!((0.5 * eps).sign_within(band), Ok(Sign::Zero));
    assert_eq!((-0.5 * eps).sign_within(band), Ok(Sign::Zero));
    let definite = 2.0 * tolerance.k * eps;
    assert_eq!(definite.sign_within(band), Ok(Sign::Positive));
    assert_eq!((-definite).sign_within(band), Ok(Sign::Negative));
    let mid = (1.0 + tolerance.k) / 2.0 * eps; // strictly inside (eps, K*eps)
    let sliver = mid
        .sign_within(band)
        .expect_err("the band midpoint lies inside the ambiguity band");
    assert_eq!(sliver.margin, MarginDiag::Value(mid));
    assert_eq!(sliver.band, band);
}
