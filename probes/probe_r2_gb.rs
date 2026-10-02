//! Reviewer probe (PR 3817, lane reach-dual3817-r2): the Gauss–Bonnet
//! sphere arm against an INDEPENDENT Monte Carlo oracle — the left
//! region of the loop (interior-left under the outward normal `σ(p−c)/R`)
//! found by a stereographic winding number of the densely sampled
//! boundary, its area and its flux `∫ x·N dA` integrated by sampling.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::point::{p3, v3};
use crate::shared::tol::band;
use geom::{Curve3, Surface};
use geom_brep::props::{LoopEdge, curved_face};
use geom_core::Vec3;

type V = [f64; 3];
fn add(a: V, b: V) -> V { [a[0] + b[0], a[1] + b[1], a[2] + b[2]] }
fn sub(a: V, b: V) -> V { [a[0] - b[0], a[1] - b[1], a[2] - b[2]] }
fn mul(a: V, s: f64) -> V { [a[0] * s, a[1] * s, a[2] * s] }
fn dot(a: V, b: V) -> f64 { a[0] * b[0] + a[1] * b[1] + a[2] * b[2] }
fn cross(a: V, b: V) -> V {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
fn unit(a: V) -> V { mul(a, 1.0 / dot(a, a).sqrt()) }
fn vv(a: V) -> Vec3<f64> { v3(a[0], a[1], a[2]) }

struct Rng(u64);
impl Rng {
    fn f(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
    fn dir(&mut self) -> V {
        loop {
            let v = [2.0 * self.f() - 1.0, 2.0 * self.f() - 1.0, 2.0 * self.f() - 1.0];
            let n = dot(v, v);
            if n > 1e-4 && n < 1.0 { return unit(v); }
        }
    }
}

/// One arc on the UNIT sphere (local frame): plane normal `n` (traversal
/// ccw about `n`), offset `d = n·p`, from `a` sweeping `span`.
#[derive(Clone, Copy)]
struct Arc { n: V, d: f64, u: V, span: f64 }
impl Arc {
    /// The arc from `a` to `b` ccw about `n` (both on the plane `n·p = d`).
    fn through(n: V, a: V, b: V) -> Arc {
        let d = dot(n, a);
        let u = unit(sub(a, mul(n, d)));
        let w = cross(n, u);
        let bl = sub(b, mul(n, d));
        let mut span = dot(bl, w).atan2(dot(bl, u));
        if span <= 0.0 { span += core::f64::consts::TAU; }
        Arc { n, d, u, span }
    }
    fn at(&self, t: f64) -> V {
        let rho = (1.0 - self.d * self.d).sqrt();
        let w = cross(self.n, self.u);
        add(mul(self.n, self.d), add(mul(self.u, rho * t.cos()), mul(w, rho * t.sin())))
    }
}

/// The kernel loop (sphere centre `c`, radius `r`), each arc forward or
/// (to exercise `forward = false`) stored reversed with a flipped axis.
fn kernel_loop(arcs: &[Arc], c: V, r: f64, flip: &[bool]) -> Vec<LoopEdge<f64>> {
    let k = arcs.len() as u32;
    arcs.iter().enumerate().map(|(i, a)| {
        let rho = (1.0 - a.d * a.d).sqrt();
        let i = i as u32;
        if !flip[i as usize] {
            let carrier = Curve3::Circle {
                center: p3(c[0] + r * a.d * a.n[0], c[1] + r * a.d * a.n[1], c[2] + r * a.d * a.n[2]),
                axis: vv(a.n), radius: r * rho, u_ref: vv(a.u),
            };
            LoopEdge::hand_built(carrier, 0.0, a.span, true, i, (i + 1) % k)
        } else {
            // Same locus, axis −n, u_ref at the arc's END: parameter
            // 0 → span runs end → start, traversed reversed.
            let e = unit(sub(a.at(a.span), mul(a.n, a.d)));
            let carrier = Curve3::Circle {
                center: p3(c[0] + r * a.d * a.n[0], c[1] + r * a.d * a.n[1], c[2] + r * a.d * a.n[2]),
                axis: vv(mul(a.n, -1.0)), radius: r * rho, u_ref: vv(e),
            };
            LoopEdge::hand_built(carrier, 0.0, a.span, false, i, (i + 1) % k)
        }
    }).collect()
}

/// Monte Carlo: area and flux of the region LEFT of the loop under the
/// normal `σ·p` (unit sphere local frame), then scaled to (c, r).
fn oracle(arcs: &[Arc], sigma: f64, c: V, r: f64, n_samples: usize) -> (f64, f64) {
    // Dense boundary polyline.
    let mut poly: Vec<V> = Vec::new();
    for a in arcs {
        let m = 4000;
        for j in 0..m { poly.push(a.at(a.span * j as f64 / m as f64)); }
    }
    // A probe point just left of the boundary (left = σp × T).
    let a0 = arcs[0];
    let tm = 0.5 * a0.span;
    let pm = a0.at(tm);
    let tang = unit(sub(a0.at(tm + 1e-6), a0.at(tm - 1e-6)));
    let left = unit(add(pm, mul(cross(mul(pm, sigma), tang), 1e-3)));
    // Stereographic pole: a random direction off the curve.
    let mut rng = Rng(0x5eed);
    let pole = rng.dir();
    let proj = |p: V| -> (f64, f64) {
        // Frame with pole as "north".
        let e1 = unit(cross(pole, if pole[0].abs() < 0.9 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] }));
        let e2 = cross(pole, e1);
        let s = 1.0 - dot(p, pole);
        (dot(p, e1) / s, dot(p, e2) / s)
    };
    let pp: Vec<(f64, f64)> = poly.iter().map(|&p| proj(p)).collect();
    let wind = |q: V| -> i64 {
        let (qx, qy) = proj(q);
        let mut tot = 0.0;
        for i in 0..pp.len() {
            let (ax, ay) = (pp[i].0 - qx, pp[i].1 - qy);
            let j = (i + 1) % pp.len();
            let (bx, by) = (pp[j].0 - qx, pp[j].1 - qy);
            tot += (ax * by - ay * bx).atan2(ax * bx + ay * by);
        }
        (tot / core::f64::consts::TAU).round() as i64
    };
    let w_in = wind(left);
    let mut inside = 0usize;
    let mut fsum = 0.0;
    let mut rng = Rng(12345);
    for _ in 0..n_samples {
        let p = rng.dir();
        if wind(p) == w_in {
            inside += 1;
            // x = c + r p, N = σ p: x·N = σ(c·p + r).
            fsum += sigma * (dot(c, p) + r);
        }
    }
    let sphere_area = 4.0 * core::f64::consts::PI * r * r;
    let frac = inside as f64 / n_samples as f64;
    (frac * sphere_area, fsum / n_samples as f64 * sphere_area)
}

