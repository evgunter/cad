//! Review lane place2-r2's viewer probe on PR #3676: the mate door
//! through the viewer's history (undo, redo, replay).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::asm;
use pncad::document::Node;
use pncad::geom_core::Tol;
use pncad::select::ContactClass;
use viewer::history::History;
use viewer::session::SessionOp;

fn offset_of(
    doc: &pncad::document::Doc<pncad::document::ProfileProgram>,
    id: pncad::document::RecipeNodeId,
) -> Option<pncad::document::Placement> {
    match doc.node(id) {
        Some(Node::InstantiatePart { offset, .. }) => offset.clone(),
        other => panic!("{other:?}"),
    }
}

#[test]
fn probe_mate_door_through_undo_redo_and_replay() {
    let tol = Tol::witness();
    let bench = asm::bench("r2door", tol);
    let mut session = asm::open_bench(&bench, tol);
    let before = session.doc().clone();
    assert!(offset_of(&before, bench.post_b).is_some());
    common::commit_mate(
        &mut session,
        asm::seat_op(
            &bench,
            bench.post_b,
            ContactClass::Rest,
            asm::middle_seat_alignment(),
        ),
    );
    let after = session.doc().clone();
    assert_eq!(
        offset_of(&after, bench.post_b),
        None,
        "the first operand's root offset cleared"
    );
    assert!(
        offset_of(&after, bench.shelf_i).is_some(),
        "the second's kept"
    );
    let out = session.perform(SessionOp::Undo);
    assert!(out.refusal.is_none());
    assert!(session.doc().bit_eq(&before), "undo restores the offset");
    let out = session.perform(SessionOp::Redo);
    assert!(out.refusal.is_none());
    assert!(session.doc().bit_eq(&after), "redo clears it again");
    let h = session.history();
    let root = h.entry(h.root()).doc().clone();
    let replayed = History::replayed(root, &h.path_edits(), tol).expect("replays with no store");
    assert!(
        replayed.doc().bit_eq(&after),
        "the log replays to the same document"
    );
}
