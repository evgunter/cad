//! **Display frames are saved with the document** (IDENTITY DI5;
//! `crates/viewer/src/layout.rs`): a save keeps the committed frames —
//! an "Add part…" insert's and a free-move's — in a layout file beside
//! the document, outside its bytes, each with a witness drawn in its
//! space; an open restores a frame while its space is shown and its
//! witness stands in it, and drops and says every other.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use std::path::{Path, PathBuf};

use crate::common;
use common::asm;
use pncad::document::{Doc, DocumentId, Frame, RecipeNodeId, VarId, content_pin, save};
use pncad::geom_core::Tol;
use viewer::display::Unheld;
use viewer::frame::outcome_notices;
use viewer::layout::{self, Layout, LayoutFault, LayoutReport, Skipped};
use viewer::session::{DocSession, OpOutcome, SessionOp};

/// A session over a new empty document saved into `bench`'s directory,
/// so the parts beside it resolve, and the path it was saved at.
fn authored(bench: &asm::Bench, label: &str, tol: Tol) -> (DocSession, PathBuf) {
    let path = bench.dir.join(format!("{label}.pncad"));
    let mut session = DocSession::inline(Doc::empty_derived(label, tol), tol);
    let saved = session.perform(SessionOp::Save(path.clone()));
    assert!(saved.refusal.is_none(), "{:?}", saved.refusal);
    (session, path)
}

/// `path` opened in a fresh session, landed, and the open's outcome.
fn open_fresh(path: &Path, tol: Tol) -> (DocSession, OpOutcome) {
    let mut session = DocSession::inline(Doc::empty_derived("saved-layout-boot", tol), tol);
    let outcome = session.perform(SessionOp::Open(path.to_path_buf()));
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    (session, outcome)
}

/// Save `session` at `path`, accepted, and answer the outcome.
fn saved(session: &mut DocSession, path: &Path) -> OpOutcome {
    let outcome = session.perform(SessionOp::Save(path.to_path_buf()));
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    outcome
}

/// The layout text beside `path`.
fn layout_text(path: &Path) -> String {
    std::fs::read_to_string(layout::path_of(path)).expect("the layout reads")
}

/// Every op of a free-move of `node`'s space to `frame`, each accepted.
fn free_move(session: &mut DocSession, node: RecipeNodeId, frame: Frame) {
    for op in [
        SessionOp::BeginFreeMove { node },
        SessionOp::PreviewFreeMove { node, frame },
        SessionOp::CommitFreeMove { node },
    ] {
        let outcome = session.perform(op.clone());
        assert!(outcome.refusal.is_none(), "{op:?}: {:?}", outcome.refusal);
    }
}

/// The notices an outcome's layout puts on the line.
fn said(outcome: &OpOutcome) -> Vec<String> {
    outcome_notices(outcome)
        .map(|message| message.text().to_string())
        .filter(|text| text.starts_with("layout: "))
        .collect()
}

/// A session holding two inserted parts, the first's space free-moved:
/// the session, its path, the two spaces and the free-move's frame.
fn two_parts(
    bench: &asm::Bench,
    label: &str,
    tol: Tol,
) -> (DocSession, PathBuf, [VarId; 2], Frame) {
    let (mut session, path) = authored(bench, label, tol);
    let first = common::instance_in(&mut session, bench.post.id);
    let second = common::instance_in(&mut session, bench.post.id);
    let doc = session.committed_doc();
    let spaces = [first, second].map(|i| doc.node_space(i).expect("a space"));
    let moved = Frame::translation([0.0, 0.0, 0.05]);
    free_move(&mut session, first, moved);
    (session, path, spaces, moved)
}

