//! Reviewer probes for PR 5225.
#![allow(clippy::unwrap_used, clippy::panic)]
use core::f64::consts::TAU;
use geom::{Curve3, Surface};
use geom_brep::chart_pcurve_over;
use geom_core::{Band, Point3, Tol, Vec3};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// A Villarceau circle restated whole periods off: does the stored
/// image (both channels) stay put?
#[test]
fn q1_villarceau_restated_whole_periods_off() {
    let (big, r) = (2.0_f64, 0.5_f64);
    let th = (r / big).asin();
    let torus = Surface::Torus {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::unit_z(),
        major_radius: big,
        minor_radius: r,
        u_ref: Vec3::unit_x(),
    };
    let c = Curve3::Circle {
        center: Point3::new(0.0, r, 0.0),
        axis: Vec3::new(-th.sin(), 0.0, th.cos()),
        radius: big,
        u_ref: Vec3::new(th.cos(), 0.0, th.sin()),
    };
    let (t0, t1) = (0.4, 1.6);
    let base = chart_pcurve_over(&c, t0, t1, &torus, band()).unwrap();
    let m0 = base.eval(0.5 * (t0 + t1));
    eprintln!("base {base:?}\n mid {m0:?}");
    let mut bad = vec![];
    for k in [-3.0, -1.0, 1.0, 3.0] {
        let (a, b) = (t0 + k * TAU, t1 + k * TAU);
        let img = chart_pcurve_over(&c, a, b, &torus, band()).unwrap();
        let m = img.eval(0.5 * (a + b));
        eprintln!("k={k}: mid {m:?}");
        if (m.x - m0.x).abs() > 1e-9 || (m.y - m0.y).abs() > 1e-9 {
            bad.push((k, m, m0));
        }
    }
    assert!(bad.is_empty(), "{bad:?}");
}

/// The sphere meridian's polar channel: where does an edge below
/// latitude −1 put its midpoint?
#[test]
fn q2_sphere_meridian_polar_channel_near_the_south_pole() {
    let sphere = Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: 1.0,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    // point(t) = cos t x + sin t z
    let c = Curve3::Circle {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, -1.0, 0.0),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    for (a, b) in [(-1.4, -1.1), (-0.9, -0.3), (1.1, 1.4)] {
        let img = chart_pcurve_over(&c, a, b, &sphere, band()).unwrap();
        eprintln!(
            "span [{a},{b}]: start {:?} mid {:?} end {:?}",
            img.eval(a),
            img.eval(0.5 * (a + b)),
            img.eval(b)
        );
    }
}
