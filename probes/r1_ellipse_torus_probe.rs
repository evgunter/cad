//! Reviewer r1 probe for PR #3973: drives `ellipse_roots` on an ellipse ×
//! torus pose generator independent of the PR's `torus_rows::pose`, and
//! dumps each pose and answer as one `R1ET` JSON line for
//! `probes/r1_oracle.py` (mpmath, exact f64 inputs). Mounted locally by
//! appending `#[cfg(test)] #[path = "../../../../probes/r1_ellipse_torus_probe.rs"]
//! mod r1_probe;` to `crates/topo/src/boolean/ellipse_roots.rs`.
#![allow(clippy::unwrap_used, clippy::panic)]
use super::*;
use core::f64::consts::{PI, TAU};
use geom_core::{Point3, Vec3};

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    fn range(&mut self, a: f64, b: f64) -> f64 {
        a + (b - a) * self.next()
    }
    fn unit(&mut self) -> Vec3<f64> {
        loop {
            let v = Vec3::new(self.range(-1., 1.), self.range(-1., 1.), self.range(-1., 1.));
            let n = v.norm();
            if n > 0.1 && n < 1.0 {
                return v / n;
            }
        }
    }
    fn perp(&mut self, a: Vec3<f64>) -> Vec3<f64> {
        let w = self.unit();
        (w - a * w.dot(a)).normalize()
    }
}

/// Rotation taking orthonormal (a0, b0, c0) to (a1, b1, c1), applied to x.
fn rot(x: Vec3<f64>, f0: [Vec3<f64>; 3], f1: [Vec3<f64>; 3]) -> Vec3<f64> {
    f1[0] * x.dot(f0[0]) + f1[1] * x.dot(f0[1]) + f1[2] * x.dot(f0[2])
}

/// A torus (hub, axis) with radii (big, small) whose surface point at tube
/// angle `v` has outward normal `nrm` and passes through `s`, with its
/// `u`-direction (beta=0) / `v`-direction mixed by `beta` along `tan`.
fn torus_at(s: Point3<f64>, nrm: Vec3<f64>, tan: Vec3<f64>, big: f64, small: f64, v: f64, beta: f64) -> (Point3<f64>, Vec3<f64>) {
    let (sv, cv) = v.sin_cos();
    let p0 = Vec3::new(big + small * cv, 0.0, small * sv);
    let n0 = Vec3::new(cv, 0.0, sv);
    let eu = Vec3::new(0.0, 1.0, 0.0);
    let ev = Vec3::new(-sv, 0.0, cv);
    let t0 = eu * beta.cos() + ev * beta.sin();
    let f0 = [n0, t0, n0.cross(t0)];
    let f1 = [nrm, tan, nrm.cross(tan)];
    let hub = s - rot(p0, f0, f1);
    (hub, rot(Vec3::new(0.0, 0.0, 1.0), f0, f1).normalize())
}

