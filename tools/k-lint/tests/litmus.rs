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
//!
//! This contract is the BINDING lower bound on the M7 floor refresh
//! (`BASELINE_FLOOR_MARGIN`, now 4.0e-5): the floor may never be cut
//! below 2.315e-6, or the datum goes definite-and-clean at 1e-9 and
//! 1e-12 and the lint stops earning its name. Current clearance: 1.2
//! decades. `carrier_line_circle` is also deliberately NOT an
//! ε-coupled family — the #99 margin is a model-scale distance, so it
//! must answer to the metre floor.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Point2;
use geom_core::Tol;
use geom_core::k_stats::{self, Probe};
use k_lint::{BASELINE_FLOOR_MARGIN, Reason, is_eps_coupled, lint_sample};
use profile::RawLoop;
use profile::{Open, Profile, ProfileLoop, ProfileVertex, SketchPlane, Start, bulge_from_via};

fn p2(x: f64, y: f64) -> Point2<Probe> {
    Point2::new(Probe(x), Probe(y))
}

/// Validates a bracket loop and returns the smallest recorded
/// `carrier_line_circle` margin — the #99 statistic, measured by the
/// real predicates at the recording scalar (process ε = the compiled
/// default, 1e-9, where both variants validate with definite
/// outcomes).
fn carrier_line_circle_margin(lp: ProfileLoop<Probe>, tol: Tol) -> f64 {
    k_stats::start_recording();
    Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol)
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
    // RAW loop data, deliberately: this datum's whole point is a via
    // point that is NOT tangent, and the authoring lattice's junction
    // check refuses to mint a near-tangent junction at ε = 1e-6 — which
    // is exactly the escalation #99 was. The historical shape is a
    // vertex table, so it is written as one; the bulge comes from the
    // same public constructor the raw builder called.
    let (start, via, end) = (p2(1.5, 1.0), p2(1.146, 1.146), p2(1.0, 1.5));
    let v = |pos, bulge| ProfileVertex::new(pos, bulge);
    ProfileLoop::new(vec![
        v(p2(0.0, 0.0), Probe(0.0)),
        v(p2(3.0, 0.0), Probe(0.0)),
        v(p2(3.0, 1.0), Probe(0.0)),
        v(start, bulge_from_via(start, via, end)),
        v(end, Probe(0.0)),
        v(p2(1.0, 3.0), Probe(0.0)),
        v(p2(0.0, 3.0), Probe(0.0)),
    ])
}

/// The shipped bracket loop (post-#100/#101): the fillet constructor
/// computes the tangent arc exactly and DECLARES both joints (an
/// undeclared exact tangency would refuse typed — #101 discipline).
fn fixed_bracket_loop(tol: Tol) -> ProfileLoop<Probe> {
    // The shipped bracket, authored through the lattice: the corner is
    // reached by an exact axis director and the filleted side ends at
    // its authored far vertex, which is the spelling `profile`'s
    // differential suite pins bit-for-bit against the raw chain.
    Open.at(p2(0.0, 0.0))
        .line_to(p2(3.0, 0.0), tol)
        .unwrap()
        .line_to(p2(3.0, 1.0), tol)
        .unwrap()
        .toward(Probe(-1.0), Probe(0.0), tol)
        .unwrap()
        .fillet(Probe(0.5), tol)
        .expect("bracket fillet fits")
        .toward(Probe(0.0), Probe(1.0), tol)
        .unwrap()
        .to(p2(1.0, 3.0), tol)
        .unwrap()
        .line_to(p2(0.0, 3.0), tol)
        .unwrap()
        .line_to(Start, tol)
        .unwrap()
        .loop_
}

/// The supported ε rows, as (band_zero, band_escalate) with the
/// ratified K = 10 — the bands the sweep would record at each row
/// (margins are geometry; only the band moves with ε).
const EPS_ROWS: [(f64, f64); 3] = [(1e-6, 1e-5), (1e-9, 1e-8), (1e-12, 1e-11)];

#[test]
fn old_bracket_datum_lights_up_at_every_eps_row() {
    let tol = Tol::witness();
    assert_eq!(
        tol.eps(),
        1e-9,
        "litmus measures at the compiled default eps (run without \
         CAD_TOLERANCE_EPS)"
    );
    // Resurrect the pre-#100 datum: via point rounded to 1.146.
    let margin = carrier_line_circle_margin(old_bracket_loop(), tol);
    // Pin the resurrection: the historical margin, re-derived by the
    // #99 review in exact rationals, is 2.315e-6 m.
    assert!(
        (2.31e-6..2.32e-6).contains(&margin),
        "the resurrected #99 margin moved: {margin:e} (expected ~2.315e-6)"
    );
    // The floor refresh's binding lower bound, asserted rather than
    // documented: cut the floor below this datum and the two definite
    // rows below go clean.
    assert!(
        !is_eps_coupled("carrier_line_circle") && margin < BASELINE_FLOOR_MARGIN,
        "the baseline floor {BASELINE_FLOOR_MARGIN:e} no longer covers \
         the #99 datum {margin:e} — the litmus contract is broken"
    );

    for (zero, escalate) in EPS_ROWS {
        // The outcome the kernel records at this row: in-band at
        // ε = 1e-6 (that WAS #99's panic), definite above it elsewhere.
        let outcome = if margin > zero && margin < escalate {
            "indeterminate"
        } else {
            "positive"
        };
        let reasons = lint_sample("carrier_line_circle", margin, zero, escalate, outcome);
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
    let tol = Tol::witness();
    // The shipped fix: the constructive fillet (exact tangency,
    // declared by construction).
    let margin = carrier_line_circle_margin(fixed_bracket_loop(tol), tol);
    assert!(
        margin < 1e-15,
        "the fixed bracket's tangency margin must be rounding noise, \
         got {margin:e}"
    );
    for (zero, escalate) in EPS_ROWS {
        // Rounding noise classifies Zero at every supported ε.
        let reasons = lint_sample("carrier_line_circle", margin, zero, escalate, "zero");
        assert!(
            reasons.is_empty(),
            "the FIXED bracket must lint clean at eps={zero:e}: {reasons:?}"
        );
    }
}
