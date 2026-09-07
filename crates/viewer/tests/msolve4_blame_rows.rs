//! **The finding's two observed sequences, through the session doors
//! a user actually drives** — and the invariant they were reported
//! against: for EVERY node, the solve's blame and the evaluation's
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
    Doc, Evaluation, MateFault, MateRole, Node, NodeErrorKind, NodeResult, ProfileProgram,
    RecipeNodeId, SitedRef, ValuePayload,
};
use pncad::geom_core::Tol;
use pncad::select::ContactClass;
use viewer::session::{DocSession, SessionOp};
use viewer::tree::RowStatus;

/// Every disagreement between the solve's blame and the evaluation's
/// rows, over every live node: a faulted node must be `Failed` with
/// that exact fault; an unfaulted mate must be `Ok` with the solve's
/// role.
fn disagreements(
    session: &DocSession,
    doc: &Doc<ProfileProgram>,
    ev: &Evaluation<f64>,
    tol: Tol,
) -> Vec<String> {
    let poses = common::solve(session, doc, tol);
    let mut out = Vec::new();
    for &id in doc.order() {
        let is_mate = matches!(doc.node(id), Some(Node::Mate { .. }));
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
            (None, Some(NodeResult::Ok(v))) if is_mate => match &v.payload {
                ValuePayload::Mate(role) if Some(*role) == poses.role(id) => {}
                other => out.push(format!(
                    "{id:?}: solve role {:?}, row payload {other:?}",
                    poses.role(id)
                )),
            },
            (None, other) if is_mate => {
                out.push(format!("{id:?}: unfaulted mate, row is {other:?}"))
            }
            (None, _) => {}
        }
    }
    out
}

fn mate_row_fault(ev: &Evaluation<f64>, id: RecipeNodeId) -> MateFault {
    match ev.result(id) {
        Some(NodeResult::Failed(err)) => match &err.kind {
            NodeErrorKind::Mate(f) => (**f).clone(),
            other => panic!("{id:?}: expected a mate refusal, got {other:?}"),
        },
        other => panic!("{id:?}: expected Failed, got {other:?}"),
    }
}

fn mate_row_role(ev: &Evaluation<f64>, id: RecipeNodeId) -> MateRole {
    match ev.result(id) {
        Some(NodeResult::Ok(v)) => match &v.payload {
            ValuePayload::Mate(role) => *role,
            other => panic!("{id:?}: expected a mate payload, got {other:?}"),
        },
        other => panic!("{id:?}: expected Ok, got {other:?}"),
    }
}

fn add_seat(
    session: &mut DocSession,
    bench: &common::asm::Bench,
    post: RecipeNodeId,
    b_x: f64,
    clocking: Option<f64>,
) -> RecipeNodeId {
    common::insert(
        session,
        SessionOp::AddMate {
            a: SitedRef::at_mint(common::asm::in_part(post, &bench.post_top)),
            b: SitedRef::at_mint(common::asm::in_part(bench.shelf_i, &bench.shelf_bottom)),
            class: ContactClass::Rest,
            alignment: common::asm::seat_alignment(b_x, clocking),
        },
    )
}

