//! Probe (review of PR 3847): circle x cylinder (square arm and ladder)
//! and circle x torus verdicts, bits printed, for a main/head diff.
//! Included into `circle_cylinder.rs` via `#[path]` for the run only.
use super::*;
use crate::boolean::circle_roots::CircleRoots;
use geom_core::{Point3, Vec3};
use std::io::Write;

fn f(s: &str) -> f64 {
    f64::from_bits(u64::from_str_radix(s, 16).unwrap())
}

fn show(r: Result<CircleRoots<f64>, BooleanError>) -> String {
    match r {
        Ok(CircleRoots::Certified { count, thetas }) => format!(
            "C{count} {}",
            thetas[..count].iter().map(|t| format!("{:x}", t.to_bits())).collect::<Vec<_>>().join(" ")
        ),
        Ok(CircleRoots::Miss) => "M".into(),
        Ok(CircleRoots::Uncertain) => "U".into(),
        Ok(CircleRoots::OnSurface) => "O".into(),
        Ok(CircleRoots::CountDisagrees) => "D".into(),
        Err(_) => "E".into(),
    }
}

#[test]
fn cyl_probe_run() {
    let Ok(inp) = std::env::var("CYL_PROBE_IN") else { return };
    let out = std::env::var("CYL_PROBE_OUT").unwrap();
    let mut w = std::io::BufWriter::new(std::fs::File::create(out).unwrap());
    for line in std::fs::read_to_string(inp).unwrap().lines() {
        let x: Vec<f64> = line.split_whitespace().map(f).collect();
        let carrier = geom::Curve3::Circle {
            center: Point3::new(x[0], x[1], x[2]),
            axis: Vec3::new(x[3], x[4], x[5]),
            radius: x[9],
            u_ref: Vec3::new(x[6], x[7], x[8]),
        };
        let band = Band::new(x[16], 10.0 * x[16]).unwrap();
        let o = Point3::new(x[10], x[11], x[12]);
        let ax = Vec3::new(x[17], x[18], x[19]);
        let rx = Vec3::new(x[20], x[21], x[22]);
        let wall = geom::Surface::Cylinder { origin: o, axis: ax, radius: x[13], u_ref: rx };
        let torus = geom::Surface::Torus { center: o, axis: ax, major_radius: x[13] + x[9] * 0.5, minor_radius: x[13] * 0.5, u_ref: rx };
        let a = show(circle_cylinder_roots(&carrier, x[14], x[15], &wall, band));
        let b = show(crate::boolean::circle_torus::circle_torus_roots(&carrier, x[14], x[15], &torus, band));
        writeln!(w, "{a} | {b}").unwrap();
    }
}
