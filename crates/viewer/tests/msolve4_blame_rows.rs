//! **The finding's two observed sequences, through the session doors
//! a user actually drives** — and the invariant they were reported
//! against: for EVERY placement, the solve's blame and the evaluation's
//! row say the same thing.
//!
//! `msolve4_mate_memo` (editor-core) pins the kernel one document at
//! a time. These rows pump the SESSION between edits — which is what
//! made the defect visible in the first place, because a pump leaves
//! a prior behind for the next evaluation to serve from — and then
//! read the tree the chrome draws, so the badge a user sees is what
//! the kernel decided.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]
#![allow(clippy::unwrap_used)]

use crate::common;

use std::sync::Arc;

use pncad::document::{
    Doc, Evaluation, MateFault, MateRef, NodeErrorKind, NodeResult, ProfileProgram,
    RecipeNodeId,
};
use pncad::geom_core::Tol;
use pncad::select::ContactClass;
use viewer::session::{DocSession, SessionOp};
use viewer::tree::RowStatus;

/// Every disagreement between the solve's blame and the evaluation's
/// rows, over every placement: a faulted placement must be `Failed`
/// with that exact fault; an unfaulted one must be `Ok`.
fn disagreements(
    session: &DocSession,
    doc: &Doc<ProfileProgram>,
    ev: &Evaluation<f64>,
    tol: Tol,
) -> Vec<String> {
    let poses = common::solve(session, doc, tol);
    let mut out = Vec::new();
    for id in doc.placements() {
        match (poses.fault(id), ev.result(id)) {
            (Some(fault), Some(NodeResult::Failed(err))) => match &err.kind {
                NodeErrorKind::Mate(row) if **row == *fault => {}
                other => out.push(format!(
                    "{id:?}: solve faults {fault:?}, row says {other:?}"
                )),
            },
            (Some(fault), other) => {
                out.push(format!("{id:?}: solve faults {fault:?}, row is {other:?}"))
            }
            (None, Some(NodeResult::Ok(_))) => {}
            (None, other) => out.push(format!("{id:?}: unfaulted placement, row is {other:?}")),
        }
    }
    out
}

fn placement_fault(ev: &Evaluation<f64>, id: RecipeNodeId) -> MateFault {
    match ev.result(id) {
        Some(NodeResult::Failed(err)) => match &err.kind {
            NodeErrorKind::Mate(f) => (**f).clone(),
            other => panic!("{id:?}: expected a mate refusal, got {other:?}"),
        },
        other => panic!("{id:?}: expected Failed, got {other:?}"),
    }
}

fn placement_ok(ev: &Evaluation<f64>, id: RecipeNodeId) {
    assert!(
        matches!(ev.result(id), Some(NodeResult::Ok(_))),
        "{id:?}: expected Ok, got {:?}",
        ev.result(id)
    );
}

fn rest_shelf_on(
    session: &mut DocSession,
    bench: &common::asm::Bench,
    post: RecipeNodeId,
    offset: f64,
) -> MateRef {
    common::commit_mate(session, common::asm::shelf_rest_op(bench, post, offset))
}

fn remove(session: &mut DocSession, mate: MateRef) {
    let outcome = session.perform(SessionOp::RemoveConstraint { mate });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
}

fn landed(session: &DocSession) -> (Doc<ProfileProgram>, Arc<Evaluation<f64>>) {
    let doc = session.committed_doc().clone();
    let ev = Arc::clone(session.evaluation_arc().expect("a result landed"));
    (doc, ev)
}

fn check(
    session: &DocSession,
    tol: Tol,
    what: &str,
) -> (Doc<ProfileProgram>, Arc<Evaluation<f64>>) {
    let (doc, ev) = landed(session);
    let bad = disagreements(session, &doc, &ev, tol);
    assert!(bad.is_empty(), "{what}: blame and rows disagree: {bad:#?}");
    (doc, ev)
}