/// **Two inserted parts and a free-move reopen where they were drawn,
/// and the document's bytes and pin are the layout-free ones.**
///
/// *Breaks if* a save keeps no frame (the reopened parts are drawn on
/// each other, at the identity), an open restores none or the wrong
/// one, a frame reaches the document's bytes or its pin, or a fork's
/// layout is not its own.
#[test]
fn two_inserted_parts_and_a_free_move_reopen_drawn_apart() {
    let tol = Tol::witness();
    let bench = asm::bench("saved-layout-apart", tol);
    let (mut session, path, [one, two], moved) = two_parts(&bench, "apart", tol);
    let inserted = *session
        .display()
        .free_move_of(two)
        .expect("the premise: the second part is drawn clear of the first");
    let doc = session.committed_doc().clone();
    assert_eq!(saved(&mut session, &path).layout, LayoutReport::default());
    let bytes = std::fs::read_to_string(&path).expect("the document reads");
    assert_eq!(
        bytes,
        save(&doc, &[], tol).expect("the document saves"),
        "the document file holds the document's own bytes, no frame in them"
    );

    let (mut reopened, opened) = open_fresh(&path, tol);
    assert_eq!(opened.layout, LayoutReport::default(), "nothing dropped");
    assert_eq!(
        content_pin(reopened.committed_doc(), tol).expect("pins"),
        content_pin(&doc, tol).expect("pins"),
        "the reopened document is the same version"
    );
    assert_eq!(
        reopened.display().free_move_of(two).copied(),
        Some(inserted),
        "the second part reopens drawn clear of the first, by its insert's frame"
    );
    assert_eq!(
        reopened.display().free_move_of(one).copied(),
        Some(moved),
        "the free-moved space reopens moved"
    );

    let fork = path.with_file_name("apart-fork.pncad");
    let forked = reopened.perform(SessionOp::SaveAsNewDocument(fork.clone()));
    assert!(forked.refusal.is_none(), "{:?}", forked.refusal);
    assert_eq!(forked.layout, LayoutReport::default());
    let (fork_session, fork_opened) = open_fresh(&fork, tol);
    assert_eq!(
        fork_opened.layout,
        LayoutReport::default(),
        "the fork's own"
    );
    assert_eq!(
        fork_session
            .display()
            .saved_frames(fork_session.committed_doc()),
        reopened.display().saved_frames(reopened.committed_doc()),
        "the fork reopens drawn as the original"
    );
}

/// **A document with no layout opens at the identity; a session never
/// given a frame writes no layout, and one that lost its frames writes
/// none over the old one.**
///
/// *Breaks if* an absent layout is said, a layout is invented, or a
/// save with no frame leaves the earlier save's frames to draw by.
#[test]
fn a_document_saved_without_a_layout_opens_at_the_identity() {
    let tol = Tol::witness();
    let bench = asm::bench("saved-layout-none", tol);
    let (api, opened) = open_fresh(&bench.asm_path, tol);
    assert_eq!(opened.layout, LayoutReport::default(), "nothing said");
    assert!(api.display().saved_frames(api.committed_doc()).is_empty());

    let (mut session, path) = authored(&bench, "cleared", tol);
    assert!(
        !layout::path_of(&path).exists(),
        "a document never given a frame gets no layout"
    );
    common::instance_in(&mut session, bench.post.id);
    common::instance_in(&mut session, bench.post.id);
    saved(&mut session, &path);
    assert!(layout::path_of(&path).is_file(), "the premise: a layout");
    assert!(session.perform(SessionOp::Undo).refusal.is_none());
    session.pump();
    assert_eq!(saved(&mut session, &path).layout, LayoutReport::default());
    let (left, _) = Layout::parse(&layout_text(&path), tol).expect("parses");
    assert!(left.frames.is_empty(), "the old frames are written over");
    let (reopened, _) = open_fresh(&path, tol);
    assert!(
        reopened
            .display()
            .saved_frames(reopened.committed_doc())
            .is_empty()
    );
}

