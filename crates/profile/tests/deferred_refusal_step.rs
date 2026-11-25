//! **A refusal is filed against the step its recourse edits.** A
//! `fillet(r)` is resolved by the step that fixes its arrival, and every
//! fillet arc's stored form is re-read where the loop closes; either can
//! be steps past the fillet. A refusal whose levers include the fillet's
//! own is the fillet's: the replay files it there ([`ReplayError::step`])
//! and its sentence names the fillet's step and the later step that
//! raised it ([`PathError::Deferred`]). One whose only lever is the
//! arrival's, the arrival step's own arguments, and a junction's, stay
//! the raising step's. The drawing replay files the fillet's
//! construction at the fillet's step, which is how the two match.

#![allow(clippy::expect_used, clippy::panic)]

use crate::common;

use common::tol;
use geom_core::{Point2, Tol};
use profile::path::program::replay_drawn;
use profile::{
    ArcData, CornerReason, CornerWindow, Deferral, Open, PathError, PathErrorKind, ReplayError,
    ReplayErrorKind, Start, Step, Target, replay,
};

const QUARTER: f64 = core::f64::consts::FRAC_PI_2;

/// The step the fillet below is bound at.
const FILLET: usize = 3;
/// The step that fixes its arrival, and resolves it.
const ARRIVAL: usize = 5;

