//! **The gather's at-rest refusal names the roots whose bodies fail.**
//!
//! The product is gated once, as the aggregate; a refusal re-gates each
//! source body on its own and lists every one that fails, by root and
//! output index (`ProductError::RootInvalid`). These rows hold the
//! attribution, over the shapes a document can hand the gather: one
//! source of one solid, two sources, one source of several solids, and
//! two sources whose defects the aggregate's own findings cannot both
//! report.
//!
//! The invalid bodies are evaluated blocks, written into the
//! evaluation's own value slot, which is the one place the gather reads
//! a source body from. Two kinds: one face's sense bit flipped
//! (`Body::flipped_face_sense_for_tests`), and the whole body reversed
//! (`Body::revert`), which is inside out. Both are key-for-key, so every
//! key the name table and contact records hold still resolves, and both
//! are planar, so the tier-3 refusal is exact at every ε.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::{
    BooleanValue, CancelToken, Datum, DocEdit, DocumentId, EvalOptions, Evaluation, Expr, Frame,
    Node, NodeResult, PatternKind, ProductError, ProfileDoc, RecipeNodeId, SourceFinding,
    SplitHalf, SplitSide, ValuePayload, evaluate, product_recorded,
};
use fixture::resolver::{PartStore, with_resolver};
use fixture::{insert, len, on_frame, scl, square, step};
use geom_core::Tol;
use std::sync::Arc;
use test_utils::source::{ItemBody, blanked, code_only, item_body, required_matches};
use topo::{Body, ValidationError};

const PRODUCT: &str = include_str!("../src/product.rs");

