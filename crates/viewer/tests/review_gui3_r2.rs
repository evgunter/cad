//! Reviewer R2's consumer suite for GUI-3 (PR #1101) — an independent
//! derivation of what the PR claims about the document panels, driven
//! through `viewer`'s public surface exactly as an outside consumer
//! would call it.
//!
//! Nothing here re-reads the unit's own fixtures: the documents are
//! authored from scratch through `apply` (a different shape from
//! `tests/common`'s plate), and every assertion is written from the
//! PR's prose rather than from the shipped rows.
//!
//! Randomized rows follow `memories/test-suite-cost.md`: a fresh seed
//! per run through `test_utils::fuzz` (logged unconditionally,
//! `CAD_FUZZ_SEED` replays), counts on `CAD_FUZZ_EFFORT`. Every row
//! asserts; there is no print-only probe in this file.
//!
//! # Two rows are `#[ignore]`d because they are RED against the head
//! this review froze on (956ef3cf)
//!
//! They are the review's two seam findings, written as the gates they
//! should become. Remove the `#[ignore]` when the fix lands — that is
//! the whole of the promotion work they need.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

test_utils::gated_to!["crates/viewer/src/", "crates/pncad/src/"];

use std::sync::Arc;

use pncad::document::{
    Dimension, Doc, DocEdit, DocParam, EvalOutcome, Expr, LoopProgram, Node, ParamName,
    ProfileProgram, RecipeNodeId, SlotId, apply,
};
use pncad::geom_core::Tol;
use pncad::profile::SketchPlane;
use test_utils::fuzz;
use viewer::evalseam::{EvalRequest, EvalService, Generation, InlineEvaluator, ThreadEvaluator};
use viewer::history::History;
use viewer::props::{SlotDriver, SlotValue};
use viewer::session::{DocSession, Landing, Refusal, Selection, SessionOp};
use viewer::{docio, props, tree};

// --- fixtures, authored here rather than borrowed -------------------

fn len(m: f64) -> Expr {
    Expr::literal(m, Dimension::Length).expect("a finite length")
}

fn scl(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).expect("a finite scalar")
}

fn rect(w: f64, h: f64) -> Node<ProfileProgram> {
    Node::Profile(ProfileProgram {
        plane: SketchPlane::xy(),
        loops: vec![
            LoopProgram::polygon([(0.0, 0.0), (w, 0.0), (w, h), (0.0, h)]).expect("finite corners"),
        ],
    })
}

fn push(
    doc: &Doc<ProfileProgram>,
    node: Node<ProfileProgram>,
    tol: Tol,
) -> (Doc<ProfileProgram>, RecipeNodeId) {
    let applied = apply(doc, &DocEdit::InsertNode { node }, tol).expect("the insert applies");
    let id = applied.record.minted.expect("an insert mints an id");
    (applied.doc, id)
}

fn width_param() -> ParamName {
    ParamName::new("width")
}

/// A slab whose extrude distance is a LITERAL and whose transform's
/// x-translation is DRIVEN by `width * 2`. Two slots, one of each
/// driver class, in one document — deliberately a different shape from
/// the unit's own `parametric_plate`.
fn slab(tol: Tol) -> (Doc<ProfileProgram>, RecipeNodeId, RecipeNodeId) {
    let doc: Doc<ProfileProgram> = Doc::empty_derived("r2-gui3-slab", tol);
    let doc = apply(
        &doc,
        &DocEdit::SetDocParam {
            name: width_param(),
            value: DocParam::continuous(Dimension::Length, 0.005),
        },
        tol,
    )
    .expect("the parameter declares")
    .doc;
    let (doc, profile) = push(&doc, rect(0.03, 0.02), tol);
    let (doc, extrude) = push(
        &doc,
        Node::Extrude {
            profile,
            distance: len(0.006),
        },
        tol,
    );
    let (doc, moved) = push(
        &doc,
        Node::Transform {
            input: extrude,
            translation: [
                Expr::mul(Expr::param(width_param(), Dimension::Length), scl(2.0))
                    .expect("length * scalar is a length"),
                len(0.0),
                len(0.0),
            ],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: Expr::literal(0.0, Dimension::Angle).expect("finite"),
        },
        tol,
    );
    (doc, extrude, moved)
}

