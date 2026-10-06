//! **A torus touching a partner at one point, decided per face.**
//!
//! The donut (`R = 2`, `r = 0.5`, about y) against a plane, a ball or a
//! wall parallel to its axis, each tangent to the tube at a point of its
//! outer half — an elliptic point, where the partner's level function
//! over the torus has its strict extreme. The touch is at `(0, 0, 2.5)`,
//! on the outer equator, clear of the donut's seam meridian (the
//! half-plane `z = 0, x > 0`). Where the partner's face does not hold
//! the touch, the bodies are apart and every op builds, in both operand
//! orders, against the volumes the radii and lengths give. Where it
//! does, the boundaries touch and the pair keeps its refusal. A
//! tangency on the tube's inner half, where the torus is a saddle,
//! keeps its refusal wherever it stands. And each pose moved off the
//! touch by `±100ε` and `±1e-4` answers by the closed-form distance
//! between the carriers: apart, it builds; across, it builds when the
//! faces stay apart and refuses typed when they meet.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use topo::{Body, BooleanError, BooleanOp};

use super::extent_scan_off_face_tangency::{
    Pose, assert_every_op, assert_every_op_refuses, ball, ball_volume, boolean, built, cap_volume,
    dir, rod_z,
};

/// The donut of the torus doors: the circle of radius 0.5 about
/// `(2, 0)` in the xy-plane, revolved about y.
fn donut() -> Body<f64> {
    sweep::test_support::revolved_about_y(
        vec![(Point2::new(2.0, -0.5), 1.0), (Point2::new(2.0, 0.5), 1.0)],
        Revolution::Full,
        Tol::witness(),
    )
}

/// `2π²Rr²`.
const DONUT_VOLUME: f64 = TAU * PI * 2.0 * 0.25;

fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    sweep::test_support::brick(x, y, z, Tol::witness())
}

fn brick_volume(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> f64 {
    (x.1 - x.0) * (y.1 - y.0) * (z.1 - z.0)
}

/// A rod of radius `r` along y through `(x, z)`, from `y0` to `y1`: a
/// z rod turned a quarter about x, which carries `(x, y, z)` to
/// `(x, z, −y)`.
fn rod_y(r: f64, (x, z): (f64, f64), (y0, y1): (f64, f64)) -> Body<f64> {
    let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_x(), -PI / 2.0);
    topo::transform_rigid(&rod_z(r, (x, -z), (y0, y1)), &turn, Tol::witness()).unwrap()
}

/// A ball of radius `r` about `c` with everything beyond the plane
/// `z = cut` taken off (`above`: the cap above it), and its volume.
fn trimmed_ball(r: f64, c: Vec3<f64>, cut: f64, above: bool) -> (Body<f64>, f64) {
    let z = if above {
        (cut, c.z + 2.0 * r)
    } else {
        (c.z - 2.0 * r, cut)
    };
    let body = built(
        BooleanOp::Subtract,
        &ball(r, c),
        &brick((c.x - 2.0, c.x + 2.0), (c.y - 2.0, c.y + 2.0), z),
    )
    .expect("the trimmed ball");
    let h = if above {
        c.z + r - cut
    } else {
        cut - (c.z - r)
    };
    (body, ball_volume(r) - cap_volume(r, h))
}

/// The plane `z = 2.5 + δ`, the bottom of a brick, against the outer
/// equator's top at `(0, 0, 2.5)`: off its face (`x ∈ [1, 2]`) or over
/// it (`x ∈ [−1, 1]`). `δ` is the carriers' distance.
fn plane_brick(over: bool, delta: f64) -> (Body<f64>, f64) {
    let (x, y, z) = (
        if over { (-1.0, 1.0) } else { (1.0, 2.0) },
        (-1.0, 1.0),
        (2.5 + delta, 3.5),
    );
    (brick(x, y, z), brick_volume(x, y, z))
}

