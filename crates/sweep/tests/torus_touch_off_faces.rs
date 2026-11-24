//! **A torus touching a partner at an extreme, decided per face.**
//!
//! The donut (`R = 2`, `r = 0.5`, about y) against a plane, a ball or a
//! wall, each tangent to the tube at an extreme of the partner's level
//! function over the torus (its height, the distance from the ball's
//! centre or the wall's axis): at a point of the tube's outer half or its
//! inner, or along a whole parallel. Where the partner's face does not
//! hold the touch, every op builds, in both operand orders, against the
//! volumes the radii and lengths give. Where it does, the boundaries
//! touch and the pair keeps its refusal. A tangency at a saddle of the
//! level function keeps its refusal wherever it stands. Each pose moved
//! off the touch by `±100ε` and `±1e-4` answers by the closed-form
//! distance between the carriers: apart, it builds; across, it builds
//! when the faces stay apart and refuses typed when they meet. And an
//! edge standing within the band of the torus beside an off-face touch,
//! on the side the touch's argument needs the sweep to see it (the
//! section certificate's S′), refuses.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use topo::{Body, BooleanError, BooleanOp};

use super::extent_scan_off_face_tangency::{
    Pose, assert_every_op, assert_every_op_refuses, ball, boolean, built, dir, rod_z,
};
use crate::common::oracles::{ball_volume, cap_volume};

/// The donut of the torus doors at scale `l`: the circle of radius
/// `0.5·l` about `(2l, 0)` in the xy-plane, revolved about y.
fn donut(l: f64) -> Body<f64> {
    sweep::test_support::revolved_about_y(donut_disc(l), Revolution::Full, Tol::witness())
}

