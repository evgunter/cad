//! **Review probes for `edit/instance-crossing-names` (PR #2872).**
//!
//! Not acceptance rows: two measurements a reviewer took, left here so
//! the fix pass can adopt or delete them deliberately.
//!
//! 1. `a_rebind_of_a_name_equal_to_an_inner_leaves_the_inner_alone` —
//!    the NEGATIVE half of the rewriting twin's ruling ("`Rebind` over
//!    an `inner` would be a wrong repair") has no row on the PR head: a
//!    mutant that rewrites an `inner` equal to `from` survives all 1477
//!    integration rows. This row reds under that mutant.
//! 2. `row5_cs_splice_guard_is_satisfied_by_the_neighbour_alone` — the
//!    re-fixture of `asm_r2b_assembly::row5_c` put a second instance in
//!    the document, and that row's `!back.doc.order().is_empty()`
//!    ("the part's recipe is spliced in") is now satisfied by the
//!    neighbour whatever inline splices.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    Alignment, AxisSense, CapEnd, ContactClass, DocEdit, DocRef, DocumentId, EntityKind, FaceName,
    InterfaceCrossing, InterfaceRecord, MateFrame, MatePrimitive, Node, ProfileDoc, ProfileProgram,
    RecipeNodeId, RoleSeg, StableName, content_pin, inline,
};
use fixture::resolver::{PART_BODY, PartStore, in_part};
use fixture::{insert, on_frame, step};
use geom_core::Tol;

/// A one-block part document.
fn part_doc(label: &str) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let (doc, _) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: fixture::len(1.0),
        },
    );
    doc
}

fn part_ref(label: &str) -> DocRef {
    let doc = part_doc(label);
    let pin = content_pin(&doc, Tol::witness()).expect("the pin computes");
    DocRef { id: doc.id(), pin }
}

/// A part-side face name — spelled in the PART's id space, which is
/// why it is not a payload name of the host document.
fn part_side(cap: CapEnd) -> FaceName {
    FaceName::new(StableName {
        kind: EntityKind::Face,
        node: PART_BODY,
        path: vec![RoleSeg::Cap(cap)],
    })
    .expect("a crossing's references are face names")
}

fn mate(a: StableName, b: StableName) -> Node<ProfileProgram> {
    let frame = MateFrame {
        origin: [0.0, 0.0, 0.0],
        axis: [0.0, 0.0, 1.0],
        reference: [1.0, 0.0, 0.0],
    };
    Node::Mate {
        a: fixture::head(a),
        b: fixture::head(b),
        class: ContactClass::Rest,
        alignment: Alignment {
            a: frame,
            b: frame,
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Aligned,
            clocking: None,
        },
    }
}

fn record_of(doc: &ProfileDoc, id: RecipeNodeId) -> InterfaceRecord {
    let Some(Node::InstantiatePart { interface, .. }) = doc.node(id) else {
        panic!("the fixture's instance is an InstantiatePart");
    };
    interface.clone()
}

/// **A `Rebind` whose source is EQUAL to a crossing's `inner` leaves
/// that `inner` alone.**
///
/// An `inner` is spelled in the PART's id space, and a part-side id
/// can COLLIDE with a live node of this document — `PART_BODY` is
/// `RecipeNodeId(2)`, and this document's third node is live. So a
/// name that is byte-equal to the `inner` can be a legal rebind
/// source here (the mate head below is its site), and the rewriting
/// twin must still not touch the record: rewriting it would move a
/// part-space reference onto a node id of this document, which is the
/// "wrong repair" the arm's own prose forbids.
#[test]
fn a_rebind_of_a_name_equal_to_an_inner_leaves_the_inner_alone() {
    let doc_ref = part_ref("probe-inner-rebind-part");
    let doc = ProfileDoc::empty(DocumentId::derive("probe-inner-rebind"), Tol::witness());
    let (doc, zero) = insert(doc, Node::instantiate_part(doc_ref));
    let (doc, one) = insert(doc, Node::instantiate_part(doc_ref));
    // The third node makes `PART_BODY`'s id live in THIS document, which
    // is what lets a name equal to the `inner` be a rebind source.
    let (doc, collide) = insert(doc, Node::instantiate_part(doc_ref));
    assert_eq!(collide, PART_BODY, "the probe needs the collision it names");

    let inner = part_side(CapEnd::Start);
    let (doc, crossing_mate) = insert(doc, mate((*inner).clone(), in_part(one, CapEnd::End)));
    let record = InterfaceRecord {
        crossings: vec![InterfaceCrossing::Mate {
            mate: crossing_mate,
            class: ContactClass::Rest,
            outer: FaceName::new(in_part(zero, CapEnd::End)).expect("a face name"),
            inner: inner.clone(),
        }],
    };
    let (doc, instance) = insert(doc, Node::instantiate_part_with(doc_ref, record));
    let before = record_of(&doc, instance);

    let (rebound, _) = step(
        doc,
        DocEdit::Rebind {
            from: (*inner).clone(),
            to: in_part(collide, CapEnd::End),
        },
    );
    assert_eq!(
        record_of(&rebound, instance),
        before,
        "a crossing's `inner` is not a name in this document, so `Rebind` \
         never rewrites it — not even when a part-side id collides with a \
         live node here"
    );
}

