//! **A section circle inside a one-band sphere face that crosses no edge
//! of either face builds, and the result carries no trace of the cut that
//! built it.**
//!
//! A full revolve's ball is one face whose seam is its pole-to-pole wrap
//! edge. A plane whose circle on it misses that meridian leaves a loop
//! inside both faces, which no event marks: the interior-loop guard
//! proves it, the crossings path cuts the sphere face in along the
//! meridian of its chart through the circle and re-enters, and the output
//! merge glues the cut away. On a sphere every same-face edge the cut
//! leaves that is no spur is a bridge, whatever the face holds: a pole
//! and its wrap edge (a band), the seam from rim to rim (a zone), or no
//! seam at all (an annulus clear of it). The rows:
//!
//! - the unit ball poled on `y`, its seam on `+x`, against the slab
//!   `x ∈ (−0.5, 0.5)`: the circle `x = −0.5` misses the seam;
//! - the same ball against the box `x < 0.8, z > −0.7`, whose circle
//!   `z = −0.7` lies parallel to the seam's plane beside one crossing it;
//! - the same ball less the cap `x > 0.5` (a trimmed one-band face)
//!   against the slab `x ∈ (−0.5, 0.3)`, the half-space `x < −0.5`, and
//!   the slab `x ∈ (−0.8, −0.5)`, whose ∩ is an annulus holding neither
//!   a pole nor the seam;
//! - the zone `|y| < 0.5` of the ball, its seam running rim to rim and no
//!   pole in it, against `x < −0.9`, `x > −0.9`, and the box
//!   `x > −0.9, y < 0.45` (on the crossings path): the cut runs rim to
//!   rim too, into no tip.
//!
//! Each runs ∪, ∩ and both differences in both operand orders, read
//! through `differential::outcome` (tiers, certificate, legal operand,
//! closed-form volume), and through `cut_trace::assert_no_cut_trace`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Tol, Vec3};
use sweep::test_support::{ball_poled, brick, finished};
use topo::{AtRestBody, Body};

use crate::common::differential::{every_op_both_orders, outcome};
use crate::common::oracles::{ball_volume, cap_volume};
use sweep::test_support::cut_trace::assert_no_cut_trace;

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

/// The brick `xs × ys × zs`.
fn brick_of(xs: (f64, f64), ys: (f64, f64), zs: (f64, f64)) -> AtRestBody<f64> {
    finished("the brick", brick::<f64>(xs, ys, zs, tol()), tol())
}

/// The brick `x ∈ xs` across the whole ball in `y` and `z`.
fn slab(xs: (f64, f64)) -> AtRestBody<f64> {
    brick_of(xs, (-2.0, 2.0), (-2.0, 2.0))
}

/// The built body of an op, at rest.
fn built(r: Result<topo::BooleanResult<f64>, topo::BooleanError>) -> AtRestBody<f64> {
    finished(
        "the operand",
        Body::clone(&r.unwrap().body().unwrap().body),
        tol(),
    )
}

/// The ball less its cap `x > 0.5`: a trimmed one-band sphere face, its
/// wrap edge cut where the cap's circle crosses it.
fn trimmed() -> AtRestBody<f64> {
    built(topo::subtract(&ball(), &slab((0.5, 2.0)), tol()))
}

/// The zone `|y| < 0.5` of the ball: one sphere face, its seam from rim
/// to rim, no pole in it.
fn zone() -> AtRestBody<f64> {
    built(topo::intersect(
        &ball(),
        &brick_of((-2.0, 2.0), (-0.5, 0.5), (-2.0, 2.0)),
        tol(),
    ))
}

