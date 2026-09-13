//! The accepted edit travels whole through the refactoring doors:
//! [`split`]'s outcome carries the cluster-record maintenance its
//! remainder edits and its part edits performed, and [`inline`]'s
//! carries its own — beside the documents and the recorded edits,
//! never instead of them.
//!
//! Each row asserts a maintenance act the door's edits are known to
//! perform (the act is checked against the cluster partition of the
//! document that came back), so a door that swapped a document in and
//! let the maintenance fall goes red here rather than reporting an
//! empty list that reads as "nothing moved".

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::BTreeSet;

use editor_core::{
    Alignment, AxisSense, CapEnd, ClusterMaintenance, ContactClass, DocEdit, DocRef, DocumentId,
    EntityKind, Frame, MateFrame, MatePrimitive, Node, ProfileDoc, RecipeNodeId, RoleSeg, SitedRef,
    StableName, clusters, inline, split,
};
use fixture::resolver::{PartStore, in_part};
use fixture::{insert, len, on_frame_keeping, square, step};
use geom_core::Tol;

/// A one-block part: a unit square extruded 1 tall, so its extrude is
/// the fixture's `PART_BODY`.
fn part(label: &str) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, _, profile) = on_frame_keeping(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    let (doc, _) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    doc
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
    MateFrame {
        origin: [0.0, 0.0, 0.0],
        axis: [0.0, 0.0, 1.0],
        reference: [1.0, 0.0, 0.0],
    }
}

/// A frame-coincidence rest mate between two references, each read
/// at its own mint.
fn mate(a: StableName, b: StableName) -> Node<editor_core::ProfileProgram> {
    Node::Mate {
        a: SitedRef::at_mint(a),
        b: SitedRef::at_mint(b),
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

/// A host with one kept instance of `doc_ref` mated to a local block:
/// the mate welds nothing before the split (its far end is no member)
/// and welds the kept instance to the new part instance after it.
fn kept_instance_mated_to_a_local_block(
    label: &str,
    doc_ref: DocRef,
) -> (ProfileDoc, RecipeNodeId, BTreeSet<RecipeNodeId>) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, kept) = insert(doc, Node::instantiate_part(doc_ref));
    let (doc, cut, body) = local_block(doc, 3.0);
    let (doc, _) = insert(doc, mate(in_part(kept, CapEnd::Start), local_cap(body)));
    assert_eq!(
        clusters(&doc),
        vec![vec![kept]],
        "before the split the kept instance is a singleton: the mate's far end is a local body"
    );
    (doc, kept, cut)
}

/// Row 1 — the remainder's `Rebind` joins two clusters, and the
/// outcome says so.
///
/// Re-anchoring the mate's far end onto the new instance is what
/// makes the mate weld: the kept instance's singleton absorbs the
/// instance the split just minted. That join is a fact about the
/// remainder the outcome hands back, so it must ride the outcome.
#[test]
fn a_rebind_that_joins_two_clusters_appears_in_the_remainder_maintenance() {
    let mut store = PartStore::new();
    let doc_ref = store.insert(part("eval4-r1-part"), Tol::witness());
    let (doc, kept, cut) = kept_instance_mated_to_a_local_block("eval4-r1", doc_ref);
    let out = split(
        &doc,
        &cut,
        DocumentId::derive("eval4-r1-cell"),
        Tol::witness(),
    )
    .expect("a local block whose only outside reference is a mate operand cuts");
    assert_eq!(
        clusters(&out.remainder),
        vec![vec![kept, out.instance]],
        "after the split the re-anchored mate welds the kept instance to the new one"
    );
    assert_eq!(
        out.remainder_maintenance,
        vec![ClusterMaintenance::Join {
            survived: kept,
            absorbed: out.instance,
            absorbed_frame: None,
        }],
        "the join the rebind performed rides the outcome"
    );
    assert!(
        out.part_maintenance.is_empty(),
        "the part holds no mate, so its edits moved no mate graph: {:?}",
        out.part_maintenance
    );
}

/// Row 2 — a cluster cut whole re-forms in the part (its mate's insert
/// is a join there) and dissolves in the remainder (its mate's delete
/// is a split there); each side's record rides its own outcome field.
#[test]
fn a_whole_cluster_cut_records_its_join_in_the_part_and_its_split_in_the_remainder() {
    let mut store = PartStore::new();
    let doc_ref = store.insert(part("eval4-r2-part"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("eval4-r2"), Tol::witness());
    let (doc, a) = insert(doc, Node::instantiate_part(doc_ref));
    let (doc, b) = insert(doc, Node::instantiate_part(doc_ref));
    let (doc, joint) = insert(
        doc,
        mate(in_part(a, CapEnd::End), in_part(b, CapEnd::Start)),
    );
    let (doc, _) = step(
        doc,
        DocEdit::SetPlacement {
            node: a,
            frame: Frame::translation([0.0, 0.0, 4.0]),
        },
    );
    assert_eq!(clusters(&doc), vec![vec![a, b]], "one cluster, two members");

    let out = split(
        &doc,
        &BTreeSet::from([a, b, joint]),
        DocumentId::derive("eval4-r2-cell"),
        Tol::witness(),
    )
    .expect("a whole cluster and its mate cut");
    let (pa, pb) = (out.node_map[&a], out.node_map[&b]);
    assert_eq!(
        clusters(&out.part),
        vec![vec![pa, pb]],
        "the cluster re-forms in the part"
    );
    assert_eq!(
        out.part_maintenance,
        vec![ClusterMaintenance::Join {
            survived: pa,
            absorbed: pb,
            absorbed_frame: None,
        }],
        "the part's mate insert joined the two spliced members"
    );
    // The remainder deletes the mate first (reverse document order),
    // which splits the cluster and re-mints the orphan's frame from
    // its solved pose; the two member deletes that follow move no mate
    // graph, so the split is the whole record.
    assert!(
        matches!(
            out.remainder_maintenance[..],
            [ClusterMaintenance::Split { from, to, frame: Some(_) }] if from == a && to == b
        ),
        "the remainder's mate delete split the cluster: {:?}",
        out.remainder_maintenance
    );
}

/// Row 3 — `inline` carries what its splice did: the wrapped name's
/// re-anchoring onto the spliced local body un-welds the instance from
/// the kept one (a split), which is the row-1 join undone.
#[test]
fn inline_records_the_split_its_re_anchoring_performs() {
    let mut store = PartStore::new();
    let doc_ref = store.insert(part("eval4-r3-part"), Tol::witness());
    let (doc, kept, cut) = kept_instance_mated_to_a_local_block("eval4-r3", doc_ref);
    let out = split(
        &doc,
        &cut,
        DocumentId::derive("eval4-r3-cell"),
        Tol::witness(),
    )
    .expect("cuts");
    store.insert(out.part.clone(), Tol::witness());
    let back = inline(&out.remainder, out.instance, &store, Tol::witness())
        .expect("the instance inlines back");
    assert_eq!(
        clusters(&back.doc),
        vec![vec![kept]],
        "after the splice the mate's far end is local again and welds nothing"
    );
    assert!(
        matches!(
            back.maintenance.first(),
            Some(ClusterMaintenance::Split { from, to, .. }) if *from == kept && *to == out.instance
        ),
        "the re-anchoring rebind split the instance off the kept cluster: {:?}",
        back.maintenance
    );
    assert!(
        !back
            .maintenance
            .iter()
            .any(|act| matches!(act, ClusterMaintenance::Join { .. })),
        "nothing the splice did joined a cluster: {:?}",
        back.maintenance
    );
}