/// **A frame whose space is gone is dropped and said by the space's
/// saved name**, and the frame that holds is restored.
///
/// *Breaks if* the gone space's frame is applied, dropped in silence,
/// said by a bare id, or takes the frame that holds with it.
#[test]
fn a_frame_for_a_missing_space_is_dropped_and_reported() {
    let tol = Tol::witness();
    let bench = asm::bench("saved-layout-missing", tol);
    let (mut session, path, [one, two], moved) = two_parts(&bench, "missing", tol);
    saved(&mut session, &path);
    let both = layout_text(&path);
    let (parsed, _) = Layout::parse(&both, tol).expect("parses");
    let two_said = parsed
        .frames
        .iter()
        .find(|saved| saved.space == two)
        .expect("the second space's frame")
        .space_said
        .clone();
    assert!(session.perform(SessionOp::Undo).refusal.is_none());
    session.pump();
    assert!(session.committed_doc().var(two).is_none(), "the premise");
    saved(&mut session, &path);
    std::fs::write(layout::path_of(&path), both).expect("the stale layout is put back");

    let (reopened, opened) = open_fresh(&path, tol);
    assert_eq!(reopened.display().free_move_of(one).copied(), Some(moved));
    assert!(reopened.display().free_move_of(two).is_none());
    let [dropped] = opened.layout.dropped.as_slice() else {
        panic!("one frame dropped: {:?}", opened.layout);
    };
    assert_eq!((dropped.space, &dropped.why), (two, &Unheld::Gone));
    let notices = said(&opened);
    assert!(
        notices.iter().any(|text| text.contains(&two_said)),
        "said by the name the saving document gave the space, {two_said:?}: {notices:?}"
    );
}

/// **A space id minted again after an undo is not the space a saved
/// frame was drawn for** (the review of PR 6296): a post inserted and
/// free-moved, saved, undone, and a shelf inserted in its place mints
/// the same space id. Its witness, the post, does not stand there, so
/// the stale frame is dropped rather than drawn on the shelf.
///
/// *Breaks if* a restore asks the space id alone.
#[test]
fn a_reused_space_id_does_not_take_the_old_frame() {
    let tol = Tol::witness();
    let bench = asm::bench("saved-layout-reused", tol);
    let (mut session, path) = authored(&bench, "reused", tol);
    let post = common::instance_in(&mut session, bench.post.id);
    let space = session.committed_doc().node_space(post).expect("a space");
    free_move(&mut session, post, Frame::translation([0.0, 0.0, 0.05]));
    saved(&mut session, &path);
    let stale = layout_text(&path);
    assert!(session.perform(SessionOp::Undo).refusal.is_none());
    let shelf = common::instance_in(&mut session, bench.shelf.id);
    assert_eq!(
        session.committed_doc().node_space(shelf),
        Some(space),
        "the premise: the shelf's insert mints the same space id"
    );
    saved(&mut session, &path);
    std::fs::write(layout::path_of(&path), stale).expect("the stale layout is put back");

    let (reopened, opened) = open_fresh(&path, tol);
    assert!(
        reopened.display().free_move_of(space).is_none(),
        "the shelf is drawn where it stands"
    );
    let [dropped] = opened.layout.dropped.as_slice() else {
        panic!("one frame dropped: {:?}", opened.layout);
    };
    assert!(
        matches!(&dropped.why, Unheld::Replaced { .. }),
        "{dropped:?}"
    );
    assert_eq!(said(&opened).len(), 1, "and said");
}

