//! Node labels in the viewer (DESIGN.md Band 1, "Node labels"): a
//! rename is one `SetLabel` and one undo, a labelled creation is the
//! insert and its label as one undo, a labelled row leads with its
//! label, and a create form proposes `Kind N` counted among that kind's
//! labels.

#![allow(clippy::expect_used, clippy::panic)]

use crate::common;

use pncad::document::{Doc, DocEdit, Label, Node, ProfileProgram, RecipeNodeId};
use pncad::geom_core::Tol;
use test_utils::refusal::tag;
use viewer::session::{Creation, DocSession, ProfilePlane, SessionOp};
use viewer::tree::{Headline, headline, proposed_label};

fn label(text: &str) -> Label {
    Label::new(text).expect("a valid label")
}

/// A frame, a square on it and its extrude: the document and the
/// extrude.
fn extruded(seed: &str, tol: Tol) -> (Doc<ProfileProgram>, RecipeNodeId) {
    let doc: Doc<ProfileProgram> = Doc::empty_derived(seed, tol);
    let (doc, profile) = common::framed_square(&doc, 0.02, tol);
    common::inserted(
        &doc,
        Node::Extrude {
            profile,
            distance: common::len(0.01),
        },
        tol,
    )
}

fn relabelled(
    doc: &Doc<ProfileProgram>,
    node: RecipeNodeId,
    text: &str,
    tol: Tol,
) -> Doc<ProfileProgram> {
    let (doc, _) = common::edited(
        doc,
        DocEdit::SetLabel {
            node,
            label: Some(label(text)),
        },
        tol,
    );
    doc
}

/// A rename is one edit and one undo; writing the label the node
/// already has is no action and costs no history state; clearing is
/// the same door with `None`.
#[test]
fn a_rename_is_one_undo_and_the_same_label_again_is_no_action() {
    let tol = Tol::witness();
    let (doc, extrude) = extruded("viewer-node-labels-rename", tol);
    let mut session = DocSession::inline(doc, tol);
    let rename = |text: Option<&str>| SessionOp::SetLabel {
        node: extrude,
        label: text.map(label),
    };

    let renamed = session.perform(rename(Some("base plate")));
    assert!(renamed.refusal.is_none(), "{:?}", renamed.refusal);
    assert_eq!(renamed.committed.len(), 1, "one edit");
    assert_eq!(session.doc().label(extrude), Some(&label("base plate")));

    let again = session.perform(rename(Some("base plate")));
    assert!(
        again.committed.is_empty() && again.refusal.is_none(),
        "the same label again commits nothing and refuses nothing: {again:?}"
    );

    let undone = session.perform(SessionOp::Undo);
    assert!(undone.refusal.is_none(), "{:?}", undone.refusal);
    assert_eq!(
        session.doc().label(extrude),
        None,
        "one undo takes the rename back"
    );

    session.perform(SessionOp::Redo);
    let cleared = session.perform(rename(None));
    assert_eq!(cleared.committed.len(), 1, "clearing is one edit");
    assert_eq!(session.doc().label(extrude), None);
}

/// A labelled creation commits the insert and then the label, and one
/// undo takes both back.
#[test]
fn a_labelled_creation_is_the_insert_and_its_label_as_one_undo() {
    let tol = Tol::witness();
    let doc: Doc<ProfileProgram> = Doc::empty_derived("viewer-node-labels-create", tol);
    let mut session = DocSession::inline(doc, tol);
    let creation = Creation::of(SessionOp::AddDatum {
        datum: ProfilePlane::world_xy().expect("the world xy frame lowers"),
    })
    .expect("adding a datum creates a node");
    let outcome = session.perform(SessionOp::CreateLabelled {
        creation,
        label: label("floor"),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let [frame] = outcome.minted[..] else {
        panic!("one node minted, got {:?}", outcome.minted);
    };
    assert!(
        matches!(
            &outcome.committed[..],
            [DocEdit::InsertNode { .. }, DocEdit::SetLabel { node, label: Some(l) }]
                if *node == frame && *l == label("floor")
        ),
        "the insert, then its label: {:?}",
        outcome.committed
    );
    assert_eq!(session.doc().label(frame), Some(&label("floor")));

    session.perform(SessionOp::Undo);
    assert!(
        session.doc().node(frame).is_none(),
        "one undo takes the node back"
    );
    assert!(session.doc().labels().is_empty(), "and its label with it");
}

/// Only an operation that creates a node can be labelled.
#[test]
fn only_a_creation_can_be_labelled() {
    let refused = Creation::of(SessionOp::Undo);
    assert!(
        matches!(refused.as_ref().map_err(|op| &**op), Err(SessionOp::Undo)),
        "undo creates nothing, and comes back"
    );
}

/// A labelled row leads with its label and mutes its kind and tag; an
/// unlabelled one reads as the spoken node and its pose.
#[test]
fn a_labelled_rows_headline_is_its_label_with_kind_and_tag_muted() {
    let tol = Tol::witness();
    let (doc, extrude) = extruded("viewer-node-labels-headline", tol);
    let t = tag(extrude.0);
    assert_eq!(
        headline(&doc.spoken(extrude), None),
        Headline {
            lead: format!("Extrude {t}"),
            muted: None,
        }
    );
    let doc = relabelled(&doc, extrude, "base plate", tol);
    assert_eq!(
        headline(&doc.spoken(extrude), Some("unused pose")),
        Headline {
            lead: "base plate".to_owned(),
            muted: Some(format!("Extrude {t}")),
        }
    );
}

/// The proposal counts the labels its kind carries now, skips any
/// `Kind N` already taken by that kind, and ignores other kinds'.
#[test]
fn a_create_form_proposes_kind_n_counted_among_that_kinds_labels() {
    let tol = Tol::witness();
    let (doc, first) = extruded("viewer-node-labels-proposal", tol);
    let proposal = |doc: &Doc<ProfileProgram>| {
        proposed_label(doc, "Extrude").map(|label| label.as_str().to_owned())
    };
    assert_eq!(
        proposal(&doc).as_deref(),
        Some("Extrude 1"),
        "none labelled"
    );

    let (doc, profile) = common::framed_square(&doc, 0.01, tol);
    let (doc, second) = common::inserted(
        &doc,
        Node::Extrude {
            profile,
            distance: common::len(0.02),
        },
        tol,
    );
    let doc = relabelled(&doc, first, "base", tol);
    assert_eq!(proposal(&doc).as_deref(), Some("Extrude 2"), "one labelled");
    let doc = relabelled(&doc, second, "Extrude 3", tol);
    assert_eq!(
        proposal(&doc).as_deref(),
        Some("Extrude 4"),
        "two labelled, and `Extrude 3` taken"
    );
    let doc = relabelled(&doc, profile, "Extrude 4", tol);
    assert_eq!(
        proposal(&doc).as_deref(),
        Some("Extrude 4"),
        "a profile's label is not an extrude's"
    );
    assert_eq!(
        proposed_label(&doc, "Ex\ttrude"),
        None,
        "a noun no label can hold proposes nothing"
    );
}
