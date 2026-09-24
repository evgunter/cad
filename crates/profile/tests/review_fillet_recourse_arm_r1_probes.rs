//! **Review probes (R1) for the fillet recourse arm.**
//!
//! What the followability suite pins is read here through fixtures of
//! this reviewer's own, and the two pre-emption claims that rested on a
//! point sweep rather than a proof are attacked with the parameter the
//! sweeps held fixed: the LEVER ARM for the turn gate (its margin is
//! `sin(turn) · arm`, so it is in band whenever the arm is short and
//! the turn is not — a corner that is not degenerate at all), and the
//! offset radius for the conditioning gate (its margin is
//! `|ρ₂| − least_lever`, so a lens whose small offset circle sits ON
//! the large one puts it in band while both clearances stay definite).
//!
//! Every fixture is placed in units of the run's ε, so each row holds
//! at every tolerance the run can be given.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::{
    ArcSweep, Center, FILLET_ENCLOSING_RECOURSE, FILLET_NO_CORNER_RECOURSE,
    FILLET_OFFSET_LEVER_RECOURSE, FILLET_TURN_INBAND_RECOURSE, Open, PathError, Profile,
    ProfileLoop, SketchPlane, Start, Step, replay_guided, replay_recording,
};

fn tol() -> Tol {
    Tol::witness()
}
fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

type Built = Result<ProfileLoop<f64>, PathError<f64>>;

/// The escalation's predicate, or a panic naming what came instead.
fn escalates_under(err: &PathError<f64>, what: &str) -> &'static str {
    match err {
        PathError::Escalated { source } => source.predicate.expect("a named predicate"),
        other => panic!("{what}: expected an in-band escalation, got {other:?}"),
    }
}

/// What an in-band fillet verdict must read like at the door.
fn reads_its_own_sentence(err: &PathError<f64>, sentence: &str, what: &str) {
    let shown = err.to_string();
    assert!(
        shown.starts_with("the fillet at this corner is undecided"),
        "{what}: must name the site.\n  got: {shown}"
    );
    assert!(
        shown.contains(sentence),
        "{what}: must carry the gate's sentence.\n  got: {shown}"
    );
    assert!(
        !shown.contains(geom_core::COINCIDENCE_RECOURSE),
        "{what}: must not carry the shared recourse.\n  got: {shown}"
    );
}

fn builds_and_validates(lp: Built, what: &str) {
    let lp = lp.unwrap_or_else(|e| panic!("{what}: must build, got {e:?}"));
    Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol())
        .unwrap_or_else(|e| panic!("{what}: must validate, got {e:?}"));
}

// ------------------------------------------------------------ fixtures

/// **line × circle**, radius-3 carrier: the offset line `y = r` and the
/// offset circle `3 − r` touch at `r = 1.5`; the margin is `3 − 2r`.
fn line_arc_r3(radius: f64) -> Built {
    Open.at(p2(0.0, 3.0))
        .line_to(p2(0.0, 0.0), tol())?
        .toward(1.0, 0.0, tol())?
        .fillet_arc(
            radius,
            Center {
                c: p2(0.0, 0.0),
                winding: ArcSweep::Ccw,
                p: Start,
            },
            tol(),
        )
        .map(|c| c.loop_)
}

/// **Two unit lobes about (±0.6, 0)**: the external clearance is
/// `2(1 − r) − 1.2 = 0.8 − 2r`, closing at `r = 0.4`; the enclosing
/// gate's ρ = 1 − r crosses zero at `r = 1`.
fn lobes_06(radius: f64) -> Built {
    let tip = 0.64f64.sqrt();
    Open.arc_fillet_arc(
        Center {
            c: p2(-0.6, 0.0),
            winding: ArcSweep::Ccw,
            p: p2(0.0, -tip),
        },
        radius,
        Center {
            c: p2(0.6, 0.0),
            winding: ArcSweep::Ccw,
            p: Start,
        },
        tol(),
    )
    .map(|c| c.loop_)
}

/// **Mixed winding, radius-1.5 carriers about (±0.8, 0)**: offsets go
/// to `1.5 + r` and `1.5 − r`, so the internal clearance is `1.6 − 2r`.
fn mixed_08(radius: f64) -> Built {
    Open.arc_fillet_arc(
        Center {
            c: p2(-0.8, 0.0),
            winding: ArcSweep::Ccw,
            p: p2(0.7, 0.0),
        },
        radius,
        Center {
            c: p2(0.8, 0.0),
            winding: ArcSweep::Cw,
            p: p2(2.3, 0.0),
        },
        tol(),
    )?
    .line_to(Start, tol())
    .map(|c| c.loop_)
}

