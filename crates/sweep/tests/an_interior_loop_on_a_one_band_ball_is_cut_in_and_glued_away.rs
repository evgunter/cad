//! **A section circle inside a one-band sphere face that crosses no edge
//! of either face builds, and the result carries no trace of the cut that
//! built it.**
//!
//! A full revolve's ball is one face whose seam is its pole-to-pole wrap
//! edge. A plane whose circle on it misses that meridian leaves a loop
//! inside both faces, which no event marks: the interior-loop guard
//! proves it, the crossings path cuts the sphere face in along the
//! meridian of its chart through the circle and re-enters, and the output
//! merge glues the cut away, the face keeping one wrap edge into each
//! pole it holds. The rows:
//!
//! - the unit ball poled on `y`, its seam on `+x`, against the slab
//!   `x ∈ (−0.5, 0.5)`: the circle `x = −0.5` misses the seam;
//! - the same ball against the box `x < 0.8, z > −0.7`, whose circle
//!   `z = −0.7` lies parallel to the seam's plane beside one crossing it;
//! - the same ball less the cap `x > 0.5` (a trimmed one-band face)
//!   against the slab `x ∈ (−0.5, 0.3)` and the half-space `x < −0.5`.
//!
//! Each runs ∪, ∩ and both differences in both operand orders, read
//! through `differential::outcome` (tiers, certificate, legal operand,
//! closed-form volume). The trace reading: no merge was skipped, no edge
//! separates two faces on one sphere record, every edge with one sphere
//! face both sides runs on the authored seam's half-meridian
//! (`z = 0, x > 0`), never on the cut's (`x < 0`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Tol, Vec3};
use sweep::test_support::{ball_poled, brick, finished};
use topo::{AtRestBody, Body, BooleanResult};

use crate::common::differential::{every_op_both_orders, outcome};
use crate::common::oracles::{ball_volume, cap_volume};

fn tol() -> Tol {
    Tol::witness()
}

/// The unit ball about the origin, poled on `y`: its wrap edge runs on
/// the half-meridian `z = 0, x > 0`.
fn ball() -> AtRestBody<f64> {
    finished(
        "the ball",
        ball_poled(1.0, Vec3::zero(), Vec3::new(0.0, 1.0, 0.0), tol()),
        tol(),
    )
}

/// The brick `x ∈ xs` across the whole ball in `y` and `z`.
fn slab(xs: (f64, f64)) -> AtRestBody<f64> {
    finished(
        "the slab",
        brick::<f64>(xs, (-2.0, 2.0), (-2.0, 2.0), tol()),
        tol(),
    )
}

/// The ball less its cap `x > 0.5`: a trimmed one-band sphere face, its
/// wrap edge cut where the cap's circle crosses it.
fn trimmed() -> AtRestBody<f64> {
    let cut = topo::subtract(&ball(), &slab((0.5, 2.0)), tol()).unwrap();
    finished(
        "the trimmed ball",
        Body::clone(&cut.body().unwrap().body),
        tol(),
    )
}

/// The trace reading (module docs), of one built result.
fn assert_no_trace(label: &str, r: &BooleanResult<f64>) {
    let Some(out) = r.body() else {
        return;
    };
    assert!(
        out.naming.merge_skipped.is_empty(),
        "{label}: the output merge skipped {:?}",
        out.naming.merge_skipped
    );
    let body = &*out.body;
    let point = |v| topo::readback::vertex_point(body, v).unwrap();
    for (e, _) in body.edges() {
        let sides = topo::readback::edge_sides(body, e).unwrap();
        let (f1, f2) = sides.faces();
        let (k1, k2) = sides.surfaces();
        let sphere = matches!(
            topo::query::face_surface_kind(body, f1),
            Some(geom::SurfaceKind::Sphere)
        );
        if !sphere || k1 != k2 {
            continue;
        }
        assert_eq!(
            f1, f2,
            "{label}: {e:?} divides two faces of one sphere: a cut survives"
        );
        let (u, v) = body.edge_vertices(e).unwrap();
        let ends = [point(u), point(v)];
        assert!(
            ends.iter().all(|p| p.z.abs() < 1e-9 && p.x > -1e-9) && ends.iter().any(|p| p.x > 0.25),
            "{label}: {e:?} inside its sphere face runs off the authored seam's half-meridian: \
             {ends:?}"
        );
    }
}