fn sphere(c: V, r: f64, axis: V) -> Surface<f64> {
    let u = unit(cross(axis, [0.3, 0.7, 0.1]));
    Surface::Sphere { center: p3(c[0], c[1], c[2]), radius: r, axis: vv(axis), u_ref: vv(u) }
}

fn check(label: &str, arcs: &[Arc], c: V, r: f64, n: usize) {
    let s = sphere(c, r, unit([0.1, 0.2, 1.0]));
    let k = arcs.len();
    for sense in [true, false] {
        for flips in [vec![false; k], (0..k).map(|i| i % 2 == 1).collect::<Vec<_>>()] {
            let lp = kernel_loop(arcs, c, r, &flips);
            let got = curved_face(&s, &lp, sense, band());
            let (oa, of) = oracle(arcs, if sense { 1.0 } else { -1.0 }, c, r, n);
            let tol = 4.0 * (4.0 * core::f64::consts::PI * r * r) / (n as f64).sqrt();
            match got {
                Ok(fc) => {
                    println!("{label} sense={sense} flips={flips:?}: area {:.6} oracle {:.6} | flux {:.6} oracle {:.6}", fc.area, oa, fc.flux, of);
                    assert!((fc.area - oa).abs() < tol, "{label}: AREA kernel {} oracle {}", fc.area, oa);
                    let ftol = tol * (r + (dot(c, c)).sqrt());
                    assert!((fc.flux - of).abs() < ftol, "{label}: FLUX kernel {} oracle {}", fc.flux, of);
                }
                Err(e) => println!("{label} sense={sense} flips={flips:?}: REFUSED {e:?} (oracle area {oa:.6})"),
            }
        }
    }
}

