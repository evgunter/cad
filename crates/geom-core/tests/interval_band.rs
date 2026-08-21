//! Interval classification against the run's global [`Tolerance`] —
//! the tolerance-coupled counterpart of `src/interval.rs`'s pure tests
//! (which only ever build bands via `Band::new`).
//!
//! This lives in its own integration-test binary — i.e. its own process —
//! per the funnel-test discipline (see `src/tolerance.rs`'s test module):
//! the global tolerance commits exactly once per process, so this file
//! must contain exactly ONE `#[test]` (a second would race it for first
//! touch of the global).
//!
//! Deliberately NO explicit `init`: the global self-initializes from the
//! environment on the first `Band::linear(Tol::witness())` call, so the CI interval
//! job's `CAD_TOLERANCE_EPS` run genuinely exercises a different band —
//! every assertion is written relative to the run's ε, never to a fixed
//! value. (The multipliers 0.5, 3, 20 are safely interior to their
//! regions under fp rounding — same reasoning as `tests/band_tolerance.rs`
//! — and 0.5·ε / 20·ε are exact-or-commuting scalings, so no product can
//! cross the thresholds ε and 10·ε.)

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Bounds, Decide, Indeterminate, Interval, MarginDiag, Real, Sign, Tolerance};
use geom_core::Tol;

#[test]
fn interval_classification_tracks_the_global_tolerance() {
    let band = Band::linear(Tol::witness()).expect("the run's eps is sane, so K*eps cannot overflow");
    let eps = Tol::witness().get().eps;

    // Point enclosures land where their f64 margins would (the two
    // `Decide` instantiations implement the same decision table).
    assert_eq!(
        Interval::from_f64(0.5 * eps).sign_within(band),
        Ok(Sign::Zero)
    );
    assert_eq!(
        Interval::from_f64(20.0 * eps).sign_within(band),
        Ok(Sign::Positive)
    );
    assert_eq!(
        Interval::from_f64(-20.0 * eps).sign_within(band),
        Ok(Sign::Negative)
    );

    // A genuine enclosure wholly inside the coincidence region.
    assert_eq!(
        Interval::from_bounds(-0.5 * eps, 0.5 * eps).sign_within(band),
        Ok(Sign::Zero)
    );

    // An enclosure straddling the band carries its exact bounds in the
    // diagnostic (the Enclosure variant, landed with this PR).
    let straddling = Interval::from_bounds(0.5 * eps, 20.0 * eps)
        .sign_within(band)
        .expect_err("straddles both thresholds — no certifiable sign");
    assert_eq!(
        straddling,
        Indeterminate {
            margin: MarginDiag::Enclosure {
                lo: 0.5 * eps,
                hi: 20.0 * eps,
            },
            band,
            predicate: None,
        }
    );

    // A point in the sliver band is indeterminate even though it is
    // exact — the band is semantic, at every ε in the CI matrix.
    let sliver = Interval::from_f64(3.0 * eps)
        .sign_within(band)
        .expect_err("3ε lies inside the ambiguity band (ε, 10ε)");
    assert_eq!(
        sliver.margin,
        MarginDiag::Enclosure {
            lo: 3.0 * eps,
            hi: 3.0 * eps,
        }
    );

    // The decoration channel at the run's tolerance: sqrt of an
    // enclosure dipping below zero clamps (plausible bounds!) but must
    // never branch.
    let clamped = Interval::from_bounds(-eps, eps).sqrt();
    assert!(clamped.lo() >= 0.0, "clamped enclosure looks plausible");
    assert_eq!(
        clamped.sign_within(band),
        Err(Indeterminate {
            margin: MarginDiag::Invalid,
            band,
            predicate: None,
        })
    );

    // Bounds extraction (certification's door) returns the exact
    // endpoints the driver put in.
    let x = Interval::from_bounds(-eps, 2.0 * eps);
    assert_eq!(x.lo(), -eps);
    assert_eq!(x.hi(), 2.0 * eps);
}
