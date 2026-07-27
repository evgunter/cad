//! The lint's LITMUS (M4 PR 8b spec D3): the #99 case must light up.
//!
//! History: the demo bracket's inner fillet once used a 3-decimal via
//! point (1.146 vs the exact apex 1.5 − 0.5/√2 = 1.1464466…), leaving
//! the arc carrier ~2.315e-6 m clear of the adjacent line carriers
//! instead of tangent. At CAD_TOLERANCE_EPS=1e-6 that margin sat
//! INSIDE the `carrier_line_circle` escalation band — the kernel
//! correctly refused, the demo's `.expect` panicked (#99); #100 fixed
//! the datum in place. The lint's whole point is to catch that class
//! BEFORE any escalation band is entered: at the default and 1e-12
//! rows the margin was a *definite* outcome — invisible to every
//! existing gate — yet 3+ decades below the baseline distribution's
//! floor.
//!
//! This test RESURRECTS the old bracket profile (test-only — the demo
//! keeps the fix) at the recording scalar, re-measures the historical
//! margin from the actual kernel predicates, and asserts the lint
//! flags it at EVERY supported ε row: in-band at 1e-6, below the
//! baseline floor at 1e-9 and 1e-12. The FIXED bracket's margin
//! (~1.1e-16, a definite Zero) must stay clean at every row — the
//! lint separates the two brackets exactly as #99 hindsight demands.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::k_stats::{self, Probe};
use geom_core::{Point2, Tolerance};
use k_lint::{Reason, lint_sample};
use profile::{LoopBuilder, Profile, ProfileLoop, SketchPlane};

fn p2(x: f64, y: f64) -> Point2<Probe> {
    Point2::new(Probe(x), Probe(y))
}

/// Validates a bracket loop and returns the smallest recorded
/// `carrier_line_circle` margin — the #99 statistic, measured by the
/// real predicates at the recording scalar (process ε = the compiled
/// default, 1e-9, where both variants validate with definite
/// outcomes).
fn carrier_line_circle_margin(lp: ProfileLoop<Probe>) -> f64 {
    k_stats::start_recording();
    Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tolerance::get())
        .expect("bracket profile validates at the default eps");
    let samples = k_stats::take_samples();
    samples
        .iter()
        .filter(|s| s.predicate == "carrier_line_circle")
        .map(|s| s.margin.abs())
        .fold(f64::INFINITY, f64::min)
}

/// The pre-#100 bracket loop, resurrected verbatim: hand-supplied via
/// point rounded to 1.146 — NOT tangent (2.315e-6 m clear), so it
/// carries no tangency declaration and validates wherever the margin
/// is definite (the compiled default; at 1e-6 it was #99's panic).
fn old_bracket_loop() -> ProfileLoop<Probe> {
    LoopBuilder::start(p2(0.0, 0.0))
        .line_to(p2(3.0, 0.0))
        .line_to(p2(3.0, 1.0))
        .line_to(p2(1.5, 1.0))
        .arc_to_via(p2(1.146, 1.146), p2(1.0, 1.5))
        .line_to(p2(1.0, 3.0))
        .line_to(p2(0.0, 3.0))
        .close()
}

/// The shipped bracket loop (post-#100/#101): the fillet constructor
/// computes the tangent arc exactly and DECLARES both joints (an
/// undeclared exact tangency would refuse typed — #101 discipline).
fn fixed_bracket_loop() -> ProfileLoop<Probe> {
    LoopBuilder::start(p2(0.0, 0.0))
        .line_to(p2(3.0, 0.0))
        .line_to(p2(3.0, 1.0))
        .fillet(p2(1.0, 1.0), p2(1.0, 3.0), Probe(0.5))
        .expect("bracket fillet fits")
        .line_to(p2(1.0, 3.0))
        .line_to(p2(0.0, 3.0))
        .close()
}

/// The supported ε rows, as (band_zero, band_escalate) with the
/// ratified K = 10 — the bands the sweep would record at each row
/// (margins are geometry; only the band moves with ε).
const EPS_ROWS: [(f64, f64); 3] = [(1e-6, 1e-5), (1e-9, 1e-8), (1e-12, 1e-11)];

#[test]
fn old_bracket_datum_lights_up_at_every_eps_row() {
    assert_eq!(
        Tolerance::get().eps,
        1e-9,
        "litmus measures at the compiled default eps (run without \
         CAD_TOLERANCE_EPS)"
    );
    // Resurrect the pre-#100 datum: via point rounded to 1.146.
    let margin = carrier_line_circle_margin(old_bracket_loop());
    // Pin the resurrection: the historical margin, re-derived by the
    // #99 review in exact rationals, is 2.315e-6 m.
    assert!(
        (2.31e-6..2.32e-6).contains(&margin),
        "the resurrected #99 margin moved: {margin:e} (expected ~2.315e-6)"
    );

    for (zero, escalate) in EPS_ROWS {
        // The outcome the kernel records at this row: in-band at
        // ε = 1e-6 (that WAS #99's panic), definite above it elsewhere.
        let outcome = if margin > zero && margin < escalate {
            "indeterminate"
        } else {
            "positive"
        };
        let reasons = lint_sample(margin, zero, escalate, outcome);
        assert!(
            !reasons.is_empty(),
            "the #99 datum must light up the lint at eps={zero:e} \
             (margin {margin:e}, outcome {outcome})"
        );
        if zero == 1e-6 {
            assert_eq!(
                reasons,
                vec![Reason::InBand],
                "at 1e-6 the datum is IN the escalation band"
            );
        } else {
            assert!(
                reasons.contains(&Reason::BelowBaselineFloor),
                "at eps={zero:e} the datum is definite — only the \
                 baseline floor can catch it, and it must ({reasons:?})"
            );
        }
    }
}

#[test]
fn fixed_bracket_stays_clean_at_every_eps_row() {
    // The shipped fix: the constructive fillet (exact tangency,
    // declared by construction).
    let margin = carrier_line_circle_margin(fixed_bracket_loop());
    assert!(
        margin < 1e-15,
        "the fixed bracket's tangency margin must be rounding noise, \
         got {margin:e}"
    );
    for (zero, escalate) in EPS_ROWS {
        // Rounding noise classifies Zero at every supported ε.
        let reasons = lint_sample(margin, zero, escalate, "zero");
        assert!(
            reasons.is_empty(),
            "the FIXED bracket must lint clean at eps={zero:e}: {reasons:?}"
        );
    }
}
