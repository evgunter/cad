//! **A sphere remnant whose loop walks its seam both ways meshes.** A
//! full revolve's ball is one face whose seam is its wrap edge, each pole
//! that edge's tip (DESIGN `[topology]`, the seam clause). A boolean that
//! keeps the seam keeps it in the remnant's loop, walked out and back; the
//! trimmed lane reads that edge as the chart's cut, and the remnant meshes
//! closed, at the body's volume.
//!
//! Each row first asserts the shape is there, so it cannot pass on a body
//! that lost it: a sphere face that is not the whole sphere and whose
//! loop holds one edge twice.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom::SurfaceKind;
use geom_core::{Point2, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{ball_poled_y, brick, finished, revolved_about_y};
use topo::{AtRestBody, Body};

fn tol() -> Tol {
    Tol::witness()
}

/// Whether some sphere face of `body` is trimmed (it has an edge off its
/// seam) and its outer loop walks one edge twice.
fn a_sphere_remnant_walks_its_seam_twice(body: &Body<f64>) -> bool {
    body.faces().any(|(fk, f)| {
        let sphere = body.face_surface(fk).expect("a live face").kind() == SurfaceKind::Sphere;
        let (_, halves) =
            topo::props::loop_edges(body.cycle_loop(f.outer).expect("a loop")).expect("a loop");
        let edges: Vec<_> = halves
            .iter()
            .map(|&h| body.get_half_edge(h).expect("a live half-edge").edge)
            .collect();
        let twice = edges
            .iter()
            .any(|e| edges.iter().filter(|&x| x == e).count() == 2);
        let off_seam = edges
            .iter()
            .any(|e| edges.iter().filter(|&x| x == e).count() == 1);
        sphere && twice && off_seam
    })
}

/// `body` meshes at each δ, passes `check_mesh`, and encloses `volume`
/// to within the mesh's deviation over its curved surface (`δ·area`,
/// `area` that surface's, bounded above): a planar face meshes exactly.
fn meshes_at_its_volume(what: &str, body: &Body<f64>, volume: f64, area: f64) {
    assert!(
        a_sphere_remnant_walks_its_seam_twice(body),
        "{what}: a sphere remnant keeps its seam, walked both ways"
    );
    for delta in [5e-2, 2e-2, 1e-2] {
        let mesh = mesh::tessellate(body, delta, tol())
            .unwrap_or_else(|e| panic!("{what}, δ = {delta}: meshes, got {e:?}"));
        mesh::validate::check_mesh(&mesh).unwrap_or_else(|e| panic!("{what}, δ = {delta}: {e:?}"));
        let got = mesh::validate::signed_volume(&mesh);
        assert!(
            (got - volume).abs() <= delta * area,
            "{what}, δ = {delta}: volume {got}, exact {volume}"
        );
    }
}

fn ball(r: f64, c: Vec3<f64>) -> AtRestBody<f64> {
    finished("the ball", ball_poled_y(r, c, tol()), tol())
}

/// A unit ball under a box face at `x = 0.5`, over `z ≥ 0`: the ball keeps
/// its seam, cut by the box's two faces through it.
#[test]
fn a_ball_under_a_box_face_meshes_with_its_seam() {
    let box_ = finished(
        "the box",
        brick::<f64>((0.5, 3.0), (-2.0, 2.0), (0.0, 2.0), tol()),
        tol(),
    );
    let out = topo::boolean::union(&box_, &ball(1.0, Vec3::new(0.0, 0.0, 0.0)), tol())
        .expect("the pair unions");
    let body = &out.body().expect("a body").body;
    // The ball's cap past `x = 0.5`, height `h`, halved by `z = 0`.
    let h: f64 = 0.5;
    let cap = PI * h * h * (3.0 - h) / 3.0;
    let volume = 4.0 * PI / 3.0 + 2.5 * 4.0 * 2.0 - cap / 2.0;
    meshes_at_its_volume("the ball under a box face", body, volume, 4.0 * PI);
}

/// Two unit balls `1.4` apart, the second leaning toward `y` so the
/// radical circle passes through the first ball's north pole: the pole is
/// a vertex of the section, and each remnant keeps its seam.
#[test]
fn a_section_through_a_pole_meshes_with_its_seam() {
    let c = Vec3::new(2.0, 2.0, 0.5);
    let s: f64 = 0.7;
    let lean = Vec3::new(1.4 * (1.0 - s * s).sqrt(), 1.4 * s, 0.0);
    let revolved = |c: Vec3<f64>| {
        let b = revolved_about_y(
            vec![(Point2::new(0.0, -1.0), 1.0), (Point2::new(0.0, 1.0), 0.0)],
            Revolution::Full,
            tol(),
        );
        let b =
            topo::transform_rigid(&b, &geom_core::RigidMotion::translation(c).unwrap()).unwrap();
        finished("the ball", b, tol())
    };
    let out =
        topo::boolean::union(&revolved(c), &revolved(c + lean), tol()).expect("the pair unions");
    let body = &out.body().expect("a body").body;
    let d: f64 = 1.4;
    let lens = PI * (4.0 + d) * (2.0 - d).powi(2) / 12.0;
    let volume = 2.0 * 4.0 * PI / 3.0 - lens;
    meshes_at_its_volume("the pair through a pole", body, volume, 8.0 * PI);
}