fn distance_of(doc: &Doc<ProfileProgram>, node: RecipeNodeId) -> SlotValue {
    props::slot_rows(doc, node)
        .into_iter()
        .find(|row| row.slot == SlotId::Distance)
        .expect("the extrude carries a distance")
        .value
        .expect("the distance evaluates")
}

fn tempdir(label: &str) -> std::path::PathBuf {
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_nanos());
    let dir = std::env::temp_dir().join(format!("{label}-{unique}"));
    std::fs::create_dir_all(&dir).expect("the fixture directory is creatable");
    dir
}

// --- the undo TREE --------------------------------------------------

/// **Nothing is ever destroyed, two levels deep.**
///
/// The unit's own row branches once and checks the abandoned child.
/// This one abandons a whole SUBTREE — a grandchild reached through the
/// branch that redo walked away from — and asserts the grandchild's
/// document and its edit are both still readable by id. A truncating
/// stack, and equally an implementation that pruned only the immediate
/// child, goes red here and green on a one-level check.
#[test]
fn an_abandoned_subtree_keeps_its_grandchildren_their_documents_and_their_edits() {
    let tol = Tol::witness();
    let (doc, extrude, _moved) = slab(tol);
    let mut session = DocSession::inline(doc, tol);

    let set = |session: &mut DocSession, v: f64| {
        let outcome = session.perform(SessionOp::SetSlot {
            node: extrude,
            slot: SlotId::Distance,
            value: SlotValue::Continuous(v),
        });
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
        assert_eq!(outcome.committed.len(), 1);
    };

    set(&mut session, 0.007);
    set(&mut session, 0.008);
    set(&mut session, 0.009);
    assert_eq!(session.history().len(), 4);
    let deep_leaf = session.history().current();

    // Two undos put the cursor two above the leaf; the branch below it
    // is what the next edit abandons.
    session.perform(SessionOp::Undo);
    session.perform(SessionOp::Undo);
    let branch_point = session.history().current();
    set(&mut session, 0.020);

    let history = session.history();
    assert_eq!(history.len(), 5, "a sibling was minted, nothing removed");
    assert_eq!(
        history.entry(branch_point).children().len(),
        2,
        "the branch point carries both branches"
    );
    // The grandchild of the abandoned branch is intact, by id.
    assert_eq!(
        distance_of(history.entry(deep_leaf).doc(), extrude),
        SlotValue::Continuous(0.009),
        "the abandoned subtree's deepest document survives"
    );
    assert!(
        !history.entry(deep_leaf).edits().is_empty(),
        "and so does the edit that produced it"
    );
    let mid = history
        .entry(deep_leaf)
        .parent()
        .expect("the leaf has a parent");
    assert_eq!(
        distance_of(history.entry(mid).doc(), extrude),
        SlotValue::Continuous(0.008)
    );
    assert_eq!(
        history.entry(mid).active_child(),
        Some(deep_leaf),
        "the abandoned branch remembers its own redo target"
    );
}