/// A ball of radius 0.5 above the outer equator, `δ` clear of it: its
/// carrier touches the tube at `(0, 0, 2.5)` at `δ = 0`. Off the face,
/// its cap below `z = 2.6 + δ` is taken off.
fn top_ball(over: bool, delta: f64) -> (Body<f64>, f64) {
    let c = Vec3::new(0.0, 0.0, 3.0 + delta);
    if over {
        (ball(0.5, c), ball_volume(0.5))
    } else {
        trimmed_ball(0.5, c, 2.6 + delta, false)
    }
}

/// A rod of radius 0.5 along y through `(0, 3 + δ)`: its wall touches
/// the outer equator at `(0, 0, 2.5)` at `δ = 0`, past the rod's end
/// (`y ≥ 0.2`) or along it (`y ∈ [−2, 2]`).
fn top_rod(over: bool, delta: f64) -> (Body<f64>, f64) {
    let y = if over { (-2.0, 2.0) } else { (0.2, 2.0) };
    (rod_y(0.5, (0.0, 3.0 + delta), y), PI * 0.25 * (y.1 - y.0))
}

/// The plane `z = 1.5 − δ`, the top of a brick standing in the hole,
/// tangent at `δ = 0` to the inner equator at `(0, 0, 1.5)`, a saddle
/// point. The plane meets the torus in a figure eight through the
/// touch; the face (`x ∈ [0.1, 0.4]`, `y ∈ [0.3, 0.45]`) holds neither
/// the touch nor the eight, and the brick is clear of the donut.
fn hole_brick(delta: f64) -> (Body<f64>, f64) {
    let (x, y, z) = ((0.1, 0.4), (0.3, 0.45), (1.0, 1.5 - delta));
    (brick(x, y, z), brick_volume(x, y, z))
}

/// A ball of radius 1 about `(0, 0, 0.5 − δ)`, in the hole, tangent at
/// `δ = 0` to the inner equator at `(0, 0, 1.5)`, with its cap above
/// `z = 1.3 − δ` taken off.
fn hole_ball(delta: f64) -> (Body<f64>, f64) {
    trimmed_ball(1.0, Vec3::new(0.0, 0.0, 0.5 - delta), 1.3 - delta, true)
}

/// The no-crossings path's tangency refusal.
fn tangent(e: &BooleanError) -> bool {
    matches!(e, BooleanError::FallbackExtentUnsupported { what, .. } if what.contains("tangent"))
}

/// **A plane touching the outer equator off its face builds** (the
/// item's donut × brick): the bottom of `[1, 2] × [−1, 1] × [2.5, 3.5]`
/// touches the tube at `(0, 0, 2.5)`, outside the face. The torus × plane
/// arm reads a touch there (mutant: R-tan on any `Zero` margin).
#[test]
fn a_brick_whose_plane_touches_the_outer_equator_off_its_face_builds() {
    let (b, vb) = plane_brick(false, 0.0);
    assert_every_op(
        "donut, brick",
        &donut(),
        &b,
        (DONUT_VOLUME, vb),
        Pose::Apart,
    );
}

/// **A ball touching the outer equator off its face builds**: the ball's
/// cap about the touch is taken off, so its face does not hold it.
#[test]
fn a_ball_touching_the_outer_equator_off_its_face_builds() {
    let (b, vb) = top_ball(false, 0.0);
    assert_every_op("donut, ball", &donut(), &b, (DONUT_VOLUME, vb), Pose::Apart);
}

/// **A wall parallel to the axis touching the outer equator past the
/// rod's end builds.**
#[test]
fn a_rod_whose_wall_touches_the_outer_equator_past_its_end_builds() {
    let (b, vb) = top_rod(false, 0.0);
    assert_every_op("donut, rod", &donut(), &b, (DONUT_VOLUME, vb), Pose::Apart);
}

