//! **Carrier tangencies on the no-crossings path, decided per face.**
//!
//! Two operands whose carriers touch — a sphere against a plane, two
//! spheres inside or outside one another, a sphere against a cylinder,
//! two skew cylinders — meet at one point of the carriers, and that
//! point lies on both faces or it does not. Each pose below sets the
//! touch on a part of one carrier its face does not hold (trimmed away
//! by a snowman's or a lens's other sphere, or past the end of a rod),
//! with no edge of either body touching a face, so the no-crossings
//! path runs and the faces are apart: every op builds, in both operand
//! orders, against the volumes of the disjoint or nested operands,
//! computed here from the radii and lengths alone. Each `on_face` twin
//! moves the touch onto both faces, where it is a real tangency of the
//! two boundaries, and keeps its refusal.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane};
use sweep::{ExtrudeSide, Extrusion, Revolution, extrude};
use topo::{Body, BooleanError, BooleanOp};

/// A ball of radius `r` centred at `c`, poled on y: its seam meridians
/// lie in the half-plane `z = 0, x ≥ 0` about its centre.
fn ball(r: f64, c: Vec3<f64>) -> Body<f64> {
    let b = sweep::test_support::revolved_about_y(
        vec![(Point2::new(0.0, -r), 1.0), (Point2::new(0.0, r), 0.0)],
        Revolution::Full,
        Tol::witness(),
    );
    topo::transform_rigid(&b, &Affine3::translation(c), Tol::witness()).unwrap()
}

