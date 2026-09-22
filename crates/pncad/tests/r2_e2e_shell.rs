//! R2 review end-to-end: a hollowed box authored and evaluated the
//! way a user would, entirely through the `pncad` façade — at `f64`,
//! at `Dual64`, and (under the feature) at `Interval`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::document::{
    CancelToken, Datum, DocEdit, EvalOptions, Evaluation, Expr, LoopProgram, Node, NodeErrorKind,
    Dimension, NodeResult, ProfileDoc, ProfileProgram, RecipeNodeId, RefusingReach, apply,
    evaluate,
};
use pncad::geom_core::Tol;
use pncad::prelude::StableName;
use pncad::select::{CapEnd, EntityKind, RoleSeg};

fn len(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).unwrap()
}
fn scl(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).unwrap()
}

fn insert(doc: ProfileDoc, node: Node<ProfileProgram>) -> (ProfileDoc, RecipeNodeId) {
    let applied = apply(
        &doc,
        &DocEdit::InsertNode { node },
        Tol::witness(),
        &RefusingReach,
    )
    .expect("the insert applies");
    let id = applied.record.minted.expect("a minted id");
    (applied.doc, id)
}

/// The hollowed box: plane -> square profile -> extrude -> shell, its
/// top cap designated open.
fn open_box() -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("r2-e2e", Tol::witness());
    let (doc, plane) = insert(
        doc,
        Node::Datum(Datum::Frame {
            origin: [0.0; 3].map(len),
            u: [1.0, 0.0, 0.0].map(scl),
            v: [0.0, 1.0, 0.0].map(scl),
        }),
    );
    let square = LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]).unwrap();
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![square],
        }),
    );
    let (doc, blank) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    let top = StableName {
        kind: EntityKind::Face,
        node: blank,
        path: vec![RoleSeg::Cap(CapEnd::End)],
    };
    let (doc, cup) = insert(doc, Node::shell(blank, len(0.125), vec![top]));
    (doc, blank, cup)
}

// NOTE: `EvalScalar` is NOT on the facade (it is in `NOT_CARRIED`),
// so a facade consumer cannot write this helper generically at all —
// each scalar gets its own call.
macro_rules! run {
    ($t:ty, $doc:expr) => {
        evaluate::<$t>(
            $doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            Tol::witness(),
        )
    };
}

#[test]
fn r2_e2e_the_box_hollows_at_f64() {
    let (doc, _blank, cup) = open_box();
    let ev: Evaluation<f64> = run!(f64, &doc);
    for (id, r) in &ev.nodes {
        assert!(
            matches!(r, NodeResult::Ok(_)),
            "node {id:?} did not evaluate: {r:?}"
        );
    }
    let pncad::document::ValuePayload::Body(body) =
        &ev.value(cup).expect("the shell has a value").payload
    else {
        panic!("the shell's payload is not a body");
    };
    let props = pncad::topo::mass_properties(body, Tol::witness()).expect("the cup measures");
    // The closed form of an open cup, l = h = 1, t = 0.125:
    // l*l*h - (l-2t)^2 * (h-t).
    let inner: f64 = 1.0 - 0.25;
    let want = 1.0 - inner * inner * (1.0 - 0.125);
    assert_eq!(props.volume, want, "the cup's volume is its closed form");
    println!("f64 volume bits {:016x} (= {want})", props.volume.to_bits());
    println!(
        "f64 census V{}E{}F{}",
        body.vertices().count(),
        body.edges().count(),
        body.faces().count()
    );
}

#[test]
fn r2_e2e_the_box_refuses_typed_at_dual_and_alone() {
    let (doc, blank, cup) = open_box();
    let ev: Evaluation<pncad::geom_core::Dual64> = run!(pncad::geom_core::Dual64, &doc);
    let head = ev.nodes.get(&cup).expect("the shell node ran");
    let NodeResult::Failed(e) = head else {
        panic!("the shell did not refuse at a dual: {head:?}");
    };
    assert!(
        matches!(e.kind, NodeErrorKind::ShellLaneUnsupported { lane: "Dual" }),
        "the refusal is not the typed shell-door absence: {:?}",
        e.kind
    );
    println!("Dual64 refusal: {}", e.kind);
    for (id, r) in &ev.nodes {
        if *id == cup {
            continue;
        }
        assert!(
            matches!(r, NodeResult::Ok(_)),
            "the refusal is not the shell's alone: {id:?} -> {r:?}"
        );
    }
    let _ = blank;
}

#[cfg(feature = "interval")]
#[test]
fn r2_e2e_the_box_hollows_at_interval() {
    use pncad::geom_core::interval::Interval;
    let (doc, _blank, cup) = open_box();
    let ev: Evaluation<Interval> = run!(Interval, &doc);
    for (id, r) in &ev.nodes {
        assert!(
            matches!(r, NodeResult::Ok(_)),
            "node {id:?} did not evaluate at Interval: {r:?}"
        );
    }
    let pncad::document::ValuePayload::Body(body) =
        &ev.value(cup).expect("the shell has a value").payload
    else {
        panic!("the shell's payload is not a body");
    };
    println!(
        "Interval census V{}E{}F{}",
        body.vertices().count(),
        body.edges().count(),
        body.faces().count()
    );
}
