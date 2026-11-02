//! **The save and the fork through the session's own doors**, measured
//! from the files on disk and the history's every state.

#![allow(clippy::expect_used, clippy::panic)]

use crate::common;
use crate::common::asm;
use pncad::document::{Doc, DocumentId};
use pncad::geom_core::Tol;
use pncad::workspace::{Workspace, WorkspaceError};
use viewer::docio;
use viewer::history::History;
use viewer::session::{DocSession, Refusal, SessionOp, Step};

fn ids_in(history: &History) -> Vec<DocumentId> {
    let mut out = Vec::new();
    let mut stack = vec![history.root()];
    while let Some(at) = stack.pop() {
        let entry = history.entry(at);
        out.push(entry.doc().id());
        stack.extend(entry.children().iter().copied());
    }
    assert_eq!(out.len(), history.len(), "the walk reaches every state");
    out
}

fn file_id(path: &std::path::Path, tol: Tol) -> DocumentId {
    docio::open(path, tol).expect("the file opens").doc().id()
}

/// **The walk the review was asked for**: new document, edit, Save
/// (the dialog's answer), Save As beside, the refusal's fork, undo,
/// save again, open the original, open the fork.
#[test]
fn new_save_beside_fork_undo_resave_and_reopen_both() {
    let tol = Tol::witness();
    let mut session = DocSession::inline(Doc::empty_derived("save-fork-start", tol), tol);
    assert!(
        session
            .perform(SessionOp::NewDocument {
                name: "bracket".to_owned()
            })
            .refusal
            .is_none()
    );
    let original_id = session.committed_doc().id();
    common::box_in(&mut session, [0.04, 0.02, 0.01]);
    let dir = common::tempdir("save-fork-walk");
    let original = dir.join("bracket.pncad");
    let copy = dir.join("bracket-copy.pncad");

    assert!(
        session
            .perform(SessionOp::Save(original.clone()))
            .refusal
            .is_none()
    );
    let original_bytes = std::fs::read_to_string(&original).expect("reads");

    let refusal = session
        .perform(SessionOp::Save(copy.clone()))
        .refusal
        .expect("beside the original, the save refuses");
    let choice = refusal.save_choice().expect("and offers its choice");
    assert!(!copy.exists());

    assert!(session.perform(choice.fork()).refusal.is_none());
    let fork_id = session.committed_doc().id();
    assert_ne!(fork_id, original_id);
    assert!(
        ids_in(session.history()).iter().all(|id| *id == fork_id),
        "every state re-identified: {:?}",
        ids_in(session.history())
    );
    assert_eq!(
        std::fs::read_to_string(&original).expect("reads"),
        original_bytes
    );
    assert_eq!(file_id(&copy, tol), fork_id);

    // Undo to the root and redo to the tip: never the original's id.
    let mut steps = 0;
    while session.perform(SessionOp::Undo).refusal.is_none() {
        steps += 1;
        assert_eq!(session.committed_doc().id(), fork_id);
    }
    assert!(steps > 0, "the session's undo survives the fork");
    assert!(
        session.committed_doc().is_empty(),
        "the root is the empty new document"
    );
    assert_eq!(session.committed_doc().id(), fork_id, "even the root");
    for _ in 0..steps {
        assert!(session.perform(SessionOp::Redo).refusal.is_none());
    }

    // An undo, then a plain Save: a resave of the fork in place.
    assert!(session.perform(SessionOp::Undo).refusal.is_none());
    let resaved = session.perform(SessionOp::Save(
        session.path().expect("the fork's path").to_path_buf(),
    ));
    assert!(resaved.refusal.is_none(), "{:?}", resaved.refusal);
    assert_eq!(file_id(&copy, tol), fork_id);
    assert_eq!(
        std::fs::read_to_string(&original).expect("reads"),
        original_bytes
    );
    assert_eq!(
        Workspace::open(&dir)
            .expect("the folder scans")
            .documents()
            .len(),
        2
    );

    for (path, id) in [(&original, original_id), (&copy, fork_id)] {
        assert!(
            session
                .perform(SessionOp::Open(path.clone()))
                .refusal
                .is_none()
        );
        assert_eq!(session.committed_doc().id(), id);
        assert!(matches!(
            session.perform(SessionOp::Undo).refusal,
            Some(Refusal::NothingToDo {
                direction: Step::Undo
            })
        ));
    }
    std::fs::remove_dir_all(&dir).expect("removable");
}

