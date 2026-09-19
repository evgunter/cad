//! **An instance's crossing `outer`s are payload names** (N5's ladder
//! over `InterfaceRecord`).
//!
//! A `Node::InstantiatePart` carries an `InterfaceRecord`, and every
//! `InterfaceCrossing::Mate` in it holds two face names. One of them —
//! `outer` — is a REMAINDER name: it denotes a face in THIS document,
//! on a node this document can delete and under a name this document
//! can rebind. So it is a payload name, and `Node::payload_names` is
//! the one list that says so; the three doors reading that list — the
//! insert door's liveness check, `DocEdit::Rebind`, and DM7's strand
//! report through `Doc::name_carriers` — reach it because of the list
//! and for no other reason, which is what these rows measure.
//!
//! An `inner` is NOT listed and these rows pin that too: it is spelled
//! in the PART's id space, so a liveness check over it would ask this
//! document about an id it does not own, and `Rebind` would rewrite a
//! name whose meaning lives in another document. Its life is the
//! pinned product's, re-verified at evaluation
//! (`NodeErrorKind::CrossingUnverified`, ASSEMBLY A4).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    Alignment, AxisSense, CapEnd, ContactClass, DocEdit, DocRef, DocumentId, EditError, EntityKind,
    FaceName, InterfaceCrossing, InterfaceRecord, Maintenance, MateFrame, MatePrimitive, Node,
    ProfileDoc, ProfileProgram, RecipeNodeId, RoleSeg, StableName, apply, content_pin,
};
use fixture::resolver::{PART_BODY, in_part};
use fixture::{insert, on_frame, step};
use geom_core::Tol;

/// A one-block part document, as a reference with its content pin —
/// no store, because no row here evaluates: the doors under test are
/// edit doors and the record is data to all of them.
fn part_ref(label: &str) -> DocRef {
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
    let pin = content_pin(&doc, Tol::witness()).expect("the pin computes");
    DocRef { id: doc.id(), pin }
}

/// The part-local face an `inner` names — spelled in the PART's id
/// space, which is the whole reason it is not a payload name here.
fn part_side(cap: CapEnd) -> FaceName {
    FaceName::new(StableName {
        kind: EntityKind::Face,
        node: PART_BODY,
        path: vec![RoleSeg::Cap(cap)],
    })
    .expect("a crossing's references are face names")
}

fn crossing(mate: RecipeNodeId, outer: StableName, inner: FaceName) -> InterfaceCrossing {
    InterfaceCrossing::Mate {
        mate,
        class: ContactClass::Rest,
        outer: FaceName::new(outer).expect("a crossing's references are face names"),
        inner,
    }
}

/// A mate whose two heads are the named faces, read at their own
/// mints.
fn mate(a: StableName, b: StableName) -> Node<ProfileProgram> {
    Node::Mate {
        a: fixture::head(a),
        b: fixture::head(b),
        class: ContactClass::Rest,
        alignment: Alignment {
            a: MateFrame {
                origin: [0.0, 0.0, 0.0],
                axis: [0.0, 0.0, 1.0],
                reference: [1.0, 0.0, 0.0],
            },
            b: MateFrame {
                origin: [0.0, 0.0, 0.0],
                axis: [0.0, 0.0, 1.0],
                reference: [1.0, 0.0, 0.0],
            },
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Aligned,
            clocking: None,
        },
    }
}

/// The record of the instance at `id`, cloned — the subject of every
/// before/after comparison below.
fn record_of(doc: &ProfileDoc, id: RecipeNodeId) -> InterfaceRecord {
    let Some(Node::InstantiatePart { interface, .. }) = doc.node(id) else {
        panic!("the fixture's instance is an InstantiatePart");
    };
    interface.clone()
}

