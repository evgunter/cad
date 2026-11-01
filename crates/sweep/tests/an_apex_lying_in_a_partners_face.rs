//! **A line edge lying in a partner's planar face is recorded where the
//! face holds it, and only there.** The operand is a triangle prism over
//! `z ∈ [0, 2]` whose apex edge runs up `x = 0, y = 0.5`; each partner
//! has a face on the plane `y = 0.5`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::test_support::{brick, finished};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{AtRestBody, BooleanResult};

fn tol() -> Tol {
    Tol::witness()
}

fn zprism(pts: &[(f64, f64, f64)], z: (f64, f64)) -> topo::Body<f64> {
    let plane = SketchPlane::xy_at(Point3::new(0.0, 0.0, z.0));
    let lp = bulge_loop(
        pts.iter()
            .map(|&(x, y, b)| (Point2::new(x, y), b))
            .collect(),
    );
    let p = Profile::new(plane, vec![lp]).validate(tol()).unwrap();
    extrude(
        &p,
        Extrusion::Distance {
            depth: z.1 - z.0,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body
}

fn apex() -> AtRestBody<f64> {
    let tri = [(0.5, 0.0, 0.0), (0.0, 0.5, 0.0), (-0.5, 0.0, 0.0)];
    finished("the apex", zprism(&tri, (0.0, 2.0)), tol())
}

fn point(body: &topo::Body<f64>, v: topo::VertexKey) -> Point3<f64> {
    *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
}

/// An edge's two ends, and a face's plane `(normal, origin · normal)`
/// (`None` off a plane).
type EdgeInPlane = ([Point3<f64>; 2], Option<(Vec3<f64>, f64)>);

/// Every edge-face record of `r`'s body.
fn edge_face_records(r: &BooleanResult<f64>) -> Vec<EdgeInPlane> {
    let BooleanResult::Body(b) = r else {
        panic!("the op builds a body: {r:?}");
    };
    let body = &b.body;
    b.contacts
        .cell_pairs()
        .filter_map(|pair| match pair {
            (topo::Cell::Edge(e), topo::Cell::Face(f))
            | (topo::Cell::Face(f), topo::Cell::Edge(e)) => {
                let d = body.get_edge(e).unwrap();
                let p = point(body, body.get_half_edge(d.he_plus).unwrap().start);
                let q = point(body, body.half_edge_end(d.he_plus).unwrap());
                let plane = match body.get_surface(body.get_face(f).unwrap().surface) {
                    Some(geom::Surface::Plane { origin, normal, .. }) => {
                        let d = origin.x * normal.x + origin.y * normal.y + origin.z * normal.z;
                        Some((*normal, d))
                    }
                    _ => None,
                };
                Some(([p, q], plane))
            }
            _ => None,
        })
        .collect()
}

/// **A round hole through the face the apex lies in.** The hole's
/// circle crosses the apex at `z = 0.8` and `z = 1.2`; the union records
/// the apex in the face `y = 0.5` once.
#[test]
fn an_apex_across_a_round_hole_is_recorded_in_the_face() {
    let disc = [
        (0.2, 1.0, (core::f64::consts::PI / 4.0).tan()),
        (-0.2, 1.0, (core::f64::consts::PI / 4.0).tan()),
    ];
    let rod = topo::transform_rigid(
        &zprism(&disc, (-2.0, 2.0)),
        &geom_core::test_support::rigid(
            &Affine3::rotation_about_axis(
                Point3::new(0.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
                core::f64::consts::FRAC_PI_2,
            ),
            tol(),
        ),
        tol(),
    )
    .unwrap();
    let wide = finished(
        "the block",
        brick((-1.0, 1.0), (0.5, 1.5), (-0.5, 2.5), tol()),
        tol(),
    );
    let rod = finished("the rod", rod, tol());
    let holed = match topo::subtract(&wide, &rod, tol()).unwrap() {
        BooleanResult::Body(r) => finished("the holed block", topo::Body::clone(&r.body), tol()),
        BooleanResult::Empty => panic!("the holed block is not empty"),
    };
    let records = edge_face_records(&topo::union(&apex(), &holed, tol()).unwrap());
    let [([p, q], plane)] = records.as_slice() else {
        panic!("one edge-face record: {records:?}");
    };
    let on_apex = |p: &Point3<f64>| p.x.abs() < 1e-9 && (p.y - 0.5).abs() < 1e-9;
    assert!(
        on_apex(p) && on_apex(q),
        "the record's edge is the apex: {p:?} {q:?}"
    );
    let (n, d) = plane.expect("the record's face is planar");
    assert!(
        n.y.abs() > 1.0 - 1e-9 && (d.abs() - 0.5).abs() < 1e-9,
        "the record's face lies on y = 0.5: {n:?} {d}"
    );
}

/// **A face whose edge runs half a band beside the apex holds none of
/// it.** The partner's face on `y = 0.5` spans `x ∈ [δ, 1]` with `δ`
/// half a band either side of the apex: the apex runs along the face's
/// boundary, a touch of two edges, and the union records it in no face.
#[test]
fn an_apex_in_band_of_a_faces_edge_is_recorded_in_no_face() {
    let zero = Band::linear(tol()).unwrap().zero();
    for k in [-0.5, 0.5] {
        let d = k * zero;
        let b = finished(
            "the block",
            brick((d, 1.0), (0.5, 1.5), (-0.5, 2.5), tol()),
            tol(),
        );
        let r = topo::union(&apex(), &b, tol()).unwrap();
        let records = edge_face_records(&r);
        assert!(
            records.is_empty(),
            "k = {k}: no edge-face record: {records:?}"
        );
    }
}
