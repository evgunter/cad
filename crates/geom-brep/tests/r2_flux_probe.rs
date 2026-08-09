//! **R2 independent review probe (PR #264)** — probes branch only.
//! Re-derives the Gauss flux arm's exactness on a fixture R1 did not
//! use: the parabola y = 2x − x² carried as a degree-3 (elevated)
//! B-spline, closed by its chord. True area 4/3 (∫₀²(2x−x²)dx), and
//! the degree-3 integrand (P−ref)×P′ has degree 5 = 2·3−1, exactly
//! the 3-point rule's exactness ceiling — any off-by-one in the rule
//! table's degree accounting shows up here as a truncation error far
//! above rounding.
#![allow(clippy::unwrap_used, clippy::panic)]

use geom_brep::LoopEdge;
use geom_brep::props::loop_vector_area;
use geom_core::spline::KnotVector;
use geom_core::{Point3, Vec3};
use geom_curves::{Curve3, NurbsCurve3};

#[test]
fn r2_cubic_parabola_area_is_exact_to_rounding() {
    // Quadratic Bézier (0,0),(1,2),(2,0) elevated to cubic:
    // Q1 = (P0+2P1)/3, Q2 = (2P1+P2)/3.
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let curve = NurbsCurve3::new(
        kv,
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(2.0 / 3.0, 4.0 / 3.0, 0.0),
            Point3::new(4.0 / 3.0, 4.0 / 3.0, 0.0),
            Point3::new(2.0, 0.0, 0.0),
        ],
        vec![1.0; 4],
    )
    .unwrap();
    let edges = vec![
        LoopEdge {
            carrier: Curve3::Nurbs(std::sync::Arc::new(curve)),
            t0: 0.0,
            t1: 1.0,
            forward: true,
            start: 0,
            end: 1,
        },
        LoopEdge {
            carrier: Curve3::Line {
                origin: Point3::new(2.0, 0.0, 0.0),
                dir: Vec3::new(-2.0, 0.0, 0.0),
            },
            t0: 0.0,
            t1: 1.0,
            forward: true,
            start: 1,
            end: 0,
        },
    ];
    // Two ref points: exactness must not depend on conditioning anchor.
    for anchor in [Point3::new(0.0, 0.0, 0.0), Point3::new(-3.5, 7.25, 0.0)] {
        let a = loop_vector_area(&edges, anchor).unwrap();
        let err = (a.norm() - 4.0 / 3.0).abs();
        println!("anchor {anchor:?}: |area| err {err:e}");
        assert!(err < 1e-13, "exact to rounding: {err:e}");
        assert!(
            a.x.abs() < 1e-13 && a.y.abs() < 1e-13,
            "planar loop, z-normal area"
        );
    }
}