#[test]
#[ignore = "reviewer probe dump"]
fn r1_dump() {
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15 ^ std::env::var("R1SEED").map(|s| s.parse::<u64>().unwrap()).unwrap_or(1));
    let per = std::env::var("R1N").map(|s| s.parse::<usize>().unwrap()).unwrap_or(40);
    let fams = ["cross", "graze_outer", "graze_inner", "graze_any", "coaxial", "near_circle", "far_out"];
    for eps in [1e-6, 1e-9, 1e-12] {
        let band = Band::new(eps, 10.0 * eps).unwrap();
        for scale in [1e-3, 1.0, 1e3] {
            for (fi, fam) in fams.iter().enumerate() {
                for _ in 0..per {
                    let a = scale * rng.range(0.2, 2.0);
                    let b = if *fam == "near_circle" {
                        let rel = [1e-3, 1e-6, 30.0 * eps / a][rng.next().mul_add(3.0, 0.0) as usize % 3];
                        a * (1.0 - rel.max(30.0 * eps / a))
                    } else {
                        a * rng.range(0.05, 0.95)
                    };
                    let n = rng.unit();
                    let u = rng.perp(n);
                    let mut c = Point3::new(0.0, 0.0, 0.0);
                    if *fam == "far_out" {
                        c = c + rng.unit() * (1e3 * scale);
                    }
                    let e = geom::Curve3::Ellipse { center: c, axis: n, major: a, minor: b, u_ref: u };
                    let (hub, t_axis, big, small);
                    if *fam == "coaxial" {
                        // In the ellipse's plane (or raised), centred on its axis.
                        small = rng.range(0.01, 0.3) * b;
                        let kind = rng.next();
                        big = if kind < 0.4 {
                            rng.range(b, a) // crosses: core between the semi-axes
                        } else if kind < 0.7 {
                            a - small + eps * rng.range(-40.0, 40.0) // outer tube wall grazes the major vertices
                        } else {
                            b + small + eps * rng.range(-40.0, 40.0) // inner tube wall grazes the minor vertices
                        };
                        let lift = if rng.next() < 0.5 { 0.0 } else { small * rng.range(-0.9, 0.9) };
                        hub = c + n * lift;
                        t_axis = if rng.next() < 0.5 { n } else { -n };
                    } else {
                        let theta = rng.range(0.0, TAU);
                        let p = e.eval(theta);
                        let tan = e.deriv(theta).normalize();
                        let ratio = rng.range(1.2, 8.0);
                        let s_small = scale * rng.range(0.03, 0.6);
                        small = s_small;
                        big = s_small * ratio;
                        let nrm = rng.perp(tan);
                        let gap = match *fam {
                            "cross" | "far_out" => -small * rng.range(0.0, 1.5),
                            _ => eps * rng.range(-40.0, 40.0),
                        };
                        let v = match *fam {
                            "graze_outer" => 0.0,
                            "graze_inner" => PI,
                            _ => rng.range(0.0, TAU),
                        };
                        let beta = rng.range(0.0, TAU);
                        let surf = p - nrm * gap;
                        let (h, ax) = torus_at(surf, nrm, tan, big, small, v, beta);
                        hub = h;
                        t_axis = ax;
                    }
                    let x = Vec3::new(1.0, 0.0, 0.0);
                    let x = if t_axis.cross(x).norm() < 0.1 { Vec3::new(0.0, 1.0, 0.0) } else { x };
                    let s = geom::Surface::Torus { center: hub, axis: t_axis, major_radius: big, minor_radius: small, u_ref: (x - t_axis * x.dot(t_axis)).normalize() };
                    let t0 = rng.range(-TAU, TAU);
                    let t1 = t0 + rng.range(0.1, TAU);
                    let (ans, roots) = match ellipse_roots(&e, t0, t1, &s, band) {
                        Ok(CircleRoots::Certified { count, thetas }) => ("certified".to_string(), thetas[..count].to_vec()),
                        Ok(CircleRoots::Miss) => ("miss".to_string(), vec![]),
                        Ok(CircleRoots::Uncertain) => ("uncertain".to_string(), vec![]),
                        Ok(other) => (format!("other:{other:?}"), vec![]),
                        Err(err) => (format!("err:{err:?}").replace('"', "'"), vec![]),
                    };
                    let v = |p: Vec3<f64>| format!("[{:?},{:?},{:?}]", p.x, p.y, p.z);
                    let pt = |p: Point3<f64>| format!("[{:?},{:?},{:?}]", p.x, p.y, p.z);
                    println!(
                        "R1ET {{\"fam\":\"{fam}\",\"fi\":{fi},\"scale\":{scale:?},\"eps\":{eps:?},\"c\":{},\"n\":{},\"u\":{},\"a\":{a:?},\"b\":{b:?},\"hub\":{},\"ax\":{},\"R\":{big:?},\"r\":{small:?},\"t0\":{t0:?},\"t1\":{t1:?},\"ans\":\"{ans}\",\"roots\":{roots:?}}}",
                        pt(c), v(n), v(u), pt(hub), v(t_axis)
                    );
                }
            }
        }
    }
}
