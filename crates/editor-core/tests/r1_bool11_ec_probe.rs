//! R1 review probe for BOOL-11: the lifting door's typed refusal.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{LoopProgram, RecordedProgramError};
use geom_core::{Point2, Tol};
use profile::{Open, Start};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// The document-vocabulary gap is REACHABLE from ordinary authoring and
/// refuses TYPED (not a panic, not a silent drop).
#[test]
fn r1_lifting_door_refuses_continue_to_typed() {
    use std::f64::consts::FRAC_PI_2;
    let t = Tol::witness();
    let closed = Open
        .at(p2(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .continue_to(p2(2.0, 0.0), t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(2.0, t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(2.0, t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .continue_to(Start, t)
        .unwrap();
    let err = LoopProgram::from_recorded(&closed.program)
        .expect_err("continue_to has no document spelling yet");
    println!("R1: lifting door -> {err:?} / {err}");
    assert!(matches!(
        err,
        RecordedProgramError::VerbNotInDocumentVocabulary(profile::Verb::ContinueTo)
    ));
    let msg = err.to_string();
    assert!(msg.contains("ContinueTo"), "{msg}");
    assert!(msg.contains("format change"), "{msg}");
}

/// And the `Start` arm alone reaches it too (the closer, with no
/// interior point target).
#[test]
fn r1_lifting_door_refuses_the_closer_too() {
    use std::f64::consts::FRAC_PI_2;
    let t = Tol::witness();
    let closed = Open
        .at(p2(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(1.0, t)
        .unwrap()
        .turn(FRAC_PI_2, t)
        .unwrap()
        .line(0.5, t)
        .unwrap()
        .continue_to(Start, t)
        .unwrap();
    let err = LoopProgram::from_recorded(&closed.program).expect_err("closer has no spelling");
    assert!(matches!(
        err,
        RecordedProgramError::VerbNotInDocumentVocabulary(profile::Verb::ContinueTo)
    ));
}
