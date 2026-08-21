//! M5 S1, recipe layer: the declared-REST union zip driven by recipe
//! intent (`Declare` → `Boolean{declare}`) — persistence round-trip of
//! a glued body and the naming-key stability row.
//!
//! The document: two stacked plates (full-face REST contact at z = 1)
//! whose recipe DECLARES the contact caps and the four flush wall
//! pairs, then unions them. The kernel's M5 S1 lane zips the mate; the
//! recipe layer must carry it like any seamed boolean:
//!
//! - the evaluation is green with the exact dyadic volume (8);
//! - re-evaluating is BIT-identical (name tables, arenas, content
//!   keys — the naming-key stability row);
//! - the persisted document (snapshot AND edit-log shapes) loads and
//!   replays to the same bit-identical evaluation (persistence
//!   round-trip of a glued body).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{
    BooleanOp, BooleanValue, CancelToken, CapEnd, EntityKind, EvalOptions, Node, ProfileDoc,
    RecipeNodeId, RoleSeg, StableName, ValuePayload, evaluate, load, save,
};
use fixture::{desc, insert, len, wall};
use geom_core::Tol;

fn fname(node: RecipeNodeId, seg: RoleSeg) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node,
        path: vec![seg],
    }
}

/// An axis-aligned (0..2)² block at height z0, extruded dz.
fn block(doc: ProfileDoc, z0: f64, dz: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = insert(
        doc,
        Node::Profile(desc(
            [0.0, 0.0, z0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)]],
        )),
    );
    insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(dz),
        },
    )
}

/// The stacked-plates REST document: plates + Declare + union.
/// Returns (doc, union node).
fn rest_doc() -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("m5_s1_rest_declare", Tol::witness());
    let (doc, a) = block(doc, 0.0, 1.0);
    let (doc, b) = block(doc, 1.0, 1.0);
    // The author's intent, stated: the contact pair (A's top cap on
    // B's bottom cap) plus the four flush wall pairs (the same-plane
    // sides the output stage merges).
    let pairs = vec![
        (
            fname(a, RoleSeg::Cap(CapEnd::Top)),
            fname(b, RoleSeg::Cap(CapEnd::Bottom)),
        ),
        (fname(a, wall(0)), fname(b, wall(0))),
        (fname(a, wall(1)), fname(b, wall(1))),
        (fname(a, wall(2)), fname(b, wall(2))),
        (fname(a, wall(3)), fname(b, wall(3))),
    ];
    let (doc, decl) = insert(doc, Node::declare_rest(pairs));
    let (doc, u) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b,
            declare: Some(decl),
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
fn declared_rest_union_evaluates_green() {
    let (doc, u) = rest_doc();
    let ev = evaluate::<f64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    let value = ev.value(u).expect("union evaluated");
    let ValuePayload::Boolean(BooleanValue::Body {
        body,
        kind,
        contacts,
    }) = &value.payload
    else {
        panic!("expected a boolean body, got {}", value.payload.kind_name());
    };
    assert_eq!(*kind, topo::BooleanResultKind::Seamed);
    assert_eq!(
        topo::mass_properties(body, Tol::witness()).unwrap().volume,
        8.0
    );
    assert_eq!(
        topo::validate_pseudomanifold(body, contacts, Tol::witness()),
        Ok(())
    );
    assert!(
        contacts.vv.is_empty() && contacts.a_on_b.is_empty() && contacts.b_on_a.is_empty(),
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
