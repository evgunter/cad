//! **The gather's at-rest refusal names the placements whose copies fail.**
//!
//! The product is gated once, as the aggregate; a refusal re-gates each
//! source body on its own and lists every one that fails, by placement and
//! output index (`ProductError::PieceInvalid`). These rows hold the
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
    BooleanValue, CancelToken, DocumentId, EvalOptions, Evaluation, Frame, Node, NodeResult,
    ProductError, ProfileDoc, RecipeNodeId, SourceFinding, ValuePayload, evaluate,
    product_recorded,
};
use fixture::resolver::{PartStore, with_resolver};
use fixture::{insert, len, on_frame, square};
use geom_core::Tol;
use std::sync::Arc;
use test_utils::source::{ItemBody, blanked, code_only, item_body, required_matches};
use topo::{Body, ValidationError};

const PRODUCT: &str = include_str!("../src/product.rs");

/// **A successful gather runs the at-rest gate once.** The gate is
/// called in exactly two places: on the aggregate (the verdict-keeping
/// form), and inside `attribute_at_rest`, which is called only from the
/// arm that the aggregate's refusal enters. A per-source pass
/// re-inserted before the graft would be a third call, and one moved out
/// of the refusal arm would leave the arm; either reds here. No
/// behavioural row can see this: an extra pass on the same geometry
/// changes no verdict.
#[test]
fn the_gather_gates_the_aggregate_once_and_sources_only_on_refusal() {
    let code = blanked(code_only, "editor-core/src/product.rs", PRODUCT);
    let what = "editor-core/src/product.rs (code view)";
    let gates: Vec<usize> = required_matches(&code, what, "T::gate_at_rest")
        .into_iter()
        .filter(|at| !code[*at..].starts_with("T::gate_at_rest_declared"))
        .collect();
    assert_eq!(gates.len(), 2, "{what}: the gate is called in two places");
    let on_aggregate = required_matches(&code, what, "match T::gate_at_rest_kept(aggregate");
    assert_eq!(on_aggregate.len(), 1, "{what}: one aggregate gate");
    let ItemBody::Body(arms) = item_body(&code, on_aggregate[0]) else {
        panic!("{what}: the aggregate gate's `match` has no arms");
    };
    let refusal_arm = required_matches(&code[arms.clone()], what, "Err(errors) =>");
    assert_eq!(refusal_arm.len(), 1, "{what}: one refusal arm");
    let ItemBody::Body(refusal) = item_body(&code, arms.start + refusal_arm[0]) else {
        panic!("{what}: the aggregate gate's refusal arm has no block");
    };
    let attribute = required_matches(&code, what, "fn attribute_at_rest");
    assert_eq!(attribute.len(), 1, "{what}: one attribution fn");
    let ItemBody::Body(attribution) = item_body(&code, attribute[0]) else {
        panic!("{what}: `attribute_at_rest` has no body");
    };
    assert!(
        gates
            .iter()
            .all(|at| *at == on_aggregate[0] + "match ".len() || attribution.contains(at)),
        "{what}: a gate call sits outside the aggregate gate and the attribution fn"
    );
    let calls: Vec<usize> = required_matches(&code, what, "attribute_at_rest(")
        .into_iter()
        .filter(|at| !code[..*at].ends_with("fn "))
        .collect();
    assert!(
        calls.len() == 1 && refusal.contains(&calls[0]),
        "{what}: `attribute_at_rest` is called once, from the aggregate's refusal arm"
    );
}

const AT_REST: &str = include_str!("../src/checks/at_rest.rs");
const CHECKS: &str = include_str!("../src/checks.rs");

/// **The at-rest census runs in the gather, and only there.** The
/// gather hands the tier-3′ census to the recording door once
/// (`topo::record_at_rest`, whose next pass confirms what it records)
/// and keeps its findings on the product (`Product::census`); the
/// at-rest check reads them and calls no census of its own. Red if the
/// check runs the census again, or the gather calls it from a second
/// site: an extra pass over one product changes no verdict, so no
/// behavioural row can see it.
#[test]
fn the_census_runs_in_the_gather_and_never_at_the_check() {
    let census = "T::gate_at_rest_declared(";
    let product = blanked(code_only, "editor-core/src/product.rs", PRODUCT);
    let calls = required_matches(&product, "editor-core/src/product.rs (code view)", census);
    assert_eq!(calls.len(), 1, "the gather calls the census from one site");
    for (path, source) in [
        ("editor-core/src/checks/at_rest.rs", AT_REST),
        ("editor-core/src/checks.rs", CHECKS),
    ] {
        let code = blanked(code_only, path, source);
        assert!(
            !code.contains("gate_at_rest_declared") && !code.contains("record_at_rest("),
            "{path}: the check reads the gather's census and runs none"
        );
    }
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
    let (doc, body) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    );
    fixture::place(doc, body)
}

/// `node`'s body, as the evaluation holds it.
fn slot(ev: &mut Evaluation<f64>, node: RecipeNodeId) -> &mut editor_core::Framed<f64> {
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
    let was = Arc::clone(body.shared());
    *body = body.with_body(flipped(body.body()));
    let now = Arc::clone(body.shared());
    let solids = now.solids().count();
    in_every_assemble(ev, &was, &now);
    solids
}

/// Writes the reversed, inside-out copy of `node`'s body into its value
/// slot.
fn turn_inside_out(ev: &mut Evaluation<f64>, node: RecipeNodeId) {
    let body = slot(ev, node);
    let was = Arc::clone(body.shared());
    *body = body.with_body(Arc::new(body.body().revert().unwrap()));
    let now = Arc::clone(body.shared());
    in_every_assemble(ev, &was, &now);
}

