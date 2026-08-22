//! **The analytic blend arms, pinned against their closed forms**
//! (M5 PR 12 §2).
//!
//! These rows are the geometric half of the ordering claim: the
//! setbacks the battery refuses on are these functions' outputs, so
//! if the closed forms here are right, predicate 2 is measuring the
//! real thing. They also pin the two structural facts the corner
//! patch rests on — the corner ball's centre lies on EVERY incident
//! blend cylinder's axis, and both have radius `r`, which is what
//! makes sphere and cylinder tangent along a whole circle rather than
//! crossing — and the second-order separation that keeps those
//! circular trimlines jet-determinate.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom::Curve3;
use geom::Surface;
use geom_brep::{implicit_residual, tangent_jet};
use geom_core::{Point3, Vec3};
use sweep::fillet::Convexity;
use sweep::fillet::blend::{corner_ball, plane_plane_blend, plane_sphere_blend};

fn p(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

fn v(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}

/// The box edge at `x = y = 0` of the solid `x, y > 0`: outward
/// normals `−x̂`, `−ŷ`, edge along `+ẑ`. The blend is the cylinder of
/// radius `r` about `(r, r, ·)`, the trimlines are the rulings at
/// `(0, r, ·)` and `(r, 0, ·)`, and the setback is exactly `r`.
#[test]
fn plane_plane_blend_on_a_right_dihedral_is_the_quarter_cylinder() {
    let r = 0.15;
    let b = plane_plane_blend(
        p(0.0, 0.0, 0.4),
        v(0.0, 0.0, 1.0),
        v(-1.0, 0.0, 0.0),
        v(0.0, -1.0, 0.0),
        r,
        true,
    );
    let Surface::Cylinder {
        origin,
        axis,
        radius,
        ..
    } = b.surface
    else {
        panic!("a plane–plane blend is a cylinder");
    };
    assert!((origin.x - r).abs() < 1e-15 && (origin.y - r).abs() < 1e-15);
    assert!((axis - v(0.0, 0.0, 1.0)).norm() < 1e-15);
    assert!((radius - r).abs() < 1e-15);
    assert_eq!(b.spine_curvature, 0.0, "a straight spine");
    assert!((b.trim_a.1 - r).abs() < 1e-15, "the 90° setback is r");
    assert!((b.trim_b.1 - r).abs() < 1e-15);
    let Curve3::Line { origin: oa, .. } = b.trim_a.0 else {
        panic!("a straight trimline");
    };
    assert!(
        (oa.x).abs() < 1e-15 && (oa.y - r).abs() < 1e-15,
        "on x = 0 at y = r"
    );
}

/// The setback closed form `r·tan(φ/2)` (φ = the outward-normal
/// angle), checked away from the right angle where the two candidate
/// formulas would agree by accident. A shallow wedge sets back less
/// than `r`; a sharp one sets back more.
#[test]
fn plane_plane_setback_is_r_times_tan_half_the_normal_angle() {
    let r = 0.1;
    for phi_deg in [30.0_f64, 60.0, 90.0, 120.0, 150.0] {
        let phi = phi_deg.to_radians();
        let n_a = v(-1.0, 0.0, 0.0);
        let n_b = v(-phi.cos(), -phi.sin(), 0.0);
        let b = plane_plane_blend(p(0.0, 0.0, 0.0), v(0.0, 0.0, 1.0), n_a, n_b, r, true);
        let want = r * (phi / 2.0).tan();
        assert!(
            (b.trim_a.1 - want).abs() < 1e-12,
            "φ = {phi_deg}°: setback {} vs r·tan(φ/2) = {want}",
            b.trim_a.1
        );
    }
}

/// A CONCAVE chain rolls the ball on the other side: the centre moves
/// to the opposite side of both supports, and the blend face's sense
/// bit is `false` — read off the stored convexity verdict, never off
/// a sampled normal (S10/S11).
#[test]
fn a_concave_chain_rolls_the_ball_on_the_other_side_and_reverses_sense() {
    let r = 0.15;
    let (n_a, n_b) = (v(-1.0, 0.0, 0.0), v(0.0, -1.0, 0.0));
    let convex = plane_plane_blend(p(0.0, 0.0, 0.0), v(0.0, 0.0, 1.0), n_a, n_b, r, true);
    let concave = plane_plane_blend(p(0.0, 0.0, 0.0), v(0.0, 0.0, 1.0), n_a, n_b, r, false);
    let (Surface::Cylinder { origin: oc, .. }, Surface::Cylinder { origin: ok, .. }) =
        (convex.surface, concave.surface)
    else {
        panic!("cylinders");
    };
    assert!((oc.x - r).abs() < 1e-15 && (ok.x + r).abs() < 1e-15);
    assert!(Convexity::Convex.blend_sense());
    assert!(!Convexity::Concave.blend_sense());
}

/// The corner ball of the box corner at the origin (solid `x,y,z > 0`)
/// sits at `(r, r, r)` with `|det(n₁,n₂,n₃)| = 1`.
#[test]
fn the_corner_ball_is_the_point_at_depth_r_inside_all_three_planes() {
    let r = 0.15;
    let ball = corner_ball(
        [p(0.0, 0.0, 0.0); 3],
        [v(-1.0, 0.0, 0.0), v(0.0, -1.0, 0.0), v(0.0, 0.0, -1.0)],
        r,
        true,
    );
    assert!((ball.center - p(r, r, r)).norm() < 1e-15);
    assert!((ball.independence - 1.0).abs() < 1e-15);
    let Surface::Sphere { center, radius, .. } = ball.surface else {
        panic!("a sphere");
    };
    assert!((center - p(r, r, r)).norm() < 1e-15);
    assert!((radius - r).abs() < 1e-15);
}

/// **The structural fact the octant patch rests on**: the corner
/// ball's centre lies ON the axis of every incident blend cylinder
/// (each axis is the locus at distance `r` from two of the three
/// planes; the centre is at distance `r` from all three), and both
/// have radius `r`. Two surfaces of revolution that share a point on
/// one's axis and have equal radii are TANGENT along a whole circle —
/// which is why the corner trimlines are circles and why the jet
/// certificate's circle arm is the one that certifies them.
#[test]
fn the_corner_ball_centre_lies_on_every_incident_cylinder_axis() {
    let r = 0.15;
    let normals = [v(-1.0, 0.0, 0.0), v(0.0, -1.0, 0.0), v(0.0, 0.0, -1.0)];
    let ball = corner_ball([p(0.0, 0.0, 0.0); 3], normals, r, true);
    // The three edges at the corner, each between a pair of planes.
    for (i, j, tau) in [
        (0usize, 1usize, v(0.0, 0.0, 1.0)),
        (1, 2, v(1.0, 0.0, 0.0)),
        (2, 0, v(0.0, 1.0, 0.0)),
    ] {
        let b = plane_plane_blend(p(0.0, 0.0, 0.0), tau, normals[i], normals[j], r, true);
        let Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } = b.surface
        else {
            panic!("a cylinder");
        };
        let d = ball.center - origin;
        let off = (d - axis * d.dot(axis)).norm();
        assert!(off < 1e-14, "the ball centre is off the axis by {off}");
        assert!((radius - r).abs() < 1e-15, "equal radii");
    }
}