/// A rod of radius `r` along z through `(x, y)`, from `z0` to `z1`: two
/// half-walls, whose seam lines run at `(x ± r, y)`.
fn rod_z(r: f64, (x, y): (f64, f64), (z0, z1): (f64, f64)) -> Body<f64> {
    let tol = Tol::witness();
    let lp = profile::circle(Point2::new(x, y), r, tol).unwrap();
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp.into()]).validate(tol).unwrap();
    extrude(
        &profile,
        Extrusion::Distance {
            depth: z1 - z0,
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .unwrap()
    .body
}

/// A rod of radius `r` along x through `(y, z)`, from `x0` to `x1`: a
/// z rod turned a quarter about y, which carries `(x, y, z)` to
/// `(z, y, −x)`. Its two seam lines run at `(y, z ± r)`, or at
/// `(y ± r, z)` once the rod is first turned a quarter about its own
/// axis (`seams_beside`).
fn rod_x(r: f64, (y, z): (f64, f64), (x0, x1): (f64, f64), seams_beside: bool) -> Body<f64> {
    let own = Affine3::rotation_about_axis(
        Point3::new(-z, y, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        if seams_beside { core::f64::consts::FRAC_PI_2 } else { 0.0 },
    );
    let quarter = Affine3::rotation_about_axis(
        Point3::origin(),
        Vec3::new(0.0, 1.0, 0.0),
        core::f64::consts::FRAC_PI_2,
    );
    topo::transform_rigid(&rod_z(r, (-z, y), (x0, x1)), &(quarter * own), Tol::witness())
        .unwrap()
}

fn ball_volume(r: f64) -> f64 {
    4.0 / 3.0 * PI * r.powi(3)
}

/// The volume of a spherical cap of height `h` on a sphere of radius `r`.
fn cap_volume(r: f64, h: f64) -> f64 {
    PI * h.powi(2) * (3.0 * r - h) / 3.0
}

/// The lens two balls `r1`, `r2` at centre distance `d` share.
fn lens_volume(r1: f64, r2: f64, d: f64) -> f64 {
    let x = (d.powi(2) + r1.powi(2) - r2.powi(2)) / (2.0 * d);
    cap_volume(r1, r1 - x) + cap_volume(r2, r2 - (d - x))
}

const R1: f64 = 1.0;
const R2: f64 = 0.8;
const D: f64 = 1.4;

fn snowman() -> Body<f64> {
    built(
        BooleanOp::Union,
        &ball(R1, Vec3::new(0.0, 0.0, 0.0)),
        &ball(R2, Vec3::new(0.0, D, 0.0)),
    )
    .expect("the snowman")
}

fn lens() -> Body<f64> {
    built(
        BooleanOp::Intersect,
        &ball(R1, Vec3::new(0.0, 0.0, 0.0)),
        &ball(R2, Vec3::new(0.0, D, 0.0)),
    )
    .expect("the lens")
}

fn boolean(op: BooleanOp, a: &Body<f64>, b: &Body<f64>) -> Result<topo::BooleanResult<f64>, BooleanError> {
    match op {
        BooleanOp::Union => topo::boolean::union(a, b, Tol::witness()),
        BooleanOp::Intersect => topo::boolean::intersect(a, b, Tol::witness()),
        BooleanOp::Subtract => topo::boolean::subtract(a, b, Tol::witness()),
    }
}

/// The boolean's body, `None` for an empty result; a refusal fails with
/// the payload.
fn built(op: BooleanOp, a: &Body<f64>, b: &Body<f64>) -> Option<Body<f64>> {
    boolean(op, a, b)
        .unwrap_or_else(|e| panic!("{op:?} refused: {e:?}"))
        .body()
        .map(|b| b.body.clone())
}

/// Every tier of validation, a closed tessellation, then the volume
/// against `want` through the kernel's mass properties (exact on
/// closed-form faces), to 1e-9 of the result floored at unit scale.
fn assert_body(label: &str, body: &Body<f64>, want: f64) {
    let tol = Tol::witness();
    assert_eq!(topo::validate(body), Ok(()), "{label}: validate");
    assert_eq!(topo::validate_closed(body), Ok(()), "{label}: validate_closed");
    assert_eq!(
        topo::validate_geometric(body, tol),
        Ok(()),
        "{label}: validate_geometric"
    );
    let m = mesh::tessellate(body, 1e-3, tol)
        .unwrap_or_else(|e| panic!("{label}: tessellates, got {e:?}"));
    assert_eq!(
        mesh::validate::check_mesh(&m),
        Ok(()),
        "{label}: a closed manifold mesh"
    );
    let p = topo::mass_properties(body, tol)
        .unwrap_or_else(|e| panic!("{label}: mass properties, got {e:?}"));
    assert_eq!(p.volume_pad, 0.0, "{label}: closed-form faces only");
    assert!(
        (p.volume - want).abs() <= 1e-9 * want.max(1.0),
        "{label}: volume {} against the closed form {want}",
        p.volume
    );
}

/// How two operands stand: apart (touching at a point at most), or the
/// second inside the first.
#[derive(Clone, Copy)]
enum Pose {
    Apart,
    Inside,
}

/// Every op in both operand orders, each against the closed form the
/// operands' volumes `va`, `vb` and their pose give.
fn assert_every_op(label: &str, a: &Body<f64>, b: &Body<f64>, (va, vb): (f64, f64), pose: Pose) {
    let (union, meet) = match pose {
        Pose::Apart => (va + vb, 0.0),
        Pose::Inside => (va, vb),
    };
    for (op, x, y, want) in [
        (BooleanOp::Union, a, b, union),
        (BooleanOp::Union, b, a, union),
        (BooleanOp::Intersect, a, b, meet),
        (BooleanOp::Intersect, b, a, meet),
        (BooleanOp::Subtract, a, b, va - meet),
        (BooleanOp::Subtract, b, a, vb - meet),
    ] {
        let name = format!("{label}: {op:?}");
        match built(op, x, y) {
            Some(out) => assert_body(&name, &out, want),
            None => assert!(want.abs() <= 1e-12, "{name}: empty against {want}"),
        }
    }
}

const OPS: [BooleanOp; 3] = [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract];

/// Every op in both operand orders refuses, and `expect` holds of each
/// refusal.
fn assert_every_op_refuses(label: &str, a: &Body<f64>, b: &Body<f64>, expect: fn(&BooleanError) -> bool) {
    for op in OPS {
        for (order, x, y) in [("A·B", a, b), ("B·A", b, a)] {
            match boolean(op, x, y) {
                Err(e) => assert!(expect(&e), "{label} {op:?} {order}: got {e:?}"),
                Ok(_) => panic!("{label} {op:?} {order}: built across a tangency on both faces"),
            }
        }
    }
}

/// The unit vector along `(x, y, z)`.
fn dir(x: f64, y: f64, z: f64) -> Vec3<f64> {
    let v = Vec3::new(x, y, z);
    v / v.norm()
}

/// **Sphere × sphere, internally tangent off the face.** A ball of
/// radius 0.4 inside the snowman's unit sphere, touching it at
/// `(0, 0.7, 0.3)/‖·‖`, a point inside the radius-0.8 ball, which the
/// snowman's unit-sphere face does not hold. The small ball's sphere
/// crosses the radius-0.8 carrier off its face, as the crossing case of
/// the same scan already reads.
#[test]
fn a_ball_touching_the_snowman_sphere_where_it_is_trimmed_builds() {
    let snowman = snowman();
    let v_snowman = ball_volume(R1) + ball_volume(R2) - lens_volume(R1, R2, D);
    let inner = ball(0.4, dir(0.0, 0.7, 0.3) * 0.6);
    assert_every_op("snowman ⊃ ball(0.4)", &snowman, &inner, (v_snowman, ball_volume(0.4)), Pose::Inside);
}

/// **Sphere × plane, tangent off the plane face.** The brick's bottom
/// plane `y = 1` touches the unit sphere at `(0, 1, 0)`, outside the
/// plane face (`x ≥ 1.5`) and inside the radius-0.8 ball.
#[test]
fn a_brick_whose_plane_touches_the_snowman_sphere_off_its_face_builds() {
    let snowman = snowman();
    let v_snowman = ball_volume(R1) + ball_volume(R2) - lens_volume(R1, R2, D);
    let brick: Body<f64> =
        sweep::test_support::brick((1.5, 3.0), (1.0, 2.0), (-1.0, 1.0), Tol::witness());
    assert_every_op("snowman, brick", &snowman, &brick, (v_snowman, 3.0), Pose::Apart);
}

/// **Sphere × sphere, externally tangent off the face.** A ball of
/// radius 0.3 outside the lens, touching the lens's unit sphere at
/// `−(0.3, 0.8, 0.5)/‖·‖`, far below the lens's top face (which keeps
/// only `y > 0.83`).
#[test]
fn a_ball_touching_the_lens_sphere_where_it_is_trimmed_builds() {
    let lens = lens();
    let outer = ball(0.3, dir(0.3, 0.8, 0.5) * -1.3);
    assert_every_op(
        "lens, ball(0.3)",
        &lens,
        &outer,
        (lens_volume(R1, R2, D), ball_volume(0.3)),
        Pose::Apart,
    );
}

/// **Sphere × cylinder, tangent past the rod's end.** A rod of radius
/// 0.5 along x at `(y, z) = (0, 1.5)`, from `x = 0.5`: its wall's
/// carrier touches the unit ball at `(0, 0, 1)`, before the rod begins.
#[test]
fn a_rod_whose_wall_touches_a_ball_past_its_end_builds() {
    let b = ball(1.0, Vec3::new(0.0, 0.0, 0.0));
    let rod = rod_x(0.5, (0.0, 1.5), (0.5, 2.0), true);
    assert_every_op(
        "ball, rod",
        &b,
        &rod,
        (ball_volume(1.0), PI * 0.25 * 1.5),
        Pose::Apart,
    );
}

/// **Cylinder × cylinder, skew and tangent past one rod's end.** A rod
/// along x through the origin and one along z through `(0, 1)`, both of
/// radius 0.5, so the carriers touch at `(0, 0.5, 0)`; the z rod starts
/// at `z = 0.2`.
#[test]
fn a_rod_whose_wall_touches_a_crossing_rod_past_its_end_builds() {
    let along_x = rod_x(0.5, (0.0, 0.0), (-2.0, 2.0), false);
    let along_z = rod_z(0.5, (0.0, 1.0), (0.2, 3.0));
    assert_every_op(
        "rod x, rod z",
        &along_x,
        &along_z,
        (PI * 0.25 * 4.0, PI * 0.25 * 2.8),
        Pose::Apart,
    );
}

/// **The same tangencies on both faces keep their refusals.** Each
/// touch is moved to `(0, 0, 1)` or `(0, 0.5, 0)`, a point interior to
/// both faces: the boundaries genuinely touch there, which the
/// no-crossings path does not cut in.
#[test]
fn the_same_tangencies_on_both_faces_refuse() {
    let unit = ball(1.0, Vec3::new(0.0, 0.0, 0.0));
    let brick: Body<f64> =
        sweep::test_support::brick((-1.0, 1.0), (-1.0, 1.0), (1.0, 2.0), Tol::witness());
    assert_every_op_refuses("ball, brick", &unit, &brick, |e| {
        matches!(
            e,
            BooleanError::Escalated {
                decision: topo::BooleanDecision::Sphere(topo::SphereQuestion::AgainstPlane),
                ..
            }
        )
    });
    let inner = ball(0.4, Vec3::new(0.0, 0.0, 0.6));
    assert_every_op_refuses("ball ⊃ ball(0.4)", &unit, &inner, |e| {
        matches!(e, BooleanError::SpheresMeet { .. })
    });
    let outer = ball(0.3, Vec3::new(0.0, 0.0, 1.3));
    assert_every_op_refuses("ball, ball(0.3)", &unit, &outer, |e| {
        matches!(
            e,
            BooleanError::Escalated {
                decision: topo::BooleanDecision::Sphere(topo::SphereQuestion::Apart),
                ..
            }
        )
    });
    let tangent = |e: &BooleanError| {
        matches!(e, BooleanError::FallbackExtentUnsupported { what, .. } if what.contains("tangent"))
    };
    let rod = rod_x(0.5, (0.0, 1.5), (-1.5, 2.0), true);
    assert_every_op_refuses("ball, rod", &unit, &rod, tangent);
    let along_x = rod_x(0.5, (0.0, 0.0), (-2.0, 2.0), false);
    let along_z = rod_z(0.5, (0.0, 1.0), (-1.0, 3.0));
    assert_every_op_refuses("rod x, rod z", &along_x, &along_z, tangent);
}