/// **A refusal reaches the placement chained to the one that refused,
/// pumped between the edits**: post_b's sound mate evaluates, a
/// contradiction then breaks the shelf's bundle it is placed against,
/// and post_b's row reports the refusal rather than the `Ok` its memo
/// entry still held. The reverse closes the row — remove one of the
/// pair and every row returns to `Ok` off the faulted prior.
#[test]
fn a_bundle_refusal_reaches_the_placement_that_evaluated_before_it() {
    let tol = Tol::witness();
    let bench = common::asm::bench("rev4-cluster", tol);
    let mut session = common::asm::open_bench(&bench, tol);
    common::asm::unpin(&mut session, bench.post_b);

    let sound = common::commit_mate(
        &mut session,
        common::asm::seat_op(
            &bench,
            bench.post_b,
            ContactClass::Rest,
            common::asm::middle_seat_alignment(),
        ),
    );
    assert_eq!(sound.placement, bench.post_b);
    let (_, first) = check(&session, tol, "sound alone");
    placement_ok(&first, bench.post_b);

    common::asm::unpin(&mut session, bench.shelf_i);
    let held = rest_shelf_on(&mut session, &bench, bench.post_a, 0.0);
    let offender = rest_shelf_on(&mut session, &bench, bench.post_a, 0.01);
    assert_eq!(offender.placement, bench.shelf_i);
    let (doc, second) = check(&session, tol, "after the offender");
    let carried = placement_fault(&second, bench.post_b);
    assert_eq!(
        Some(&carried),
        common::solve(&session, &doc, tol).fault(bench.post_b)
    );
    assert!(second.recomputed >= 1, "post_b must have re-run");

    let rows = session.tree_rows();
    let status_of = |id| common::status_of(&rows, id);
    assert!(matches!(status_of(bench.shelf_i), RowStatus::Failed { .. }));
    match status_of(bench.post_b) {
        RowStatus::Poisoned { through, .. } => assert_eq!(through, bench.shelf_i),
        other => panic!("post_b: expected Poisoned through the shelf, got {other:?}"),
    }

    // Reverse: remove the offender; post_b returns to Ok on the faulted
    // evaluation as prior.
    remove(&mut session, offender);
    session.pump();
    let (_, third) = check(&session, tol, "offender removed");
    placement_ok(&third, bench.post_b);
    let rows = session.tree_rows();
    for id in [bench.post_a, bench.post_b, bench.shelf_i] {
        assert_eq!(common::status_of(&rows, id), RowStatus::Ok, "{id:?}");
    }
    let _ = held;
    std::fs::remove_dir_all(&bench.dir).expect("removable");
}

/// **Two DIFFERENT faults in succession on one unedited constraint's
/// placement never serve a stale one.**
///
/// The shelf's bundle is faulted by a pair contradiction, then by a
/// different pair once the first offender is removed and another added.
/// The memo serves only `Ok` priors, so a stale fault can never be
/// served — this row is what says so on a document rather than on the
/// reuse rule, and it ends on the full repair with the faulted run as
/// prior.
#[test]
fn two_faults_in_succession_on_one_placement_never_serve_a_stale_one() {
    let tol = Tol::witness();
    let bench = common::asm::bench("rev4-contra", tol);
    let mut session = common::asm::open_bench(&bench, tol);
    common::asm::unpin(&mut session, bench.shelf_i);

    let held = rest_shelf_on(&mut session, &bench, bench.post_a, 0.0);
    check(&session, tol, "held alone");

    let added = rest_shelf_on(&mut session, &bench, bench.post_a, 0.01);
    let (_, ev) = check(&session, tol, "contradiction");
    let f1 = placement_fault(&ev, bench.shelf_i);
    assert!(
        matches!(&f1, MateFault::Contradictory { held: h, added: a, .. }
            if [*h, *a].contains(&held) && [*h, *a].contains(&added)),
        "{f1:?}"
    );
    let rows = session.tree_rows();
    assert!(matches!(
        common::status_of(&rows, bench.shelf_i),
        RowStatus::Failed { .. }
    ));

    // A second, different fault: the first offender removed, another
    // at a third offset added.
    remove(&mut session, added);
    let again = rest_shelf_on(&mut session, &bench, bench.post_a, 0.02);
    let (_, ev) = check(&session, tol, "a second contradiction");
    let f2 = placement_fault(&ev, bench.shelf_i);
    assert!(
        !matches!(&f2, MateFault::Contradictory { held: h, added: a, .. }
            if [*h, *a].contains(&added)),
        "stale pair contradiction carried: {f2:?}"
    );
    assert_ne!(
        f2, f1,
        "a second, different fault — not the first served again"
    );

    // Repair fully: Ok again, evaluated with the faulted run as prior.
    remove(&mut session, again);
    session.pump();
    let (_, ev) = check(&session, tol, "repaired");
    placement_ok(&ev, bench.shelf_i);
    assert!(ev.recomputed >= 1, "prior was Failed, so the placement re-ran");
    std::fs::remove_dir_all(&bench.dir).expect("removable");
}
