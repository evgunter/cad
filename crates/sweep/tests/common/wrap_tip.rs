//! **A pole or a cone's apex reached by one wrap edge** (DESIGN
//! `[topology]`'s seam clause): a full revolve's two meridians at the
//! singular point made one by `kef`, restated as the face's wrap edge.
//! No door mints a tip yet, so the suites that read one build it here.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use geom_brep::EdgeDescriptionSpec;
use geom_core::{Band, Point2, Point3, Tol};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::construction::Construct;
use topo::{Body, EdgeKey, FaceKey, HalfEdgeKey, VertexKey};

fn tol() -> Tol {
    Tol::witness()
}

/// The pole, or the apex.
pub const NORTH: Point3<f64> = Point3::new(0.0, 1.0, 0.0);

/// The unit hemisphere `y ≥ 0` on its base disk: two quarter-sphere lunes
/// meeting along two meridians at the pole.
pub fn hemisphere() -> Body<f64> {
    revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), (PI / 8.0).tan()),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
        tol(),
    )
}

/// The cone of radius and height 1 on its base disk, apex at `NORTH`.
pub fn cone() -> Body<f64> {
    revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
        tol(),
    )
}

pub fn point(body: &Body<f64>, v: VertexKey) -> Point3<f64> {
    *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
}

/// The vertex at `at`.
pub fn vertex_at(body: &Body<f64>, at: Point3<f64>) -> VertexKey {
    body.vertices()
        .find(|&(v, _)| (point(body, v) - at).norm() < 1e-12)
        .map(|(v, _)| v)
        .expect("a vertex there")
}

/// The half-edges leaving `v`, in half-edge-slot order.
pub fn leaving(body: &Body<f64>, v: VertexKey) -> Vec<HalfEdgeKey> {
    body.half_edges()
        .filter(|(_, h)| h.start == v)
        .map(|(he, _)| he)
        .collect()
}

/// **The two meridians at the pole made one wrap edge**: the second is
/// killed, merging the two lunes into the face it leaves, and the first is
/// restated as that face's wrap edge. Where `join`, the join then takes
/// the rim's two arcs back into one closed edge at the vertex the kill
/// left with nothing else at it; where not, that vertex is left for a
/// spur to stand on. Returns the face, the wrap edge and its tip.
pub fn one_wrap_edge(mut body: Body<f64>, join: bool) -> (Body<f64>, FaceKey, EdgeKey, VertexKey) {
    let tip = vertex_at(&body, NORTH);
    let out = leaving(&body, tip);
    assert_eq!(out.len(), 2, "two meridians leave the pole");
    let wraps = body.get_half_edge(out[0]).unwrap().edge;
    let face = body.face_of_half_edge(out[0]).unwrap();
    let surface = body.get_face(face).unwrap().surface;
    body.kef_describing(
        out[1],
        &[(wraps, EdgeDescriptionSpec::wrap(surface))],
        tol(),
    )
    .expect("the survivor restates as the face's wrap edge");
    if join {
        body.join_edges(Band::linear(tol()).unwrap(), tol())
            .expect("the rim joins");
    }
    let face = body.face_of_half_edge(out[0]).unwrap();
    (body, face, wraps, tip)
}
