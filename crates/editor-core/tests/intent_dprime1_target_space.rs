//! INTENT stage 3 D′1 acceptance — a placement names its target space
//! (`docs/INTENT-STAGE3-SPEC.md` §5′.1, tests 18 and 23).
//!
//! Which space a body is in is derived from the recipe alone: a
//! construction's body is in its own space, a placed copy in its
//! placement's `target`, an operator's result in its operands' space.
//! The edit door refuses a node whose reads lie in two spaces before
//! any evaluation; a re-target that strands a downstream reader is
//! admitted and reported, the document saves and loads, and the reader
//! refuses `SpaceMismatch` at evaluation. Every bundle pins its copy.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::{
    Bodies, CapEnd, ContactClass, Coord, DocEdit, DocumentId, EditError, EvalOptions, Maintenance,
    MateFault, Node, NodeErrorKind, NodeResult, Operand, OperandSlot, ProfileDoc, RecipeNodeId,
    RoleSeg, SlotId, SlotValue, TargetAxis, VarId, VarKind, apply, load, save,
};
use fixture::{insert, len, on_frame, run, square, step};
use geom_core::Tol;

/// A unit block centred at `(x, 0)`: the document and its extrude, a
/// construction in a space of its own.
fn block(doc: ProfileDoc, x: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(x, 0.0, 0.5)],
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    )
}

fn union(a: impl Into<Operand>, b: impl Into<Operand>) -> editor_core::AuthoredNode {
    Node::Union {
        members: Bodies::Spelled(vec![a.into().into(), b.into().into()]),
    }
}

/// **Test 23, the door half: a union of two constructions refuses
/// `SpaceMismatch` at the edit door, naming both spaces, before any
/// evaluation.** Each block is in the space its extrude defines; no
/// evaluation is asked for, so the refusal is the recipe's.
///
/// *Breaks if* the check reads a value, or admits two spaces.
#[test]
fn a_union_of_two_constructions_refuses_at_the_door() {
    let doc = ProfileDoc::empty(DocumentId::derive("dprime1-door"), Tol::witness());
    let (doc, a) = block(doc, 0.0);
    let (doc, b) = block(doc, 3.0);
    let (space_a, space_b) = (fixture::space(&doc, a), fixture::space(&doc, b));
    assert_ne!(space_a, space_b, "each construction has a space of its own");
    assert_eq!(doc.output(a, 1), Some(space_a), "its last port");
    match apply(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(union(a, b)),
            fresh: Vec::new(),
        },
        Tol::witness(),
    ) {
        Err(EditError::SpaceMismatch {
            a: one, b: other, ..
        }) => {
            assert_eq!((one.id(), other.id()), (space_a, space_b));
        }
        other => panic!("two spaces refuse at the door, got {other:?}"),
    }
}

/// **Test 23, the re-target half.** `c` is copied into `a`'s space and
/// unioned with `a`; re-targeting the copy into `b`'s space is admitted
/// and reported (`Maintenance::StrandedSpace`), the document saves and
/// loads, and the union refuses `SpaceMismatch` at evaluation naming
/// both spaces; re-targeting back resolves it.
///
/// *Breaks if* the re-target is refused, the load door refuses the
/// stranded document, or the union evaluates across two spaces.
#[test]
fn a_re_target_strands_the_union_and_re_targeting_back_resolves_it() {
    let doc = ProfileDoc::empty(DocumentId::derive("dprime1-retarget"), Tol::witness());
    let (doc, a) = block(doc, 0.0);
    let (doc, b) = block(doc, 3.0);
    let (doc, c) = block(doc, 6.0);
    let (space_a, space_b) = (fixture::space(&doc, a), fixture::space(&doc, b));
    let (doc, copy) = fixture::stood_in(doc, c, a);
    let (doc, joined) = insert(doc, union(a, copy));
    assert_eq!(doc.node_space(joined), Some(space_a));
    let o = EvalOptions::default();
    assert!(matches!(
        run(&doc, &o).result(joined),
        Some(NodeResult::Ok(_))
    ));

    let retarget = |doc: &ProfileDoc, space: VarId| {
        apply(
            doc,
            &DocEdit::SetParam {
                node: copy,
                slot: SlotId::Operand(OperandSlot::Space),
                value: SlotValue::Read(Operand::Var(space)),
                fresh: Vec::new(),
            },
            Tol::witness(),
        )
        .expect("a re-target is admitted")
    };
    let moved = retarget(&doc, space_b);
    assert!(
        moved.maintenance.iter().any(|m| matches!(
            m,
            Maintenance::StrandedSpace { node, .. } if node.id() == joined
        )),
        "the stranded union is reported: {:?}",
        moved.maintenance
    );
    let stranded = moved.doc;
    assert_eq!(stranded.space_fault(joined), Some((space_a, space_b)));

    let text = save(&stranded, &[], Tol::witness()).expect("the stranded document saves");
    let loaded = load(&text, Tol::witness()).expect("and loads").doc;
    assert!(loaded.bit_eq(&stranded), "as written");
    let ev = run(&loaded, &o);
    match ev.node_error(joined).map(|e| &e.kind) {
        Some(NodeErrorKind::SpaceMismatch { a: one, b: other }) => {
            assert_eq!((*one, *other), (space_a, space_b));
        }
        other => panic!("the union refuses SpaceMismatch at evaluation, got {other:?}"),
    }

    let back = retarget(&loaded, space_a).doc;
    assert_eq!(back.space_fault(joined), None);
    assert!(matches!(
        run(&back, &o).result(joined),
        Some(NodeResult::Ok(_))
    ));
}

