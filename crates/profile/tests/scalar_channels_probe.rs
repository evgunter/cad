//! The Probe-lane half of `scalar_channels.rs`: the k-stats `Probe`
//! scalar records the margin distribution without changing any
//! decision, and every reified gate lands in that distribution.
//!
//! Split out because `Probe` — the K-telemetry recording scalar — is
//! gated behind the `probe` cargo feature: it is a `Real` instantiation,
//! so every generic-over-`Real` body monomorphizes at it, and the
//! default build has no reason to pay for a diagnostics scalar. The
//! `Dual<f64>` cross-scalar tests stay ungated in `scalar_channels.rs`;
//! only this file carries the whole-file gate.
//!
//! **CI EXECUTES THIS SUITE.** It is rostered in
//! `scripts/gates/probe-suite-census.sh` (`RUN_FLOOR`) and run under the
//! DEFAULT selection by `scripts/k_probe_sweep.sh`, whose tally is floored
//! by `--check-executed`, so every assertion below is a gate and a red here
//! fails the merge. By hand:
//! `cargo test -p profile --features probe --test all -- scalar_channels_probe::`.

#![cfg(feature = "probe")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{annulus, lift, near_tangent_hole, profile, tol};
use geom_core::Sign;
use geom_core::Tol;
use geom_core::k_stats::{self, Probe, SampleOutcome};

#[test]
fn probe_records_margin_distributions_without_changing_decisions() {
    // Two accepting fixtures so every predicate fires: the all-arc
    // annulus and a plate (line outer, arc hole — line/arc pairs and
    // rays through lines).
    let plate = profile(vec![
        common::rect(0.0, 0.0, 6.0, 6.0),
        common::circle_h(3.0, 3.0, 1.0),
    ]);
    k_stats::start_recording();
    let outcome = lift::<Probe>(&annulus()).validate(tol());
    let outcome2 = lift::<Probe>(&plate).validate(tol());
    let samples = k_stats::take_samples();
    assert!(outcome.is_ok(), "Probe must decide exactly like f64");
    assert!(outcome2.is_ok(), "Probe must decide exactly like f64");
    assert!(!samples.is_empty(), "validation must have recorded margins");

    // The recording is per-predicate and internally consistent with the
    // band semantics.
    for s in &samples {
        assert!((s.band_escalate / s.band_zero - 10.0).abs() < 1e-9 || s.band_zero < 1e-300);
        match s.outcome {
            SampleOutcome::Definite(Sign::Zero) => assert!(s.margin.abs() <= s.band_zero),
            SampleOutcome::Definite(_) => assert!(s.margin.abs() >= s.band_escalate),
            SampleOutcome::Indeterminate => {
                assert!(s.margin.abs() > s.band_zero && s.margin.abs() < s.band_escalate);
            }
            SampleOutcome::Invalid => assert!(s.margin.is_nan()),
        }
    }
    // The suite's expected predicates all fired.
    for name in [
        "vertex_separation",
        "segment_straightness",
        "arc_diameter_clearance",
        "carrier_circles_identity",
        "carrier_circles_internal",
        "arc_span",
        "arc_apex_identity",
        "contact_at_shared_vertex",
        "ray_side",
        "ray_advance",
        "carrier_line_circle",
        "loop_orientation",
        "canonical_order_x",
        "canonical_order_y",
        "chord_side",
        "line_span",
    ] {
        assert!(
            samples.iter().any(|s| s.predicate == name),
            "no sample recorded for predicate '{name}'"
        );
    }
    // A healthy fixture records no in-band or poisoned margins.
    assert!(samples.iter().all(|s| !matches!(
        s.outcome,
        SampleOutcome::Indeterminate | SampleOutcome::Invalid
    )));
}

#[test]
fn probe_records_the_near_tangent_escalation() {
    let eps = tol().eps();
    k_stats::start_recording();
    let outcome = lift::<Probe>(&near_tangent_hole(eps)).validate(tol());
    let samples = k_stats::take_samples();
    assert!(outcome.is_err());
    let hit = samples
        .iter()
        .find(|s| {
            s.predicate == "carrier_circles_internal" && s.outcome == SampleOutcome::Indeterminate
        })
        .expect("the in-band internal clearance must be recorded");
    assert!((hit.margin + 5.0 * eps).abs() < 0.5 * eps);
}

#[test]
fn recording_is_opt_in() {
    // Without start_recording, validation at Probe records nothing.
    let _ = lift::<Probe>(&annulus()).validate(tol());
    assert!(k_stats::take_samples().is_empty());
}

/// **K-funnel registration (M5 S2)**: every gate the arc-carrier fillet
/// takes is a *reified* predicate through the same k_stats recorder
/// validation uses — so the corner's margins land in the K-experiment
/// distribution with the rest of the kernel's. This pins that all eight
/// fire under recording, with band semantics intact.
///
/// Neither corner is closed: an interior arc arrival leaves the tip ON
/// its carrier, which is all these rows need — the gates fire inside
/// the fused verb, before any closing segment exists.
#[test]
fn probe_records_every_arc_fillet_gate() {
    use geom_core::{Point2, Real};
    use profile::{ArcSweep, Center, Open};

    let pp = |x: f64, y: f64| Point2::new(Probe::from_f64(x), Probe::from_f64(y));
    let pr = Probe::from_f64;
    k_stats::start_recording();
    // line×arc: fires the arm, turn, line/circle-offset, reach and fit
    // gates. The corner (2, 0) is the +x ray from (0, 0) meeting the
    // carrier about the origin through (0, 2).
    Open.at(pp(0.0, 0.0))
        .toward(pr(1.0), pr(0.0), Tol::witness())
        .expect("the incoming ray runs +x")
        .fillet_arc(
            pr(0.5),
            Center {
                c: pp(0.0, 0.0),
                winding: ArcSweep::Ccw,
                p: pp(0.0, 2.0),
            },
            Tol::witness(),
        )
        .expect("the line×arc fillet constructs at Probe");
    // arc×arc: the vesica of two radius-2 carriers whose centres are
    // two apart, which fires the two circle-offset clearances as well.
    Open.arc_fillet_arc(
        Center {
            c: pp(-1.0, 0.0),
            winding: ArcSweep::Ccw,
            p: pp(1.0, 0.0),
        },
        pr(0.5),
        Center {
            c: pp(1.0, 0.0),
            winding: ArcSweep::Ccw,
            p: pp(-1.0, 0.0),
        },
        Tol::witness(),
    )
    .expect("the arc×arc fillet constructs at Probe");
    let samples = k_stats::take_samples();
    for name in [
        "fillet_corner_arm",
        "fillet_corner_turn",
        "fillet_offset_line_circle",
        "fillet_offset_circles_external",
        "fillet_offset_circles_internal",
        "fillet_offset_lever",
        "fillet_leg_reach",
        "fillet_leg_fit",
    ] {
        assert!(
            samples.iter().any(|s| s.predicate == name),
            "no sample recorded for predicate '{name}'"
        );
    }
    // Healthy corners record no in-band or poisoned margins, and every
    // sample obeys its own band.
    for s in &samples {
        match s.outcome {
            SampleOutcome::Definite(Sign::Zero) => assert!(s.margin.abs() <= s.band_zero),
            SampleOutcome::Definite(_) => assert!(s.margin.abs() >= s.band_escalate),
            other => panic!("unexpected outcome {other:?} for '{}'", s.predicate),
        }
    }
}
