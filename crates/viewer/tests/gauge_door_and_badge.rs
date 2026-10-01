//! **The mate door and an unplaced group's own space, through the
//! session** (ASSEMBLY.md A11 (2)): the door's cleared offset through
//! undo, redo and replay, and the at-rest badge over an unplaced
//! group's own space.

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

/// **The mate door through the session's history**: committing a placing
/// mate clears the first operand's offset and keeps the second's; undo
/// restores it, redo clears it again, and the log replays to the same
/// document with no store.
#[test]
fn the_mate_door_clears_through_undo_redo_and_replay() {
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

/// **The at-rest badge checks an unplaced group's own space** — the one
/// gate the kernel's `assemble` runs, reached through
/// `assemble_gathered`. Two posts seated at one spot under the shelf
/// interpenetrate; moved with the shelf onto a gauge that is then
/// deleted, they live in their own space beside one world instance. The
/// kernel refuses, and the badge refuses with it rather than certify the
/// world alone.
#[test]
fn the_at_rest_badge_checks_an_unplaced_groups_own_space() {
    use pncad::document::{
        Alignment, CancelToken, DocEdit, EvalOptions, Frame, Placement, RefusingReach, apply,
        assemble, evaluate, load, save,
    };
    let tol = Tol::witness();
    let bench = asm::bench("r2badge", tol);
    let text = std::fs::read_to_string(&bench.asm_path).unwrap();
    let mut doc = load(&text, tol).unwrap().doc;
    let mut step = |doc: &mut pncad::document::Doc<pncad::document::ProfileProgram>,
                    edit: DocEdit<pncad::document::ProfileProgram>| {
        let applied = apply(doc, &edit, tol, &RefusingReach).unwrap();
        *doc = applied.doc;
        applied.record.minted
    };
    let seat = |post, alignment: Alignment| Node::Mate {
        a: common::head(asm::in_part(post, &bench.post_top)),
        b: common::head(asm::in_part(bench.shelf_i, &bench.shelf_bottom)),
        class: ContactClass::Rest,
        alignment,
    };
    // The world-side lone instance, far off.
    let lone = step(
        &mut doc,
        DocEdit::InsertNode {
            node: Box::new(Node::instantiate_part(bench.post)),
        },
    )
    .unwrap();
    step(
        &mut doc,
        DocEdit::SetOffset {
            instance: lone,
            offset: Some(Placement::literal(&Frame::translation([1.0, 0.0, 0.0]))),
        },
    );
    // Both posts at ONE spot under the shelf: undeclared interference.
    step(
        &mut doc,
        DocEdit::InsertNode {
            node: Box::new(seat(bench.post_a, asm::middle_seat_alignment())),
        },
    );
    step(
        &mut doc,
        DocEdit::InsertNode {
            node: Box::new(seat(bench.post_b, asm::middle_seat_alignment())),
        },
    );
    let opts = |dir: &std::path::Path| EvalOptions {
        resolver: Some(std::sync::Arc::new(
            pncad::workspace::Workspace::open(dir).unwrap(),
        )),
        ..EvalOptions::default()
    };
    let o = opts(&bench.dir);
    let ev = evaluate::<f64>(&doc, None, &CancelToken::new(), &o, tol);
    assert!(
        assemble(&doc, &ev, tol).is_err(),
        "placed, the two posts interpenetrate in the world"
    );
    // Onto a gauge, then delete it.
    let g = step(
        &mut doc,
        DocEdit::InsertNode {
            node: Box::new(Node::gauge(
                None,
                Placement::literal(&Frame::translation([0.0, 0.0, 0.0])),
            )),
        },
    )
    .unwrap();
    for id in [bench.post_a, bench.shelf_i, bench.post_b] {
        step(
            &mut doc,
            DocEdit::SetGauge {
                node: id,
                gauge: Some(g),
            },
        );
    }
    step(&mut doc, DocEdit::DeleteNode { id: g });
    let ev = evaluate::<f64>(&doc, None, &CancelToken::new(), &o, tol);
    assert!(
        ev.unplaced.contains_key(&bench.shelf_i),
        "the group is unplaced"
    );
    let kernel = assemble(&doc, &ev, tol);
    assert!(
        kernel.is_err(),
        "the kernel refuses the own space's interference"
    );
    std::fs::write(&bench.asm_path, save(&doc, &[], tol).unwrap()).unwrap();
    let session = asm::open_bench(&bench, tol);
    let badge = session.at_rest().cloned();
    assert!(
        matches!(badge, Some(viewer::session::AtRestBadge::Refused { .. })),
        "the badge refuses with the kernel: {badge:?}"
    );
}