/// **Test 18: every freedom is written.** An empty bundle, and a plane
/// mate with one of its three values, each refuse `Unpinned` at the
/// door, naming the freedoms they leave.
///
/// *Breaks if* a bundle that leaves a freedom unset is admitted — a
/// loose copy under another name.
#[test]
fn an_empty_bundle_and_a_plane_mate_with_one_value_refuse_unpinned() {
    let doc = ProfileDoc::empty(DocumentId::derive("dprime1-unpinned"), Tol::witness());
    let (doc, a) = block(doc, 0.0);
    let (doc, b) = block(doc, 3.0);
    let (doc, under) = fixture::place(doc, a);
    let (doc, target) = fixture::place(doc, b);

    let empty = fixture::bundle_refusal(&doc, under, Vec::new());
    assert!(
        matches!(empty, MateFault::Unpinned { placement, free: 6 } if placement == under),
        "{empty:?}"
    );

    let mate = fixture::mate(
        fixture::plane(fixture::head(fixture::fname(
            a,
            RoleSeg::Cap(CapEnd::Start),
        ))),
        fixture::flip(fixture::plane(fixture::head_at(
            target,
            fixture::fname(b, RoleSeg::Cap(CapEnd::End)),
        ))),
        ContactClass::Rest,
    );
    let one_value = editor_core::Placed {
        mate: Some(mate),
        values: vec![(Coord::Slide(TargetAxis::X), len(0.0))],
        by: editor_core::PlacedBy::PlanarFace,
    };
    let short = fixture::bundle_refusal(&doc, under, one_value.bundle());
    assert!(
        matches!(short, MateFault::Unpinned { placement, free: 2 } if placement == under),
        "{short:?}"
    );
}

/// **A copy of a copy in a new space reaches the product.** `a` placed
/// into a `new_space()` by the identity bundle is in that space and not
/// the product; a copy of that copy placed into the world puts `a` in
/// the product.
///
/// *Breaks if* a placement fails to relate the two spaces, or the
/// product gathers a space other than the world.
#[test]
fn a_copy_in_a_new_space_placed_into_the_world_is_in_the_product() {
    let doc = ProfileDoc::empty(DocumentId::derive("dprime1-new-space"), Tol::witness());
    let (doc, a) = block(doc, 0.0);
    let (doc, space) = insert(doc, Node::Space);
    let space = doc.output(space, 0).expect("a new space defines its space");
    let (doc, inner) = step(
        doc,
        DocEdit::place(
            vec![a.into()],
            Operand::Var(space),
            editor_core::stand().bundle(),
        ),
    );
    let inner = inner.expect("the placement mints");
    assert_eq!(doc.node_space(inner), Some(space));
    let o = EvalOptions::default();
    assert!(
        editor_core::product(&doc, &run(&doc, &o), Tol::witness()).is_err(),
        "nothing is in the world yet"
    );

    let (doc, outer) = fixture::place(doc, inner);
    assert_eq!(doc.node_space(outer), Some(VarId::WORLD));
    let product = editor_core::product(&doc, &run(&doc, &o), Tol::witness())
        .expect("the world holds the copy");
    assert_eq!(product.solids().count(), 1, "`a`, once");
}

/// **Test 18, the census half: no evaluation of a corpus document names
/// a space other than a `Space` variable.** Every node's derived space
/// is a live variable of kind `Space`, and every `SpaceMismatch` names
/// two.
///
/// *Breaks if* a space is derived from anything but a `Space` output.
#[test]
fn every_corpus_space_is_a_space_variable() {
    let o = EvalOptions::default();
    for d in crate::corpus::documents() {
        let is_space = |var: VarId| {
            var == VarId::WORLD || d.doc.var(var).is_some_and(|v| v.kind() == VarKind::Space)
        };
        for (node, space) in d.doc.spaces() {
            assert!(is_space(space), "{}: {node:?} is in {space:?}", d.name);
        }
        let ev = run(&d.doc, &o);
        for id in d.doc.ids() {
            if let Some(NodeErrorKind::SpaceMismatch { a, b }) = ev.node_error(id).map(|e| &e.kind)
            {
                assert!(is_space(*a) && is_space(*b), "{}: {id:?}", d.name);
            }
        }
    }
}
