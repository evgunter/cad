//! **The one-face ball meshes as its wrap edge's two sides** (DESIGN
//! `[topology]`, the seam clause and tiers 2/3): a sphere face whose loop
//! walks one wrap meridian pole to pole and back. Its chart image is
//! empty and both ends of the edge are tips, so the walk places the
//! edge's two traversals a period apart, the whole chart strip, rather
//! than as one zero-width column.
//!
//! A full revolve mints the tips: its curved wall reaches each pole or
//! apex by one wrap edge.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;
use core::f64::consts::PI;
use geom::Curve3;
use geom_brep::EdgeCurveSpec;
use geom_core::{Point2, Point3, Tol};
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

/// A full revolve reaches `pole` by one wrap edge: the body as built.
fn one_wrap_edge(body: Body<f64>, pole: Point3<f64>) -> Body<f64> {
    let out = leaving(&body, vertex_at(&body, pole));
    assert_eq!(out.len(), 1, "one wrap edge leaves the pole");
    body
}

/// The unit ball, one face across its one meridian.
fn ball() -> Body<f64> {
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
        let ball = one_wrap_edge(ball(), pole);
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

/// **A wrap edge carried by two or three edges is one strip, whichever
/// half the walk opens on.** The one-face ball with its wrap edge split
/// off the poles, at points across the meridian, from either pole: its
/// loop walks the edges down and back, its chart image is still empty and
/// its tips are still the poles, so each side is one column across the
/// splits and the strip meshes as the ball, and reverted as its void. The
/// split moves the loop's first half-edge off the near tip, so the lift's
/// near half is not the identity on every row: the strip reads the far
/// half's lift relative to the near half's, not the far half's alone. The
/// split vertices are joinable at rest, which tier 3 refuses and
/// `tessellate` does not re-validate.
#[test]
fn a_split_wrap_edge_is_one_strip_whichever_half_the_walk_opens_on() {
    let (volume, area) = BALL;
    for (name, pole) in [("north", NORTH), ("south", SOUTH)] {
        for t in [0.05, 0.7, 1.5, 2.5, 3.1] {
            let mut ball = one_wrap_edge(ball(), pole);
            let wraps = ball.edges().map(|(e, _)| e).next().expect("the wrap edge");
            let split = ball
                .split_edge(wraps, t, tol())
                .unwrap_or_else(|e| panic!("{name} t {t}: split the wrap edge: {e:?}"));
            assert_eq!(ball.edges().count(), 2, "{name} t {t}: two edges");
            common::check_mesh_acceptance(&ball, DELTA, Some(BALL));
            let void = ball.revert().expect("the split ball reverts");
            let inside_out = mesh::tessellate(&void, DELTA, tol())
                .unwrap_or_else(|e| panic!("{name} t {t}: the reverted ball meshes: {e:?}"));
            assert_eq!(
                check_volume(&inside_out, -volume, area, DELTA),
                Ok(()),
                "{name} t {t}: the reverted split ball encloses the void"
            );

            let mut thrice = ball.clone();
            thrice
                .split_edge(split.new_edge, 0.5 * (t + PI), tol())
                .unwrap_or_else(|e| panic!("{name} t {t}: split the far piece: {e:?}"));
            assert_eq!(thrice.edges().count(), 3, "{name} t {t}: three edges");
            common::check_mesh_acceptance(&thrice, DELTA, Some(BALL));
        }
    }
}

/// **The one-face spindle meshes as its revolve.** A lemon, a spindle
/// torus's outer sheet whose two apexes are the singular points, revolved
/// fully into one face with one wrap edge from either apex: the strip's
/// turn round each apex is the lift's, on a chart whose singular points
/// are sheet apexes, not a sphere's poles. It meshes watertight with its
/// revolve's volume, and reverted as its void, at three bulges.
#[test]
fn the_one_face_spindle_meshes_as_its_revolve() {
    for bulge in [0.3, 0.6, 0.9] {
        for (name, pole) in [("north", NORTH), ("south", SOUTH)] {
            let lemon = || {
                revolved_about_y(
                    vec![
                        (Point2::new(0.0, -1.0), bulge),
                        (Point2::new(0.0, 1.0), 0.0),
                    ],
                    Revolution::Full,
                    tol(),
                )
            };
            let two_bands = mesh::validate::signed_volume(
                &mesh::tessellate(&lemon(), DELTA, tol()).expect("the lemon meshes"),
            );
            let one = one_wrap_edge(lemon(), pole);
            let mesh = mesh::tessellate(&one, DELTA, tol())
                .unwrap_or_else(|e| panic!("bulge {bulge} {name}: the one-face lemon: {e:?}"));
            assert_eq!(
                check_mesh(&mesh),
                Ok(()),
                "bulge {bulge} {name}: the one-face lemon is watertight"
            );
            let volume = mesh::validate::signed_volume(&mesh);
            assert!(
                (volume - two_bands).abs() < 0.05 * two_bands,
                "bulge {bulge} {name}: volume {volume} against the revolve's {two_bands}"
            );
            let void = mesh::tessellate(&one.revert().expect("reverts"), DELTA, tol())
                .unwrap_or_else(|e| panic!("bulge {bulge} {name}: the reverted lemon: {e:?}"));
            let inside = mesh::validate::signed_volume(&void);
            assert!(
                (inside + two_bands).abs() < 0.05 * two_bands,
                "bulge {bulge} {name}: void volume {inside} against −{two_bands}"
            );
        }
    }
}

/// **A slit between the tips is not the strip.** The one-face ball with
/// its meridian cut at two points and a second arc stated on that
/// meridian between them (a slit face). The ball
/// face's loop runs pole to pole and back, its spurs ending on both
/// poles, but its chart image keeps the slit's two arcs, so the face is
/// not read as the strip: it is walked as any other rimless loop, its
/// columns off its lift. The lift turns the face a whole turn round each
/// pole, the slit's two arcs a turn apart (`topo::pcurves`' slit rule), so
/// the ball face meshes as the sphere cut by the slit, and the refusal is
/// the slit face's: its two coincident arcs open one column
/// (`SingleColumnCurvedFace`). The body is outside `tessellate`'s contract
/// (tier 2 reads the spurs' ends as strut tips); the row pins which
/// reading the walk takes of it.
#[test]
fn a_slit_between_the_tips_is_not_the_strip() {
    let mut body = ball();
    let out = leaving(&body, vertex_at(&body, NORTH));
    let cut = body.get_half_edge(out[0]).unwrap().edge;
    let p = body.split_edge(cut, 1.0, tol()).expect("cut at P");
    let q = body.split_edge(p.new_edge, 2.0, tol()).expect("cut at Q");
    let (pv, qv) = (
        point(&body, p.vertex) - Point3::origin(),
        point(&body, q.vertex) - Point3::origin(),
    );
    // The new arc runs Q to P, so `mef` mints the slit face as the new
    // face and the ball face keeps its arena slot ahead of it.
    let normal = qv.cross(pv) * (1.0 / qv.cross(pv).norm());
    let meridian = Curve3::circle(Point3::origin(), normal, 1.0, qv);
    let pq = body.get_edge(p.new_edge).unwrap();
    let from_p = [pq.he_plus, pq.he_minus]
        .into_iter()
        .find(|&he| body.get_half_edge(he).unwrap().start == p.vertex)
        .unwrap();
    let to_q = body.get_half_edge(from_p).unwrap().next;
    body.mef(
        MefSite::Chords {
            he1: to_q,
            he2: from_p,
        },
        EdgeCurveSpec::arc_of_circle(meridian, 0.0, pv.dot(qv).acos()).unwrap(),
        FaceSurface::Inherit,
        tol(),
    )
    .expect("the slit face");
    let ball_face = body.face_of_half_edge(out[0]).unwrap();
    assert_eq!(body.faces().count(), 2, "the ball face and the slit face");
    assert_eq!(
        body.faces().next().map(|(f, _)| f),
        Some(ball_face),
        "the ball face is first in arena order, so the fold meshes it before the slit face"
    );
    let slit_face = body
        .faces()
        .map(|(f, _)| f)
        .find(|&f| f != ball_face)
        .unwrap();
    assert_eq!(
        mesh::tessellate(&body, DELTA, tol()).map(|_| ()),
        Err(mesh::TessellateError::SingleColumnCurvedFace {
            face: slit_face,
            surface: geom::SurfaceKind::Sphere,
        })
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
/// changed mesh: re-pin it with its cause, or fix the change. Re-pinned
/// when the revolve came to build the cap's wall as one band (PR 7414):
/// the body is the revolve's own rather than two bands merged by hand,
/// so its keys and arena order moved.
#[test]
fn a_wrap_tip_cap_meshes_off_its_rim() {
    for (name, body, exact, pinned) in [
        (
            "hemisphere",
            one_wrap_edge(hemisphere(), NORTH),
            (2.0 * PI / 3.0, 3.0 * PI),
            0xa788_be8c_4cab_dfcc_u64,
        ),
        (
            "cone",
            one_wrap_edge(cone(), NORTH),
            (PI / 3.0, PI * (1.0 + 2.0f64.sqrt())),
            0xd080_9d14_121d_fcc8,
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
