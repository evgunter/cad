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
use geom::Curve3;
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec};
use geom_core::{Band, Point2, Point3, Tol};
use mesh::validate::{check_mesh, check_volume};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::construction::{Construct, FaceSurface, MefSite};
use topo::{Body, HalfEdgeKey, VertexKey};

const DELTA: f64 = 0.05;
const NORTH: Point3<f64> = Point3::new(0.0, 1.0, 0.0);
const SOUTH: Point3<f64> = Point3::new(0.0, -1.0, 0.0);
/// The unit ball's volume and area.
const BALL: (f64, f64) = (4.0 * PI / 3.0, 4.0 * PI);

fn tol() -> Tol {
    Tol::witness()
}

fn point(body: &Body<f64>, v: VertexKey) -> Point3<f64> {
    *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
}

fn vertex_at(body: &Body<f64>, at: Point3<f64>) -> VertexKey {
    body.vertices()
        .find(|&(v, _)| (point(body, v) - at).norm() < 1e-12)
        .map(|(v, _)| v)
        .expect("a vertex there")
}

fn leaving(body: &Body<f64>, v: VertexKey) -> Vec<HalfEdgeKey> {
    body.half_edges()
        .filter(|(_, h)| h.start == v)
        .map(|(he, _)| he)
        .collect()
}

