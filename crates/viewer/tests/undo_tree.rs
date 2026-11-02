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

use pncad::document::{Doc, DocEdit, ProfileProgram};
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
/// variable, which every row below shares.
fn set_thickness(session: &mut DocSession, metres: f64) -> Vec<DocEdit<ProfileProgram>> {
    session
        .perform(SessionOp::SetVariable {
            var: common::thickness_var(session.committed_doc()),
            value: SlotValue::Continuous(metres),
        })
        .committed
}

fn thickness_of(doc: &Doc<ProfileProgram>) -> f64 {
    match props::variable_rows(doc)
        .into_iter()
        .find(|row| row.label.name() == Some(&common::thickness_param()))
        .expect("the fixture declares the variable")
        .value
    {
        SlotValue::Continuous(v) => v,
        SlotValue::Count(_) => panic!("the fixture's variable is continuous"),
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
        Some(viewer::Refusal::NothingToDo {
            direction: viewer::session::Step::Undo
        })
    ));
    set_thickness(&mut session, 0.010);
    assert!(matches!(
        session.perform(SessionOp::Redo).refusal,
        Some(viewer::Refusal::NothingToDo {
            direction: viewer::session::Step::Redo
        })
    ));
    assert!(!session.history().can_redo());
    assert!(session.history().can_undo());
}

#[test]
fn a_save_writes_the_document_the_cursor_is_on() {
    let tol = Tol::witness();
    let (mut session, _extrude) = session(tol);
    set_thickness(&mut session, 0.010);
    set_thickness(&mut session, 0.012);
    session.perform(SessionOp::Undo);
    set_thickness(&mut session, 0.014);
    assert_eq!(session.history().len(), 4);

    let dir = common::tempdir("gui3-save-path");
    let file = dir.join("doc.pncad");
    assert!(
        session
            .perform(SessionOp::Save(file.clone()))
            .refusal
            .is_none()
    );

    let reopened = docio::open(&file, tol).expect("the saved file opens");
    assert_eq!(thickness_of(reopened.doc()), 0.014);
    assert_eq!(reopened.len(), 1, "one state: undo ends at an open");
    std::fs::remove_dir_all(&dir).expect("the fixture directory is removable");
}

/// **Undo restores a reader a delete stranded** (D10; review A's
/// MINOR-3): deleting the plate's profile is accepted and leaves its
/// extrude reading a variable no operation defines, refused
/// `UnresolvedRead`; undoing the delete returns the document it started
/// from, bit for bit, and the extrude reads its profile and builds again.
#[test]
fn undo_restores_a_reader_a_delete_stranded() {
    let tol = Tol::witness();
    let (doc, profile, extrude) = common::parametric_plate(tol);
    let error_of = |doc: &Doc<ProfileProgram>| {
        let evaluation: pncad::document::Evaluation<f64> = pncad::document::evaluate(
            doc,
            None,
            &pncad::document::CancelToken::new(),
            &pncad::document::EvalOptions::default(),
            tol,
        );
        evaluation
            .node_error(extrude)
            .map(|e| format!("{:?}", e.kind))
    };
    assert_eq!(error_of(&doc), None, "the plate builds");
    let mut history = History::new(doc.clone());
    let edit = DocEdit::DeleteNode { id: profile };
    let deleted =
        pncad::document::apply(&doc, &edit, tol).expect("a delete with a reader is accepted");
    history.commit(edit, deleted.doc);
    let stranded = error_of(history.doc()).unwrap_or_default();
    assert!(
        stranded.starts_with("UnresolvedRead"),
        "the stranded extrude refuses typed: {stranded}"
    );
    assert!(
        history.doc().upstream(extrude).is_empty(),
        "and reads nothing live"
    );

    history.undo().expect("the delete undoes");
    assert!(
        history.doc().bit_eq(&doc),
        "undo restores the document bit for bit"
    );
    assert_eq!(
        history.doc().upstream(extrude),
        vec![profile],
        "the read is back"
    );
    assert_eq!(
        error_of(history.doc()),
        None,
        "and the extrude builds again"
    );
}
