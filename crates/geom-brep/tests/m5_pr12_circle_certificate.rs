//! The jet certificate's **circle arm** (M5 PR 12): the fillet
//! trimlines the die mints are contact CIRCLES, not lines — the
//! corner ball against its edge cylinders, and the pip-rim torus
//! blend against the flat face and the pip sphere.
//!
//! The contract these rows pin is the equivariance one: in the
//! coaxial configuration (the only configuration in which two
//! distinct elementary surfaces of revolution are tangent along a
//! whole circle) both span bounds are EXACTLY zero, because `κ_rel`
//! and the implicit residual are isometry invariants and the
//! carrier's motion is a symmetry flow of both surfaces. A
//! configuration that misses coaxiality pays continuously — there is
//! no gate, so no tangency is ever demanded that cannot be certified.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Curve3;
use geom::Surface;
use geom_brep::{
    EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec, SurfaceKey, tangent_certificate_lane,
    tangent_jet,
};
use geom_core::Tol;
use geom_core::{Band, Point3, Vec3};
use slotmap::SlotMap;

fn band() -> Band {
    let tol = Tol::witness().get();
    Band::new(tol.eps, tol.k * tol.eps).unwrap()
}

fn p(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

fn v(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}

/// The corner configuration of the filleted die: the corner ball
/// (radius r, centred on the edge cylinder's axis) and one edge
/// cylinder (radius r). They are tangent along the full circle at
/// the ball's latitude — the octant patch's trimline.
fn corner_pair(r: f64) -> (Surface<f64>, Surface<f64>, Curve3<f64>) {
    let sphere = Surface::Sphere {
        center: p(r, r, r),
        radius: r,
        axis: v(0.0, 0.0, 1.0),
        u_ref: v(1.0, 0.0, 0.0),
    };
    let cylinder = Surface::Cylinder {
        origin: p(r, r, 0.0),
        axis: v(0.0, 0.0, 1.0),
        radius: r,
        u_ref: v(1.0, 0.0, 0.0),
    };
    let circle = Curve3::Circle {
        center: p(r, r, r),
        axis: v(0.0, 0.0, 1.0),
        radius: r,
        u_ref: v(1.0, 0.0, 0.0),
    };
    (sphere, cylinder, circle)
}

#[test]
fn circle_carriers_are_in_the_certified_lane_on_every_revolution_kind() {
    let (sphere, cylinder, circle) = corner_pair(0.2);
    let plane = Surface::Plane {
        origin: p(0.0, 0.0, 0.0),
        normal: v(0.0, 0.0, 1.0),
        u_ref: v(1.0, 0.0, 0.0),
    };
    let torus = Surface::Torus {
        center: p(0.0, 0.0, 0.0),
        axis: v(0.0, 0.0, 1.0),
        major_radius: 1.0,
        minor_radius: 0.2,
        u_ref: v(1.0, 0.0, 0.0),
    };
    for s2 in [&sphere, &cylinder, &plane, &torus] {
        assert!(
            tangent_certificate_lane(&circle, &torus, s2),
            "circle carriers admit every surface of revolution"
        );
    }
    // The LINE arm is unchanged: torus stays out of it.
    let line = Curve3::Line {
        origin: p(0.0, 0.0, 0.0),
        dir: v(0.0, 0.0, 1.0),
    };
    assert!(!tangent_certificate_lane(&line, &torus, &plane));
    assert!(tangent_certificate_lane(&line, &cylinder, &plane));
    // A carrier kind outside both arms stays outside.
    let ellipse = Curve3::Ellipse {
        center: p(0.0, 0.0, 0.0),
        axis: v(0.0, 0.0, 1.0),
        major: 2.0,
        minor: 1.0,
        u_ref: v(1.0, 0.0, 0.0),
    };
    assert!(!tangent_certificate_lane(&ellipse, &sphere, &plane));
}

/// The die's corner trimline certifies as `TangentIntersection` with
/// the circle carrier — the acceptance shape of the whole arm.
#[test]
fn the_corner_ball_cylinder_circle_certifies_as_a_tangent_intersection() {
    let r = 0.2;
    let (sphere, cylinder, circle) = corner_pair(r);
    let mut surfaces: SlotMap<SurfaceKey, Surface<f64>> = SlotMap::with_key();
    let k_sphere = surfaces.insert(sphere.clone());
    let k_cyl = surfaces.insert(cylinder.clone());
    // A QUARTER of the contact circle — one octant patch edge.
    let (t0, t1) = (0.0, core::f64::consts::FRAC_PI_2);
    let start = p(r + r, r, r);
    let end = p(r, r + r, r);
    let witness = p(
        r + r * core::f64::consts::FRAC_PI_4.cos(),
        r + r * core::f64::consts::FRAC_PI_4.sin(),
        r,
    );
    let spec = EdgeCurveSpec {
        description: EdgeDescriptionSpec::TangentIntersection {
            s1: k_sphere,
            s2: k_cyl,
            witness,
        },
        carrier: circle,
        param_start: t0,
        param_end: t1,
    };
    let curve = EdgeCurve::certify(spec, start, end, |k| surfaces.get(k).cloned(), band())
        .expect("the corner trimline is a certified tangent intersection");
    assert!(
        curve.certificate().max_residual < 1e-12,
        "the coaxial contact circle is EXACT geometry: {}",
        curve.certificate().max_residual
    );
}

/// The second-order separation the certificate rests on: the sphere
/// curves with radius r transverse to the contact circle while the
/// cylinder is FLAT there (its ruling), so `κ_rel = ±1/r` — bounded
/// away from zero, and constant along the circle by equivariance.
#[test]
fn the_corner_trimline_jet_is_one_over_the_radius_everywhere() {
    let r = 0.2;
    let (sphere, cylinder, circle) = corner_pair(r);
    let mut seen = Vec::new();
    for i in 0..9 {
        let t = core::f64::consts::FRAC_PI_2 * f64::from(i) / 8.0;
        let point = circle.eval(t);
        let tangent = circle.deriv(t);
        let jet = tangent_jet(&sphere, &cylinder, point, tangent);
        assert!(
            jet.sin_theta.abs() < 1e-12,
            "tangency is exact at t = {t}: sin θ = {}",
            jet.sin_theta
        );
        seen.push(jet.kappa_rel);
    }
    for k in &seen {
        assert!(
            (k.abs() - 1.0 / r).abs() < 1e-9,
            "κ_rel must be ±1/r = {} everywhere, got {k}",
            1.0 / r
        );
    }
    // Equivariance, made a row: the jet is CONSTANT along the circle
    // — which is exactly why the certificate's drift bound is zero.
    for k in &seen {
        assert!((k - seen[0]).abs() < 1e-12, "κ_rel drifts along the circle");
    }
}