fn delete(session: &mut DocSession, node: RecipeNodeId) {
    let outcome = session.perform(SessionOp::DeleteNode { node });
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

/// **The finding's first shape, pumped between the edits**: a sound
/// mate evaluates, a second mate then breaks the cluster around it,
/// and the sound mate's row reports the refusal rather than the `Ok`
/// its memo entry still held. The reverse closes the row — delete the
/// offender and every row returns to `Ok` off the faulted prior.
#[test]
fn a_cluster_refusal_reaches_the_mate_that_evaluated_before_it() {
    let tol = Tol::witness();
    let bench = common::asm::bench("rev4-cluster", tol);
    let mut session = common::asm::open_bench(&bench, tol);

    let sound = add_seat(
        &mut session,
        &bench,
        bench.post_a,
        common::asm::SHELF_LENGTH / 2.0,
        None,
    );
    session.pump();
    let (_, first) = check(&session, tol, "sound alone");
    assert_eq!(mate_row_role(&first, sound), MateRole::Determining);

    let offender = add_seat(
        &mut session,
        &bench,
        bench.post_b,
        common::asm::SHELF_LENGTH / 4.0,
        Some(0.3),
    );
    session.pump();
    let (doc, second) = check(&session, tol, "after the offender");
    let carried = mate_row_fault(&second, sound);
    assert_eq!(
        Some(&carried),
        common::solve(&session, &doc, tol).fault(sound)
    );
    assert!(second.recomputed >= 1, "the sound mate must have re-run");

    let rows = session.tree_rows();
    let status_of = |id| common::status_of(&rows, id);
    assert!(matches!(status_of(offender), RowStatus::Failed { .. }));
    for id in [sound, bench.post_a, bench.post_b, bench.shelf_i] {
        match status_of(id) {
            RowStatus::Poisoned { through, .. } => assert_eq!(through, offender, "{id:?}"),
            other => panic!("{id:?}: expected Poisoned through the offender, got {other:?}"),
        }
    }
    // The row pointed at is one the tree badges FAILED.
    assert!(matches!(status_of(offender), RowStatus::Failed { .. }));

    // Reverse: delete the offender; the sound mate returns to Ok on
    // the faulted evaluation as prior.
    delete(&mut session, offender);
    session.pump();
    let (_, third) = check(&session, tol, "offender deleted");
    assert_eq!(mate_row_role(&third, sound), MateRole::Determining);
    let rows = session.tree_rows();
    for id in [sound, bench.post_a, bench.post_b, bench.shelf_i] {
        assert_eq!(common::status_of(&rows, id), RowStatus::Ok, "{id:?}");
    }
    std::fs::remove_dir_all(&bench.dir).expect("removable");
}

/// **The finding's second shape, and then two DIFFERENT faults in
/// succession on one unedited mate.**
///
/// The succession is the part no single edit reaches: a mate faulted
/// by a pair contradiction, then faulted again by a cluster refusal
/// once the contradiction is removed. The memo serves only `Ok`
/// priors, so a stale fault can never be served — this row is what
/// says so on a document rather than on the reuse rule, and it ends
/// on the full repair with the faulted run as prior.
#[test]
fn two_faults_in_succession_on_one_mate_never_serve_a_stale_one() {
    let tol = Tol::witness();
    let bench = common::asm::bench("rev4-contra", tol);
    let mut session = common::asm::open_bench(&bench, tol);

    let held = add_seat(
        &mut session,
        &bench,
        bench.post_a,
        common::asm::SHELF_LENGTH / 2.0,
        None,
    );
    session.pump();
    check(&session, tol, "held alone");

    let added = add_seat(
        &mut session,
        &bench,
        bench.post_a,
        common::asm::SHELF_LENGTH / 2.0 + 0.01,
        None,
    );
    session.pump();
    let (_, ev) = check(&session, tol, "contradiction");
    let f1 = mate_row_fault(&ev, held);
    assert!(
        matches!(f1, MateFault::Contradictory { held: h, added: a, .. } if h == held && a == added),
        "{f1:?}"
    );
    let rows = session.tree_rows();
    let status_of = |id| common::status_of(&rows, id);
    assert!(matches!(status_of(held), RowStatus::Failed { .. }));
    assert!(matches!(status_of(added), RowStatus::Failed { .. }));
    for id in [bench.post_a, bench.shelf_i] {
        match status_of(id) {
            RowStatus::Poisoned { through, .. } => {
                assert_eq!(through, held, "first named in the fault's order");
                assert!(matches!(status_of(through), RowStatus::Failed { .. }));
            }
            other => panic!("{id:?}: {other:?}"),
        }
    }

    // A second, different fault on `held` without repairing the first:
    // the cluster now also refuses through post_b's clocking rider.
    let offender = add_seat(
        &mut session,
        &bench,
        bench.post_b,
        common::asm::SHELF_LENGTH / 4.0,
        Some(0.3),
    );
    session.pump();
    let (_, ev) = check(&session, tol, "contradiction + offender");
    // Faulted under both at once; which fault wins is the solve's
    // business, and the row's job is only to carry the one it recorded
    // (`check` above asserted exactly that, for every node).
    let _ = mate_row_fault(&ev, held);

    // Remove the contradiction: held's fault must now be the cluster
    // refusal (a different fault), never the stale contradiction.
    delete(&mut session, added);
    session.pump();
    let (_, ev) = check(&session, tol, "offender only");
    let f3 = mate_row_fault(&ev, held);
    // The offender's fault is itself a self-contradiction (held == added
    // == offender, "mate_clocking_redundant"); what must not survive is
    // the PAIR contradiction naming held and added.
    assert!(
        !matches!(f3, MateFault::Contradictory { held: h, added: a, .. } if h == held && a == added),
        "stale pair contradiction carried: {f3:?}"
    );
    assert_ne!(
        f3, f1,
        "a second, different fault — not the first served again"
    );

    // Repair fully: Ok again with the same role as before any fault,
    // evaluated with the faulted run as prior.
    delete(&mut session, offender);
    session.pump();
    let (_, ev) = check(&session, tol, "repaired");
    assert_eq!(mate_row_role(&ev, held), MateRole::Determining);
    assert!(ev.recomputed >= 1, "prior was Failed, so the mate re-ran");
    std::fs::remove_dir_all(&bench.dir).expect("removable");
}
