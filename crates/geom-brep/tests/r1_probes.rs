//! r1 review probes for PR 3999 (not for merge).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stdout)]

use crate::shared::tol::{band, eps};
use geom::{NurbsSurface, Surface};
use geom_brep::ssi::{self, SsiDomain, SsiOutcome};
use geom_core::spline::KnotVector;
use geom_core::{Point3, Vec3};

fn sphere() -> Surface<f64> {
    Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: 1.0,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

#[test]
fn r1_old_golden_at_006() {
    let c = Surface::Cylinder {
        origin: Point3::new(0.03, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 0.08,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    for cx in [0.06, 0.0595, 0.0605, 0.059, 0.061, 0.055] {
        let d = SsiDomain {
            center: Point3::new(cx, 0.0, 0.996),
            half_extent: 0.05,
            extent: 0.2,
            floor_scale: 1.0,
        };
        match ssi::cylinder_sphere_ssi(&c, &sphere(), d, band()) {
            Ok(o) => println!("R1 golden cx={cx}: Ok {} branches {:?}", o.branches.len(), o.branches.iter().map(|b| b.end).collect::<Vec<_>>()),
            Err(e) => println!("R1 golden cx={cx}: Err {e}\n   recourse: {}", e.ending(geom_brep::recourse::Reading::Build)),
        }
    }
}

/// Bi-(dx, 2) Bézier graph z = A(x) + B(y), A given by Bézier ordinates.
fn wall(xs: (f64, f64), ys: (f64, f64), a_ord: &[f64], b_ord: [f64; 3]) -> NurbsSurface<f64> {
    let p = a_ord.len() - 1;
    let mut ku = vec![0.0; p + 1];
    ku.extend(vec![1.0; p + 1]);
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let ku = KnotVector::clamped(ku, p).unwrap();
    let mut ctrl = Vec::new();
    for (i, a) in a_ord.iter().enumerate() {
        let x = xs.0 + (xs.1 - xs.0) * i as f64 / p as f64;
        for (j, b) in b_ord.iter().enumerate() {
            let y = ys.0 + (ys.1 - ys.0) * j as f64 / 2.0;
            ctrl.push(Point3::new(x, y, a + b));
        }
    }
    NurbsSurface::new(ku, kv, ctrl, vec![1.0; (p + 1) * 3]).unwrap()
}

/// Zeros of z on a dense grid farther than `far` from every carrier.
fn lost(w: &NurbsSurface<f64>, out: &SsiOutcome, far: f64) -> (usize, usize) {
    let pts: Vec<Point3<f64>> = out
        .branches
        .iter()
        .flat_map(|b| {
            let (t0, t1) = b.params;
            (0..=4000).map(move |k| b.carrier.eval(t0 + (t1 - t0) * f64::from(k) / 4000.0))
        })
        .collect();
    let n = 300;
    let mut zeros = Vec::new();
    for j in 0..=n {
        let v = f64::from(j) / f64::from(n);
        for i in 0..n {
            let (u0, u1) = (f64::from(i) / f64::from(n), f64::from(i + 1) / f64::from(n));
            let (p, q) = (w.eval(u0, v), w.eval(u1, v));
            if p.z.signum() != q.z.signum() {
                zeros.push(Point3::new(0.5 * (p.x + q.x), 0.5 * (p.y + q.y), 0.0));
            }
        }
        // and along u lines
        let u = v;
        for i in 0..n {
            let (v0, v1) = (f64::from(i) / f64::from(n), f64::from(i + 1) / f64::from(n));
            let (p, q) = (w.eval(u, v0), w.eval(u, v1));
            if p.z.signum() != q.z.signum() {
                zeros.push(Point3::new(0.5 * (p.x + q.x), 0.5 * (p.y + q.y), 0.0));
            }
        }
    }
    let far_n = zeros
        .iter()
        .filter(|z| pts.iter().all(|p| (p.x - z.x).powi(2) + (p.y - z.y).powi(2) > far * far))
        .count();
    (zeros.len(), far_n)
}

fn ground() -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

fn dom(extent: f64) -> SsiDomain {
    SsiDomain {
        center: Point3::new(0.0, 0.0, 0.0),
        half_extent: 10.0,
        extent,
        floor_scale: 1.0,
    }
}

/// The fold, swept over c, a, the x range and orientation.
#[test]
fn r1_fold_sweep() {
    let mut bad = Vec::new();
    let (mut oks, mut errs) = (0, 0);
    for cm in [20.0, 40.0, 80.0, 160.0] {
        for af in [0.2, 0.26, 0.28, 0.3, 0.4] {
            for (xl, xh) in [(-1.5, 1.8), (-1.2, 1.5), (-2.0, 2.5), (-1.5, 1.0)] {
                for flip in [false, true] {
                    let beta = eps();
                    let c = cm * beta;
                    let a = af * c * c / beta;
                    let w = beta / c;
                    let l = 1.2 * w;
                    let (x0, x1) = (xl * w, xh * w);
                    let g = |x: f64| c * x + a * x * x;
                    let s = if flip { -1.0 } else { 1.0 };
                    let gb = [s * g(x0), s * (g(x0) + 0.5 * (x1 - x0) * (c + 2.0 * a * x0)), s * g(x1)];
                    let hb = [0.0, s * 2.0 * beta, 0.0];
                    let wl = wall((x0, x1), (0.0, l), &gb, hb);
                    match ssi::plane_nurbs_ssi(&ground(), &wl, dom(1.0), band()) {
                        Ok(o) => {
                            oks += 1;
                            let (n, far) = lost(&wl, &o, 0.3 * w);
                            if far > 0 {
                                bad.push(format!("cm={cm} af={af} x=({xl},{xh}) flip={flip}: {far}/{n} zeros far, {} branches", o.branches.len()));
                            }
                        }
                        Err(e) => {
                            errs += 1;
                            println!("R1 fold cm={cm} af={af} x=({xl},{xh}) flip={flip}: Err {e}");
                        }
                    }
                }
            }
        }
    }
    println!("R1 fold sweep: {oks} Ok, {errs} Err");
    assert!(bad.is_empty(), "LOST COMPONENTS:\n{}", bad.join("\n"));
}

/// A small closed loop beside a long U branch: z = A(x) + k y², A quartic.
#[test]
fn r1_loop_beside_branch() {
    let mut bad = Vec::new();
    let (mut oks, mut errs) = (0, 0);
    for scale in [1.0, 1e-3, 1e-6] {
        for gap in [0.05, 0.1, 0.2] {
            for extent in [0.1, 1.0, 10.0] {
                // roots of A: 0.2, 0.3, 0.3+gap, 1.3; A = 4 Π (x - r)
                let r = [0.2, 0.3, 0.3 + gap, 1.3];
                let a = |x: f64| 4.0 * r.iter().map(|ri| x - ri).product::<f64>() * scale;
                // power -> Bezier ordinates of a quartic on [0,1] via sampling solve
                let ord = bezier_ordinates(&a, 4);
                let k = scale;
                // B(y) = k y^2 over y in [-1, 1]: ordinates [k, -k, k]
                let wl = wall((0.0, 1.0), (-1.0, 1.0), &ord, [k, -k, k]);
                match ssi::plane_nurbs_ssi(&ground(), &wl, dom(extent), band()) {
                    Ok(o) => {
                        oks += 1;
                        let (n, far) = lost(&wl, &o, 0.01);
                        println!("R1 loop scale={scale} gap={gap} ext={extent}: Ok {} branches, {far}/{n} far", o.branches.len());
                        if far > 0 {
                            bad.push(format!("scale={scale} gap={gap} ext={extent}: {far}/{n}"));
                        }
                    }
                    Err(e) => {
                        errs += 1;
                        println!("R1 loop scale={scale} gap={gap} ext={extent}: Err {e}");
                    }
                }
            }
        }
    }
    println!("R1 loop: {oks} Ok, {errs} Err");
    assert!(bad.is_empty(), "LOST COMPONENTS:\n{}", bad.join("\n"));
}

/// Bézier ordinates of a degree-p polynomial on [0,1] by solving the
/// Bernstein collocation system at p+1 equispaced points.
fn bezier_ordinates(f: &impl Fn(f64) -> f64, p: usize) -> Vec<f64> {
    let n = p + 1;
    let binom = |n: usize, k: usize| (0..k).fold(1.0, |acc, i| acc * (n - i) as f64 / (i + 1) as f64);
    let mut m = vec![vec![0.0; n + 1]; n];
    for (i, row) in m.iter_mut().enumerate() {
        let t = i as f64 / p as f64;
        for (k, cell) in row.iter_mut().take(n).enumerate() {
            *cell = binom(p, k) * t.powi(k as i32) * (1.0 - t).powi((p - k) as i32);
        }
        row[n] = f(t);
    }
    for c in 0..n {
        let piv = (c..n).max_by(|a, b| m[*a][c].abs().total_cmp(&m[*b][c].abs())).unwrap();
        m.swap(c, piv);
        for r in 0..n {
            if r != c {
                let fct = m[r][c] / m[c][c];
                for k in c..=n {
                    m[r][k] -= fct * m[c][k];
                }
            }
        }
    }
    (0..n).map(|i| m[i][n] / m[i][i]).collect()
}

/// The R3 clipped circle swept over the two cut angles.
#[test]
fn r1_r3_clipped_circle_sweep() {
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
    let mut bad = Vec::new();
    let (mut oks, mut errs) = (0, 0);
    for ad in [2.0f64, 5.0, 10.0, 20.0] {
        for gap in [0.5f64, 2.0, 6.0, 15.0] {
            let (alpha, beta) = (ad.to_radians(), (ad + gap).to_radians());
            let half = 2.0;
            let domain = SsiDomain {
                center: Point3::new(alpha.cos() - half, beta.sin() - half, 2.0),
                half_extent: half,
                extent: 4.8,
                floor_scale: 1.0,
            };
            match ssi::cylinder_sphere_ssi(&cylinder, &sphere, domain, band()) {
                Ok(out) => {
                    oks += 1;
                    let mid = 0.5 * (alpha + beta);
                    let on_short = Point3::new(mid.cos(), mid.sin(), 8.0f64.sqrt());
                    let carried = out.branches.iter().any(|b| {
                        let (t0, t1) = b.params;
                        (0..=4000).any(|i| {
                            let q = b.carrier.eval(t0 + (t1 - t0) * f64::from(i) / 4000.0);
                            (q - on_short).norm() < 1e-3
                        })
                    });
                    println!("R1 r3 a={ad} gap={gap}: Ok {} branches, short carried {carried}", out.branches.len());
                    if !carried {
                        bad.push(format!("a={ad} gap={gap}: {} branches", out.branches.len()));
                    }
                }
                Err(e) => {
                    errs += 1;
                    println!("R1 r3 a={ad} gap={gap}: Err {e}");
                }
            }
        }
    }
    println!("R1 r3: {oks} Ok, {errs} Err");
    assert!(bad.is_empty(), "LOST COMPONENTS:\n{}", bad.join("\n"));
}
