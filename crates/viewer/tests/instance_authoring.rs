//! **Placing an instance: the assembly-authoring door, headless**
//! (GAUTH-3). A directory of part documents, a new assembly authored
//! into it through `SessionOp::AddInstance`, the two instances mated
//! with the shipped op, saved, reloaded and re-evaluated — plus every
//! refusal arm the door owns, and the tree badges an authored
//! reference can end up wearing.
//!
//! The fixture is the gallery-shaped workspace the GUI-4 suites use
//! (`common::asm`): part documents beside the assembly that pins them.
//! What is new here is that the assembly under test is authored BY
//! this crate's operations rather than assembled by the fixture, which
//! is the whole subject.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

mod common;

use std::path::{Path, PathBuf};

use common::asm;
use pncad::document::{
    Alignment, AxisSense, Doc, DocumentId, MateFrame, MatePrimitive, Node, NodeResult, RecipeNodeId,
};
use pncad::geom_core::Tol;
use pncad::select::ContactClass;
use pncad::workspace::Workspace;
use viewer::parts::PartChooser;
use viewer::session::{DocSession, Refusal, SessionOp};
use viewer::tree::{self, RowStatus};

/// The seat mate the acceptance authors: the post's top cap under the
/// shelf's underside, frames in each part's own coordinates.
fn seat_alignment() -> Alignment {
    Alignment {
        a: MateFrame {
            origin: [
                asm::POST_SECTION / 2.0,
                asm::POST_SECTION / 2.0,
                asm::POST_HEIGHT,
            ],
            axis: [0.0, 0.0, 1.0],
            reference: [1.0, 0.0, 0.0],
        },
        b: MateFrame {
            origin: [asm::SHELF_LENGTH / 2.0, asm::SHELF_DEPTH / 2.0, 0.0],
            axis: [0.0, 0.0, -1.0],
            reference: [1.0, 0.0, 0.0],
        },
        primitive: MatePrimitive::FrameCoincidence,
        sense: AxisSense::Opposed,
        clocking: None,
    }
}

/// A session over a NEW empty document, saved into the bench's
/// directory — which is what gives it a resolver, and therefore a
/// catalogue.
fn authored_session(bench: &asm::Bench, label: &str, tol: Tol) -> (DocSession, PathBuf) {
    let mut session = DocSession::inline(Doc::empty_derived(label, tol), tol);
    let path = bench.dir.join(format!("{label}.pncad"));
    let outcome = session.perform(SessionOp::Save(path.clone()));
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    (session, path)
}

/// Perform one `AddInstance` and answer the node it minted, asserting
/// the op's own contract: no refusal, exactly one committed edit.
fn add_instance(session: &mut DocSession, id: DocumentId) -> RecipeNodeId {
    let before: Vec<RecipeNodeId> = session.doc().order().to_vec();
    let outcome = session.perform(SessionOp::AddInstance { id });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(
        outcome.committed.len(),
        1,
        "an instance is exactly one committed edit"
    );
    session.pump();
    let minted: Vec<RecipeNodeId> = session
        .doc()
        .order()
        .iter()
        .copied()
        .filter(|id| !before.contains(id))
        .collect();
    match minted.as_slice() {
        [only] => *only,
        other => panic!("one insert mints one node, got {other:?}"),
    }
}

/// The instance's node, as the document holds it.
fn instance_of(session: &DocSession, node: RecipeNodeId) -> (pncad::document::DocRef, bool) {
    match session.doc().node(node) {
        Some(Node::InstantiatePart { doc_ref, interface }) => (*doc_ref, interface.is_empty()),
        other => panic!("node {} should be an instance, got {other:?}", node.0),
    }
}

// --- the acceptance ------------------------------------------------

