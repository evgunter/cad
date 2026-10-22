//! Review 5590: what pncad-py's `Body.mass_properties()` now pays on a
//! curved body, against the one-quadrature spelling.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use pncad::authoring::polygon;
use pncad::geom_core::linalg::frame::path_start_frame;
use pncad::geom_core::{Point3, RigidMotion, Tol, UnitVec3};
use pncad::profile::SketchPlane;
use pncad::topo::{AtRestBody, Body};
use std::time::Instant;

fn main() {
    let tol = Tol::witness();
    let pts: Vec<Point3<f64>> = (0..=32)
        .map(|k| {
            let s = f64::from(k) / 32.0;
            let a = std::f64::consts::TAU * 0.4 * s;
            Point3::new(a.cos(), a.sin(), 0.6 * s)
        })
        .collect();
    let path = pncad::geom::NurbsCurve3::interpolate(&pts, 3).expect("path");
    let (t0, t1) = path.domain();
    let section =
        vec![polygon(&[(-0.1, -0.1), (0.1, -0.1), (0.1, 0.1), (-0.1, 0.1)], tol).expect("square")];
    let stations: u32 = 6;
    let places: Vec<SketchPlane<f64>> = (0..stations)
        .map(|i| {
            let u = f64::from(i) / f64::from(stations - 1);
            let t = (t1 - t0).mul_add(u, t0);
            let (p, d) = path.ders1(t);
            let frame = path_start_frame(p, d, tol).expect("frame");
            let roll = RigidMotion::rotation(UnitVec3::exact_z(), 0.8 * u).expect("roll");
            SketchPlane::from_frame(frame.compose(roll).expect("rolled"))
        })
        .collect();
    let sections: Vec<_> = (0..stations).map(|_| section.clone()).collect();
    let body: Body<f64> = pncad::sweep::loft_body::<f64>(&sections, &places, 3, tol)
        .expect("loft")
        .body;
    let n = 5;
    let held = AtRestBody::validate(Body::clone(&body), tol).expect("finishes");
    let time = |label: &str, f: &dyn Fn() -> (f64, f64)| {
        let start = Instant::now();
        let mut v = (0.0, 0.0);
        for _ in 0..n {
            v = f();
        }
        let per = start.elapsed().as_secs_f64() / f64::from(n);
        println!("{label}: {per:.4} s/call  volume {:e} pad {:e}", v.0, v.1);
        per
    };
    let new_py = time("new Body.mass_properties (validate + door)", &|| {
        let b = AtRestBody::validate(Body::clone(&body), tol).unwrap();
        let m = pncad::topo::mass_properties(&b, tol).unwrap();
        (m.volume, m.volume_pad)
    });
    let one = time(
        "validate_geometric_measured (certificate + refine)",
        &|| {
            let c = pncad::topo::validate_geometric_certificate(&body, tol).unwrap();
            let m = c.measure().unwrap();
            (m.volume, m.volume_pad)
        },
    );
    let old_py = time("old Body.mass_properties (door alone)", &|| {
        let m = pncad::topo::mass_properties(&held, tol).unwrap();
        (m.volume, m.volume_pad)
    });
    let gate = time("validate alone", &|| {
        let _ = AtRestBody::validate(Body::clone(&body), tol).unwrap();
        (0.0, 0.0)
    });
    println!(
        "ratios: new/old {:.2}, new/one-quadrature {:.2}, gate share {:.2}",
        new_py / old_py,
        new_py / one,
        gate / new_py
    );
}
