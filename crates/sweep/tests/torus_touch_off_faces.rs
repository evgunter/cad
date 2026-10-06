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

/// The donut of the torus doors at scale `l`: the circle of radius
/// `0.5·l` about `(2l, 0)` in the xy-plane, revolved about y.
fn donut(l: f64) -> Body<f64> {
    sweep::test_support::revolved_about_y(
        vec![
            (Point2::new(2.0 * l, -0.5 * l), 1.0),
            (Point2::new(2.0 * l, 0.5 * l), 1.0),
        ],
        Revolution::Full,
        Tol::witness(),
    )
}

/// `2π²Rr²` at unit scale.
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
        &brick(
            (c.x - 4.0 * r, c.x + 4.0 * r),
            (c.y - 4.0 * r, c.y + 4.0 * r),
            z,
        ),
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
/// it (`x ∈ [−1, 1]`). `δ` is the carriers' distance. At scale `l`
/// every length but `δ` is `l` times its own.
fn plane_brick(over: bool, delta: f64, l: f64) -> (Body<f64>, f64) {
    let x = if over { (-1.0, 1.0) } else { (1.0, 2.0) };
    let (x, y, z) = ((x.0 * l, x.1 * l), (-l, l), (2.5 * l + delta, 3.5 * l));
    (brick(x, y, z), brick_volume(x, y, z))
}

/// A ball of radius 0.5 above the outer equator, `δ` clear of it: its
/// carrier touches the tube at `(0, 0, 2.5)` at `δ = 0`. Off the face,
/// its cap below `z = 2.6 + δ` is taken off. At scale `l` every length
/// but `δ` is `l` times its own.
fn top_ball(over: bool, delta: f64, l: f64) -> (Body<f64>, f64) {
    let c = Vec3::new(0.0, 0.0, 3.0 * l + delta);
    if over {
        (ball(0.5 * l, c), ball_volume(0.5 * l))
    } else {
        trimmed_ball(0.5 * l, c, 2.6 * l + delta, false)
    }
}

/// A rod of radius 0.5 along y through `(0, 3 + δ)`: its wall touches
/// the outer equator at `(0, 0, 2.5)` at `δ = 0`, past the rod's end
/// (`y ≥ 0.2`) or along it (`y ∈ [−2, 2]`). At scale `l` every length
/// but `δ` is `l` times its own.
fn top_rod(over: bool, delta: f64, l: f64) -> (Body<f64>, f64) {
    let y = if over { (-2.0, 2.0) } else { (0.2, 2.0) };
    let y = (y.0 * l, y.1 * l);
    (
        rod_y(0.5 * l, (0.0, 3.0 * l + delta), y),
        PI * 0.25 * l * l * (y.1 - y.0),
    )
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
    let (b, vb) = plane_brick(false, 0.0, 1.0);
    assert_every_op(
        "donut, brick",
        &donut(1.0),
        &b,
        (DONUT_VOLUME, vb),
        Pose::Apart,
    );
}

/// **A ball touching the outer equator off its face builds**: the ball's
/// cap about the touch is taken off, so its face does not hold it.
#[test]
fn a_ball_touching_the_outer_equator_off_its_face_builds() {
    let (b, vb) = top_ball(false, 0.0, 1.0);
    assert_every_op(
        "donut, ball",
        &donut(1.0),
        &b,
        (DONUT_VOLUME, vb),
        Pose::Apart,
    );
}

/// **A wall parallel to the axis touching the outer equator past the
/// rod's end builds.**
#[test]
fn a_rod_whose_wall_touches_the_outer_equator_past_its_end_builds() {
    let (b, vb) = top_rod(false, 0.0, 1.0);
    assert_every_op(
        "donut, rod",
        &donut(1.0),
        &b,
        (DONUT_VOLUME, vb),
        Pose::Apart,
    );
}

/// **The same touches on both faces keep their refusals**: the brick
/// moved over the touch, the whole ball, the rod along it. The
/// boundaries genuinely touch at `(0, 0, 2.5)` (mutant: a touch cleared
/// without placing it).
#[test]
fn the_same_touches_on_both_faces_refuse() {
    let d = donut(1.0);
    for (label, (b, _)) in [
        ("brick over the touch", plane_brick(true, 0.0, 1.0)),
        ("whole ball", top_ball(true, 0.0, 1.0)),
        ("rod along the touch", top_rod(true, 0.0, 1.0)),
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
    let d = donut(1.0);
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
    let d = donut(1.0);
    type Fixture = fn(f64) -> (Body<f64>, f64);
    let poses: [(&str, Fixture, bool); 8] = [
        ("off-face brick", |t| plane_brick(false, t, 1.0), false),
        ("brick over the touch", |t| plane_brick(true, t, 1.0), true),
        ("trimmed ball", |t| top_ball(false, t, 1.0), false),
        ("whole ball", |t| top_ball(true, t, 1.0), true),
        ("short rod", |t| top_rod(false, t, 1.0), false),
        ("rod along the touch", |t| top_rod(true, t, 1.0), true),
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

/// **The off-face touches build at ×1e-3 and ×1e3**: the brick, the
/// ball and the rod, every length scaled with the donut's, against the
/// closed form scaled by `l³`. Where ε is within 64 ulps of the donut's
/// largest coordinate (`2.5·l`), its rounding-scale residuals reach the
/// band and the donut is not a finished body: there it refuses typed,
/// an escalation, and the pose is read as that refusal.
#[test]
fn the_off_face_touches_build_at_every_scale() {
    let eps = Tol::witness().eps();
    for l in [1e-3, 1e3] {
        let d = donut(l);
        if eps < 64.0 * f64::EPSILON * 2.5 * l {
            let refused = topo::validate_geometric(&d, Tol::witness())
                .expect_err("a donut within 64 ulps of ε refuses");
            assert!(
                refused.iter().all(|e| matches!(
                    e,
                    topo::ValidationError::VolumeUncomputable {
                        source: topo::MassPropsError::Face {
                            source: geom_brep::PropsError::Escalated { .. },
                            ..
                        },
                        ..
                    }
                )),
                "donut ×{l:e} at ε {eps:e}: refused by escalation only, got {refused:?}"
            );
            test_utils::vacuity::stood_down(
                &format!("donut ×{l:e} at ε {eps:e}"),
                "the donut refuses as an operand, so its touches are not built",
            );
            continue;
        }
        for (label, (b, vb)) in [
            ("brick", plane_brick(false, 0.0, l)),
            ("ball", top_ball(false, 0.0, l)),
            ("rod", top_rod(false, 0.0, l)),
        ] {
            assert_every_op(
                &format!("donut ×{l:e}, {label}"),
                &d,
                &b,
                (DONUT_VOLUME * l.powi(3), vb),
                Pose::Apart,
            );
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
    let d = tilt(&donut(1.0));
    for (label, (b, vb)) in [
        ("brick", plane_brick(false, 0.0, 1.0)),
        ("ball", top_ball(false, 0.0, 1.0)),
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
