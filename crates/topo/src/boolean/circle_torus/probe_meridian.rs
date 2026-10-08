//! REVIEW PROBE (PR 4343 review, not for merge): the door on fixtures
//! read from `PROBE_IN`, one per line, its answer written per line.
#![allow(clippy::unwrap_used, clippy::panic, clippy::float_cmp, clippy::print_stderr, clippy::print_stdout)]
use super::*;
use geom_core::{Point3, Tol, Vec3};

#[test]
#[ignore = "review probe"]
fn probe_meridian_fixtures() {
    let path = std::env::var("PROBE_IN").unwrap();
    let out = std::env::var("PROBE_OUT").unwrap();
    let text = std::fs::read_to_string(path).unwrap();
    let mut lines = Vec::new();
    for line in text.lines() {
        let f: Vec<f64> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let [eps, cx, cy, cz, nx, ny, nz, rho, ux, uy, uz, tx, ty, tz, ax, ay, az, big, small] = f[..] else { panic!() };
        let band = Band::linear_at(Tol::witness(), eps).unwrap();
        let carrier = geom::Curve3::Circle {
            center: Point3::new(cx, cy, cz),
            axis: Vec3::new(nx, ny, nz),
            radius: rho,
            u_ref: Vec3::new(ux, uy, uz),
        };
        let a = Vec3::new(ax, ay, az);
        let k = if a.x.abs() < 0.9 { Vec3::new(1.0, 0.0, 0.0) } else { Vec3::new(0.0, 1.0, 0.0) };
        let torus = geom::Surface::Torus {
            center: Point3::new(tx, ty, tz),
            axis: a,
            major_radius: big,
            minor_radius: small,
            u_ref: a.cross(k).normalize(),
        };
        let w0 = Point3::new(cx, cy, cz) - Point3::new(tx, ty, tz);
        let h0 = w0.dot(a);
        let wp = w0 - a * h0;
        let dev = meridian_deviation(Vec3::new(nx, ny, nz), rho, wp, wp.norm(), h0, a, big, small);
        let got = circle_torus_roots(&carrier, 0.3, 2.1, &torus, band);
        let tag = match got {
            Ok(CircleRoots::OnSurface) => "OnSurface".to_string(),
            Ok(CircleRoots::Uncertain) => "Uncertain".to_string(),
            Ok(CircleRoots::Miss) => "Miss".to_string(),
            Ok(CircleRoots::Certified { count, .. }) => format!("Certified{count}"),
            Err(BooleanError::Escalated { diag, .. }) => format!("Esc:{}", diag.predicate.unwrap_or("?")),
            Err(e) => format!("Err:{e:?}").replace(' ', "_"),
            Ok(o) => format!("{o:?}").replace(' ', "_"),
        };
        lines.push(format!("{tag} {dev:e}"));
    }
    std::fs::write(out, lines.join("\n")).unwrap();
}
