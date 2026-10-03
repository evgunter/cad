//! **A cylinder's flat caps over a merged bar's valence-2 rim
//! vertices** (review of PR 3770): the cylinder stands across the
//! bar's y = 1 wall, its caps coplanar with the bar's, so the rim
//! vertex's pierce run holds the whole orbit. Before
//! `classify_vertex_on_face` recorded that run as the strut `mev_null`
//! builds, the union refused `JoinDesync`. Oracle: the half-disc
//! outside the bar, `π r² / 2`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use sweep::test_support::{cylinder_of_arcs_at, finished};
use topo::flush::{declare_all, find_flush_candidates};
use topo::{BooleanResult, intersect_with, mass_properties, subtract_with, union_with};

#[test]
fn a_cylinder_cap_over_a_merged_rim_vertex_answers_every_op() {
    let tol = Tol::witness();
    let c = topo::test_support::brick((0.5, 1.5), (0.0, 1.0), (0.0, 1.0), tol);
    let c = finished("brick C", c, tol);
    let d = topo::test_support::brick((1.2, 2.2), (0.0, 1.0), (0.0, 1.0), tol);
    let d = finished("brick D", d, tol);
    let decls = declare_all(&find_flush_candidates(&c, &d, tol).unwrap());
    let BooleanResult::Body(bar) = union_with(&c, &d, &decls, tol).unwrap() else {
        panic!("the bar is not empty");
    };
    let bar = bar.body;
    for (n, r, cx) in [
        (4usize, 0.3, 1.35),
        (3, 0.3, 1.35),
        (4, 0.1, 1.2),
        (6, 0.25, 1.5),
    ] {
        let cyl = cylinder_of_arcs_at(n, r, Point2::new(cx, 1.0), 0.0, 1.0, tol);
        let cyl = finished("the cylinder", cyl, tol);
        let decls = declare_all(&find_flush_candidates(&bar, &cyl, tol).unwrap());
        let half = core::f64::consts::PI * r * r / 2.0;
        for (name, op, want) in [
            ("∪", union_with as fn(_, _, _, _) -> _, 1.7 + half),
            ("∩", intersect_with, half),
            ("∖", subtract_with, 1.7 - half),
        ] {
            let label = format!("bar {name} cylinder(n {n}, r {r}, x {cx})");
            let BooleanResult::Body(bb) =
                op(&bar, &cyl, &decls, tol).unwrap_or_else(|e| panic!("{label}: {e:?}"))
            else {
                panic!("{label}: an overlapping pair cannot be Empty");
            };
            assert_eq!(topo::validate_closed(&bb.body), Ok(()), "{label}: tier 2");
            assert_eq!(
                topo::validate_geometric(&bb.body, tol),
                Ok(()),
                "{label}: tier 3"
            );
            let v = mass_properties(&bb.body, tol).unwrap().volume;
            assert!((v - want).abs() < 1e-9, "{label}: volume {v}, want {want}");
        }
    }
}
