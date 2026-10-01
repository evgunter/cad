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

use crate::common;

use std::path::{Path, PathBuf};

use common::asm;
use pncad::document::{Doc, DocumentId, Node, NodeResult, RecipeNodeId, SlotId};
use pncad::geom_core::Tol;
use pncad::select::ContactClass;
use pncad::workspace::Workspace;
use viewer::display::AdmissionFault;
use viewer::parts::{PartChooser, PartEntry};
use viewer::session::{DocSession, Refusal, SessionOp};
use viewer::tree::{self, RowStatus};

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

    let post_i = common::instance_in(&mut session, bench.post.id);
    let shelf_i = common::instance_in(&mut session, bench.shelf.id);

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
            instance: shelf_i,
            frame: pncad::document::Frame::translation([0.0, 0.0, 0.02]),
        },
        SessionOp::CommitFreeMove { instance: shelf_i },
    ] {
        let outcome = session.perform(op);
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    }
    assert!(session.display().free_move_of(shelf_i).is_some());

    // The shipped mate tool's op takes them from here.
    let outcome = session.perform(asm::seat_op_under(
        &bench,
        post_i,
        shelf_i,
        ContactClass::Rest,
        asm::middle_seat_alignment(),
    ));
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(outcome.committed.len(), 1);
    let [superseded] = &outcome.withdrawn.superseded[..] else {
        panic!(
            "exactly one placement is superseded: {:?}",
            outcome.withdrawn.superseded
        )
    };
    assert_eq!(
        superseded.instance, shelf_i,
        "the mate supersedes the probe on the instance it constrains"
    );
    assert!(
        matches!(
            &superseded.cause,
            AdmissionFault::MateConstrained { instance, mates }
                if *instance == shelf_i && !mates.is_empty()
        ),
        "and the outcome carries WHY it went, not only which went — the \
         fault's own PAYLOAD, which is what would go red if the prune paired \
         the right fault with the wrong instance: {}",
        superseded.cause
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
    let chooser = PartChooser::opened(session.part_census());
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

    let first = common::instance_in(&mut session, bench.post.id);
    let second = common::instance_in(&mut session, bench.post.id);
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
    let chooser = PartChooser::opened(session.part_census());
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
    let chooser = PartChooser::opened(session.part_census());
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

    let mut chooser = PartChooser::opened(session.part_census());
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

    chooser.rescan(session.part_census());
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
    let instance = common::instance_in(&mut session, bench.post.id);
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

/// The referenced part gains a feature — A4's Cargo.lock semantics
/// mean the assembly is NOT retargeted, so its pin no longer holds.
fn move_the_posts_pin(bench: &asm::Bench, tol: Tol) {
    let text = std::fs::read_to_string(post_file(bench)).expect("the post reads");
    let loaded = pncad::document::load(&text, tol).expect("the post loads");
    let (edited, _) = common::framed_square(&loaded.doc, 0.005, tol);
    let mut ws = Workspace::open(&bench.dir).expect("the directory scans");
    ws.resave(&edited, tol).expect("the post rewrites");
}

/// The post document as a process at a different ε wrote it: one
/// process, one ε, so the seam refuses at resolution (A2).
fn record_another_epsilon(bench: &asm::Bench) {
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
    let file = post_file(bench);
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
        RowStatus::Failed { message, .. } => message.clone(),
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
    move_the_posts_pin(&bench, tol);

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
    record_another_epsilon(&bench);

    let message = failed_badge(&path, instance, tol);
    assert!(
        message.contains("recorded tolerance disagrees with this process's"),
        "{message}"
    );
}

/// **Every badge a part that does not resolve wears meets the refusal
/// standard** (`test_utils::refusal::problems`), on the store's own
/// sentences through this crate's resolver: the part's file gone, its
/// pin moved, its ε another process's, and the assembly held in memory
/// with no file, which `docio::NoFile` refuses. The ids and pins each prints are
/// admitted span by span, filed with their owner.
#[test]
fn every_unresolved_part_badge_meets_the_refusal_standard() {
    use test_utils::refusal::{Admission, problems_admitting};
    const HEX: &str = "work/edit/part-refusals-name-documents-by-hex-id.md";
    let tol = Tol::witness();
    let mut rows: Vec<(&str, String, Vec<String>)> = Vec::new();

    // Each recourse a door can be seen to honour is followed: the
    // part's file put back, and the held assembly saved beside its
    // parts, each resolves.
    let (bench, path, instance) = one_instance("std-gone", "std-gone-asm", tol);
    let file = post_file(&bench);
    let kept = std::fs::read_to_string(&file).expect("the post reads");
    std::fs::remove_file(&file).expect("the post is removed");
    rows.push((
        "Part/Unresolved(Unresolved)",
        failed_badge(&path, instance, tol),
        vec![bench.post.id.to_string()],
    ));
    std::fs::write(&file, kept).expect("the post is put back");
    let mut back = DocSession::inline(Doc::empty_derived("std-gone-back", tol), tol);
    assert!(
        back.perform(SessionOp::Open(path.clone()))
            .refusal
            .is_none()
    );
    back.pump();
    assert_eq!(
        common::status_of(&back.tree_rows(), instance),
        RowStatus::Ok,
        "the part's file put back in the store's directory resolves"
    );

    let (bench, path, instance) = one_instance("std-pin", "std-pin-asm", tol);
    move_the_posts_pin(&bench, tol);
    let moved = Workspace::open(&bench.dir)
        .expect("the directory scans")
        .current_pin(bench.post.id, tol)
        .expect("the moved post pins");
    rows.push((
        "Part/Unresolved(PinMismatch)",
        failed_badge(&path, instance, tol),
        vec![
            bench.post.id.to_string(),
            bench.post.pin.to_string(),
            moved.to_string(),
        ],
    ));

    let (bench, path, instance) = one_instance("std-eps", "std-eps-asm", tol);
    record_another_epsilon(&bench);
    rows.push((
        "Part/Unresolved(EpsilonSeam)",
        failed_badge(&path, instance, tol),
        vec![bench.post.id.to_string()],
    ));

    // Saved over its own file: a second file would claim the same id,
    // which the store's scan refuses.
    let (_bench, path, instance) = one_instance("std-none", "std-none-asm", tol);
    let held = viewer::docio::open(&path, tol).expect("the assembly opens");
    let mut session = DocSession::inline(held.doc().clone(), tol);
    session.pump();
    let none = match &common::status_of(&session.tree_rows(), instance) {
        RowStatus::Failed { message, .. } => message.clone(),
        other => panic!("an instance with no file fails, got {other:?}"),
    };
    assert!(
        none.contains("Recourse: save it beside its parts"),
        "the recourse followed below: {none}"
    );
    rows.push(("Part/Unresolved(NoFile)", none, Vec::new()));
    assert!(
        session
            .perform(SessionOp::Save(path.clone()))
            .refusal
            .is_none()
    );
    session.pump();
    assert_eq!(
        common::status_of(&session.tree_rows(), instance),
        RowStatus::Ok,
        "saved beside its parts, the assembly evaluates over their store"
    );

    let mut problems = Vec::new();
    for (name, text, spans) in &rows {
        eprintln!("MEASURE {} {name}: {text}", text.split_whitespace().count());
        let admissions: Vec<Admission<'_>> = spans
            .iter()
            .map(|span| Admission {
                row: name,
                span,
                filed: HEX,
            })
            .collect();
        problems.extend(problems_admitting(name, text, &[], false, &admissions));
    }
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

// --- accepting a part's updated version ----------------------------
//
// The bench holds two instances of the post and one of the shelf, so
// moving the post's pin gives one part with two refusing sites beside
// one that still resolves. The session is opened AFTER the move: the
// memo keys an instance on its reference, not on the store, so an open
// session would not see the file change (the badge rows above reopen
// for the same reason).

/// The bench, its post moved on disk, opened fresh.
fn bench_with_the_post_moved(tag: &str, tol: Tol) -> (asm::Bench, DocSession) {
    let bench = asm::bench(tag, tol);
    move_the_posts_pin(&bench, tol);
    let session = asm::open_bench(&bench, tol);
    (bench, session)
}

/// The row `node` draws on.
fn row_of(session: &DocSession, node: RecipeNodeId) -> tree::TreeRow {
    session
        .tree_rows()
        .into_iter()
        .find(|row| row.id == node)
        .unwrap_or_else(|| panic!("node {} has a row", node.0))
}

/// **Offer → accept → one undo**, on the real session over the bench's
/// directory. Red if a pin-mismatched instance offers nothing, or an
/// offer names another part or file; if the resolving shelf offers
/// anything; if accepting commits anything but `update_to_store`'s
/// edits, or as more than one history step; if the posts do not then
/// evaluate; or if one undo does not bring back the document, the
/// mismatch and the offer.
#[test]
fn a_pin_mismatched_instance_offers_the_accept_and_accepting_is_one_undo() {
    let tol = Tol::witness();
    let (bench, mut session) = bench_with_the_post_moved("auth15-accept", tol);
    let post_file_name = post_file(&bench)
        .file_name()
        .expect("the post's file has a name")
        .to_string_lossy()
        .into_owned();

    let offered = |session: &DocSession| {
        [bench.post_a, bench.post_b].map(|post| {
            let row = row_of(session, post);
            assert_eq!(row.status.badge(), "FAILED", "the post refuses: {row:?}");
            let offer = row
                .version_offer
                .clone()
                .unwrap_or_else(|| panic!("a pin-mismatched instance offers the accept: {row:?}"));
            assert!(
                matches!(offer.accept(), SessionOp::AcceptPartVersion { id } if id == bench.post.id),
                "the accept names the post: {:?}",
                offer.accept()
            );
            offer
        })
    };
    let [offer, other] = offered(&session);
    assert_eq!(offer, other, "both sites of one part offer one accept");
    assert!(
        Refusal::version_question(&offer).contains(&post_file_name),
        "the question names the post's file: {}",
        Refusal::version_question(&offer)
    );
    let shelf = row_of(&session, bench.shelf_i);
    assert_eq!(shelf.status, RowStatus::Ok, "the shelf still resolves");
    assert_eq!(
        shelf.version_offer, None,
        "a resolving instance offers nothing"
    );

    let before = session.committed_doc().clone();
    let steps = session.history().len();
    let kernel = pncad::workspace::update_to_store(
        &before,
        bench.post.id,
        &Workspace::open(&bench.dir).expect("the directory scans"),
        tol,
    )
    .expect("the store holds a newer post");
    assert_eq!(kernel.len(), 2, "the premise: one edit per post instance");

    let outcome = session.perform(offer.accept());
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(
        outcome.committed, kernel,
        "accepting commits the kernel's edits, in its order"
    );
    assert_eq!(session.history().len(), steps + 1, "as one action");
    // Until the accept's own run lands, the rows are the old run's, and
    // an offer read off them would be the edit just made, offered again.
    let unlanded = session.tree_rows();
    assert_eq!(row_of(&session, bench.post_a).status.badge(), "FAILED");
    assert!(
        unlanded.iter().all(|row| row.version_offer.is_none()),
        "no offer stands while the accept's run is outstanding: {unlanded:?}"
    );
    session.pump();
    let rows = session.tree_rows();
    assert!(!tree::has_faults(&rows), "the posts now evaluate: {rows:?}");
    assert!(
        rows.iter().all(|row| row.version_offer.is_none()),
        "and nothing offers any more"
    );

    let undone = session.perform(SessionOp::Undo);
    assert!(undone.refusal.is_none(), "{:?}", undone.refusal);
    assert!(
        session.committed_doc().bit_eq(&before),
        "one undo restores the old pins"
    );
    session.pump();
    let [again, _] = offered(&session);
    assert_eq!(again, offer, "and with them the mismatch and its offer");
}

/// **What the store answers is said as-is**: accepting when every
/// reference already holds the store's version, and after the part's
/// file has gone from under an offer. Red if either commits, or if the
/// status line says anything but the kernel's own refusal; and red if
/// an instance whose file is gone offers the accept.
#[test]
fn accepting_with_no_newer_version_or_no_file_says_the_kernels_refusal() {
    let tol = Tol::witness();
    let said = |session: &mut DocSession, bench: &asm::Bench| {
        let before = session.committed_doc().clone();
        let steps = session.history().len();
        let refusal = session
            .perform(SessionOp::AcceptPartVersion { id: bench.post.id })
            .refusal
            .expect("the accept refuses");
        assert!(session.committed_doc().bit_eq(&before), "commits nothing");
        assert_eq!(session.history().len(), steps, "and records no step");
        let kernel = pncad::workspace::update_to_store(
            &before,
            bench.post.id,
            &Workspace::open(&bench.dir).expect("the directory scans"),
            tol,
        )
        .expect_err("the kernel refuses too");
        assert_eq!(
            viewer::frame::refusal_message(&refusal).text(),
            kernel.to_string(),
            "the status line is the kernel's sentence"
        );
        kernel
    };

    // Every reference already names the store's version.
    let bench = asm::bench("auth15-current", tol);
    let mut session = asm::open_bench(&bench, tol);
    assert!(
        session
            .tree_rows()
            .iter()
            .all(|row| row.version_offer.is_none()),
        "a resolving assembly offers nothing"
    );
    let kernel = said(&mut session, &bench);
    assert!(
        matches!(
            kernel,
            pncad::workspace::WorkspaceError::Update {
                error: pncad::document::UpdateError::AlreadyPinned { .. }
            }
        ),
        "{kernel:?}"
    );

    // An offer drawn, then its part's file removed.
    let (bench, mut session) = bench_with_the_post_moved("auth15-gone", tol);
    assert!(row_of(&session, bench.post_a).version_offer.is_some());
    std::fs::remove_file(post_file(&bench)).expect("the post is removed");
    let kernel = said(&mut session, &bench);
    assert!(
        matches!(kernel, pncad::workspace::WorkspaceError::UnknownId { .. }),
        "{kernel:?}"
    );
    // Opened now, the instance does not resolve at all, and no version
    // of a part with no file is on offer.
    let reopened = asm::open_bench(&bench, tol);
    let gone = row_of(&reopened, bench.post_a);
    assert_eq!(gone.status.badge(), "FAILED");
    assert_eq!(gone.version_offer, None, "a missing part offers nothing");
}

/// **The accept is computed against the committed document, not the
/// landed one.** One post instance is deleted and the accept performed
/// before either run lands: the landed run still holds both posts, the
/// committed document one. Red if the accept elaborates over the landed
/// document (two edits, one naming a deleted node); if the assembly
/// does not then evaluate; or if one undo takes back more than the
/// accept.
#[test]
fn the_accept_reads_the_committed_document_before_its_run_lands() {
    let tol = Tol::witness();
    let (bench, mut session) = bench_with_the_post_moved("auth15-committed", tol);
    let deleted = session.perform(SessionOp::DeleteNode { node: bench.post_b });
    assert!(deleted.refusal.is_none(), "{:?}", deleted.refusal);
    let after_delete = session.committed_doc().clone();
    let steps = session.history().len();

    let outcome = session.perform(SessionOp::AcceptPartVersion { id: bench.post.id });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let kernel = pncad::workspace::update_to_store(
        &after_delete,
        bench.post.id,
        &Workspace::open(&bench.dir).expect("the directory scans"),
        tol,
    )
    .expect("the store holds a newer post");
    assert_eq!(
        outcome.committed.len(),
        1,
        "one post instance is left to move: {:?}",
        outcome.committed
    );
    assert_eq!(
        outcome.committed, kernel,
        "the kernel's edits, as committed"
    );
    assert_eq!(session.history().len(), steps + 1, "as one action");

    session.pump();
    let rows = session.tree_rows();
    assert!(!tree::has_faults(&rows), "the assembly evaluates: {rows:?}");

    let undone = session.perform(SessionOp::Undo);
    assert!(undone.refusal.is_none(), "{:?}", undone.refusal);
    assert!(
        session.committed_doc().bit_eq(&after_delete),
        "one undo takes back the accept and leaves the delete"
    );
}
