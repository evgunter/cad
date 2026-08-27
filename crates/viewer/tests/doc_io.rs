//! **open(path) / save(path) round-trip, with no dialog anywhere.**
//!
//! The file dialog is the one thing in this unit that escapes headless
//! testing. These rows are the reason that costs nothing: the
//! operations the dialog calls are ordinary typed functions over a
//! `Path`, and everything a user gets from Open… and Save… is
//! exercised here without a window.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

mod common;

use pncad::document::SlotId;
use pncad::geom_core::Tol;
use viewer::docio::{self, DocIoError};
use viewer::props::SlotValue;
use viewer::session::{DocSession, Refusal, SessionOp};

/// A fresh directory under the OS temp root, named for the row.
fn tempdir(label: &str) -> std::path::PathBuf {
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_nanos());
    let dir = std::env::temp_dir().join(format!("{label}-{unique}"));
    std::fs::create_dir_all(&dir).expect("the fixture directory is creatable");
    dir
}

fn distance(
    doc: &pncad::document::Doc<pncad::document::ProfileProgram>,
    node: pncad::document::RecipeNodeId,
) -> SlotValue {
    viewer::props::slot_rows(doc, node)
        .into_iter()
        .find(|row| row.slot == SlotId::Distance)
        .expect("the extrude carries a distance")
        .value
        .expect("the distance evaluates")
}

#[test]
fn a_document_round_trips_through_save_and_open() {
    let tol = Tol::witness();
    let (doc, _profile, extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    session.perform(SessionOp::SetParam {
        name: common::thickness_param(),
        value: SlotValue::Continuous(0.020),
    });
    assert_eq!(
        distance(session.committed_doc(), extrude),
        SlotValue::Continuous(0.010)
    );

    let dir = tempdir("gui3-round-trip");
    let file = dir.join("plate.pncad");
    assert!(
        session
            .perform(SessionOp::Save(file.clone()))
            .refusal
            .is_none()
    );
    assert_eq!(session.path(), Some(file.as_path()));

    // Opening into a fresh session reproduces the document AND its
    // history: the file's log is the reopened session's current path.
    let mut reopened = DocSession::inline(common::parametric_plate(tol).0, tol);
    assert!(
        reopened
            .perform(SessionOp::Open(file.clone()))
            .refusal
            .is_none()
    );
    assert_eq!(
        distance(reopened.committed_doc(), extrude),
        SlotValue::Continuous(0.010)
    );
    assert_eq!(reopened.history().path_edits().len(), 1);
    assert_eq!(reopened.path(), Some(file.as_path()));

    // And the reopened session can still undo the edit the file
    // carried, because the log became history rather than being
    // flattened into the snapshot.
    reopened.perform(SessionOp::Undo);
    assert_eq!(
        distance(reopened.committed_doc(), extrude),
        SlotValue::Continuous(0.004)
    );

    std::fs::remove_dir_all(&dir).expect("the fixture directory is removable");
}

#[test]
fn opening_a_missing_file_refuses_typed_and_leaves_the_session_alone() {
    let tol = Tol::witness();
    let (doc, _profile, extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    let before = distance(session.committed_doc(), extrude);
    let outcome = session.perform(SessionOp::Open(
        std::env::temp_dir().join("gui3-no-such-document.pncad"),
    ));
    assert!(matches!(
        outcome.refusal,
        Some(Refusal::Io(DocIoError::Read { .. }))
    ));
    assert_eq!(distance(session.committed_doc(), extrude), before);
    assert!(session.path().is_none());
}

#[test]
fn opening_a_file_that_is_not_a_document_refuses_at_the_persistence_door() {
    let tol = Tol::witness();
    let dir = tempdir("gui3-not-a-document");
    let file = dir.join("junk.pncad");
    std::fs::write(&file, "not a document").expect("the fixture file is writable");
    match docio::open(&file, tol) {
        Err(DocIoError::Persist(_)) => {}
        other => panic!("expected a persistence refusal, got {other:?}"),
    }
    std::fs::remove_dir_all(&dir).expect("the fixture directory is removable");
}

#[test]
fn a_saved_file_is_byte_identical_when_nothing_changed_between_saves() {
    // Save is a function of the history's current path, so saving the
    // same path twice must produce the same bytes — the property that
    // makes "did this document change" answerable at all.
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    session.perform(SessionOp::SetParam {
        name: common::thickness_param(),
        value: SlotValue::Continuous(0.020),
    });
    let dir = tempdir("gui3-stable-save");
    let a = dir.join("a.pncad");
    let b = dir.join("b.pncad");
    session.perform(SessionOp::Save(a.clone()));
    session.perform(SessionOp::Save(b.clone()));
    assert_eq!(
        std::fs::read_to_string(&a).expect("a is readable"),
        std::fs::read_to_string(&b).expect("b is readable")
    );
    std::fs::remove_dir_all(&dir).expect("the fixture directory is removable");
}