/// **A short straight leg meeting a radius-2 circle at a real angle.**
/// The ray `y = 2 − delta` heads east from `a` behind its first crossing
/// with the circle about the origin. At that crossing the leg and the
/// circle's tangent meet at `sin θ = √delta`; the fillet's lever arm is
/// the straight leg's extent `a`, so the turn gate's levered margin is
/// `a · √delta`.
fn short_leg_at_angle(a: f64, delta: f64, radius: f64) -> Built {
    let y = 2.0 - delta;
    let cx = -(4.0 - y * y).sqrt();
    Open.at(p2(cx - a, y))
        .toward(1.0, 0.0, tol())?
        .fillet_arc(
            radius,
            Center {
                c: p2(0.0, 0.0),
                winding: ArcSweep::Ccw,
                p: p2(0.0, -2.0),
            },
            tol(),
        )?
        .line_to(p2(-3.0, -3.0), tol())?
        .line_to(p2(-3.0, 3.0), tol())?
        .line_to(Start, tol())
        .map(|c| c.loop_)
}

/// **The lever lens.** Two carriers of radius `big`, mixed winding, so
/// the offsets are `big + r` and `big − r = rho2`; the centres sit
/// `d = big + r` apart, which puts the small offset circle's centre ON
/// the large one — both clearances equal `rho2` and are definite while
/// the conditioning margin `rho2 − least_lever` is whatever `rho2` is
/// set to.
fn lever_lens(big: f64, rho2: f64) -> Built {
    let r = big - rho2;
    let d = big + r;
    Open.arc_fillet_arc(
        Center {
            c: p2(0.0, 0.0),
            winding: ArcSweep::Ccw,
            p: p2(0.0, -big),
        },
        r,
        Center {
            c: p2(d, 0.0),
            winding: ArcSweep::Cw,
            p: p2(d, big),
        },
        tol(),
    )?
    .line_to(p2(d, 2.0 * big), tol())?
    .line_to(p2(-1.5 * big, 2.0 * big), tol())?
    .line_to(p2(-1.5 * big, -1.5 * big), tol())?
    .line_to(Start, tol())
    .map(|c| c.loop_)
}

/// The gate's own least lever for the lens (`sugar.rs`'s law with its
/// shipped constant), found by fixed point since it depends weakly on
/// `rho2` through the scene scale.
fn least_lever(big: f64, eps: f64) -> f64 {
    let law = |rho2: f64| {
        let r = big - rho2;
        let d = big + r;
        let rho1 = big + r;
        let scale2 = d * d + rho1 * rho1 + rho2 * rho2;
        128.0 * f64::EPSILON * big * scale2 / (d * eps)
    };
    let mut l = law(0.0);
    for _ in 0..4 {
        l = law(l);
    }
    l
}

// ------------------------------------------------------------------ rows

/// The four reachable gates, in band through this reviewer's fixtures,
/// each read off the door and each followed to a build.
#[test]
fn the_four_reachable_gates_render_their_own_sentence_on_other_fixtures() {
    let eps = tol().eps();

    let e = line_arc_r3(1.5 - 2.5 * eps).expect_err("line x circle in band");
    assert_eq!(
        escalates_under(&e, "line x circle"),
        "fillet_offset_line_circle"
    );
    reads_its_own_sentence(&e, FILLET_NO_CORNER_RECOURSE, "line x circle");
    builds_and_validates(line_arc_r3(0.75), "the smaller radius");

    let e = lobes_06(0.4 - 2.5 * eps).expect_err("external clearance in band");
    assert_eq!(
        escalates_under(&e, "external"),
        "fillet_offset_circles_external"
    );
    reads_its_own_sentence(&e, FILLET_NO_CORNER_RECOURSE, "external");
    builds_and_validates(lobes_06(0.3), "the smaller radius");

    let e = mixed_08(0.8 - 2.5 * eps).expect_err("internal clearance in band");
    assert_eq!(
        escalates_under(&e, "internal"),
        "fillet_offset_circles_internal"
    );
    reads_its_own_sentence(&e, FILLET_NO_CORNER_RECOURSE, "internal");
    builds_and_validates(mixed_08(0.4), "the smaller radius");

    let e = lobes_06(1.0 + 5.0 * eps).expect_err("enclosing in band");
    assert_eq!(escalates_under(&e, "enclosing"), "fillet_enclosing_carrier");
    reads_its_own_sentence(&e, FILLET_ENCLOSING_RECOURSE, "enclosing");
    builds_and_validates(lobes_06(0.3), "the radius moved clearly downward");
}

