//! **Review probes for `edit/orphaned-declare-report` (lane `orphan-rv`).**
//!
//! Not the unit's rows: these are the reviewer's, written to try to
//! break the claims the PR makes about
//! `Maintenance::OrphanedDeclare`. They are kept because each one
//! pins a fact the unit's own rows leave unpinned.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::docm7_union_declare::{block, declared_union, flush_pairs};
use crate::fixture;
use editor_core::{
    BooleanOp, DocEdit, DocumentId, Maintenance, Node, ProfileDoc, RecipeNodeId, SplitError, apply,
    cascade_delete_order, split,
};
use fixture::insert;
use geom_core::Tol;

fn delete(doc: &ProfileDoc, id: RecipeNodeId) -> editor_core::Applied<editor_core::ProfileProgram> {
    apply(doc, &DocEdit::DeleteNode { id }, Tol::witness()).expect("the delete is legal")
}

/// **A `Boolean` and a `Union` sharing one `Declare` is the same row,
/// not a second.** The unit's multi-consumer row authors two
/// `Union`s; the door reads `Node::inputs`, so the mixed pair has to
/// behave identically. Deleting either one alone reports nothing;
/// the second reports the orphan once.
#[test]
fn rv_a_boolean_and_a_union_sharing_one_declare_are_one_consumer_count() {
    let doc = ProfileDoc::empty_derived("rv_orphan_mixed_consumers", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, the_union, decl) = declared_union(doc, &[a, b], flush_pairs((a, a), (b, b)));
    let (doc, the_boolean) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b,
            declare: Some(decl),
        },
    );

    let after_union = delete(&doc, the_union);
    assert_eq!(
        after_union.maintenance,
        Vec::new(),
        "the boolean still consumes the declaration"
    );
    let after_boolean = delete(&after_union.doc, the_boolean);
    assert_eq!(
        after_boolean.maintenance,
        vec![Maintenance::OrphanedDeclare { declare: decl }],
        "the last consumer is the last consumer whatever kind it is"
    );

    // …and the other order, so neither kind is privileged.
    let after_boolean = delete(&doc, the_boolean);
    assert_eq!(after_boolean.maintenance, Vec::new());
    let after_union = delete(&after_boolean.doc, the_union);
    assert_eq!(
        after_union.maintenance,
        vec![Maintenance::OrphanedDeclare { declare: decl }]
    );
}

/// **At most ONE orphan row is producible per accepted delete, under
/// the node vocabulary as it stands.** `Node::declare_input` is an
/// `Option` and no node kind holds two, so the `Vec`, the
/// `doc.order()` walk and the "in the document's node order" clause
/// of `Applied::maintenance` all describe a set whose size is
/// provably 0 or 1. This probe is the guard that would go red the day
/// that stops being true — at which point the ordering clause starts
/// carrying weight and needs a row of its own.
#[test]
fn rv_no_delete_can_report_two_orphans_today() {
    let doc = ProfileDoc::empty_derived("rv_orphan_at_most_one", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, union, decl) = declared_union(doc, &[a, b], flush_pairs((a, a), (b, b)));
    // A second declaration, consumerless, in the same document.
    let (doc, spare) = insert(doc, Node::declare_rest(flush_pairs((a, a), (b, b))));
    let applied = delete(&doc, union);
    assert_eq!(
        applied.maintenance,
        vec![Maintenance::OrphanedDeclare { declare: decl }],
        "one delete, one declare edge, one row"
    );
    assert!(
        applied.doc.node(spare).is_some(),
        "the other declaration is nobody's business here"
    );
    for id in applied.doc.order() {
        let node = applied.doc.node(*id).expect("live");
        let declares = node
            .inputs()
            .iter()
            .filter(|i| matches!(applied.doc.node(**i), Some(Node::Declare { .. })))
            .count();
        assert!(
            declares <= 1,
            "a node with two declare inputs would make the order clause load-bearing"
        );
    }
}