/// **Redo follows the branch the cursor is on, at arbitrary depth.**
///
/// A counterexample search over undo/redo walks: after any sequence of
/// undos and edits, walking undo to the root and redo back must land on
/// the document the last commit produced. The seed varies per run
/// (`memories/test-suite-cost.md`); the walk length rides the effort
/// dial.
#[test]
fn redo_from_the_root_returns_to_the_last_committed_state_under_random_walks() {
    let tol = Tol::witness();
    let mut rng = fuzz::start("r2_gui3_undo_walk");
    for _ in 0..fuzz::scaled(6) {
        let (doc, extrude, _moved) = slab(tol);
        let mut session = DocSession::inline(doc, tol);
        let mut last = 0.006_f64;
        for step in 0..fuzz::scaled(8).max(4) {
            if rng.below(3) == 0 {
                // An undo may refuse at the root; that is not a finding.
                session.perform(SessionOp::Undo);
            } else {
                let v = 0.001 + rng.range(0.001, 0.05);
                let outcome = session.perform(SessionOp::SetSlot {
                    node: extrude,
                    slot: SlotId::Distance,
                    value: SlotValue::Continuous(v),
                });
                assert!(
                    outcome.refusal.is_none(),
                    "step {step} refused: {:?} ({})",
                    outcome.refusal,
                    fuzz::replay()
                );
                last = v;
            }
        }
        let states = session.history().len();
        // Walk to the root, then redo all the way back.
        while session.perform(SessionOp::Undo).refusal.is_none() {}
        assert_eq!(
            session.history().current(),
            session.history().root(),
            "undo reaches the root ({})",
            fuzz::replay()
        );
        while session.perform(SessionOp::Redo).refusal.is_none() {}
        assert_eq!(
            distance_of(session.committed_doc(), extrude),
            SlotValue::Continuous(last),
            "redo returns to the last committed state ({})",
            fuzz::replay()
        );
        assert_eq!(
            session.history().len(),
            states,
            "the walk minted nothing ({})",
            fuzz::replay()
        );
    }
}

// --- open / save ----------------------------------------------------

/// **open → save with no edits is BYTE-stable**, against the file's own
/// bytes rather than against a second save.
///
/// The unit's rows check that two saves of one session agree and that
/// the reopened DOCUMENT is `bit_eq`. Neither would catch a save that
/// wrote a different but equivalent serialization of the same document,
/// which is the property the acceptance walk ("open a demo document,
/// save it back") actually rests on.
#[test]
fn opening_a_file_and_saving_it_straight_back_reproduces_its_bytes() {
    let tol = Tol::witness();
    let (doc, extrude, _moved) = slab(tol);
    let dir = tempdir("r2-gui3-bytes");

    // Author a file the ordinary way: a session with two edits, saved.
    let original = dir.join("original.pncad");
    let mut session = DocSession::inline(doc, tol);
    session.perform(SessionOp::SetSlot {
        node: extrude,
        slot: SlotId::Distance,
        value: SlotValue::Continuous(0.011),
    });
    session.perform(SessionOp::SetSlot {
        node: extrude,
        slot: SlotId::Distance,
        value: SlotValue::Continuous(0.013),
    });
    assert!(
        session
            .perform(SessionOp::Save(original.clone()))
            .refusal
            .is_none()
    );
    let first = std::fs::read_to_string(&original).expect("the file is readable");

    // Open it in a fresh session and save straight back.
    let mut reopened = DocSession::inline(Doc::empty_derived("r2-gui3-blank", tol), tol);
    assert!(
        reopened
            .perform(SessionOp::Open(original.clone()))
            .refusal
            .is_none()
    );
    let again = dir.join("again.pncad");
    assert!(
        reopened
            .perform(SessionOp::Save(again.clone()))
            .refusal
            .is_none()
    );
    let second = std::fs::read_to_string(&again).expect("the file is readable");
    assert_eq!(first, second, "open → save with no edits changed the bytes");

    // And the file's log IS the current path: two undos reach the root.
    assert_eq!(reopened.history().path_edits().len(), 2);
    assert!(reopened.perform(SessionOp::Undo).refusal.is_none());
    assert_eq!(
        distance_of(reopened.committed_doc(), extrude),
        SlotValue::Continuous(0.011),
        "an opened document undoes step by step"
    );
    assert!(reopened.perform(SessionOp::Undo).refusal.is_none());
    assert!(
        matches!(
            reopened.perform(SessionOp::Undo).refusal,
            Some(Refusal::NothingToDo)
        ),
        "the file's log is exactly two steps long"
    );

    std::fs::remove_dir_all(&dir).expect("the fixture directory is removable");
}

