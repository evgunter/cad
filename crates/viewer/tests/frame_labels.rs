//! **A frame's label says WHICH frame it is** — the picker's entries
//! and the feature tree's rows, from one home (`tree::frame_pose`).
//!
//! The defect these rows are filed against is that every frame in the
//! document read `feature 3` in the add-profile combo and `Datum
//! frame` on its own tree row, so two frames a centimetre apart were
//! indistinguishable in both places. The central assertion here is
//! therefore a DIFFERENCE: two frames that differ only in where they
//! are get two different labels.
//!
//! # What the label is allowed to read
//!
//! The NODE, and nothing else. `tree::rows` draws a row for every node
//! in the document — including the unevaluated ones before a run lands
//! and the failed ones whose value does not exist — so a pose read off
//! an evaluation would go blank on exactly the rows a person is
//! diagnosing, and would need this one as its fallback anyway. A
//! component that is not a literal is therefore not evaluated and not
//! guessed: the label says the origin is driven and prints no number,
//! which the driven row below holds.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use pncad::document::{
    Datum, Dimension, Doc, DocEdit, DocParam, Expr, Node, ParamName, ProfileProgram, RecipeNodeId,
};
use pncad::geom_core::Tol;
use pncad::prelude::{EntityKind, MM, StableName};
use viewer::session::ProfilePlane;
use viewer::tree;

/// A `Datum::Frame` node with world axes and the given origin, in
/// canonical metres.
///
/// The axes come from `ProfilePlane::xy_numbers` rather than being
/// typed here: this suite asserts that the world xy frame is LABELLED
/// `xy`, so it has to be drawing the frame the tree actually calls
/// that one.
fn frame_at(origin: [f64; 3]) -> Node<ProfileProgram> {
    let (_, u, v) = ProfilePlane::xy_numbers();
    Node::Datum(Datum::Frame {
        origin: common::len3(origin),
        u: common::scl3(u),
        v: common::scl3(v),
    })
}

/// The label for one node, through the home the picker and the tree
/// both read.
fn label(node: &Node<ProfileProgram>, id: u64) -> String {
    tree::node_label(node, RecipeNodeId(id))
}

/// **The row's own complaint, as an assertion**: two frames a
/// centimetre apart are told apart.
#[test]
fn two_frames_a_centimetre_apart_get_different_labels() {
    let here = label(&frame_at([0.0, 0.0, 0.0]), 3);
    let there = label(&frame_at([0.0, 0.0, 0.01]), 7);
    assert_ne!(
        here, there,
        "the picker's whole job at this row is telling these two apart"
    );
    assert!(here.contains("feature 3"), "{here}");
    assert!(
        here.contains("xy"),
        "the world xy frame is named as one: {here}"
    );
    assert!(
        here.contains("(0, 0, 0)"),
        "and its origin is what separates it: {here}"
    );
    assert!(there.contains("(0, 0, 0.01)"), "{there}");
}

/// **The origin is written in the unit it was AUTHORED in** — the
/// same unit the property panel edits that slot in, so a person
/// reading the combo and the panel reads one number twice, not two.
#[test]
fn a_frames_origin_is_written_in_its_own_notation() {
    let node = Node::Datum(Datum::Frame {
        origin: [
            Expr::literal_with_unit(0.0, Dimension::Length, MM.def())
                .expect("a millimetre literal"),
            Expr::literal_with_unit(0.0, Dimension::Length, MM.def())
                .expect("a millimetre literal"),
            // Ten millimetres, written in millimetres: the canonical
            // value and its notation, which is what a literal is.
            Expr::literal_with_unit(0.010, Dimension::Length, MM.def())
                .expect("a millimetre literal"),
        ],
        u: common::scl3(ProfilePlane::xy_numbers().1),
        v: common::scl3(ProfilePlane::xy_numbers().2),
    });
    let shown = label(&node, 5);
    assert!(
        shown.contains("(0, 0, 10) mm"),
        "ten millimetres up, said the way it was typed: {shown}"
    );
    assert!(
        !shown.contains("0.01"),
        "and not re-written into canonical metres: {shown}"
    );
}