/// Runs every op both ways and reads each line and its trace.
fn every_op_sound(a: &AtRestBody<f64>, b: &AtRestBody<f64>, v: (f64, f64, f64)) {
    for (op, r, want) in every_op_both_orders(a, b, v, tol()) {
        if let Ok(built) = &r {
            assert_no_cut_trace(op, built, [a, b]);
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
    every_op_sound(&b, &s, (v, 16.0, v - 2.0 * cap_volume(1.0, 0.5)));
}

/// The ball against the box `x < 0.8, z > −0.7`: the circle `x = 0.8`
/// crosses the seam, so the op takes the crossings path, and the circle
/// `z = −0.7` lies in a plane parallel to the seam's, missing every
/// meridian of that plane, the class a ball cut in two lunes met too.
#[test]
fn a_circle_parallel_to_the_seams_plane_beside_a_crossing_one() {
    let (b, x) = (ball(), brick_of((-2.0, 0.8), (-2.0, 2.0), (-0.7, 2.0)));
    let v = ball_volume(1.0);
    let both = v - cap_volume(1.0, 0.2) - cap_volume(1.0, 0.3);
    every_op_sound(&b, &x, (v, 2.8 * 4.0 * 2.7, both));
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

/// The ball less its cap `x > 0.5` against the slab `x ∈ (−0.8, −0.5)`:
/// the ∩ is an annulus about `−x` holding neither a pole nor the seam,
/// its cut running from rim to rim, which the merge leaves seamless.
#[test]
fn a_trimmed_ball_and_a_slab_cutting_an_annulus_off_its_seam() {
    let (t, s) = (trimmed(), slab((-0.8, -0.5)));
    let vt = ball_volume(1.0) - cap_volume(1.0, 0.5);
    let both = cap_volume(1.0, 0.5) - cap_volume(1.0, 0.2);
    every_op_sound(&t, &s, (vt, 0.3 * 16.0, both));
}

/// The zone `|y| < 0.5` against `x < −0.9`, `x > −0.9` and the box
/// `x > −0.9, y < 0.45`: the cap `x < −0.9` (radius `0.436`) lies inside
/// the zone, the box's face `y = 0.45` cuts it in a circle crossing the
/// seam, and each cut runs from rim to rim into no tip.
#[test]
fn a_zone_against_planes_whose_circles_miss_its_seam() {
    let z = zone();
    let vz = ball_volume(1.0) - 2.0 * cap_volume(1.0, 0.5);
    let tip = cap_volume(1.0, 0.1);
    every_op_sound(&z, &slab((-2.0, -0.9)), (vz, 1.1 * 16.0, tip));
    every_op_sound(&z, &slab((-0.9, 2.0)), (vz, 2.9 * 16.0, vz - tip));
    let below = ball_volume(1.0) - cap_volume(1.0, 0.5) - cap_volume(1.0, 0.55) - tip;
    every_op_sound(
        &z,
        &brick_of((-0.9, 2.0), (-2.0, 0.45), (-2.0, 2.0)),
        (vz, 2.9 * 2.45 * 4.0, below),
    );
}

/// The ball less its cap `x > 0.6`, hollowed by the concentric ball of
/// radius `0.5` turned half a turn about `y`, its seam on `z = 0, x < 0`,
/// against `x > −0.9`: the circle `x = −0.9` misses the outer seam, the
/// cut runs along the outer face's meridian on `−x`, and the inner ball's
/// seam lies in that half-plane too. The merge glues the cut away; the
/// inner seam is its own face's, which the op leaves as it found it.
#[test]
fn a_hollow_shell_whose_inner_seam_lies_in_the_cuts_half_plane() {
    use geom_core::{Affine3, Point3};
    let y = Vec3::new(0.0, 1.0, 0.0);
    let inner = topo::transform_rigid(
        &ball_poled(0.5, Vec3::zero(), y, tol()),
        &geom_core::test_support::rigid(
            &Affine3::rotation_about_axis(Point3::origin(), y, std::f64::consts::PI),
            tol(),
        ),
    )
    .unwrap();
    let inner = finished("the inner ball", inner, tol());
    let shell = built(topo::subtract(
        &built(topo::subtract(&ball(), &slab((0.6, 2.0)), tol())),
        &inner,
        tol(),
    ));
    let vs = ball_volume(1.0) - cap_volume(1.0, 0.4) - ball_volume(0.5);
    every_op_sound(
        &shell,
        &slab((-0.9, 2.0)),
        (vs, 2.9 * 16.0, vs - cap_volume(1.0, 0.1)),
    );
}
