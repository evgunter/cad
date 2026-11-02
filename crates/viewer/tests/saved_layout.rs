//! **Display frames are saved with the document** (IDENTITY DI5;
//! `crates/viewer/src/layout.rs`): a save keeps the committed frames —
//! an "Add part…" insert's and a free-move's — in a layout file beside
//! the document, outside its bytes, and an open restores them over the
//! document it opened, dropping and saying any frame whose space that
//! document does not show.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use std::path::{Path, PathBuf};

use crate::common;
use common::asm;
use pncad::document::{Doc, DocumentId, Frame, RecipeNodeId, VarId, content_pin, save};
use pncad::geom_core::Tol;
use viewer::display::Held;
use viewer::frame::outcome_notices;
use viewer::layout::{self, Layout, LayoutFault, LayoutReport, SavedFrame};
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

/// **Two inserted parts and a free-move reopen where they were drawn,
/// and the document's bytes and pin are the layout-free ones.**
///
/// Two "Add part…" of one part, the second drawn clear of the first by
/// its insert's frame, then the first's space free-moved; saved; and
/// opened in a fresh session. The reopened session draws both frames
/// as saved. The document file holds exactly the bytes `save` writes
/// for the document with no layout, its content pin is the session's,
/// and the layout is a file of its own beside it.
///
/// *Breaks if* a save keeps no frame (the reopened parts are drawn on
/// each other, at the identity), an open restores none or the wrong
/// one, or a frame reaches the document's bytes or its pin.
#[test]
fn two_inserted_parts_and_a_free_move_reopen_drawn_apart() {
    let tol = Tol::witness();
    let bench = asm::bench("saved-layout-apart", tol);
    let (mut session, path) = authored(&bench, "apart", tol);
    let first = common::instance_in(&mut session, bench.post.id);
    let second = common::instance_in(&mut session, bench.post.id);
    let doc = session.committed_doc().clone();
    let [one, two] = [first, second].map(|i| doc.node_space(i).expect("a space"));
    let inserted = *session
        .display()
        .free_move_of(two)
        .expect("the premise: the second part is drawn clear of the first");
    let moved = Frame::translation([0.0, 0.0, 0.05]);
    free_move(&mut session, first, moved);

    let saved = session.perform(SessionOp::Save(path.clone()));
    assert!(saved.refusal.is_none(), "{:?}", saved.refusal);
    assert_eq!(
        saved.layout,
        LayoutReport::default(),
        "the layout is written"
    );
    assert!(
        layout::path_of(&path).is_file(),
        "the layout is a file beside the document"
    );
    let bytes = std::fs::read_to_string(&path).expect("the document reads");
    assert_eq!(
        bytes,
        save(&doc, &[], tol).expect("the document saves"),
        "the document file holds the document's own bytes, no frame in them"
    );
    let pin = content_pin(&doc, tol).expect("the document pins");

    let (reopened, opened) = open_fresh(&path, tol);
    assert_eq!(opened.layout, LayoutReport::default(), "nothing dropped");
    assert_eq!(
        content_pin(reopened.committed_doc(), tol).expect("the reopened document pins"),
        pin,
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
    let mut reopened = reopened;
    let forked = reopened.perform(SessionOp::SaveAsNewDocument(fork.clone()));
    assert!(forked.refusal.is_none(), "{:?}", forked.refusal);
    assert_eq!(forked.layout, LayoutReport::default());
    let (fork_session, fork_opened) = open_fresh(&fork, tol);
    assert_eq!(
        fork_opened.layout,
        LayoutReport::default(),
        "the fork's layout is written under the fork's id"
    );
    assert_eq!(
        fork_session.display().saved_frames(),
        reopened.display().saved_frames(),
        "the fork reopens drawn as the original"
    );
    let view = reopened.display_view();
    assert_eq!(view.moved_nodes.get(&first), Some(&moved), "drawn moved");
    assert_eq!(
        view.moved_nodes.get(&second),
        Some(&inserted),
        "drawn apart"
    );
}

/// **A document with no layout opens at the identity, and a save with
/// no frame leaves none.**
///
/// The bench's assembly is written through the store's API, so it has
/// no layout: it opens with no frame and nothing said. A session that
/// saved a frame and then lost it (an undo takes an insert's frame
/// back) saves again with no frame, which clears the layout it left,
/// so the reopen is at the identity too.
///
/// *Breaks if* an absent layout is said as a fault, a layout is
/// invented, or a save with no frame leaves the earlier save's layout
/// to draw the document by frames its session no longer holds.
#[test]
fn a_document_saved_without_a_layout_opens_at_the_identity() {
    let tol = Tol::witness();
    let bench = asm::bench("saved-layout-none", tol);
    let (api, opened) = open_fresh(&bench.asm_path, tol);
    assert_eq!(opened.layout, LayoutReport::default(), "nothing said");
    assert!(
        api.display().saved_frames().is_empty(),
        "an API-written document has no frame: {:?}",
        api.display().saved_frames()
    );

    let (mut session, path) = authored(&bench, "cleared", tol);
    common::instance_in(&mut session, bench.post.id);
    let second = common::instance_in(&mut session, bench.post.id);
    let two = session.committed_doc().node_space(second).expect("a space");
    assert!(session.display().free_move_of(two).is_some(), "the premise");
    assert!(
        session
            .perform(SessionOp::Save(path.clone()))
            .refusal
            .is_none()
    );
    assert!(layout::path_of(&path).is_file(), "the premise: a layout");
    assert!(session.perform(SessionOp::Undo).refusal.is_none());
    session.pump();
    assert!(session.display().saved_frames().is_empty(), "the premise");
    let saved = session.perform(SessionOp::Save(path.clone()));
    assert_eq!(saved.layout, LayoutReport::default());
    assert!(
        !layout::path_of(&path).exists(),
        "a save with no frame clears the layout"
    );
    let (reopened, _) = open_fresh(&path, tol);
    assert!(reopened.display().saved_frames().is_empty());
}

/// **A saved frame whose space the opened document does not hold is
/// dropped and said**, and the frames that hold are restored.
///
/// A layout saved with two parts' frames is put back beside the same
/// document after the second part was undone and the document saved
/// again: its space is gone. The open restores the first frame (a
/// free-move), drops the second naming the node and the gone space,
/// and the chrome's notices say it.
///
/// *Breaks if* the gone space's frame is applied (to anything), is
/// dropped in silence, or takes the frame that holds with it.
#[test]
fn a_frame_for_a_missing_space_is_dropped_and_reported() {
    let tol = Tol::witness();
    let bench = asm::bench("saved-layout-missing", tol);
    let (mut session, path) = authored(&bench, "missing", tol);
    let first = common::instance_in(&mut session, bench.post.id);
    let second = common::instance_in(&mut session, bench.post.id);
    let doc = session.committed_doc().clone();
    let [one, two] = [first, second].map(|i| doc.node_space(i).expect("a space"));
    let moved = Frame::translation([0.0, 0.0, 0.05]);
    free_move(&mut session, first, moved);
    assert!(
        session
            .perform(SessionOp::Save(path.clone()))
            .refusal
            .is_none()
    );
    let both = std::fs::read_to_string(layout::path_of(&path)).expect("the layout reads");

    assert!(session.perform(SessionOp::Undo).refusal.is_none());
    session.pump();
    assert!(session.committed_doc().var(two).is_none(), "the premise");
    assert!(
        session
            .perform(SessionOp::Save(path.clone()))
            .refusal
            .is_none()
    );
    std::fs::write(layout::path_of(&path), both).expect("the stale layout is put back");

    let (reopened, opened) = open_fresh(&path, tol);
    assert_eq!(
        reopened.display().free_move_of(one).copied(),
        Some(moved),
        "the frame that holds is restored"
    );
    assert!(
        reopened.display().free_move_of(two).is_none(),
        "the gone space's frame is not"
    );
    assert_eq!(opened.layout.fault, None);
    let [dropped] = opened.layout.dropped.as_slice() else {
        panic!("one frame dropped: {:?}", opened.layout.dropped);
    };
    assert_eq!(dropped.held, Held::Node(second), "named by its insert");
    assert!(
        matches!(
            dropped.cause,
            viewer::display::AdmissionFault::NoSuchSpace { .. }
        ),
        "dropped because its space is gone: {:?}",
        dropped.cause
    );
    let said: Vec<String> = outcome_notices(&opened)
        .map(|message| message.text().to_string())
        .collect();
    assert!(
        said.iter()
            .any(|text| text.starts_with("layout: a saved placement was not restored")),
        "the drop is said: {said:?}"
    );
}

/// **A layout of another document, or one this viewer does not write,
/// restores nothing and is said; the document opens all the same.**
///
/// *Breaks if* a layout is applied to a document it was not written
/// for, a malformed layout refuses the open or is half applied, or
/// either is dropped in silence.
#[test]
fn a_foreign_or_malformed_layout_restores_nothing_and_is_said() {
    let tol = Tol::witness();
    let bench = asm::bench("saved-layout-foreign", tol);
    let (mut session, path) = authored(&bench, "foreign", tol);
    common::instance_in(&mut session, bench.post.id);
    let second = common::instance_in(&mut session, bench.post.id);
    assert!(
        session
            .perform(SessionOp::Save(path.clone()))
            .refusal
            .is_none()
    );
    let text = std::fs::read_to_string(layout::path_of(&path)).expect("the layout reads");
    let mut saved = Layout::parse(&text, tol).expect("the session's own layout parses");
    assert_eq!(saved.to_text(), text, "the layout round-trips");
    let mut awkward = saved.clone();
    let turn = std::f64::consts::FRAC_1_SQRT_2;
    awkward.frames[0].frame = Frame {
        columns: [[turn, turn, 0.0], [-turn, turn, 0.0], [0.0, 0.0, 1.0]],
        translation: [0.1 + 0.2, -0.0, 1e-300],
    };
    let back = Layout::parse(&awkward.to_text(), tol).expect("an awkward frame parses");
    assert!(
        back.frames[0].frame.bit_eq(&awkward.frames[0].frame),
        "every coordinate reads back bit for bit: {:?} vs {:?}",
        back.frames[0].frame,
        awkward.frames[0].frame
    );

    let other = DocumentId::derive("saved-layout-someone-else");
    saved.document = other;
    std::fs::write(layout::path_of(&path), saved.to_text()).expect("the foreign layout is written");
    let (reopened, opened) = open_fresh(&path, tol);
    assert!(
        matches!(
            opened.layout.fault,
            Some(LayoutFault::OtherDocument { layout, .. }) if layout == other
        ),
        "{:?}",
        opened.layout
    );
    assert!(
        reopened.display().saved_frames().is_empty(),
        "nothing restored"
    );
    assert!(
        reopened.committed_doc().node(second).is_some(),
        "the document opened"
    );

    for (why, text) in [
        ("not TOML", "document = ".to_string()),
        ("a stray key", format!("extra = 1\n{text}")),
        ("an integer coordinate", text.replacen("1.0", "1", 1)),
        ("a frame on the world", world_frame(&saved, tol)),
        ("a mirror", mirrored(&saved, tol)),
    ] {
        std::fs::write(layout::path_of(&path), &text).expect("the layout is written");
        let (reopened, opened) = open_fresh(&path, tol);
        assert!(
            matches!(opened.layout.fault, Some(LayoutFault::Malformed { .. })),
            "{why}: {:?}",
            opened.layout
        );
        assert!(
            reopened.display().saved_frames().is_empty(),
            "{why}: nothing restored"
        );
        assert!(
            outcome_notices(&opened).any(|message| message.text().starts_with("layout: ")),
            "{why}: said"
        );
    }
}

/// `layout`'s text with its first frame moved onto the world.
fn world_frame(layout: &Layout, tol: Tol) -> String {
    let mut layout = layout.clone();
    layout.frames[0].space = VarId::WORLD;
    let text = layout.to_text();
    assert!(Layout::parse(&text, tol).is_err(), "the premise");
    text
}

/// `layout`'s text with its first frame a mirror, which no pose is.
fn mirrored(layout: &Layout, tol: Tol) -> String {
    let mut layout = layout.clone();
    let SavedFrame { frame, .. } = &mut layout.frames[0];
    frame.columns[0] = [-1.0, 0.0, 0.0];
    let text = layout.to_text();
    assert!(Layout::parse(&text, tol).is_err(), "the premise");
    text
}