/// **A fork of a fork**, and a branch the cursor is not on: every
/// state of the history, the abandoned branch's included, carries the
/// latest id.
#[test]
fn a_fork_of_a_fork_reidentifies_the_abandoned_branch_too() {
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    let var = common::thickness_var(session.committed_doc());
    for value in [0.02, 0.03] {
        assert!(
            session
                .perform(SessionOp::SetVariable {
                    var,
                    value: viewer::props::SlotValue::Continuous(value),
                })
                .refusal
                .is_none()
        );
    }
    assert!(session.perform(SessionOp::Undo).refusal.is_none());
    assert!(
        session
            .perform(SessionOp::SetVariable {
                var,
                value: viewer::props::SlotValue::Continuous(0.05),
            })
            .refusal
            .is_none()
    );
    let dir = common::tempdir("save-fork-fork2");
    let a = dir.join("a.pncad");
    assert!(
        session
            .perform(SessionOp::SaveAsNewDocument(a.clone()))
            .refusal
            .is_none()
    );
    let first = session.committed_doc().id();
    let b = dir.join("b.pncad");
    assert!(
        session
            .perform(SessionOp::SaveAsNewDocument(b.clone()))
            .refusal
            .is_none()
    );
    let second = session.committed_doc().id();
    assert_ne!(first, second);
    assert!(ids_in(session.history()).iter().all(|id| *id == second));
    assert_eq!(file_id(&a, tol), first, "the first fork's file stays");
    assert_eq!(file_id(&b, tol), second);
    std::fs::remove_dir_all(&dir).expect("removable");
}

/// **The window after a fork**: the landed run still answers the old
/// identity until the next landing.
#[test]
fn just_after_a_fork_the_landed_run_is_the_old_identity_until_it_lands() {
    let tol = Tol::witness();
    let bench = asm::bench("save-fork-window", tol);
    let mut session = asm::open_bench(&bench, tol);
    asm::unpin(&mut session, bench.post_b);
    session.pump();
    let before = session.committed_doc().id();
    let fork = bench.dir.join("fork.pncad");
    assert!(
        session
            .perform(SessionOp::SaveAsNewDocument(fork))
            .refusal
            .is_none()
    );
    let (landed, _) = session.landed_pair().expect("a run is on screen");
    assert_eq!(
        landed.id(),
        before,
        "the window: the landed run is the old id"
    );
    assert!(session.busy(), "and the session reads busy");
    let early = session.perform(SessionOp::BeginFreeMove { node: bench.post_b });
    assert!(
        matches!(
            early.refusal,
            Some(Refusal::Display(viewer::display::DisplayFault::Admission(
                viewer::display::AdmissionFault::NotEvaluated { .. }
            )))
        ),
        "in the window a free move waits for the landing, as after any edit: {:?}",
        early.refusal
    );
    session.pump();
    assert_eq!(
        session.landed_pair().expect("landed").0.id(),
        session.committed_doc().id()
    );
    let later = session.perform(SessionOp::BeginFreeMove { node: bench.post_b });
    assert!(later.refusal.is_none(), "{:?}", later.refusal);
    std::fs::remove_dir_all(&bench.dir).expect("removable");
}

/// **A fork during a free move**: permitted, and the move commits onto
/// the forked document.
#[test]
fn a_fork_during_a_free_move_keeps_the_move_and_the_new_identity() {
    let tol = Tol::witness();
    let bench = asm::bench("save-fork-freemove", tol);
    let mut session = asm::open_bench(&bench, tol);
    asm::unpin(&mut session, bench.post_b);
    session.pump();
    assert!(
        session
            .perform(SessionOp::BeginFreeMove { node: bench.post_b })
            .refusal
            .is_none()
    );
    let fork = bench.dir.join("fork.pncad");
    let forked = session.perform(SessionOp::SaveAsNewDocument(fork));
    assert!(forked.refusal.is_none(), "{:?}", forked.refusal);
    let id = session.committed_doc().id();
    let committed = session.perform(SessionOp::CommitFreeMove { node: bench.post_b });
    assert!(committed.refusal.is_none(), "{:?}", committed.refusal);
    session.pump();
    assert_eq!(session.committed_doc().id(), id);
    assert!(ids_in(session.history()).iter().all(|i| *i == id));
    std::fs::remove_dir_all(&bench.dir).expect("removable");
}

