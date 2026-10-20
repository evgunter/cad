//! **The mate door and a loose copy's own space, through the session**
//! (ASSEMBLY.md A11 (2)): a committed mate through undo, redo and
//! replay, and the at-rest badge over a loose space.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::asm;
use pncad::geom_core::Tol;
use pncad::select::ContactClass;
use viewer::history::History;
use viewer::session::SessionOp;

/// **The mate door through the session's history**: committing a mate
/// between post_b, its world pin taken off, and the shelf adds the
/// constraint to post_b's bundle and leaves the shelf's as it was; undo
/// restores the document, redo commits it again, and the log replays to
/// the same document with no store.
#[test]
fn the_mate_door_commits_through_undo_redo_and_replay() {
    let tol = Tol::witness();
    let bench = asm::bench("r2door", tol);
    let mut session = asm::open_bench(&bench, tol);
    asm::unpin(&mut session, bench.post_b);
    let before = session.doc().clone();
    assert!(crate::fixture::bundle(&before, bench.post_b).is_empty());
    let shelf_bundle = crate::fixture::bundle(&before, bench.shelf_i);
    let mate = common::commit_mate(
        &mut session,
        asm::seat_op(
            &bench,
            bench.post_b,
            ContactClass::Rest,
            asm::middle_seat_values(),
        ),
    );
    assert_eq!(mate.placement, bench.post_b, "the mate places post_b");
    let after = session.doc().clone();
    assert_eq!(
        crate::fixture::bundle(&after, bench.shelf_i),
        shelf_bundle,
        "the shelf's bundle is kept"
    );
    let out = session.perform(SessionOp::Undo);
    assert!(out.refusal.is_none());
    assert!(session.doc().bit_eq(&before), "undo restores the bundle");
    let out = session.perform(SessionOp::Redo);
    assert!(out.refusal.is_none());
    assert!(session.doc().bit_eq(&after), "redo commits it again");
    let h = session.history();
    let root = h.entry(h.root()).doc().clone();
    let replayed = History::replayed(root, &h.path_edits(), tol).expect("replays with no store");
    assert!(
        replayed.doc().bit_eq(&after),
        "the log replays to the same document"
    );
}

/// **The at-rest badge checks a loose copy's own space** — the one gate
/// the kernel's `assemble` runs, reached through `assemble_gathered`.
/// Two posts seated at one spot under the shelf interpenetrate; with the
/// shelf's world pin taken off, the three live in the shelf's own space
/// beside one world instance. The kernel refuses, and the badge refuses
/// with it rather than certify the world alone.
#[test]
fn the_at_rest_badge_checks_a_loose_space() {
    use pncad::document::{
        CancelToken, DocEdit, EvalOptions, Value, apply, assemble, evaluate, load, save,
    };
    let tol = Tol::witness();
    let bench = asm::bench("r2badge", tol);
    let text = std::fs::read_to_string(&bench.asm_path).unwrap();
    let mut doc = load(&text, tol).unwrap().doc;
    let opts = EvalOptions {
        resolver: Some(std::sync::Arc::new(
            pncad::workspace::Workspace::open(&bench.dir).unwrap(),
        )),
        ..EvalOptions::default()
    };
    let step = |doc: &mut pncad::document::Doc<pncad::document::ProfileProgram>,
                edit: DocEdit<pncad::document::ProfileProgram>| {
        let applied = apply(doc, &edit, tol).unwrap();
        *doc = applied.doc;
        applied.record.minted_constraints
    };
    let seat = |post| {
        crate::fixture::rest(
            common::head(asm::in_part(post, &bench.post_top)),
            common::head(asm::in_part(bench.shelf_i, &bench.shelf_bottom)),
            ContactClass::Rest,
        )
    };
    let unpin = |doc: &pncad::document::Doc<pncad::document::ProfileProgram>, placement| {
        let pin = asm::world_pin_of(doc, placement);
        DocEdit::RemoveConstraint {
            placement,
            constraint: pin.id,
        }
    };
    // The world-side lone instance, far off.
    asm::instance_at(&mut doc, bench.post, [1.0, 0.0, 0.0], &opts, tol);
    // Both posts at ONE spot under the shelf: undeclared interference.
    for post in [bench.post_a, bench.post_b] {
        let edit = unpin(&doc, post);
        step(&mut doc, edit);
        let [mate] = step(
            &mut doc,
            DocEdit::AddConstraint {
                placement: post,
                mate: Box::new(seat(post)),
                fresh: Vec::new(),
            },
        )[..] else {
            panic!("the mate mints one id")
        };
        for (coord, by) in asm::middle_seat_values() {
            step(
                &mut doc,
                DocEdit::AddValue {
                    placement: post,
                    value: Box::new(Value {
                        mate: Some(mate),
                        coord,
                        by,
                    }),
                    fresh: Vec::new(),
                },
            );
        }
    }
    let ev = evaluate::<f64>(&doc, None, &CancelToken::new(), &opts, tol);
    assert!(
        assemble(&doc, &ev, tol).is_err(),
        "placed, the two posts interpenetrate in the world"
    );
    // The shelf's world pin off: the shelf and both posts are loose.
    let edit = unpin(&doc, bench.shelf_i);
    step(&mut doc, edit);
    let ev = evaluate::<f64>(&doc, None, &CancelToken::new(), &opts, tol);
    assert_eq!(
        ev.loose_roots(),
        vec![bench.shelf_i],
        "the shelf roots the one loose space"
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