fn p(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// A radius-5 fillet on a quarter turn whose legs are 1 and 0.5 long:
/// its setback, 5, outruns both, so the fit refuses at its arrival, and
/// its levers are both anchors and the radius.
fn outrun() -> Vec<Step<f64>> {
    vec![
        Step::At(p(0.0, 0.0)),
        Step::Angle(0.0),
        Step::Line(1.0),
        Step::Fillet { radius: 5.0 },
        Step::Angle(QUARTER),
        Step::At(p(2.0, 0.5)),
        Step::Line(1.0),
        Step::LineTo(Target::Start),
    ]
}

/// The refusal's attribution and the refusal under it, or a panic
/// naming what arrived instead.
fn deferred(err: &PathError<f64>) -> (usize, usize, Deferral, &PathError<f64>) {
    match err {
        PathError::Deferred {
            step,
            raised_at,
            deferral,
            refusal,
        } => (*step, *raised_at, *deferral, refusal),
        other => panic!("expected a refusal filed against an earlier step, got {other}"),
    }
}

fn path_refusal(err: &ReplayError<f64>) -> &PathError<f64> {
    match &err.kind {
        ReplayErrorKind::Path(source) => source,
        ReplayErrorKind::Transition { .. } => panic!("expected a geometry refusal, got {err}"),
    }
}

/// A refusal the raising step keeps as its own: filed at the step the
/// replay was applying, with nothing deferred.
fn own_refusal(steps: &[Step<f64>], at: usize, what: &str) -> PathError<f64> {
    let err = replay(steps, Tol::witness()).expect_err(what);
    assert_eq!((err.step(), err.raised_at()), (at, at), "{what}: {err}");
    let source = path_refusal(&err).clone();
    assert_eq!(source.step(), None, "{what}: nothing deferred: {source}");
    source
}

#[test]
fn a_fillet_refused_at_its_arrival_is_filed_against_its_own_step() {
    let err = replay(&outrun(), Tol::witness()).expect_err("the setback outruns both legs");
    assert_eq!(
        err.step(),
        FILLET,
        "the replay files it at the fillet: {err}"
    );
    assert_eq!(err.raised_at(), ARRIVAL, "raised by the arrival: {err}");
    let source = path_refusal(&err);
    let (step, raised_at, deferral, refusal) = deferred(source);
    assert_eq!(
        (step, raised_at, deferral),
        (FILLET, ARRIVAL, Deferral::FilletResolution),
        "{err}"
    );
    assert!(
        matches!(refusal, PathError::NoCornerOfPair { .. }),
        "the fit's own refusal, unchanged under the attribution: {refusal}"
    );
    assert!(core::ptr::eq(source.refusal(), refusal), "{source}");
    assert_eq!(
        source.kind(),
        PathErrorKind::NoCornerOfPair,
        "its class is the fit's"
    );
    let source_of = std::error::Error::source(source).map(ToString::to_string);
    assert_eq!(source_of, Some(refusal.to_string()), "the wrapped refusal");
    // Whole on its own: it names the fillet's step and the arrival's.
    assert_eq!(
        source.to_string(),
        format!("step 3's fillet, not resolved at step 5: {refusal}"),
    );
    assert_eq!(err.to_string(), source.to_string(), "not named twice");

    // The drawing replay files the refused fit's construction at the
    // same step as the refusal, so the editor matches them by it.
    let (drawn, built) = replay_drawn(&outrun(), Tol::witness());
    let drawn = drawn.expect_err("the drawing replay refuses the same way");
    let refused = built.last().expect("the fillet step's construction");
    assert_eq!((refused.step, drawn.step()), (FILLET, FILLET), "{drawn}");

    // The typed surface the replay drives raises the same refusal.
    let t = Tol::witness();
    let typed = Open
        .at(p(0.0, 0.0))
        .angle(0.0, t)
        .and_then(|x| x.line(1.0, t))
        .and_then(|x| x.fillet(5.0, t))
        .and_then(|x| x.angle(QUARTER, t))
        .and_then(|x| x.at(p(2.0, 0.5), t))
        .expect_err("the same fit refuses on the typed surface");
    assert_eq!(typed.to_string(), source.to_string());
}

/// The stored-form row is the shape the viewer reported: the fillet at
/// step 2, refused by the close at step 8 re-reading the stored arc.
const STORED_FILLET: usize = 2;
const CLOSE: usize = 8;

/// A fillet on a milliradian corner whose arc is half ε long: built,
/// and refused where the close re-reads its stored form, since its two
/// stored vertices coincide.
fn flattened() -> Vec<Step<f64>> {
    let theta = 1e-3;
    let radius = 0.5 * tol().eps() / theta;
    vec![
        Step::At(p(0.0, 0.0)),
        Step::Angle(0.0),
        Step::Fillet { radius },
        Step::At(p(4.0 + 3.0 * theta.cos(), 3.0 * theta.sin())),
        Step::Angle(theta),
        Step::Line(1.0),
        Step::LineTo(Target::Point(p(8.0, 2.0))),
        Step::LineTo(Target::Point(p(4.0, 3.0))),
        Step::LineTo(Target::Start),
    ]
}

#[test]
fn a_fillet_its_stored_form_cannot_hold_is_filed_against_its_own_step_at_the_close() {
    let steps = flattened();
    assert_eq!(steps.len(), CLOSE + 1, "the close is the last step");
    let err = replay(&steps, tol()).expect_err("an arc shorter than ε refuses at the close");
    assert_eq!(
        (err.step(), err.raised_at()),
        (STORED_FILLET, CLOSE),
        "filed at the fillet, raised at the close: {err}"
    );
    let (step, raised_at, deferral, refusal) = deferred(path_refusal(&err));
    assert_eq!(
        (step, raised_at, deferral),
        (STORED_FILLET, CLOSE, Deferral::FilletStoredForm),
        "{err}"
    );
    assert!(
        matches!(refusal, PathError::FilletArcFlattenedInStorage { .. }),
        "the stored-form refusal, unchanged under the attribution: {refusal}"
    );
    assert_eq!(
        err.to_string(),
        format!("step 2's fillet, not stored at the close: {refusal}"),
    );

    let t = tol();
    let theta = 1e-3;
    let typed = Open
        .at(p(0.0, 0.0))
        .angle(0.0, t)
        .and_then(|x| x.fillet(0.5 * t.eps() / theta, t))
        .and_then(|x| x.at(p(4.0 + 3.0 * theta.cos(), 3.0 * theta.sin()), t))
        .and_then(|x| x.angle(theta, t))
        .and_then(|x| x.line(1.0, t))
        .and_then(|x| x.line_to(p(8.0, 2.0), t))
        .and_then(|x| x.line_to(p(4.0, 3.0), t))
        .and_then(|x| x.line_to(Start, t))
        .expect_err("the same close refuses on the typed surface");
    assert_eq!(typed.to_string(), path_refusal(&err).to_string());
}

/// What the arrival step refuses of its own arguments is its own,
/// though a fillet is open across it: a zero director at the step whose
/// direction would resolve the fillet.
#[test]
fn the_resolving_steps_own_argument_fault_is_its_own() {
    let mut steps = outrun();
    steps[ARRIVAL - 1] = Step::At(p(2.0, 0.5));
    steps[ARRIVAL] = Step::Toward { dx: 0.0, dy: 0.0 };
    let source = own_refusal(&steps, ARRIVAL, "a zero director names no direction");
    assert_eq!(source.kind(), PathErrorKind::ZeroDirection, "{source}");
}

/// A corner behind the arrival's own anchor is resolved inside the
/// fillet's resolution, but its one lever is the arrival's anchor
/// ("anchor the arrival side beyond the corner"), so it is the
/// arrival's refusal.
#[test]
fn a_corner_behind_the_arrival_anchor_is_the_arrivals_refusal() {
    let mut steps = outrun();
    steps[FILLET] = Step::Fillet { radius: 0.2 };
    // The arrival runs up x = 2 from (2, -0.5): the corner (2, 0) is
    // ahead of its anchor, not behind it.
    steps[ARRIVAL] = Step::At(p(2.0, -0.5));
    let source = own_refusal(&steps, ARRIVAL, "the corner is past the arrival's anchor");
    let PathError::NoCornerOfPair { corners, .. } = &source else {
        panic!("expected the corner envelope, got {source}")
    };
    assert!(
        corners.iter().all(|c| matches!(
            c.reason,
            CornerReason::OutsideAnchors(CornerWindow::BehindArrivalAnchor)
        )),
        "{source}"
    );
}

/// A sharp arc leg reached while a fillet is open is the step order's
/// refusal at that leg, not the fillet's.
#[test]
fn an_arc_leg_on_an_open_fillet_is_the_legs_refusal() {
    let steps = [
        Step::At(p(0.0, 0.0)),
        Step::Angle(0.0),
        Step::Line(4.0),
        Step::Fillet { radius: 0.5 },
        Step::At(p(6.0, 2.0)),
        Step::ArcTo(ArcData::Bulge {
            target: Target::Point(p(6.0, 6.0)),
            b: 0.2,
        }),
        Step::LineTo(Target::Start),
    ];
    let source = own_refusal(&steps, 5, "an arc leg while the fillet is open");
    assert_eq!(source.kind(), PathErrorKind::ArcLegOnOpenFillet, "{source}");
}

/// With two fillets in one loop, the second's refusal names the second:
/// the first resolved, and the second's arrival two steps later raises
/// its outrun.
#[test]
fn the_second_of_two_fillets_is_named_when_it_refuses() {
    let steps = [
        Step::At(p(0.0, 0.0)),
        Step::Angle(0.0),
        Step::Line(4.0),
        Step::Fillet { radius: 0.5 },
        Step::Angle(QUARTER),
        Step::At(p(5.0, 1.0)),
        Step::Line(3.0),
        Step::Fillet { radius: 9.0 },
        Step::Angle(2.0 * QUARTER),
        Step::At(p(0.0, 6.0)),
        Step::LineTo(Target::Start),
    ];
    let err = replay(&steps, Tol::witness()).expect_err("radius 9 outruns legs of 2 and 5");
    assert_eq!((err.step(), err.raised_at()), (7, 9), "{err}");
    let (_, _, deferral, refusal) = deferred(path_refusal(&err));
    assert_eq!(deferral, Deferral::FilletResolution, "{err}");
    assert_eq!(refusal.kind(), PathErrorKind::NoCornerOfPair, "{err}");
}

/// The seam fillet is resolved by the close itself, which its sentence
/// calls the close rather than by a number.
#[test]
fn a_seam_fillet_refused_by_the_close_says_the_close() {
    let steps = [
        Step::At(p(0.0, 0.0)),
        Step::Angle(0.0),
        Step::Line(4.0),
        Step::LineTo(Target::Point(p(4.0, 4.0))),
        Step::LineTo(Target::Point(p(2.0, 3.0))),
        Step::Fillet { radius: 50.0 },
        Step::CloseTo,
    ];
    let err = replay(&steps, Tol::witness()).expect_err("radius 50 outruns the seam's legs");
    assert_eq!((err.step(), err.raised_at()), (5, 6), "{err}");
    let (_, _, deferral, refusal) = deferred(path_refusal(&err));
    assert_eq!(deferral, Deferral::FilletResolvedAtClose, "{err}");
    assert_eq!(
        err.to_string(),
        format!("step 5's fillet, not resolved at the close: {refusal}")
    );
}

/// A cusp written just after a fillet resolved, on the leg the fillet's
/// run out rides, is the cusp's refusal, never the fillet's.
#[test]
fn a_cusp_just_after_a_resolution_is_not_the_fillets() {
    let steps = [
        Step::At(p(0.0, 0.0)),
        Step::Angle(0.0),
        Step::Line(4.0),
        Step::Fillet { radius: 0.5 },
        Step::Angle(QUARTER),
        Step::At(p(5.0, 2.0)),
        Step::Line(1.0),
        Step::Cusp,
        Step::Line(1.0),
        Step::LineTo(Target::Start),
    ];
    let err = replay(&steps, Tol::witness()).expect_err("the leg doubles back over the run out");
    let source = path_refusal(&err);
    assert_eq!(source.step(), None, "nothing deferred: {err}");
    assert!(err.step() > ARRIVAL, "a step after the resolution: {err}");
    assert!(
        matches!(
            source.kind(),
            PathErrorKind::JunctionCusp | PathErrorKind::CuspDoublesBackOverLeg
        ),
        "{err}"
    );
}

/// **A junction a director binds before a fillet is read again at the
/// fillet's run in**, inside the fillet's resolution: the turn of
/// `1e-5` passes against the 10 m leg it follows, and the run in the
/// fillet leaves is `1e4 ε` long, too short to resolve it. The refusal
/// is the junction's, charged to the step whose leg leaves it — the
/// arrival, which emits the run in — and not to the fillet.
#[test]
fn a_junction_read_at_a_fillets_run_in_is_not_the_fillets() {
    let delta: f64 = 1e-5;
    let gap = 1e4 * Tol::witness().eps();
    let along = 1.0 + gap;
    let corner = p(10.0 + along * delta.cos(), along * delta.sin());
    let anchor = p(corner.x - 5.0 * delta.sin(), corner.y + 5.0 * delta.cos());
    let steps = [
        Step::At(p(0.0, 0.0)),
        Step::Angle(0.0),
        Step::Line(10.0),
        Step::Turn(delta),
        Step::Fillet { radius: 1.0 },
        Step::Angle(QUARTER + delta),
        Step::At(anchor),
        Step::LineTo(Target::Point(p(0.0, 5.0))),
        Step::LineTo(Target::Start),
    ];
    let source = own_refusal(&steps, 6, "the junction is unresolvable at the run in");
    let junction = match &source {
        PathError::JunctionTurnWithinTolerance { .. } => true,
        PathError::Escalated { gate, .. } => gate.predicate().starts_with("path_junction_"),
        _ => false,
    };
    assert!(junction, "the junction's own refusal: {source}");
}
