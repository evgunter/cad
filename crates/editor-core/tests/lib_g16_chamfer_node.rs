//! **LIB-G16 — the `Chamfer` recipe node** (RECIPE-DOORS D2, #918).
//!
//! `Node::Chamfer` wires `sweep::fillet::build::chamfer_edges` over an
//! authored selection of its target's edges. The document under test
//! is `corpus/die_chamfer.rs` — `die_fillet`'s recipe with the blend
//! swapped — so these rows are the fillet node's rows over the twin,
//! and a difference between them is a difference the blend accounts
//! for.
//!
//! # The oracles, derived rather than measured
//!
//! A cube of side `L` with all twelve edges chamfered at setback `d`
//! is the cube intersected with twelve edge half-spaces (one per
//! edge: the two distances to the edge's supports sum to at least
//! `d`) and eight corner half-spaces (the corner patch is the plane
//! through the three feet, `x + y + z ≥ 2d` at the origin corner).
//! Integrating that region gives
//!
//! ```text
//! V    = L³ − 6Ld² + (16/3)d³
//! area = 6(L−2d)² + 12√2·d(L−2d) + 4√3·d²
//! ```
//!
//! — six full support squares (each corner patch meets its support at
//! the single foot point, so it trims no area off them), twelve
//! rectangular strips `d√2` wide and `L−2d` long, and eight
//! equilateral corner patches of side `d√2`. Both carry irrationals,
//! which is why the corpus document pins no exact mass and these rows
//! meter at a stated relative tolerance instead.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

use corpus::{body_of, die_chamfer, eval, failures};
use editor_core::{
    CancelToken, EvalOptions, EvalOutcome, Node, NodeErrorKind, NodeResult, ProfileDoc,
    ProfileProgram, RecipeNodeId, SlotId, StableName, evaluate,
};
use geom_core::Tol;

/// The chamfered cube's closed forms (module docs).
fn chamfered_box(l: f64, d: f64) -> (f64, f64) {
    let volume = l.powi(3) - 6.0 * l * d * d + (16.0 / 3.0) * d.powi(3);
    let area = 6.0 * (l - 2.0 * d).powi(2)
        + 12.0 * core::f64::consts::SQRT_2 * d * (l - 2.0 * d)
        + 4.0 * 3.0_f64.sqrt() * d * d;
    (volume, area)
}

/// **The f64 row**: the document evaluates green, its head is a valid
/// closed solid with the twin's face count, and its certified mass
/// meets the closed forms above.
#[test]
fn die_chamfer_evaluates_green_and_meters_the_closed_form() {
    let d = die_chamfer::document();
    let ev = eval::<f64>(&d.doc);
    let bad = failures(&ev);
    assert!(bad.is_empty(), "die_chamfer:\n{}", bad.join("\n"));
    assert_eq!(ev.outcome, EvalOutcome::Completed);
    assert_eq!(ev.order.len(), d.len());

    let body = body_of(&ev, d.result.expect("the blank is the head"));
    assert_eq!(topo::validate(body), Ok(()), "tier 1");
    assert_eq!(topo::validate_closed(body), Ok(()), "closed");

    // The same 26 the fillet's twin produces, and for the same
    // reasons: 6 shrunken supports, 12 strips, 8 corner patches. Only
    // the SURFACES differ — every one of these is a plane.
    assert_eq!(
        body.faces().count(),
        26,
        "6 supports + 12 strips + 8 corner patches"
    );

    let m = topo::mass_properties(body, Tol::witness()).expect("mass properties");
    let (v, a) = chamfered_box(die_chamfer::L, die_chamfer::D);
    assert!(
        (m.volume - v).abs() <= 1e-9 * v,
        "volume {} vs closed form {v}",
        m.volume
    );
    assert!(
        (m.surface_area - a).abs() <= 1e-9 * a,
        "area {} vs closed form {a}",
        m.surface_area
    );
}

