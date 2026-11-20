//! **The offset door sections a cone with a coaxial sphere or torus**
//! (`topo::section_near`, the derive `offset_derive::section_closed`'s
//! cone arm shares): each lane's own door reads the partner, the
//! cylinder's answering the wrong lane for any other, and the circle
//! nearest the seed is the section's. A native row of the arm the STEP
//! build reaches through the same door.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_core::{Band, Point3, Tol, Vec3};

fn on_both(c: &Curve3<f64>, a: &Surface<f64>, b: &Surface<f64>) {
    for t in [0.0, 1.3, 2.9, 4.4] {
        let p = c.eval(t);
        let r = |s: &Surface<f64>| geom_brep::implicit_residual(s, p).abs();
        assert!(
            r(a) < 1e-12 && r(b) < 1e-12,
            "the section lies on both at t = {t}: {c:?}"
        );
    }
}

fn derived(cone: &Surface<f64>, other: &Surface<f64>, seed: &Curve3<f64>) -> Curve3<f64> {
    let span = (0.0, core::f64::consts::TAU);
    let region = geom_brep::Reach::of_span(seed, span).expect("the seed has a region");
    let band = Band::linear(Tol::witness()).expect("the run's band forms");
    match topo::section_near(
        None,
        (cone, other),
        (seed, span),
        &region,
        geom_core::test_support::fixture_lever(1.0),
        band,
    ) {
        Ok(Ok(c)) => c,
        other => panic!("the coaxial pair derives a circle: {other:?}"),
    }
}

#[test]
fn a_cone_and_a_coaxial_sphere_derive_their_circle() {
    let axis = Vec3::new(0.0, 0.0, 1.0);
    let u_ref = Vec3::new(1.0, 0.0, 0.0);
    let cone = Surface::Cone {
        apex: Point3::new(0.0, 0.0, 0.0),
        axis,
        half_angle: core::f64::consts::FRAC_PI_4,
        u_ref,
    };
    let sphere = Surface::Sphere {
        center: Point3::new(0.0, 0.0, 1.0),
        axis,
        radius: 1.0,
        u_ref,
    };
    let seed = Curve3::circle(Point3::new(0.0, 0.0, 1.0), axis, 1.0, u_ref);
    let c = derived(&cone, &sphere, &seed);
    on_both(&c, &cone, &sphere);
    assert!(
        matches!(&c, Curve3::Circle(k) if (k.radius - 1.0).abs() < 1e-12),
        "the circle at the sphere's equator, away from the apex: {c:?}"
    );
}

#[test]
fn a_cone_and_a_coaxial_torus_derive_their_circle() {
    let axis = Vec3::new(0.0, 1.0, 0.0);
    let u_ref = Vec3::new(1.0, 0.0, 0.0);
    let cone = Surface::Cone {
        apex: Point3::new(0.0, -0.65625, 0.0),
        axis,
        half_angle: (1.0_f64 / 6.0).atan(),
        u_ref,
    };
    let torus = Surface::Torus {
        center: Point3::new(0.0, 0.109375, 0.0),
        axis,
        major_radius: 0.140625,
        minor_radius: 0.078125,
        u_ref,
    };
    let seed = Curve3::circle(Point3::new(0.0, 0.1875, 0.0), axis, 0.140625, u_ref);
    let c = derived(&cone, &torus, &seed);
    on_both(&c, &cone, &torus);
    assert!(
        matches!(&c, Curve3::Circle(k) if (k.radius - 0.140625).abs() < 1e-12
            && (k.frame.origin.y - 0.1875).abs() < 1e-12),
        "the torus's top parallel, which the cone passes through: {c:?}"
    );
}
