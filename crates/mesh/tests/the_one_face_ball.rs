//! **The one-face ball meshes as its wrap edge's two sides** (DESIGN
//! `[topology]`, the seam clause and tiers 2/3): a sphere face whose loop
//! walks one wrap meridian pole to pole and back. Its chart image is
//! empty and both ends of the edge are tips, so the walk places the
//! edge's two traversals a period apart, the whole chart strip, rather
//! than as one zero-width column.
//!
//! No door mints a wrap tip yet, so each body is a full revolve cut by
//! Euler operators: the two meridians the revolve leaves at a pole or an
//! apex become one by `kef`, restated as the face's wrap edge.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;
use core::f64::consts::PI;
use geom_brep::EdgeDescriptionSpec;
use geom_core::{Band, Point2, Point3, Tol};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::construction::Construct;
use topo::{Body, HalfEdgeKey, VertexKey};

const DELTA: f64 = 0.05;
const NORTH: Point3<f64> = Point3::new(0.0, 1.0, 0.0);

fn tol() -> Tol {
    Tol::witness()
}

fn vertex_at(body: &Body<f64>, at: Point3<f64>) -> VertexKey {
    body.vertices()
        .find(|&(v, _)| {
            let p = *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
            (p - at).norm() < 1e-12
        })
        .map(|(v, _)| v)
        .expect("a vertex there")
}

fn leaving(body: &Body<f64>, v: VertexKey) -> Vec<HalfEdgeKey> {
    body.half_edges()
        .filter(|(_, h)| h.start == v)
        .map(|(he, _)| he)
        .collect()
}

/// The two meridians a full revolve leaves at `NORTH` made one wrap edge:
/// the second is killed and the first restated as the merged face's wrap
/// edge. Where `join`, the rim's two arcs are then joined back into one.
fn one_wrap_edge(mut body: Body<f64>, join: bool) -> Body<f64> {
    let out = leaving(&body, vertex_at(&body, NORTH));
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
    body
}

/// The unit ball, its two pole-to-pole bands.
fn two_band_ball() -> Body<f64> {
    revolved_about_y(
        vec![(Point2::new(0.0, -1.0), 1.0), (Point2::new(0.0, 1.0), 0.0)],
        Revolution::Full,
        tol(),
    )
}

/// **The one-face ball meshes watertight and encloses the ball.** It is
/// tier-3 valid, and its mesh passes the acceptance battery with the
/// unit ball's volume and area, as the two-band ball's does.
#[test]
fn the_one_face_ball_meshes_watertight_with_the_balls_volume() {
    let ball = one_wrap_edge(two_band_ball(), false);
    assert_eq!(ball.faces().count(), 1, "one face");
    assert_eq!(ball.edges().count(), 1, "one edge");
    assert_eq!(
        topo::validate_geometric(&ball, tol()),
        Ok(()),
        "the one-face ball is tier-3 valid"
    );
    let exact = Some((4.0 * PI / 3.0, 4.0 * PI));
    let one = common::check_mesh_acceptance(&ball, DELTA, exact);
    let two = common::check_mesh_acceptance(&two_band_ball(), DELTA, exact);
    let (v1, v2) = (
        mesh::validate::signed_volume(&one),
        mesh::validate::signed_volume(&two),
    );
    eprintln!(
        "one-face ball: {} triangles, volume {v1}; two-band ball: {} triangles, volume {v2}",
        mesh::validate::triangle_count(&one),
        mesh::validate::triangle_count(&two),
    );
}
