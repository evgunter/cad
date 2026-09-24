//! **An instance's crossing `outer`s are payload names** (N5's ladder
//! over `InterfaceRecord`).
//!
//! A `Node::InstantiatePart` carries an `InterfaceRecord`, and every
//! `InterfaceCrossing::Mate` in it holds two face names. One of them —
//! `outer` — is a REMAINDER name: it denotes a face in THIS document,
//! on a node this document can delete and under a name this document
//! can rebind. So it is a payload name, and `Node::payload_names` is
//! the one list that says so; the FOUR doors reading that list — the
//! insert door's liveness check, `DocEdit::Rebind`, DM7's strand
//! report through `Doc::name_carriers`, and `split`'s
//! `PartNameReachesRemainder` precondition, which reads the same walk
//! — reach it because of the list and for no other reason, which is
//! what these rows measure. The fourth changes `split`'s ACCEPTED
//! SET: a cut that takes an instance whose record names a kept node
//! is now refused, because a part cannot name the remainder.
//!
//! An `inner` is NOT listed and these rows pin that too — it is no
//! name of this document (`Node::payload_names`' arm is the one home
//! for why), so neither the liveness check nor `Rebind` may reach it.
//!
//! Those two references and the class are the WHOLE of a crossing
//! (`InterfaceCrossing::Mate`'s doc argues why it carries no
//! provenance), so an instance has no read site at all:
//! `Node::payload_read_sites` answers nothing for one, and the last
//! row here pins that.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    Alignment, AxisSense, CapEnd, ContactClass, DocEdit, DocRef, DocumentId, EditError, EntityKind,
    FaceName, InterfaceCrossing, InterfaceRecord, Maintenance, MateFrame, MatePrimitive, Node,
    ProfileDoc, ProfileProgram, RecipeNodeId, RoleSeg, SplitError, StableName, apply, content_pin,
    inline, split,
};
use fixture::resolver::{PART_BODY, PartStore, in_part};
use fixture::{insert, on_frame, step, step_with};
use geom_core::Tol;

/// A one-block part document — the part every row below instantiates.
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