/// The two meridians a full revolve leaves at `pole` made one wrap edge:
/// the second is killed and the first restated as the merged face's wrap
/// edge. Where `join`, the rim's two arcs are then joined back into one.
fn one_wrap_edge(mut body: Body<f64>, pole: Point3<f64>, join: bool) -> Body<f64> {
    let out = leaving(&body, vertex_at(&body, pole));
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

/// **The one-face ball meshes as the ball, and reverted as its void,
/// whichever pole its loop leaves from.** Killing the meridian at the
/// north pole leaves the loop's first side running south, and at the
/// south pole running north, so with the reversal the four rows take
/// every pairing of the side's direction with the face's sense, which is
/// what the period's sign is read from. Each ball is tier-3 valid, with
/// one face and one edge, and passes the acceptance battery with the
/// unit ball's volume and area; each reversal meshes watertight around
/// the void's volume.
#[test]
fn the_one_face_ball_meshes_as_the_ball_or_its_void_from_either_pole() {
    let (volume, area) = BALL;
    for (name, pole) in [("north", NORTH), ("south", SOUTH)] {
        let ball = one_wrap_edge(two_band_ball(), pole, false);
        assert_eq!(
            (ball.faces().count(), ball.edges().count()),
            (1, 1),
            "{name}: one face, one edge"
        );
        assert_eq!(
            topo::validate_geometric(&ball, tol()),
            Ok(()),
            "{name}: the one-face ball is tier-3 valid"
        );
        common::check_mesh_acceptance(&ball, DELTA, Some(BALL));

        let void = ball.revert().expect("the one-face ball reverts");
        let inside_out = mesh::tessellate(&void, DELTA, tol()).expect("the reverted ball meshes");
        assert_eq!(
            check_mesh(&inside_out),
            Ok(()),
            "{name}: the reverted ball is watertight"
        );
        assert_eq!(
            check_volume(&inside_out, -volume, area, DELTA),
            Ok(()),
            "{name}: the reverted ball encloses the void"
        );
    }
}

/// **A wrap edge carried by two edges is one strip.** The one-face ball
/// with its wrap edge split off the poles: its loop walks two edges down
/// and back, its chart image is still empty and its tips are still the
/// poles, so each side is one column across the split and the strip
/// meshes as the ball. The split vertex is joinable at rest, which tier 3
/// refuses and `tessellate` does not re-validate.
#[test]
fn a_split_wrap_edge_is_one_strip() {
    let mut ball = one_wrap_edge(two_band_ball(), NORTH, false);
    let wraps = ball.edges().map(|(e, _)| e).next().expect("the wrap edge");
    ball.split_edge(wraps, 0.7, tol())
        .expect("split the wrap edge off the poles");
    assert_eq!(ball.edges().count(), 2, "two edges");
    common::check_mesh_acceptance(&ball, DELTA, Some(BALL));
}

/// **A slit between the tips is not the strip.** The two-band ball with
/// one meridian cut at two points, a second arc stated on that meridian
/// between them (a slit face), and the other meridian killed. The ball
/// face's loop runs pole to pole and back, its spurs ending on both
/// poles, but its chart image keeps the slit's two arcs, so the face is
/// not the whole sphere: it is walked as any other rimless loop, and the
/// slit's two coincident arcs refuse it `PinchWedge`. Read as the strip,
/// the ball face would mesh as the whole sphere and leave the slit face
/// to refuse instead. The body is outside `tessellate`'s contract (tier 2
/// reads the spurs' ends as strut tips); the row pins which reading the
/// walk takes of it.
#[test]
fn a_slit_between_the_tips_is_not_the_strip() {
    let mut body = two_band_ball();
    let out = leaving(&body, vertex_at(&body, NORTH));
    let cut = body.get_half_edge(out[0]).unwrap().edge;
    let p = body.split_edge(cut, 1.0, tol()).expect("cut at P");
    let q = body.split_edge(p.new_edge, 2.0, tol()).expect("cut at Q");
    let (pv, qv) = (
        point(&body, p.vertex) - Point3::origin(),
        point(&body, q.vertex) - Point3::origin(),
    );
    let normal = pv.cross(qv) * (1.0 / pv.cross(qv).norm());
    let meridian = Curve3::circle(Point3::origin(), normal, 1.0, pv);
    let pq = body.get_edge(p.new_edge).unwrap();
    let from_p = [pq.he_plus, pq.he_minus]
        .into_iter()
        .find(|&he| body.get_half_edge(he).unwrap().start == p.vertex)
        .unwrap();
    let to_q = body.get_half_edge(from_p).unwrap().next;
    body.mef(
        MefSite::Chords {
            he1: from_p,
            he2: to_q,
        },
        EdgeCurveSpec::arc_of_circle(meridian, 0.0, pv.dot(qv).acos()).unwrap(),
        FaceSurface::Inherit,
        tol(),
    )
    .expect("the slit face");
    body.kef_describing(out[1], &[], tol())
        .expect("the bands merge into the ball face");
    let ball_face = body.face_of_half_edge(out[0]).unwrap();
    assert_eq!(body.faces().count(), 2, "the ball face and the slit face");
    assert_eq!(
        body.faces().next().map(|(f, _)| f),
        Some(ball_face),
        "the ball face is first in arena order, so the fold answers for it"
    );
    assert_eq!(
        mesh::tessellate(&body, DELTA, tol()).map(|_| ()),
        Err(mesh::TessellateError::PinchWedge { face: ball_face })
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

/// FNV-1a over [`common::dump`]: every position's bits, every patch's
/// triangles and every boundary polyline.
fn digest(mesh: &mesh::Mesh) -> u64 {
    common::dump(mesh)
        .bytes()
        .fold(0xcbf2_9ce4_8422_2325, |h, b| {
            (h ^ u64::from(b)).wrapping_mul(0x100_0000_01b3)
        })
}

/// **A wrap-tip cap meshes off its rim**: the hemisphere's and the cone's
/// curved face, its pole or apex reached by one wrap edge from its rim,
/// carries a rim, so its walk never reaches the wrap strip. Each cap is
/// tier-3 valid and passes the acceptance battery, and its mesh is pinned
/// to the bytes it had before the walk read a strip. A moved digest is a
/// changed mesh: re-pin it with its cause, or fix the change.
#[test]
fn a_wrap_tip_cap_meshes_off_its_rim() {
    for (name, body, exact, pinned) in [
        (
            "hemisphere",
            one_wrap_edge(hemisphere(), NORTH, true),
            (2.0 * PI / 3.0, 3.0 * PI),
            0x892d_5d6f_cbc6_497f_u64,
        ),
        (
            "cone",
            one_wrap_edge(cone(), NORTH, true),
            (PI / 3.0, PI * (1.0 + 2.0f64.sqrt())),
            0xc2c7_c2b9_1072_e474,
        ),
    ] {
        assert_eq!(
            topo::validate_geometric(&body, tol()),
            Ok(()),
            "the {name} cap is tier-3 valid"
        );
        let m = common::check_mesh_acceptance(&body, DELTA, Some(exact));
        assert_eq!(digest(&m), pinned, "the {name} cap's mesh bytes");
    }
}