/// **The same touches on both faces keep their refusals**: the brick
/// moved over the touch, the whole ball, the rod along it. The
/// boundaries genuinely touch at `(0, 0, 2.5)` (mutant: a touch cleared
/// without placing it).
#[test]
fn the_same_touches_on_both_faces_refuse() {
    let d = donut();
    for (label, (b, _)) in [
        ("brick over the touch", plane_brick(true, 0.0)),
        ("whole ball", top_ball(true, 0.0)),
        ("rod along the touch", top_rod(true, 0.0)),
    ] {
        assert_every_op_refuses(label, &d, &b, tangent);
    }
}

/// **A tangency at a saddle point keeps R-tan**, though neither face
/// holds the touch: the brick's top and the trimmed ball touch the inner
/// equator at `(0, 0, 1.5)` from the hole (mutant: a touch on the inner
/// half of the tube).
#[test]
fn a_touch_on_the_inner_equator_refuses_off_the_faces() {
    let d = donut();
    for (label, (b, _)) in [
        ("brick in the hole", hole_brick(0.0)),
        ("ball in the hole", hole_ball(0.0)),
    ] {
        assert_every_op_refuses(label, &d, &b, tangent);
    }
}

/// **Off the touch, the closed-form distance decides.** Each pose above
/// moved `δ ∈ {±100ε, ±1e-4}` along the touch's normal, `δ > 0` apart:
/// the carriers' distance is `δ`. Apart, every op builds against the
/// operands' volumes. Across, a pose whose faces stay apart (the off-face
/// brick, ball and rod; the hole's brick and ball, whose faces hold none
/// of the section) builds the same way, and one whose faces meet (the
/// brick over the touch, the whole ball, the rod along it) refuses typed.
#[test]
fn near_misses_answer_by_the_closed_form_distance() {
    let eps = Tol::witness().eps();
    let d = donut();
    type Fixture = fn(f64) -> (Body<f64>, f64);
    let poses: [(&str, Fixture, bool); 8] = [
        ("off-face brick", |t| plane_brick(false, t), false),
        ("brick over the touch", |t| plane_brick(true, t), true),
        ("trimmed ball", |t| top_ball(false, t), false),
        ("whole ball", |t| top_ball(true, t), true),
        ("short rod", |t| top_rod(false, t), false),
        ("rod along the touch", |t| top_rod(true, t), true),
        ("brick in the hole", hole_brick, false),
        ("ball in the hole", hole_ball, false),
    ];
    for (label, pose, faces_meet) in poses {
        for delta in [100.0 * eps, -100.0 * eps, 1e-4, -1e-4] {
            let (b, vb) = pose(delta);
            let label = format!("{label}, δ = {delta:e}");
            if delta < 0.0 && faces_meet {
                for op in [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract] {
                    for (order, x, y) in [("A·B", &d, &b), ("B·A", &b, &d)] {
                        assert!(
                            boolean(op, x, y).is_err(),
                            "{label} {op:?} {order}: built across a crossing of the faces"
                        );
                    }
                }
            } else {
                assert_every_op(&label, &d, &b, (DONUT_VOLUME, vb), Pose::Apart);
            }
        }
    }
}

/// **The off-face touches in a tilted frame**: the brick and the ball,
/// both operands turned 0.7 rad about `(1, 2, 3)`, build as they do
/// axis-aligned.
#[test]
fn the_off_face_touches_build_in_a_tilted_frame() {
    let turn = Affine3::rotation_about_axis(Point3::origin(), dir(1.0, 2.0, 3.0), 0.7);
    let tilt = |b: &Body<f64>| topo::transform_rigid(b, &turn, Tol::witness()).unwrap();
    let d = tilt(&donut());
    for (label, (b, vb)) in [
        ("brick", plane_brick(false, 0.0)),
        ("ball", top_ball(false, 0.0)),
    ] {
        assert_every_op(
            &format!("tilted donut, {label}"),
            &d,
            &tilt(&b),
            (DONUT_VOLUME, vb),
            Pose::Apart,
        );
    }
}