/// **A frame whose writer left its space reopens; a frame whose space
/// is emptied does not.** The shelf's space holds the shelf and post_b;
/// the shelf free-moves it, then rests in the world, so the frame's
/// writer has left the space while post_b still shows it — a save then
/// reopen keeps the frame (DI5: it is the space's). Then post_b is given
/// a space of its own, which empties the shelf's space: the earlier
/// layout, put back, is dropped as not shown.
///
/// *Breaks if* a restore asks for the writer rather than the space (the
/// first half), or restores a frame on a space nothing is drawn in.
#[test]
fn a_frame_keeps_its_space_whoever_wrote_it() {
    let tol = Tol::witness();
    let bench = asm::bench("saved-layout-writer", tol);
    let mut session = asm::open_bench(&bench, tol);
    asm::unpin(&mut session, bench.shelf_i);
    common::commit_mate(
        &mut session,
        asm::seat_op(&bench, bench.post_b, asm::own_space_seat_values()),
    );
    session.pump();
    let space = session
        .committed_doc()
        .node_space(bench.post_b)
        .expect("a space");
    let probe = Frame::translation([0.0, 0.0, 0.07]);
    free_move(&mut session, bench.shelf_i, probe);
    let rested = session.perform(asm::shelf_rest_op(&bench, bench.post_a, 0.0));
    assert!(rested.refusal.is_none(), "{:?}", rested.refusal);
    session.pump();
    assert_ne!(
        session.committed_doc().node_space(bench.shelf_i),
        Some(space),
        "the premise: the writer left the space"
    );
    assert_eq!(session.display().free_move_of(space), Some(&probe));
    saved(&mut session, &bench.asm_path);
    let kept = layout_text(&bench.asm_path);
    let (reopened, opened) = open_fresh(&bench.asm_path, tol);
    assert_eq!(opened.layout, LayoutReport::default());
    assert_eq!(
        reopened.display().free_move_of(space),
        Some(&probe),
        "save then reopen with no edit keeps the picture"
    );

    let own = session.perform(SessionOp::IntoOwnSpace {
        placement: bench.post_b,
    });
    assert!(own.refusal.is_none(), "{:?}", own.refusal);
    session.pump();
    assert!(session.committed_doc().var(space).is_some(), "the premise");
    saved(&mut session, &bench.asm_path);
    std::fs::write(layout::path_of(&bench.asm_path), kept).expect("put back");
    let (reopened, opened) = open_fresh(&bench.asm_path, tol);
    assert!(reopened.display().free_move_of(space).is_none());
    let [dropped] = opened.layout.dropped.as_slice() else {
        panic!("one frame dropped: {:?}", opened.layout);
    };
    assert_eq!(dropped.why, Unheld::NotShown);
}

/// **A layout that cannot be read, or written, is said; the document
/// opens, or is saved, all the same.** The layout's path is made a
/// directory, which no user can read or write as a file.
///
/// *Breaks if* either is silent, refuses the document's own act, or the
/// write's notice says a reopen draws at the identity while the old
/// layout stands.
#[test]
fn a_layout_not_read_or_not_written_is_said() {
    let tol = Tol::witness();
    let bench = asm::bench("saved-layout-io", tol);
    let (mut session, path, [one, _], _) = two_parts(&bench, "io", tol);
    std::fs::create_dir_all(layout::path_of(&path)).expect("the layout path is a directory");

    let saving = saved(&mut session, &path);
    assert!(
        matches!(saving.layout.fault, Some(LayoutFault::Write(_))),
        "{:?}",
        saving.layout
    );
    assert!(
        said(&saving)
            .iter()
            .any(|text| text.contains("left as it was")),
        "the notice says what stays: {:?}",
        said(&saving)
    );

    let (reopened, opened) = open_fresh(&path, tol);
    assert!(
        matches!(opened.layout.fault, Some(LayoutFault::Read(_))),
        "{:?}",
        opened.layout
    );
    assert!(reopened.display().free_move_of(one).is_none());
    assert!(
        reopened.committed_doc().var(one).is_some(),
        "the document opened"
    );
    assert_eq!(said(&opened).len(), 1, "said");
}

