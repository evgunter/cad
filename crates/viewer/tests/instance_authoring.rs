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
    Alignment, AxisSense, Doc, DocumentId, MateFrame, MatePrimitive, Node, NodeResult,
    RecipeNodeId, SlotId,
};
use pncad::geom_core::Tol;
use pncad::select::ContactClass;
use pncad::workspace::Workspace;
use viewer::parts::{PartChooser, PartEntry};
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

    // The three shipped GUI-4 tools this door exists to feed, on an
    // instance it authored: hide, the free-move probe, and the mate.
    let hidden = session.perform(SessionOp::SetInstanceHidden {
        instance: post_i,
        hidden: true,
    });
    assert!(hidden.refusal.is_none(), "{:?}", hidden.refusal);
    assert!(session.display().hidden().contains(&post_i));
    let shown = session.perform(SessionOp::SetInstanceHidden {
        instance: post_i,
        hidden: false,
    });
    assert!(shown.refusal.is_none(), "{:?}", shown.refusal);

    // Free-move: an authored instance is completely unconstrained
    // until a mate lands on it, so the probe opens and commits.
    for op in [
        SessionOp::BeginFreeMove { instance: shelf_i },
        SessionOp::PreviewFreeMove {
            frame: pncad::document::Frame::translation([0.0, 0.0, 0.02]),
        },
        SessionOp::CommitFreeMove,
    ] {
        let outcome = session.perform(op);
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    }
    assert!(session.display().free_move_of(shelf_i).is_some());

    // The shipped mate tool's op takes them from here.
    let outcome = session.perform(SessionOp::AddMate {
        a: asm::in_part(post_i, &bench.post_top),
        b: asm::in_part(shelf_i, &bench.shelf_bottom),
        class: ContactClass::Rest,
        alignment: seat_alignment(),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(outcome.committed.len(), 1);
    assert_eq!(
        outcome.superseded,
        vec![shelf_i],
        "the mate supersedes the probe on the instance it constrains"
    );
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
        Err(refusal @ Refusal::NoDocumentDirectory) => {
            let said = refusal.to_string();
            assert!(
                said.contains("save the document first")
                    && said.contains("references resolve against the file's directory"),
                "the refusal names the recourse the directory rule gives: {said}"
            );
        }
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
fn the_listing_is_ordered_by_file_name_not_by_identity() {
    let tol = Tol::witness();
    let bench = asm::bench("gauth3-order", tol);
    let (session, _) = authored_session(&bench, "gauth3-order-asm", tol);

    let entries = session.part_catalogue().expect("the directory scans");
    let names: Vec<String> = entries.iter().map(PartEntry::file_name).collect();
    let mut sorted = names.clone();
    sorted.sort();
    assert_eq!(
        names, sorted,
        "a chooser is read by name; identity order is hash order"
    );
    assert!(entries.len() >= 3, "the bench's parts and the assemblies");
}

#[test]
fn a_gesture_in_flight_refuses_the_door() {
    let tol = Tol::witness();
    let bench = asm::bench("gauth3-gesture", tol);
    // A PART document, opened from the same directory: it carries the
    // extrude whose distance a gesture can take hold of.
    let post_path = post_file(&bench);
    let mut session = DocSession::inline(Doc::empty_derived("gauth3-gesture-boot", tol), tol);
    let opened = session.perform(SessionOp::Open(post_path));
    assert!(opened.refusal.is_none(), "{:?}", opened.refusal);
    session.pump();
    let extrude = *session
        .doc()
        .order()
        .iter()
        .find(|&&id| matches!(session.doc().node(id), Some(Node::Extrude { .. })))
        .expect("the part has an extrude");

    let begun = session.perform(SessionOp::BeginGesture {
        node: extrude,
        slot: SlotId::Distance,
    });
    assert!(begun.refusal.is_none(), "{:?}", begun.refusal);

    let outcome = session.perform(SessionOp::AddInstance { id: bench.shelf.id });
    assert!(outcome.committed.is_empty(), "mid-gesture, nothing lands");
    assert!(
        matches!(outcome.refusal, Some(Refusal::GestureInFlight)),
        "{:?}",
        outcome.refusal
    );
}

#[test]
fn an_unreadable_sibling_refuses_with_the_scans_own_header_message() {
    let tol = Tol::witness();
    let bench = asm::bench("gauth3-header", tol);
    let (mut session, _) = authored_session(&bench, "gauth3-header-asm", tol);

    // A file CLAIMING to be a document with no readable header: the
    // scan refuses the whole store, which is the posture the workspace
    // documents and the arm the catalogue's `# Errors` names.
    std::fs::write(
        bench.dir.join("not-really.pncad"),
        "this is not a document\n",
    )
    .expect("the junk file writes");
    let expected = Workspace::open(&bench.dir).expect_err("the scan refuses");
    assert!(
        matches!(expected, pncad::workspace::WorkspaceError::Header { .. }),
        "{expected:?}"
    );

    match session.part_catalogue() {
        Err(refusal @ Refusal::Workspace(_)) => {
            assert_eq!(refusal.to_string(), expected.to_string());
        }
        other => panic!("the catalogue shows the scan's refusal, got {other:?}"),
    }
    let outcome = session.perform(SessionOp::AddInstance { id: bench.post.id });
    assert!(outcome.committed.is_empty());
    match outcome.refusal {
        Some(refusal @ Refusal::Workspace(_)) => {
            assert_eq!(refusal.to_string(), expected.to_string());
        }
        other => panic!("expected the scan's refusal, got {other:?}"),
    }
}

#[test]
fn two_instances_of_one_part_insert_and_evaluate() {
    let tol = Tol::witness();
    let bench = asm::bench("gauth3-twice", tol);
    let (mut session, _) = authored_session(&bench, "gauth3-twice-asm", tol);

    let first = add_instance(&mut session, bench.post.id);
    let second = add_instance(&mut session, bench.post.id);
    assert_ne!(first, second, "each insert mints its own node");
    assert_eq!(instance_of(&session, first).0, bench.post);
    assert_eq!(
        instance_of(&session, second).0,
        bench.post,
        "one part, two instances, one reference value"
    );
    assert!(
        !tree::has_faults(&session.tree_rows()),
        "both resolve: {:?}",
        session.tree_rows()
    );
}

#[test]
fn a_directory_that_lost_the_open_documents_own_file_lists_nothing() {
    let tol = Tol::witness();
    let bench = asm::bench("gauth3-vanished-dir", tol);
    let (session, path) = authored_session(&bench, "gauth3-vanished-asm", tol);

    // The only state that reaches the chooser's empty arm: a scan that
    // succeeds and finds nothing, which needs even the open document's
    // own file to have gone.
    for entry in std::fs::read_dir(&bench.dir).expect("the directory reads") {
        let entry = entry.expect("the entry reads");
        if entry.path().extension().is_some_and(|ext| ext == "pncad") {
            std::fs::remove_file(entry.path()).expect("the document is removed");
        }
    }
    assert!(!path.exists(), "the session's own file is gone too");

    let entries = session.part_catalogue().expect("an empty directory scans");
    assert!(
        entries.is_empty(),
        "nothing on offer, not even the open document: {entries:?}"
    );
    let chooser = PartChooser::opened(&session);
    assert_eq!(chooser.offered().expect("the scan succeeds").len(), 0);
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
    let (edited, _) = common::framed_square(&loaded.doc, 0.005, tol);
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
    //
    // **Editing the saved text is the only door, and this is why.** ε
    // is a process-global commitment: a document records the ε of the
    // process that authored it, `Tol` cannot be re-witnessed at a
    // second value inside one test binary, and neither the save door
    // nor the workspace's write side takes an ε to write. The kernel's
    // own ε-seam rows reach this state through a stub resolver, which
    // this suite cannot use — the resolver under test is the real one
    // over a real directory.
    //
    // The mechanism is `doc_io`'s: find the ε LINE by its field name
    // (not by a byte offset into the file), and assert there is
    // exactly one, so a format change fails the row loudly instead of
    // quietly editing the wrong number.
    let file = post_file(&bench);
    let text = std::fs::read_to_string(&file).expect("the post reads");
    let is_epsilon = |line: &&str| line.trim_start().starts_with("\"epsilon\":");
    assert_eq!(
        text.lines().filter(is_epsilon).count(),
        1,
        "a saved document records exactly one ε"
    );
    let line = text.lines().find(is_epsilon).expect("checked above");
    let recorded: f64 = line
        .trim_start()
        .trim_start_matches("\"epsilon\":")
        .trim()
        .trim_end_matches(',')
        .parse()
        .expect("the ε field is a number");
    let doubled = format!("  \"epsilon\": {:e},", recorded * 2.0);
    let mut text: String = text
        .lines()
        .map(|l| if is_epsilon(&l) { doubled.as_str() } else { l })
        .collect::<Vec<&str>>()
        .join("\n");
    text.push('\n');
    std::fs::write(&file, &text).expect("the post rewrites");

    let message = failed_badge(&path, instance, tol);
    assert!(
        message.contains("recorded tolerance disagrees with this process's"),
        "{message}"
    );
}