/// **A save after an undo writes the SHORTER log**, and reopening it
/// produces a history that undoes exactly that far.
///
/// The branch the cursor is not on is deliberately not persisted (the
/// v1 contract); this row is the consumer-side statement of that, taken
/// through the file rather than through `path_edits()`.
#[test]
fn a_save_taken_after_an_undo_persists_only_the_path_the_cursor_is_on() {
    let tol = Tol::witness();
    let (doc, extrude, _moved) = slab(tol);
    let dir = tempdir("r2-gui3-shortlog");
    let mut session = DocSession::inline(doc, tol);
    for v in [0.007, 0.008, 0.009] {
        session.perform(SessionOp::SetSlot {
            node: extrude,
            slot: SlotId::Distance,
            value: SlotValue::Continuous(v),
        });
    }
    session.perform(SessionOp::Undo);
    session.perform(SessionOp::Undo);

    let file = dir.join("short.pncad");
    assert!(
        session
            .perform(SessionOp::Save(file.clone()))
            .refusal
            .is_none()
    );
    assert_eq!(
        session.history().len(),
        4,
        "the states the undo walked past are all still held"
    );

    let reopened = docio::open(&file, tol).expect("the short log opens");
    assert_eq!(reopened.path_edits().len(), 1, "one edit was on the path");
    assert_eq!(
        distance_of(reopened.doc(), extrude),
        SlotValue::Continuous(0.007)
    );
    std::fs::remove_dir_all(&dir).expect("the fixture directory is removable");
}

/// A history seeded by `History::replayed` walks back through the log
/// one entry at a time, and its root is the file's snapshot.
#[test]
fn a_replayed_history_undoes_one_logged_edit_at_a_time() {
    let tol = Tol::witness();
    let (doc, extrude, _moved) = slab(tol);
    let edits: Vec<DocEdit<ProfileProgram>> = [0.007_f64, 0.008, 0.009]
        .into_iter()
        .map(|v| DocEdit::SetParam {
            node: extrude,
            slot: SlotId::Distance,
            expr: len(v),
        })
        .collect();
    let mut history = History::replayed(doc, &edits, tol).expect("the log replays");
    assert_eq!(history.len(), 4);
    for expected in [0.008_f64, 0.007, 0.006] {
        history.undo().expect("a step back");
        assert_eq!(
            distance_of(history.doc(), extrude),
            SlotValue::Continuous(expected)
        );
    }
    assert_eq!(history.current(), history.root());
    assert!(history.undo().is_none(), "the root has no parent");
}

// --- edit emission (G1 preview vs commit) ---------------------------

/// **One gesture, one undo step, whatever the drag did.**
///
/// A counterexample search over drag shapes: any number of previews,
/// any values, still exactly one committed edit and exactly one new
/// history state, and one undo returns the pre-drag document. The
/// preview count rides the effort dial and the values vary per run.
#[test]
fn a_drag_of_any_length_commits_exactly_one_edit_and_one_undo_step() {
    let tol = Tol::witness();
    let mut rng = fuzz::start("r2_gui3_gesture");
    for _ in 0..fuzz::scaled(6) {
        let (doc, extrude, _moved) = slab(tol);
        let mut session = DocSession::inline(doc, tol);
        let before_states = session.history().len();
        let before_value = distance_of(session.committed_doc(), extrude);

        assert!(
            session
                .perform(SessionOp::BeginGesture {
                    node: extrude,
                    slot: SlotId::Distance,
                })
                .refusal
                .is_none()
        );
        let steps = 1 + rng.below(fuzz::scaled(10).max(2));
        let mut last = 0.0;
        for _ in 0..steps {
            last = rng.range(0.001, 0.05);
            let outcome = session.perform(SessionOp::PreviewGesture { value: last });
            assert!(
                outcome.committed.is_empty(),
                "a preview committed ({})",
                fuzz::replay()
            );
            assert_eq!(outcome.previewed.len(), 1);
            assert_eq!(
                session.history().len(),
                before_states,
                "the history moved mid-gesture ({})",
                fuzz::replay()
            );
        }
        let outcome = session.perform(SessionOp::CommitGesture);
        assert_eq!(
            outcome.committed.len(),
            1,
            "{steps} previews produced {} commits ({})",
            outcome.committed.len(),
            fuzz::replay()
        );
        assert_eq!(session.history().len(), before_states + 1);
        assert_eq!(
            distance_of(session.committed_doc(), extrude),
            SlotValue::Continuous(last),
            "the commit carries the LAST previewed value ({})",
            fuzz::replay()
        );
        session.perform(SessionOp::Undo);
        assert_eq!(
            distance_of(session.committed_doc(), extrude),
            before_value,
            "one undo returns the whole drag ({})",
            fuzz::replay()
        );
    }
}

