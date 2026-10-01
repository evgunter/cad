//! The accepted edit travels whole through the refactoring doors:
//! [`split`]'s outcome carries the [`editor_core::Maintenance`] its remainder edits
//! and its part edits reported, and [`inline`]'s carries its own —
//! beside the documents and the recorded edits, never instead of them.
//!
//! No edit records a frame (A11 (2)), so a group cut whole or spliced
//! back reports nothing: its offsets move as the chains they are. Each
//! row reads the group partition and the offsets of the documents that
//! came back, so an empty report is checked against what moved rather
//! than read as "nothing moved".

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::BTreeSet;

use editor_core::{
    Alignment, AxisSense, CapEnd, ContactClass, DocEdit, DocRef, DocumentId, EntityKind, Frame,
    MateFrame, MatePrimitive, Node, Placement, ProfileDoc, RecipeNodeId, RoleSeg, StableName,
    groups, inline, split,
};
use fixture::resolver::{PartStore, in_part};
use fixture::{insert, len, on_frame_keeping, square, step};
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
            profile,
            distance: len(1.0),
        },
    )
}

/// A local block in the host: its frame, profile and extrude, as the
/// cut set, and the extrude's id.
fn local_block(doc: ProfileDoc, cx: f64) -> (ProfileDoc, BTreeSet<RecipeNodeId>, RecipeNodeId) {
    let (doc, plane, profile) = on_frame_keeping(
        doc,
        [cx, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    let (doc, body) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    (doc, BTreeSet::from([plane, profile, body]), body)
}

/// A cap face of a LOCAL extrude — a name whose head places no body
/// of its own as a member, so a mate ending on it welds nothing.
fn local_cap(body: RecipeNodeId) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: body,
        path: vec![RoleSeg::Cap(CapEnd::Start)],
    }
}

fn z_up() -> MateFrame {
    MateFrame::authored([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0])
}

/// A frame-coincidence rest mate between two references, each read
/// at its own mint.
fn mate(a: StableName, b: StableName) -> Node<editor_core::ProfileProgram> {
    Node::Mate {
        a: crate::fixture::head(a),
        b: crate::fixture::head(b),
        class: ContactClass::Rest,
        alignment: Alignment {
            a: z_up(),
            b: z_up(),
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Aligned,
            clocking: None,
        },
    }
}

/// A host with one kept instance of `doc_ref`, whose body is
/// `part_body`, mated to a local block: the mate welds nothing (its far
/// end is no member). The insert door refuses a head that resolves to
/// no member, so the mate is authored the way such a head arises after
/// insert (`insert_mate_with_stranded_head`).
fn kept_instance_mated_to_a_local_block(
    label: &str,
    doc_ref: DocRef,
    part_body: RecipeNodeId,
) -> (
    ProfileDoc,
    RecipeNodeId,
    RecipeNodeId,
    BTreeSet<RecipeNodeId>,
) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, kept) = insert(doc, Node::instantiate_part(doc_ref));
    let (doc, cut, body) = local_block(doc, 3.0);
    let (doc, mate) = crate::fixture::insert_mate_with_stranded_head(
        doc,
        mate(in_part(kept, part_body, CapEnd::Start), local_cap(body)),
        editor_core::MateSide::B,
        kept,
        part_body,
    );
    assert_eq!(
        groups(&doc),
        vec![vec![kept]],
        "the kept instance is a singleton: the mate's far end is a local body"
    );
    (doc, kept, mate, cut)
}

/// Row 1 — a cut whose re-anchoring would make a mate START placing
/// refuses (A4): the instance the split leaves behind sits on the
/// world, as the kept instance does, so the mate that welded nothing
/// would weld the two and state a place nobody authored.
#[test]
fn a_cut_that_would_start_a_mate_placing_refuses() {
    let mut store = PartStore::new();
    let (doc_ref, part_body) = store.insert_part(part("eval4-r1-part"), Tol::witness());
    let (doc, _kept, mate, cut) =
        kept_instance_mated_to_a_local_block("eval4-r1", doc_ref, part_body);
    let err = split(
        &doc,
        &cut,
        DocumentId::derive("eval4-r1-cell"),
        Tol::witness(),
        None,
    )
    .expect_err("the re-anchored mate would place");
    assert!(
        matches!(&err, editor_core::SplitError::WouldStartPlacing { mate: m } if m.id() == mate),
        "{err:?}"
    );
}

