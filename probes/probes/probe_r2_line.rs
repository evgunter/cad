//! R2 review probe (PR #4135): the line × cone root door
//! (`line_cone_roots`) against an independent oracle — the double cone's
//! signed residual `ρ cos α − |h| sin α` read from each point along the
//! line, sampled densely over a window round the span and bisected.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stdout)]

use super::*;
use crate::boolean::circle_roots::CircleRoots;
use geom_core::{Tol, Vec3};

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

fn res(apex: Point3<f64>, axis: Vec3<f64>, alpha: f64, p: Point3<f64>) -> f64 {
    let q = p - apex;
    let h = q.dot(axis);
    (q - axis * h).norm() * alpha.cos() - h.abs() * alpha.sin()
}

fn sign_changes(f: &impl Fn(f64) -> f64, t0: f64, t1: f64, n: u32) -> Vec<f64> {
    let at = |k: u32| t0 + (t1 - t0) * f64::from(k) / f64::from(n);
    let mut out = Vec::new();
    let mut prev = f(at(0));
    for k in 0..n {
        let next = f(at(k + 1));
        if (prev < 0.0) != (next < 0.0) {
            let (mut a, mut b) = (at(k), at(k + 1));
            let sa = f(a) < 0.0;
            for _ in 0..90 {
                let m = 0.5 * (a + b);
                if (f(m) < 0.0) == sa {
                    a = m;
                } else {
                    b = m;
                }
            }
            out.push(0.5 * (a + b));
        }
        prev = next;
    }
    out
}

#[derive(Default)]
struct Census {
    certified: usize,
    miss: usize,
    refused: usize,
    wrong: Vec<String>,
    max_err_over_eps: f64,
}

/// One pose: roots inside the window `[t0 − L, t1 + L]` must match the
/// oracle's one for one, within ε of arc length; a Miss must have none.
fn judge(label: &str, o: Point3<f64>, d: Vec3<f64>, cone: (Point3<f64>, Vec3<f64>, f64), span: (f64, f64), cen: &mut Census) {
    let eps = Tol::witness().eps();
    let band = Band::linear(Tol::witness()).unwrap();
    let (apex, axis, alpha) = cone;
    let ans = line_cone_roots(o, d, cone, span, band);
    let l = span.1 - span.0;
    let (w0, w1) = (span.0 - l, span.1 + l);
    let f = |t: f64| res(apex, axis, alpha, o + d * t);
    let truth = sign_changes(&f, w0, w1, 20_000);
    let speed = d.norm();
    match ans {
        Ok(CircleRoots::Certified { count, thetas }) => {
            cen.certified += 1;
            let got: Vec<f64> = thetas[..count].iter().copied().filter(|t| *t > w0 + l * 1e-3 && *t < w1 - l * 1e-3).collect();
            let truth_in: Vec<f64> = truth.iter().copied().filter(|t| *t > w0 + l * 1e-3 && *t < w1 - l * 1e-3).collect();
            if got.len() != truth_in.len() {
                cen.wrong.push(format!("{label}: roots in window {got:?} vs oracle {truth_in:?}"));
                return;
            }
            for g in &got {
                let e = truth_in.iter().map(|t| (t - g).abs()).fold(f64::INFINITY, f64::min) * speed;
                cen.max_err_over_eps = cen.max_err_over_eps.max(e / eps);
                // The door's contract: a root outside the span is placed no
                // nearer it than the band; inside, within the band.
                let outside = speed * (span.0 - g).max(g - span.1).max(0.0);
                if e > eps + outside {
                    cen.wrong.push(format!("{label}: root {g} is {e:e} m from the oracle's"));
                }
            }
        }
        Ok(CircleRoots::Miss) => {
            cen.miss += 1;
            if !truth.is_empty() {
                cen.wrong.push(format!("{label}: Miss but oracle crossings {truth:?}"));
            }
        }
        _ => cen.refused += 1,
    }
}

const ALPHAS: [f64; 6] = [0.002, 0.05, 0.6, 1.0, 1.5, 1.568];
const SCALES: [f64; 3] = [1e-3, 1.0, 1e3];

