//! **A literal frame compares by bits** (D7): an explicit rule's listed
//! frames are part of the node's bits.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::AuthoredNode;

use crate::fixture::{insert, len, on_frame};
use editor_core::{Frame, Node, ProfileDoc, RecipeNodeId, load, save};
use geom_core::Tol;

/// A unit cube on its own xy, and its extrude's id.
fn cube(label: &str) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived(label, Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    )
}

/// **A literal frame compares by bits** (D7): an explicit rule's listed
/// frames differing only in a signed zero are two nodes to
/// `Node::bit_eq`, and a saved literal reads back to its own bits.
#[test]
fn a_literal_frame_compares_by_bits() {
    let signed = Frame::translation([-0.0, 0.0, 0.25]);
    let plain = Frame::translation([0.0, 0.0, 0.25]);
    let body = RecipeNodeId::new(0, 3);
    let group = |f: &Frame| -> AuthoredNode { Node::placed_union_at(body, vec![*f]) };
    assert!(group(&signed).bit_eq(&group(&signed)));
    assert!(
        !group(&signed).bit_eq(&group(&plain)),
        "an explicit rule's -0.0 is not its 0.0"
    );

    let (doc, input) = cube("placement-bits");
    let (doc, _) = insert(doc, Node::placed_union_at(input, vec![signed]));
    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    let back = load(&text, Tol::witness()).expect("its own bytes load").doc;
    assert!(back.bit_eq(&doc), "the literal round-trips to its own bits");
}