/// **A successful gather runs the at-rest gate once.** The gate is
/// called in exactly two places: on the aggregate, and inside
/// `attribute_at_rest`, which is called only from the block that the
/// aggregate's refusal enters. A per-source pass re-inserted before the
/// graft would be a third call, and one moved out of the refusal block
/// would leave the block; either reds here. No behavioural row can see
/// this: an extra pass on the same geometry changes no verdict.
#[test]
fn the_gather_gates_the_aggregate_once_and_sources_only_on_refusal() {
    let code = blanked(code_only, "editor-core/src/product.rs", PRODUCT);
    let what = "editor-core/src/product.rs (code view)";
    let gates = required_matches(&code, what, "T::gate_at_rest(");
    assert_eq!(gates.len(), 2, "{what}: the gate is called in two places");
    let on_aggregate = required_matches(
        &code,
        what,
        "if let Err(errors) = T::gate_at_rest(&aggregate",
    );
    assert_eq!(on_aggregate.len(), 1, "{what}: one aggregate gate");
    let ItemBody::Body(refusal) = item_body(&code, on_aggregate[0]) else {
        panic!("{what}: the aggregate gate's `if let` has no block");
    };
    let attribute = required_matches(&code, what, "fn attribute_at_rest");
    assert_eq!(attribute.len(), 1, "{what}: one attribution fn");
    let ItemBody::Body(attribution) = item_body(&code, attribute[0]) else {
        panic!("{what}: `attribute_at_rest` has no body");
    };
    assert!(
        gates
            .iter()
            .all(|at| *at == on_aggregate[0] + "if let Err(errors) = ".len()
                || attribution.contains(at)),
        "{what}: a gate call sits outside the aggregate gate and the attribution fn"
    );
    let calls: Vec<usize> = required_matches(&code, what, "attribute_at_rest(")
        .into_iter()
        .filter(|at| !code[..*at].ends_with("fn "))
        .collect();
    assert!(
        calls.len() == 1 && refusal.contains(&calls[0]),
        "{what}: `attribute_at_rest` is called once, from the aggregate's refusal block"
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

/// `node`'s body, as the evaluation holds it.
fn slot(ev: &mut Evaluation<f64>, node: RecipeNodeId) -> &mut Arc<Body<f64>> {
    let Some(NodeResult::Ok(value)) = ev.nodes.get_mut(&node) else {
        panic!("node {node:?} evaluated to no value");
    };
    match &mut value.payload {
        ValuePayload::Body(body) | ValuePayload::Boolean(BooleanValue::Body { body, .. }) => body,
        other => panic!("node {node:?} is not a body: {other:?}"),
    }
}

/// Writes a copy of `node`'s body with one face's sense flipped into
/// its value slot, and returns how many solids that body holds.
fn flip_one_face(ev: &mut Evaluation<f64>, node: RecipeNodeId) -> usize {
    let body = slot(ev, node);
    *body = flipped(body);
    body.solids().count()
}

/// Writes the reversed, inside-out copy of `node`'s body into its value
/// slot.
fn turn_inside_out(ev: &mut Evaluation<f64>, node: RecipeNodeId) {
    let body = slot(ev, node);
    let reversed = body.revert().expect("an evaluated block reverses");
    *body = Arc::new(reversed);
}

/// The refusal's findings, as (root, output) pairs, in list order.
fn named(err: &ProductError) -> Vec<(RecipeNodeId, u32)> {
    let ProductError::RootInvalid { findings } = err else {
        panic!("want the per-root refusal, got {err:?}");
    };
    assert!(
        findings.iter().all(|f| !f.errors.is_empty()),
        "every listed root carries its findings: {findings:?}"
    );
    findings.iter().map(|f| (f.node, f.output)).collect()
}

/// **One source of one solid**: the part and the aggregate are the
/// same body, and the refusal names that root rather than the product.
#[test]
fn a_lone_single_solid_source_is_named() {
    let (doc, a) = block(ProfileDoc::empty_derived("per-part-1", Tol::witness()), 0.0);
    assert_eq!(doc.roots(), &[a][..]);
    let mut ev = run(&doc);
    assert_eq!(flip_one_face(&mut ev, a), 1);
    let err = product_recorded(&doc, &ev, Tol::witness()).expect_err("an invalid root refuses");
    assert_eq!(named(&err), [(a, 0)]);
}

/// **Two single-solid sources, one inside-out**: the refusal names the
/// failing root and not the clean one beside it.
#[test]
fn two_sources_name_only_the_failing_one() {
    let (doc, a) = block(ProfileDoc::empty_derived("per-part-2", Tol::witness()), 0.0);
    let (doc, b) = block(doc, 5.0);
    assert_eq!(doc.roots(), &[a, b][..]);
    let mut ev = run(&doc);
    assert_eq!(flip_one_face(&mut ev, b), 1);
    let err = product_recorded(&doc, &ev, Tol::witness()).expect_err("an invalid root refuses");
    assert_eq!(named(&err), [(b, 0)], "the refusal names the inverted root");
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
/// solids — **with one inside-out**. The source is re-gated whole, as
/// one body, and named: the ordinary case, with nothing special about
/// a source's solid count.
#[test]
fn a_lone_multi_solid_source_is_named() {
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
        flip_one_face(&mut ev, source),
        2,
        "the one source holds two solids"
    );
    let err = product_recorded(&doc, &ev, Tol::witness()).expect_err("an invalid root refuses");
    assert_eq!(named(&err), [(source, 0)]);
}

/// **The cascade the re-gate exists for.** Root `a` has one face's
/// sense flipped (a check 1–6 finding, `LoopRoleInverted`); root `b` is
/// wholly inside out (a check 7 finding, `NegativeVolume`). Gated as
/// one body, the battery's check 7 never runs once an earlier check has
/// found anything, so the aggregate's own list reports `a`'s defect and
/// not `b`'s. The refusal names BOTH, each with its own finding.
#[test]
fn a_defect_that_stops_check_7_does_not_hide_another_roots_inside_out_body() {
    let (doc, a) = block(ProfileDoc::empty_derived("per-part-4", Tol::witness()), 0.0);
    let (doc, b) = block(doc, 5.0);
    assert_eq!(doc.roots(), &[a, b][..]);
    let mut ev = run(&doc);
    flip_one_face(&mut ev, a);
    turn_inside_out(&mut ev, b);

    let is_inverted_loop =
        |e: &ValidationError| matches!(e, ValidationError::LoopRoleInverted { .. });
    let is_inside_out = |e: &ValidationError| matches!(e, ValidationError::NegativeVolume { .. });

    // The precondition this row discriminates on: the two bodies grafted
    // and gated as one report `a`'s defect only. Red here means the
    // battery now reports every solid's check 7 whatever the others
    // found, and this row no longer tells re-gating from reading the
    // aggregate's list.
    let mut aggregate = Body::new();
    for node in [a, b] {
        topo::graft_disjoint_all(&mut aggregate, slot(&mut ev, node), Tol::witness())
            .expect("the two blocks are disjoint");
    }
    let together =
        topo::validate_geometric(&aggregate, Tol::witness()).expect_err("the aggregate is invalid");
    assert!(
        together.iter().any(is_inverted_loop) && !together.iter().any(is_inside_out),
        "the aggregate's own list hides b's inside-out body: {together:?}"
    );

    let err = product_recorded(&doc, &ev, Tol::witness()).expect_err("invalid roots refuse");
    assert_eq!(named(&err), [(a, 0), (b, 0)], "both roots are named");
    let ProductError::RootInvalid { findings } = &err else {
        unreachable!("`named` read this arm");
    };
    let [
        SourceFinding { errors: of_a, .. },
        SourceFinding { errors: of_b, .. },
    ] = findings.as_slice()
    else {
        unreachable!("`named` counted two");
    };
    assert!(
        of_a.iter().any(is_inverted_loop),
        "a's own finding: {of_a:?}"
    );
    assert!(of_b.iter().any(is_inside_out), "b's own finding: {of_b:?}");
    let text = err.to_string();
    assert!(
        text.contains(&format!("root {} output 0", a.0))
            && text.contains(&format!("root {} output 0", b.0)),
        "the message names both roots: {text}"
    );
}

/// A copy of `body` with one face's sense flipped: invalid at rest.
fn flipped(body: &Body<f64>) -> Arc<Body<f64>> {
    let (face, _) = body.faces().next().expect("the body has a face");
    let out = body
        .flipped_face_sense_for_tests(face)
        .expect("the face is live");
    assert!(
        topo::validate_geometric(&out, Tol::witness()).is_err(),
        "the flipped body is invalid at rest on its own (anti-vacuity)"
    );
    Arc::new(out)
}

/// **A pattern root names each failing INSTANCE by its output index.**
/// Three instances, the first and the last flipped: the refusal lists
/// outputs 0 and 2 of the one root, and not 1.
#[test]
fn a_pattern_root_names_each_failing_instance() {
    let (doc, a) = block(ProfileDoc::empty_derived("per-part-5", Tol::witness()), 0.0);
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: a,
            count: Expr::count(3),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(3.0),
            },
        },
    );
    assert_eq!(doc.roots(), &[pattern][..]);
    let mut ev = run(&doc);
    let Some(NodeResult::Ok(value)) = ev.nodes.get_mut(&pattern) else {
        panic!("the pattern evaluated to no value");
    };
    let ValuePayload::Instances(bodies) = &mut value.payload else {
        panic!("the pattern is not an instance list: {:?}", value.payload);
    };
    assert_eq!(bodies.len(), 3);
    for i in [0, 2] {
        bodies[i] = flipped(&bodies[i]);
    }
    let err = product_recorded(&doc, &ev, Tol::witness()).expect_err("invalid instances refuse");
    assert_eq!(named(&err), [(pattern, 0), (pattern, 2)]);
    let text = err.to_string();
    assert!(
        text.starts_with("product: 1 root not valid at rest:"),
        "one root, however many of its outputs fail: {text}"
    );
}

