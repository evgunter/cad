//! r2 review probes for PR 3999 (limb 3 one-arc).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use crate::shared::tol::{band, eps};
use geom::{NurbsSurface, Surface};
use geom_brep::ssi::{self, SsiDomain, SsiOutcome};
use geom_core::spline::KnotVector;
use geom_core::{Point3, Vec3};

fn bern(c: &[f64], t: f64) -> f64 {
    let mut b = c.to_vec();
    let n = b.len();
    for r in 1..n {
        for i in 0..n - r {
            b[i] = (1.0 - t) * b[i] + t * b[i + 1];
        }
    }
    b[0]
}

/// z = g(x) + h(y), g and h given by Bernstein coefficients.
fn wall(x: (f64, f64), l: f64, gc: &[f64], hc: &[f64]) -> NurbsSurface<f64> {
    let (p, q) = (gc.len() - 1, hc.len() - 1);
    let kv = |d: usize| {
        let mut k = vec![0.0; d + 1];
        k.extend(vec![1.0; d + 1]);
        KnotVector::clamped(k, d).unwrap()
    };
    let mut ctrl = Vec::new();
    for i in 0..=p {
        for j in 0..=q {
            ctrl.push(Point3::new(
                x.0 + (x.1 - x.0) * i as f64 / p as f64,
                l * j as f64 / q as f64,
                gc[i] + hc[j],
            ));
        }
    }
    NurbsSurface::new(kv(p), kv(q), ctrl, vec![1.0; (p + 1) * (q + 1)]).unwrap()
}

fn ground() -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// zeros found by sign changes along both grid directions, farther than far from every carrier.
fn far_zeros(phi: &dyn Fn(f64, f64) -> f64, x: (f64, f64), l: f64, out: &SsiOutcome, far: f64) -> (usize, usize) {
    let pts: Vec<Point3<f64>> = out
        .branches
        .iter()
        .flat_map(|b| {
            let (t0, t1) = b.params;
            (0..=4000).map(move |k| b.carrier.eval(t0 + (t1 - t0) * f64::from(k) / 4000.0))
        })
        .collect();
    let n = 300u32;
    let at = |i: u32, lo: f64, hi: f64| lo + (hi - lo) * f64::from(i) / f64::from(n);
    let mut zeros = Vec::new();
    for j in 0..=n {
        for i in 0..n {
            let y = at(j, 0.0, l);
            let (a, b) = (at(i, x.0, x.1), at(i + 1, x.0, x.1));
            if phi(a, y).signum() != phi(b, y).signum() {
                zeros.push((0.5 * (a + b), y));
            }
            let xx = at(j, x.0, x.1);
            let (a, b) = (at(i, 0.0, l), at(i + 1, 0.0, l));
            if phi(xx, a).signum() != phi(xx, b).signum() {
                zeros.push((xx, 0.5 * (a + b)));
            }
        }
    }
    let far_n = zeros
        .iter()
        .filter(|(x, y)| pts.iter().all(|p| (p.x - x).powi(2) + (p.y - y).powi(2) > far * far))
        .count();
    (far_n, zeros.len())
}

fn run(tag: &str, x: (f64, f64), l: f64, gc: &[f64], hc: &[f64], extent: f64) -> Option<usize> {
    let w = wall(x, l, gc, hc);
    let domain = SsiDomain {
        center: Point3::new(0.0, 0.0, 0.0),
        half_extent: 1.0,
        extent,
        floor_scale: 1.0,
    };
    let phi = |xx: f64, y: f64| bern(gc, (xx - x.0) / (x.1 - x.0)) + bern(hc, y / l);
    match ssi::plane_nurbs_ssi(&ground(), &w, domain, band()) {
        Ok(out) => {
            let span = (x.1 - x.0).max(l);
            let (far, tot) = far_zeros(&phi, x, l, &out, 0.05 * span);
            let ends: Vec<String> = out.branches.iter().map(|b| format!("{:?}", b.end).chars().take(60).collect()).collect();
            eprintln!("PROBE {tag}: OK branches={} far_zeros={far}/{tot} ends={ends:?}", out.branches.len());
            Some(far)
        }
        Err(e) => {
            let s = format!("{e:?}");
            eprintln!("PROBE {tag}: ERR {}", &s[..s.len().min(110)]);
            None
        }
    }
}

