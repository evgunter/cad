//! **Linear chrome over tree-shaped state.**
//!
//! The plan's undo note is the specification these rows test: an edit
//! after an undo mints a SIBLING rather than truncating, nothing is
//! destroyed, v1 chrome walks only the current branch, and a save
//! writes that branch's linear log.
//!
//! The rows deliberately assert on the TREE (entry counts, children,
//! parents) and not only on what undo/redo show. A stack-shaped
//! implementation that truncates would satisfy every "undo shows the
//! old value" assertion and fail these.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use pncad::document::{Doc, DocEdit, ProfileProgram, SlotId};
use pncad::geom_core::Tol;
use viewer::history::History;
use viewer::props::SlotValue;
use viewer::session::{DocSession, SessionOp};
use viewer::{docio, props};

/// A session over the parametric plate, plus the extrude node whose
/// profile-side square is edited by these rows.
fn session(tol: Tol) -> (DocSession, pncad::document::RecipeNodeId) {
    let (doc, _profile, extrude) = common::parametric_plate(tol);
    (DocSession::inline(doc, tol), extrude)
}

/// The distance edit these rows use: the extrude carries a DRIVEN
/// distance, so the literal slot they move is the profile's — but the
/// simplest editable literal in this fixture is a new document
/// parameter, which every row below shares.
fn set_thickness(session: &mut DocSession, metres: f64) -> Vec<DocEdit<ProfileProgram>> {
    session
        .perform(SessionOp::SetParam {
            name: common::thickness_param(),
            value: SlotValue::Continuous(metres),
        })
        .committed
}

fn thickness_of(doc: &Doc<ProfileProgram>) -> f64 {
    match props::param_rows(doc)
        .into_iter()
        .find(|row| row.name == common::thickness_param())
        .expect("the fixture declares the parameter")
        .value
    {
        SlotValue::Continuous(v) => v,
        SlotValue::Count(_) => panic!("the fixture's parameter is continuous"),
    }
}

#[test]
fn an_edit_after_an_undo_mints_a_sibling_and_destroys_nothing() {
    let tol = Tol::witness();
    let (mut session, _extrude) = session(tol);

    assert_eq!(set_thickness(&mut session, 0.010).len(), 1);
    assert_eq!(set_thickness(&mut session, 0.012).len(), 1);
    let after_two = session.history().len();
    assert_eq!(after_two, 3, "root plus two edits");

    // Step back onto the first edit, then edit again: the 0.012 state
    // must survive as a sibling of the new one.
    session.perform(SessionOp::Undo);
    let abandoned = session
        .history()
        .entry(session.history().current())
        .active_child()
        .expect("undo remembers the branch it left");
    assert_eq!(set_thickness(&mut session, 0.014).len(), 1);

    let history = session.history();
    assert_eq!(
        history.len(),
        4,
        "the sibling is minted, nothing is dropped"
    );
    let parent = history
        .entry(history.current())
        .parent()
        .expect("the sibling has a parent");
    assert_eq!(
        history.entry(parent).children().len(),
        2,
        "the branch point has both children"
    );
    assert!(
        history.entry(parent).children().contains(&abandoned),
        "the branch redo would have reached is still a child"
    );
    assert_eq!(
        thickness_of(history.entry(abandoned).doc()),
        0.012,
        "the abandoned branch's document is intact"
    );
    assert_eq!(thickness_of(session.committed_doc()), 0.014);
}

#[test]
fn redo_follows_the_current_branch_not_the_abandoned_one() {
    let tol = Tol::witness();
    let (mut session, _extrude) = session(tol);
    set_thickness(&mut session, 0.010);
    set_thickness(&mut session, 0.012);
    session.perform(SessionOp::Undo);
    set_thickness(&mut session, 0.014);

    // Undo off the new branch, then redo: the chrome walks the branch
    // the cursor is on, which is the one the last edit minted.
    session.perform(SessionOp::Undo);
    assert_eq!(thickness_of(session.committed_doc()), 0.010);
    session.perform(SessionOp::Redo);
    assert_eq!(
        thickness_of(session.committed_doc()),
        0.014,
        "redo returns to the branch the cursor left, not to 0.012"
    );
}

#[test]
fn undo_at_the_root_and_redo_at_a_leaf_refuse_rather_than_wrap() {
    let tol = Tol::witness();
    let (mut session, _extrude) = session(tol);
    assert!(matches!(
        session.perform(SessionOp::Undo).refusal,
        Some(viewer::Refusal::NothingToDo)
    ));
    set_thickness(&mut session, 0.010);
    assert!(matches!(
        session.perform(SessionOp::Redo).refusal,
        Some(viewer::Refusal::NothingToDo)
    ));
    assert!(!session.history().can_redo());
    assert!(session.history().can_undo());
}

#[test]
fn a_save_writes_the_current_paths_linear_log() {
    let tol = Tol::witness();
    let (mut session, _extrude) = session(tol);
    set_thickness(&mut session, 0.010);
    set_thickness(&mut session, 0.012);
    session.perform(SessionOp::Undo);
    set_thickness(&mut session, 0.014);

    // Four states, three edits in the tree — but the path from the
    // root to the cursor is two edits long, and that is what a save
    // records.
    assert_eq!(session.history().len(), 4);
    let path = session.history().path_edits();
    assert_eq!(path.len(), 2, "the path, not the tree");

    let dir = common::tempdir("gui3-save-path");
    let file = dir.join("doc.pncad");
    assert!(
        session
            .perform(SessionOp::Save(file.clone()))
            .refusal
            .is_none()
    );

    let reopened = docio::open(&file, tol).expect("the saved file opens");
    assert_eq!(
        reopened.path_edits().len(),
        2,
        "the reopened history replays exactly the saved log"
    );
    assert_eq!(thickness_of(reopened.doc()), 0.014);
    std::fs::remove_dir_all(&dir).expect("the fixture directory is removable");
}

#[test]
fn a_replayed_history_is_the_files_log_step_for_step() {
    let tol = Tol::witness();
    let (doc, _profile, extrude) = common::parametric_plate(tol);
    let edits = vec![
        DocEdit::SetParam {
            node: extrude,
            slot: SlotId::Distance,
            expr: common::len(0.02),
        },
        DocEdit::SetParam {
            node: extrude,
            slot: SlotId::Distance,
            expr: common::len(0.03),
        },
    ];
    let history = History::replayed(doc, &edits, tol).expect("the log replays");
    assert_eq!(history.len(), 3);
    assert_eq!(history.path_edits().len(), 2);
    assert_eq!(
        props::slot_rows(history.doc(), extrude)
            .into_iter()
            .find(|row| row.slot == SlotId::Distance)
            .expect("the extrude has a distance")
            .value
            .expect("the distance evaluates"),
        SlotValue::Continuous(0.03)
    );
}
