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
    match outcome.refusal {
        Some(Refusal::Io(ref error)) => assert!(matches!(**error, DocIoError::Read { .. })),
        ref other => panic!("expected a read refusal, got {other:?}"),
    }
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

/// **A real gallery document, opened through the typed door.**
///
/// The file is the tour's `gallery` mode output for the ring scene,
/// committed verbatim — so this row is the acceptance walk (open a
/// gallery `.pncad`, see its feature tree with live statuses, save it
/// back) with the dialog and the window taken out.
///
/// It is version-stamped in its name, as `pncad`'s own fixture is: a
/// schema break makes this file unreadable, and the fix is to
/// regenerate it from `demo-tour gallery` and rename, never to teach
/// the loader about an old shape.
const GALLERY_RING: &str = include_str!("gallery_ring.v14.pncad");

/// **ε is a run parameter, and a saved document records the one it was
/// decided at** — "one process, one ε", which `load` enforces by
/// refusing a file whose recorded ε is not the process's
/// (`PersistError::ToleranceConflict`). The CI matrix sweeps ε, so a
/// committed document fixture is loadable at exactly one of its
/// points and refuses at the others; this row went red at 1e-12 for
/// precisely that reason.
///
/// So the fixture is re-stamped with THIS run's ε before it is
/// opened. That is not an adjustment to make a row pass: it is
/// byte-for-byte what `demo-tour gallery` writes under this ε, because
/// the only ε-dependent byte in the file is the recorded value itself
/// (the ring's dimensions are authored literals, and `save` takes the
/// ε from `Doc::epsilon`). Measured, not assumed — the row below
/// asserts that the re-stamp changes exactly the ε line and nothing
/// else, so a future ε-dependent byte in the format fails here instead
/// of being quietly papered over.
///
/// The new ε line comes from `save` itself, via a throwaway document
/// at the process tolerance: spelling a float the way the serializer
/// spells it is the serializer's job, not this file's.
fn gallery_ring_at(tol: Tol) -> String {
    let probe: pncad::document::Doc<pncad::document::ProfileProgram> =
        pncad::document::Doc::empty_derived("gui3-epsilon-probe", tol);
    let probe_text = pncad::document::save(&probe, &[], tol).expect("an empty document saves");
    let is_epsilon = |line: &&str| line.trim_start().starts_with("\"epsilon\":");
    let wanted = probe_text
        .lines()
        .find(is_epsilon)
        .expect("a saved document records its ε");

    let (kept, replaced): (Vec<&str>, Vec<&str>) =
        GALLERY_RING.lines().partition(|line| !is_epsilon(line));
    assert_eq!(
        replaced.len(),
        1,
        "the fixture must carry exactly one ε line; found {}",
        replaced.len()
    );
    let restamped: Vec<String> = GALLERY_RING
        .lines()
        .map(|line| {
            if is_epsilon(&line) {
                wanted.to_owned()
            } else {
                line.to_owned()
            }
        })
        .collect();
    // The only line that moved is ε's — the claim above, checked.
    let stripped: Vec<&String> = restamped
        .iter()
        .filter(|line| !is_epsilon(&line.as_str()))
        .collect();
    assert_eq!(
        stripped.len(),
        kept.len(),
        "re-stamping ε changed the line count"
    );
    assert!(
        stripped.iter().zip(&kept).all(|(a, b)| a.as_str() == *b),
        "re-stamping ε changed a line that is not ε's"
    );
    let mut text = restamped.join("\n");
    text.push('\n');
    text
}

#[test]
fn a_gallery_document_opens_evaluates_and_saves_back() {
    let tol = Tol::witness();
    let dir = tempdir("gui3-gallery");
    let file = dir.join("ring.pncad");
    std::fs::write(&file, gallery_ring_at(tol)).expect("the fixture is writable");

    let history = docio::open(&file, tol).expect("the gallery document opens");
    let mut session = DocSession::new(
        history.doc().clone(),
        tol,
        Box::new(viewer::InlineEvaluator::new()),
    );
    session.pump();

    let rows = session.tree_rows();
    assert_eq!(rows.len(), 3, "profile, axis datum, revolve");
    assert!(
        !viewer::tree::has_faults(&rows),
        "a gallery document evaluates clean: {:?}",
        rows.iter().map(|r| &r.status).collect::<Vec<_>>()
    );
    assert!(
        rows.iter().any(|row| row.kind == "Revolve" && row.root),
        "the revolve is the product root"
    );

    // Round-trip: opened, saved, and opened again is the same document.
    let out = dir.join("ring-again.pncad");
    assert!(
        session
            .perform(SessionOp::Save(out.clone()))
            .refusal
            .is_none()
    );
    let reopened = docio::open(&out, tol).expect("the re-saved document opens");
    assert!(
        reopened.doc().bit_eq(history.doc()),
        "bit-identical round trip"
    );

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