/// [`part_doc`] as a reference with its content pin — no store,
/// because most rows here do not evaluate: the doors under test are
/// edit doors and the record is data to all of them. The two rows
/// that DO run a refactoring take their reference from a
/// [`PartStore`] instead, which is the same reference plus the
/// resolver the refactoring reads the part through.
fn part_ref(label: &str) -> DocRef {
    let doc = part_doc(label);
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

fn crossing(outer: StableName, inner: FaceName) -> InterfaceCrossing {
    InterfaceCrossing::Mate {
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
            a: MateFrame::authored([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
            b: MateFrame::authored([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
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
            crossing(outers[0].clone(), part_side(CapEnd::Start)),
            crossing(outers[1].clone(), part_side(CapEnd::End)),
        ],
    };
    let (doc, instance) = insert(doc, Node::instantiate_part_with(doc_ref, record));

    let node = doc.node(instance).expect("the instance is live");
    // EQUALITY, not containment: the two `inner`s are faces of a node
    // this document does not hold, so a list that leaked one would
    // fail here — a separate `!contains` loop would assert nothing
    // this comparison does not already.
    assert_eq!(
        node.payload_names(),
        outers.iter().collect::<Vec<_>>(),
        "the list is the crossings' `outer`s, in record order, and never an `inner`"
    );
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
    // `target` and `keeper` are mated, so the delete below moves that
    // cluster's gauge and levers the parts through the store's reach.
    let mut store = PartStore::default();
    let doc_ref = store.insert(part_doc("crossnames-door-part"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("crossnames-door"), Tol::witness());
    let (doc, target) = insert(doc, Node::instantiate_part(doc_ref));
    let (doc, keeper) = insert(doc, Node::instantiate_part(doc_ref));
    let outer = in_part(target, CapEnd::End);
    // The mate the crossing came from — it welds the two instances
    // into the cluster whose gauge the delete below moves, which is
    // what makes the reach answer.
    let (doc, _) = insert(doc, mate(outer.clone(), in_part(keeper, CapEnd::Start)));
    let record = || InterfaceRecord {
        crossings: vec![crossing(outer.clone(), part_side(CapEnd::Start))],
    };

    // The control: live, so the door accepts the same record — and
    // the record it accepted is the one the fixture wrote.
    let (live, live_instance) = insert(doc.clone(), Node::instantiate_part_with(doc_ref, record()));
    assert_eq!(
        record_of(&live, live_instance),
        record(),
        "while the `outer`'s node is live the door accepts the record whole"
    );

    let opts = fixture::resolver::with_resolver(store);
    let reach = editor_core::mate_reach::<f64>(&opts, Tol::witness());
    let (doc, _) = step_with(doc, DocEdit::DeleteNode { id: target }, &reach);
    match apply(
        &doc,
        &DocEdit::InsertNode {
            node: Node::instantiate_part_with(doc_ref, record()),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
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
        crossings: vec![crossing(from.clone(), part_side(CapEnd::Start))],
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
        vec![crossing(to.clone(), part_side(CapEnd::Start))],
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
    let (doc, _) = insert(doc, mate(outer.clone(), elsewhere.clone()));
    let record = InterfaceRecord {
        crossings: vec![crossing(outer.clone(), part_side(CapEnd::Start))],
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

/// **Deleting an `outer`'s minting node reports a DM7 strand carried
/// by the INSTANCE** — the surviving carrier of the name, as it is
/// for every other payload.
///
/// The fixture is the shape a split mints: the crossing's `outer` is
/// the remainder-side head of the crossing's own mate, so the one
/// delete strands the SAME name on two carriers and the row reads
/// both, in document order. The instance's row is the one this unit
/// added; the mate's was there before it.
#[test]
fn deleting_an_outers_minting_node_strands_it_on_the_instance() {
    // `target` and `keeper` are mated, so deleting `target` moves that
    // cluster's gauge and the maintenance levers the parts through the
    // store's reach.
    let mut store = PartStore::default();
    let doc_ref = store.insert(part_doc("crossnames-strand-part"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("crossnames-strand"), Tol::witness());
    let (doc, target) = insert(doc, Node::instantiate_part(doc_ref));
    let (doc, keeper) = insert(doc, Node::instantiate_part(doc_ref));
    let outer = in_part(target, CapEnd::End);
    let (doc, crossing_mate) = insert(doc, mate(outer.clone(), in_part(keeper, CapEnd::Start)));
    let record = InterfaceRecord {
        crossings: vec![crossing(outer.clone(), part_side(CapEnd::Start))],
    };
    let (doc, instance) = insert(doc, Node::instantiate_part_with(doc_ref, record));

    let opts = fixture::resolver::with_resolver(store);
    let reach = editor_core::mate_reach::<f64>(&opts, Tol::witness());
    let applied = apply(
        &doc,
        &DocEdit::DeleteNode { id: target },
        Tol::witness(),
        &reach,
    )
    .expect("a name reference is not a DAG edge, so the delete is legal");
    let strands: Vec<(RecipeNodeId, StableName)> = applied
        .maintenance
        .iter()
        .filter_map(|row| match row {
            Maintenance::Strand { node, name } => Some((*node, name.clone())),
            Maintenance::Cluster(_)
            | Maintenance::StrandedAppearance { .. }
            | Maintenance::OrphanedDeclare { .. }
            | Maintenance::Rebound { .. } => None,
        })
        .collect();
    assert_eq!(
        strands,
        vec![(crossing_mate, outer.clone()), (instance, outer)],
        "the instance is a surviving carrier of the stranded `outer`, beside the mate"
    );
}

/// **A `Rebind` whose source is EXACTLY a crossing's `inner` leaves
/// that `inner` alone** — the negative half of the rewriting twin's
/// ruling, which the positive rows above cannot see.
///
/// A part-side id can COLLIDE with a live node of this document —
/// `PART_BODY` is `RecipeNodeId(2)`, and this document's third node
/// is live — so a name byte-equal to the `inner` can be a legal
/// rebind source here (the mate head below is its site). The
/// rewriting twin must still not touch the record: rewriting it would
/// move a part-space reference onto a node id of THIS document, which
/// is the wrong repair the arm's own prose forbids.
#[test]
fn a_rebind_of_a_name_equal_to_an_inner_leaves_the_inner_alone() {
    let doc_ref = part_ref("crossnames-inner-rebind-part");
    let doc = ProfileDoc::empty(
        DocumentId::derive("crossnames-inner-rebind"),
        Tol::witness(),
    );
    let (doc, zero) = insert(doc, Node::instantiate_part(doc_ref));
    let (doc, one) = insert(doc, Node::instantiate_part(doc_ref));
    // The third node makes `PART_BODY`'s id live in THIS document,
    // which is what lets a name equal to the `inner` be a rebind
    // source at all.
    let (doc, collide) = insert(doc, Node::instantiate_part(doc_ref));
    assert_eq!(collide, PART_BODY, "the row needs the collision it names");

    let inner = part_side(CapEnd::Start);
    let (doc, _) = insert(doc, mate((*inner).clone(), in_part(one, CapEnd::End)));
    let record = InterfaceRecord {
        crossings: vec![crossing(in_part(zero, CapEnd::End), inner.clone())],
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
        "an `inner` is no name of this document, so `Rebind` never rewrites it — not even \
         when a part-side id collides with a live node here"
    );
}

/// **An instance's record answers NO read site**, however many
/// crossings it holds — so the insert door checks a record's NAMES
/// and nothing else.
///
/// A crossing is a class and two face names: the `outer` is a payload
/// name the reading twin lists, the `inner` is spelled in the part's
/// id space and belongs to no door here, and there is no third
/// reference to check. `Node::payload_read_sites`' match is
/// exhaustive with no wildcard, so an instance is classified with the
/// variants that answer nothing or does not compile — which is what
/// keeps this answer honest as `Node` grows.
///
/// The mate beside it is the CONTROL: the same call on the same
/// document answers that mate's two operands, so an empty answer here
/// is the instance's and not a list that stopped working.
#[test]
fn an_instances_record_answers_no_read_site() {
    let doc_ref = part_ref("crossnames-readsite-part");
    let doc = ProfileDoc::empty(DocumentId::derive("crossnames-readsite"), Tol::witness());
    let (doc, a) = insert(doc, Node::instantiate_part(doc_ref));
    let (doc, b) = insert(doc, Node::instantiate_part(doc_ref));
    let (doc, crossing_mate) = insert(
        doc,
        mate(in_part(a, CapEnd::End), in_part(b, CapEnd::Start)),
    );
    // Two crossings, so a per-crossing answer would be visible.
    let record = InterfaceRecord {
        crossings: vec![
            crossing(in_part(a, CapEnd::End), part_side(CapEnd::Start)),
            crossing(in_part(b, CapEnd::Start), part_side(CapEnd::End)),
        ],
    };
    let (doc, instance) = insert(doc, Node::instantiate_part_with(doc_ref, record));

    let node = doc.node(instance).expect("the instance is live");
    assert!(
        node.payload_read_sites().is_empty(),
        "a record carries no id, so the instance names no read site: {:?}",
        node.payload_read_sites()
    );
    assert_eq!(
        node.named_nodes(),
        vec![a, b],
        "its references are NAMES, and they are checked as names"
    );
    let control = doc.node(crossing_mate).expect("the mate is live");
    assert_eq!(
        control.payload_read_sites(),
        vec![a, b],
        "the same list answers a mate's two operands — the empty answer above \
         is the instance's own"
    );
}

/// **A split that TAKES an instance whose record names a KEPT node is
/// refused** — the fourth door, and the one that changes `split`'s
/// accepted set.
///
/// `split`'s `PartNameReachesRemainder` precondition walks
/// `Doc::name_carriers`, which reads `payload_names`; a crossing
/// `outer` is now on that list, so a cut that would carry the record
/// into the part while the name it holds stays behind is refused.
/// That is the right answer — a part cannot name the remainder — and
/// on `origin/main` the same cut was accepted, with `remap_node`
/// cloning a host-space name into the part.
///
/// Reachable: an inhabited record is hand-buildable through the
/// public `Node::instantiate_part_with` and round-trips through the
/// wire, so a loaded document can hold one and a later cut can take
/// the instance.
#[test]
fn a_split_that_takes_an_instance_naming_a_kept_node_is_refused() {
    let mut store = PartStore::default();
    let doc_ref = store.insert(part_doc("crossnames-split-part"), Tol::witness());
    let (doc, keeper) = insert(
        ProfileDoc::empty(DocumentId::derive("crossnames-split"), Tol::witness()),
        Node::instantiate_part(doc_ref),
    );
    let outer = in_part(keeper, CapEnd::End);
    let record = InterfaceRecord {
        crossings: vec![crossing(outer.clone(), part_side(CapEnd::End))],
    };
    let (doc, carrier) = insert(doc, Node::instantiate_part_with(doc_ref, record));

    let cut: std::collections::BTreeSet<RecipeNodeId> = [carrier].into_iter().collect();
    let refused = split(
        &doc,
        &cut,
        DocumentId::derive("crossnames-split-cut"),
        Tol::witness(),
        None,
    )
    .expect_err("the instance's `outer` is a payload name reaching the remainder");
    let SplitError::PartNameReachesRemainder {
        node,
        name,
        missing,
    } = refused
    else {
        panic!("the seam name is what refuses, not another precondition: {refused:?}");
    };
    assert_eq!(
        (node, *name),
        (carrier, outer),
        "the refusal names the instance and the crossing `outer` it carries"
    );
    assert_eq!(
        missing, keeper,
        "and the node it reaches: the `InPart` argument is another document's id space, so \
         the one LOCAL node the name derives from is the kept instance"
    );
}

/// **`inline` of an instance carrying a record splices the PART's own
/// nodes into the remainder** — the positive half of the dissolve,
/// measured by what the splice contributed rather than by the
/// document being non-empty.
///
/// The document holds a neighbour instance that `inline` never
/// removes, so `!order().is_empty()` would hold however little the
/// splice contributed; the order's GROWTH by the part's node count is
/// what only the splice can produce.
#[test]
fn inlining_an_instance_with_a_record_splices_the_parts_own_nodes() {
    let mut store = PartStore::default();
    let part = part_doc("crossnames-inline-part");
    let part_nodes = part.order().len();
    let doc_ref = store.insert(part, Tol::witness());
    let (doc, neighbour) = insert(
        ProfileDoc::empty(DocumentId::derive("crossnames-inline"), Tol::witness()),
        Node::instantiate_part(doc_ref),
    );
    let outer = in_part(neighbour, CapEnd::End);
    let record = InterfaceRecord {
        crossings: vec![crossing(outer, part_side(CapEnd::End))],
    };
    let (doc, instance) = insert(doc, Node::instantiate_part_with(doc_ref, record));
    let before = doc.order().len();

    let back = inline(
        &doc,
        instance,
        &(std::sync::Arc::new(store) as std::sync::Arc<dyn editor_core::PartResolver>),
        Tol::witness(),
    )
    .expect("inline succeeds");
    assert!(
        back.doc.node(instance).is_none(),
        "the instance is gone, and its record with it"
    );
    assert_eq!(
        back.doc.order().len(),
        before - 1 + part_nodes,
        "the instance went and the part's own nodes came in its place — a count the \
         untouched neighbour cannot supply"
    );
}
