//! **A pole or a cone's apex reached by one wrap edge** (DESIGN
//! `[topology]`'s seam clause): a full revolve's curved wall is one band,
//! its seam meridian the wrap edge and the pole or apex its tip. The
//! suites that read one take it from the revolve here.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use geom_core::{Point2, Point3, Tol};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::construction::Construct;
use topo::{Body, EdgeKey, FaceKey, HalfEdgeKey, VertexKey};

fn tol() -> Tol {
    Tol::witness()
}

/// The pole, or the apex.
pub const NORTH: Point3<f64> = Point3::new(0.0, 1.0, 0.0);

/// The unit hemisphere `y ≥ 0` on its base disk: one cap face, its seam
/// meridian a wrap edge from the equator to its tip at the pole.
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

/// **The revolve's wrap edge at the pole**: the one edge leaving the tip,
/// its face, and the tip. Where `join`, the body is the revolve as built;
/// where not, its rim, the one closed edge besides the wrap edge, is split
/// at its antipode, leaving a vertex there for a spur to stand on.
/// Returns the face, the wrap edge and its tip.
pub fn one_wrap_edge(mut body: Body<f64>, join: bool) -> (Body<f64>, FaceKey, EdgeKey, VertexKey) {
    let tip = vertex_at(&body, NORTH);
    let out = leaving(&body, tip);
    assert_eq!(out.len(), 1, "one wrap edge leaves the pole");
    let wraps = body.get_half_edge(out[0]).unwrap().edge;
    if !join {
        let rim = body
            .edges()
            .map(|(e, _)| e)
            .find(|&e| e != wraps && body.edge_vertices(e).is_some_and(|(a, b)| a == b))
            .expect("the rim is one closed edge");
        let (t0, t1) = body
            .get_curve_geom(body.get_edge(rim).unwrap().curve)
            .unwrap()
            .certified()
            .unwrap()
            .params();
        body.split_edge(rim, 0.5 * (t0 + t1), tol())
            .expect("the rim splits at its antipode");
    }
    let face = body.face_of_half_edge(out[0]).unwrap();
    (body, face, wraps, tip)
}
