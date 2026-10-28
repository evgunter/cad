//! Review probes for PR 5889 (E1c).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, missing_docs)]

use crate::shared::tol::{band, eps};
use geom::{Curve3, Surface};
use geom_brep::{IncidenceRead, chart_pcurve};
use geom_brep::recourse::Reading;
use geom_core::{Interval, Point3, Vec3};

fn both(what: &str, carrier: &Curve3<f64>, surface: &Surface<f64>) {
    let lift = <Interval as geom_core::Real>::from_f64;
    let f = chart_pcurve(carrier, surface, band()).map(|_| ());
    let cert = chart_pcurve(carrier, surface, band()).and_then(|img| geom_brep::PcurveCache::certify(img, 0.0, 6.0, carrier, surface, band()).map(|_| ()));
    let (ci, si) = (carrier.map_scalar(lift), surface.map_scalar(lift));
    let certi = chart_pcurve(&ci, &si, band()).and_then(|img| geom_brep::PcurveCache::certify(img, lift(0.0), lift(6.0), &ci, &si, band()).map(|_| ()));
    eprintln!("CERTI eps={:e} {what} [Interval full certify 0..6]: {:?}", eps(), certi.as_ref().err().map(|e| format!("{e}").chars().take(140).collect::<String>()));
    eprintln!("CERT eps={:e} {what} [f64 full certify 0..6]: {:?}", eps(), cert.as_ref().err().map(|e| format!("{e}").chars().take(140).collect::<String>()));
    let i = chart_pcurve(&carrier.map_scalar(lift), &surface.map_scalar(lift), band()).map(|_| ());
    for (lane, got) in [("f64", f), ("Interval", i)] {
        let read = got.as_ref().err().and_then(IncidenceRead::of);
        let end = read.map(|r| r.ending(Reading::Build));
        eprintln!(
            "PROBE eps={:e} {what} [{lane}]: {:?} | read={:?} definite={:?} | ending={:?}",
            eps(),
            got.as_ref().err().map(|e| format!("{e}").chars().take(160).collect::<String>()),
            read.map(|r| format!("{r:?}").chars().take(60).collect::<String>()),
            read.map(|r| r.is_definite()),
            end.map(|s| s.chars().take(90).collect::<String>()),
        );
    }
}

#[test]
fn probe_far_out_and_far_anchor() {
    let far = Point3::new(1e7, 1e7, 1e7);
    // A genuine 0.5 m miss far out: wrong-radius rim 1e7 m out.
    let cyl = Surface::Cylinder { origin: far, axis: Vec3::unit_z(), radius: 1.0, u_ref: Vec3::unit_x() };
    both("rim r+0.5 1e7 out", &Curve3::Circle { center: far, axis: Vec3::unit_z(), radius: 1.5, u_ref: Vec3::unit_x() }, &cyl);
    // Sphere polar circle 0.5 off, far out.
    let sph = Surface::Sphere { center: far, radius: 5.0, axis: Vec3::unit_z(), u_ref: Vec3::unit_x() };
    both("sphere polar r+0.5 1e7 out", &Curve3::Circle { center: far + Vec3::new(0.0, 0.0, 3.0), axis: Vec3::unit_z(), radius: 4.5, u_ref: Vec3::unit_x() }, &sph);
    // Chart anchor far from the carrier: a plane through the origin with a
    // tilted normal, a circle on it d out.
    let n = Vec3::new(0.0, 0.6, 0.8);
    let u = Vec3::unit_x();
    let v = n.cross(u);
    let plane = Surface::Plane { origin: Point3::origin(), normal: n, u_ref: u };
    for d in [1024.0, 16384.0, 131072.0, 8388608.0] {
        let c = Point3::origin() + u * d + v * d;
        both(&format!("plane circle on, anchor {d:e} away"), &Curve3::Circle { center: c, axis: n, radius: 1.0, u_ref: u }, &plane);
        both(&format!("plane circle 0.5 off, anchor {d:e} away"), &Curve3::Circle { center: c + n * 0.5, axis: n, radius: 1.0, u_ref: u }, &plane);
    }
    // Cylinder whose origin is far down its tilted axis from the rim.
    let ax = Vec3::new(0.0, 0.6, 0.8);
    let cyl2 = Surface::Cylinder { origin: Point3::origin(), axis: ax, radius: 1.0, u_ref: Vec3::unit_x() };
    for h in [1024.0, 16384.0, 131072.0, 8388608.0] {
        both(&format!("tilted cyl rim h={h:e} up its axis"), &Curve3::Circle { center: Point3::origin() + ax * h, axis: ax, radius: 1.0, u_ref: Vec3::unit_x() }, &cyl2);
    }
    // Torus / cone far anchors: cone rim far up a tilted axis.
    let cone = Surface::Cone { apex: Point3::origin(), axis: ax, half_angle: 0.25, u_ref: Vec3::unit_x() };
    for h in [1024.0, 131072.0] {
        both(&format!("tilted cone rim h={h:e}"), &Curve3::Circle { center: Point3::origin() + ax * h, axis: ax, radius: h * 0.25f64.tan(), u_ref: Vec3::unit_x() }, &cone);
    }
    // Non-unit axis renormalised: axis 2x long, rim on.
    let cyl3 = Surface::Cylinder { origin: Point3::origin(), axis: Vec3::new(0.0, 0.0, 2.0), radius: 1.0, u_ref: Vec3::unit_x() };
    both("axis 2x long, rim on at z=3", &Curve3::Circle { center: Point3::new(0.0, 0.0, 3.0), axis: Vec3::unit_z(), radius: 1.0, u_ref: Vec3::unit_x() }, &cyl3);
    both("axis 2x long, rim r+0.5", &Curve3::Circle { center: Point3::new(0.0, 0.0, 3.0), axis: Vec3::unit_z(), radius: 1.5, u_ref: Vec3::unit_x() }, &cyl3);
    // Tiny ellipse within eps on the plane & a mid-band wrong-radius on the plane.
    let e = eps();
    both("plane circle 0.5e off", &Curve3::Circle { center: Point3::new(0.0, 0.0, 0.5 * e), axis: Vec3::unit_z(), radius: 1.0, u_ref: u }, &Surface::Plane { origin: Point3::origin(), normal: Vec3::unit_z(), u_ref: u });
    both("plane circle 1.0e off", &Curve3::Circle { center: Point3::new(0.0, 0.0, 1.0 * e), axis: Vec3::unit_z(), radius: 1.0, u_ref: u }, &Surface::Plane { origin: Point3::origin(), normal: Vec3::unit_z(), u_ref: u });
    both("plane circle 3e off", &Curve3::Circle { center: Point3::new(0.0, 0.0, 3.0 * e), axis: Vec3::unit_z(), radius: 1.0, u_ref: u }, &Surface::Plane { origin: Point3::origin(), normal: Vec3::unit_z(), u_ref: u });
}