/// **The transient's subject is always in the doomed set**, so a
/// cascade-aware caller CAN cancel it without giving `apply` cascade
/// knowledge. The deviation's argument is that `apply` cannot tell
/// the two cases apart, which is true; this probe is the other half —
/// that the information needed to cancel exists one level up, at
/// `cascade_delete_order`'s caller (`viewer`'s `Session::delete_node`
/// composes exactly this loop).
#[test]
fn rv_the_orphan_transient_is_cancellable_at_the_cascade_door() {
    let doc = ProfileDoc::empty_derived("rv_orphan_cancellable", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, _union, decl) = declared_union(doc, &[a, b], flush_pairs((a, a), (b, b)));

    let doomed = cascade_delete_order(&doc, decl);
    let mut walked = doc;
    let mut rows: Vec<Maintenance> = Vec::new();
    for id in &doomed {
        let applied = delete(&walked, *id);
        rows.extend(applied.maintenance);
        walked = applied.doc;
    }
    assert_eq!(rows, vec![Maintenance::OrphanedDeclare { declare: decl }]);
    // Every row the run produced names a node the same run deleted,
    // which is the filter a cascade door would apply.
    let net: Vec<&Maintenance> = rows
        .iter()
        .filter(|row| match row {
            Maintenance::OrphanedDeclare { declare } => !doomed.contains(declare),
            _ => true,
        })
        .collect();
    assert!(
        net.is_empty(),
        "the cascade's NET maintenance is empty, and nothing in the tree computes it"
    );
}

/// **`split`'s closure check refuses the declare edge in BOTH
/// directions.** The unit asserts the cut that keeps the declaration
/// and moves its union; the mirror — moving the declaration and
/// keeping its union — is the one that would leave the PART with a
/// dangling edge, and is refused for the same reason. Together they
/// are what makes "no refactoring can produce an orphan" true.
#[test]
fn rv_split_refuses_the_declare_edge_in_both_directions() {
    use std::collections::BTreeSet;
    let doc = ProfileDoc::empty_derived("rv_orphan_split_both", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, union, decl) = declared_union(doc, &[a, b], flush_pairs((a, a), (b, b)));

    let just_the_declaration: BTreeSet<RecipeNodeId> = [decl].into_iter().collect();
    match split(
        &doc,
        &just_the_declaration,
        DocumentId::derive("rv-decl-only"),
        Tol::witness(),
    ) {
        Err(SplitError::SeveredEdge {
            consumer,
            input,
            consumer_is_cut,
        }) => {
            assert_eq!((consumer, input), (union, decl));
            assert!(!consumer_is_cut, "the consumer is the one left behind here");
        }
        other => panic!("expected SeveredEdge, got {other:?}"),
    }

    // The whole document moves: the declare edge is inside the cut,
    // so the split is accepted and nothing is orphaned or stranded.
    let everything: BTreeSet<RecipeNodeId> = doc.order().iter().copied().collect();
    let moved = split(
        &doc,
        &everything,
        DocumentId::derive("rv-decl-all"),
        Tol::witness(),
    )
    .expect("a cut closed under the DAG is accepted");
    let _ = moved;
}

/// **No edit but `DeleteNode` can drop a `declare` edge**, which is
/// what makes the delete door the only place the report is owed.
/// `SetMembers` is the vocabulary's one rewire of a live node's
/// inputs and it leaves `declare` exactly as it was.
#[test]
fn rv_set_members_cannot_orphan_a_declaration() {
    let doc = ProfileDoc::empty_derived("rv_orphan_set_members", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block(doc, (1.0, 2.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, union, decl) = declared_union(doc, &[a, b], flush_pairs((a, a), (b, b)));

    let applied = apply(
        &doc,
        &DocEdit::SetMembers {
            node: union,
            members: vec![a, b, c],
        },
        Tol::witness(),
    )
    .expect("the member list is replaceable");
    assert_eq!(
        applied.maintenance,
        Vec::new(),
        "a rewire reports nothing, and there is nothing to report"
    );
    assert_eq!(
        applied.doc.node(union).expect("live").declare_input(),
        Some(decl),
        "SetMembers leaves the declare edge as it was, so it cannot orphan"
    );
}

/// **What the orphaned declaration BECOMES**: `roots::on_delete`
/// re-roots the deleted node's inputs that its departure turned into
/// sinks, and it does not ask what kind they are — so the same delete
/// that reports the orphan also registers the `Declare` as a product
/// ROOT (A10). Pre-existing, not this unit's doing, but it is the
/// fact the arm's `Display` sentence ("nothing reads the
/// declaration") and the `pncad.pyi` paragraph ("no longer read by
/// anything") are written against.
#[test]
fn rv_the_orphaned_declaration_is_re_rooted_by_the_same_delete() {
    let doc = ProfileDoc::empty_derived("rv_orphan_roots", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, union, decl) = declared_union(doc, &[a, b], flush_pairs((a, a), (b, b)));
    assert_eq!(doc.roots(), [union], "the union is the document's product");

    let applied = delete(&doc, union);
    assert_eq!(
        applied.maintenance,
        vec![Maintenance::OrphanedDeclare { declare: decl }]
    );
    assert!(
        applied.doc.roots().contains(&decl),
        "the declaration the delete reported as unread is now a product root"
    );
}