/// Row 2 — a group cut whole HOISTS its root's offset onto the instance
/// left behind and lands the root at the empty chain in the part (A4);
/// the group re-forms there, and neither side reports anything: no
/// edit records a frame.
#[test]
fn a_whole_group_cut_hoists_its_root_offset_and_reports_nothing() {
    let (doc, store, [a, b, joint], offset) = placed_pair("eval4-r2");
    let out = split(
        &doc,
        &BTreeSet::from([a, b, joint]),
        DocumentId::derive("eval4-r2-cell"),
        Tol::witness(),
        Some(&store),
    )
    .expect("a whole group and its mate cut");
    let (pa, pb) = (out.node_map[&a], out.node_map[&b]);
    assert_eq!(
        groups(&out.part),
        vec![vec![pa, pb]],
        "the group re-forms in the part"
    );
    assert_eq!(
        offset_of(&out.part, pa),
        Some(Placement::IDENTITY),
        "the root lands at the empty chain"
    );
    assert_eq!(
        offset_of(&out.part, pb),
        None,
        "the member stays mate-placed"
    );
    assert!(
        offset_of(&out.remainder, out.instance).is_some_and(|o| o.bit_eq(&offset)),
        "the instance takes the root's offset"
    );
    assert!(
        out.part_maintenance.is_empty(),
        "{:?}",
        out.part_maintenance
    );
    assert!(
        out.remainder_maintenance.is_empty(),
        "{:?}",
        out.remainder_maintenance
    );
}

/// Row 3 — inline of that split is A4's sugar: the part is one group
/// rooted at the empty chain on its world, so its root takes the
/// instance's offset, and the document split was given comes back up
/// to node ids, reporting nothing.
#[test]
fn inline_of_a_hoisted_split_restores_the_root_offset_and_reports_nothing() {
    let (doc, store, [a, b, joint], offset) = placed_pair("eval4-r3");
    let out = split(
        &doc,
        &BTreeSet::from([a, b, joint]),
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
    assert_eq!(groups(&back.doc), vec![vec![ia, ib]]);
    assert!(offset_of(&back.doc, ia).is_some_and(|o| o.bit_eq(&offset)));
    assert_eq!(offset_of(&back.doc, ib), None);
    assert!(back.maintenance.is_empty(), "{:?}", back.maintenance);
}

/// Two instances of one block mated, the first placed at an offset:
/// the document, its part store, `[a, b, mate]` and `a`'s offset.
fn placed_pair(
    label: &str,
) -> (
    ProfileDoc,
    std::sync::Arc<dyn editor_core::PartResolver>,
    [RecipeNodeId; 3],
    Placement,
) {
    let mut store = PartStore::new();
    let (doc_ref, part_body) = store.insert_part(part(&format!("{label}-part")), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, a) = insert(doc, Node::instantiate_part(doc_ref));
    let (doc, b) = insert(doc, fixture::mated_instance(doc_ref));
    let (doc, joint) = insert(
        doc,
        mate(
            in_part(a, part_body, CapEnd::End),
            in_part(b, part_body, CapEnd::Start),
        ),
    );
    let offset = Placement::literal(&Frame::translation([0.0, 0.0, 4.0]));
    let (doc, _) = step(
        doc,
        DocEdit::SetOffset {
            instance: a,
            offset: Some(offset.clone()),
        },
    );
    assert_eq!(groups(&doc), vec![vec![a, b]], "one group, two members");
    (doc, std::sync::Arc::new(store), [a, b, joint], offset)
}

/// An instance's offset.
fn offset_of(doc: &ProfileDoc, id: RecipeNodeId) -> Option<Placement> {
    match doc.node(id) {
        Some(Node::InstantiatePart { offset, .. }) => offset.clone(),
        other => panic!("node {} is an instance, got {other:?}", id.0),
    }
}