/// Runs every op both ways and reads each line and its trace.
fn every_op_sound(a: &AtRestBody<f64>, b: &AtRestBody<f64>, v: (f64, f64, f64)) {
    for (op, r, want) in every_op_both_orders(a, b, v, tol()) {
        if let Ok(built) = &r {
            assert_no_trace(op, built);
        }
        let line = outcome(r, want, tol());
        assert!(line.starts_with("OK SOUND"), "{op}: {line}");
    }
}

/// The ball against the slab `|x| < 0.5`: the band between the circles
/// holds both poles, one face with the authored wrap edge into each.
#[test]
fn a_ball_and_a_slab_whose_far_circle_misses_the_seam() {
    let (b, s) = (ball(), slab((-0.5, 0.5)));
    let v = ball_volume(1.0);
    let both = v - 2.0 * cap_volume(1.0, 0.5);
    every_op_sound(&b, &s, (v, 16.0, both));
    let band = topo::intersect(&b, &s, tol()).unwrap();
    let body = &*band.body().unwrap().body;
    let spheres = body
        .faces()
        .filter(|&(f, _)| {
            matches!(
                topo::query::face_surface_kind(body, f),
                Some(geom::SurfaceKind::Sphere)
            )
        })
        .count();
    assert_eq!(spheres, 1, "the band is one face");
    assert_eq!(body.faces().count(), 3, "the band and the two discs");
}

/// The ball less its cap `x > 0.5` against the slab `x ∈ (−0.5, 0.3)`:
/// the circle `x = 0.3` crosses what is left of the seam, `x = −0.5`
/// does not.
#[test]
fn a_trimmed_ball_and_a_slab() {
    let (t, s) = (trimmed(), slab((-0.5, 0.3)));
    let vt = ball_volume(1.0) - cap_volume(1.0, 0.5);
    let both = ball_volume(1.0) - cap_volume(1.0, 0.5) - cap_volume(1.0, 0.7);
    every_op_sound(&t, &s, (vt, 0.8 * 16.0, both));
}

/// The ball less its cap `x > 0.5` against the half-space `x < −0.5`:
/// no crossing anywhere, the no-crossings path's cut-in on a trimmed
/// one-band face.
#[test]
fn a_trimmed_ball_and_a_far_half_space() {
    let (t, h) = (trimmed(), slab((-2.0, -0.5)));
    let vt = ball_volume(1.0) - cap_volume(1.0, 0.5);
    every_op_sound(&t, &h, (vt, 1.5 * 16.0, cap_volume(1.0, 0.5)));
}

/// The ball against the box `x < 0.8, z > −0.7`: the circle `x = 0.8`
/// crosses the seam, so the op takes the crossings path, and the circle
/// `z = −0.7` lies in a plane parallel to the seam's, missing every
/// meridian of that plane, the class a ball cut in two lunes met too.
#[test]
fn a_circle_parallel_to_the_seams_plane_beside_a_crossing_one() {
    let (b, x) = (
        ball(),
        finished(
            "the box",
            brick::<f64>((-2.0, 0.8), (-2.0, 2.0), (-0.7, 2.0), tol()),
            tol(),
        ),
    );
    let v = ball_volume(1.0);
    let both = v - cap_volume(1.0, 0.2) - cap_volume(1.0, 0.3);
    every_op_sound(&b, &x, (v, 2.8 * 4.0 * 2.7, both));
}
