//! **The gather's per-source gate: the call to `topo::per_part_gate_owed`
//! is there, and the answer it gets is the one the gather acts on.**
//!
//! Two kinds of row, because the two failures differ:
//!
//! - **The call is present** (source rows, code view). A pass that
//!   spelled its own threshold again (`total_solids > 1`) would behave
//!   identically today and give the policy a second home, which no
//!   behavioural row can see. What these rows hold is that the call is
//!   made and that the per-source gate sits inside the block it guards —
//!   not what it is asked with.
//! - **What it is asked with, and what follows** (behaviour rows). An
//!   inverted condition, an `|| true`, or counting SOURCES where the
//!   policy counts solids all keep the call and change the refusal. The
//!   three documents below sit on both sides of the threshold, and the
//!   third is the one where sources and solids disagree.
//!
//! The invalid body is an evaluated block with one face's sense bit
//! flipped (`Body::flipped_face_sense_for_tests`): a clone, so every key
//! the name table and contact records hold still resolves, and planar,
//! so the tier-3 refusal is exact at every ε. It is written into the
//! evaluation's own value slot, which is the one place the gather reads
//! a source body from.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::{
    BooleanValue, CancelToken, DocEdit, DocumentId, EvalOptions, Evaluation, Frame, Node,
    NodeResult, ProductError, ProfileDoc, RecipeNodeId, ValuePayload, evaluate, product_recorded,
};
use fixture::resolver::{PartStore, with_resolver};
use fixture::{insert, len, on_frame, square, step};
use geom_core::Tol;
use std::sync::Arc;
use test_utils::source::{ItemBody, blanked, code_only, item_body, required_matches};

const PRODUCT: &str = include_str!("../src/product.rs");

/// The one `if` whose condition calls the policy, and the per-source
/// gate call inside its block — and nowhere else. The per-source call
/// is the one on a SOURCE body (`T::gate_at_rest(body`); the
/// aggregate's own call, on `&aggregate`, is not under the policy and
/// is not matched.
#[test]
fn the_gather_calls_the_per_part_policy_around_its_per_source_gate() {
    let code = blanked(code_only, "editor-core/src/product.rs", PRODUCT);
    let what = "editor-core/src/product.rs (code view)";
    let heads = required_matches(&code, what, "if topo::per_part_gate_owed(");
    assert_eq!(
        heads.len(),
        1,
        "{what}: the gather calls the per-part policy at exactly one `if`"
    );
    let ItemBody::Body(block) = item_body(&code, heads[0]) else {
        panic!("{what}: the policy's `if` has no block");
    };
    let gates = required_matches(&code, what, "T::gate_at_rest(body");
    assert!(
        gates.iter().all(|at| block.contains(at)),
        "{what}: the per-source gate is called outside the block `topo::per_part_gate_owed` \
         guards"
    );
}

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// A unit block centred at `cx` on the z = 0 plane: one solid.
fn block(doc: ProfileDoc, cx: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(cx, 0.0, 0.5)],
    );
    insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    )
}

/// Writes an inside-out copy of `node`'s body into its value slot and
/// returns how many solids that body holds.
fn invert(ev: &mut Evaluation<f64>, node: RecipeNodeId) -> usize {
    let Some(NodeResult::Ok(value)) = ev.nodes.get_mut(&node) else {
        panic!("node {node:?} evaluated to no value");
    };
    let body = match &mut value.payload {
        ValuePayload::Body(body) | ValuePayload::Boolean(BooleanValue::Body { body, .. }) => body,
        other => panic!("node {node:?} is not a body: {other:?}"),
    };
    let (face, _) = body.faces().next().expect("the body has a face");
    let flipped = body
        .flipped_face_sense_for_tests(face)
        .expect("the face is live");
    assert!(
        topo::validate_geometric(&flipped, Tol::witness()).is_err(),
        "the flipped body is invalid at rest on its own (anti-vacuity)"
    );
    let solids = flipped.solids().count();
    *body = Arc::new(flipped);
    solids
}

