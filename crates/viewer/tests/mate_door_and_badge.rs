//! **The mate door and a space of a copy's own, through the session**
//! (ASSEMBLY.md A11 (2)): a committed mate through undo and redo, and
//! the at-rest badge over a space other than the world.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::asm;
use pncad::geom_core::Tol;
use viewer::session::SessionOp;

/// **The mate door through the session's history**: committing a mate
/// between post_b, in a space of its own, and the shelf writes post_b's
/// bundle and leaves the shelf's as it was; undo
/// restores the document, and redo commits it again.
#[test]
fn the_mate_door_commits_through_undo_and_redo() {
    let tol = Tol::witness();
    let bench = asm::bench("r2door", tol);
    let mut session = asm::open_bench(&bench, tol);
    let own = asm::unpin(&mut session, bench.post_b);
    let before = session.doc().clone();
    assert_eq!(before.node_space(bench.post_b), Some(own));
    let shelf_bundle = crate::fixture::bundle(&before, bench.shelf_i);
    let mate = common::commit_mate(
        &mut session,
        asm::seat_op(&bench, bench.post_b, asm::middle_seat_values()),
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
}

/// **The at-rest badge checks a `⊔` the product does not read** — the
/// at-rest check reads every `⊔`'s census. Two posts seated at one spot under the shelf
/// fill one volume in the world, and the kernel reports one loud
/// interference. With the shelf placed into a space of its own, the
/// three live in that space beside one world instance, and a `⊔` of the
/// three is authored there; the product has no overlap, the gate checks
/// the `⊔` by itself and reports the posts' overlap there, and the badge
/// counts it rather than report the product clear.
#[test]
fn the_at_rest_badge_checks_a_space_of_its_own() {
    use pncad::document::{
        CancelToken, CheckEvidence, ChecksConfig, EvalOptions, evaluate, load, run_checks, save,
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
    let seat = |post| {
        crate::fixture::rest(
            common::head(asm::in_part(post, &bench.post_top)),
            common::head(asm::in_part(bench.shelf_i, &bench.shelf_bottom)),
        )
    };
    // The world-side lone instance, far off.
    asm::instance_at(&mut doc, bench.post, [1.0, 0.0, 0.0], &opts, tol);
    // Both posts at ONE spot under the shelf: undeclared interference.
    for post in [bench.post_a, bench.post_b] {
        doc = crate::fixture::mated(doc, post, seat(post), &asm::middle_seat_values()).0;
    }
    let ev = evaluate::<f64>(&doc, None, &CancelToken::new(), &opts, tol);
    let loud = |kernel: &Result<pncad::document::ChecksReport, _>| {
        match kernel {
        Ok(report) => Some(
            report
                .findings
                .iter()
                .filter(|f| matches!(&f.evidence, CheckEvidence::Interference { finding } if finding.is_loud()))
                .count(),
        ),
        Err(_) => None,
    }
    };
    let kernel = run_checks(&doc, &ev, &ChecksConfig::default(), tol);
    assert_eq!(
        loud(&kernel),
        Some(1),
        "placed, the two posts interpenetrate in the world: {:?}",
        kernel.err()
    );
    // The shelf into a space of its own, and the posts with it: they
    // read the shelf's underside.
    let (next, space) = crate::fixture::into_new_space(doc, bench.shelf_i);
    doc = next;
    for post in [bench.post_a, bench.post_b] {
        doc = crate::fixture::mated(doc, post, seat(post), &asm::middle_seat_values()).0;
        assert_eq!(
            doc.node_space(post),
            Some(space),
            "the post joins the shelf"
        );
    }
    let three = [bench.shelf_i, bench.post_a, bench.post_b]
        .into_iter()
        .map(|copy| {
            pncad::document::BodyRead::plain(pncad::document::Operand::Var(
                doc.output(copy, 0).expect("a copy"),
            ))
        })
        .collect();
    doc = crate::fixture::insert(
        doc,
        pncad::document::Node::Assemble {
            members: pncad::document::Bodies::Spelled(three),
        },
    )
    .0;
    let ev = evaluate::<f64>(&doc, None, &CancelToken::new(), &opts, tol);
    let kernel = run_checks(&doc, &ev, &ChecksConfig::default(), tol);
    assert_eq!(
        loud(&kernel),
        Some(1),
        "the at-rest check reads the shelf's `⊔` by itself: {:?}",
        kernel.err()
    );
    std::fs::write(&bench.asm_path, save(&doc, &[], tol).unwrap()).unwrap();
    let session = asm::open_bench(&bench, tol);
    let badge = session.at_rest().cloned();
    assert_eq!(
        badge,
        Some(viewer::session::AtRestBadge {
            loud: 1,
            quiet: 0,
            could_not_look: 0,
        }),
        "the badge counts the space's overlap with the kernel"
    );
}