/// **A fork during a value gesture is refused**, and writes nothing.
#[test]
fn a_fork_during_a_value_gesture_refuses_and_writes_nothing() {
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    let var = common::thickness_var(session.committed_doc());
    let begun = session.perform(SessionOp::BeginVariableGesture { var });
    assert!(begun.refusal.is_none(), "{:?}", begun.refusal);
    let dir = common::tempdir("save-fork-gesture");
    let fork = dir.join("fork.pncad");
    let before = session.committed_doc().id();
    let refused = session.perform(SessionOp::SaveAsNewDocument(fork.clone()));
    assert!(
        matches!(refused.refusal, Some(Refusal::GestureInFlight)),
        "the fork refuses mid-drag: {:?}",
        refused.refusal
    );
    assert!(!fork.exists());
    assert_eq!(session.committed_doc().id(), before);
    std::fs::remove_dir_all(&dir).expect("removable");
}

/// **A fork aimed at the original's own file**, through the op (the
/// toolbar never aims it there), refuses and leaves the original's
/// document in its folder.
#[test]
fn a_fork_aimed_at_the_originals_own_file_keeps_the_original() {
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    let dir = common::tempdir("save-fork-onto-self");
    let file = dir.join("plate.pncad");
    assert!(
        session
            .perform(SessionOp::Save(file.clone()))
            .refusal
            .is_none()
    );
    let original_id = session.committed_doc().id();
    let refused = session.perform(SessionOp::SaveAsNewDocument(file.clone()));
    assert!(
        matches!(
            refused.refusal,
            Some(Refusal::Workspace(ref error))
                if matches!(**error, WorkspaceError::SaveWouldReplaceDocument { id, .. } if id == original_id)
        ),
        "the fork would replace the original: {:?}",
        refused.refusal
    );
    assert_eq!(
        session.committed_doc().id(),
        original_id,
        "the session keeps its identity"
    );
    assert_eq!(
        file_id(&file, tol),
        original_id,
        "the original survives the fork"
    );
    std::fs::remove_dir_all(&dir).expect("removable");
}

/// **A save at a name holding a third document** refuses that it would
/// replace it, before the duplicate-id check, so the toolbar is offered
/// no choice whose fork the store would refuse, and the third document
/// survives.
#[test]
fn a_save_over_a_file_holding_a_third_document_refuses_and_offers_no_fork() {
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    let dir = common::tempdir("save-fork-third");
    let file = dir.join("plate.pncad");
    assert!(
        session
            .perform(SessionOp::Save(file.clone()))
            .refusal
            .is_none()
    );
    // A third document at other.pncad.
    let third = Doc::empty_derived("save-fork-third-doc", tol);
    let other = dir.join("other.pncad");
    Workspace::open(&dir)
        .expect("scans")
        .save_at(&third, &other, tol)
        .expect("the third document stores");
    let refusal = session
        .perform(SessionOp::Save(other.clone()))
        .refusal
        .expect("refuses: other.pncad holds another document");
    assert!(
        matches!(
            refusal,
            Refusal::Workspace(ref error)
                if matches!(**error, WorkspaceError::SaveWouldReplaceDocument { id, .. } if id == third.id())
        ),
        "{refusal:?}"
    );
    assert!(refusal.save_choice().is_none(), "no choice to offer");
    assert_eq!(
        file_id(&other, tol),
        third.id(),
        "the third document survives"
    );
    std::fs::remove_dir_all(&dir).expect("removable");
}

