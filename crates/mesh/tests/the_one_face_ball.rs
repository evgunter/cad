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
use mesh::validate::{check_volume, signed_volume};
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
/// tier-3 valid; its mesh passes the acceptance battery, and its volume
/// is the unit ball's to within the chordal deficit, as the two-band
/// ball's is. Reverted, it meshes as the void the reversal makes: the
/// period between its sides is signed by the face's sense, and a strip
/// wound against it would mesh inside out.
#[test]
fn the_one_face_ball_meshes_watertight_with_the_balls_volume() {
    let ball = one_wrap_edge(two_band_ball(), false);
    assert_eq!(
        (ball.faces().count(), ball.edges().count()),
        (1, 1),
        "one face, one edge"
    );
    assert_eq!(
        topo::validate_geometric(&ball, tol()),
        Ok(()),
        "the one-face ball is tier-3 valid"
    );
    let (volume, area) = (4.0 * PI / 3.0, 4.0 * PI);
    let one = common::check_mesh_acceptance(&ball, DELTA, Some((volume, area)));
    let two = common::check_mesh_acceptance(&two_band_ball(), DELTA, Some((volume, area)));
    for (name, m) in [("one-face", &one), ("two-band", &two)] {
        assert_eq!(
            check_volume(m, volume, area, DELTA),
            Ok(()),
            "the {name} ball encloses the unit ball"
        );
    }
    let two_band = signed_volume(&two);
    assert_eq!(
        check_volume(&one, two_band, area, DELTA),
        Ok(()),
        "the one-face ball encloses the two-band ball's volume"
    );

    let void = ball.revert().expect("the one-face ball reverts");
    let inside_out = mesh::tessellate(&void, DELTA, tol()).expect("the reverted ball meshes");
    assert_eq!(
        mesh::validate::check_mesh(&inside_out),
        Ok(()),
        "the reverted ball: watertight"
    );
    assert_eq!(
        check_volume(&inside_out, -volume, area, DELTA),
        Ok(()),
        "the reverted ball encloses the void"
    );
}

/// The unit hemisphere `y ≥ 0` on its base disk.
fn hemisphere() -> Body<f64> {
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
fn cone() -> Body<f64> {
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

/// **A wrap-tip cap meshes off its rim**: the hemisphere's and the cone's
/// curved face, its pole or apex reached by one wrap edge from its rim,
/// carries a rim, so its walk never reaches the wrap strip.
#[test]
fn a_wrap_tip_cap_meshes_off_its_rim() {
    for (name, body, exact) in [
        (
            "hemisphere",
            one_wrap_edge(hemisphere(), true),
            (2.0 * PI / 3.0, 3.0 * PI),
        ),
        (
            "cone",
            one_wrap_edge(cone(), true),
            (PI / 3.0, PI * (1.0 + 2.0f64.sqrt())),
        ),
    ] {
        assert_eq!(
            topo::validate_geometric(&body, tol()),
            Ok(()),
            "the {name} cap is tier-3 valid"
        );
        let m = common::check_mesh_acceptance(&body, DELTA, Some(exact));
        assert_eq!(
            check_volume(&m, exact.0, exact.1, DELTA),
            Ok(()),
            "the {name} cap encloses its solid"
        );
    }
}

/// **A wrap edge carried by two edges is one strip.** The one-face ball
/// with its wrap edge split off the poles: its loop walks two edges down
/// and back, its chart image is still empty and its tips are still the
/// poles, so each side is one column across the split (#653's runs) and
/// the strip meshes as the ball. The split vertex is joinable at rest,
/// which tier 3 refuses and `tessellate` does not re-validate.
#[test]
fn a_split_wrap_edge_is_one_strip() {
    let mut ball = one_wrap_edge(two_band_ball(), false);
    let wraps = ball.edges().map(|(e, _)| e).next().expect("the wrap edge");
    ball.split_edge(wraps, 0.7, tol())
        .expect("split the wrap edge off the poles");
    assert_eq!(ball.edges().count(), 2, "two edges");
    common::check_mesh_acceptance(&ball, DELTA, Some((4.0 * PI / 3.0, 4.0 * PI)));
}