/// **`fillet_corner_turn` IS reachable in band through the public
/// door** — the pre-emption claim held only while the lever arm was
/// long. With `a = 500ε` and `delta = 1e-4` the carriers meet
/// definitely (margin `1e-4`), the advance gate passes (margin `500ε`),
/// and the turn gate's levered margin `500ε · 0.01 = 5ε` is in band.
///
/// The sentence it then renders says the corner is degenerate. It is
/// not: the legs meet at `asin(0.01)`, and the same corner with a leg
/// long enough for the setback builds — the lever the caller has is the
/// leg's extent, which the sentence never names. The definite sibling
/// has the same shape: at `a = 50ε` the margin `0.5ε` classifies Zero
/// and the door refuses the corner as ALREADY TANGENT.
#[test]
fn the_turn_gate_is_reachable_in_band_with_a_short_leg_and_a_real_angle() {
    let eps = tol().eps();
    let err = short_leg_at_angle(500.0 * eps, 1e-4, 0.05).expect_err("the turn margin is in band");
    assert_eq!(escalates_under(&err, "short leg"), "fillet_corner_turn");
    reads_its_own_sentence(&err, FILLET_TURN_INBAND_RECOURSE, "short leg");
    // The corner is real: the identical carriers with a leg long enough
    // to take the setback build and validate.
    builds_and_validates(short_leg_at_angle(1.0, 1e-4, 0.05), "the longer leg");
    // The definite sibling below the band: the same real corner refused
    // as tangent.
    let zero =
        short_leg_at_angle(50.0 * eps, 1e-4, 0.05).expect_err("the turn margin is below zero");
    assert!(
        matches!(&zero, PathError::NoCornerOfPair { .. })
            || !matches!(&zero, PathError::Escalated { .. }),
        "expected a definite refusal, got {zero:?}"
    );
    let shown = zero.to_string();
    assert!(
        !shown.contains(FILLET_TURN_INBAND_RECOURSE),
        "definite arm: {shown}"
    );
}

/// **`fillet_corner_arm` is pre-empted on the same number**: a straight
/// leg's arm IS its advance margin, so an arm in band is an advance in
/// band, and `path_corner_advance` answers first.
#[test]
fn the_arm_gate_is_pre_empted_by_the_advance_gate_on_the_same_margin() {
    let eps = tol().eps();
    for k in [1.5, 5.0, 9.0] {
        let err = short_leg_at_angle(k * eps, 1e-2, 0.05).expect_err("the advance is in band");
        assert_eq!(escalates_under(&err, "arm in band"), "path_corner_advance");
    }
}

/// **`fillet_offset_lever` IS reachable in band through the public
/// door, at every ε row.** The lens puts the conditioning margin at
/// `±5ε` with both clearances definite (`rho2 ≈ least_lever ≫ K·ε`) and
/// the enclosing gate definite. The carrier radius scales with ε so the
/// least lever (∝ `R²/ε`) stays a fixed fraction of the scene.
#[test]
fn the_lever_gate_is_reachable_in_band_on_the_lens_and_renders_its_sentence() {
    let eps = tol().eps();
    let big = 3e10 * eps;
    let least = least_lever(big, eps);
    for k in [-5.0, 5.0] {
        let err = lever_lens(big, least + k * eps).expect_err("the lever margin is in band");
        assert_eq!(escalates_under(&err, "lever lens"), "fillet_offset_lever");
        reads_its_own_sentence(&err, FILLET_OFFSET_LEVER_RECOURSE, "lever lens");
    }
    // The definite sibling, and the sentence's own request followed:
    // the radius moved away from the carrier radius builds.
    let short = lever_lens(big, least - 50.0 * eps).expect_err("below the least lever");
    assert!(
        matches!(short, PathError::FilletOffsetLeverTooShort { .. }),
        "expected the definite sibling, got {short:?}"
    );
    builds_and_validates(lever_lens(big, big / 10.0), "the radius moved clearly away");
}

/// **The guided replay door renders the shared coincidence recourse for
/// an in-band fillet gate.** `StructureRefusal`'s indeterminate arm
/// prints the `Indeterminate` whole, and that Display ends in
/// `COINCIDENCE_RECOURSE` — so through `replay_guided` the caller
/// reads "declare the coincidence" at a fillet they asked for, and the
/// gate's own sentence is nowhere in the text. A characterization.
#[test]
fn a_guided_replay_still_renders_the_coincidence_recourse_for_a_fillet_gate() {
    let eps = tol().eps();
    let closed = Open
        .at(p2(0.0, 3.0))
        .line_to(p2(0.0, 0.0), tol())
        .unwrap()
        .toward(1.0, 0.0, tol())
        .unwrap()
        .fillet_arc(
            0.75,
            Center {
                c: p2(0.0, 0.0),
                winding: ArcSweep::Ccw,
                p: Start,
            },
            tol(),
        )
        .unwrap();
    let (_, structure) = replay_recording(&closed.program, tol()).expect("records");
    let mut steps = closed.program.clone();
    for s in &mut steps {
        if let Step::FilletArc { radius, .. } = s {
            *radius = 1.5 - 2.5 * eps;
        }
    }
    let err = replay_guided(&steps, &structure, tol()).expect_err("in band under guidance");
    let shown = err.to_string();
    assert!(
        shown.contains("'fillet_offset_line_circle'"),
        "names the gate: {shown}"
    );
    assert!(
        shown.contains(geom_core::COINCIDENCE_RECOURSE),
        "the guided door has stopped rendering the shared recourse — flip this row to a pin: {shown}"
    );
    assert!(
        !shown.contains(FILLET_NO_CORNER_RECOURSE),
        "the guided door has started rendering the gate's sentence — flip this row to a pin: {shown}"
    );
}