/// **Two documents created under one name are two parts**: each
/// `NewDocument` mints its own id, so the second, saved at the first's
/// file, refuses that it would replace another document instead of
/// resaving over it.
#[test]
fn a_second_new_document_of_one_name_refuses_to_save_over_the_first() {
    let tol = Tol::witness();
    let new_bracket = || {
        let mut session = DocSession::inline(Doc::empty_derived("save-fork-name-start", tol), tol);
        let created = session.perform(SessionOp::NewDocument {
            name: "bracket".to_owned(),
        });
        assert!(created.refusal.is_none(), "{:?}", created.refusal);
        session
    };
    let mut first = new_bracket();
    let mut second = new_bracket();
    let first_id = first.committed_doc().id();
    assert_ne!(
        first_id,
        second.committed_doc().id(),
        "one name, two identities"
    );

    let dir = common::tempdir("save-fork-one-name");
    let file = dir.join("bracket.pncad");
    let saved = first.perform(SessionOp::Save(file.clone()));
    assert!(saved.refusal.is_none(), "{:?}", saved.refusal);
    common::box_in(&mut second, [0.04, 0.02, 0.01]);
    let refusal = second
        .perform(SessionOp::Save(file.clone()))
        .refusal
        .expect("refuses: bracket.pncad holds the first document");
    assert!(
        matches!(
            refusal,
            Refusal::Workspace(ref error)
                if matches!(**error, WorkspaceError::SaveWouldReplaceDocument { id, .. } if id == first_id)
        ),
        "{refusal:?}"
    );
    assert_eq!(file_id(&file, tol), first_id, "the first document survives");
    assert!(
        docio::open(&file, tol)
            .expect("the file opens")
            .doc()
            .is_empty(),
        "and its content is the first's, not the second's box"
    );
    std::fs::remove_dir_all(&dir).expect("removable");
}

/// **A file still carrying a non-empty edit log opens** at its replayed
/// state, with no undo.
#[test]
fn a_file_with_an_edit_log_opens_at_its_replayed_state() {
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc.clone(), tol);
    let var = common::thickness_var(session.committed_doc());
    let edited = session.perform(SessionOp::SetVariable {
        var,
        value: viewer::props::SlotValue::Continuous(0.02),
    });
    assert!(edited.refusal.is_none());
    let edits: Vec<_> = session
        .history()
        .path()
        .into_iter()
        .flat_map(|id| session.history().entry(id).edits().to_vec())
        .collect();
    assert!(!edits.is_empty());
    let dir = common::tempdir("save-fork-oldlog");
    let file = dir.join("old.pncad");
    std::fs::write(
        &file,
        pncad::document::save(&doc, &edits, tol).expect("an old-shaped file saves"),
    )
    .expect("writes");
    let opened = docio::open(&file, tol).expect("opens");
    assert!(
        opened.doc().bit_eq(session.committed_doc()),
        "the replayed state"
    );
    assert!(!opened.can_undo());
    // And saving it back drops the log.
    let mut reopened = DocSession::inline(Doc::empty_derived("x", tol), tol);
    assert!(
        reopened
            .perform(SessionOp::Open(file.clone()))
            .refusal
            .is_none()
    );
    assert!(
        reopened
            .perform(SessionOp::Save(file.clone()))
            .refusal
            .is_none()
    );
    let text = std::fs::read_to_string(&file).expect("reads");
    assert!(
        pncad::document::load(&text, tol)
            .expect("loads")
            .edits
            .is_empty()
    );
    std::fs::remove_dir_all(&dir).expect("removable");
}

/// **A relative path with a folder** goes through `file_dir` and the
/// store's `root` comparison as spelled.
#[test]
fn a_save_at_a_relative_path_with_a_folder() {
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    let dir = common::tempdir("save-fork-rel");
    let cwd = std::env::current_dir().expect("cwd");
    let rel = pathdiff(&dir, &cwd);
    let out = session.perform(SessionOp::Save(rel.join("plate.pncad")));
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    assert!(dir.join("plate.pncad").exists());
    std::fs::remove_dir_all(&dir).expect("removable");
}

fn pathdiff(target: &std::path::Path, base: &std::path::Path) -> std::path::PathBuf {
    let mut up = std::path::PathBuf::new();
    for _ in base.components().skip(1) {
        up.push("..");
    }
    up.join(target.strip_prefix("/").expect("absolute"))
}
