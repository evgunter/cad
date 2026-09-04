//! **Failed and Poisoned badges carry the TYPED payload's message.**
//!
//! GQ2's per-node result DAG is the whole subject: a failing node is
//! `Failed(NodeError)` and its descendants are `Poisoned { through }`,
//! independent subgraphs complete, and what a badge says is the typed
//! error's own rendering rather than a sentence this crate wrote.
//!
//! The rows that matter most are the downstream ones. A poisoned row
//! must say where the failure is and must NOT recite what it was —
//! the defect that reading gives is four instance rows carrying the
//! same paragraph of refusal prose — and the row it points at must be
//! one this same tree badges FAILED, or "upstream failure at node 5"
//! sends the user somewhere there is nothing to read.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use pncad::document::{BooleanOp, CancelToken, EvalOptions, NodeResult, evaluate};
use pncad::geom_core::Tol;
use pncad::select::ContactClass;
use viewer::session::{DocSession, SessionOp};
use viewer::tree::{self, RowStatus};

#[test]
fn a_failing_document_renders_failed_and_poisoned_from_the_typed_payloads() {
    let tol = Tol::witness();
    let (doc, extrude, moved) = common::broken_document(tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let rows = session.tree_rows();
    assert!(tree::has_faults(&rows));

    let failed = rows
        .iter()
        .find(|row| row.id == extrude)
        .expect("the extrude has a row");
    let RowStatus::Failed { message } = &failed.status else {
        panic!("expected Failed, got {:?}", failed.status);
    };
    assert_eq!(failed.status.badge(), "FAILED");
    assert!(!message.is_empty());

    // The message is the payload's, not this crate's: it must be
    // byte-identical to the shipped error's own rendering.
    let evaluation = session.evaluation().expect("a result landed");
    let Some(NodeResult::Failed(error)) = evaluation.result(extrude) else {
        panic!("the evaluation should report the extrude as failed");
    };
    assert_eq!(message, &error.to_string());

    let poisoned = rows
        .iter()
        .find(|row| row.id == moved)
        .expect("the transform has a row");
    match &poisoned.status {
        RowStatus::Poisoned { through, message } => {
            assert_eq!(*through, extrude, "poison names the failure it came from");
            assert_eq!(
                message.as_deref(),
                Some(tree::downstream_wording(extrude).as_str()),
                "a poisoned row POINTS at the cause's row; it does not recite it"
            );
        }
        other => panic!("expected Poisoned, got {other:?}"),
    }
    assert_eq!(poisoned.status.badge(), "POISONED");
}

#[test]
fn an_independent_subgraph_completes_beside_a_failure() {
    let tol = Tol::witness();
    let (doc, extrude, _moved) = common::broken_document(tol);
    // A second, unrelated body in the same document: GQ2's ratified
    // "a failure poisons only its descendants".
    let (doc, other_profile) = common::framed_square(&doc, 0.02, tol);
    let (doc, other_extrude) = common::inserted(
        &doc,
        pncad::document::Node::Extrude {
            profile: other_profile,
            distance: common::len(0.005),
        },
        tol,
    );
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let rows = session.tree_rows();

    let status_of = |id| common::status_of(&rows, id);
    assert!(matches!(status_of(extrude), RowStatus::Failed { .. }));
    assert_eq!(status_of(other_extrude), RowStatus::Ok);
    assert_eq!(status_of(other_profile), RowStatus::Ok);
}

#[test]
fn rows_before_the_first_result_read_as_unevaluated_rather_than_ok() {
    let tol = Tol::witness();
    let (doc, _profile, extrude) = common::parametric_plate(tol);
    let session = DocSession::inline(doc, tol);
    // Deliberately NOT pumped: nothing has been evaluated yet.
    let rows = session.tree_rows();
    assert!(rows.iter().all(|row| row.status == RowStatus::Unevaluated));
    assert!(
        !tree::has_faults(&rows),
        "unevaluated is not a fault; it is an absence of measurement"
    );
    assert_eq!(
        rows.iter()
            .find(|row| row.id == extrude)
            .map(|row| row.status.badge()),
        Some("—")
    );
}

#[test]
fn a_canceled_runs_missing_tail_reads_as_unevaluated() {
    // A canceled evaluation carries the completed PREFIX only, so
    // nodes past it have no entry at all. The tree must show that as
    // an absence rather than as success.
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = common::parametric_plate(tol);
    let cancel = CancelToken::new();
    cancel.cancel();
    let evaluation = evaluate::<f64>(&doc, None, &cancel, &EvalOptions::default(), tol);
    let rows = tree::rows(&doc, Some(&evaluation));
    assert!(!rows.is_empty());
    assert!(rows.iter().all(|row| row.status == RowStatus::Unevaluated));
    assert!(!tree::has_faults(&rows));
}

#[test]
fn the_tree_marks_the_documents_product_roots() {
    let tol = Tol::witness();
    let (doc, profile, extrude) = common::parametric_plate(tol);
    let rows = tree::rows(&doc, None);
    let root_ids: Vec<_> = rows
        .iter()
        .filter(|row| row.root)
        .map(|row| row.id)
        .collect();
    assert_eq!(
        root_ids,
        vec![extrude],
        "the extrude is the product; the profile it consumes is not"
    );
    assert_eq!(
        rows.iter()
            .find(|row| row.id == profile)
            .map(|row| row.kind),
        Some("Profile")
    );
}

/// **A refused mate solve badges the MATE as the cause and everything
/// else it reached as downstream.**
///
/// The kernel records one cluster refusal against every instance in
/// the cluster and every mate holding it together, each as that node's
/// own `Failed` — mates and instances are DAG leaves, so the placement
/// solve poisons across a graph the result DAG has no edges for. The
/// tree must not read that verbatim: the fault names the mate it is
/// about, so that mate's row is the cause and every other row it
/// reached — including instances the mate does not touch, and the
/// other mate in the cluster — reads as downstream of it.
#[test]
fn a_refused_mate_solve_names_the_mate_and_reads_every_other_row_downstream() {
    let tol = Tol::witness();
    let bench = common::asm::bench("badge-refusal", tol);
    let mut session = common::asm::open_bench(&bench, tol);

    // Two seat mates joining all three instances into ONE cluster:
    // post_a under the shelf's middle, post_b under its quarter point.
    // The second carries a clocking rider on a frame coincidence,
    // which the coset table decides against (`mate_clocking_redundant`
    // — a coincidence has already pinned the roll).
    let add_mate = |session: &mut DocSession, post, alignment| {
        common::insert(
            session,
            SessionOp::AddMate {
                a: common::asm::in_part(post, &bench.post_top),
                b: common::asm::in_part(bench.shelf_i, &bench.shelf_bottom),
                class: ContactClass::Rest,
                alignment,
            },
        )
    };
    let sound = add_mate(
        &mut session,
        bench.post_a,
        common::asm::seat_alignment(common::asm::SHELF_LENGTH / 2.0, None),
    );
    let offender = add_mate(
        &mut session,
        bench.post_b,
        common::asm::seat_alignment(common::asm::SHELF_LENGTH / 4.0, Some(0.3)),
    );
    // ONE evaluation over both mates. Pumping between them leaves the
    // sound mate's row reading `Ok` off the memo — a mate's key does
    // not carry the solve, so a cluster that breaks around it does not
    // re-run it. That is kernel behaviour this row neither exercises
    // nor endorses; what it pins is the attribution.
    session.pump();

    let rows = session.tree_rows();
    assert!(tree::has_faults(&rows), "the cluster refused: {rows:?}");
    let status_of = |id| common::status_of(&rows, id);

    // The offending mate is the cause, and the only row that is.
    let RowStatus::Failed { message } = status_of(offender) else {
        panic!(
            "the offending mate carries the cause: {:?}",
            status_of(offender)
        );
    };
    assert!(
        message.contains("mate_clocking_redundant"),
        "the kernel's own words on the mate's row: {message}"
    );
    let causes: Vec<_> = rows
        .iter()
        .filter(|row| matches!(row.status, RowStatus::Failed { .. }))
        .map(|row| row.id)
        .collect();
    assert_eq!(
        causes,
        vec![offender],
        "exactly one actionable row, and it is the mate the fault names"
    );

    // Every other row the refusal reached reads as downstream of it —
    // the two posts, the shelf, and the sound mate. `post_a` is the
    // row the issue is about: the offending mate does not touch it.
    // What each of them says is the POINTER, not a fourth copy of the
    // refusal prose the offending mate's row already carries.
    for (id, what) in [
        (
            bench.post_a,
            "an instance the offending mate does not touch",
        ),
        (bench.post_b, "an instance the offending mate does touch"),
        (bench.shelf_i, "the shelf the cluster hangs from"),
        (sound, "the sound mate in the refused cluster"),
    ] {
        match status_of(id) {
            RowStatus::Poisoned { through, message } => {
                assert_eq!(through, offender, "{what} points at the offending mate");
                assert_eq!(
                    message,
                    Some(tree::downstream_wording(offender)),
                    "{what} points at the cause's row"
                );
                assert!(
                    !message.unwrap_or_default().contains("mate_clocking"),
                    "{what} must not recite the refusal a user reads once, on the mate"
                );
            }
            other => panic!("{what} must read as downstream, got {other:?}"),
        }
        assert_eq!(status_of(id).badge(), "POISONED", "{what}");
    }

    std::fs::remove_dir_all(&bench.dir).expect("the fixture directory is removable");
}

/// **A contradiction between two mates blames both, and the rows it
/// reaches point at one the evaluation agrees is failing.**
///
/// Two mates on ONE pair at incompatible alignments: the fold names
/// `held` and `added` together, because neither is the wrong one on
/// the fault's own telling. What a downstream row must never do is
/// point at a row reading `Ok` — which is reachable, since a mate's
/// memo key does not carry the solve and the mate already evaluated
/// before the second one broke the pair.
#[test]
fn a_contradiction_points_downstream_rows_at_a_row_that_is_actually_failing() {
    let tol = Tol::witness();
    let bench = common::asm::bench("badge-contradiction", tol);
    let mut session = common::asm::open_bench(&bench, tol);

    let add_mate = |session: &mut DocSession, alignment| {
        common::insert(
            session,
            SessionOp::AddMate {
                a: common::asm::in_part(bench.post_a, &bench.post_top),
                b: common::asm::in_part(bench.shelf_i, &bench.shelf_bottom),
                class: ContactClass::Rest,
                alignment,
            },
        )
    };
    // The first mate lands and EVALUATES — the memo now holds an `Ok`
    // for it — and only then does the second one contradict it.
    let held = add_mate(
        &mut session,
        common::asm::seat_alignment(common::asm::SHELF_LENGTH / 2.0, None),
    );
    session.pump();
    let added = add_mate(
        &mut session,
        common::asm::seat_alignment(common::asm::SHELF_LENGTH / 2.0 + 0.01, None),
    );
    session.pump();

    let rows = session.tree_rows();
    let status_of = |id| common::status_of(&rows, id);
    // THE PREMISE THIS ROW RESTS ON, asserted rather than assumed: the
    // run reports `held` as `Ok` even though the fault names it. That
    // is the memo hazard (`work/issues/mate-memo-key-does-not-carry-
    // the-solve`), and it is what makes the two blamed mates
    // distinguishable here — fix the kernel and this row must be
    // rewritten rather than quietly passing as a copy of the one
    // above.
    assert_eq!(
        status_of(held),
        RowStatus::Ok,
        "the memo hazard is the premise: the first mate reads Ok in the run that blames it"
    );
    let failing: Vec<_> = rows
        .iter()
        .filter(|row| matches!(row.status, RowStatus::Failed { .. }))
        .map(|row| row.id)
        .collect();
    assert!(
        failing.contains(&added),
        "the mate that broke the pair is a cause: {rows:?}"
    );
    assert!(
        !failing.contains(&bench.post_a) && !failing.contains(&bench.shelf_i),
        "the mated instances are not causes: {rows:?}"
    );
    // Both mates are blamed, so whichever of them this evaluation
    // reports as failing is where the instances point — never at a row
    // the evaluation calls `Ok`.
    for instance in [bench.post_a, bench.shelf_i] {
        match status_of(instance) {
            RowStatus::Poisoned { through, message } => {
                assert!(
                    [held, added].contains(&through),
                    "the instance points at one of the two blamed mates, got {through:?}"
                );
                assert!(
                    matches!(status_of(through), RowStatus::Failed { .. }),
                    "and that mate's own row reports the failure: {:?}",
                    status_of(through)
                );
                assert_eq!(
                    message,
                    Some(tree::downstream_wording(through)),
                    "the message points at that mate's row"
                );
            }
            other => panic!("a mated instance must read as downstream, got {other:?}"),
        }
    }
    // post_b is in no cluster with them: an independent subgraph
    // completes, exactly as GQ2 says.
    assert_eq!(status_of(bench.post_b), RowStatus::Ok);

    std::fs::remove_dir_all(&bench.dir).expect("the fixture directory is removable");
}

/// **A row poisoned through a row that is ITSELF downstream points at
/// the terminal cause, not at a POISONED row.**
///
/// The reachable document: a boolean over two instances of a cluster
/// that then refuses. The kernel poisons the boolean through its first
/// blocking input — an instance — and reports that instance as its own
/// `Failed`; the tree redraws the instance as downstream of the mate.
/// Read verbatim the boolean would point at a row drawn POISONED and
/// weak, so the user's walk would be two hops through a row with
/// nothing to act on.
#[test]
fn a_boolean_over_a_refused_clusters_instances_points_at_the_mate() {
    let tol = Tol::witness();
    let bench = common::asm::bench("badge-two-hop", tol);
    let mut session = common::asm::open_bench(&bench, tol);

    // The boolean lands first, over two instances that are both `Ok`:
    // the operand seat admits an instance (`combine::denotes_body`).
    let boolean = common::insert(
        &mut session,
        SessionOp::AddBoolean {
            op: BooleanOp::Union,
            a: bench.post_a,
            b: bench.shelf_i,
        },
    );
    session.pump();
    assert_eq!(
        common::status_of(&session.tree_rows(), boolean),
        RowStatus::Ok,
        "the boolean builds before the cluster refuses"
    );

    let add_mate = |session: &mut DocSession, post, alignment| {
        common::insert(
            session,
            SessionOp::AddMate {
                a: common::asm::in_part(post, &bench.post_top),
                b: common::asm::in_part(bench.shelf_i, &bench.shelf_bottom),
                class: ContactClass::Rest,
                alignment,
            },
        )
    };
    add_mate(
        &mut session,
        bench.post_a,
        common::asm::seat_alignment(common::asm::SHELF_LENGTH / 2.0, None),
    );
    let offender = add_mate(
        &mut session,
        bench.post_b,
        common::asm::seat_alignment(common::asm::SHELF_LENGTH / 4.0, Some(0.3)),
    );
    session.pump();

    // The kernel's own reading: poisoned through the INSTANCE, which
    // is what makes the hop a real one rather than a hypothetical.
    let (_, ev) = session.landed_pair().expect("landed");
    assert_eq!(
        ev.result(boolean).and_then(NodeResult::poisoned_through),
        Some(bench.post_a),
        "the evaluation poisons the boolean through its first blocking input"
    );

    let rows = session.tree_rows();
    match common::status_of(&rows, boolean) {
        RowStatus::Poisoned { through, message } => {
            assert_eq!(
                through, offender,
                "the boolean points past the instance at the mate that refused"
            );
            assert_eq!(message, Some(tree::downstream_wording(offender)));
        }
        other => panic!("the boolean reads as downstream, got {other:?}"),
    }

    // The invariant that makes the pointer worth following, over the
    // whole tree: every POISONED row names a row THIS TREE badges
    // FAILED, so one hop lands on words to read.
    for row in &rows {
        if let RowStatus::Poisoned { through, .. } = &row.status {
            assert!(
                matches!(common::status_of(&rows, *through), RowStatus::Failed { .. }),
                "{:?} points at {through:?}, which draws {}",
                row.id,
                common::status_of(&rows, *through).badge()
            );
        }
    }

    std::fs::remove_dir_all(&bench.dir).expect("the fixture directory is removable");
}