/// **`payload_names` answers an instance's crossing `outer`s, in
/// record order, and never an `inner`.**
///
/// Two crossings, so ORDER is observable and a one-crossing row could
/// not tell "record order" from "some order". The `inner`s are faces
/// of a node this document does not hold, so a list that leaked one
/// would be visible here rather than coinciding with an `outer`.
#[test]
fn an_instances_payload_names_are_its_crossing_outers_in_record_order() {
    let doc_ref = part_ref("crossnames-list-part");
    let doc = ProfileDoc::empty(DocumentId::derive("crossnames-list"), Tol::witness());
    let (doc, first) = insert(doc, Node::instantiate_part(doc_ref));
    let (doc, second) = insert(doc, Node::instantiate_part(doc_ref));

    // Second first, so the answer is the RECORD's order and not the
    // document's or the ids'.
    let outers = [in_part(second, CapEnd::End), in_part(first, CapEnd::Start)];
    let record = InterfaceRecord {
        crossings: vec![
            crossing(RecipeNodeId(7), outers[0].clone(), part_side(CapEnd::Start)),
            crossing(RecipeNodeId(8), outers[1].clone(), part_side(CapEnd::End)),
        ],
    };
    let (doc, instance) = insert(doc, Node::instantiate_part_with(doc_ref, record));

    let node = doc.node(instance).expect("the instance is live");
    assert_eq!(
        node.payload_names(),
        outers.iter().collect::<Vec<_>>(),
        "the list is the crossings' `outer`s, in record order"
    );
    for inner in [part_side(CapEnd::Start), part_side(CapEnd::End)] {
        assert!(
            !node.payload_names().contains(&&*inner),
            "an `inner` is spelled in the part's id space and is never a name in this document"
        );
    }
    assert_eq!(
        node.named_nodes(),
        vec![second, first],
        "the heads the insert door checks are the two instances the `outer`s name"
    );
}

/// **The insert door refuses a record whose `outer` names a node that
/// is not live**, through `Node::instantiate_part_with` — the public
/// door a split reaches the record through.
///
/// The dead node ONCE lived, so the refusal is the liveness check's
/// and not a never-minted id's arithmetic; the control above it
/// inserts the same record while the node is live.
#[test]
fn the_insert_door_refuses_a_record_whose_outer_is_not_live() {
    let doc_ref = part_ref("crossnames-door-part");
    let doc = ProfileDoc::empty(DocumentId::derive("crossnames-door"), Tol::witness());
    let (doc, target) = insert(doc, Node::instantiate_part(doc_ref));
    let outer = in_part(target, CapEnd::End);
    let record = || InterfaceRecord {
        crossings: vec![crossing(
            RecipeNodeId(7),
            outer.clone(),
            part_side(CapEnd::Start),
        )],
    };

    // The control: live, so the door accepts the same record.
    let (live, _) = insert(doc.clone(), Node::instantiate_part_with(doc_ref, record()));
    let _ = live;

    let (doc, _) = step(doc, DocEdit::DeleteNode { id: target });
    match apply(
        &doc,
        &DocEdit::InsertNode {
            node: Node::instantiate_part_with(doc_ref, record()),
        },
        Tol::witness(),
    ) {
        Err(EditError::DeclareNamesMissingNode { name }) => assert_eq!(
            name, outer,
            "the refusal names the crossing's `outer`, which is the reference that died"
        ),
        other => panic!("a record naming a dead node must refuse at the insert door: {other:?}"),
    }
}