/// **The chamfer is not the fillet.** The twin documents differ in
/// exactly one node kind, and the geometry says so: a chamfer of
/// setback `d` removes strictly more material than a fillet of radius
/// `d`, because the flat strip cuts the corner the rolling ball rides
/// around.
#[test]
fn the_chamfer_removes_more_than_the_fillet_of_the_same_size() {
    let ch = die_chamfer::document();
    let ev = eval::<f64>(&ch.doc);
    let chamfered = body_of(&ev, ch.result.expect("head"));
    let cv = topo::mass_properties(chamfered, Tol::witness())
        .expect("mass properties")
        .volume;

    let fi = corpus::die_fillet::document();
    let fev = eval::<f64>(&fi.doc);
    let filleted = body_of(&fev, fi.result.expect("head"));
    let fv = topo::mass_properties(filleted, Tol::witness())
        .expect("mass properties")
        .volume;

    assert_eq!(die_chamfer::D, corpus::die_fillet::R, "same size, by value");
    assert!(
        cv < fv,
        "the chamfer must remove more than the fillet: {cv} vs {fv}"
    );
}

/// **The construction door canonicalizes**, exactly as `Node::fillet`
/// does — a recipe's bits must not depend on click order.
#[test]
fn the_chamfer_door_sorts_and_dedups_its_selection() {
    let a = fixture::ename(RecipeNodeId(1), fixture::wall(0));
    let b = fixture::ename(RecipeNodeId(1), fixture::wall(1));
    let node: Node<ProfileProgram> = Node::chamfer(
        RecipeNodeId(1),
        fixture::len(0.1),
        vec![b.clone(), a.clone(), b.clone()],
    );
    let Node::Chamfer { selection, .. } = &node else {
        panic!("the door builds a chamfer");
    };
    let mut want = vec![a, b];
    want.sort();
    assert_eq!(selection, &want, "sorted and deduplicated");
}

/// **The slot is the chamfer's own**, and it is a Length: a setback is
/// not a radius, and the vocabulary says so rather than reusing the
/// fillet's name for a different quantity.
#[test]
fn the_distance_slot_is_named_and_dimensioned_for_the_setback() {
    let node: Node<ProfileProgram> = Node::chamfer(RecipeNodeId(1), fixture::len(0.1), Vec::new());
    assert_eq!(node.slots(), vec![SlotId::ChamferDistance]);
    assert_eq!(
        SlotId::ChamferDistance.dimension(),
        editor_core::Dimension::Length
    );
    assert!(!SlotId::ChamferDistance.is_structural());
    assert_eq!(SlotId::ChamferDistance.label(), "chamfer distance");
    assert!(node.expr(SlotId::Radius).is_none(), "not the fillet's slot");
    assert!(node.expr(SlotId::ChamferDistance).is_some());
}

/// **The payload's names are the selection**, so `Rebind` reaches
/// them and the insert door checks their heads — the `Fillet`
/// contract, which `payload_names` is the single answer for.
#[test]
fn the_selection_is_payload_names() {
    let a = fixture::ename(RecipeNodeId(1), fixture::wall(0));
    let node: Node<ProfileProgram> =
        Node::chamfer(RecipeNodeId(1), fixture::len(0.1), vec![a.clone()]);
    let names: Vec<&StableName> = node.payload_names();
    assert_eq!(names, vec![&a]);
    assert_eq!(node.named_nodes(), vec![RecipeNodeId(1)]);
}

/// **An empty selection refuses, naming the chamfer.** A blend of
/// nothing is an unfinished recipe, not the identity — and the refusal
/// must not tell the author a FILLET was empty.
#[test]
fn an_empty_selection_refuses_as_a_chamfer() {
    let doc = ProfileDoc::empty_derived("g16-empty", Tol::witness());
    let (doc, profile) = fixture::insert(
        doc,
        Node::Profile(fixture::desc(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
        )),
    );
    let (doc, cube) = fixture::insert(
        doc,
        Node::Extrude {
            profile,
            distance: fixture::len(1.0),
        },
    );
    let (doc, ch) = fixture::insert(doc, Node::chamfer(cube, fixture::len(0.1), Vec::new()));

    let ev = evaluate::<f64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    let Some(NodeResult::Failed(e)) = ev.nodes.get(&ch) else {
        panic!("an empty selection must refuse");
    };
    match &e.kind {
        NodeErrorKind::BlendSelectionEmpty { verb } => {
            assert_eq!(*verb, sweep::fillet::BlendKind::Chamfer);
        }
        other => panic!("expected the empty-selection refusal, got {other:?}"),
    }
    let msg = e.kind.to_string();
    assert!(
        msg.contains("chamfer") && !msg.contains("fillet"),
        "the refusal must name the chamfer: {msg}"
    );
}
