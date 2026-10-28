//! Review probe (PR 5889, lane b): the anchored incidence read.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use crate::shared::tol::{band, eps};
use geom::{Curve3, Surface};
use geom_brep::chart_pcurve;
use geom_core::{Interval, Point3, Real, Vec3};

fn show(what: &str, c: &Curve3<f64>, s: &Surface<f64>) {
    let lift = Interval::from_f64;
    let f = chart_pcurve(c, s, band()).map(|_| ());
    let i = chart_pcurve(&c.map_scalar(lift), &s.map_scalar(lift), band()).map(|_| ());
    println!("eps={:e} {what}\n   f64: {f:?}\n   iv : {i:?}", eps());
}

#[test]
fn probe_5889b_anchor() {
    let far = Point3::new(1e7, 1e7, 1e7);
    // (a) a genuine 0.5 m miss far out.
    show(
        "rim 0.5 off, 1e7 out",
        &Curve3::Circle { center: far, axis: Vec3::unit_z(), radius: 1.5, u_ref: Vec3::unit_x() },
        &Surface::Cylinder { origin: far, axis: Vec3::unit_z(), radius: 1.0, u_ref: Vec3::unit_x() },
    );
    // (b) an exact rim far from its chart's anchor along a tilted axis.
    let axis = Vec3::new(0.6, 0.0, 0.8);
    let u = Vec3::new(0.8, 0.0, -0.6);
    for h in [1e2, 1e3, 1e4, 1e5] {
        let c = Point3::new(0.6 * h, 0.0, 0.8 * h);
        show(
            &format!("exact rim {h:e} up a tilted axis, anchor at the origin"),
            &Curve3::Circle { center: c, axis, radius: 1.0, u_ref: u },
            &Surface::Cylinder { origin: Point3::origin(), axis, radius: 1.0, u_ref: u },
        );
        // the same chart, anchored at the rim
        show(
            &format!("same rim, chart anchored at the rim ({h:e})"),
            &Curve3::Circle { center: c, axis, radius: 1.0, u_ref: u },
            &Surface::Cylinder { origin: c, axis, radius: 1.0, u_ref: u },
        );
        // plane: an exact circle in a tilted plane, anchor far away in-plane
        let n = Vec3::new(0.6, 0.0, 0.8);
        show(
            &format!("exact circle {h:e} from a tilted plane's anchor"),
            &Curve3::Circle { center: Point3::new(0.8 * h, 0.0, -0.6 * h), axis: n, radius: 1.0, u_ref: Vec3::unit_y() },
            &Surface::Plane { origin: Point3::origin(), normal: n, u_ref: Vec3::unit_y() },
        );
    }
}

#[test]
fn probe_5889b_span() {
    // A unit circle tilted by theta about x; an arc of half-width w about
    // t = 0, where it crosses z = 0. Max |z| over the arc ~ sin(theta)*w.
    let e = eps();
    let theta = 1e3 * e; // full circle definitely off the plane
    let w = 1e-4;        // arc deviation ~ 1e3*e*1e-4 = 0.1 e: inside the band
    let carrier = Curve3::Circle {
        center: Point3::origin(),
        axis: Vec3::new(0.0, -theta.sin(), theta.cos()),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    let plane = Surface::Plane { origin: Point3::origin(), normal: Vec3::unit_z(), u_ref: Vec3::unit_x() };
    let img = geom_brep::chart_pcurve_over(&carrier, -w, w, &plane, band());
    let cert = img.clone().map(|p| geom_brep::PcurveCache::certify(p, -w, w, &carrier, &plane, band()).map(|_| ()));
    println!("SPAN eps={e:e} plane arc: mint={:?}\n   cert={cert:?}", img.map(|_| ()));
    // the same on a cylinder: a rim circle tilted by theta about x, at height 0,
    // arc near t=0 where it crosses the rim plane; distance from the wall ~ (z-tilt) small.
    let cyl = Surface::Cylinder { origin: Point3::origin(), axis: Vec3::unit_z(), radius: 1.0, u_ref: Vec3::unit_x() };
    let theta2 = (2e3 * e).sqrt(); // radial miss of the tilted rim ~ theta^2/2 * sin^2 t
    let rim = Curve3::Circle {
        center: Point3::origin(),
        axis: Vec3::new(0.0, -theta2.sin(), theta2.cos()),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    let img = geom_brep::chart_pcurve_over(&rim, -w, w, &cyl, band());
    let cert = img.clone().map(|p| geom_brep::PcurveCache::certify(p, -w, w, &rim, &cyl, band()).map(|_| ()));
    println!("SPAN eps={e:e} cylinder arc: mint={:?}\n   cert={cert:?}", img.map(|_| ()));
}