/// **A pose the node cannot state is not guessed.** A driven origin
/// is reported as driven and no number is printed.
#[test]
fn a_driven_origin_is_said_to_be_driven_and_never_evaluated() {
    let node = Node::Datum(Datum::Frame {
        origin: [
            Expr::literal(0.0, Dimension::Length).expect("a literal"),
            Expr::literal(0.0, Dimension::Length).expect("a literal"),
            Expr::param(ParamName::new("height"), Dimension::Length),
        ],
        u: common::scl3(ProfilePlane::xy_numbers().1),
        v: common::scl3(ProfilePlane::xy_numbers().2),
    });
    let shown = label(&node, 2);
    assert!(shown.contains("driven"), "{shown}");
    assert!(
        !shown.contains('('),
        "no coordinates at all, rather than a point with a hole in it: {shown}"
    );
}

/// **An oblique frame says where it is and declines to name a
/// plane** — the label says less rather than something else.
#[test]
fn an_oblique_frame_is_not_called_a_world_plane() {
    let node = Node::Datum(Datum::Frame {
        origin: common::len3([0.0, 0.0, 0.0]),
        u: common::scl3([1.0, 1.0, 0.0]),
        v: common::scl3([0.0, 1.0, 0.0]),
    });
    let shown = label(&node, 1);
    assert!(
        !shown.contains("xy"),
        "a frame that is not the xy plane is not called one: {shown}"
    );
    assert!(shown.contains("(0, 0, 0)"), "{shown}");
}

/// **A frame read off a face says whose face it is** — the honest
/// answer the node holds, and the limit of what it holds.
#[test]
fn a_face_frame_names_the_node_its_face_is_read_off() {
    let node = Node::Datum(Datum::FaceFrame {
        at: RecipeNodeId(4),
        face: StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId(4),
            path: vec![],
        },
        spin: Expr::literal(0.0, Dimension::Angle).expect("a literal"),
    });
    let shown = label(&node, 6);
    assert!(shown.contains("feature 6"), "{shown}");
    assert!(
        shown.contains("on feature 4's face"),
        "the face's carrier is what the node can say: {shown}"
    );
}

/// **The FEATURE TREE carries it too**, from the same home: two
/// frames a centimetre apart are two different rows, not two rows
/// both reading `Datum frame`.
#[test]
fn the_tree_rows_tell_two_frames_apart() {
    let tol = Tol::witness();
    let doc: Doc<ProfileProgram> = Doc::empty_derived("frame-labels", tol);
    let (doc, _) = common::edited(
        &doc,
        DocEdit::InsertNode {
            node: frame_at([0.0, 0.0, 0.0]),
        },
        tol,
    );
    let (doc, _) = common::edited(
        &doc,
        DocEdit::InsertNode {
            node: frame_at([0.0, 0.0, 0.01]),
        },
        tol,
    );
    let poses: Vec<Option<String>> = tree::rows(&doc, None)
        .into_iter()
        .map(|row| row.pose)
        .collect();
    assert_eq!(poses.len(), 2);
    assert!(poses.iter().all(Option::is_some), "{poses:?}");
    assert_ne!(
        poses[0], poses[1],
        "the tree's two frame rows are distinguishable: {poses:?}"
    );
}

/// A node with no such sentence carries none — `pose` is `None`, not
/// an empty string a row would draw a dangling dash after.
#[test]
fn a_node_that_is_not_a_frame_has_no_pose() {
    let tol = Tol::witness();
    let doc: Doc<ProfileProgram> = Doc::empty_derived("frame-labels-other", tol);
    let (doc, _) = common::edited(
        &doc,
        DocEdit::SetDocParam {
            name: ParamName::new("unused"),
            value: DocParam::continuous(Dimension::Length, 0.001),
        },
        tol,
    );
    let (doc, _) = common::edited(
        &doc,
        DocEdit::InsertNode {
            node: Node::Datum(Datum::Point {
                position: common::len3([1.0, 2.0, 3.0]),
            }),
        },
        tol,
    );
    let rows = tree::rows(&doc, None);
    let point = rows.last().expect("the point's row");
    assert_eq!(point.kind, "Datum point");
    assert_eq!(point.pose, None);
    assert_eq!(
        tree::node_label(
            &Node::Datum(Datum::Point {
                position: common::len3([1.0, 2.0, 3.0])
            }),
            RecipeNodeId(1)
        ),
        "feature 1",
        "a node with nothing more to say is named by its number, as it always was"
    );
}