#[test]
fn probe_line_random_and_generators() {
    let eps = Tol::witness().eps();
    let mut rng = Rng(0x5151_aaaa_0f0f_1234);
    let mut cen = Census::default();
    let mut near_gen = Census::default();
    let mut through_axis = Census::default();
    for i in 0..4000 {
        let alpha = ALPHAS[i % ALPHAS.len()];
        let sc = SCALES[(i / ALPHAS.len()) % 3];
        let axis = rng.unit();
        let apex = Point3::from_array({let v = Vec3::new(rng.r(-1.0, 1.0), rng.r(-1.0, 1.0), rng.r(-1.0, 1.0)) * sc; [v.x, v.y, v.z]});
        let cone = (apex, axis, alpha);
        let (u, v) = axis.orthonormal_basis();
        // 1: random line near the cone.
        let o = apex + rng.unit() * (sc * rng.r(0.1, 2.0));
        let d = rng.unit() * rng.r(0.5, 2.0);
        judge(&format!("rand{i}"), o, d, cone, (0.0, sc), &mut cen);
        // 2: near a generator: the generator through `apex + g·s`, tilted by δ,
        // offset by `off` from it.
        let phi = rng.r(0.0, 6.283);
        let radial = u * phi.cos() + v * phi.sin();
        let g = (axis * alpha.cos() + radial * alpha.sin()).normalize();
        let delta = 10f64.powf(rng.r(-14.0, -1.0));
        let other = radial.cross(axis);
        let dg = (g + other * delta * rng.r(-1.0, 1.0) + radial * delta * rng.r(-1.0, 1.0)).normalize();
        let off = sc * 10f64.powf(rng.r(-12.0, -1.0)) * if rng.f() < 0.5 { 1.0 } else { -1.0 };
        let o2 = apex + g * (0.5 * sc) + (radial * alpha.cos() - axis * alpha.sin()) * off;
        judge(&format!("gen{i} δ{delta:e} off{off:e}"), o2, dg, cone, (0.0, sc), &mut near_gen);
        // 3: a line through the axis at height H, crossing it.
        let hh = sc * rng.r(0.2, 1.5) * if rng.f() < 0.5 { 1.0 } else { -1.0 };
        let dir = (radial + axis * rng.r(-0.3, 0.3)).normalize();
        let o3 = apex + axis * hh - dir * (0.5 * sc * (1.0 + hh.abs() * alpha.tan().min(10.0) / sc));
        let len = sc * (1.0 + 2.0 * hh.abs() * alpha.tan().min(10.0) / sc);
        judge(&format!("axis{i}"), o3, dir, cone, (0.0, len), &mut through_axis);
    }
    for (name, c) in [("random", &cen), ("near-generator", &near_gen), ("through-axis", &through_axis)] {
        println!(
            "PROBE line {name} eps={eps:e}: certified {} miss {} refused {} max err/eps {:e} WRONG {}",
            c.certified, c.miss, c.refused, c.max_err_over_eps, c.wrong.len()
        );
        for w in c.wrong.iter().take(15) {
            println!("  WRONG {w}");
        }
    }
}

/// The apex edge: a steep line passing δ from the apex, both nappes.
#[test]
fn probe_line_apex_edge() {
    let eps = Tol::witness().eps();
    let mut cen = Census::default();
    for &alpha in &[0.01, 0.785, 1.5] {
        for &sc in &SCALES {
            let apex = Point3::from_array({let v = Vec3::new(0.2, 0.1, -0.3) * sc; [v.x, v.y, v.z]});
            let axis = Vec3::new(0.0, 0.6, 0.8);
            let (u, _) = axis.orthonormal_basis();
            let mut nearest = f64::INFINITY;
            for p in 0..50 {
                let delta = sc * 10f64.powf(-15.0 + 0.3 * f64::from(p));
                let o = apex + u * delta - axis * sc;
                let before = cen.certified + cen.miss;
                judge(&format!("apex α{alpha} s{sc} δ{delta:e}"), o, axis, (apex, axis, alpha), (0.0, 2.0 * sc), &mut cen);
                if cen.certified + cen.miss > before {
                    nearest = nearest.min(delta);
                }
            }
            println!("LINE-APEX-EDGE α{alpha} s{sc}: nearest answered δ = {:e} = {:e} bands", nearest, nearest / eps);
        }
    }
    println!("PROBE line apex: certified {} miss {} refused {} max err/eps {:e} WRONG {}", cen.certified, cen.miss, cen.refused, cen.max_err_over_eps, cen.wrong.len());
    for w in cen.wrong.iter().take(15) {
        println!("  WRONG {w}");
    }
}