#[test]
fn probe_r2_gb_against_monte_carlo() {
    let n = 40_000;
    let mut rng = Rng(77);
    let c = [0.7, -1.3, 2.1];
    // 1. Random spherical triangles / quads with small-circle arcs.
    for case in 0..6 {
        let m = 3 + case % 2;
        let centre = rng.dir();
        let mut vs: Vec<V> = Vec::new();
        let e1 = unit(cross(centre, [0.3, 0.1, 0.9]));
        let e2 = cross(centre, e1);
        for j in 0..m {
            let ang = core::f64::consts::TAU * j as f64 / m as f64 + 0.3 * rng.f();
            let rad = 0.4 + 0.6 * rng.f();
            vs.push(unit(add(centre, add(mul(e1, rad * ang.cos()), mul(e2, rad * ang.sin())))));
        }
        let arcs: Vec<Arc> = (0..m).map(|j| {
            let (a, b) = (vs[j], vs[(j + 1) % m]);
            let g = unit(cross(a, b));
            let mm = unit(add(a, b));
            let phi = (rng.f() - 0.5) * 1.2;
            Arc::through(unit(add(mul(g, phi.cos()), mul(mm, phi.sin()))), a, b)
        }).collect();
        check(&format!("poly{case}"), &arcs, c, 1.7, n);
    }
    // 2. Tilted cap: one full circle, and the same split in two arcs.
    let nn = unit([0.4, -0.5, 0.77]);
    let d: f64 = 0.35;
    let a = Arc::through(nn, unit(add(mul(nn, d), mul(unit(cross(nn, [1.0, 0.0, 0.0])), (1.0 - d * d).sqrt()))), [0.0; 3]);
    let full = Arc { span: core::f64::consts::TAU, ..a };
    check("cap-1arc", &[full], c, 1.0, n);
    let half1 = Arc { span: 2.0, ..a };
    let half2 = Arc { u: unit(sub(a.at(2.0), mul(nn, d))), span: core::f64::consts::TAU - 2.0, ..a };
    check("cap-2arc", &[half1, half2], c, 1.0, n);
    // 3. Lune between two tilted great circles (antipodal vertices).
    let v0 = unit([0.3, 0.8, -0.2]);
    let g1 = unit(cross(v0, [0.2, 0.1, 0.9]));
    let g2 = unit(add(mul(g1, 0.6f64.cos()), mul(cross(v0, g1), 0.6f64.sin())));
    let l1 = Arc::through(g1, v0, mul(v0, -1.0));
    let l1 = Arc { span: core::f64::consts::PI, ..l1 };
    let l2 = Arc { n: mul(g2, -1.0), d: 0.0, u: mul(v0, -1.0), span: core::f64::consts::PI };
    check("lune", &[l1, l2], c, 2.5, n);
    // 4. Zone between two parallel tilted circles cut open by a seam
    // (great-circle arc traversed both ways): annulus as one loop.
    let za = Arc { span: core::f64::consts::TAU, ..Arc::through(nn, {
        let d = 0.5; unit(add(mul(nn, d), mul(unit(cross(nn, [1.0, 0.0, 0.0])), (1.0f64 - d * d).sqrt())))
    }, [0.0; 3]) };
    let zb0 = Arc::through(nn, {
        let d = -0.2; unit(add(mul(nn, d), mul(unit(cross(nn, [1.0, 0.0, 0.0])), (1.0f64 - d * d).sqrt())))
    }, [0.0; 3]);
    let p_top = za.at(0.0);
    let p_bot = zb0.at(0.0);
    let gs = unit(cross(p_top, p_bot));
    let seam_down = Arc::through(gs, p_top, p_bot);
    let zb_rev = Arc { n: mul(nn, -1.0), d: 0.2, u: zb0.u, span: core::f64::consts::TAU };
    let seam_up = Arc::through(mul(gs, -1.0), p_bot, p_top);
    // za (ccw about nn: the cap ABOVE is to its left) → down → bottom
    // reversed → up: left region is the cap above?? record both senses.
    check("zone-seam", &[za, seam_down, zb_rev, seam_up], c, 1.3, n);
    // 5. More than a hemisphere: reverse the first triangle (complement).
    let tri: Vec<V> = vec![unit([1.0, 0.1, 0.0]), unit([0.0, 1.0, 0.2]), unit([0.1, 0.0, 1.0])];
    let arcs: Vec<Arc> = (0..3).map(|j| Arc::through(unit(cross(tri[j], tri[(j + 1) % 3])), tri[j], tri[(j + 1) % 3])).collect();
    let rev: Vec<Arc> = (0..3).rev().map(|j| Arc::through(mul(arcs[j].n, -1.0), tri[(j + 1) % 3], tri[j])).collect();
    check("tri-big", &rev, c, 1.0, n);
    // 6. Scales.
    for r in [1e-3, 1e3] {
        check(&format!("cap-2arc r{r}"), &[half1, half2], mul(c, r), r, n / 4);
    }
}