/// The donut's meridian disc at scale `l`.
fn donut_disc(l: f64) -> Vec<(Point2<f64>, f64)> {
    vec![
        (Point2::new(2.0 * l, -0.5 * l), 1.0),
        (Point2::new(2.0 * l, 0.5 * l), 1.0),
    ]
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
    topo::transform_rigid(
        &rod_z(r, (x, -z), (y0, y1)),
        &geom_core::test_support::rigid(&turn, Tol::witness()),
    )
    .unwrap()
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

/// **A block in the hole whose top touches the inner equator at a
/// saddle keeps R-tan**, though its face does not hold the touch: the
/// plane meets the torus in a figure eight there, a saddle of its
/// height, and a touch at a saddle leans on the line × torus door outside
/// the solid, which is silent within the band
/// (`work/germ/the-line-torus-door-misses-silently-within-the-band.md`).
#[test]
fn a_block_at_the_inner_equators_saddle_refuses_off_its_face() {
    assert_every_op_refuses(
        "brick in the hole",
        &donut(1.0),
        &hole_brick(0.0).0,
        tangent,
    );
}

/// **A trimmed ball in the hole touching the inner equator builds**: the
/// inner equator's point `(0, 0, 1.5)` is the strict minimum of the
/// distance from the ball's centre over the torus, an extreme on the
/// tube's inner half, and the ball's cap about it is taken off (mutant:
/// the elliptic margin restored).
#[test]
fn a_trimmed_ball_in_the_hole_touching_the_inner_equator_builds() {
    let (b, vb) = hole_ball(0.0);
    assert_every_op(
        "ball in the hole",
        &donut(1.0),
        &b,
        (DONUT_VOLUME, vb),
        Pose::Apart,
    );
}

/// A sleeve of inner radius `inner`, outer `2.8`, `y ∈ [lo, hi]`, about
/// the donut's axis, over the turn `sweep` (radians) from `+x`.
fn sleeve(inner: f64, (lo, hi): (f64, f64), sweep: Revolution<f64>) -> Body<f64> {
    sweep::test_support::revolved_about_y(
        vec![
            (Point2::new(inner, lo), 0.0),
            (Point2::new(2.8, lo), 0.0),
            (Point2::new(2.8, hi), 0.0),
            (Point2::new(inner, hi), 0.0),
        ],
        sweep,
        Tol::witness(),
    )
}

/// `b` moved by `turn` (an affine map that is rigid).
fn moved(b: &Body<f64>, turn: &Affine3<f64>) -> Body<f64> {
    topo::transform_rigid(b, &geom_core::test_support::rigid(turn, Tol::witness())).unwrap()
}

/// **A touch along a whole parallel keeps R-tan, its faces off it or
/// not**: a hub flush with the ring's top (a rod of radius 1 along the
/// axis, its top `y = 0.5` on the top parallel's plane), and a sleeve
/// above the widest circle (its inner wall coaxial and tangent along the
/// outer equator, `y ∈ [0.2, 1]`). The near band about a parallel is one
/// annulus, but in a pose the band admits, the axis moved or tilted, the
/// partner dips into the tube on one side only, and the edges that part
/// the dip from a point read off the exact pose stand outside the solid,
/// where the line × torus door is silent
/// (`work/germ/the-line-torus-door-misses-silently-within-the-band.md`).
#[test]
fn a_touch_along_a_parallel_refuses_off_its_face() {
    let d = donut(1.0);
    assert_every_op_refuses("hub", &d, &rod_y(1.0, (0.0, 0.0), (-1.0, 0.5)), tangent);
    assert_every_op_refuses(
        "sleeve",
        &d,
        &sleeve(2.5, (0.2, 1.0), Revolution::Full),
        tangent,
    );
}

/// **A parallel touch read off its exact pose would clear a real
/// crossing; it refuses** (the review's probes). A sleeve sector over
/// `[60°, 300°]` about the axis, its inner wall the outer equator's
/// coaxial tangent, moved `e ∈ {0.2ε, 0.5ε}` along `+x` (into the tube
/// on the sector's far side, off it beside the window about `+x`); and a
/// D-pin in the hole, a rod of radius `1.5 − h` about `(−δ, ·, 0)` cut at
/// `x ≥ −δ + 0.5`, against the inner equator at `(−1.5, 0, 0)` for
/// `(h, δ)` of `(0.2ε, 0.2ε)`, `(0.4ε, 0.4ε)` and `(0.2ε, 0.6ε)` (a 0.4ε
/// crossing along some 140° of the arc). Each was built across its
/// crossing while a parallel touch cleared on the parallel's point at the
/// torus frame's reference direction.
#[test]
fn a_parallel_touch_off_its_exact_pose_refuses() {
    let eps = Tol::witness().eps();
    let d = donut(1.0);
    let sector = sleeve(2.5, (-1.0, 1.0), Revolution::Partial(2.0 * TAU / 3.0));
    for e in [0.2 * eps, 0.5 * eps] {
        let place = Affine3::translation(Vec3::new(e, 0.0, 0.0))
            * Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_y(), PI / 3.0);
        assert_every_op_refuses("sleeve sector", &d, &moved(&sector, &place), |_| true);
    }
    for (h, delta) in [(0.2, 0.2), (0.4, 0.4), (0.2, 0.6)] {
        let (h, delta) = (h * eps, delta * eps);
        let pin = built(
            BooleanOp::Subtract,
            &rod_y(1.5 - h, (-delta, 0.0), (-1.0, 1.0)),
            &brick((-delta + 0.5, 3.0), (-2.0, 2.0), (-3.0, 3.0)),
        )
        .expect("the D-pin");
        assert_every_op_refuses("D-pin", &d, &pin, |_| true);
    }
}

/// **A point touch read off its pose clears no crossing; it refuses**:
/// each point arm moved `0.4ε` across, so the partner dips into the tube
/// about its actual deepest point, its face cut through that dip on the
/// far side of the deepest point, which it does not hold: a brick whose
/// edge stands inside the cap, a ball trimmed through it, and a rod tilted
/// by a turn the band admits whose end stands in it. Each pair's faces
/// cross, and every op refuses: a build would move some 1e-19 of volume
/// against the operands apart, no volume check sees it.
#[test]
fn a_point_touch_off_its_exact_pose_refuses() {
    let eps = Tol::witness().eps();
    let d = donut(1.0);
    let dip = 0.4 * eps;
    let cap = (2.0 * 0.4 * dip).sqrt();
    let block = brick((0.3 * cap, 2.0), (-1.0, 1.0), (2.5 - dip, 3.5));
    assert_every_op_refuses("brick", &d, &block, |_| true);
    let ball_cut = built(
        BooleanOp::Subtract,
        &ball(0.5, Vec3::new(0.0, 0.0, 3.0 - dip)),
        &brick((-2.0, 0.3 * cap), (-2.0, 2.0), (2.0, 4.0)),
    )
    .expect("the cut ball");
    assert_every_op_refuses("ball", &d, &ball_cut, |_| true);
    let rod = moved(
        &rod_y(0.5, (0.0, 3.0 - dip), (0.3 * cap, 2.0)),
        &Affine3::rotation_about_axis(
            Point3::new(0.0, 0.0, 3.0),
            Vec3::unit_z(),
            IN_BAND_TILT * eps,
        ),
    );
    assert_every_op_refuses("tilted rod", &d, &rod, |_| true);
}

