//! Review R1's end-to-end consumer walk for GUI-3 (PR #1101).
//!
//! Drives the panels' typed operation vocabulary headlessly, as a
//! consumer would: open real gallery documents through the typed door,
//! walk the tree, edit through `apply`-emitting operations, undo/redo
//! across a branch point, cancel a long evaluation on the THREADED
//! seam, save and re-open. The dialog and pixel-painting are the only
//! surfaces not exercised.
//!
//! Usage: `cargo run -p viewer --example r1_e2e -- <gallery-dir>`
//! where `<gallery-dir>` is `demo-tour gallery`'s output.

#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use std::path::Path;

use pncad::geom_core::Tol;
use viewer::evalseam::{EvalService, ThreadEvaluator};
use viewer::props::SlotValue;
use viewer::session::{DocSession, Selection, SessionOp};
use viewer::tree::RowStatus;
use viewer::{docio, tree};

fn banner(text: &str) {
    println!("\n== {text} ==");
}

fn show_tree(session: &DocSession) {
    for row in session.tree_rows() {
        let indent = "  ".repeat(row.depth);
        let root = if row.root { " ▸root" } else { "" };
        let message = row
            .status
            .message()
            .map(|m| format!("  [{m}]"))
            .unwrap_or_default();
        println!(
            "   {indent}{} {}{root}{message}",
            row.status.badge(),
            row.kind
        );
    }
}

