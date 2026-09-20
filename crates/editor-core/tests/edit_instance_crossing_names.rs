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
//! The crossing's THIRD reference, its `mate`, is a node id rather
//! than a name: `Node::payload_read_sites` lists it, so the insert
//! door refuses a never-existed one as the typo it is, and a later
//! delete of it is not reported — the record then names a mate the
//! document no longer holds, which is what the id always denoted.

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
/// A two-copy pattern over `instance`: what lets a mate stand on ONE
/// instance twice without naming one member twice — the instance and
/// a copy of it are two members, so the edit door admits the mate
/// (one member on both sides it refuses as a self-mate), and the
/// solve welds nothing by it (a pair standing on one instance joins
/// no clusters). The mate these rows want: one that exists and
/// touches no cluster.
fn copies_of(instance: RecipeNodeId) -> Node<ProfileProgram> {
    Node::Pattern {
        input: instance,
        count: editor_core::Expr::count(2),
        kind: editor_core::PatternKind::Linear {
            direction: [
                crate::fixture::scl(1.0),
                crate::fixture::scl(0.0),
                crate::fixture::scl(0.0),
            ],
            spacing: crate::fixture::len(1.0),
        },
    }
}

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

    // Each crossing is keyed by its own mate, and a `mate` is a read
    // site the insert door checks is live — so the fixture mints two
    // rather than spelling ids.
    let (doc, first_mate) = insert(
        doc,
        mate(in_part(second, CapEnd::End), in_part(first, CapEnd::Start)),
    );
    let (doc, second_mate) = insert(
        doc,
        mate(in_part(first, CapEnd::End), in_part(second, CapEnd::Start)),
    );

    // Second first, so the answer is the RECORD's order and not the
    // document's or the ids'.
    let outers = [in_part(second, CapEnd::End), in_part(first, CapEnd::Start)];
    let record = InterfaceRecord {
        crossings: vec![
            crossing(first_mate, outers[0].clone(), part_side(CapEnd::Start)),
            crossing(second_mate, outers[1].clone(), part_side(CapEnd::End)),
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
    // The crossing's mate, live throughout: only the `outer`'s node
    // dies below, so the refusal can only be the NAME half's.
    let (doc, crossing_mate) = insert(doc, mate(outer.clone(), in_part(keeper, CapEnd::Start)));
    let record = || InterfaceRecord {
        crossings: vec![crossing(
            crossing_mate,
            outer.clone(),
            part_side(CapEnd::Start),
        )],
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
        crossings: vec![crossing(
            crossing_mate,
            outer.clone(),
            part_side(CapEnd::Start),
        )],
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
            | Maintenance::OrphanedDeclare { .. } => None,
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
    let (doc, crossing_mate) = insert(doc, mate((*inner).clone(), in_part(one, CapEnd::End)));
    let record = InterfaceRecord {
        crossings: vec![crossing(
            crossing_mate,
            in_part(zero, CapEnd::End),
            inner.clone(),
        )],
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

/// **The insert door refuses a record whose `mate` is not live**,
/// naming the id it could not find.
///
/// A crossing's `mate` is PROVENANCE rather than an operand — nothing
/// recomputes from it — but it is an id the payload carries into this
/// document, so it is a read site (`Node::payload_read_sites`) and
/// the typo rule applies whole: a never-existed id is a typo, refused
/// here. The control beside it is the same record with a live mate.
#[test]
fn the_insert_door_refuses_a_record_whose_mate_is_not_live() {
    let doc_ref = part_ref("crossnames-mate-part");
    let doc = ProfileDoc::empty(DocumentId::derive("crossnames-mate"), Tol::witness());
    let (doc, a) = insert(doc, Node::instantiate_part(doc_ref));
    let (doc, b) = insert(doc, Node::instantiate_part(doc_ref));
    let outer = in_part(a, CapEnd::End);
    let (doc, crossing_mate) = insert(doc, mate(outer.clone(), in_part(b, CapEnd::Start)));
    let with_mate = |mate_id| InterfaceRecord {
        crossings: vec![crossing(mate_id, outer.clone(), part_side(CapEnd::Start))],
    };

    // The control: the live mate inserts, so the refusal below is the
    // id's and not the record's shape.
    let (live, live_instance) = insert(
        doc.clone(),
        Node::instantiate_part_with(doc_ref, with_mate(crossing_mate)),
    );
    assert_eq!(
        record_of(&live, live_instance),
        with_mate(crossing_mate),
        "a record whose mate is live inserts whole"
    );

    // An id past the counter: never minted here, so a typo.
    let never = RecipeNodeId(99);
    match apply(
        &doc,
        &DocEdit::InsertNode {
            node: Node::instantiate_part_with(doc_ref, with_mate(never)),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    ) {
        Err(EditError::ReadSiteMissingNode { at }) => assert_eq!(
            at, never,
            "the refusal names the crossing's `mate`, the id that is not live"
        ),
        other => panic!("a record naming a mate that never existed must refuse: {other:?}"),
    }
}

/// **A delete of a crossing's `mate` is NOT reported**, exactly as a
/// read site's delete is not.
///
/// The record then names a mate the document no longer holds, which
/// is what the id always denoted: the mate the split observed. The
/// insert door's check is against a TYPO, not against the mate's
/// later life.
#[test]
fn deleting_a_crossings_mate_reports_nothing() {
    let doc_ref = part_ref("crossnames-mate-delete-part");
    let doc = ProfileDoc::empty(DocumentId::derive("crossnames-mate-delete"), Tol::witness());
    let (doc, a) = insert(doc, Node::instantiate_part(doc_ref));
    let (doc, b) = insert(doc, Node::instantiate_part(doc_ref));
    // The mate's own heads both stand on `b` — one on `b` itself, one
    // on a copy of it, two members over one instance, which the edit
    // door admits where one member twice it refuses — so deleting it
    // strands nothing of its own: the one name in play is the
    // `outer`, whose node `a` stays live.
    let outer = in_part(a, CapEnd::End);
    let (doc, copies) = insert(doc, copies_of(b));
    let (doc, crossing_mate) = insert(
        doc,
        mate(
            crate::fixture::in_copy(copies, 1, in_part(b, CapEnd::End)),
            in_part(b, CapEnd::Start),
        ),
    );
    let record = InterfaceRecord {
        crossings: vec![crossing(crossing_mate, outer, part_side(CapEnd::Start))],
    };
    let (doc, instance) = insert(doc, Node::instantiate_part_with(doc_ref, record));
    let before = record_of(&doc, instance);

    // The mate's heads both stand on `b`, so it joins no cluster and
    // its delete moves no gauge: the reach is never asked.
    let applied = apply(
        &doc,
        &DocEdit::DeleteNode { id: crossing_mate },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("a crossing's mate is not a DAG edge, so the delete is legal");
    assert!(
        applied.maintenance.is_empty(),
        "no row reports the gone mate: {:?}",
        applied.maintenance
    );
    assert_eq!(
        record_of(&applied.doc, instance),
        before,
        "and the record is unchanged — it names the mate it always named"
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
    let (doc, copies) = insert(doc, copies_of(keeper));
    let (doc, crossing_mate) = insert(
        doc,
        mate(
            outer.clone(),
            crate::fixture::in_copy(copies, 1, in_part(keeper, CapEnd::Start)),
        ),
    );
    let record = InterfaceRecord {
        crossings: vec![crossing(
            crossing_mate,
            outer.clone(),
            part_side(CapEnd::End),
        )],
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
    let SplitError::PartNameReachesRemainder { node, name } = refused else {
        panic!("the seam name is what refuses, not another precondition: {refused:?}");
    };
    assert_eq!(
        (node, *name),
        (carrier, outer),
        "the refusal names the instance and the crossing `outer` it carries"
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
    let (doc, copies) = insert(doc, copies_of(neighbour));
    let (doc, crossing_mate) = insert(
        doc,
        mate(
            outer.clone(),
            crate::fixture::in_copy(copies, 1, in_part(neighbour, CapEnd::Start)),
        ),
    );
    let record = InterfaceRecord {
        crossings: vec![crossing(crossing_mate, outer, part_side(CapEnd::End))],
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
