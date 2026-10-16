//! C4's placement: a periodic chart's image is placed over its edge's
//! span in the chart's principal window ([`super::ANGLE_WINDOW_START`]),
//! so where a carrier's parameter starts moves no stored image.

use core::f64::consts::TAU;

use geom::{Curve3, Surface};
use geom_core::{Band, Point3, Tol, Vec3};

use super::{ANGLE_WINDOW_START, PcurveCache, chart_pcurve_over};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn circle(center: Point3<f64>, axis: Vec3<f64>, radius: f64, u_ref: Vec3<f64>) -> Curve3<f64> {
    Curve3::Circle {
        center,
        axis,
        radius,
        u_ref,
    }
}

/// A named arm: its chart, its carrier, and an edge's span on it.
type Arm = (&'static str, Surface<f64>, Curve3<f64>, (f64, f64));

/// One carrier per periodic arm of the derivation, with an edge's span.
fn arms() -> Vec<Arm> {
    let (x, y, z) = (Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z());
    let o = Point3::new(0.0, 0.0, 0.0);
    let cylinder = Surface::Cylinder {
        origin: o,
        axis: z,
        radius: 1.0,
        u_ref: x,
    };
    let cone = Surface::Cone {
        apex: o,
        axis: z,
        half_angle: 0.4,
        u_ref: x,
    };
    let sphere = Surface::Sphere {
        center: o,
        radius: 1.0,
        axis: z,
        u_ref: x,
    };
    let torus = Surface::Torus {
        center: o,
        axis: z,
        major_radius: 2.0,
        minor_radius: 0.5,
        u_ref: x,
    };
    let lat: f64 = 0.5;
    let tube: f64 = 0.3;
    vec![
        (
            "cylinder rim",
            cylinder.clone(),
            circle(Point3::new(0.0, 0.0, 0.5), z, 1.0, x),
            (0.3, 1.9),
        ),
        (
            "cylinder rim, reversed",
            cylinder,
            circle(Point3::new(0.0, 0.0, 0.5), -z, 1.0, x),
            (4.0, 5.9),
        ),
        (
            "cone rim",
            cone,
            circle(Point3::new(0.0, 0.0, 1.0), z, 0.4_f64.tan(), x),
            (2.0, 3.5),
        ),
        (
            "sphere parallel",
            sphere.clone(),
            circle(Point3::new(0.0, 0.0, lat.sin()), z, lat.cos(), x),
            (5.0, 6.2),
        ),
        ("sphere meridian", sphere, circle(o, -y, 1.0, x), (0.2, 1.0)),
        (
            "torus parallel",
            torus.clone(),
            circle(
                Point3::new(0.0, 0.0, 0.5 * tube.sin()),
                z,
                2.0 + 0.5 * tube.cos(),
                x,
            ),
            (1.0, 4.0),
        ),
        (
            "torus meridian",
            torus,
            circle(Point3::new(2.0, 0.0, 0.0), -y, 0.5, x),
            (0.1, 2.0),
        ),
    ]
}

fn in_window(u: f64) -> bool {
    ANGLE_WINDOW_START < u && u <= ANGLE_WINDOW_START + TAU
}

/// **A carrier restated whole periods off derives one placed image.**
/// On every periodic arm, the edge's span shifted by `k` whole turns of
/// the carrier's parameter (`k` from −10 to 10) derives an image that
/// meets the unshifted one at every point of the span, its azimuth in
/// the principal window at the span's midpoint, and certifies with the
/// same envelope: check 4 re-derives the image the mint placed.
#[test]
fn a_carrier_restated_whole_periods_off_derives_one_placed_image() {
    let band = band();
    for (name, surface, carrier, (t0, t1)) in arms() {
        let derive = |k: f64| {
            let (a, b) = (t0 + k * TAU, t1 + k * TAU);
            let image = chart_pcurve_over(&carrier, a, b, &surface, band)
                .unwrap_or_else(|e| panic!("{name}, {k} turns: derives: {e:?}"));
            let cache = PcurveCache::certify(image.clone(), a, b, &carrier, &surface, band)
                .unwrap_or_else(|e| panic!("{name}, {k} turns: certifies: {e:?}"));
            (image, cache.certificate().envelope, (a, b))
        };
        let (base, envelope, _) = derive(0.0);
        for k in [-10.0, -1.0, 1.0, 10.0] {
            let (image, env, (a, b)) = derive(k);
            for s in [0.0, 0.25, 0.5, 0.75, 1.0] {
                let (p, q) = (image.eval(a + s * (b - a)), base.eval(t0 + s * (t1 - t0)));
                assert!(
                    (p.x - q.x).abs() < 1e-9 && (p.y - q.y).abs() < 1e-9,
                    "{name}, {k} turns, at {s} of the span: {p:?} against {q:?}"
                );
            }
            assert!(
                in_window(image.eval(0.5 * (a + b)).x),
                "{name}, {k} turns: the azimuth at the midpoint is in the window"
            );
            assert!(
                (env - envelope).abs() < 1e-12,
                "{name}, {k} turns: the envelope {env:e} against {envelope:e}"
            );
        }
    }
}

/// **The window holds an image's midpoint, wherever the edge sits.** A
/// cylinder rim's edge of a fixed length slid around the circle in
/// steps of a tenth of a turn reads its azimuth at the span's midpoint
/// in `(−1, τ − 1]` every time.
#[test]
fn every_edge_of_a_rim_is_placed_in_the_window_at_its_midpoint() {
    let band = band();
    let cylinder = Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    let rim = circle(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::unit_z(),
        1.0,
        Vec3::unit_x(),
    );
    for step in 0..30 {
        let a = -10.0 + 0.1 * TAU * f64::from(step);
        let image = chart_pcurve_over(&rim, a, a + 1.3, &cylinder, band).unwrap();
        let u = image.eval(a + 0.65).x;
        assert!(in_window(u), "an edge from {a}: its midpoint azimuth {u}");
    }
}

// ---- review 5225 r2 probes ----

fn villarceau(phi: f64, family: f64, psi: f64) -> (Surface<f64>, Curve3<f64>) {
    let (major, minor) = (2.0, 0.7);
    let axis = Vec3::unit_z();
    let u_ref = Vec3::unit_x();
    let torus = Surface::Torus {
        center: Point3::new(0.0, 0.0, 0.0),
        axis,
        major_radius: major,
        minor_radius: minor,
        u_ref,
    };
    let d = u_ref * phi.cos() + axis.cross(u_ref) * phi.sin();
    let tilt = (minor / major).asin();
    let lean = d.cross(axis) * tilt.cos() + axis * (family * tilt.sin());
    let c = Curve3::Circle {
        center: Point3::new(0.0, 0.0, 0.0) + d * minor,
        axis: d.cross(lean),
        radius: major,
        u_ref: d * psi.cos() + lean * psi.sin(),
    };
    (torus, c)
}

fn report(name: &str, surface: &Surface<f64>, carrier: &Curve3<f64>, (t0, t1): (f64, f64)) -> Vec<String> {
    let band = band();
    let mut bad = vec![];
    let base = chart_pcurve_over(carrier, t0, t1, surface, band).unwrap();
    for k in [-3.0, -1.0, 0.0, 1.0, 3.0] {
        let (a, b) = (t0 + k * TAU, t1 + k * TAU);
        let image = chart_pcurve_over(carrier, a, b, surface, band).unwrap();
        let m = image.eval(0.5 * (a + b));
        let q = base.eval(0.5 * (t0 + t1));
        eprintln!("{name} k={k}: mid {m:?} (base {q:?})");
        if !(in_window(m.x)) { bad.push(format!("{name} k={k}: u {} out of window", m.x)); }
        if (m.y - q.y).abs() > 1e-9 { bad.push(format!("{name} k={k}: v {} vs base {}", m.y, q.y)); }
        if (m.x - q.x).abs() > 1e-9 { bad.push(format!("{name} k={k}: u {} vs base {}", m.x, q.x)); }
    }
    bad
}

#[test]
fn r2_villarceau_v_channel() {
    let (torus, c) = villarceau(0.7, 1.0, 0.3);
    let bad = report("villarceau", &torus, &c, (0.4, 1.6));
    assert!(bad.is_empty(), "{bad:#?}");
}

#[test]
fn r2_spiric_wall_v_channel() {
    let (r, rr, d) = (0.09375, 0.0703125, 0.0078125);
    let c = Curve3::Spiric {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::unit_y(),
        u_ref: Vec3::unit_x(),
        major_radius: r,
        minor_radius: rr,
        offset: d,
    };
    let torus = Surface::Torus {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::unit_y(),
        major_radius: r,
        minor_radius: rr,
        u_ref: Vec3::unit_x(),
    };
    let t = 0.8911225078866529;
    let bad = report("spiric wall", &torus, &c, (-t, t));
    assert!(bad.is_empty(), "{bad:#?}");
}

#[test]
fn r2_sphere_meridian_south() {
    let sphere = Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: 1.0,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    // meridian in the xz plane; circle(o, -y, 1, x): t=0 at +x, t=π/2 at ?
    let c = circle(Point3::new(0.0, 0.0, 0.0), -Vec3::unit_y(), 1.0, Vec3::unit_x());
    let band = band();
    for (t0, t1) in [(0.2, 1.0), (-1.5, -1.1), (-1.4, -0.9), (4.9, 5.2)] {
        let image = chart_pcurve_over(&c, t0, t1, &sphere, band).unwrap();
        let old = super::chart_pcurve(&c, &sphere, band).unwrap();
        eprintln!("sphere meridian [{t0},{t1}]: mid {:?}, ends {:?} {:?}; spanless mid {:?}", image.eval(0.5*(t0+t1)), image.eval(t0), image.eval(t1), old.eval(0.5*(t0+t1)));
        let cert = PcurveCache::certify(image.clone(), t0, t1, &c, &sphere, band);
        eprintln!("  certifies: {:?}", cert.as_ref().map(|c| c.certificate().envelope));
    }
}

#[test]
fn r2_cone_focal_u() {
    let cone = Surface::Cone {
        apex: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::unit_z(),
        half_angle: 0.4,
        u_ref: Vec3::unit_x(),
    };
    // an oblique plane z = 1 + 0.2 x cuts an ellipse
    let n = Vec3::new(-0.2, 0.0, 1.0).normalize();
    // find ellipse via intersect? build directly: param the section numerically is hard; use geom's plane-cone section if any
    let _ = (cone, n);
}
