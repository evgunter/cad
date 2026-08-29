//! M5 PR 5 acceptance shape (i), corpus form: the `cut_cylinder`
//! document evaluates green, its Split's sides carry the exact
//! `Ellipse` section edges (rung 2), and the pins replay at Probe
//! (the K-funnel recording scalar — every new predicate is recorded
//! by the standard probe sweep because it routes through
//! `geom_core::k_stats::decide`).
//!
//! The standard corpus rows (every ε row, `interval`, persistence
//! D6.1 round-trip, latency reporting) come from the REGISTRY — this
//! file adds only the shape-specific pins.
//!
//! The Probe replay pin lives in `m5_pr5_corpus_probe.rs`, gated on the
//! `probe` feature; the pins here are f64 and run in the default build.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

use editor_core::{Node, RecipeNodeId, SplitSide, ValuePayload};
use topo::Body;
use topo::Curve3;

fn split_node(doc: &editor_core::ProfileDoc) -> RecipeNodeId {
    *doc.order()
        .iter()
        .find(|id| matches!(doc.node(**id), Some(Node::Split { .. })))
        .expect("cut_cylinder carries a Split node")
}

fn ellipse_edges(body: &Body<f64>) -> Vec<topo::EdgeCurve<f64>> {
    let mut out = Vec::new();
    for (_, edge) in body.edges() {
        if let Some(c) = body.get_curve_geom(edge.curve).and_then(|g| g.certified())
            && matches!(c.carrier(), Curve3::Ellipse { .. })
        {
            out.push(c.clone());
        }
    }
    out
}

/// Shape (i) in the corpus: both split sides carry exactly two exact
/// `Ellipse` section arcs (a = r/cos φ, b = r), Intersection-described,
/// with rounding-scale certificate residuals.
#[test]
fn cut_cylinder_sides_carry_exact_ellipses() {
    let doc = corpus::cut_cylinder::document();
    let ev = corpus::eval::<f64>(&doc.doc);
    assert_eq!(corpus::failures(&ev), Vec::<String>::new());
    let split = split_node(&doc.doc);
    let ValuePayload::Split { above, below } = &ev.value(split).unwrap().payload else {
        panic!("split payload");
    };
    let (SplitSide::Body(above), SplitSide::Body(below)) = (above, below) else {
        panic!("both sides carry material");
    };
    let phi = corpus::cut_cylinder::TILT;
    for part in [above.as_ref(), below.as_ref()] {
        let ellipses = ellipse_edges(part);
        assert_eq!(ellipses.len(), 2, "two ellipse arcs bound the section");
        for c in &ellipses {
            let Curve3::Ellipse { major, minor, .. } = *c.carrier() else {
                panic!("ellipse filter guarantees the variant");
            };
            assert!((minor - 0.5).abs() < 1e-12);
            assert!((major - 0.5 / phi.cos()).abs() < 1e-12);
            assert!(matches!(
                c.description(),
                topo::EdgeDescription::Intersection { .. }
            ));
            assert!(c.certificate().max_residual < 1e-12);
        }
    }
}

/// The bump edit's cone is a proper subset (the D2 incremental row's
/// precondition, asserted here so the registry row has teeth).
#[test]
fn cut_cylinder_bump_cone_is_proper() {
    let doc = corpus::cut_cylinder::document();
    let cone = corpus::cone(&doc.doc, doc.bump_root);
    assert!(cone.len() < doc.doc.len(), "the bump must leave reuse");
    let bumped = doc.bumped();
    let ev = corpus::eval::<f64>(&bumped);
    assert_eq!(corpus::failures(&ev), Vec::<String>::new());
}
