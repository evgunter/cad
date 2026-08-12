//! Cross-scalar decision consistency: validation at `Dual<f64>` makes
//! bit-identical decisions to plain `f64` (value-part delegation — a
//! derivative never influences a branch), and the k-stats `Probe`
//! scalar records the margin distribution without changing any
//! decision.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{
    annulus, arc_kisses_line, bowtie, lift, near_tangent_hole, profile, tangent_hole, tol,
};
use geom_core::{Dual, Dual64, Sign};
use profile::RawLoop;
use profile::k_stats::{self, Probe, SampleOutcome};
use profile::{LoopRole, SegmentKind, ValidatedProfile};

/// The decision skeleton of a canonical form: roles, per-segment kind
/// and turn, and the value-channel bits of every canonical vertex — the
/// part that must agree bit-for-bit across scalar instantiations.
type Skeleton = Vec<(bool, Vec<(u64, u64, u64, i8)>)>;

fn skeleton_f64(vp: &ValidatedProfile<f64>) -> Skeleton {
    vp.loops()
        .iter()
        .map(|lp| {
            (
                lp.role() == LoopRole::Outer,
                lp.vertices()
                    .iter()
                    .zip(lp.segments())
                    .map(|(v, s)| {
                        (
                            v.pos.x.to_bits(),
                            v.pos.y.to_bits(),
                            v.bulge.to_bits(),
                            kind_code(&s.kind),
                        )
                    })
                    .collect(),
            )
        })
        .collect()
}

fn skeleton_dual(vp: &ValidatedProfile<Dual64>) -> Skeleton {
    vp.loops()
        .iter()
        .map(|lp| {
            (
                lp.role() == LoopRole::Outer,
                lp.vertices()
                    .iter()
                    .zip(lp.segments())
                    .map(|(v, s)| {
                        (
                            v.pos.x.value.to_bits(),
                            v.pos.y.value.to_bits(),
                            v.bulge.value.to_bits(),
                            dual_kind_code(&s.kind),
                        )
                    })
                    .collect(),
            )
        })
        .collect()
}

fn turn_code(turn: Sign) -> i8 {
    match turn {
        Sign::Positive => 1,
        Sign::Negative => -1,
        Sign::Zero => 0,
    }
}

fn kind_code(k: &SegmentKind<f64>) -> i8 {
    match k {
        SegmentKind::Line => 0,
        SegmentKind::Arc { turn, .. } => turn_code(*turn),
    }
}

fn dual_kind_code(k: &SegmentKind<Dual64>) -> i8 {
    match k {
        SegmentKind::Line => 0,
        SegmentKind::Arc { turn, .. } => turn_code(*turn),
    }
}

#[test]
fn dual_accepts_exactly_like_f64_with_identical_canonical_values() {
    let base = annulus();
    let f = base.validate(tol()).expect("annulus validates at f64");
    let d = lift::<Dual64>(&base)
        .validate(tol())
        .expect("annulus validates at Dual<f64>");
    assert_eq!(skeleton_f64(&f), skeleton_dual(&d));
}

#[test]
fn dual_rejects_with_the_identical_typed_error() {
    // ProfileError is scalar-free, so the errors must be *equal* —
    // including the escalation's margin diagnostic (the value channel
    // is bit-identical by construction).
    for fixture in [
        profile(vec![bowtie()]),
        tangent_hole(),
        near_tangent_hole(tol().eps),
        arc_kisses_line(),
    ] {
        let f_err = fixture.validate(tol()).expect_err("fixture rejects at f64");
        let d_err = lift::<Dual64>(&fixture)
            .validate(tol())
            .expect_err("fixture rejects at Dual<f64>");
        assert_eq!(f_err, d_err);
    }
}

#[test]
fn dual_with_seeded_derivatives_still_decides_by_value_only() {
    // Poisoned/adversarial derivative channels must not affect any
    // decision: seed every coordinate's derivative with NaN.
    let base = annulus();
    let seeded = profile::Profile::new(
        profile::SketchPlane::xy(),
        base.loops
            .iter()
            .map(|lp| {
                profile::ProfileLoop::new(
                    lp.vertices
                        .iter()
                        .map(|v| profile::ProfileVertex {
                            pos: geom_core::Point2::new(
                                Dual::new(v.pos.x, f64::NAN),
                                Dual::new(v.pos.y, f64::NAN),
                            ),
                            bulge: Dual::new(v.bulge, f64::NAN),
                        })
                        .collect(),
                )
            })
            .collect(),
    );
    let f = base.validate(tol()).expect("annulus validates at f64");
    let d = seeded
        .validate(tol())
        .expect("NaN derivatives must not poison decisions");
    assert_eq!(skeleton_f64(&f), skeleton_dual(&d));
}

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
    let eps = tol().eps;
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

/// **K-funnel registration (M5 S2)**: every gate the arc-leg fillet
/// constructor takes is a *reified* predicate through the same k_stats
/// recorder validation uses — so the constructor's margins land in the
/// K-experiment distribution with the rest of the kernel's. This pins
/// that all seven fire under recording, with band semantics intact.
#[test]
fn probe_records_every_arc_fillet_gate() {
    use geom_core::{Point2, Real};
    use profile::{ArcSweep, FilletLegShape, ProfileLoop};

    let pp = |x: f64, y: f64| Point2::new(Probe::from_f64(x), Probe::from_f64(y));
    let leg_arc = |cx: f64, cy: f64, sweep| FilletLegShape::Arc {
        center: pp(cx, cy),
        sweep,
    };
    k_stats::start_recording();
    // line×arc: fires the arm, turn, line/circle-offset, reach and fit
    // gates.
    ProfileLoop::builder(pp(0.0, 0.0))
        .fillet_corner(
            FilletLegShape::Line,
            pp(2.0, 0.0),
            leg_arc(0.0, 0.0, ArcSweep::Ccw),
            pp(0.0, 2.0),
            Probe::from_f64(0.5),
            tol(),
        )
        .expect("the line×arc fillet constructs at Probe");
    // arc×arc: fires the two circle-offset clearances as well.
    let s3 = 3.0f64.sqrt();
    ProfileLoop::builder(pp(1.0, 0.0))
        .fillet_corner(
            leg_arc(-1.0, 0.0, ArcSweep::Ccw),
            pp(0.0, s3),
            leg_arc(1.0, 0.0, ArcSweep::Ccw),
            pp(-1.0, 0.0),
            Probe::from_f64(0.5),
            tol(),
        )
        .expect("the arc×arc fillet constructs at Probe");
    let samples = k_stats::take_samples();
    for name in [
        "fillet_corner_arm",
        "fillet_corner_turn",
        "fillet_offset_line_circle",
        "fillet_offset_circles_external",
        "fillet_offset_circles_internal",
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
