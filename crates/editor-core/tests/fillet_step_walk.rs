//! **A `Fillet` chain is learnable from its refusals.** A newcomer
//! authoring R2's rounded pad (`m10_8_r2_probes_interval::pad`) writes
//! the steps a sketch would suggest and is refused; each refusal names
//! the steps its tip state takes, read off the transition table the
//! replay ran on, so the next attempt is the one the refusal named.
//!
//! The walk below is the attempts in the order a newcomer makes them,
//! each through the insert door a person uses.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{len, len2, scl};
use editor_core::{
    DocEdit, EditError, LoopProgram, Node, ProfileDoc, ProfileProgram, ProgramRefusal, ProgramStep,
    ProgramTarget, apply,
};
use geom_core::Tol;
use profile::{TipState, Verb};

const W: f64 = 4.0e-3;
const H: f64 = 2.5e-3;
const R: f64 = 0.8e-3;

/// The insert door's answer for one chain: `Ok` when it builds, the
/// typed refusal otherwise.
fn insert(steps: Vec<ProgramStep<editor_core::Formula>>) -> Result<(), ProgramRefusal> {
    let doc = ProfileDoc::empty_derived("fillet_step_walk", Tol::witness());
    match apply(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::Profile(ProfileProgram {
                loops: vec![LoopProgram::Chain(steps)],
                ids: Vec::new(),
                on: None,
            })),
            fresh: Vec::new(),
        },
        Tol::witness(),
    ) {
        Ok(_) => Ok(()),
        Err(EditError::ProfileProgramRefused { refusal, .. }) => Err(*refusal),
        Err(other) => panic!("the insert door refused outside the program class: {other}"),
    }
}

fn pt(x: f64, y: f64) -> ProgramStep<editor_core::Formula> {
    ProgramStep::LineTo(ProgramTarget::Point(len2([x, y])))
}

fn fillet() -> ProgramStep<editor_core::Formula> {
    ProgramStep::Fillet(len(R))
}

fn toward(dx: f64, dy: f64) -> ProgramStep<editor_core::Formula> {
    ProgramStep::Toward {
        dx: scl(dx),
        dy: scl(dy),
    }
}

fn far_end(x: f64, y: f64) -> ProgramStep<editor_core::Formula> {
    ProgramStep::FarEndTo([len(x), len(y)])
}

/// The refusal's lattice coordinate and its sentence.
fn lattice(
    attempt: &str,
    steps: Vec<ProgramStep<editor_core::Formula>>,
) -> (u32, TipState, Option<Verb>, String) {
    match insert(steps) {
        Err(refusal @ ProgramRefusal::Transition { .. }) => {
            let ProgramRefusal::Transition {
                step, state, verb, ..
            } = refusal
            else {
                unreachable!()
            };
            let said = refusal.to_string();
            eprintln!("WALK {attempt}: {said}");
            (step, state, verb, said)
        }
        other => panic!("{attempt}: expected a lattice refusal, got {other:?}"),
    }
}

