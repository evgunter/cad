//! Reviewer probe (reach-dual4135-r1): `reduce::line_cone_roots` against
//! an independent oracle (the double cone's residual and distance read
//! from the point; dense sampling + bisection). Mounted temporarily as a
//! `#[cfg(test)]` child of `topo::boolean::reduce` via `#[path]`.
#![allow(clippy::unwrap_used, clippy::panic, clippy::cast_precision_loss)]
use super::*;
use crate::boolean::circle_roots::CircleRoots;
use core::f64::consts::{PI, TAU};
use geom_core::{Tol, Vec3};

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
    fn pick<T: Copy>(&mut self, xs: &[T]) -> T {
        xs[(self.next() * xs.len() as f64) as usize % xs.len()]
    }
    fn unit(&mut self) -> Vec3<f64> {
        loop {
            let v = Vec3::new(self.range(-1., 1.), self.range(-1., 1.), self.range(-1., 1.));
            let n = v.norm();
            if n > 0.2 && n < 1.0 {
                return v / n;
            }
        }
    }
}

fn res(apex: Point3<f64>, axis: Vec3<f64>, a: f64, p: Point3<f64>) -> f64 {
    let q = p - apex;
    let h = q.dot(axis);
    (q - axis * h).norm() * a.cos() - h.abs() * a.sin()
}
fn dist(apex: Point3<f64>, axis: Vec3<f64>, a: f64, p: Point3<f64>) -> f64 {
    let q = p - apex;
    let h = q.dot(axis);
    let rho = (q - axis * h).norm();
    let (sa, ca) = a.sin_cos();
    [ca, -ca]
        .into_iter()
        .map(|c| if rho * sa + h * c <= 0.0 { rho.hypot(h) } else { (rho * c - h * sa).abs() })
        .fold(f64::INFINITY, f64::min)
}

fn changes(f: &dyn Fn(f64) -> f64, t0: f64, t1: f64, n: usize) -> Vec<f64> {
    let at = |k: usize| t0 + (t1 - t0) * k as f64 / n as f64;
    let mut out = Vec::new();
    for k in 0..n {
        let (mut lo, mut hi) = (at(k), at(k + 1));
        let flo = f(lo);
        if (flo < 0.0) == (f(hi) < 0.0) {
            continue;
        }
        for _ in 0..90 {
            let m = 0.5 * (lo + hi);
            if (f(m) < 0.0) == (flo < 0.0) {
                lo = m;
            } else {
                hi = m;
            }
        }
        out.push(0.5 * (lo + hi));
    }
    out
}