/// **A `Rebind` of an `outer` rewrites the record and the mate that
/// carries the same head TOGETHER** — one list, so the two cannot
/// disagree about the seam after a repair.
#[test]
fn a_rebind_of_an_outer_rewrites_the_record_and_its_mate_together() {
    let doc_ref = part_ref("crossnames-rebind-part");
    let doc = ProfileDoc::empty(DocumentId::derive("crossnames-rebind"), Tol::witness());
    let (doc, a) = insert(doc, Node::instantiate_part(doc_ref));
    let (doc, b) = insert(doc, Node::instantiate_part(doc_ref));
    let from = in_part(a, CapEnd::End);
    let to = in_part(a, CapEnd::Start);
    let (doc, crossing_mate) = insert(doc, mate(from.clone(), in_part(b, CapEnd::End)));
    let record = InterfaceRecord {
        crossings: vec![crossing(
            crossing_mate,
            from.clone(),
            part_side(CapEnd::Start),
        )],
    };
    let (doc, instance) = insert(doc, Node::instantiate_part_with(doc_ref, record));

    let (rebound, _) = step(
        doc,
        DocEdit::Rebind {
            from: from.clone(),
            to: to.clone(),
        },
    );
    assert_eq!(
        record_of(&rebound, instance).crossings,
        vec![crossing(
            crossing_mate,
            to.clone(),
            part_side(CapEnd::Start)
        )],
        "the record's `outer` followed the rebind"
    );
    let Some(Node::Mate { a: head, .. }) = rebound.node(crossing_mate) else {
        panic!("the crossing mate survives the rebind");
    };
    assert_eq!(
        *head.name, to,
        "the mate carrying the same head followed it too"
    );
}

/// **A `Rebind` of an unrelated name leaves the record alone** — the
/// rewrite is by EXACT equality, so the row above measures the
/// rebind and not the visit.
#[test]
fn a_rebind_of_an_unrelated_name_leaves_the_record_untouched() {
    let doc_ref = part_ref("crossnames-unrelated-part");
    let doc = ProfileDoc::empty(DocumentId::derive("crossnames-unrelated"), Tol::witness());
    let (doc, a) = insert(doc, Node::instantiate_part(doc_ref));
    let (doc, b) = insert(doc, Node::instantiate_part(doc_ref));
    let outer = in_part(a, CapEnd::End);
    // The rebind's subject is the mate's OTHER head, so the edit has a
    // site and is accepted (a rebind with no site is refused).
    let elsewhere = in_part(b, CapEnd::End);
    let (doc, crossing_mate) = insert(doc, mate(outer.clone(), elsewhere.clone()));
    let record = InterfaceRecord {
        crossings: vec![crossing(
            crossing_mate,
            outer.clone(),
            part_side(CapEnd::Start),
        )],
    };
    let (doc, instance) = insert(doc, Node::instantiate_part_with(doc_ref, record));
    let before = record_of(&doc, instance);

    let (rebound, _) = step(
        doc,
        DocEdit::Rebind {
            from: elsewhere,
            to: in_part(b, CapEnd::Start),
        },
    );
    assert_eq!(
        record_of(&rebound, instance),
        before,
        "an `outer` that is not the rebind's source is not rewritten"
    );
}

/// **Deleting an `outer`'s minting node reports one DM7 strand,
/// carried by the INSTANCE** — the surviving carrier of the name, as
/// it is for every other payload.
#[test]
fn deleting_an_outers_minting_node_strands_it_on_the_instance() {
    let doc_ref = part_ref("crossnames-strand-part");
    let doc = ProfileDoc::empty(DocumentId::derive("crossnames-strand"), Tol::witness());
    let (doc, target) = insert(doc, Node::instantiate_part(doc_ref));
    let outer = in_part(target, CapEnd::End);
    let record = InterfaceRecord {
        crossings: vec![crossing(
            RecipeNodeId(9),
            outer.clone(),
            part_side(CapEnd::Start),
        )],
    };
    let (doc, instance) = insert(doc, Node::instantiate_part_with(doc_ref, record));

    let applied = apply(&doc, &DocEdit::DeleteNode { id: target }, Tol::witness())
        .expect("a name reference is not a DAG edge, so the delete is legal");
    let strands: Vec<(RecipeNodeId, StableName)> = applied
        .maintenance
        .iter()
        .filter_map(|row| match row {
            Maintenance::Strand { node, name } => Some((*node, name.clone())),
            Maintenance::Cluster(_) | Maintenance::StrandedAppearance { .. } => None,
        })
        .collect();
    assert_eq!(
        strands,
        vec![(instance, outer)],
        "the instance is the surviving carrier of the stranded `outer`"
    );
}