/// The pip-rim torus: plane `z = 0` with material below, pip ball of
/// radius `R` centred `c` ABOVE the plane. Closed forms:
/// `h = c + r`, `s = √((R+r)² − h²)`, torus centre `(0,0,−r)`, plane
/// trimline the circle of radius `s` at `z = 0`, sphere trimline the
/// circle of radius `Rs/(R+r)` at `z = −r(R−c)/(R+r)`; and `s > a`
/// for the rim radius `a = √(R²−c²)` — the blend eats into the FLAT
/// face, which is what makes it a fillet.
#[test]
fn plane_sphere_blend_is_the_rim_torus_and_it_widens_the_flat_face() {
    let (big_r, c, r) = (0.5, 0.2, 0.06);
    let b = plane_sphere_blend(
        p(0.0, 0.0, 0.0),
        v(0.0, 0.0, 1.0),
        v(1.0, 0.0, 0.0),
        p(0.0, 0.0, c),
        big_r,
        r,
        // The pip's dimple: material OUTSIDE the ball.
        false,
    );
    let h = c + r;
    let s = ((big_r + r).powi(2) - h * h).sqrt();
    let a = (big_r * big_r - c * c).sqrt();
    assert!(s > a, "s = {s} must exceed the rim radius a = {a}");
    assert!(
        (s * s - (a * a + 2.0 * r * (big_r - c))).abs() < 1e-12,
        "s² = a² + 2r(R − c)"
    );
    let Surface::Torus {
        center,
        major_radius,
        minor_radius,
        ..
    } = b.surface
    else {
        panic!("a plane–sphere blend is a torus");
    };
    assert!((center - p(0.0, 0.0, -r)).norm() < 1e-14);
    assert!((major_radius - s).abs() < 1e-12);
    assert!((minor_radius - r).abs() < 1e-15);
    assert!((b.spine_curvature - 1.0 / s).abs() < 1e-12);
    let Curve3::Circle {
        center: ca,
        radius: ra,
        ..
    } = b.trim_a.0
    else {
        panic!("a circular trimline on the plane");
    };
    assert!(ca.z.abs() < 1e-14 && (ra - s).abs() < 1e-12);
    assert!(
        (b.trim_a.1 - (s - a)).abs() < 1e-12,
        "the plane-side setback is s − a"
    );
    let Curve3::Circle {
        center: cb,
        radius: rb,
        ..
    } = b.trim_b.0
    else {
        panic!("a circular trimline on the sphere");
    };
    assert!((rb - big_r * s / (big_r + r)).abs() < 1e-12);
    assert!((cb.z + r * (big_r - c) / (big_r + r)).abs() < 1e-12);
}

