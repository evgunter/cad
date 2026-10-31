//! Recipe layer: a REST union — persistence round-trip of a glued
//! body and the naming-key stability row.
//!
//! The document: two stacked plates (full-face REST contact at z = 1,
//! four flush wall pairs) unioned. The kernel's join glues the mate;
//! the recipe layer must carry it like any seamed boolean:
//!
//! - the evaluation is green with the exact dyadic volume (8);
//! - re-evaluating is BIT-identical (name tables, arenas, content
//!   keys — the naming-key stability row);
//! - the persisted document (snapshot AND edit-log shapes) loads and
//!   replays to the same bit-identical evaluation (persistence
//!   round-trip of a glued body).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::insert;
use editor_core::{
    BooleanValue, CancelToken, EvalOptions, Node, ProfileDoc, RecipeNodeId, ValuePayload, evaluate,
    load, save,
};
use geom_core::Tol;

/// The stacked-plates REST document: plates + their union.
/// Returns (doc, union node).
fn rest_doc() -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("m5_s1_rest_declare", Tol::witness());
    let (doc, a) = crate::blocks::block(doc, (0.0, 2.0), (0.0, 2.0), 1.0);
    let (doc, b) = crate::blocks::block_on(doc, a, (0.0, 2.0), (0.0, 2.0), 1.0, 1.0);
    let (doc, u) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
        },
    );
    (doc, u)
}

/// The whole-evaluation bit fingerprint (the m4_pr6_roundtrip
/// discipline: `Debug` prints floats shortest-round-trip, bit-faithful
/// for the finite values documents carry).
fn fingerprint(doc: &ProfileDoc) -> String {
    let ev = evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    format!("{:?}|{:?}|{:?}", ev.order, ev.nodes, ev.appearance)
}

/// The glued union evaluates green: one seamed body, exact volume 8,
/// rest records consumed (3′ ≡ tier 3 on the result).
#[test]
fn rest_union_evaluates_green() {
    let (doc, u) = rest_doc();
    let ev = evaluate::<f64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    let value = ev.value(u).expect("union evaluated");
    let contacts = &value.carry.contacts;
    let ValuePayload::Boolean(BooleanValue::Body { body, kind }) = &value.payload else {
        panic!("expected a boolean body, got {}", value.payload.kind_name());
    };
    assert_eq!(*kind, topo::BooleanResultKind::Seamed);
    assert_eq!(
        topo::test_support::pinned_volume(
            "the measured body",
            &topo::test_support::finished(
                "the measured body",
                topo::Body::clone(body.body()),
                Tol::witness()
            ),
            Tol::witness()
        )
        .volume,
        8.0
    );
    assert_eq!(
        topo::validate_pseudomanifold(body.body(), contacts, Tol::witness()),
        Ok(())
    );
    assert!(
        contacts.vertex_vertex().next().is_none() && contacts.vertex_face().next().is_none(),
        "REST records consumed into seam structure: {contacts:?}"
    );
}

/// Naming-key stability: two independent evaluations of the glued
/// document are bit-identical — name tables, bodies, verdicts,
/// content keys (D9).
#[test]
fn rest_union_rerun_is_bit_identical() {
    let (doc, _) = rest_doc();
    assert_eq!(fingerprint(&doc), fingerprint(&doc));
}

/// Persistence round-trip of the glued body's document: both persisted
/// shapes (bare snapshot; the canonical save of the current state)
/// reload through `load`'s replay doors and evaluate bit-identically
/// to the original.
#[test]
fn rest_union_persistence_round_trip() {
    let (doc, _) = rest_doc();
    let expected = fingerprint(&doc);
    let text = save(&doc, &[], Tol::witness()).expect("save");
    let loaded = load(&text, Tol::witness()).expect("load");
    assert_eq!(loaded.doc, doc, "replayed document equals the original");
    assert_eq!(
        fingerprint(&loaded.doc),
        expected,
        "persisted → replayed evaluation is bit-identical"
    );
    // Save AGAIN from the loaded state: canonical bytes are stable.
    let text2 = save(&loaded.doc, &[], Tol::witness()).expect("re-save");
    assert_eq!(text, text2, "canonical persisted bytes are stable");
}
