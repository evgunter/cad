//! PROBE (temporary)
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::tol::band;
use geom::Surface;
use geom_brep::Reach;
use geom_brep::intersect::plane_cylinder_section;
use geom_core::{Point3, Vec3};

fn cyl_z(r: f64) -> Surface<f64> {
    Surface::Cylinder {
        frame: geom::Frame::new(Point3::new(1.0, 2.0, 3.0), Vec3::unit_z(), Vec3::unit_x()),
        radius: r,
    }
}

fn plane(n: Vec3<f64>, u: Vec3<f64>) -> Surface<f64> {
    Surface::Plane {
        frame: geom::Frame::new(Point3::new(0.0, 0.0, 7.0), n, u),
    }
}

#[test]
fn probe_plane_cylinder() {
    for r in [-1.0, 0.0, f64::NAN, f64::INFINITY, 1e-300] {
        let cyl = cyl_z(r);
        let reach = Reach::fixture(Point3::new(1.0, 2.0, 3.0), 1.0);
        for (name, p) in [
            ("rim", plane(Vec3::unit_z(), Vec3::unit_x())),
            ("tilt", plane(Vec3::new(0.6, 0.0, 0.8), Vec3::new(0.8, 0.0, -0.6))),
            ("par", plane(Vec3::unit_x(), Vec3::unit_y())),
        ] {
            let got = std::panic::catch_unwind(|| plane_cylinder_section(&p, &cyl, &reach, band()));
            eprintln!("r={r} {name}: {}", match got { Ok(Ok(s)) => format!("Ok {s:?}").chars().take(90).collect::<String>(), Ok(Err(e)) => format!("Err {e:?}").chars().take(120).collect(), Err(_) => "PANIC".into() });
        }
    }
}