#[test]
fn r1_line_cone_roots_against_the_oracle() {
    let band = Band::linear(Tol::witness()).unwrap();
    let eps = band.zero();
    let mut rng = Rng(0x2545_f491_4f6c_dd1d ^ eps.to_bits());
    let alphas = [0.003, 0.01, 0.05, 0.3, 0.785, 1.2, 1.5, 1.56, 1.567];
    for class in 0..4 {
        for scale in [1e-3, 1.0, 1e3] {
            let (mut cert, mut miss, mut apex_n, mut unc, mut err) = (0, 0, 0, 0, 0);
            let mut wrong: Vec<String> = Vec::new();
            let mut worst = 0.0f64;
            for i in 0..2000 {
                let a: f64 = rng.pick(&alphas);
                let axis = rng.unit();
                let apex = Point3::new(rng.range(-1., 1.) * scale, rng.range(-1., 1.) * scale, rng.range(-1., 1.) * scale);
                let (p, q) = axis.orthonormal_basis();
                let nappe = if rng.next() < 0.5 { 1.0 } else { -1.0 };
                let phi = rng.range(0., TAU);
                let radial = p * phi.cos() + q * phi.sin();
                let genr = axis * (nappe * a.cos()) + radial * a.sin();
                let normal = radial * a.cos() - axis * (nappe * a.sin());
                let (origin, dir, label) = match class {
                    // generic: through a point near the cone, random dir
                    0 => {
                        let s = rng.range(0.2, 2.0) * scale;
                        let o = apex + genr * s + rng.unit() * (rng.range(0.0, 0.5) * scale);
                        (o, rng.unit() * rng.range(0.5, 2.0), format!("generic a={a}"))
                    }
                    // near a generator: tilted off it by `tilt`, moved off the cone by `off`
                    1 => {
                        let s = rng.range(0.3, 2.0) * scale;
                        let tilt = rng.pick(&[0.0, 1e-13, 1e-11, 1e-9, 1e-7, 1e-5, 1e-3]);
                        let off = rng.pick(&[-30.0, -3.0, -1.0, 0.0, 1.0, 3.0, 30.0, 1e4]) * eps;
                        let side = if rng.next() < 0.5 { normal } else { axis.cross(genr).normalize() };
                        let d = (genr + side * tilt).normalize();
                        (apex + genr * s + normal * off, d, format!("generator tilt={tilt:e} off={:e} a={a}", off))
                    }
                    // through the axis at height h, random direction
                    2 => {
                        let h = rng.range(-2.0, 2.0) * scale;
                        (apex + axis * h, rng.unit(), format!("through-axis h/scale={:.3} a={a}", h / scale))
                    }
                    // passing at distance d from the apex
                    _ => {
                        let d = rng.pick(&[0.0, 0.3, 1.0, 3.0, 10.0, 100.0, 1e4]) * eps;
                        let dir = rng.unit();
                        let perp = dir.cross(rng.unit()).normalize();
                        (apex + perp * d - dir * (rng.range(0.1, 1.0) * scale), dir, format!("apex d={d:e} a={a}"))
                    }
                };
                let (t0, t1) = (0.0, rng.range(0.5, 3.0) * scale / dir.norm());
                let got = line_cone_roots(origin, dir, (apex, axis, a), (t0, t1), band);
                let at = |t: f64| origin + dir * t;
                let f = |t: f64| res(apex, axis, a, at(t));
                let speed = dir.norm();
                let mut truth = changes(&f, t0, t1, 20_000);
                match got {
                    Ok(CircleRoots::Certified { count, thetas }) => {
                        cert += 1;
                        let roots = &thetas[..count];
                        // A close pair the grid stepped over: re-sample finely.
                        let n_in = roots.iter().filter(|&&t| (t - t0) * speed > eps && (t1 - t) * speed > eps).count();
                        if n_in != truth.iter().filter(|&&o| (o - t0) * speed > eps && (t1 - o) * speed > eps).count() {
                            truth = changes(&f, t0, t1, 4_000_000);
                        }
                        let inside: Vec<f64> = roots.iter().copied().filter(|&t| t > t0 && t < t1).collect();
                        for &r in roots {
                            let off = dist(apex, axis, a, at(r));
                            // A root outside the span only has to be placed no
                            // nearer the span than the band; inside, on the cone.
                            let outside = speed * (t0 - r).max(r - t1).max(0.0);
                            if outside == 0.0 && (r - t0) * speed > eps && (t1 - r) * speed > eps {
                                let near = truth.iter().map(|&o| (o - r).abs() * speed).fold(f64::INFINITY, f64::min);
                                worst = worst.max(near.max(off) / eps);
                                if near > eps || off > eps {
                                    wrong.push(format!("#{i} {label} scale={scale}: root {r} {near:e} from the oracle's {truth:?}, {off:e} off the cone"));
                                }
                            }
                            if (at(r) - apex).norm() <= eps {
                                wrong.push(format!("#{i} {label}: a certified root within eps of the apex"));
                            }
                        }
                        // every oracle crossing strictly inside must be a kernel root
                        let interior: Vec<f64> = truth.iter().copied().filter(|&o| (o - t0) * speed > eps && (t1 - o) * speed > eps).collect();
                        for o in &interior {
                            if !inside.iter().any(|&r| (r - o).abs() * speed <= eps) {
                                wrong.push(format!("#{i} {label} scale={scale}: oracle crossing {o} has no certified root ({roots:?})"));
                            }
                        }
                    }
                    Ok(CircleRoots::Miss) => {
                        miss += 1;
                        // the WHOLE line misses the double cone: check a wide window.
                        let big = 1e4 * scale / speed;
                        let wide = changes(&f, t0 - big, t1 + big, 200_000);
                        if !truth.is_empty() || !wide.is_empty() {
                            wrong.push(format!("#{i} {label} scale={scale}: Miss but the oracle crosses at {truth:?} / {} on the wide window", wide.len()));
                        }
                    }
                    Ok(CircleRoots::AtApex) => apex_n += 1,
                    Ok(_) => unc += 1,
                    Err(_) => err += 1,
                }
            }
            eprintln!("R1LINE eps={eps:e} class={class} scale={scale:e}: cert {cert} miss {miss} apex {apex_n} uncertain {unc} escalated {err} | worst/eps {worst:.3e} | WRONG {}", wrong.len());
            for w in wrong.iter().take(6) {
                eprintln!("R1LWRONG {w}");
            }
        }
    }
    let _ = PI;
}
