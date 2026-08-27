//! **Review R1's consumer suite for GUI-3 (PR #1101)** — an
//! independent derivation of the unit's claims, with its own fixtures
//! (`memories/review-and-dependency-policy.md`: pointing a review
//! suite at the implementation's own constants would spend exactly
//! the independence that is its value).
//!
//! Shapes per `memories/test-suite-cost.md`: every row here is a
//! static-witness row (deterministic fixtures authored through the
//! public doors); nothing samples, so nothing needs a seed or an
//! effort dial.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use pncad::document::{
    Dimension, Doc, DocEdit, DocParam, EvalOutcome, Expr, LoopProgram, Node, ParamName,
    ProfileProgram, RecipeNodeId, SlotId, apply,
};
use pncad::geom_core::Tol;
use pncad::profile::SketchPlane;
use viewer::evalseam::EvalDone;
use viewer::history::History;
use viewer::props::{SlotDriver, SlotValue};
use viewer::session::{DocSession, Landing, Refusal, Selection, SessionOp};
use viewer::tree::RowStatus;
use viewer::{docio, props, tree};

/// R1's own parameter name — not the implementation suites'.
fn depth_param() -> ParamName {
    ParamName::new("r1_depth")
}

/// A triangle profile — deliberately not the square the unit's own
/// fixtures use.
fn triangle(side: f64) -> Node<ProfileProgram> {
    Node::Profile(ProfileProgram {
        plane: SketchPlane::xy(),
        loops: vec![
            LoopProgram::polygon([(0.0, 0.0), (side, 0.0), (0.0, side)]).expect("finite corners"),
        ],
    })
}

fn len(metres: f64) -> Expr {
    Expr::literal(metres, Dimension::Length).expect("a finite length")
}

fn scl(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).expect("a finite scalar")
}

fn applied(
    doc: &Doc<ProfileProgram>,
    edit: DocEdit<ProfileProgram>,
    tol: Tol,
) -> Doc<ProfileProgram> {
    apply(doc, &edit, tol)
        .expect("the fixture edit applies")
        .doc
}

fn insert(
    doc: &Doc<ProfileProgram>,
    node: Node<ProfileProgram>,
    tol: Tol,
) -> (Doc<ProfileProgram>, RecipeNodeId) {
    let out = apply(doc, &DocEdit::InsertNode { node }, tol).expect("the insert applies");
    (out.doc, out.record.minted.expect("an insert mints an id"))
}

/// A wedge whose extrude distance is `r1_depth * 3`: a driven slot
/// over one parameter, R1's own derivation of the affordance fixture.
fn wedge(tol: Tol) -> (Doc<ProfileProgram>, RecipeNodeId, RecipeNodeId) {
    let doc: Doc<ProfileProgram> = Doc::empty_derived("r1-wedge", tol);
    let doc = applied(
        &doc,
        DocEdit::SetDocParam {
            name: depth_param(),
            value: DocParam::Continuous {
                dim: Dimension::Length,
                value: 0.002,
            },
        },
        tol,
    );
    let (doc, profile) = insert(&doc, triangle(0.03), tol);
    let (doc, extrude) = insert(
        &doc,
        Node::Extrude {
            profile,
            distance: Expr::mul(Expr::param(depth_param(), Dimension::Length), scl(3.0))
                .expect("length * scalar is a length"),
        },
        tol,
    );
    (doc, profile, extrude)
}