/// **The written body where a `⊔` holds it too**: a `⊔` keeps its
/// members' bodies as its pieces, and its census reads them there, so a
/// piece holding `was` holds `now`, and the `⊔`'s census, not yet run,
/// is over the written pieces.
fn in_every_assemble(ev: &mut Evaluation<f64>, was: &Arc<Body<f64>>, now: &Arc<Body<f64>>) {
    for (&id, result) in &mut ev.nodes {
        let NodeResult::Ok(value) = result else {
            continue;
        };
        let Some(pieces) = value.carry.pieces.as_mut() else {
            continue;
        };
        for piece in &mut Arc::make_mut(pieces).pieces {
            for body in &mut piece.bodies {
                if Arc::ptr_eq(body.shared(), was) {
                    *body = body.with_body(Arc::clone(now));
                }
            }
        }
        if value
            .carry
            .census
            .as_ref()
            .is_some_and(|census| census.node == id)
        {
            value.carry.census = Some(Arc::new(editor_core::product::Census::new(
                id,
                Arc::clone(pieces),
            )));
        }
    }
}

/// The refusal's findings, as (placement, output) pairs, in list order.
fn named(err: &ProductError) -> Vec<(RecipeNodeId, u32)> {
    let ProductError::PieceInvalid { findings } = err else {
        panic!("want the per-placement refusal, got {err:?}");
    };
    assert!(
        findings.iter().all(|f| !f.errors.is_empty()),
        "every listed placement carries its findings: {findings:?}"
    );
    findings.iter().map(|f| (f.node, f.output)).collect()
}

/// **One source of one solid**: the part and the aggregate are the
/// same body, and the refusal names that placement rather than the product.
#[test]
fn a_lone_single_solid_source_is_named() {
    let (doc, a) = block(ProfileDoc::empty_derived("per-part-1", Tol::witness()), 0.0);
    let mut ev = run(&doc);
    assert_eq!(flip_one_face(&mut ev, a), 1);
    let err =
        product_recorded(&doc, &ev, Tol::witness()).expect_err("an invalid placement refuses");
    assert_eq!(named(&err), [(a, 0)]);
}

/// **Two single-solid sources, one inside-out**: the refusal names the
/// failing placement and not the clean one beside it.
#[test]
fn two_sources_name_only_the_failing_one() {
    let (doc, _a) = block(ProfileDoc::empty_derived("per-part-2", Tol::witness()), 0.0);
    let (doc, b) = block(doc, 5.0);
    let mut ev = run(&doc);
    assert_eq!(flip_one_face(&mut ev, b), 1);
    let err =
        product_recorded(&doc, &ev, Tol::witness()).expect_err("an invalid placement refuses");
    assert_eq!(
        named(&err),
        [(b, 0)],
        "the refusal names the inverted placement"
    );
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
    let o = with_resolver(store.clone());
    for dx in [0.0, 3.0] {
        let (next, id) = insert(sub, fixture::mated_instance(p));
        sub = fixture::pin(next, id, &Frame::translation([dx, 0.0, 0.0]), &o);
    }
    let sub = crate::fixture::assemble_world_copies(sub);
    let b = store.insert(sub, Tol::witness());
    let o = with_resolver(store);
    let (doc, source) = fixture::insert_pinned(
        ProfileDoc::empty(DocumentId::derive("per-part-3-a"), Tol::witness()),
        crate::fixture::mated_instance(b),
        &o,
    );
    let mut ev = fixture::run(&doc, &o);
    assert_eq!(
        flip_one_face(&mut ev, source),
        2,
        "the one source holds two solids"
    );
    let err =
        product_recorded(&doc, &ev, Tol::witness()).expect_err("an invalid placement refuses");
    assert_eq!(named(&err), [(source, 0)]);
}

/// **The cascade the re-gate exists for.** Placement `a` has one face's
/// sense flipped (a check 1–6 finding, `LoopRoleInverted`); placement `b` is
/// wholly inside out (a check 7 finding, `NegativeVolume`). Gated as
/// one body, the battery's check 7 never runs once an earlier check has
/// found anything, so the aggregate's own list reports `a`'s defect and
/// not `b`'s. The refusal names BOTH, each with its own finding.
#[test]
fn a_defect_that_stops_check_7_does_not_hide_another_placements_inside_out_body() {
    let (doc, a) = block(ProfileDoc::empty_derived("per-part-4", Tol::witness()), 0.0);
    let (doc, b) = block(doc, 5.0);
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
        // Each block where it stands in the world.
        let framed = slot(&mut ev, node).clone();
        let placed = crate::fixture::in_space(&framed);
        topo::graft_disjoint_all(&mut aggregate, &placed).expect("the two blocks are disjoint");
    }
    let together =
        topo::validate_geometric(&aggregate, Tol::witness()).expect_err("the aggregate is invalid");
    assert!(
        together.iter().any(is_inverted_loop) && !together.iter().any(is_inside_out),
        "the aggregate's own list hides b's inside-out body: {together:?}"
    );

    let err = product_recorded(&doc, &ev, Tol::witness()).expect_err("invalid placements refuse");
    assert_eq!(named(&err), [(a, 0), (b, 0)], "both placements are named");
    let ProductError::PieceInvalid { findings } = &err else {
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
        text.contains(&format!(
            "piece {} output 0",
            test_utils::refusal::tag(a.0.digest())
        )) && text.contains(&format!(
            "piece {} output 0",
            test_utils::refusal::tag(b.0.digest())
        )),
        "the message names both pieces: {text}"
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
