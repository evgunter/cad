//! **A step order no value makes valid refuses at both doors**
//! (`[recipe.doors]`): the program's shape, values erased, refuses an
//! arc leg off a fillet's open arrival, a far-end anchor with no fillet
//! open and a `circle_split` of one arc, typed as the replay's own
//! refusal (`ProgramRefusal::Order`), at the edit door and, for a file
//! that holds it, at the load door (`ProgramFault::Order`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, len, len2, scl};
use crate::wire::doctored;
use editor_core::{
    DocEdit, EditError, Formula, LoopProgram, Node, PersistError, ProfileDoc, ProfileProgram,
    ProgramFault, ProgramRefusal, ProgramStep, ProgramTarget, VarDecl, VarName, load, save,
};
use geom_core::Tol;
use profile::PathErrorKind;

fn profile(loops: Vec<LoopProgram<Formula>>) -> editor_core::AuthoredNode {
    Node::Profile(ProfileProgram {
        loops,
        ids: Vec::new(),
        on: None,
    })
}

/// The insert door's order refusal of `loops`: its step and class.
fn refused_at_insert(doc: &ProfileDoc, loops: Vec<LoopProgram<Formula>>) -> (u32, PathErrorKind) {
    match fixture::insert_refused(doc, profile(loops)) {
        EditError::ProfileProgramRefused { refusal, .. } => match *refusal {
            ProgramRefusal::Order {
                loop_: 0,
                step,
                kind,
                ..
            } => (step, kind),
            other => panic!("an order refusal, got {other:?}"),
        },
        other => panic!("a program refusal, got {other:?}"),
    }
}

/// The load door's order refusal of `text`: its step and class.
fn refused_at_load(text: &str) -> (u32, PathErrorKind) {
    match load(text, Tol::witness()) {
        Err(PersistError::ProfileProgram {
            fault:
                ProgramFault::Order {
                    loop_: 0,
                    step,
                    kind,
                    ..
                },
            ..
        }) => (step, kind),
        other => panic!("an order refusal at load, got {other:?}"),
    }
}

fn toward(dx: f64, dy: f64) -> ProgramStep<Formula> {
    ProgramStep::Toward {
        dx: scl(dx),
        dy: scl(dy),
    }
}

fn point(x: f64, y: f64) -> ProgramTarget<Formula> {
    ProgramTarget::Point(len2([x, y]))
}

/// A fillet opened off `(0, 0)` heading `+x`, its arrival anchored at
/// `(4, 2)`, then `leg` from that anchor, closing back.
fn after_an_open_fillet(leg: ProgramStep<Formula>) -> Vec<LoopProgram<Formula>> {
    vec![LoopProgram::Chain(vec![
        ProgramStep::At(len2([0.0, 0.0])),
        toward(1.0, 0.0),
        ProgramStep::Fillet(len(0.5)),
        ProgramStep::At(len2([4.0, 2.0])),
        leg,
        ProgramStep::LineTo(ProgramTarget::Start),
    ])]
}

/// **An arc leg off a fillet's open arrival refuses at both doors**,
/// where `line_to` from the same anchor is the fillet's arrival and
/// lands. The file is the landed twin with its leg rewritten as the
/// arc. Red if either door admits the arc, which evaluation would then
/// refuse as `ArcLegOnOpenFillet`.
#[test]
fn an_arc_leg_off_an_open_fillet_refuses_at_both_doors() {
    let doc = ProfileDoc::empty_derived("door-step-order-arc", Tol::witness());
    let doc = fixture::step(
        doc,
        DocEdit::DeclareVar {
            name: VarName::from_static("b"),
            def: VarDecl::Free(editor_core::FreeVar::continuous(
                editor_core::Dimension::Scalar,
                0.3,
            )),
        },
    )
    .0;
    let b = doc.var_named("b").expect("declared");
    let arc = ProgramStep::ArcTo(editor_core::ProgramArcData::Bulge {
        target: point(4.0, 4.0),
        b: Formula::named(VarName::from_static("b"), editor_core::Dimension::Scalar),
    });
    assert_eq!(
        refused_at_insert(&doc, after_an_open_fillet(arc)),
        (4, PathErrorKind::ArcLegOnOpenFillet),
        "the insert door"
    );
    let (doc, twin) = fixture::insert(
        doc,
        profile(after_an_open_fillet(ProgramStep::LineTo(point(4.0, 4.0)))),
    );
    let text = save(&doc, Tol::witness()).expect("the twin saves");
    let b = serde_json::to_value(b).unwrap();
    let mangled = doctored(&text, |v| {
        let leg =
            &mut v["snapshot"]["nodes"][twin.0.to_string()]["Profile"]["loops"][0]["Chain"][4];
        let target = leg["LineTo"].take();
        *leg = serde_json::json!({ "ArcTo": { "Bulge": { "target": target, "b": b } } });
    });
    assert_eq!(
        refused_at_load(&mangled),
        (4, PathErrorKind::ArcLegOnOpenFillet),
        "the load door"
    );
}

/// **A far-end anchor with no fillet open refuses at the edit door**:
/// from the entry, a bound direction ends no arrival side. Red if the
/// door admits it, which evaluation would then refuse as
/// `FarEndAnchorWithoutFillet`.
#[test]
fn a_far_end_anchor_with_no_fillet_open_refuses_at_the_door() {
    let doc = ProfileDoc::empty_derived("door-step-order-far-end", Tol::witness());
    let loops = vec![LoopProgram::Chain(vec![
        toward(1.0, 0.0),
        ProgramStep::FarEndTo(len2([2.0, 0.0])),
        ProgramStep::LineTo(point(2.0, 2.0)),
        ProgramStep::LineTo(ProgramTarget::Start),
    ])];
    assert_eq!(
        refused_at_insert(&doc, loops),
        (1, PathErrorKind::FarEndAnchorWithoutFillet)
    );
}

/// **A `circle_split` of one arc refuses at both doors**: its count is
/// written, and one vertex cannot carry a full turn. The file is a
/// four-arc split with its count rewritten. Red if either door admits
/// it, which evaluation would then refuse as `CircleSplitCount`.
#[test]
fn a_circle_split_of_one_arc_refuses_at_both_doors() {
    let doc = ProfileDoc::empty_derived("door-step-order-split", Tol::witness());
    let split = |n| LoopProgram::CircleSplit {
        centre: [len(0.0), len(0.0)],
        radius: len(1.0),
        n,
        phase: Formula::zero(editor_core::Dimension::Angle),
    };
    assert_eq!(
        refused_at_insert(&doc, vec![split(1)]),
        (0, PathErrorKind::CircleSplitCount),
        "the insert door"
    );
    let (doc, four) = fixture::insert(doc, profile(vec![split(4)]));
    let text = save(&doc, Tol::witness()).expect("the four-arc split saves");
    let mangled = doctored(&text, |v| {
        v["snapshot"]["nodes"][four.0.to_string()]["Profile"]["loops"][0]["CircleSplit"]["n"] =
            serde_json::json!(1);
    });
    assert_eq!(
        refused_at_load(&mangled),
        (0, PathErrorKind::CircleSplitCount),
        "the load door"
    );
}