/// Scratch state is not the document: mid-gesture the history's
/// document is untouched, and a save taken mid-gesture writes the
/// COMMITTED state rather than the preview.
#[test]
fn a_save_taken_mid_gesture_writes_the_committed_document_not_the_preview() {
    let tol = Tol::witness();
    let (doc, extrude, _moved) = slab(tol);
    let dir = tempdir("r2-gui3-scratch");
    let mut session = DocSession::inline(doc, tol);
    session.perform(SessionOp::BeginGesture {
        node: extrude,
        slot: SlotId::Distance,
    });
    session.perform(SessionOp::PreviewGesture { value: 0.042 });
    assert_eq!(
        distance_of(session.doc(), extrude),
        SlotValue::Continuous(0.042),
        "the panels show the preview"
    );

    let file = dir.join("mid.pncad");
    session.perform(SessionOp::Save(file.clone()));
    let reopened = docio::open(&file, tol).expect("the mid-gesture save opens");
    assert_eq!(
        distance_of(reopened.doc(), extrude),
        SlotValue::Continuous(0.006),
        "the preview never reached the file"
    );
    assert_eq!(reopened.path_edits().len(), 0);
    std::fs::remove_dir_all(&dir).expect("the fixture directory is removable");
}

// --- the expression affordance --------------------------------------

