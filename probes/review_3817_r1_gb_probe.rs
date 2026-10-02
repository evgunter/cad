//! Reviewer probe for PR #3817 (lane reach-dual3817-r1): the Gauss–Bonnet
//! sphere arm (`curved::sphere_circle_loop`) against a Monte Carlo
//! oracle — uniform samples on the sphere, point-in-face by a winding
//! number of the sampled loop under stereographic projection, area and
//! flux `∫ p·n dA` summed directly. Nothing of the kernel is read but the
//! answer under test.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use crate::shared::tol::band;
use geom::{Curve3, Surface};
use geom_brep::props::{LoopEdge, curved_face};
use geom_core::{Point3, Vec3};

struct Lcg(u64);
impl Lcg {
    fn f(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
    fn unit(&mut self) -> Vec3<f64> {
        loop {
            let v = Vec3::new(2.0 * self.f() - 1.0, 2.0 * self.f() - 1.0, 2.0 * self.f() - 1.0);
            let n = v.norm();
            if n > 0.1 && n <= 1.0 {
                return v / n;
            }
        }
    }
}

fn perp(n: Vec3<f64>) -> Vec3<f64> {
    let a = if n.x.abs() < 0.9 { Vec3::new(1.0, 0.0, 0.0) } else { Vec3::new(0.0, 1.0, 0.0) };
    let p = n.cross(a);
    p / p.norm()
}

/// The arc from `a` to `b` on the circle of the sphere (`c`, `r`) cut by
/// the plane through `a`, `b` with unit normal `n` (n ⊥ b − a). `long`
/// picks the longer of the two arcs. Returns the arc as a forward edge.
fn arc(c: Point3<f64>, r: f64, a: Point3<f64>, b: Point3<f64>, n: Vec3<f64>, long: bool, s: u32, e: u32) -> LoopEdge<f64> {
    let h = (a - c).dot(n);
    let cc = c + n * h;
    let rho = (r * r - h * h).sqrt();
    let u = (a - cc) / (a - cc).norm();
    let v = n.cross(u);
    let mut t1 = ((b - cc).dot(v)).atan2((b - cc).dot(u));
    if t1 < 0.0 {
        t1 += 2.0 * PI;
    }
    let (axis, t1) = if (t1 > PI) != long { (n * -1.0, 2.0 * PI - t1) } else { (n, t1) };
    let carrier = Curve3::Circle { center: cc, axis, radius: rho, u_ref: u };
    let pb = carrier.eval(t1);
    assert!((pb - b).norm() < 1e-9 * r.max(1.0), "arc param convention");
    assert!((carrier.eval(0.0) - a).norm() < 1e-9 * r.max(1.0));
    LoopEdge::hand_built(carrier, 0.0, t1, true, s, e)
}

fn sample_loop(edges: &[LoopEdge<f64>]) -> Vec<Point3<f64>> {
    let mut pts = Vec::new();
    for e in edges {
        let n = 300;
        for i in 0..n {
            let s = i as f64 / n as f64;
            let t = if e.forward { e.t0 + (e.t1 - e.t0) * s } else { e.t1 - (e.t1 - e.t0) * s };
            pts.push(e.carrier.eval(t));
        }
    }
    pts
}

/// Monte Carlo area and flux of the face left of `edges` under outward
/// normal `σ·(p − c)/R`.
fn mc(c: Point3<f64>, r: f64, edges: &[LoopEdge<f64>], sense: bool, n: usize, seed: u64) -> (f64, f64) {
    let sigma = if sense { 1.0 } else { -1.0 };
    let pts = sample_loop(edges);
    let mut rng = Lcg(seed);
    // One pole for the whole run, far from the loop.
    let pole = loop {
        let p = rng.unit();
        if pts.iter().all(|&q| ((q - c) / r - p).norm() > 0.05) {
            break p;
        }
    };
    let e1 = perp(pole);
    let e2 = pole.cross(e1);
    let proj = |p: Point3<f64>| {
        let u = (p - c) / r;
        let d = 1.0 - u.dot(pole);
        (u.dot(e1) / d, u.dot(e2) / d)
    };
    let pp: Vec<(f64, f64)> = pts.iter().map(|&p| proj(p)).collect();
    let mut sarea = 0.0;
    for i in 0..pp.len() {
        let (x0, y0) = pp[i];
        let (x1, y1) = pp[(i + 1) % pp.len()];
        sarea += x0 * y1 - x1 * y0;
    }
    let (mut area, mut flux) = (0.0, 0.0);
    let da = 4.0 * PI * r * r / n as f64;
    for _ in 0..n {
        let u = rng.unit();
        let q = c + u * r;
        let (qx, qy) = proj(q);
        let mut wind = 0.0;
        for i in 0..pp.len() {
            let (x0, y0) = pp[i];
            let (x1, y1) = pp[(i + 1) % pp.len()];
            let mut d = (y1 - qy).atan2(x1 - qx) - (y0 - qy).atan2(x0 - qx);
            if d > PI {
                d -= 2.0 * PI;
            }
            if d < -PI {
                d += 2.0 * PI;
            }
            wind += d;
        }
        let w = (wind / (2.0 * PI)).round();
        // calibrated in probe_gb_calibration (ORIENT = +1)
        let v = -sigma * w;
        let ap = -sigma * sarea;
        if v == if ap > 0.0 { 1.0 } else { 0.0 } {
            area += da;
            let nrm = u * sigma;
            flux += (q - Point3::origin()).dot(nrm) * da;
        }
    }
    (area, flux)
}

fn sphere(c: Point3<f64>, r: f64, axis: Vec3<f64>) -> Surface<f64> {
    Surface::Sphere { center: c, radius: r, axis, u_ref: perp(axis) }
}

fn check(label: &str, c: Point3<f64>, r: f64, axis: Vec3<f64>, edges: &[LoopEdge<f64>], sense: bool, fails: &mut Vec<String>) {
    let n = 40_000;
    let (ma, mf) = mc(c, r, edges, sense, n, 99);
    let got = curved_face(&sphere(c, r, axis), edges, sense, band());
    let tot = 4.0 * PI * r * r;
    let p = ma / tot;
    let sd = tot * (p * (1.0 - p) / n as f64).sqrt() + 1e-3 * tot / 1000.0;
    match got {
        Ok(fc) => {
            let line = format!(
                "{label} sense={sense}: kernel area {:.6} mc {:.6} (sd {:.1e}); flux {:.6} mc {:.6}",
                fc.area, ma, sd, fc.flux, mf
            );
            eprintln!("{line}");
            let fsd = sd * ((c - Point3::origin()).norm() + r) * 1.5;
            if (fc.area - ma).abs() > 5.0 * sd || (fc.flux - mf).abs() > 5.0 * fsd {
                fails.push(line);
            }
        }
        Err(e) => eprintln!("{label} sense={sense}: REFUSED {e:?} (mc area {ma:.6})"),
    }
}

#[test]
fn probe_gb_against_monte_carlo() {
    let mut fails = Vec::new();
    let mut rng = Lcg(5);
    for (ci, (c, r)) in [(Point3::new(0.0, 0.0, 0.0), 1.0), (Point3::new(3.0, -2.0, 1.5), 2.5)].into_iter().enumerate() {
        let axis = Vec3::new(0.2, 1.0, -0.3);
        let axis = axis / axis.norm();
        let on = |d: Vec3<f64>| c + d / d.norm() * r;
        // 1. tilted cap: one circle, a single vertex.
        for (k, h) in [0.6, -0.4].into_iter().enumerate() {
            let n = Vec3::new(1.0, 0.4, 0.7);
            let n = n / n.norm();
            let cc = c + n * (h * r);
            let rho = r * (1.0 - h * h).sqrt();
            let carrier = Curve3::Circle { center: cc, axis: n, radius: rho, u_ref: perp(n) };
            let e = vec![LoopEdge::hand_built(carrier, 0.0, 2.0 * PI, true, 0, 0)];
            for s in [true, false] {
                check(&format!("c{ci} cap{k}"), c, r, axis, &e, s, &mut fails);
            }
        }
        // 2. tilted lune: two great semicircles between antipodes.
        let a = on(Vec3::new(1.0, 0.3, 0.2));
        let b = c + (c - a);
        let d = (a - c) / r;
        let n1 = perp(d);
        let n2 = (n1 * 0.6 + d.cross(n1) * 0.8) * 1.0;
        let lune = vec![arc(c, r, a, b, n1, false, 0, 1), arc(c, r, b, a, n2 * -1.0, false, 1, 0)];
        // which way is valid is decided by the oracle; just check both senses
        for s in [true, false] {
            check(&format!("c{ci} lune"), c, r, axis, &lune, s, &mut fails);
        }
        // 3. random small-circle polygons (convex-ish around a direction).
        for trial in 0..8 {
            let dir = rng.unit();
            let e1 = perp(dir);
            let e2 = dir.cross(e1);
            let k = 3 + (trial % 4);
            let spread = 0.3 + 0.9 * rng.f();
            let verts: Vec<Point3<f64>> = (0..k)
                .map(|i| {
                    let t = 2.0 * PI * (i as f64 + 0.3 * rng.f()) / k as f64;
                    on(dir + (e1 * t.cos() + e2 * t.sin()) * spread)
                })
                .collect();
            let mut edges = Vec::new();
            for i in 0..k {
                let (p, q) = (verts[i], verts[(i + 1) % k]);
                let chord = (q - p) / (q - p).norm();
                // a plane containing p,q: normal ⊥ chord, rotated off the great circle
                let g = (p - c).cross(q - c);
                let g = g / g.norm();
                let tilt = (rng.f() - 0.5) * 0.8;
                let w = chord.cross(g);
                let n = g * tilt.cos() + w * tilt.sin();
                edges.push(arc(c, r, p, q, n, false, i as u32, ((i + 1) % k) as u32));
            }
            for s in [true, false] {
                check(&format!("c{ci} poly{trial} k{k}"), c, r, axis, &edges, s, &mut fails);
            }
        }
        // 4. a face larger than a hemisphere stated with a LONG arc.
        let p = on(Vec3::new(0.0, 0.0, 1.0));
        let q = on(Vec3::new(0.0, 1.0, 0.2));
        let g = (p - c).cross(q - c);
        let g = g / g.norm();
        let w = ((q - p) / (q - p).norm()).cross(g);
        let n_a = g * 0.9_f64.cos() + w * 0.9_f64.sin();
        let n_b = g * (-0.5_f64).cos() + w * (-0.5_f64).sin();
        let big = vec![arc(c, r, p, q, n_a, true, 0, 1), arc(c, r, q, p, n_b, false, 1, 0)];
        for s in [true, false] {
            check(&format!("c{ci} long-arc"), c, r, axis, &big, s, &mut fails);
        }
    }
    assert!(fails.is_empty(), "FAILS:\n{}", fails.join("\n"));
}
