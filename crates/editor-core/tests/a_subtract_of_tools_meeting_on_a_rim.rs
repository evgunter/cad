//! **A subtract of two tools that meet on a rim**: a rod and an elbow
//! revolved off its own end cap (the pair `a_pipe_unions_with_its_elbow`
//! glues a junction on), both cutting one kept body that holds the rim.
//!
//! The fold judges the two tools as their union, for its verdicts, and
//! a rim junction has no reading through the accumulation's turned tool
//! face (`topo::boolean::nary`'s `given_pairs`): a step given one there
//! refuses as that member's fold step. Today the judgement stops first,
//! where the union of the same two stops: past the crossing layer (which
//! reads an edge ending on the rim, `geom_brep::curve_face::end_root`),
//! at the finish on the seams' stand-in descriptions
//! (`work/heldrecords/a-pipe-does-not-union-with-an-elbow-revolved-off-its-own-cap.md`),
//! so this row pins that refusal, and the turned-junction guard behind
//! it is the kernel's own row
//! (`a_subtract_reads_its_tools_turned_and_a_junction_has_no_turned_reading`).
//! The gap is `work/formwords/a-subtract-of-two-tools-that-meet-on-a-rim-refuses-its-second-cut.md`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::f64::consts::FRAC_PI_2;

use editor_core::test_support::{ang, len};
use editor_core::{
    BodyRead, CancelToken, CapEnd, DocEdit, EntityKind, EvalOptions, Formula, LoopProgram, Node,
    OnFace, Operand, ProfileDoc, ProfileProgram, RecipeNodeId, RoleSeg, StableName, evaluate,
};
use geom_core::Tol;

fn insert(
    doc: ProfileDoc,
    node: Node<ProfileProgram<Formula>, Formula>,
) -> (ProfileDoc, RecipeNodeId) {
    let applied = doc
        .apply(
            &DocEdit::InsertNode {
                node: Box::new(node),
                fresh: Vec::new(),
            },
            Tol::witness(),
        )
        .unwrap();
    (applied.doc, applied.record.minted.unwrap())
}

fn circle(x: f64, r: f64) -> LoopProgram<Formula> {
    LoopProgram::Circle {
        centre: [len(x), len(0.0)],
        radius: len(r),
    }
}

fn profile(doc: ProfileDoc, loops: Vec<LoopProgram<Formula>>) -> (ProfileDoc, RecipeNodeId) {
    insert(
        doc,
        Node::Profile(ProfileProgram {
            loops,
            ids: Vec::new(),
            on: None,
        }),
    )
}

/// **The cut by a rod and its elbow refuses, typed, and builds no
/// body**: the finish's refusal of the seams' stand-in descriptions, as
/// the tools' own union refuses. Red where the cut builds a body (the
/// junction read through a turned face as it stands), or refuses
/// otherwise than at the finish's stand-ins or a fold step naming the
/// elbow.
#[test]
fn a_rod_and_its_elbow_cutting_one_block_refuse_typed() {
    let doc = ProfileDoc::empty_derived("a_subtract_of_tools_meeting_on_a_rim", Tol::witness());
    let (doc, rod_section) = profile(doc, vec![circle(0.0, 1.0)]);
    let (doc, rod) = insert(
        doc,
        Node::Extrude {
            profile: rod_section.into(),
            distance: len(2.0),
        },
    );
    let cap = StableName {
        kind: EntityKind::Face,
        node: rod,
        path: vec![RoleSeg::Cap(CapEnd::End)],
    };
    let drawn = editor_core::draw_on_face(
        &doc,
        BodyRead::plain(Operand::output(rod, 0)),
        cap,
        vec![circle(0.0, 1.0)],
        OnFace::nearest(),
        &EvalOptions::default(),
        &CancelToken::new(),
        Tol::witness(),
    )
    .unwrap();
    let (doc, elbow) = insert(
        drawn.doc,
        Node::Revolve {
            profile: drawn.profile.into(),
            axis_through: [len(3.0), len(0.0)],
            axis_heading: ang(-FRAC_PI_2),
            angle: ang(-FRAC_PI_2),
        },
    );
    let (doc, block_section) = profile(doc, vec![circle(1.0, 9.0)]);
    let (doc, block) = insert(
        doc,
        Node::Extrude {
            profile: block_section.into(),
            distance: len(9.0),
        },
    );
    // The block in the rod's space, from z = -1: it holds the rod, the
    // elbow and the rim between them.
    let (doc, block) = crate::fixture::slid(doc, rod, block, [0.0, 0.0, -1.0]);
    let (doc, cut) = insert(
        doc,
        Node::Subtract {
            from: block.into(),
            tool: editor_core::Bodies::Spelled(vec![
                rod.into(),
                BodyRead::plain(Operand::output(elbow, 0)),
            ]),
        },
    );
    let ev = evaluate::<f64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    for n in [rod, elbow, block] {
        assert!(ev.node_error(n).is_none(), "{n} builds");
    }
    let refusal = ev.node_error(cut).expect("the cut refuses");
    match &refusal.kind {
        editor_core::NodeErrorKind::Boolean(e @ topo::BooleanError::ResultInvalid { .. }) => {
            assert!(
                format!("{e:?}").contains("ScaffoldAtRest"),
                "the finish's stand-in descriptions: {e:?}"
            );
        }
        editor_core::NodeErrorKind::FoldStep {
            member, refusal, ..
        } => {
            assert_eq!(*member, 2, "the step folding in the elbow refuses");
            assert!(
                matches!(
                    refusal.as_ref(),
                    editor_core::NodeErrorKind::Boolean(topo::BooleanError::TangencyUnsupported)
                ),
                "the turned junction's refusal: {refusal:?}"
            );
        }
        other => panic!("the cut refuses at the finish or the elbow's step: {other:?}"),
    }
}