/// **The other direction of the text door**: a slot that is a bare
/// literal accepts a number today, and once an expression is written
/// into it through `parse_expr` it starts REFUSING numbers with the
/// affordance. The unit's rows walk driven → expression; this walks
/// literal → driven → refusal → navigate → parameter edit → the slot
/// follows.
#[test]
fn a_literal_slot_becomes_driven_through_the_text_door_and_then_refuses_numbers() {
    let tol = Tol::witness();
    let (doc, extrude, _moved) = slab(tol);
    let mut session = DocSession::inline(doc, tol);
    session.perform(SessionOp::Select(Selection::Node(extrude)));

    let driver = |session: &DocSession| {
        session
            .slot_rows()
            .into_iter()
            .find(|row| row.slot == SlotId::Distance)
            .expect("the extrude carries a distance")
            .driver
    };
    assert_eq!(driver(&session), SlotDriver::Literal);
    assert!(
        session
            .perform(SessionOp::SetSlot {
                node: extrude,
                slot: SlotId::Distance,
                value: SlotValue::Continuous(0.007),
            })
            .refusal
            .is_none(),
        "a literal slot takes a number"
    );

    // Write an expression over the document parameter into it.
    let outcome = session.perform(SessionOp::SetSlotExpression {
        node: extrude,
        slot: SlotId::Distance,
        text: "width * 3.0".to_owned(),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(outcome.committed.len(), 1);
    assert_eq!(
        driver(&session),
        SlotDriver::Expression {
            params: vec![width_param()]
        },
        "the slot is now driven, and says by what"
    );
    assert_eq!(
        distance_of(session.committed_doc(), extrude),
        SlotValue::Continuous(0.015)
    );

    // Now a number is refused, with the affordance's whole payload.
    let states = session.history().len();
    match session
        .perform(SessionOp::SetSlot {
            node: extrude,
            slot: SlotId::Distance,
            value: SlotValue::Continuous(0.001),
        })
        .refusal
    {
        Some(Refusal::DrivenByExpression {
            node,
            slot,
            params,
            current,
        }) => {
            assert_eq!((node, slot), (extrude, SlotId::Distance));
            assert_eq!(params, vec![width_param()]);
            assert_eq!(current, Some(SlotValue::Continuous(0.015)));
        }
        other => panic!("expected the driven refusal, got {other:?}"),
    }
    assert_eq!(session.history().len(), states, "a refusal mints nothing");

    // And the navigation half closes the loop.
    session.perform(SessionOp::Select(Selection::Param(width_param())));
    assert!(
        session
            .perform(SessionOp::SetParam {
                name: width_param(),
                value: SlotValue::Continuous(0.010),
            })
            .refusal
            .is_none()
    );
    assert_eq!(
        distance_of(session.committed_doc(), extrude),
        SlotValue::Continuous(0.030),
        "the driven slot followed its parameter"
    );
}

/// A slot driven by arithmetic over NO parameter is still refused —
/// the branch-free case the unit's `parametric_plate` fixture (whose
/// driven slot always references `thickness`) cannot reach. The
/// affordance then names no navigation target, which is the honest
/// answer and the one the chrome has to render.
#[test]
fn a_parameterless_expression_is_driven_and_offers_no_navigation_target() {
    let tol = Tol::witness();
    let (doc, extrude, _moved) = slab(tol);
    let mut session = DocSession::inline(doc, tol);
    let outcome = session.perform(SessionOp::SetSlotExpression {
        node: extrude,
        slot: SlotId::Distance,
        // No parameter anywhere: pure arithmetic over literals.
        text: "0.004 m + 0.003 m".to_owned(),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.perform(SessionOp::Select(Selection::Node(extrude)));
    let row = session
        .slot_rows()
        .into_iter()
        .find(|row| row.slot == SlotId::Distance)
        .expect("the extrude carries a distance");
    assert_eq!(
        row.driver,
        SlotDriver::Expression { params: vec![] },
        "arithmetic over literals is driven, with nothing to navigate to"
    );
    assert!(matches!(
        session
            .perform(SessionOp::SetSlot {
                node: extrude,
                slot: SlotId::Distance,
                value: SlotValue::Continuous(0.001),
            })
            .refusal,
        Some(Refusal::DrivenByExpression { params, .. }) if params.is_empty()
    ));
}

// --- tree badges ----------------------------------------------------

/// **Badges are the typed payload's own rendering, byte for byte**, on
/// a document whose failure is reached a different way from the unit's
/// own fixture: a transform whose ROTATION AXIS is degenerate, rather
/// than a division by zero in a distance.
#[test]
fn failed_and_poisoned_badges_carry_the_payloads_own_text_and_nothing_else() {
    let tol = Tol::witness();
    let doc: Doc<ProfileProgram> = Doc::empty_derived("r2-gui3-broken", tol);
    let (doc, profile) = push(&doc, rect(0.03, 0.02), tol);
    let (doc, bad) = push(
        &doc,
        Node::Extrude {
            profile,
            // A zero extrude distance: well-dimensioned at the edit
            // door, refused by the operation at evaluation.
            distance: len(0.0),
        },
        tol,
    );
    let (doc, downstream) = push(
        &doc,
        Node::Transform {
            input: bad,
            translation: [len(0.01), len(0.0), len(0.0)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: Expr::literal(0.0, Dimension::Angle).expect("finite"),
        },
        tol,
    );

    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let evaluation = session.evaluation().expect("a result landed");
    let rows = session.tree_rows();
    assert!(tree::has_faults(&rows), "the document does not build");

    let failed = rows
        .iter()
        .find(|row| row.id == bad)
        .expect("the failing node has a row");
    let expected = match evaluation.result(bad).expect("the node has a result") {
        pncad::document::NodeResult::Failed(error) => error.to_string(),
        other => panic!("expected a failure, got {other:?}"),
    };
    assert_eq!(
        failed.status,
        viewer::tree::RowStatus::Failed {
            message: expected.clone()
        },
        "the badge is NodeError's own Display, not a sentence the panel wrote"
    );

    let poisoned = rows
        .iter()
        .find(|row| row.id == downstream)
        .expect("the downstream node has a row");
    match &poisoned.status {
        viewer::tree::RowStatus::Poisoned { through, message } => {
            assert_eq!(*through, bad, "the poison names the failed ancestor");
            assert_eq!(
                message.as_deref(),
                Some(expected.as_str()),
                "a poisoned row reports the ANCESTOR's typed error verbatim"
            );
        }
        other => panic!("expected a poisoning, got {other:?}"),
    }
    // No status carries a string this crate composed: every message in
    // the tree is one of the evaluation's own renderings.
    for row in &rows {
        if let Some(message) = row.status.message() {
            assert_eq!(
                message, expected,
                "an unexpected message appeared in the tree: {message}"
            );
        }
    }
}

/// Absence is not success: a node with no entry reads `Unevaluated`,
/// and `has_faults` does not count it.
#[test]
fn a_document_with_no_result_yet_reads_unevaluated_and_reports_no_faults() {
    let tol = Tol::witness();
    let (doc, _extrude, _moved) = slab(tol);
    let session = DocSession::inline(doc, tol);
    let rows = session.tree_rows();
    assert!(!rows.is_empty(), "the tree draws before the first result");
    assert!(
        rows.iter()
            .all(|row| row.status == viewer::tree::RowStatus::Unevaluated)
    );
    assert!(
        !tree::has_faults(&rows),
        "an absent measurement is not a fault"
    );
}

// --- the evaluation seam --------------------------------------------

/// `pump()` drains a BACKLOG of results and lands only the one that
/// answers the current document.
#[test]
fn a_backlog_of_results_drains_in_one_pump_and_only_the_current_one_lands() {
    let tol = Tol::witness();
    let (doc, extrude, _moved) = slab(tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let stale = Arc::clone(session.evaluation_arc().expect("the first result landed"));

    session.perform(SessionOp::SetSlot {
        node: extrude,
        slot: SlotId::Distance,
        value: SlotValue::Continuous(0.011),
    });
    let current = session.generation();
    // Three answers arrive at once; two of them are for documents the
    // session has moved past.
    assert_eq!(
        session.land(viewer::evalseam::EvalDone {
            generation: Generation::FIRST,
            evaluation: Arc::clone(&stale),
        }),
        Landing::Stale
    );
    assert_eq!(
        session.land(viewer::evalseam::EvalDone {
            generation: current.next().next(),
            evaluation: Arc::clone(&stale),
        }),
        Landing::Stale,
        "a result from the FUTURE is not the current document either"
    );
    assert!(session.busy());
    assert_eq!(session.pump(), vec![Landing::Landed]);
    assert!(!session.busy());
}

/// **Was RED at 956ef3cf; green since the fix pass.** A canceled run's completed
/// prefix must not become the session's result.
///
/// `DocSession::land` compares generations only, so a run canceled for
/// the CURRENT generation — which is exactly what the toolbar's Cancel
/// button produces — lands its empty prefix as the answer: the feature
/// tree goes all-`Unevaluated`, the scene rebuilds from nothing, and
/// `busy()` goes dark claiming the picture answers the document. The
/// seam's own module docs say the crate "never mistakes it for a
/// completed one"; this row is that sentence.
#[test]
fn a_canceled_run_is_not_mistaken_for_the_current_documents_answer() {
    let tol = Tol::witness();
    let (doc, extrude, _moved) = slab(tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let good = Arc::clone(session.evaluation_arc().expect("the first result landed"));
    assert!(!tree::has_faults(&session.tree_rows()));

    // Edit, then press Cancel before the result arrives.
    session.perform(SessionOp::SetSlot {
        node: extrude,
        slot: SlotId::Distance,
        value: SlotValue::Continuous(0.011),
    });
    session.perform(SessionOp::CancelEvaluation);
    session.pump();

    assert!(
        session.busy(),
        "a canceled run leaves the session still waiting for an answer"
    );
    let shown = session
        .evaluation_arc()
        .expect("the last good result is still on screen");
    assert_eq!(
        shown.outcome,
        EvalOutcome::Completed,
        "the result on screen is a COMPLETED run, not a canceled prefix"
    );
    assert!(
        Arc::ptr_eq(shown, &good),
        "the previous good evaluation was replaced by a canceled prefix"
    );
}

/// **Was RED at 956ef3cf; green since the fix pass.** The two seam implementations
/// must agree on what two rapid submits produce.
///
/// `evalseam`'s module docs say multiple submits COALESCE — "only the
/// newest document is queued". `InlineEvaluator` holds one
/// `Option<EvalRequest>` and does exactly that; `ThreadEvaluator` sends
/// every job into an unbounded `mpsc`, so N submits produce N results
/// and N `evaluate` calls. The behaviour the docs describe is the
/// inline one; the application runs the other.
#[test]
fn both_seam_implementations_coalesce_two_submits_into_one_result() {
    let tol = Tol::witness();
    let (doc, _extrude, _moved) = slab(tol);

    let count_results = |seam: &mut dyn EvalService| {
        seam.submit(EvalRequest {
            generation: Generation::FIRST,
            doc: doc.clone(),
            tol,
            resolver: None,
        });
        seam.submit(EvalRequest {
            generation: Generation::FIRST.next(),
            doc: doc.clone(),
            tol,
            resolver: None,
        });
        let mut results = Vec::new();
        for _ in 0..10_000 {
            while let Some(done) = seam.poll() {
                results.push(done.generation);
            }
            if !seam.busy() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        results
    };

    let inline = count_results(&mut InlineEvaluator::new());
    let threaded = count_results(&mut ThreadEvaluator::spawn().expect("the worker starts"));
    assert_eq!(
        inline,
        vec![Generation::FIRST.next()],
        "the inline seam coalesces to the newest generation"
    );
    assert_eq!(
        threaded, inline,
        "the threaded seam must answer the same way the inline one does"
    );
}

/// The memo lives inside the seam: nothing above the boundary hands a
/// prior evaluation back, and the second run of an edited document
/// still reuses. Driven through the SESSION rather than the seam
/// directly, which is the consumer's view of the same claim.
#[test]
fn the_session_never_hands_a_prior_evaluation_back_and_still_gets_reuse() {
    let tol = Tol::witness();
    let (doc, extrude, _moved) = slab(tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    assert_eq!(
        session.evaluation().expect("a first result").reused,
        0,
        "a cold run reuses nothing"
    );
    session.perform(SessionOp::SetSlot {
        node: extrude,
        slot: SlotId::Distance,
        value: SlotValue::Continuous(0.0125),
    });
    session.pump();
    let second = session.evaluation().expect("a second result");
    assert!(
        second.reused > 0,
        "the unchanged prefix was reused across an edit the session never memoized"
    );
    assert!(second.recomputed < second.order.len());
}

// --- the panels are pure functions of (Doc, Evaluation) -------------

/// `tree_rows()` and `slot_rows()` are pure: called twice with nothing
/// in between they answer identically, and they answer the same thing
/// as the free functions given the same inputs. A hidden per-frame
/// cache or a shadow of a document field would show up as a difference
/// on the second call.
#[test]
fn the_panel_models_are_pure_functions_of_the_document_and_the_evaluation() {
    let tol = Tol::witness();
    let (doc, extrude, _moved) = slab(tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    session.perform(SessionOp::Select(Selection::Node(extrude)));

    for _ in 0..3 {
        assert_eq!(session.tree_rows(), session.tree_rows());
        assert_eq!(session.slot_rows(), session.slot_rows());
        assert_eq!(
            session.tree_rows(),
            tree::rows(session.doc(), session.evaluation()),
            "the session's tree is the free function's"
        );
        assert_eq!(
            session.slot_rows(),
            props::slot_rows(session.doc(), extrude),
            "the session's property rows are the free function's"
        );
    }
}