/// **One source of one solid**: the per-part gate is not owed, so the
/// inside-out block is refused by the AGGREGATE gate. Red under an
/// inverted or always-true condition, which would gate the source and
/// answer `SolidInvalid`.
#[test]
fn a_lone_single_solid_source_is_refused_as_the_product() {
    let (doc, a) = block(ProfileDoc::empty_derived("per-part-1", Tol::witness()), 0.0);
    assert_eq!(doc.roots(), &[a][..]);
    let mut ev = run(&doc);
    assert_eq!(invert(&mut ev, a), 1);
    match product_recorded(&doc, &ev, Tol::witness()) {
        Err(ProductError::ProductInvalid { .. }) => {}
        other => panic!("want the aggregate gate's refusal, got {other:?}"),
    }
}

/// **Two single-solid sources, one inside-out**: the per-part gate is
/// owed, and its refusal names the inverted root. Red under an inverted
/// condition, which would skip the per-source pass and answer
/// `ProductInvalid`.
#[test]
fn two_single_solid_sources_name_the_invalid_one() {
    let (doc, a) = block(ProfileDoc::empty_derived("per-part-2", Tol::witness()), 0.0);
    let (doc, b) = block(doc, 5.0);
    assert_eq!(doc.roots(), &[a, b][..]);
    let mut ev = run(&doc);
    assert_eq!(invert(&mut ev, b), 1);
    match product_recorded(&doc, &ev, Tol::witness()) {
        Err(ProductError::SolidInvalid { node, .. }) => {
            assert_eq!(node, b, "the refusal names the inverted root");
        }
        other => panic!("want the per-source gate's refusal, got {other:?}"),
    }
}

/// A one-solid part document, for the store the sub-assembly reads.
fn part(label: &str) -> ProfileDoc {
    block(
        ProfileDoc::empty(DocumentId::derive(label), Tol::witness()),
        0.0,
    )
    .0
}

/// **One source holding several solids** — an instantiated
/// sub-assembly of two parts, which evaluates to one body of two
/// solids — **with one inside-out**. The policy counts SOLIDS, so the
/// per-part gate is owed here and gates the source whole:
/// `SolidInvalid`, naming the instantiation. This pins TODAY's
/// behaviour, where the part gated is a source and the aggregate is the
/// same geometry. Counting sources instead would answer
/// `ProductInvalid`; that is an open decision
/// (`work/gather/product-per-part-gate-counts-solids-but-gates-sources.md`),
/// and this row is what moves if it is taken.
#[test]
fn a_lone_multi_solid_source_is_gated_as_a_part_today() {
    let mut store = PartStore::default();
    let p = store.insert(part("per-part-3-p"), Tol::witness());
    let mut sub = ProfileDoc::empty(DocumentId::derive("per-part-3-b"), Tol::witness());
    for dx in [0.0, 3.0] {
        let (next, id) = insert(sub, Node::instantiate_part(p));
        let (next, _) = step(
            next,
            DocEdit::SetPlacement {
                node: id,
                frame: Frame::translation([dx, 0.0, 0.0]),
            },
        );
        sub = next;
    }
    let b = store.insert(sub, Tol::witness());
    let (doc, source) = insert(
        ProfileDoc::empty(DocumentId::derive("per-part-3-a"), Tol::witness()),
        Node::instantiate_part(b),
    );
    assert_eq!(doc.roots(), &[source][..], "one source");
    let mut ev = fixture::run(&doc, &with_resolver(store));
    assert_eq!(
        invert(&mut ev, source),
        2,
        "the one source holds two solids"
    );
    match product_recorded(&doc, &ev, Tol::witness()) {
        Err(ProductError::SolidInvalid { node, .. }) => assert_eq!(node, source),
        other => panic!("want the per-source gate's refusal, got {other:?}"),
    }
}