/// **`asm_r2b_assembly::row5_c`'s splice guard no longer measures the
/// splice**: its document holds a neighbour instance that `inline`
/// never removes, so `!back.doc.order().is_empty()` holds however
/// little the splice contributes.
#[test]
fn row5_cs_splice_guard_is_satisfied_by_the_neighbour_alone() {
    let mut store = PartStore::default();
    let doc_ref = store.insert(part_doc("probe-row5c-part"), Tol::witness());
    // The re-fixtured shape: a neighbour instance, then the instance
    // carrying the record.
    let (doc, neighbour) = insert(
        ProfileDoc::empty(DocumentId::derive("probe-row5c"), Tol::witness()),
        Node::instantiate_part(doc_ref),
    );
    let record = InterfaceRecord {
        crossings: vec![InterfaceCrossing::Mate {
            mate: RecipeNodeId(9),
            class: ContactClass::Rest,
            outer: FaceName::new(in_part(neighbour, CapEnd::End)).expect("a face name"),
            inner: part_side(CapEnd::End),
        }],
    };
    let (doc, instance) = insert(doc, Node::instantiate_part_with(doc_ref, record));

    let back = inline(&doc, instance, &store, Tol::witness()).expect("inline succeeds");
    assert!(
        back.doc.node(neighbour).is_some(),
        "the neighbour is untouched by the inline"
    );
    assert!(
        back.doc.order().contains(&neighbour),
        "so the row's `!order().is_empty()` is satisfied by the neighbour \
         alone, whatever the splice contributed"
    );
}

/// **A FOURTH door follows from the one list, and the PR body names
/// three.** `split`'s `PartNameReachesRemainder` precondition reads
/// `Doc::name_carriers` (`refactor.rs:1334`), which reads
/// `payload_names` — so a cut that TAKES an instance whose record's
/// `outer` names a kept node is now refused, where on `origin/main`
/// it was accepted and `remap_node`'s `InstantiatePart` arm
/// (`refactor.rs:1067`, "its interface record rides with it") cloned
/// the host-space name into the part.
///
/// Reachable: an inhabited record is hand-buildable through the
/// public `Node::instantiate_part_with` and round-trips through the
/// wire (`asm_r2b_interface_wire`), so a loaded document can hold one
/// and a later cut can take the instance. The refusal is the RIGHT
/// answer; it is the disclosure and the row that are missing.
#[test]
fn a_split_moving_an_instance_that_carries_a_record_is_refused_by_a_fourth_door() {
    let mut store = PartStore::default();
    let doc_ref = store.insert(part_doc("probe-split-part"), Tol::witness());
    let (doc, neighbour) = insert(
        ProfileDoc::empty(DocumentId::derive("probe-split"), Tol::witness()),
        Node::instantiate_part(doc_ref),
    );
    let outer = in_part(neighbour, CapEnd::End);
    let record = InterfaceRecord {
        crossings: vec![InterfaceCrossing::Mate {
            mate: RecipeNodeId(9),
            class: ContactClass::Rest,
            outer: FaceName::new(outer.clone()).expect("a face name"),
            inner: part_side(CapEnd::End),
        }],
    };
    let (doc, carrier) = insert(doc, Node::instantiate_part_with(doc_ref, record));

    let cut: std::collections::BTreeSet<RecipeNodeId> = [carrier].into_iter().collect();
    let refused = editor_core::split(
        &doc,
        &cut,
        DocumentId::derive("probe-split-cut"),
        Tol::witness(),
    )
    .expect_err(
        "the instance's `outer` is a payload name reaching the remainder, so the \
         split precondition refuses — this cut was ACCEPTED on `origin/main`",
    );
    let editor_core::SplitError::PartNameReachesRemainder { node, name } = refused else {
        panic!("the seam name is what refuses, not another precondition: {refused:?}");
    };
    assert_eq!(
        (node, *name),
        (carrier, outer),
        "the refusal names the instance and the crossing `outer` it carries"
    );
}