/// A turn, in units of ε, the parallel-axis reading takes as parallel:
/// its tilt over the torus's lever (twice its ball's diameter, 10 here)
/// is `0.4ε`.
const IN_BAND_TILT: f64 = 0.04;

/// **A rod tilted by a turn the band admits touches off its face and
/// builds**: the rod of [`top_rod`], past its end, turned `0.04ε` about
/// the z line through its axis's point over the touch, so the wall's axis
/// is read parallel to the donut's, and its touch is read off the wall's
/// own axis. Every op builds at the operands' volumes.
#[test]
fn a_rod_tilted_in_the_band_touching_off_its_face_builds() {
    let eps = Tol::witness().eps();
    let (rod, vr) = top_rod(false, 0.0, 1.0);
    let rod = moved(
        &rod,
        &Affine3::rotation_about_axis(
            Point3::new(0.0, 0.0, 3.0),
            Vec3::unit_z(),
            IN_BAND_TILT * eps,
        ),
    );
    assert_every_op(
        "tilted rod",
        &donut(1.0),
        &rod,
        (DONUT_VOLUME, vr),
        Pose::Apart,
    );
}

/// **A ball inside the tube against its inner half builds**: radius `0.2`
/// about `(0, 0, 1.7)`, tangent from inside to the inner equator at
/// `(0, 0, 1.5)`, the minimum of the distance from its centre, its cap
/// below `z = 1.55` taken off; the donut holds it.
#[test]
fn a_ball_inside_the_tube_against_its_inner_half_builds() {
    let (b, vb) = trimmed_ball(0.2, Vec3::new(0.0, 0.0, 1.7), 1.55, false);
    assert_every_op(
        "ball in the tube",
        &donut(1.0),
        &b,
        (DONUT_VOLUME, vb),
        Pose::Inside,
    );
}

