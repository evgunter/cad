//! **BLEND-4 — the convexity-parametric fillet corner** (the concave
//! rolling ball, issue 644).
//!
//! The first rows here are the unit's MEASUREMENTS, committed before
//! anything moved: the plan's precondition is that `corner_ball`'s
//! unexercised concave arm is verified — measured, not assumed —
//! before anything builds on it, and that the convex-hardcoded
//! consumers (the feet's sign, the stored chart) are pinned as found.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::{Point3, Vec3};
use sweep::blend::arms::corner_ball;

fn p(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}
fn v(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}

/// **MEASUREMENT 1 — the concave arm rests the ball in the void.**
///
/// A concave trihedron's outward normals point away from the material,
/// into the wedge of void the three walls enclose. The rolling ball at
/// rest there is at distance `r` from every wall ON THE VOID SIDE:
/// `(c − p_i)·n_i = +r`, where the convex rest has `−r`. The concave
/// arm (`signed = +radius`) is written and, until this unit, called by
/// nobody; this row is the verification the plan demands before any
/// consumer is built over it.
///
/// Orthonormal case, exact: walls through the origin with outward
/// normals `+x`, `+y`, `+z` (the mirror of the box corner whose convex
/// rest is pinned in the M5 blend rows) put the centre at `(r, r, r)` —
/// in the void — with the same independence `|det| = 1` the convex arm
/// reports, since the determinant never reads the side.
#[test]
fn the_concave_arm_rests_the_ball_in_the_void_at_depth_r() {
    let r = 0.15;
    let normals = [v(1.0, 0.0, 0.0), v(0.0, 1.0, 0.0), v(0.0, 0.0, 1.0)];
    let ball = corner_ball([p(0.0, 0.0, 0.0); 3], normals, r, false);
    assert!(
        (ball.center - p(r, r, r)).norm() < 1e-15,
        "the concave rest is at (r, r, r), got {:?}",
        ball.center
    );
    assert!((ball.independence - 1.0).abs() < 1e-15);
    for n in normals {
        let depth = (ball.center - p(0.0, 0.0, 0.0)).dot(n);
        assert!(
            (depth - r).abs() < 1e-15,
            "distance r on the VOID side of every wall, got {depth}"
        );
    }
}

/// **MEASUREMENT 1b — the concave rest holds at an oblique trihedron.**
///
/// The Cramer solve is one expression for both sides, so the property
/// worth measuring is the defining tangency itself, off the orthonormal
/// special case: for independent but non-orthogonal walls the centre
/// still satisfies `(c − p_i)·n_i = +r` for all three, and the reported
/// independence is the same `|det|` the convex solve reports.
#[test]
fn the_concave_rest_holds_at_an_oblique_trihedron() {
    let r = 0.2;
    let normals = [v(1.0, 0.0, 0.0), v(0.0, 1.0, 0.0), v(0.6, 0.0, 0.8)];
    let verts = [p(0.0, 0.0, 0.0); 3];
    let concave = corner_ball(verts, normals, r, false);
    for (i, n) in normals.iter().enumerate() {
        let depth = (concave.center - verts[i]).dot(*n);
        assert!(
            (depth - r).abs() < 1e-15,
            "wall {i}: the ball rests at +r in the void, got {depth}"
        );
    }
    assert!((concave.independence - 0.8).abs() < 1e-15);
    let convex = corner_ball(verts, normals, r, true);
    assert!(
        (convex.independence - concave.independence).abs() < 1e-15,
        "independence is side-blind"
    );
}

/// **MEASUREMENT 2 — the convex feet formula does not survive the
/// concave centre.** The surgery's corner plan derives each foot as
/// `centre + n·r`, which lands ON the support exactly when the centre
/// is at depth `r` INSIDE it (`(c − p)·n = −r`). Under the concave
/// rest the same expression lands `2r` off the wall — on the far side
/// of the void — while `centre − n·r` is the tangency point. This is
/// the measured statement of issue 644's one-change shape: the ball's
/// side and the feet's sign are the same decision, and deriving one
/// without the other builds a corner less coherent than either side
/// alone.
#[test]
fn the_convex_feet_formula_is_two_r_off_the_wall_under_the_concave_rest() {
    let r = 0.15;
    let normals = [v(1.0, 0.0, 0.0), v(0.0, 1.0, 0.0), v(0.0, 0.0, 1.0)];
    let ball = corner_ball([p(0.0, 0.0, 0.0); 3], normals, r, false);
    for n in normals {
        let convex_formula = ball.center + n * r;
        let off = (convex_formula - p(0.0, 0.0, 0.0)).dot(n);
        assert!(
            (off - 2.0 * r).abs() < 1e-15,
            "the convex-signed foot floats 2r into the void, got {off}"
        );
        let concave_foot = ball.center - n * r;
        let on = (concave_foot - p(0.0, 0.0, 0.0)).dot(n);
        assert!(
            on.abs() < 1e-15,
            "the mirrored sign is the tangency point, got {on}"
        );
    }
}

/// **MEASUREMENT 3 — the stored chart of the concave ball is
/// convex-derived (the half-derivation this unit closes).** The
/// surface `corner_ball` carries aims its pole along `+Σn`, which is
/// the apex direction of the CONVEX octant (whose feet lie along
/// `+n_i`). The concave patch's feet lie along `−n_i`, so its apex is
/// ANTIPODAL to that pole. Pinned as the opening state; the unit's
/// change makes the stored chart follow the side, and this row's
/// assertion flips with it.
#[test]
fn the_concave_ball_stored_chart_aims_at_the_convex_apex() {
    let r = 0.15;
    let normals = [v(1.0, 0.0, 0.0), v(0.0, 1.0, 0.0), v(0.0, 0.0, 1.0)];
    let ball = corner_ball([p(0.0, 0.0, 0.0); 3], normals, r, false);
    let Surface::Sphere { axis, .. } = ball.surface else {
        panic!("the corner ball's surface is a sphere");
    };
    // The concave apex foot direction: the foot opposite the walls'
    // mean, `−Σn/|Σn|`.
    let apex = -(normals[0] + normals[1] + normals[2]).normalize();
    let aim = axis.dot(apex);
    assert!(
        (aim + 1.0).abs() < 1e-15,
        "opening state: the stored pole is antipodal to the concave apex (dot {aim})"
    );
}
