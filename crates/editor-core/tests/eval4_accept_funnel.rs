//! The accepted edit travels whole through the refactoring doors:
//! [`split`]'s outcome carries the [`editor_core::Maintenance`] its remainder edits
//! and its part edits reported, and [`inline`]'s carries its own —
//! beside the documents and the recorded edits, never instead of them.
//!
//! No edit records a frame (A11 (2)), so a mated pair cut whole or
//! spliced back reports nothing: its world mate and its mate move as the
//! bundles they are. Each row reads the bundles of the documents that
//! came back, so an empty report is checked against what moved rather
//! than read as "nothing moved".

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::Dimension;
use editor_core::Formula;

use std::collections::BTreeSet;

use editor_core::{
    CapEnd, Coord, DocumentId, Frame, MateRef, Node, ProfileDoc, RecipeNodeId, TargetAxis, inline,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{insert, len, offset_of, on_frame, square};
use geom_core::Tol;

/// A one-block part: a unit square extruded 1 tall, and its body.
fn part(label: &str) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    )
}

/// `b`'s face rested on `a`'s with the three freedoms a plane mate
/// leaves each set to zero, so the copy `b` reads is pinned: the mate
/// and its values, by the mate's address.
fn stacked(
    doc: ProfileDoc,
    a: impl crate::fixture::Sited,
    b: impl crate::fixture::Sited,
) -> (ProfileDoc, MateRef) {
    let (doc, joint) = crate::fixture::constrain_with(
        doc,
        fixture::rest(fixture::head(b), fixture::head(a)),
        &[
            (
                Coord::Slide(TargetAxis::X),
                Formula::zero(Dimension::Length),
            ),
            (
                Coord::Slide(TargetAxis::Y),
                Formula::zero(Dimension::Length),
            ),
            (Coord::SPIN, Formula::zero(Dimension::Angle)),
        ],
    );
    (doc, joint)
}

/// Row 2 — a mated pair cut whole moves as selected (A4): the pinned
/// instance keeps its world mate in the part, the mate-placed one its
/// mate, and the instance left behind is pinned where it stands, at the
/// identity; neither side reports anything: no edit records a frame.
#[test]
fn a_whole_pair_cut_moves_as_selected_and_reports_nothing() {
    let (doc, store, [a, b], offset) = placed_pair("eval4-r2");
    let out = fixture::split_world(
        &doc,
        &BTreeSet::from([a, b]),
        DocumentId::derive("eval4-r2-cell"),
        Tol::witness(),
        Some(
            &(std::sync::Arc::new(store.clone()) as std::sync::Arc<dyn editor_core::PartResolver>),
        ),
    )
    .expect("a whole group and its mate cut");
    let (pa, pb) = (out.node_map[&a], out.node_map[&b]);
    assert_eq!(
        fixture::constrained(&out.part),
        vec![pb],
        "the mate re-forms in the part, in the mate-placed instance's bundle"
    );
    assert!(
        offset_of(&out.part, pa) == Some(offset),
        "the pinned instance keeps its pose"
    );
    assert_eq!(
        offset_of(&out.part, pb),
        None,
        "the member stays mate-placed"
    );
    let mut parts = store;
    parts.insert(out.part.clone(), Tol::witness());
    assert!(
        offset_of(&out.remainder, out.instance).is_some(),
        "the instance is pinned to the world"
    );
    assert!(
        fixture::solve(&out.remainder, &with_resolver(parts), Tol::witness())
            .at_identity(out.instance),
        "the instance stands at the identity"
    );
    assert!(
        out.part_maintenance.is_empty(),
        "{:?}",
        out.part_maintenance
    );
    assert!(
        crate::fixture::without_anonymous(&out.remainder_maintenance).is_empty(),
        "{:?}",
        out.remainder_maintenance
    );
}

/// Row 3 — inline of that split lands the content verbatim at the
/// identity, so the pinned instance's pose comes back as it was,
/// reporting nothing.
#[test]
fn inline_of_a_whole_pair_split_restores_the_pinned_pose_and_reports_nothing() {
    let (doc, store, [a, b], offset) = placed_pair("eval4-r3");
    let out = fixture::split_world(
        &doc,
        &BTreeSet::from([a, b]),
        DocumentId::derive("eval4-r3-cell"),
        Tol::witness(),
        Some(
            &(std::sync::Arc::new(store.clone()) as std::sync::Arc<dyn editor_core::PartResolver>),
        ),
    )
    .expect("cuts");
    let mut parts = store;
    parts.insert(out.part.clone(), Tol::witness());
    let resolver: std::sync::Arc<dyn editor_core::PartResolver> = std::sync::Arc::new(parts);
    let back = inline(&out.remainder, out.instance, &resolver, Tol::witness())
        .expect("the instance inlines back");
    let (ia, ib) = (
        back.node_map[&out.node_map[&a]],
        back.node_map[&out.node_map[&b]],
    );
    assert_eq!(fixture::constrained(&back.doc), vec![ib]);
    assert_eq!(offset_of(&back.doc, ia), Some(offset));
    assert_eq!(offset_of(&back.doc, ib), None);
    assert!(
        fixture::without_anonymous(&back.maintenance).is_empty(),
        "{:?}",
        back.maintenance
    );
}

/// Two instances of one block, the first pinned to the world at an
/// offset, the second stacked by a mate and its values onto the first:
/// the document, its part store, `[a, b]` and `a`'s world read as
/// written.
fn placed_pair(
    label: &str,
) -> (
    ProfileDoc,
    PartStore,
    [RecipeNodeId; 2],
    Vec<(editor_core::Coord, Formula)>,
) {
    let mut store = PartStore::new();
    let (doc_ref, part_body) = store.insert_part(part(&format!("{label}-part")), Tol::witness());
    let opts = with_resolver(store.clone());
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, a) = insert(doc, fixture::mated_instance(doc_ref));
    let doc = fixture::pin(doc, a, &Frame::translation([0.0, 0.0, 4.0]), &opts);
    let (doc, b) = insert(doc, fixture::mated_instance(doc_ref));
    let (doc, joint) = stacked(
        doc,
        in_part(a, part_body, CapEnd::End),
        in_part(b, part_body, CapEnd::Start),
    );
    assert_eq!(joint.placement, b, "the mate places the second instance");
    let offset = offset_of(&doc, a).expect("a is placed by values alone");
    (doc, store, [a, b], offset)
}
