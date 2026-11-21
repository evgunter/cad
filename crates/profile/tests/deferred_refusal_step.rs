//! **A refusal a later step raises about an earlier one names the
//! earlier one.** A `fillet(r)` is resolved by the step that fixes its
//! arrival, and every fillet arc's stored form is re-read where the loop
//! closes; either can be steps past the fillet. Every recourse such a
//! refusal offers edits the fillet's own step, so the replay files it
//! there ([`ReplayError::step`]) and its sentence says which later step
//! raised it ([`PathError::Deferred`]). The drawing replay files the
//! fillet's construction at the same step, which is how the two match.

#![allow(clippy::expect_used, clippy::panic)]

use crate::common;

use common::tol;
use geom_core::{Point2, Tol};
use profile::path::program::replay_drawn;
use profile::{
    Deferral, Open, PathError, PathErrorKind, ReplayError, ReplayErrorKind, Start, Step, Target,
    replay,
};

/// The step the fillet below is bound at.
const FILLET: usize = 3;
/// The step that fixes its arrival, and resolves it.
const ARRIVAL: usize = 5;

/// A radius-5 fillet on a quarter turn whose legs are 1 and 0.5 long:
/// its setback, 5, outruns both, so the fit refuses at its arrival.
fn outrun() -> Vec<Step<f64>> {
    vec![
        Step::At(Point2::new(0.0, 0.0)),
        Step::Angle(0.0),
        Step::Line(1.0),
        Step::Fillet { radius: 5.0 },
        Step::Angle(core::f64::consts::FRAC_PI_2),
        Step::At(Point2::new(2.0, 0.5)),
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

#[test]
fn a_fillet_refused_at_its_arrival_is_filed_against_its_own_step() {
    let err = replay(&outrun(), Tol::witness()).expect_err("the setback outruns both legs");
    assert_eq!(err.step, FILLET, "the replay files it at the fillet: {err}");
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
    assert_eq!(
        err.to_string(),
        format!(
            "step 3: the fillet bound at step 3 cannot be resolved against its arrival at \
             step 5: {refusal}"
        ),
    );

    // The drawing replay files the refused fit's construction at the
    // same step as the refusal, so the editor matches them by it.
    let (drawn, built) = replay_drawn(&outrun(), Tol::witness());
    let drawn = drawn.expect_err("the drawing replay refuses the same way");
    let refused = built.last().expect("the fillet step's construction");
    assert_eq!((refused.step, drawn.step), (FILLET, FILLET), "{drawn}");

    // The typed surface the replay drives raises the same refusal.
    let t = Tol::witness();
    let typed = Open
        .at(Point2::new(0.0, 0.0))
        .angle(0.0, t)
        .and_then(|x| x.line(1.0, t))
        .and_then(|x| x.fillet(5.0, t))
        .and_then(|x| x.angle(core::f64::consts::FRAC_PI_2, t))
        .and_then(|x| x.at(Point2::new(2.0, 0.5), t))
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
        Step::At(Point2::new(0.0, 0.0)),
        Step::Angle(0.0),
        Step::Fillet { radius },
        Step::At(Point2::new(4.0 + 3.0 * theta.cos(), 3.0 * theta.sin())),
        Step::Angle(theta),
        Step::Line(1.0),
        Step::LineTo(Target::Point(Point2::new(8.0, 2.0))),
        Step::LineTo(Target::Point(Point2::new(4.0, 3.0))),
        Step::LineTo(Target::Start),
    ]
}

#[test]
fn a_fillet_its_stored_form_cannot_hold_is_filed_against_its_own_step_at_the_close() {
    let steps = flattened();
    assert_eq!(steps.len(), CLOSE + 1, "the close is the last step");
    let err = replay(&steps, tol()).expect_err("an arc shorter than ε refuses at the close");
    assert_eq!(
        err.step, STORED_FILLET,
        "the replay files it at the fillet: {err}"
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
        format!(
            "step 2: the fillet bound at step 2 cannot be stored where the loop closes, at \
             step 8: {refusal}"
        ),
    );

    let t = tol();
    let theta = 1e-3;
    let typed = Open
        .at(Point2::new(0.0, 0.0))
        .angle(0.0, t)
        .and_then(|x| x.fillet(0.5 * t.eps() / theta, t))
        .and_then(|x| x.at(Point2::new(4.0 + 3.0 * theta.cos(), 3.0 * theta.sin()), t))
        .and_then(|x| x.angle(theta, t))
        .and_then(|x| x.line(1.0, t))
        .and_then(|x| x.line_to(Point2::new(8.0, 2.0), t))
        .and_then(|x| x.line_to(Point2::new(4.0, 3.0), t))
        .and_then(|x| x.line_to(Start, t))
        .expect_err("the same close refuses on the typed surface");
    assert_eq!(typed.to_string(), path_refusal(&err).to_string());
}

/// What the arrival step refuses of its own arguments is its own,
/// though a fillet is open across it: only the resolution is deferred.
#[test]
fn an_arrival_steps_own_refusal_stays_at_that_step() {
    let mut steps = outrun();
    steps[FILLET + 1] = Step::Toward { dx: 0.0, dy: 0.0 };
    let err = replay(&steps, Tol::witness()).expect_err("a zero director names no direction");
    assert_eq!(err.step, FILLET + 1, "{err}");
    let source = path_refusal(&err);
    assert_eq!(source.step(), None, "no earlier step to name: {source}");
    assert_eq!(source.kind(), PathErrorKind::ZeroDirection, "{source}");
}
