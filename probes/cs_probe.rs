//! Probe (review of PR 3847, lane reach-dual3847-r2): reads poses from
//! $CS_PROBE_IN (hex f64 words), runs `circle_sphere_roots` at the given
//! band, writes the verdict and the harmonic's fields to $CS_PROBE_OUT.
//! Included into `circle_sphere.rs` via `#[path]` for the run only.
use super::*;
use geom_core::{Point3, Vec3};
use std::io::Write;

fn f(s: &str) -> f64 {
    f64::from_bits(u64::from_str_radix(s, 16).unwrap())
}

#[test]
fn cs_probe_run() {
    let Ok(inp) = std::env::var("CS_PROBE_IN") else { return };
    let out = std::env::var("CS_PROBE_OUT").unwrap();
    let mut w = std::io::BufWriter::new(std::fs::File::create(out).unwrap());
    for line in std::fs::read_to_string(inp).unwrap().lines() {
        let x: Vec<f64> = line.split_whitespace().map(f).collect();
        let (c, n, u, rho, s, r, t0, t1, eps) = (
            Point3::new(x[0], x[1], x[2]),
            Vec3::new(x[3], x[4], x[5]),
            Vec3::new(x[6], x[7], x[8]),
            x[9],
            Point3::new(x[10], x[11], x[12]),
            x[13],
            x[14],
            x[15],
            x[16],
        );
        let carrier = geom::Curve3::Circle { center: c, axis: n, radius: rho, u_ref: u };
        let sphere = geom::Surface::Sphere {
            center: s,
            radius: r,
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let band = Band::new(eps, 10.0 * eps).unwrap();
        let got = circle_sphere_roots(&carrier, t0, t1, &sphere, band);
        let h = geom_brep::circle_sphere_harmonic(c, n, rho, u, s, r);
        let noise = rounding_charge(h.terms) / (2.0 * r);
        let verdict = match got {
            Ok(CircleRoots::Certified { count, thetas }) => {
                format!("C{count} {:x} {:x}", thetas[0].to_bits(), thetas[1].to_bits())
            }
            Ok(CircleRoots::Miss) => "M".into(),
            Ok(CircleRoots::Uncertain) => "U".into(),
            Ok(other) => format!("O {other:?}").replace(' ', "_"),
            Err(_) => "E".into(),
        };
        writeln!(
            w,
            "{verdict} {:e} {:e} {:e} {:e} {:e} {:e}",
            h.c0 - h.a1, h.c0 + h.a1, noise, h.lo, h.lo_error, h.hi_error
        )
        .unwrap();
    }
}