/// **An edge within the band of the torus beside an off-face touch
/// refuses** (S′): a brick whose bottom `z = 2.5 − ε/2` touches the outer
/// equator's top off its face (`x ≥ a`), its edge `x = a` standing `3ε/8`
/// inside the solid under the cap the plane cuts; and a ball touching the
/// top off its face, trimmed to `x ≥ a`, its rim standing `ε/2` above the
/// torus. Neither pair has a crossing the sweep would miss: the line ×
/// torus door inside the solid, and the circle × torus door either side,
/// refuse within the band (mutant: either door silent there).
#[test]
fn an_edge_within_the_band_beside_an_off_face_touch_refuses() {
    let eps = Tol::witness().eps();
    let d = donut(1.0);
    let a = 0.5 * (2.5 * eps).sqrt();
    let block = brick((a, 2.0), (-1.0, 1.0), (2.5 - 0.5 * eps, 3.5));
    assert_every_op_refuses("brick's edge inside the cap", &d, &block, |_| true);
    let a = (eps / 2.4).sqrt();
    let rimmed = built(
        BooleanOp::Subtract,
        &ball(0.5, Vec3::new(0.0, 0.0, 3.0)),
        &brick((-2.0, a), (-2.0, 2.0), (2.0, 4.0)),
    )
    .expect("the trimmed ball");
    assert_every_op_refuses("ball's rim over the top", &d, &rimmed, |_| true);
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

/// **The off-face touches build at ×1e-3 and ×250**: the brick, the
/// ball and the rod, every length scaled with the donut's, against the
/// closed form scaled by `l³`. ×250 is the largest round scale whose
/// partners, reaching `3.5·l`, lie in the 1 km session box
/// (`[tolerance.units]`); a donut past it is refused by the box
/// ([`a_donut_past_the_session_box_is_refused_by_it`]). Where the scale
/// is [`ROD_SCAN_STOPS`] ε or more (×250 at ε = 1e-12), the rod refuses
/// every op `FallbackExtentUnsupported`
/// (`work/orbit/a-rod-touching-a-big-donut-off-its-face-stops-at-the-extent-scan-at-1e-12.md`),
/// which the row pins. At ε 1e-12 the rod at a [`ROD_FRAME_COST`] scale
/// refuses ahead of that, at its own finish, so the extent scan's stop
/// goes unread at the only ε that reaches it today.
#[test]
fn the_off_face_touches_build_at_every_scale() {
    let eps = Tol::witness().eps();
    for l in [1e-3, 250.0] {
        let d = donut(l);
        let rod_stops = l >= ROD_SCAN_STOPS * eps;
        for (label, (b, vb)) in [
            ("brick", plane_brick(false, 0.0, l)),
            ("ball", top_ball(false, 0.0, l)),
            ("rod", top_rod(false, 0.0, l)),
        ] {
            let label = format!("donut ×{l:e}, {label}");
            if crate::common::frame_cost::at_the_least_band()
                && ROD_FRAME_COST.contains(&l)
                && label.ends_with("rod")
            {
                crate::common::frame_cost::assert_rod_refuses_its_finish(
                    &label,
                    &topo::finish(topo::Built::of(b.clone()), Tol::witness()).map(|_| ()),
                    (1.65e-12, 1.8e-12),
                );
                test_utils::vacuity::stood_down(
                    &format!("{label} at ε {eps:e}"),
                    "the rod refuses its own finish, pinned (the frame cost)",
                );
                continue;
            }
            if rod_stops && label.ends_with("rod") {
                assert_every_op_refuses(&label, &d, &b, |e| {
                    format!("{e:?}").contains("FallbackExtentUnsupported")
                });
                test_utils::vacuity::stood_down(
                    &format!("{label} at ε {eps:e}"),
                    "the rod's touch refuses at the extent scan, pinned",
                );
                continue;
            }
            assert_every_op(&label, &d, &b, (DONUT_VOLUME * l.powi(3), vb), Pose::Apart);
        }
    }
}

/// **The scale, in units of ε, from which the rod's touch stops at the
/// extent scan**: measured, it refuses every op at ×100 to ×250 at
/// ε = 1e-12 (`1e14` to `2.5e14` ε) and builds at ×10 there (`1e13` ε)
/// and at ×250 at 1e-9 (`2.5e11` ε). A fix that moves the band reads as
/// this row's pin going red, at the scale it moved.
const ROD_SCAN_STOPS: f64 = 1.0e14;

/// The scales at which [`the_off_face_touches_build_at_every_scale`]'s
/// rod refuses its own finish at ε 1e-12
/// (`crate::common::frame_cost::at_the_least_band`), ahead of the
/// extent scan: its wall's volume escalates
/// `props_cylinder_circle_off_axis`, margin in `(1.65e-12, 1.8e-12]`
/// (measured 1.738e-12). Cause: polar, the rod's stored frame read as
/// its polar enclosure, levered by the scale; mutant A (PR 7131's body)
/// finishes it, and turns this pin red; mutant F1X does not.
const ROD_FRAME_COST: [f64; 1] = [250.0];

/// **A donut past the session box is refused by it**: at ×401 its outer
/// equator reaches 1002.5 m.
#[test]
fn a_donut_past_the_session_box_is_refused_by_it() {
    crate::common::refused_by_the_session_box(
        "the donut ×401",
        2.5 * 401.0,
        sweep::test_support::try_revolved_about_y_at(
            donut_disc(401.0),
            Revolution::Full,
            Tol::witness(),
        ),
    );
}

/// **The off-face touches in a tilted frame**: the brick and the ball,
/// both operands turned 0.7 rad about `(1, 2, 3)`, build as they do
/// axis-aligned.
#[test]
fn the_off_face_touches_build_in_a_tilted_frame() {
    let turn = Affine3::rotation_about_axis(Point3::origin(), dir(1.0, 2.0, 3.0), 0.7);
    let tilt = |b: &Body<f64>| {
        topo::transform_rigid(b, &geom_core::test_support::rigid(&turn, Tol::witness())).unwrap()
    };
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