/// Bernstein coefficients of the quadratic g(x) = c x + a x^2 over [x0, x1].
fn quad(x0: f64, x1: f64, c: f64, a: f64) -> [f64; 3] {
    let g = |x: f64| c * x + a * x * x;
    [g(x0), g(x0) + 0.5 * (x1 - x0) * (c + 2.0 * a * x0), g(x1)]
}

#[test]
fn r2_fold_sweep() {
    let mut lost = Vec::new();
    for &scale in &[eps(), 1e-3] {
        for &ar in &[0.2, 0.25, 0.28, 0.3, 0.35, 0.45] {
            for &lf in &[0.6, 1.2, 2.0, 4.0] {
                for &(xa, xb) in &[(-1.5, 1.8), (-1.0, 1.0), (-1.7, 3.0)] {
                    let beta = scale;
                    let c = 80.0 * beta;
                    let a = ar * c * c / beta;
                    let w = beta / c;
                    let l = lf * w;
                    let x = (xa * w, xb * w);
                    let gc = quad(x.0, x.1, c, a);
                    let hc = [0.0, 2.0 * beta, 0.0];
                    let tag = format!("fold s={scale:e} a={ar} L={lf} x=({xa},{xb})");
                    if let Some(f) = run(&tag, x, l, &gc, &hc, 1.0) {
                        if f > 0 {
                            lost.push(tag);
                        }
                    }
                }
            }
        }
    }
    assert!(lost.is_empty(), "LOST: {lost:?}");
}

#[test]
fn r2_loop_beside_branch() {
    // cubic g with roots r1<r2<r3 in t; h convex: H at ends, 0 at middle.
    let mut lost = Vec::new();
    for &scale in &[eps(), 1e-3, 1.0] {
        for &(r1, r2, r3) in &[(0.15, 0.5, 0.85), (0.2, 0.3, 0.6), (0.1, 0.25, 0.4), (0.3, 0.35, 0.9)] {
            for &hf in &[0.3, 0.6, 0.9] {
                // power -> Bernstein for k (t-r1)(t-r2)(t-r3): sample-and-solve via exact formula
                let k = scale / 0.01;
                let p = |t: f64| k * (t - r1) * (t - r2) * (t - r3);
                // Bernstein from values at 0,1/3,2/3,1 (cubic interpolation, solve 4x4)
                let v = [p(0.0), p(1.0 / 3.0), p(2.0 / 3.0), p(1.0)];
                // B matrix rows at t=0,1/3,2/3,1
                let b0 = v[0];
                let b3 = v[3];
                // at 1/3: (8 b0 + 12 b1 + 6 b2 + b3)/27 ; at 2/3: (b0 + 6 b1 + 12 b2 + 8 b3)/27
                let r_a = 27.0 * v[1] - 8.0 * b0 - b3;
                let r_b = 27.0 * v[2] - b0 - 8.0 * b3;
                // 12 b1 + 6 b2 = r_a ; 6 b1 + 12 b2 = r_b
                let b1 = (2.0 * r_a - r_b) / 18.0;
                let b2 = (2.0 * r_b - r_a) / 18.0;
                let gc = [b0, b1, b2, b3];
                // dip minimum in (r2, r3)
                let m = (0..1000)
                    .map(|i| p(r2 + (r3 - r2) * i as f64 / 1000.0))
                    .fold(f64::INFINITY, f64::min);
                let hh = -m / hf; // H > -m so loop closes when hf<1
                let hc = [hh, -hh, hh]; // h(s) = H (1-2s)^2 ... check: Bernstein [H,-H,H] -> H((1-s)^2 - 2s(1-s) + s^2) = H(1-2s)^2
                for &(xw, lw) in &[(1.0, 1.0), (0.2, 1.0), (1.0, 0.2)] {
                    let x = (-0.5 * xw * scale / 0.01 * 0.01, 0.5 * xw * scale / 0.01 * 0.01);
                    let l = lw * scale;
                    let tag = format!("loop s={scale:e} r=({r1},{r2},{r3}) hf={hf} xw={xw} lw={lw}");
                    if let Some(f) = run(&tag, x, l, &gc, &hc, 1.0) {
                        if f > 0 {
                            lost.push(tag);
                        }
                    }
                }
            }
        }
    }
    assert!(lost.is_empty(), "LOST: {lost:?}");
}

