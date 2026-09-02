//! R1's BOOL-11 review probe, FLIPPED: the two chains it built to prove
//! the lifting door refused `continue_to` typed now prove the door lifts
//! them. The gap the probe pinned was a deferral, not a rule, and this
//! is the row that says it closed — the shape R1 chose is kept exactly
//! so the flip is legible against the original.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{LoopProgram, ProgramStep, ProgramTarget};
use geom_core::{Point2, Tol};
use profile::{Open, Start};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// The chain R1 built to reach the vocabulary gap now LIFTS: both
/// `continue_to` arms — the interior point target and the closer —
/// arrive in the document program as `ProgramStep::ContinueTo`.
#[test]
fn r1_lifting_door_lifts_continue_to() {
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
    let prog =
        LoopProgram::from_recorded(&closed.program).expect("continue_to has a document spelling");
    let LoopProgram::Chain(steps) = &prog else {
        panic!("a chain program: {prog:?}")
    };
    let arms: Vec<&ProgramStep> = steps
        .iter()
        .filter(|s| matches!(s, ProgramStep::ContinueTo(_)))
        .collect();
    assert_eq!(arms.len(), 2, "{steps:?}");
    assert!(matches!(
        arms[0],
        ProgramStep::ContinueTo(ProgramTarget::Point(_))
    ));
    assert!(matches!(
        arms[1],
        ProgramStep::ContinueTo(ProgramTarget::Start)
    ));
}

/// And the `Start` arm alone lifts too (the closer, with no interior
/// point target).
#[test]
fn r1_lifting_door_lifts_the_closer_too() {
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
    let prog = LoopProgram::from_recorded(&closed.program).expect("the closer has a spelling");
    let LoopProgram::Chain(steps) = &prog else {
        panic!("a chain program: {prog:?}")
    };
    assert!(
        steps
            .iter()
            .any(|s| matches!(s, ProgramStep::ContinueTo(ProgramTarget::Start))),
        "{steps:?}"
    );
}