/// **The walk.** Each attempt is the previous one repaired the way its
/// refusal said; the last builds.
#[test]
fn each_refusal_on_the_way_to_a_fillet_names_what_its_state_takes() {
    // 0. The first leg with no start point. A chain cannot spell the
    //    complete-loop forms the entry also takes, so they are not
    //    offered.
    let (step, state, verb, said) = lattice("no start point", vec![pt(-W, H - 2.0 * R)]);
    assert_eq!(
        (step, state, verb),
        (0, TipState::Entry, Some(Verb::LineTo))
    );
    assert!(
        said.ends_with(
            "the LineTo verb is not a legal continuation of tip state Entry, at loop 0 step 0. Recourse: continue with At, Angle, \
             Toward, ArcFillet or ArcFilletArc"
        ),
        "the entry names the chain's entry steps and no loop form: {said}"
    );

    // 1. A fillet straight after the start point: there is no incoming
    //    leg to round.
    let (step, state, verb, said) = lattice(
        "fillet at the start",
        vec![ProgramStep::At([len(-W), len(0.0)]), fillet()],
    );
    assert_eq!(
        (step, state, verb),
        (1, TipState::PlainPoint, Some(Verb::Fillet))
    );
    assert!(
        said.ends_with(
            "the Fillet verb is not a legal continuation of tip state PlainPoint, at loop 0 step 1. Recourse: continue with \
             Angle, Toward, LineTo, ArcTo, ArcOnto, ArcFillet or ArcFilletArc"
        ),
        "the start point names the legs that may precede a fillet: {said}"
    );

    // 2. A leg, the fillet, then the next leg as a point target: the
    //    fillet opened its arrival side, which a point target does
    //    not fill.
    let (step, state, verb, said) = lattice(
        "line_to after the fillet",
        vec![
            ProgramStep::At([len(-W), len(0.0)]),
            pt(-W, H - 2.0 * R),
            fillet(),
            pt(W - 2.0 * R, H),
        ],
    );
    assert_eq!((step, state, verb), (3, TipState::Open, Some(Verb::LineTo)));
    assert!(
        said.ends_with(
            "the LineTo verb is not a legal continuation of tip state Open, at loop 0 step 3. Recourse: continue with At, Angle, \
             Toward or CloseTo"
        ),
        "the fillet's arrival side names its directors, anchor and seam close: {said}"
    );

    // 3. The arrival's direction, then the leg by length: the arrival
    //    ends at an anchor, not after a run.
    let (step, state, verb, said) = lattice(
        "line after the arrival's direction",
        vec![
            ProgramStep::At([len(-W), len(0.0)]),
            pt(-W, H - 2.0 * R),
            fillet(),
            toward(1.0, 0.0),
            ProgramStep::Line(len(2.0 * (W - R))),
        ],
    );
    assert_eq!((step, state), (4, TipState::Angle));
    assert_eq!(verb, Some(Verb::Line));
    assert!(
        said.ends_with(
            "the Line verb is not a legal continuation of tip state Angle, at loop 0 step 4. Recourse: continue with At or FarEndTo"
        ),
        "a bound arrival direction names the far-end anchor: {said}"
    );

    // 4. Closing with a point target back to the start after the last
    //    fillet.
    let (_, state, _, said) = lattice(
        "line_to Start after the last fillet",
        vec![
            ProgramStep::At([len(-W), len(0.0)]),
            pt(-W, H - 2.0 * R),
            fillet(),
            toward(1.0, 0.0),
            far_end(W - 2.0 * R, H),
            fillet(),
            ProgramStep::LineTo(ProgramTarget::Start),
        ],
    );
    assert_eq!(state, TipState::Open);
    assert!(
        said.contains("CloseTo"),
        "the arrival side after a fillet names the seam close: {said}"
    );

    // 5. The chain left open after the last leg.
    let (_, state, verb, said) = lattice(
        "unclosed after the last fillet",
        vec![
            ProgramStep::At([len(-W), len(0.0)]),
            pt(-W, H - 2.0 * R),
            fillet(),
        ],
    );
    assert_eq!((state, verb), (TipState::Open, None));
    assert!(
        said.ends_with(
            "the program ends at tip state Open without closing the loop, at loop 0 step 3. Recourse: continue \
             with At, Angle, Toward or CloseTo"
        ),
        "an unclosed chain names how it continues: {said}"
    );

    // The pad as the refusals led to it.
    let mut pad = vec![ProgramStep::At([len(-W), len(0.0)]), pt(-W, H - 2.0 * R)];
    pad.extend([fillet(), toward(1.0, 0.0), far_end(W - 2.0 * R, H)]);
    pad.extend([fillet(), toward(0.0, -1.0), far_end(W, -H + 2.0 * R)]);
    pad.extend([fillet(), toward(-1.0, 0.0), far_end(-W + 2.0 * R, -H)]);
    pad.extend([fillet(), ProgramStep::CloseTo]);
    insert(pad).expect("the pad the refusals led to builds");
}