/// **A split root names its BELOW half by that half's output index**
/// (`SplitHalf::Below.output_body()`, 1), not by its position in the
/// gather's list.
#[test]
fn a_split_root_names_its_below_half_as_output_1() {
    let (doc, a) = block(ProfileDoc::empty_derived("per-part-6", Tol::witness()), 0.0);
    let (doc, tool) = insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(0.0), len(0.0), len(0.5)],
            normal: [scl(0.0), scl(0.0), scl(1.0)],
        }),
    );
    let (doc, split) = insert(doc, Node::Split { target: a, tool });
    assert_eq!(doc.roots(), &[split][..]);
    let mut ev = run(&doc);
    let Some(NodeResult::Ok(value)) = ev.nodes.get_mut(&split) else {
        panic!("the split evaluated to no value");
    };
    let ValuePayload::Split {
        below: SplitSide::Body(below),
        above: SplitSide::Body(_),
    } = &mut value.payload
    else {
        panic!("the split has two halves: {:?}", value.payload);
    };
    *below = flipped(below);
    let err = product_recorded(&doc, &ev, Tol::witness()).expect_err("an invalid half refuses");
    assert_eq!(SplitHalf::Below.output_body(), 1);
    assert_eq!(named(&err), [(split, SplitHalf::Below.output_body())]);
}