/// **The layout is read as the preferences are**: a key this viewer
/// does not know, or a frame it cannot read, is skipped and said while
/// the rest applies; a layout of another document, or one that is not
/// TOML, applies nothing and is said.
///
/// *Breaks if* a stray key or one bad frame takes the whole layout, a
/// bad frame (on the world, a mirror, an integer coordinate, two for
/// one space) is applied, anything is dropped in silence, or a foreign
/// layout is applied.
#[test]
fn a_layout_is_read_by_the_preferences_posture() {
    let tol = Tol::witness();
    let bench = asm::bench("saved-layout-posture", tol);
    let (mut session, path, [one, two], moved) = two_parts(&bench, "posture", tol);
    saved(&mut session, &path);
    let text = layout_text(&path);
    let (layout, skipped) = Layout::parse(&text, tol).expect("the session's own layout parses");
    assert!(skipped.is_empty());
    assert_eq!(layout.to_text(), text, "the layout round-trips");
    let mut awkward = layout.clone();
    let turn = std::f64::consts::FRAC_1_SQRT_2;
    awkward.frames[0].frame = Frame {
        columns: [[turn, turn, 0.0], [-turn, turn, 0.0], [0.0, 0.0, 1.0]],
        translation: [0.1 + 0.2, -0.0, 1e-300],
    };
    let (back, _) = Layout::parse(&awkward.to_text(), tol).expect("parses");
    assert!(
        back.frames[0].frame.bit_eq(&awkward.frames[0].frame),
        "every coordinate reads back bit for bit"
    );

    // One frame per case spoiled; the other still applies.
    let spoil = |edit: &dyn Fn(&mut Layout)| {
        let mut layout = layout.clone();
        edit(&mut layout);
        layout.to_text()
    };
    let second = layout
        .frames
        .iter()
        .position(|saved| saved.space == two)
        .expect("the second space's frame");
    let cases: [(&str, String); 5] = [
        ("a stray key", format!("extra = 1\n{text}")),
        (
            "a frame on the world",
            spoil(&|l| l.frames[second].space = VarId::WORLD),
        ),
        (
            "a mirror",
            spoil(&|l| l.frames[second].frame.columns[0] = [-1.0, 0.0, 0.0]),
        ),
        (
            "an integer coordinate",
            spoil(&|l| l.frames[second].frame.translation[0] = 7.0).replacen("7.0", "7", 1),
        ),
        (
            "two frames for one space",
            spoil(&|l| {
                let copy = l.frames[second].clone();
                l.frames.push(copy);
            }),
        ),
    ];
    for (why, text) in cases {
        std::fs::write(layout::path_of(&path), &text).expect("written");
        let (reopened, opened) = open_fresh(&path, tol);
        assert_eq!(opened.layout.fault, None, "{why}: the layout reads");
        assert!(!opened.layout.skipped.is_empty(), "{why}: skipped and said");
        assert_eq!(
            said(&opened).len(),
            opened.layout.skipped.len(),
            "{why}: each said"
        );
        assert_eq!(
            reopened.display().free_move_of(one).copied(),
            Some(moved),
            "{why}: the rest applies"
        );
        if why == "a stray key" {
            assert!(
                matches!(&opened.layout.skipped[..], [Skipped::UnknownKey(key)] if key == "extra")
            );
            assert!(
                reopened.display().free_move_of(two).is_some(),
                "{why}: all of it applies"
            );
        } else {
            assert!(
                reopened.display().free_move_of(two).is_none(),
                "{why}: the bad frame is not applied"
            );
        }
    }

    let mut foreign = layout.clone();
    foreign.document = DocumentId::derive("saved-layout-someone-else");
    for (why, text) in [
        ("another document's", foreign.to_text()),
        ("not TOML", "document = ".to_string()),
    ] {
        std::fs::write(layout::path_of(&path), &text).expect("written");
        let (reopened, opened) = open_fresh(&path, tol);
        assert!(opened.layout.fault.is_some(), "{why}: {:?}", opened.layout);
        assert!(
            reopened
                .display()
                .saved_frames(reopened.committed_doc())
                .is_empty(),
            "{why}: nothing restored"
        );
        assert_eq!(said(&opened).len(), 1, "{why}: said");
    }
}