fn pump_until_idle(session: &mut DocSession, label: &str) {
    let start = std::time::Instant::now();
    while session.busy() {
        session.pump();
        if start.elapsed().as_secs() > 300 {
            panic!("{label}: evaluation did not land within 300 s");
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    println!("   {label}: landed in {:?}", start.elapsed());
}

fn main() {
    let dir = std::env::args()
        .nth(1)
        .expect("usage: r1_e2e <gallery-dir>");
    let dir = Path::new(&dir);
    let tol = Tol::witness();

    // -- 1. Open the ring through the typed door, on the THREADED seam
    // (the lane CI's session rows never drive). --------------------
    banner("ring.pncad: open, tree, edit, undo/redo across a branch");
    let history = docio::open(&dir.join("ring.pncad"), tol).expect("ring opens");
    let mut session = DocSession::new(
        history.doc().clone(),
        tol,
        Box::new(ThreadEvaluator::spawn().expect("the worker starts")),
    );
    pump_until_idle(&mut session, "first evaluation");
    show_tree(&session);
    let rows = session.tree_rows();
    assert!(!tree::has_faults(&rows), "the ring evaluates clean");

    // Select the revolve (the root) and inspect its slots.
    let revolve = rows
        .iter()
        .find(|row| row.kind == "Revolve")
        .expect("the ring has a revolve")
        .id;
    session.perform(SessionOp::Select(Selection::Node(revolve)));
    for slot in session.slot_rows() {
        println!(
            "   slot {:?} ({:?}, {}) = {:?}",
            slot.slot,
            slot.dimension,
            if slot.driver.is_driven() {
                "driven"
            } else {
                "literal"
            },
            slot.value
        );
    }

    // Edit a literal slot twice, undo once, edit again: a branch.
    let editable = session
        .slot_rows()
        .into_iter()
        .find(|row| !row.driver.is_driven() && !row.structural)
        .expect("the revolve has an editable continuous slot");
    let base = match editable.value.as_ref().expect("it evaluates") {
        SlotValue::Continuous(v) => *v,
        SlotValue::Count(_) => panic!("expected continuous"),
    };
    for factor in [1.5, 2.0] {
        let outcome = session.perform(SessionOp::SetSlot {
            node: revolve,
            slot: editable.slot,
            value: SlotValue::Continuous(base * factor),
        });
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
        assert_eq!(outcome.committed.len(), 1);
    }
    session.perform(SessionOp::Undo);
    let outcome = session.perform(SessionOp::SetSlot {
        node: revolve,
        slot: editable.slot,
        value: SlotValue::Continuous(base * 3.0),
    });
    assert!(outcome.refusal.is_none());
    assert_eq!(
        session.history().len(),
        4,
        "root + two edits + the sibling: nothing destroyed"
    );
    session.perform(SessionOp::Undo);
    session.perform(SessionOp::Redo);
    pump_until_idle(&mut session, "after undo/redo");
    println!(
        "   history states: {}, path edits: {}",
        session.history().len(),
        session.history().path_edits().len()
    );

    // Save the branch's path and re-open it.
    let out = std::env::temp_dir().join("r1-e2e-ring-branch.pncad");
    assert!(
        session
            .perform(SessionOp::Save(out.clone()))
            .refusal
            .is_none()
    );
    let reopened = docio::open(&out, tol).expect("the saved branch opens");
    assert_eq!(
        reopened.path_edits().len(),
        2,
        "the current path (edit + sibling), not the abandoned branch"
    );
    println!(
        "   saved + reopened: {} path edits",
        reopened.path_edits().len()
    );

    // -- 2. The long evaluation: diefillet on the threaded seam, with
    // a real mid-flight cancel. -------------------------------------
    banner("diefillet.pncad: open on the threaded seam, cancel mid-flight");
    let history = docio::open(&dir.join("diefillet.pncad"), tol).expect("diefillet opens");
    let mut session = DocSession::new(
        history.doc().clone(),
        tol,
        Box::new(ThreadEvaluator::spawn().expect("the worker starts")),
    );
    std::thread::sleep(std::time::Duration::from_millis(50));
    session.perform(SessionOp::CancelEvaluation);
    // UPDATED AT THE FIX PASS. This loop waited on `busy()`, which was
    // the right wait against the head this review froze on: the
    // canceled prefix landed and `busy()` went dark. It no longer
    // does — a canceled run never replaces a landed evaluation, so
    // `busy()` correctly goes on reporting that the picture is older
    // than the document — and the wait that means "the worker stopped"
    // is `running()`.
    let start = std::time::Instant::now();
    while session.running() {
        session.pump();
        if start.elapsed().as_secs() > 120 {
            panic!("the cancel was not honored within 120 s");
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    println!(
        "   canceled: the worker stopped {:?} after the cancel",
        start.elapsed()
    );
    assert!(
        session.busy(),
        "and the session still owes an answer — the state the chrome shows as \
         'canceled, showing an older result'"
    );
    let unevaluated = session
        .tree_rows()
        .iter()
        .filter(|row| row.status == RowStatus::Unevaluated)
        .count();
    println!(
        "   {} of {} rows unevaluated after the cancel (this document had no landed \
         evaluation to keep, so the tree is honest about having nothing)",
        unevaluated,
        session.tree_rows().len()
    );

    // The recovery op, which did not exist when this review ran: ask
    // again, without editing anything.
    session.perform(SessionOp::Reevaluate);
    let start = std::time::Instant::now();
    while session.busy() {
        session.pump();
        if start.elapsed().as_secs() > 300 {
            panic!("the re-evaluation did not land within 300 s");
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    println!("   Reevaluate landed a full run in {:?}", start.elapsed());

    // The seam recovers: an edit resubmits and the run completes.
    let params = viewer::props::param_rows(session.doc());
    if let Some(param) = params.first() {
        println!("   nudging parameter {} to resubmit", param.name.0);
        session.perform(SessionOp::SetParam {
            name: param.name.clone(),
            value: param.value,
        });
    } else {
        // No document parameter: re-request through undo-at-root's
        // refusal path would not resubmit, so re-open instead.
        session.perform(SessionOp::Open(dir.join("diefillet.pncad")));
    }
    pump_until_idle(&mut session, "diefillet full evaluation");
    let rows = session.tree_rows();
    let faults = tree::has_faults(&rows);
    println!("   diefillet: {} rows, faults: {faults}", rows.len());
    assert!(!faults, "diefillet evaluates clean after the restart");

    // -- 3. heatsink: a slider gesture through the ops vocabulary. --
    banner("heatsink.pncad: gesture preview/commit, one undo step");
    let history = docio::open(&dir.join("heatsink.pncad"), tol).expect("heatsink opens");
    let mut session = DocSession::new(
        history.doc().clone(),
        tol,
        Box::new(ThreadEvaluator::spawn().expect("the worker starts")),
    );
    pump_until_idle(&mut session, "heatsink first evaluation");
    let rows = session.tree_rows();
    assert!(!tree::has_faults(&rows));
    let gesture_target = rows
        .iter()
        .find_map(|row| {
            viewer::props::slot_rows(session.doc(), row.id)
                .into_iter()
                .find(|slot| !slot.driver.is_driven() && !slot.structural)
                .map(|slot| (row.id, slot))
        })
        .expect("some node carries an editable literal slot");
    let (node, slot) = (gesture_target.0, gesture_target.1.clone());
    let base = match slot.value.as_ref().expect("it evaluates") {
        SlotValue::Continuous(v) => *v,
        SlotValue::Count(_) => panic!("expected continuous"),
    };
    let states_before = session.history().len();
    session.perform(SessionOp::BeginGesture {
        node,
        slot: slot.slot,
    });
    let mut previews = 0;
    for step in 1..=5 {
        let outcome = session.perform(SessionOp::PreviewGesture {
            value: base * (1.0 + 0.02 * f64::from(step)),
        });
        previews += outcome.previewed.len();
        assert!(outcome.committed.is_empty());
    }
    let outcome = session.perform(SessionOp::CommitGesture);
    assert_eq!(outcome.committed.len(), 1);
    assert_eq!(session.history().len(), states_before + 1);
    println!("   {previews} previews, 1 commit, 1 undo step");
    pump_until_idle(&mut session, "post-gesture evaluation");

    // -- 4. checks.pncad: open/save round trip through the session. --
    banner("checks.pncad: open, evaluate, save, byte-compare");
    let history = docio::open(&dir.join("checks.pncad"), tol).expect("checks opens");
    let mut session = DocSession::inline(history.doc().clone(), tol);
    session.pump();
    assert!(!tree::has_faults(&session.tree_rows()));
    let out = std::env::temp_dir().join("r1-e2e-checks.pncad");
    assert!(
        session
            .perform(SessionOp::Save(out.clone()))
            .refusal
            .is_none()
    );
    let original = std::fs::read_to_string(dir.join("checks.pncad")).expect("readable");
    let saved = std::fs::read_to_string(&out).expect("readable");
    assert_eq!(original, saved, "open -> save reproduces the gallery bytes");
    println!("   byte-identical round trip ({} bytes)", saved.len());

    // -- 5. The assembly workspace: InstantiatePart without a resolver
    // must be a typed Failed badge, not a crash. --------------------
    banner("assembly/*.pncad: typed InstantiatePart refusals as Failed badges");
    let mut instantiate_failures = 0usize;
    let mut clean_docs = 0usize;
    for entry in std::fs::read_dir(dir.join("assembly")).expect("the workspace lists") {
        let path = entry.expect("entry").path();
        if path.extension().is_none_or(|x| x != "pncad") {
            continue;
        }
        let history = match docio::open(&path, tol) {
            Ok(h) => h,
            Err(error) => panic!("{} refused to open: {error:?}", path.display()),
        };
        let mut session = DocSession::inline(history.doc().clone(), tol);
        session.pump();
        let rows = session.tree_rows();
        let file = path
            .file_name()
            .expect("named")
            .to_string_lossy()
            .into_owned();
        let mut this_doc_failed = false;
        for row in &rows {
            if row.kind == "InstantiatePart" {
                match &row.status {
                    RowStatus::Failed { message } => {
                        instantiate_failures += 1;
                        this_doc_failed = true;
                        println!("   {file}: InstantiatePart FAILED [{message}]");
                    }
                    other => panic!(
                        "{file}: InstantiatePart should fail typed without a resolver, got {other:?}"
                    ),
                }
            }
        }
        if !this_doc_failed {
            assert!(
                !tree::has_faults(&rows),
                "{file}: a part document should evaluate clean"
            );
            clean_docs += 1;
            println!("   {file}: {} rows, clean", rows.len());
        }
    }
    assert!(
        instantiate_failures > 0,
        "the workspace holds at least one assembly with instances"
    );
    println!(
        "   {instantiate_failures} typed InstantiatePart refusal(s); {clean_docs} part document(s) clean"
    );

    // -- 6. The seam trait object is still honest about busy. -------
    banner("threaded seam: busy count over two submits");
    let mut seam = ThreadEvaluator::spawn().expect("the worker starts");
    let ring = docio::open(&dir.join("ring.pncad"), tol).expect("ring opens");
    for generation in [
        viewer::evalseam::Generation::FIRST,
        viewer::evalseam::Generation::FIRST.next(),
    ] {
        seam.submit(viewer::evalseam::EvalRequest {
            generation,
            doc: ring.doc().clone(),
            tol,
        });
    }
    let start = std::time::Instant::now();
    let mut answers = Vec::new();
    while answers.len() < 2 {
        if let Some(done) = seam.poll() {
            answers.push((done.generation, done.evaluation.outcome));
        }
        if start.elapsed().as_secs() > 120 {
            break;
        }
    }
    println!(
        "   two submits answered as {answers:?}; busy() now {}",
        seam.busy()
    );
    assert!(
        !seam.busy(),
        "the indicator goes dark when the last answer lands"
    );

    println!("\nr1_e2e: every walk completed");
}
