//! **The viewer's boundary between a picked face and a mate head.**
//!
//! A mate head is a `FaceName`, and the viewer is one of the three
//! places that turn data into one. Its input is a `FaceSelection`,
//! whose `name` is a bare `StableName`: a face by the PICKING DOOR's
//! rule (`SelectionRefusal::NotAFace`, refused before a selection
//! exists) and not by its type, with every field public. So the mate
//! tool asks the head constructor and answers its refusal typed, in
//! every build — the two rows below are that answer and its sentence.
//!
//! `work/view/face-selection-carries-a-bare-stable-name` is the row
//! that removes the question: `FaceSelection::name` becomes a
//! `FaceName`, made where the pick is made, and
//! `MateToolError::PickIsNotAFace` goes away with the constructor call
//! it answers.

#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use common::asm;
use pncad::document::MateSide;
use pncad::geom_core::Tol;
use pncad::select::EntityKind;
use viewer::matetool::{MateTool, MateToolError};

/// **An edge-named selection is refused, not asserted against.** The
/// tool's own public doors take a `FaceSelection` by value, and
/// nothing but the picking door's rule keeps its `name` a face. A
/// selection that names an edge is answered `PickIsNotAFace` — naming
/// the side and carrying the head constructor's refusal — in debug and
/// in release alike.
#[test]
fn an_edge_named_pick_is_refused_by_the_mate_tool() {
    let tol = Tol::witness();
    let bench = asm::bench("rv-matehead", tol);
    let session = asm::open_bench(&bench, tol);
    let (mut a, b) = asm::seat_picks(&session, &bench);
    assert_eq!(a.name.kind, EntityKind::Face);
    // The one field the picking door's rule lives in, and nothing but
    // that rule keeps it a face.
    a.name.kind = EntityKind::Edge;
    let mut tool = MateTool::new();
    tool.pick(session.doc(), a);
    tool.pick(session.doc(), b);
    let (doc, _) = session.landed_pair().expect("landed");
    match tool.proposal(doc, asm::seat_choice()) {
        Err(MateToolError::PickIsNotAFace { side, refusal }) => {
            assert_eq!(side, MateSide::On);
            assert_eq!(refusal.found, EntityKind::Edge);
        }
        other => panic!("an edge-named pick must refuse as the kind, got {other:?}"),
    }
}

/// **The refusal says the kind, in the constructor's own words.** The
/// operand arm (`NotACopyPick`) names a node and says nothing about
/// what the selection denoted, so it is the wrong sentence for this
/// mistake; the kind arm forwards `NotAFaceName`'s rendering rather
/// than re-spelling it.
#[test]
fn the_kind_refusal_forwards_the_head_constructors_sentence() {
    let refusal = pncad::document::FaceName::new(pncad::prelude::StableName {
        kind: EntityKind::Edge,
        node: pncad::document::RecipeNodeId::new(0, 0),
        path: Vec::new(),
    })
    .expect_err("an edge is not a face name");
    let shown = MateToolError::PickIsNotAFace {
        side: MateSide::To,
        refusal,
    }
    .to_string();
    assert!(
        shown.contains(&refusal.to_string()) && shown.contains("an edge"),
        "the kind refusal drops the constructor's sentence: {shown:?}"
    );

    let operand = MateToolError::NotACopyPick {
        node: pncad::document::SpokenNode::absent(pncad::document::RecipeNodeId::new(0, 0)),
    }
    .to_string();
    assert!(
        !operand.contains("face") && !operand.contains("edge"),
        "the operand arm must stay silent about the kind: {operand:?}"
    );
}

/// **A face name at an instance that its copy does not carry is the
/// placement's refusal where it evaluates.** The tool reads no pose
/// before committing, so a hand-built selection naming a face at the
/// instance that its part's product holds no row of proposes and
/// commits; the instance's copy holds only its part's rows, so the
/// mate's `on` does not evaluate and the post's row fails
/// `SideUnevaluated` on the `on` side — never a placement that
/// silently stands somewhere.
#[test]
fn a_face_name_at_an_instance_its_copy_does_not_carry_refuses_where_it_evaluates() {
    let tol = Tol::witness();
    let bench = asm::bench("rv-matehead-bare", tol);
    let mut session = asm::open_bench(&bench, tol);
    let (mut a, b) = asm::seat_picks(&session, &bench);
    assert_ne!(
        a.name.node, a.node,
        "pick a is its part's name, read at its instance"
    );
    // The part's name with its role taken off: a bare face name the
    // copy does not carry.
    let mut bare = a.name.clone();
    bare.path = Vec::new();
    a.name = bare;
    let mut tool = MateTool::new();
    tool.pick(session.doc(), a);
    tool.pick(session.doc(), b);
    asm::unpin(&mut session, bench.post_b);
    let (doc, _) = session.landed_pair().expect("landed");
    let proposal = tool
        .proposal(doc, asm::seat_choice())
        .expect("no pose is read before the commit");
    let outcome = session.perform(proposal.op_into_target_space());
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let (_, eval) = session.landed_pair().expect("landed");
    match eval
        .result(bench.post_b)
        .and_then(|r| r.error())
        .map(|e| &e.kind)
    {
        Some(pncad::document::NodeErrorKind::Mate(fault))
            if matches!(
                fault.as_ref(),
                pncad::document::MateFault::SideUnevaluated {
                    side: MateSide::On,
                    ..
                }
            ) => {}
        other => {
            panic!("a face name its copy does not carry refuses where it evaluates, got {other:?}")
        }
    }
}