#[test]
fn an_assembly_authored_into_a_directory_of_parts_round_trips() {
    let tol = Tol::witness();
    let bench = asm::bench("gauth3-author", tol);
    let (mut session, path) = authored_session(&bench, "gauth3-authored", tol);

    let post_i = add_instance(&mut session, bench.post.id);
    let shelf_i = add_instance(&mut session, bench.shelf.id);

    // What the door authored: the store's CURRENT version of each
    // part, an empty interface record (an authored instance crosses no
    // split seam), and no placement — A11 puts that on the cluster.
    let (post_ref, post_interface) = instance_of(&session, post_i);
    assert_eq!(
        post_ref, bench.post,
        "the minted reference pins the store's current content"
    );
    assert!(post_interface, "an authored instance crosses no seam");
    let (shelf_ref, shelf_interface) = instance_of(&session, shelf_i);
    assert_eq!(shelf_ref, bench.shelf);
    assert!(shelf_interface);
    assert!(
        session.doc().placements().get(&post_i).is_none()
            && session.doc().placements().get(&shelf_i).is_none(),
        "AddInstance authors no placement"
    );

    // The instances evaluate: the references resolve through the
    // directory rule, with no faults on any row.
    assert!(
        !tree::has_faults(&session.tree_rows()),
        "the authored instances resolve: {:?}",
        session.tree_rows()
    );

    // The shipped mate tool's op takes them from here.
    let outcome = session.perform(SessionOp::AddMate {
        a: asm::in_part(post_i, &bench.post_top),
        b: asm::in_part(shelf_i, &bench.shelf_bottom),
        class: ContactClass::Rest,
        alignment: seat_alignment(),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(outcome.committed.len(), 1);
    session.pump();
    assert!(!tree::has_faults(&session.tree_rows()));

    // Save, reload, re-evaluate: the same document, from the file.
    let saved = session.perform(SessionOp::Save(path.clone()));
    assert!(saved.refusal.is_none(), "{:?}", saved.refusal);
    let mut reopened = DocSession::inline(Doc::empty_derived("gauth3-reboot", tol), tol);
    let opened = reopened.perform(SessionOp::Open(path));
    assert!(opened.refusal.is_none(), "{:?}", opened.refusal);
    reopened.pump();

    assert!(
        !tree::has_faults(&reopened.tree_rows()),
        "the reloaded assembly evaluates: {:?}",
        reopened.tree_rows()
    );
    assert!(
        reopened.product_fault().is_none(),
        "{:?}",
        reopened.product_fault()
    );
    assert_eq!(instance_of(&reopened, post_i).0, bench.post);
    assert_eq!(instance_of(&reopened, shelf_i).0, bench.shelf);
    assert!(
        matches!(
            reopened.at_rest(),
            Some(viewer::session::AtRestBadge::Certified { .. })
        ),
        "the authored Rest mate certifies at rest: {:?}",
        reopened.at_rest()
    );
}

// --- the refusal arms ----------------------------------------------

#[test]
fn a_session_with_no_backing_file_refuses_and_names_the_recourse() {
    let tol = Tol::witness();
    let session = DocSession::inline(Doc::empty_derived("gauth3-unsaved", tol), tol);

    // The catalogue: there is no directory to list.
    match session.part_catalogue() {
        Err(refusal @ Refusal::NoDocumentDirectory) => assert_eq!(
            refusal.to_string(),
            Refusal::SAVE_FIRST,
            "the refusal names the recourse the directory rule gives"
        ),
        other => panic!("an unsaved session has no catalogue, got {other:?}"),
    }

    // The chooser opens anyway and shows that refusal where the list
    // would be — a door that cannot open says so.
    let chooser = PartChooser::opened(&session);
    assert!(chooser.dir().is_none());
    match chooser.offered() {
        Err(Refusal::NoDocumentDirectory) => {}
        other => panic!("the chooser shows the refusal, got {other:?}"),
    }

    // And the op itself refuses, committing nothing.
    let mut session = session;
    let before = session.doc().order().len();
    let outcome = session.perform(SessionOp::AddInstance {
        id: DocumentId::derive("gauth3-anything"),
    });
    assert!(outcome.committed.is_empty(), "nothing was committed");
    match outcome.refusal {
        Some(refusal @ Refusal::NoDocumentDirectory) => {
            assert!(refusal.to_string().contains("save the document first"));
        }
        other => panic!("expected the no-directory refusal, got {other:?}"),
    }
    assert_eq!(session.doc().order().len(), before);
}

#[test]
fn the_catalogue_lists_the_directory_and_marks_the_open_document() {
    let tol = Tol::witness();
    let bench = asm::bench("gauth3-catalogue", tol);
    let (session, _) = authored_session(&bench, "gauth3-listing", tol);

    let entries = session.part_catalogue().expect("the directory scans");
    let ids: Vec<DocumentId> = entries.iter().map(|entry| entry.id).collect();
    assert!(
        ids.contains(&bench.post.id) && ids.contains(&bench.shelf.id),
        "both parts are on offer: {ids:?}"
    );
    let open = session.doc().id();
    let self_entry = entries
        .iter()
        .find(|entry| entry.id == open)
        .expect("the open document is listed too");
    assert!(
        self_entry.open_document,
        "the open document's own entry is marked, not filtered away"
    );
    assert!(entries.iter().filter(|entry| entry.open_document).count() == 1);
    let post = entries
        .iter()
        .find(|entry| entry.id == bench.post.id)
        .expect("the post is listed");
    assert!(!post.open_document);
    assert!(
        post.file_name().ends_with(".pncad"),
        "the chooser labels an entry by its file: {}",
        post.file_name()
    );
    assert_eq!(post.path.parent(), Some(bench.dir.as_path()));
}

#[test]
fn a_document_refuses_to_instantiate_itself() {
    let tol = Tol::witness();
    let bench = asm::bench("gauth3-self", tol);
    let (mut session, _) = authored_session(&bench, "gauth3-selfref", tol);
    let own = session.doc().id();
    let before = session.doc().order().len();

    let outcome = session.perform(SessionOp::AddInstance { id: own });
    assert!(outcome.committed.is_empty());
    match outcome.refusal {
        Some(Refusal::SelfInstance { id }) => assert_eq!(id, own),
        other => panic!("expected the self-instance refusal, got {other:?}"),
    }
    assert_eq!(session.doc().order().len(), before, "nothing was inserted");
}

#[test]
fn an_id_the_directory_does_not_hold_refuses_in_the_stores_own_words() {
    let tol = Tol::witness();
    let bench = asm::bench("gauth3-unknown", tol);
    let (mut session, _) = authored_session(&bench, "gauth3-unknown-asm", tol);
    let absent = DocumentId::derive("gauth3-no-such-part");

    let outcome = session.perform(SessionOp::AddInstance { id: absent });
    assert!(outcome.committed.is_empty());
    match outcome.refusal {
        Some(Refusal::Workspace(error)) => {
            let expected = Workspace::open(&bench.dir)
                .expect("the directory scans")
                .current_pin(absent, tol)
                .expect_err("no such document");
            assert_eq!(
                error.to_string(),
                expected.to_string(),
                "the store's own sentence, verbatim"
            );
        }
        other => panic!("expected the store's refusal, got {other:?}"),
    }
}

#[test]
fn a_duplicate_id_refuses_at_the_chooser_and_at_the_op() {
    let tol = Tol::witness();
    let bench = asm::bench("gauth3-duplicate", tol);
    let (mut session, _) = authored_session(&bench, "gauth3-duplicate-asm", tol);

    // The #1117 shape: a COPY of a part beside the original, so two
    // files claim one identity and the store refuses the whole scan.
    let post_path = Workspace::open(&bench.dir)
        .expect("the directory scans")
        .documents()
        .get(&bench.post.id)
        .expect("the post is stored")
        .clone();
    std::fs::copy(&post_path, bench.dir.join("a-copy-of-the-post.pncad")).expect("the copy writes");
    let expected = Workspace::open(&bench.dir).expect_err("two files, one id");
    assert!(
        matches!(
            expected,
            pncad::workspace::WorkspaceError::DuplicateId { .. }
        ),
        "{expected:?}"
    );

    // At the chooser: no list, the store's sentence in its place.
    let chooser = PartChooser::opened(&session);
    match chooser.offered() {
        Err(refusal @ Refusal::Workspace(_)) => {
            assert_eq!(refusal.to_string(), expected.to_string());
        }
        other => panic!("the chooser shows the scan's refusal, got {other:?}"),
    }

    // And at the op, which never reaches a pin.
    let outcome = session.perform(SessionOp::AddInstance { id: bench.shelf.id });
    assert!(outcome.committed.is_empty());
    match outcome.refusal {
        Some(refusal @ Refusal::Workspace(_)) => {
            assert_eq!(refusal.to_string(), expected.to_string());
        }
        other => panic!("expected the scan's refusal, got {other:?}"),
    }
}

#[test]
fn the_chooser_holds_a_snapshot_and_rescan_re_reads_it() {
    let tol = Tol::witness();
    let bench = asm::bench("gauth3-snapshot", tol);
    let (session, _) = authored_session(&bench, "gauth3-snapshot-asm", tol);

    let mut chooser = PartChooser::opened(&session);
    let before = chooser.offered().expect("the directory scans").len();
    assert_eq!(chooser.dir(), Some(bench.dir.as_path()));

    // A part arrives while the chooser is open.
    let extra = Doc::empty_derived("gauth3-latecomer", tol);
    let mut ws = Workspace::open(&bench.dir).expect("the directory scans");
    ws.create(&extra, tol).expect("the latecomer stores");
    assert_eq!(
        chooser.offered().expect("still the snapshot").len(),
        before,
        "the held listing is the scan it was taken from"
    );

    chooser.rescan(&session);
    let after = chooser.offered().expect("the directory scans").len();
    assert_eq!(after, before + 1, "rescan re-reads the directory");
}

// --- what an authored reference can end up badging -----------------
//
// The instantiate node's resolution refusals are the document layer's
// and render on the feature tree GUI-3 built. These rows drive them
// from the AUTHORED path: an instance placed by the door above, whose
// part then moves, vanishes, or turns out to record another ε.

/// An assembly authored into the bench's directory holding one
/// instance of the post, saved.
fn one_instance(tag: &str, label: &str, tol: Tol) -> (asm::Bench, PathBuf, RecipeNodeId) {
    let bench = asm::bench(tag, tol);
    let (mut session, path) = authored_session(&bench, label, tol);
    let instance = add_instance(&mut session, bench.post.id);
    let saved = session.perform(SessionOp::Save(path.clone()));
    assert!(saved.refusal.is_none(), "{:?}", saved.refusal);
    (bench, path, instance)
}

/// The post document's save file inside the bench's directory.
fn post_file(bench: &asm::Bench) -> PathBuf {
    Workspace::open(&bench.dir)
        .expect("the directory scans")
        .documents()
        .get(&bench.post.id)
        .expect("the post is stored")
        .clone()
}

/// Open `path` fresh and answer the instance row's FAILED badge
/// message, having first checked it is the evaluation's own payload
/// rendering rather than a sentence the tree composed.
fn failed_badge(path: &Path, node: RecipeNodeId, tol: Tol) -> String {
    let mut session = DocSession::inline(Doc::empty_derived("gauth3-badge-boot", tol), tol);
    let opened = session.perform(SessionOp::Open(path.to_path_buf()));
    assert!(opened.refusal.is_none(), "{:?}", opened.refusal);
    session.pump();
    let rows = session.tree_rows();
    let row = rows
        .iter()
        .find(|row| row.id == node)
        .expect("the instance has a row");
    let message = match &row.status {
        RowStatus::Failed { message } => message.clone(),
        other => panic!("expected the instance to fail, got {other:?}"),
    };
    assert_eq!(row.status.badge(), "FAILED");
    let evaluation = session.evaluation().expect("a result landed");
    let Some(NodeResult::Failed(error)) = evaluation.result(node) else {
        panic!("the evaluation should report the instance as failed");
    };
    assert_eq!(
        message,
        error.to_string(),
        "the badge is the payload's own rendering"
    );
    message
}

#[test]
fn an_authored_instance_whose_part_moved_badges_the_pin_mismatch() {
    let tol = Tol::witness();
    let (bench, path, instance) = one_instance("gauth3-pin", "gauth3-pin-asm", tol);

    // The referenced part gains a feature — A4's Cargo.lock semantics
    // mean the assembly is NOT retargeted, so its pin no longer holds.
    let text = std::fs::read_to_string(post_file(&bench)).expect("the post reads");
    let loaded = pncad::document::load(&text, tol).expect("the post loads");
    let (edited, _) = common::inserted(&loaded.doc, common::square(0.005), tol);
    let mut ws = Workspace::open(&bench.dir).expect("the directory scans");
    ws.resave(&edited, tol).expect("the post rewrites");

    let message = failed_badge(&path, instance, tol);
    assert!(
        message.contains("the reference's pin does not hold"),
        "the fault is classified as a pin mismatch: {message}"
    );
    assert!(
        message.contains(pncad::workspace::PIN_MISMATCH_RECOURSE),
        "and the badge carries the store's recourse: {message}"
    );
}

#[test]
fn an_authored_instance_whose_part_vanished_badges_unresolved() {
    let tol = Tol::witness();
    let (bench, path, instance) = one_instance("gauth3-gone", "gauth3-gone-asm", tol);
    std::fs::remove_file(post_file(&bench)).expect("the post is removed");

    let message = failed_badge(&path, instance, tol);
    assert!(
        message.contains("the reference did not resolve"),
        "{message}"
    );
    assert!(
        message.contains(&bench.post.id.to_string()),
        "the store's sentence names the id it could not find: {message}"
    );
}

#[test]
fn an_authored_instance_whose_part_records_another_epsilon_badges_the_seam() {
    let tol = Tol::witness();
    let (bench, path, instance) = one_instance("gauth3-eps", "gauth3-eps-asm", tol);

    // A part document written by a process at a different ε: one
    // process, one ε, so the seam refuses at resolution (A2).
    let file = post_file(&bench);
    let text = std::fs::read_to_string(&file).expect("the post reads");
    let key = "\"epsilon\": ";
    let at = text.find(key).expect("the document records its epsilon");
    let rest = &text[at + key.len()..];
    let end = rest.find(',').expect("the epsilon field ends");
    let recorded: f64 = rest[..end].trim().parse().expect("a number");
    let text = format!("{}{key}{:e}{}", &text[..at], recorded * 2.0, &rest[end..]);
    std::fs::write(&file, &text).expect("the post rewrites");

    let message = failed_badge(&path, instance, tol);
    assert!(
        message.contains("recorded tolerance disagrees with this process's"),
        "{message}"
    );
}
