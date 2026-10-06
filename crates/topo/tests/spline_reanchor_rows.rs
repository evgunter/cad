//! **The offset door re-anchors an untouched edge on its spline
//! carrier.** On the M7-8 cube the wall's four edges ride degree-1
//! NURBS carriers, so a cap or side offset moves the ends of two of
//! them: each end is re-read as the carrier's foot from its old
//! parameter, and the stored range follows it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::HashMap;

use geom::Curve3;
use geom_core::{Point3, Tol};
use topo::{Body, EdgeKey, ReplaceFaceError, VertexKey};

use crate::fixture::m7_8::m7_8_cube;

/// A spline-carried edge's carrier, range and end vertices.
type SplineEdge = (Curve3<f64>, (f64, f64), [VertexKey; 2]);

/// Each spline-carried edge of `body`.
fn spline_edges(body: &Body<f64>) -> HashMap<EdgeKey, SplineEdge> {
    body.edges()
        .filter_map(|(k, e)| {
            let curve = body.get_curve_geom(e.curve)?.certified()?;
            let Curve3::Nurbs(_) = curve.carrier() else {
                return None;
            };
            let start = body.get_half_edge(e.he_plus)?.start;
            let end = body.half_edge_end(e.he_plus)?;
            Some((k, (curve.carrier().clone(), curve.params(), [start, end])))
        })
        .collect()
}

fn point(body: &Body<f64>, v: VertexKey) -> Point3<f64> {
    body.vertex_points()
        .find(|(k, _)| *k == v)
        .map(|(_, p)| p)
        .expect("the vertex resolves")
}

/// The cube's planar face whose chart normal is `normal`.
fn face_on(body: &Body<f64>, normal: [f64; 3]) -> topo::FaceKey {
    body.faces()
        .find(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Plane { normal: n, .. })
                    if (n.x - normal[0]).abs() + (n.y - normal[1]).abs() + (n.z - normal[2]).abs() < 1e-12
            )
        })
        .map(|(k, _)| k)
        .expect("the cube has the face")
}

/// Offsetting the top cap inward moves the upper ends of the wall's
/// two vertical edges down the wall: each new range ends at the moved
/// vertex, strictly inside its carrier's domain, and the ends that did
/// not move keep their parameters bit for bit.
#[test]
fn an_inward_cap_offset_re_anchors_the_walls_spline_edges() {
    let body = m7_8_cube::<f64>();
    let before = spline_edges(&body);
    assert_eq!(before.len(), 4, "the wall's four edges are spline-carried");
    let mut moved = body.clone();
    topo::replace_face_offset(
        &mut moved,
        face_on(&body, [0.0, 0.0, 1.0]),
        -0.25,
        Tol::witness(),
    )
    .expect("the cap moves inward along the wall");
    let after = spline_edges(&moved);
    let mut re_anchored = 0;
    for (edge, (carrier, (t0, t1), ends)) in &after {
        let (_, (s0, s1), _) = &before[edge];
        let geom::Curve3::Nurbs(spline) = carrier else {
            unreachable!("filtered to spline carriers")
        };
        let (lo, hi) = spline.domain();
        for (t, old, v) in [(*t0, *s0, ends[0]), (*t1, *s1, ends[1])] {
            let p = point(&moved, v);
            let gap = carrier.eval(t).distance(p);
            assert!(
                gap < 1e-9,
                "{edge:?}: the end at t = {t} is {gap} m off its vertex"
            );
            if t.to_bits() != old.to_bits() {
                re_anchored += 1;
                assert!(
                    lo < t && t < hi,
                    "{edge:?}: t = {t} left the domain [{lo}, {hi}]"
                );
                assert!(
                    (p.z - 0.75).abs() < 1e-12,
                    "{edge:?}: a moved end is on the moved cap"
                );
            }
        }
    }
    assert_eq!(
        re_anchored, 2,
        "the two vertical wall edges' upper ends move"
    );
}

/// Offsetting the top cap OUTWARD runs those ends past their carriers,
/// which were minted as long as the edges: the door names the carrier's
/// end and the distance past it, which is the offset.
#[test]
fn an_outward_cap_offset_runs_past_the_spline_carriers_end() {
    let body = m7_8_cube::<f64>();
    let mut moved = body.clone();
    let e = topo::replace_face_offset(
        &mut moved,
        face_on(&body, [0.0, 0.0, 1.0]),
        0.25,
        Tol::witness(),
    )
    .expect_err("the carrier ends at the cap");
    let ReplaceFaceError::ReanchorPastCarrierEnd { edge, gap } = e else {
        panic!("expected the past-the-end refusal, got {e}");
    };
    assert!(
        spline_edges(&body).contains_key(&edge),
        "{edge:?} is a wall edge"
    );
    assert!(
        (gap - 0.25).abs() < 1e-12,
        "the vertex is the offset past the end, got {gap}"
    );
}
