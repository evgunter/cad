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

/// **The viewer's at-rest badge and an unplaced group's own space.**
/// `assemble` checks each unplaced group inside its own space; the
/// session's landing gathers the world and badges `assemble_gathered`
/// over it alone. A group whose two posts interpenetrate (both seated
/// at one spot under the shelf), moved onto a gauge that is then
/// deleted, beside one world instance.
#[test]
fn probe_viewer_badge_skips_an_unplaced_groups_own_space() {
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
            node: Node::instantiate_part(bench.post),
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
            node: seat(bench.post_a, asm::middle_seat_alignment()),
        },
    );
    step(
        &mut doc,
        DocEdit::InsertNode {
            node: seat(bench.post_b, asm::middle_seat_alignment()),
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
    let placed_verdict = assemble(&doc, &ev, tol)
        .err()
        .map(|e| e.to_string().chars().take(80).collect::<String>());
    eprintln!("placed: assemble -> {placed_verdict:?}");
    // Onto a gauge, then delete it.
    let g = step(
        &mut doc,
        DocEdit::InsertNode {
            node: Node::gauge(
                None,
                Placement::literal(&Frame::translation([0.0, 0.0, 0.0])),
            ),
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
    eprintln!(
        "unplaced: assemble -> {:?}",
        kernel
            .as_ref()
            .err()
            .map(|e| e.to_string().chars().take(80).collect::<String>())
    );
    std::fs::write(&bench.asm_path, save(&doc, &[], tol).unwrap()).unwrap();
    let session = asm::open_bench(&bench, tol);
    let badge = session.at_rest().cloned();
    eprintln!("viewer badge: {badge:?}");
    if kernel.is_err() && matches!(badge, Some(viewer::session::AtRestBadge::Certified { .. })) {
        panic!(
            "DEFECT: the viewer badges Certified while assemble refuses the unplaced group's own space"
        );
    }
}