fn set_depth(session: &mut DocSession, metres: f64) {
    let outcome = session.perform(SessionOp::SetParam {
        name: depth_param(),
        value: SlotValue::Continuous(metres),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
}

fn depth_of(doc: &Doc<ProfileProgram>) -> f64 {
    match props::param_rows(doc)
        .into_iter()
        .find(|row| row.name == depth_param())
        .expect("the fixture declares r1_depth")
        .value
    {
        SlotValue::Continuous(v) => v,
        SlotValue::Count(_) => panic!("r1_depth is continuous"),
    }
}

fn tempdir(label: &str) -> std::path::PathBuf {
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_nanos());
    let dir = std::env::temp_dir().join(format!("{label}-{unique}"));
    std::fs::create_dir_all(&dir).expect("the fixture directory is creatable");
    dir
}

/// The abandoned branch keeps its whole SUBTREE, not just the one
/// state undo stepped off. The unit's own row abandons a single leaf;
/// this one abandons a two-edit chain and asserts both states, their
/// edits, their parent links and the branch point's book-keeping.
#[test]
fn r1_an_abandoned_branch_keeps_its_whole_subtree() {
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = wedge(tol);
    let mut session = DocSession::inline(doc, tol);

    set_depth(&mut session, 0.003); // a
    let a = session.history().current();
    set_depth(&mut session, 0.004); // b (child of a)
    let b = session.history().current();
    set_depth(&mut session, 0.005); // c (child of b)
    let c = session.history().current();

    session.perform(SessionOp::Undo); // -> b
    session.perform(SessionOp::Undo); // -> a
    set_depth(&mut session, 0.006); // d, sibling of b
    let d = session.history().current();

    let history = session.history();
    assert_eq!(history.len(), 5, "root + a + b + c + d, nothing dropped");
    assert_eq!(
        history.entry(a).children(),
        &[b, d],
        "both branches hang off a"
    );
    assert_eq!(
        history.entry(a).active_child(),
        Some(d),
        "redo follows the new work"
    );
    // The abandoned chain is intact end to end: states, edits, links.
    assert_eq!(history.entry(b).children(), &[c], "b keeps its own child");
    assert_eq!(history.entry(c).parent(), Some(b));
    assert_eq!(depth_of(history.entry(b).doc()), 0.004);
    assert_eq!(depth_of(history.entry(c).doc()), 0.005);
    assert!(
        history.entry(c).edit().is_some(),
        "the abandoned edit is retained"
    );
    assert_eq!(depth_of(history.entry(d).doc()), 0.006);
}

/// Redo follows `active_child` across MORE than one level: after
/// undoing to the root past a branch point, two redos walk the new
/// branch, not the abandoned one.
#[test]
fn r1_redo_walks_the_new_branch_across_two_levels() {
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = wedge(tol);
    let mut session = DocSession::inline(doc, tol);
    set_depth(&mut session, 0.003); // a
    set_depth(&mut session, 0.004); // b
    session.perform(SessionOp::Undo); // -> a
    session.perform(SessionOp::Undo); // -> root
    set_depth(&mut session, 0.007); // e, sibling of a
    set_depth(&mut session, 0.008); // f, child of e

    session.perform(SessionOp::Undo); // -> e
    session.perform(SessionOp::Undo); // -> root
    assert!(session.history().can_redo());
    session.perform(SessionOp::Redo); // -> e (not a)
    assert_eq!(
        depth_of(session.committed_doc()),
        0.007,
        "redo took the new branch"
    );
    session.perform(SessionOp::Redo); // -> f
    assert_eq!(depth_of(session.committed_doc()), 0.008);
    assert!(!session.history().can_redo(), "f is the branch tip");
}

/// Open→save is stable at the level of FILE BYTES, not merely
/// document `bit_eq`: a file saved from a session, opened, and saved
/// again with no edits writes the identical bytes. This is the claim
/// "open→save is byte-stable" taken at its word.
#[test]
fn r1_open_then_save_reproduces_the_file_bytes_exactly() {
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = wedge(tol);
    let mut session = DocSession::inline(doc, tol);
    set_depth(&mut session, 0.0035);
    set_depth(&mut session, 0.0042);
    session.perform(SessionOp::Undo); // save from mid-path on purpose

    let dir = tempdir("r1-byte-stable");
    let first = dir.join("first.pncad");
    let second = dir.join("second.pncad");
    assert!(
        session
            .perform(SessionOp::Save(first.clone()))
            .refusal
            .is_none()
    );

    let reopened = docio::open(&first, tol).expect("the file opens");
    docio::save_path(&second, &reopened, tol).expect("the reopened history saves");
    assert_eq!(
        std::fs::read_to_string(&first).expect("first is readable"),
        std::fs::read_to_string(&second).expect("second is readable"),
        "open -> save must be the identity on the file's bytes"
    );
    // And the mid-path save recorded ONE edit (root snapshot + the
    // path to the cursor), not the tree's two.
    assert_eq!(reopened.path_edits().len(), 1);
    std::fs::remove_dir_all(&dir).expect("the fixture directory is removable");
}

/// A result for a generation the session never issued — ahead of it,
/// not behind — is discarded too: `land` accepts exactly the current
/// generation, nothing else.
#[test]
fn r1_a_result_from_a_future_generation_does_not_land() {
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = wedge(tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let landed = session
        .evaluation_arc()
        .expect("the first result landed")
        .clone();
    let future = session.generation().next().next();
    assert_eq!(
        session.land(EvalDone {
            generation: future,
            evaluation: landed,
        }),
        Landing::Stale,
        "only the current generation lands"
    );
}

/// What a user Cancel does at the SESSION level, pinned.
///
/// **Updated at the fix pass, on this row's own instruction.** As
/// written by the review it pinned the DEFECT: the canceled run's
/// result carried the current generation, so it landed, the session's
/// evaluation became the empty prefix, the tree went all-`Unevaluated`
/// and `busy()` went dark — a panel claiming to be neither building
/// anything nor waiting for anything. The row said "if the fix pass
/// changes the policy, this row is the one to update"; it did, and this
/// is that update, subject unchanged and expectation flipped.
///
/// The fixed policy: a result that did not COMPLETE never replaces a
/// landed evaluation, so the last good picture stays, `busy()` goes on
/// reporting that the picture is older than the document, and
/// `running()` says nothing is working on it.
#[test]
fn r1_a_user_cancel_keeps_the_last_completed_result_on_screen() {
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = wedge(tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump(); // first result lands, Completed
    let good = session
        .evaluation_arc()
        .expect("the first result landed")
        .clone();

    set_depth(&mut session, 0.009); // busy again
    assert!(session.busy());
    session.perform(SessionOp::CancelEvaluation);
    let landings = session.pump();
    assert_eq!(
        landings,
        vec![Landing::Canceled],
        "current generation, but not a completed run"
    );
    assert!(
        session.busy(),
        "the picture is still older than the document"
    );
    assert!(!session.running(), "and nothing is working on it");
    let evaluation = session.evaluation().expect("a result is on screen");
    assert_eq!(
        evaluation.outcome,
        EvalOutcome::Completed,
        "the last completed run, not the canceled prefix"
    );
    assert!(std::sync::Arc::ptr_eq(
        session.evaluation_arc().expect("still shown"),
        &good
    ));
    // The tree still shows the run that finished.
    let rows = session.tree_rows();
    assert!(
        rows.iter().all(|row| row.status != RowStatus::Unevaluated),
        "a cancel does not blank the tree"
    );
}

/// The text door swings BOTH ways: a literal slot accepts an
/// expression, after which the slot refuses numbers — the affordance's
/// two directions composed into one walk.
#[test]
fn r1_an_expression_written_over_a_literal_slot_makes_it_refuse_numbers() {
    let tol = Tol::witness();
    let doc: Doc<ProfileProgram> = Doc::empty_derived("r1-literal-first", tol);
    let doc = applied(
        &doc,
        DocEdit::SetDocParam {
            name: depth_param(),
            value: DocParam::Continuous {
                dim: Dimension::Length,
                value: 0.002,
            },
        },
        tol,
    );
    let (doc, profile) = insert(&doc, triangle(0.03), tol);
    let (doc, extrude) = insert(
        &doc,
        Node::Extrude {
            profile,
            distance: len(0.005), // literal to begin with
        },
        tol,
    );
    let mut session = DocSession::inline(doc, tol);
    session.perform(SessionOp::Select(Selection::Node(extrude)));

    // A literal slot takes a number...
    assert!(
        session
            .perform(SessionOp::SetSlot {
                node: extrude,
                slot: SlotId::Distance,
                value: SlotValue::Continuous(0.006),
            })
            .refusal
            .is_none()
    );
    // ...and an expression, through the text door.
    let outcome = session.perform(SessionOp::SetSlotExpression {
        node: extrude,
        slot: SlotId::Distance,
        text: "r1_depth * 4.0".to_owned(),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(outcome.committed.len(), 1);

    // Now the same slot is driven and refuses the number it took
    // before.
    let row = session
        .slot_rows()
        .into_iter()
        .find(|row| row.slot == SlotId::Distance)
        .expect("the extrude still has a distance");
    assert!(matches!(row.driver, SlotDriver::Expression { .. }));
    assert_eq!(row.value, Ok(SlotValue::Continuous(0.008)));
    let refused = session.perform(SessionOp::SetSlot {
        node: extrude,
        slot: SlotId::Distance,
        value: SlotValue::Continuous(0.02),
    });
    assert!(matches!(
        refused.refusal,
        Some(Refusal::DrivenByExpression { .. })
    ));
}

/// Structural edits are fenced off while a gesture is in flight: the
/// single-writer discipline the scratch document depends on.
#[test]
fn r1_document_edits_are_refused_while_a_gesture_is_in_flight() {
    let tol = Tol::witness();
    let doc: Doc<ProfileProgram> = Doc::empty_derived("r1-gesture-fence", tol);
    let (doc, profile) = insert(&doc, triangle(0.03), tol);
    let (doc, extrude) = insert(
        &doc,
        Node::Extrude {
            profile,
            distance: len(0.005),
        },
        tol,
    );
    let mut session = DocSession::inline(doc, tol);
    assert!(
        session
            .perform(SessionOp::BeginGesture {
                node: extrude,
                slot: SlotId::Distance,
            })
            .refusal
            .is_none()
    );
    for op in [
        SessionOp::SetSlot {
            node: extrude,
            slot: SlotId::Distance,
            value: SlotValue::Continuous(0.02),
        },
        SessionOp::SetSlotExpression {
            node: extrude,
            slot: SlotId::Distance,
            text: "1.0 m".to_owned(),
        },
        SessionOp::Undo,
        SessionOp::Redo,
        SessionOp::BeginGesture {
            node: extrude,
            slot: SlotId::Distance,
        },
    ] {
        let outcome = session.perform(op.clone());
        assert!(
            matches!(outcome.refusal, Some(Refusal::GestureInFlight)),
            "{op:?} must be refused mid-gesture, got {:?}",
            outcome.refusal
        );
        assert!(outcome.committed.is_empty());
    }
    // The fence lifts with the gesture.
    session.perform(SessionOp::CancelGesture);
    assert!(
        session
            .perform(SessionOp::SetSlot {
                node: extrude,
                slot: SlotId::Distance,
                value: SlotValue::Continuous(0.02),
            })
            .refusal
            .is_none()
    );
}

/// A poison chain TWO hops deep still reports the root cause: the
/// grandchild's badge message is the failed ancestor's own rendering,
/// reached through the evaluation, not a placeholder.
#[test]
fn r1_a_two_hop_poison_chain_reports_the_root_cause() {
    let tol = Tol::witness();
    let doc: Doc<ProfileProgram> = Doc::empty_derived("r1-poison-chain", tol);
    let (doc, profile) = insert(&doc, triangle(0.03), tol);
    let (doc, extrude) = insert(
        &doc,
        Node::Extrude {
            profile,
            // Well-dimensioned at the door, non-finite at evaluation.
            distance: Expr::div(len(0.005), scl(0.0)).expect("length / scalar"),
        },
        tol,
    );
    let transform = |input| Node::Transform {
        input,
        translation: [len(0.001), len(0.0), len(0.0)],
        rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
        rotation_angle: Expr::literal(0.0, Dimension::Angle).expect("finite"),
    };
    let (doc, child) = insert(&doc, transform(extrude), tol);
    let (doc, grandchild) = insert(&doc, transform(child), tol);

    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let rows = session.tree_rows();
    assert!(tree::has_faults(&rows));
    let failed_message = match &rows
        .iter()
        .find(|row| row.id == extrude)
        .expect("the extrude has a row")
        .status
    {
        RowStatus::Failed { message } => message.clone(),
        other => panic!("expected Failed, got {other:?}"),
    };
    for id in [child, grandchild] {
        match &rows
            .iter()
            .find(|row| row.id == id)
            .expect("the descendant has a row")
            .status
        {
            RowStatus::Poisoned { message, .. } => {
                assert_eq!(
                    message.as_deref(),
                    Some(failed_message.as_str()),
                    "every generation of the poison chain names the root cause"
                );
            }
            other => panic!("expected Poisoned for {id:?}, got {other:?}"),
        }
    }
}

/// `History::replayed` puts the cursor at the log's END: the opened
/// document is on its latest state, every logged edit is undoable, and
/// nothing is redoable until an undo happens.
#[test]
fn r1_a_replayed_history_opens_at_the_tip_with_the_log_undoable() {
    let tol = Tol::witness();
    let (doc, _profile, extrude) = wedge(tol);
    let edits = vec![
        DocEdit::SetParam {
            node: extrude,
            slot: SlotId::Distance,
            expr: len(0.011),
        },
        DocEdit::SetParam {
            node: extrude,
            slot: SlotId::Distance,
            expr: len(0.013),
        },
    ];
    let mut history = History::replayed(doc, &edits, tol).expect("the log replays");
    assert!(!history.can_redo(), "the cursor opens at the tip");
    assert!(history.can_undo());
    assert!(history.undo().is_some());
    assert!(history.undo().is_some());
    assert!(!history.can_undo(), "two edits, two undos, then the root");
    assert!(history.can_redo(), "and the walk back is available again");
}
