//! **The join door is whole past its up-front read** (`Body::join_edges`).
//! The read refuses an in-band reading before any kill; a refusal
//! that only the kill meets, after an earlier join was made, is the
//! staging's to undo. A disc whose top rim holds two joinable vertices
//! in arena order: an ordinary arc split first, then an arc restated as
//! the exact rational quadratic of itself and split. The predicate reads
//! the second along the cylinder and cap's locus like the first, but the
//! join cannot run a kept edge on along a spline
//! (`work/fuse/joining-a-spline-carrier-is-unbuilt`), so it refuses
//! inside its kill, after the first join, and the body is as found.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom_core::{Band, Tol};
use topo::{Body, EdgeKey};

fn ends(body: &Body<f64>, e: EdgeKey) -> (geom_core::Point3<f64>, geom_core::Point3<f64>) {
    let d = body.get_edge(e).unwrap();
    let p = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
    (
        p(body.get_half_edge(d.he_plus).unwrap().start),
        p(body.half_edge_end(d.he_plus).unwrap()),
    )
}

#[test]
fn a_join_refused_after_an_earlier_kill_leaves_the_body_as_found() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let mut body = sweep::test_support::disc_of_arcs(4, 1.0, 1.0, tol);
    let top: Vec<EdgeKey> = body
        .edges()
        .map(|(e, _)| e)
        .filter(|&e| {
            let (p, q) = ends(&body, e);
            p.z == 1.0 && q.z == 1.0
        })
        .collect();
    assert_eq!(top.len(), 4, "the top rim's four quarter arcs");
    let (ordinary, spline) = (top[0], top[1]);
    let (t0, t1) = body
        .get_curve_geom(body.get_edge(ordinary).unwrap().curve)
        .and_then(topo::CurveGeom::certified)
        .unwrap()
        .params();
    let first = body.split_edge(ordinary, 0.5 * (t0 + t1), tol).unwrap();

    // The second arc as the rational quadratic of itself: its ends, the
    // corner their tangents meet at, and the corner's weight cos 45°.
    let (p0, p1) = ends(&body, spline);
    let centre = geom_core::Point3::new(0.0, 0.0, 1.0);
    let corner = centre + (p0 - centre) + (p1 - centre);
    let mid = centre + ((p0 - centre) + (p1 - centre)) / ((p0 - centre) + (p1 - centre)).norm();
    let geom_brep::EdgeDescription::Intersection { s1, s2, .. } = *body
        .get_curve_geom(body.get_edge(spline).unwrap().curve)
        .and_then(topo::CurveGeom::certified)
        .unwrap()
        .description()
    else {
        panic!("a rim arc is the cylinder and cap's intersection");
    };
    let kv = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    body.set_edge_curve(
        spline,
        topo::EdgeCurveSpec {
            description: geom_brep::EdgeDescriptionSpec::Intersection {
                s1,
                s2,
                witness: mid,
            },
            carrier: geom::Curve3::Nurbs(Arc::new(
                geom::NurbsCurve3::new(
                    kv,
                    vec![p0, corner, p1],
                    vec![1.0, core::f64::consts::FRAC_1_SQRT_2, 1.0],
                )
                .unwrap(),
            )),
            param_start: 0.0,
            param_end: 1.0,
        },
        tol,
    )
    .unwrap();
    let second = body.split_edge(spline, 0.5, tol).unwrap();
    assert_eq!(
        topo::joinable_vertices(&body, band).unwrap(),
        vec![first.vertex, second.vertex],
        "the ordinary vertex first, and the up-front read passes"
    );

    let before = format!("{body:?}");
    let refusal = body.join_edges(band, tol).unwrap_err();
    assert!(
        matches!(
            refusal,
            topo::BooleanError::JoinCarrierUnsupported {
                carrier: geom::CurveKind::Nurbs,
                closed: false,
                ..
            }
        ),
        "{refusal:?}"
    );
    assert_eq!(format!("{body:?}"), before, "the body as found");
}