/// **The pair's other configuration: a CONVEX sphere.** A solid of
/// revolution's latitude rim meets its flat face from the other side —
/// the material is INSIDE the sphere, so the rolling ball rides the
/// `R − r` offset sphere, not the `R + r` one, and the blend eats
/// INWARD on the flat face instead of widening a hole. Closed forms
/// for the unit dome (plane z = 0 with outward normal −z, sphere of
/// radius R at the origin, rim the equator): `h = r`,
/// `s = √((R − r)² − r²)`, torus centre `(0, 0, r)`, plane trimline the
/// circle of radius `s` at `z = 0`, sphere trimline
/// `R·s/(R − r)` — and `s < a = R`, the shrink.
#[test]
fn plane_sphere_blend_takes_the_inner_offset_against_a_convex_sphere() {
    let (big_r, r) = (1.0, 0.05);
    let b = plane_sphere_blend(
        p(0.0, 0.0, 0.0),
        v(0.0, 0.0, -1.0),
        v(1.0, 0.0, 0.0),
        p(0.0, 0.0, 0.0),
        big_r,
        r,
        true,
    );
    let h = r;
    let s = ((big_r - r).powi(2) - h * h).sqrt();
    assert!(s < big_r, "s = {s} must fall inside the rim radius {big_r}");
    let Surface::Torus {
        center,
        major_radius,
        minor_radius,
        ..
    } = b.surface
    else {
        panic!("a plane–sphere blend is a torus");
    };
    assert!(
        (center - p(0.0, 0.0, r)).norm() < 1e-14,
        "the ball rests ABOVE the plane"
    );
    assert!((major_radius - s).abs() < 1e-12);
    assert!((minor_radius - r).abs() < 1e-15);
    assert!(
        major_radius > minor_radius,
        "the spine is a ring torus, which is what #889's net also checks at rest"
    );
    let Curve3::Circle {
        center: ca,
        radius: ra,
        ..
    } = b.trim_a.0
    else {
        panic!("a circular trimline on the plane");
    };
    assert!(ca.z.abs() < 1e-14 && (ra - s).abs() < 1e-12);
    assert!(
        (b.trim_a.1 - (big_r - s)).abs() < 1e-12,
        "the plane-side setback is a − s: the flat face SHRINKS"
    );
    let Curve3::Circle {
        center: cb,
        radius: rb,
        ..
    } = b.trim_b.0
    else {
        panic!("a circular trimline on the sphere");
    };
    assert!((rb - big_r * s / (big_r - r)).abs() < 1e-12);
    assert!(
        (cb.z - r * big_r / (big_r - r)).abs() < 1e-12,
        "the contact circle sits ABOVE the plane, on the sphere"
    );
}

/// The rim blend's two trimlines are genuine TANGENCY loci with a
/// second-order separation bounded away from zero: on the plane
/// `|κ_rel| = 1/r` (the tube against a flat face), on the sphere
/// `|κ_rel| = 1/R + 1/r` (the two curve away from each other, so the
/// curvatures ADD). Both are the `TangentIntersection` regime, and
/// both are exactly what the jet certificate needs.
#[test]
fn the_rim_trimlines_are_tangency_loci_with_definite_second_order_separation() {
    let (big_r, c, r) = (0.5, 0.2, 0.06);
    let plane = Surface::Plane {
        origin: p(0.0, 0.0, 0.0),
        normal: v(0.0, 0.0, 1.0),
        u_ref: v(1.0, 0.0, 0.0),
    };
    let sphere = Surface::Sphere {
        center: p(0.0, 0.0, c),
        radius: big_r,
        axis: v(0.0, 0.0, 1.0),
        u_ref: v(1.0, 0.0, 0.0),
    };
    let b = plane_sphere_blend(
        p(0.0, 0.0, 0.0),
        v(0.0, 0.0, 1.0),
        v(1.0, 0.0, 0.0),
        p(0.0, 0.0, c),
        big_r,
        r,
        // The pip's dimple: material OUTSIDE the ball.
        false,
    );
    for (support, trim, want) in [
        (&plane, &b.trim_a.0, 1.0 / r),
        (&sphere, &b.trim_b.0, 1.0 / big_r + 1.0 / r),
    ] {
        for i in 0..9 {
            let t = 2.0 * PI * f64::from(i) / 8.0;
            let pt = trim.eval(t);
            assert!(
                implicit_residual(support, pt).abs() < 1e-12,
                "the trimline lies on its support"
            );
            assert!(
                implicit_residual(&b.surface, pt).abs() < 1e-12,
                "the trimline lies on the blend"
            );
            let jet = tangent_jet(&b.surface, support, pt, trim.deriv(t));
            assert!(
                jet.sin_theta.abs() < 1e-9,
                "tangency: sin θ = {}",
                jet.sin_theta
            );
            assert!(
                (jet.kappa_rel.abs() - want).abs() < 1e-6,
                "κ_rel magnitude {} vs {want}",
                jet.kappa_rel.abs()
            );
        }
    }
}