#[test]
fn r2_golden_at_006() {
    let sphere = geom::Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: 1.0,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let cyl = Surface::Cylinder {
        origin: Point3::new(0.03, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 0.08,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    for cx in [0.06, 0.0595, 0.059, 0.058, 0.057, 0.056, 0.055, 0.0549, 0.05, 0.04] {
        let d = SsiDomain {
            center: Point3::new(cx, 0.0, 0.996),
            half_extent: 0.05,
            extent: 0.2,
            floor_scale: 1.0,
        };
        let r = ssi::cylinder_sphere_ssi(&cyl, &sphere, d, band());
        let s = match r {
            Ok(o) => format!("OK branches={} ends={:?}", o.branches.len(), o.branches.iter().map(|b| format!("{:?}", b.end).chars().take(30).collect::<String>()).collect::<Vec<_>>()),
            Err(e) => format!("ERR {e:?}").chars().take(160).collect(),
        };
        eprintln!("PROBE golden cx={cx}: {s}");
    }
}

#[test]
fn r2_short_arc_variants() {
    let cylinder = Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let sphere = Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: 3.0,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let mut lost = Vec::new();
    for &ad in &[1.0f64, 3.0, 5.0, 8.0, 12.0] {
        for &bd in &[6.0f64, 9.0, 11.0, 15.0, 20.0, 30.0] {
            if bd <= ad + 0.5 {
                continue;
            }
            for &(half, ext) in &[(2.0, 4.8), (2.0, 1.0), (1.5, 0.3)] {
                let (alpha, beta) = (ad.to_radians(), bd.to_radians());
                let domain = SsiDomain {
                    center: Point3::new(alpha.cos() - half, beta.sin() - half, 2.0),
                    half_extent: half,
                    extent: ext,
                    floor_scale: 1.0,
                };
                let tag = format!("short a={ad} b={bd} half={half} ext={ext}");
                match ssi::cylinder_sphere_ssi(&cylinder, &sphere, domain, band()) {
                    Ok(out) => {
                        let mid = 0.5 * (alpha + beta);
                        let on_short = Point3::new(mid.cos(), mid.sin(), 8.0f64.sqrt());
                        let carried = out.branches.iter().any(|b| {
                            let (t0, t1) = b.params;
                            (0..=4000).any(|i| (b.carrier.eval(t0 + (t1 - t0) * f64::from(i) / 4000.0) - on_short).norm() < 1e-3)
                        });
                        eprintln!("PROBE {tag}: OK branches={} short_carried={carried}", out.branches.len());
                        if !carried {
                            lost.push(tag);
                        }
                    }
                    Err(e) => eprintln!("PROBE {tag}: ERR {}", format!("{e:?}").chars().take(120).collect::<String>()),
                }
            }
        }
    }
    assert!(lost.is_empty(), "LOST: {lost:?}");
}

#[test]
fn r2_render_endings() {
    use geom_brep::recourse::Reading;
    for r in [Reading::Build, Reading::AtRest] {
        eprintln!("PROBE render {r:?}: {}", ssi::SsiError::TubeNotOneArc { rungs: 3 }.render(r));
    }
}
