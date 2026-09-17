//! **Review probes for `edit/mate-head-kind`** (lane `matehead-rv`).
//!
//! The viewer half of the three boundaries PR #2799 claims call
//! `FaceName::new`. Not rows the branch owes.

#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use common::asm;
use pncad::document::MateSide;
use pncad::geom_core::Tol;
use pncad::select::{EntityKind, Ray, RoleSeg};
use viewer::matetool::{MateTool, MateToolError};
use viewer::session::{DocSession, FaceSelection};

fn pick_at(session: &DocSession, ray: &Ray) -> FaceSelection {
    let index = asm::index_of(session);
    let (_, eval) = session.landed_pair().expect("landed");
    index
        .face_at_for(eval, ray, &session.display_view())
        .expect("the pick answers")
        .expect("the ray hits")
}

fn two_picks(session: &DocSession) -> (FaceSelection, FaceSelection) {
    let a = pick_at(
        session,
        &asm::down_at(
            asm::POST_B_AT[0] + asm::POST_SECTION / 2.0,
            asm::POST_B_AT[1] + asm::POST_SECTION / 2.0,
        ),
    );
    let b = pick_at(
        session,
        &asm::up_at(
            asm::SHELF_AT[0] + asm::SHELF_LENGTH / 2.0,
            asm::SHELF_AT[1] + asm::SHELF_DEPTH / 2.0,
        ),
    );
    (a, b)
}

/// **CLAIM 2 (the third boundary).** `FaceSelection`'s fields are all
/// `pub` and it has no checked constructor, so a caller of the
/// viewer's own public doors (`MateTool::pick` / `MateTool::proposal`)
/// can hand the mate tool a selection naming an EDGE. Before this PR
/// that reached the assembly gate and refused typed; now it hits
/// `picked_member`'s `debug_assert!(false)` and PANICS in a debug
/// build. The third boundary does not refuse — it aborts.
#[test]
#[should_panic(expected = "a face selection carries a name that is not a face")]
fn probe_an_edge_named_pick_panics_in_the_mate_tool() {
    let tol = Tol::witness();
    let bench = asm::bench("rv-matehead", tol);
    let session = asm::open_bench(&bench, tol);
    let (mut a, b) = two_picks(&session);
    assert_eq!(a.name.kind, EntityKind::Face);
    // The one field the picking door's rule lives in, and nothing but
    // that rule keeps it a face.
    a.name.kind = EntityKind::Edge;
    let mut tool = MateTool::new();
    tool.pick(a);
    tool.pick(b);
    let (doc, eval) = session.landed_pair().expect("landed");
    let _ = tool.proposal(doc, eval, tol, asm::seat());
}

/// The release half of the same probe, asserted as prose rather than
/// run: with `debug_assert` compiled out the tool answers
/// `MateToolError::NotAnInstancePick`, which names the OPERAND and not
/// the kind — so of the three boundaries only two carry the
/// constructor's sentence.
#[test]
fn probe_the_release_answer_is_the_operand_arm_not_the_kind() {
    // Reached here only as a compile-time check that the arm exists
    // with the shape the release path returns.
    let arm = MateToolError::NotAnInstancePick {
        side: MateSide::A,
        node: pncad::document::RecipeNodeId(0),
    };
    let text = format!("{arm}");
    assert!(
        !text.contains("face") && !text.contains("edge"),
        "the release answer says nothing about the kind: {text:?}"
    );
    let _ = RoleSeg::Cap(pncad::select::CapEnd::Start);
}
