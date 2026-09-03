//! Adversarial-review probes for the fillet naming work. NOT part of
//! any one PR.
//!
//! P1/P2: a shrunk support lands on `FromTarget` of the name the
//! target's own table carries — on the every-edge cube (P1, a
//! bijection onto its six face names) and on the composed die (P2).
//! P3: totality extends beyond the square corpus — a triangular prism
//! (V=6, E=9, F=5, trivalent) filleted on every edge names all
//! 20 + 36 + 18 entities and every name resolves.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture;

use std::collections::BTreeSet;

use corpus::die_composed;
use editor_core::{
    CancelToken, CapEnd, Dimension, EntityKind, EvalOptions, Expr, Node, NodeResult, ProfileDoc,
    RecipeNodeId, RoleSeg, StableName, evaluate,
};
use fixture::prism_edges;
use geom_core::Tol;

fn len(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).expect("a length literal")
}

fn eval(doc: &ProfileDoc) -> editor_core::Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

fn table_of(
    ev: &editor_core::Evaluation<f64>,
    node: RecipeNodeId,
) -> std::sync::Arc<editor_core::NameTable> {
    match ev.nodes.get(&node) {
        Some(NodeResult::Ok(v)) => std::sync::Arc::clone(&v.name_table),
        other => panic!("node {node:?} did not evaluate: {other:?}"),
    }
}

/// P1: the shrunk supports' `FromTarget` rows are a BIJECTION onto the
/// target's face names — six supports, each wrapping a distinct one of
/// {Cap(Bottom), Cap(Top), Wall(0..3)}, all kind Face.
#[test]
fn p1_shrunk_supports_wrap_exactly_the_targets_face_names() {
    let doc = ProfileDoc::empty_derived("review_m6_5_pr2_probes", Tol::witness());
    let (doc, p) = fixture::insert(
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
            profile: p,
            distance: len(1.0),
        },
    );
    let (doc, blank) = fixture::insert(doc, Node::fillet(cube, len(0.125), prism_edges(cube, 4)));
    let ev = eval(&doc);
    let table = table_of(&ev, blank);
    let target = table_of(&ev, cube);

    let mut inner: Vec<StableName> = Vec::new();
    for (n, _) in table.iter() {
        if let RoleSeg::FromTarget(up) = n.path.first().expect("a role") {
            assert_eq!(
                n.kind,
                EntityKind::Face,
                "only supports are FromTarget here"
            );
            inner.push((**up).clone());
        }
    }
    assert_eq!(inner.len(), 6);
    let got: BTreeSet<_> = inner.iter().cloned().collect();
    assert_eq!(got.len(), 6, "each support wraps a distinct upstream face");
    let expected: BTreeSet<StableName> = [
        fixture::fname(cube, RoleSeg::Cap(CapEnd::Bottom)),
        fixture::fname(cube, RoleSeg::Cap(CapEnd::Top)),
        fixture::fname(cube, fixture::wall(0)),
        fixture::fname(cube, fixture::wall(1)),
        fixture::fname(cube, fixture::wall(2)),
        fixture::fname(cube, fixture::wall(3)),
    ]
    .into_iter()
    .collect();
    assert_eq!(
        got, expected,
        "the supports wrap exactly the target's faces"
    );
    for up in &inner {
        assert!(
            target.lookup(up).is_some(),
            "the wrapped name is the target's own: {up:?}"
        );
    }
}

/// P2: the surgery door (composed die) lands its surviving supports on
/// `FromTarget` of names its target's table carries — same rule as P1,
/// the other door.
#[test]
fn p2_surgery_supports_wrap_names_the_target_table_carries() {
    let doc = die_composed::document();
    let ev = eval(&doc.doc);
    let (fillet, target) = {
        let mut found = None;
        for id in doc.doc.order() {
            if let Some(Node::Fillet { target, .. }) = doc.doc.node(*id) {
                found = Some((*id, *target));
            }
        }
        found.expect("the composed die has a fillet node")
    };
    let table = table_of(&ev, fillet);
    let target_table = table_of(&ev, target);
    let mut face_survivors = 0usize;
    for (n, _) in table.iter() {
        if n.kind != EntityKind::Face {
            continue;
        }
        if let RoleSeg::FromTarget(up) = n.path.first().expect("a role") {
            face_survivors += 1;
            assert!(
                target_table.lookup(up).is_some(),
                "a surviving support wraps a name the target does not carry: {up:?}"
            );
            assert_eq!(up.kind, EntityKind::Face);
        }
    }
    assert!(
        face_survivors >= 6,
        "the composed die's six box supports survive (got {face_survivors})"
    );
}

/// P3: totality beyond the square — a triangular prism filleted on
/// every edge. V=6, E=9, F=5 gives 5+9+6 = 20 faces, 18+18 = 36
/// edges, 18 vertices; every entity named, every name resolves.
#[test]
fn p3_totality_holds_for_a_triangular_prism() {
    use editor_core::resolve::{Resolution, RunCtx, resolve};
    let doc = ProfileDoc::empty_derived("review_m6_5_pr2_probes", Tol::witness());
    let (doc, p) = fixture::insert(
        doc,
        Node::Profile(fixture::desc(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(0.0, 0.0), (2.0, 0.0), (0.0, 2.0)]],
        )),
    );
    let (doc, prism) = fixture::insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(1.0),
        },
    );
    let (doc, blank) = fixture::insert(doc, Node::fillet(prism, len(0.1), prism_edges(prism, 3)));
    let ev = eval(&doc);
    let NodeResult::Ok(v) = ev.nodes.get(&blank).expect("the fillet") else {
        panic!(
            "the triangular prism refuses the fillet: {:?}",
            ev.nodes.get(&blank)
        )
    };
    let editor_core::ValuePayload::Body(body) = &v.payload else {
        panic!("a body")
    };
    assert_eq!(body.faces().count(), 20);
    assert_eq!(body.edges().count(), 36);
    assert_eq!(body.vertices().count(), 18);
    let table = table_of(&ev, blank);
    assert_eq!(table.len(), 20 + 36 + 18 + 1, "every entity named");
    for (name, _) in table.iter() {
        let ctx = RunCtx {
            doc: &doc,
            eval: &ev,
        };
        match resolve(ctx, name) {
            Resolution::Resolved { .. } => {}
            other => panic!("{name:?} did not resolve: {other:?}"),
        }
    }
}
