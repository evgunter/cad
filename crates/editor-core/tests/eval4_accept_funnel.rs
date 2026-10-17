//! The accepted edit travels whole through the refactoring doors:
//! [`split`]'s outcome carries the [`editor_core::Maintenance`] its remainder edits
//! and its part edits reported, and [`inline`]'s carries its own —
//! beside the documents and the recorded edits, never instead of them.
//!
//! No edit records a frame (A11 (2)), so a mated pair cut whole or
//! spliced back reports nothing: its world pin and its mate move as the
//! bundles they are. Each row reads the bundles of the documents that
//! came back, so an empty report is checked against what moved rather
//! than read as "nothing moved".

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::ExtrudeSide;
use editor_core::Formula;

use std::collections::BTreeSet;

use editor_core::{
    Alignment, AxisSense, CapEnd, ContactClass, DocumentId, Frame, MateFrame, MatePrimitive, Node,
    Placement, ProfileDoc, RecipeNodeId, StableName, inline,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{insert, len, offset_of, on_frame_keeping, square};
use geom_core::Tol;

/// A one-block part: a unit square extruded 1 tall, and its body.
fn part(label: &str) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, _, profile) = on_frame_keeping(
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
            side: ExtrudeSide::Along,
        },
    )
}

fn z_up() -> MateFrame<Formula> {
    MateFrame::authored(
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        geom_core::Tol::witness(),
    )
    .expect("a definite frame")
}

/// A frame-coincidence rest mate between two references, each read
/// at its own mint.
fn mate(a: StableName, b: StableName) -> editor_core::Mate<Formula> {
    fixture::mate(
        fixture::head(a),
        fixture::head(b),
        ContactClass::Rest,
        Alignment {
            a: z_up(),
            b: z_up(),
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Aligned,
            clocking: None,
        },
    )
}

/// Row 2 — a mated pair cut whole moves as selected (A4): the pinned
/// instance keeps its world pin in the part, the mate-placed one its
/// mate, and the instance left behind is pinned at the identity;
/// neither side reports anything: no edit records a frame.
#[test]
fn a_whole_pair_cut_moves_as_selected_and_reports_nothing() {
    let (doc, store, [a, b], offset) = placed_pair("eval4-r2");
    let out = fixture::split_world(
        &doc,
        &BTreeSet::from([a, b]),
        DocumentId::derive("eval4-r2-cell"),
        Tol::witness(),
        Some(&store),
    )
    .expect("a whole group and its mate cut");
    let (pa, pb) = (out.node_map[&a], out.node_map[&b]);
    assert_eq!(
        fixture::constrained(&out.part),
        vec![pb],
        "the mate re-forms in the part, in the mate-placed instance's bundle"
    );
    assert!(
        offset_of(&out.part, pa).is_some_and(|o| o.bit_eq(&offset)),
        "the pinned instance keeps its pose"
    );
    assert_eq!(
        offset_of(&out.part, pb),
        None,
        "the member stays mate-placed"
    );
    assert_eq!(
        offset_of(&out.remainder, out.instance),
        Some(Placement::IDENTITY),
        "the instance is pinned at the identity"
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
        Some(&store),
    )
    .expect("cuts");
    let mut parts = PartStore::new();
    parts.insert(out.part.clone(), Tol::witness());
    let resolver: std::sync::Arc<dyn editor_core::PartResolver> = std::sync::Arc::new(parts);
    let back = inline(&out.remainder, out.instance, &resolver, Tol::witness())
        .expect("the instance inlines back");
    let (ia, ib) = (
        back.node_map[&out.node_map[&a]],
        back.node_map[&out.node_map[&b]],
    );
    assert_eq!(fixture::constrained(&back.doc), vec![ib]);
    assert!(offset_of(&back.doc, ia).is_some_and(|o| o.bit_eq(&offset)));
    assert_eq!(offset_of(&back.doc, ib), None);
    assert!(back.maintenance.is_empty(), "{:?}", back.maintenance);
}

/// Two instances of one block, the first pinned to the world at an
/// offset, the second placed by a mate onto the first: the document,
/// its part store, `[a, b]` and `a`'s pose as stored.
fn placed_pair(
    label: &str,
) -> (
    ProfileDoc,
    std::sync::Arc<dyn editor_core::PartResolver>,
    [RecipeNodeId; 2],
    Placement,
) {
    let mut store = PartStore::new();
    let (doc_ref, part_body) = store.insert_part(part(&format!("{label}-part")), Tol::witness());
    let offset = Placement::literal(&Frame::translation([0.0, 0.0, 4.0]));
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, a) = insert(doc, fixture::mated_instance(doc_ref));
    let (doc, _) = fixture::constrain_at(doc, a, editor_core::Mate::world_pin(offset.clone()));
    let (doc, b) = insert(doc, fixture::mated_instance(doc_ref));
    let opts = with_resolver(store.clone());
    let reach = editor_core::mate_reach::<f64>(&opts, Tol::witness());
    let (doc, joint) = fixture::constrain_with(
        doc,
        &reach,
        mate(
            in_part(a, part_body, CapEnd::End),
            in_part(b, part_body, CapEnd::Start),
        ),
    );
    assert_eq!(joint.placement, b, "the mate places the second instance");
    (
        doc,
        std::sync::Arc::new(store),
        [a, b],
        editor_core::test_support::stored_placement(
            &mut editor_core::test_support::scratch(geom_core::Tol::witness()),
            &offset,
        ),
    )
}
