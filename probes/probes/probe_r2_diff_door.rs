//! R2 review probe (PR #4135): cross-tree differential of the conic ×
//! quadric door on spheres and walls. The same file runs on the merge
//! base and on the head; each writes its answers to `R2_DIFF_OUT`, and
//! the two files are compared byte for byte.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stdout)]

use super::*;
use geom_core::{Point3, Vec3};
use std::fmt::Write as _;

struct Rng(u64);
impl Rng {
    fn f(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    fn r(&mut self, a: f64, b: f64) -> f64 {
        a + (b - a) * self.f()
    }
    fn unit(&mut self) -> Vec3<f64> {
        loop {
            let v = Vec3::new(self.r(-1.0, 1.0), self.r(-1.0, 1.0), self.r(-1.0, 1.0));
            let n = v.norm();
            if n > 0.2 && n < 1.0 {
                return v / n;
            }
        }
    }
}

#[test]
fn probe_r2_diff_door() {
    let mut out = String::new();
    let mut rng = Rng(0x7777_1234_abcd_0001);
    for eps in [1e-6, 1e-9, 1e-12] {
        let band = Band::new(eps, 10.0 * eps).unwrap();
        for i in 0..3000 {
            let sc = [1e-3, 1.0, 1e3][i % 3];
            let off = if i % 5 == 0 { 1e4 } else { 0.0 };
            let n = rng.unit();
            let w = rng.unit();
            let uref = (w - n * w.dot(n)).normalize();
            let a = rng.r(0.1, 1.0) * sc;
            let c = Point3::new(off + rng.r(-1.0, 1.0) * sc, rng.r(-1.0, 1.0) * sc, rng.r(-1.0, 1.0) * sc);
            let e = if i % 3 == 0 {
                geom::Curve3::Circle { center: c, axis: n, radius: a, u_ref: uref }
            } else {
                geom::Curve3::Ellipse { center: c, axis: n, major: a, minor: a / rng.r(1.0, 6.0), u_ref: uref }
            };
            let through = e.eval(rng.r(0.0, 6.28));
            let r = rng.r(0.05, 2.0) * sc;
            let ax = rng.unit();
            let x = Vec3::new(1.0, 0.0, 0.0);
            // Graze some: move the surface so the point is within a few bands.
            let nudge = if i % 4 == 0 { rng.r(-5.0, 5.0) * eps } else { 0.0 };
            let s = if i % 2 == 0 {
                geom::Surface::Cylinder {
                    origin: through + rng.unit().cross(ax).normalize() * (r + nudge),
                    axis: ax,
                    radius: r,
                    u_ref: (x - ax * x.dot(ax)).normalize(),
                }
            } else {
                geom::Surface::Sphere {
                    center: through + rng.unit() * (r + nudge),
                    radius: r,
                    axis: ax,
                    u_ref: (x - ax * x.dot(ax)).normalize(),
                }
            };
            let t0 = rng.r(0.0, 6.28);
            let t1 = t0 + rng.r(0.1, 6.28);
            let got = conic_quadric_roots(&e, t0, t1, &s, band);
            writeln!(out, "{eps} {i} {got:?}").unwrap();
        }
    }
    std::fs::write(std::env::var("R2_DIFF_OUT").unwrap(), out).unwrap();
}
