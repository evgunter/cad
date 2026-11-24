//! R2 probe: kappa_rel one-signed at every interior station but zero between them.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, missing_docs)]
use geom::{Curve3, Frame, Surface};
use geom_core::{Band, Point3, Tol, Vec3};
use std::f64::consts::PI;

fn pair() -> (Surface<f64>, Surface<f64>, Curve3<f64>) {
    let torus = Surface::Torus {
        frame: Frame::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0)),
        major_radius: 3.0,
        minor_radius: 1.0,
    };
    let cyl = Surface::Cylinder {
        frame: Frame::new(Point3::new(3.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(1.0, 0.0, 0.0)),
        radius: 1.0,
    };
    let rim = Curve3::circle(Point3::new(3.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), 1.0, Vec3::new(1.0, 0.0, 0.0));
    (torus, cyl, rim)
}

#[test]
fn r2_between_station_zeros() {
    let band = Band::linear(Tol::witness()).unwrap();
    let (a, b, rim) = pair();
    for (name, t0, t1) in [
        ("two zeros in the end gaps", PI / 2.0 - 0.3, 1.5 * PI + 0.3),
        ("one zero in the start gap", PI / 2.0 - 0.3, PI),
        ("whole turn offset 0.1", 0.1, 2.0 * PI + 0.1),
        ("half, no zero", PI / 2.0 + 0.1, 1.5 * PI - 0.1),
    ] {
        for (s1, s2) in [(&a, &b), (&b, &a)] {
            let v = geom_brep::must_carry_over_edge(s1, s2, &rim, t0, t1, geom_core::test_support::fixture_lever(1.0), band);
            let mut kmin = f64::MAX;
            let mut signs = Vec::new();
            for i in 1..geom_brep::CERT_SAMPLES - 1 {
                let t = geom_brep::sample_param(t0, t1, i);
                let (p, d) = rim.ders1(t);
                let j = geom_brep::tangent_jet(s1, s2, p, d);
                kmin = kmin.min(j.kappa_rel.abs());
                signs.push(j.kappa_rel.signum() as i32);
            }
            let mut fine = f64::MAX;
            for k in 0..=10000 {
                let t = t0 + (t1 - t0) * f64::from(k) / 10000.0;
                let (p, d) = rim.ders1(t);
                fine = fine.min(geom_brep::tangent_jet(s1, s2, p, d).kappa_rel.abs());
            }
            let drift = geom_brep::tangent_span_bounds(s1, s2, &rim, t0, t1).map(|b| b.kappa_drift);
            eprintln!("R2 {name}: verdict {v:?} station-signs {signs:?} station-min|k| {kmin:.4e} true-min|k| {fine:.3e} drift {drift:?}");
        }
    }
}

#[test]
fn r2_a_dome_on_the_elbows_end() {
    let band = Band::linear(Tol::witness()).unwrap();
    let (torus, _, rim) = pair();
    let ball = Surface::Sphere {
        frame: Frame::new(Point3::new(3.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0)),
        radius: 1.0,
    };
    let (t0, t1) = (0.1, 2.0 * PI + 0.1);
    let v = geom_brep::must_carry_over_edge(&torus, &ball, &rim, t0, t1, geom_core::test_support::fixture_lever(1.0), band);
    let mut kmin = f64::MAX;
    for i in 1..geom_brep::CERT_SAMPLES - 1 {
        let t = geom_brep::sample_param(t0, t1, i);
        let (p, d) = rim.ders1(t);
        kmin = kmin.min(geom_brep::tangent_jet(&torus, &ball, p, d).kappa_rel.abs());
    }
    let drift = geom_brep::tangent_span_bounds(&torus, &ball, &rim, t0, t1).map(|b| b.kappa_drift);
    eprintln!("R2 dome: verdict {v:?} station-min|k| {kmin:.4e} drift {drift:?}");
}
