//! Review probes for PR 4540 (`piece_assembly`). Every reference is an
//! Interval enclosure ceiling of `‖C′‖` at a chosen point: a meter above
//! it is unsound with certainty.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(clippy::neg_cmp_op_on_partial_ord, clippy::cast_precision_loss)]

use geom::NurbsCurve3;
use geom_core::spline::KnotVector;
use geom_core::{Bounds, Interval, Point3, Real, Vec3};
use test_utils::fuzz;

fn ceil_at(c: &NurbsCurve3<f64>, t: f64) -> f64 {
    let ci = c.map_scalar(<Interval as Real>::from_f64);
    ci.deriv(Interval::from_f64(t)).norm().hi()
}

fn meters(c: &NurbsCurve3<f64>) -> (f64, f64) {
    let ci = c.map_scalar(<Interval as Real>::from_f64);
    (c.speed_lower_bound().get(), ci.speed_lower_bound().get().lo())
}

/// A one-span Bézier of degree `p` on `[off, off + len]` whose derivative
/// (in the span's unit parameter `s`) is `(s − s*)·w + (s − s*)²·v`
/// (degree 2 in `s`, raised to `p − 1` if `p > 3` by zero higher terms).
fn stall_bezier(p: usize, s_star: f64, w: Vec3<f64>, v: Vec3<f64>, off: f64, len: f64) -> NurbsCurve3<f64> {
    assert!(p >= 3);
    let q = p - 1;
    // Power basis of g(s) = a + b s + c s² with a = s*²v − s*w, b = w − 2s*v, c = v.
    let a = v * (s_star * s_star) - w * s_star;
    let b = w - v * (2.0 * s_star);
    let c = v;
    // Bernstein coeffs of degree q: β_k = Σ_j C(k,j)/C(q,j) m_j.
    let binom = |n: usize, k: usize| -> f64 {
        let mut r = 1.0;
        for i in 0..k {
            r = r * (n - i) as f64 / (i + 1) as f64;
        }
        r
    };
    let m = [a, b, c];
    let beta: Vec<Vec3<f64>> = (0..=q)
        .map(|k| {
            let mut acc = Vec3::new(0.0, 0.0, 0.0);
            for (j, mj) in m.iter().enumerate().take(k.min(2) + 1) {
                acc = acc + *mj * (binom(k, j) / binom(q, j));
            }
            acc
        })
        .collect();
    // dC/du = g(s)/len; Q_i = p (P_{i+1} − P_i)/len  ⇒  P_{i+1} = P_i + β_i / p.
    let mut pts = vec![Point3::new(off * 0.0 + 1.0e3, -7.0, 3.0)];
    for bk in &beta {
        let last = *pts.last().unwrap();
        pts.push(last + *bk * (1.0 / p as f64));
    }
    let mut knots = vec![off; p + 1];
    knots.extend(std::iter::repeat_n(off + len, p + 1));
    let kv = KnotVector::clamped(knots, p).unwrap();
    NurbsCurve3::new(kv, pts, vec![1.0; p + 1]).unwrap()
}

/// Claim 2: a stall at every piece boundary `k/16`, and inside pieces, of
/// a one-span curve at several degrees, scales, offsets and lengths.
#[test]
fn a_stall_at_a_piece_boundary_refuses() {
    let w = Vec3::new(3.1, 0.7, -1.3);
    let v = Vec3::new(-0.2, 1.9, 0.6);
    let mut worst_f64 = f64::NEG_INFINITY;
    let mut worst_f64_rel = f64::NEG_INFINITY;
    let mut bad = Vec::new();
    for p in [3, 4, 6, 9] {
        for scale in [1.0, 1.0e3, 1.0e6] {
            for (off, len) in [(0.0, 1.0), (1.0e6, 1.0), (0.0, 1.0e-6), (0.0, 1.0e6), (-3.7, 0.3)] {
                for k in 0..=32 {
                    let s_star = k as f64 / 32.0;
                    let c = stall_bezier(p, s_star, w * scale, v * scale, off, len);
                    let t_star = off + len * s_star;
                    let r = ceil_at(&c, t_star);
                    let (m, mi) = meters(&c);
                    if m > worst_f64 {
                        worst_f64 = m;
                    }
                    let qmag = scale / len;
                    if m / qmag > worst_f64_rel {
                        worst_f64_rel = m / qmag;
                    }
                    if mi > r || mi > 0.0 {
                        bad.push(format!("INTERVAL p{p} sc{scale} off{off} len{len} s*{s_star}: mi {mi:e} ref {r:e}"));
                    }
                    if m > r {
                        bad.push(format!("f64 p{p} sc{scale} off{off} len{len} s*{s_star}: m {m:e} ref {r:e} (m·len {:e})", m * len));
                    }
                }
            }
        }
    }
    println!("worst f64 meter on a stall: {worst_f64:e}; relative to |C'| scale: {worst_f64_rel:e}");
    for b in &bad {
        println!("{b}");
    }
    assert!(bad.iter().all(|b| !b.starts_with("INTERVAL")), "interval unsound: {bad:#?}");
}

/// Claim 1/2: a curve that comes within `gap` of stalling, at the piece
/// boundary and inside a piece: the meter stays below the ceiling at
/// the near-stall.
#[test]
fn a_near_stall_stays_below() {
    let w = Vec3::new(3.1, 0.7, -1.3);
    let v = Vec3::new(-0.2, 1.9, 0.6);
    for p in [3, 5] {
        for gap in [1.0e-12, 1.0e-9, 1.0e-6] {
            for s_star in [0.5, 0.53125, 0.3, 1.0 / 3.0] {
                let mut c = stall_bezier(p, s_star, w, v, 0.0, 1.0);
                // Lift the derivative by `gap` off-axis: shift the whole
                // derivative by a constant perpendicular to w and v.
                let n = Vec3::new(0.7 * 1.9 - (-1.3) * 0.6, -1.3 * -0.2 - 3.1 * 0.6, 3.1 * 1.9 - 0.7 * -0.2);
                let n = n * (gap / n.norm());
                let pts: Vec<Point3<f64>> = c
                    .control()
                    .iter()
                    .enumerate()
                    .map(|(i, q)| *q + n * (i as f64 / p as f64))
                    .collect();
                c = NurbsCurve3::new(c.knots().clone(), pts, vec![1.0; p + 1]).unwrap();
                let r = ceil_at(&c, s_star);
                let (m, mi) = meters(&c);
                println!("p{p} gap{gap:e} s*{s_star}: m {m:e} mi {mi:e} ref {r:e}");
                assert!(!(mi > r), "interval above the near-stall speed");
                assert!(!(m > r * (1.0 + 1e-9) + 1e-15), "f64 above the near-stall speed");
            }
        }
    }
}

/// Random multi-span nets with a stall placed at a RANDOM interior
/// parameter (not at a knot, not at a piece break): one control point
/// is moved so `C′(t*) ≈ 0`; the reference is the enclosure ceiling at
/// `t*`.
#[test]
fn a_random_interior_stall_refuses() {
    let mut rng = fuzz::start("review_4540::interior_stall");
    let cases = fuzz::scaled(400);
    let mut worst = f64::NEG_INFINITY;
    for case in 0..cases {
        let deg = 2 + case % 7;
        let npts = deg + 1 + rng.below(8);
        let mut control: Vec<Point3<f64>> = (0..npts)
            .map(|_| Point3::new(rng.range(-2.0, 2.0), rng.range(-2.0, 2.0), rng.range(-2.0, 2.0)))
            .collect();
        let interior = npts - deg - 1;
        let off = if case % 4 == 0 { 1.0e5 } else { 0.0 };
        let mut knots = vec![off; deg + 1];
        let mut v = off;
        let mut i = 0;
        while i < interior {
            v += rng.range(0.05, 1.2);
            let mult = (1 + if rng.unit() < 0.3 { rng.below(deg) } else { 0 }).min(interior - i);
            for _ in 0..mult {
                knots.push(v);
            }
            i += mult;
        }
        v += rng.range(0.05, 1.2);
        knots.extend(std::iter::repeat_n(v, deg + 1));
        let Ok(kv) = KnotVector::clamped(knots.clone(), deg) else { continue };
        let t_star = off + rng.unit() * (v - off);
        // Basis derivative N'_j(t*) via unit nets.
        let nprime: Vec<f64> = (0..npts)
            .map(|j| {
                let pts: Vec<Point3<f64>> = (0..npts)
                    .map(|k| Point3::new(if k == j { 1.0 } else { 0.0 }, 0.0, 0.0))
                    .collect();
                NurbsCurve3::new(kv.clone(), pts, vec![1.0; npts]).unwrap().deriv(t_star).x
            })
            .collect();
        let Ok(c0) = NurbsCurve3::new(kv.clone(), control.clone(), vec![1.0; npts]) else { continue };
        let d = c0.deriv(t_star);
        let (j, nj) = nprime
            .iter()
            .copied()
            .enumerate()
            .max_by(|a, b| a.1.abs().total_cmp(&b.1.abs()))
            .unwrap();
        control[j] = control[j] - d * (1.0 / nj);
        let Ok(c) = NurbsCurve3::new(kv, control, vec![1.0; npts]) else { continue };
        let r = ceil_at(&c, t_star);
        let (m, mi) = meters(&c);
        worst = worst.max(m);
        assert!(
            !(mi > r) && !(m > r + 1e-13),
            "case {case} deg {deg} npts {npts} t* {t_star}: m {m:e} mi {mi:e} ref {r:e} — {}",
            fuzz::replay()
        );
    }
    println!("worst f64 meter on interior stalls: {worst:e}");
}

/// Exact-in-f64 stalls (dyadic net, `C′(s*) = 0` exactly in ℝ): any
/// positive f64 meter here is above the true minimum, 0.
#[test]
fn an_exact_dyadic_stall_at_f64() {
    let mut pos = Vec::new();
    for (w, v) in [
        (Vec3::new(3.0, 0.75, -1.5), Vec3::new(-0.375, 6.0, 0.75)),
        (Vec3::new(3.0, 0.0, 0.0), Vec3::new(0.0, 3.0, 0.0)),
        (Vec3::new(3.0e3, 3.0e2, 0.0), Vec3::new(0.0, 3.0e-2, 3.0)),
    ] {
        for scale in [1.0, 1024.0, 1048576.0] {
            for (off, len) in [(0.0, 1.0), (1024.0, 1.0), (0.0, 0.0078125), (0.0, 65536.0), (-4.0, 0.5)] {
                for k in 0..=64 {
                    let s_star = k as f64 / 64.0;
                    let c = stall_bezier(3, s_star, w * scale, v * scale, off, len);
                    let t_star = off + len * s_star;
                    let r = ceil_at(&c, t_star);
                    let (m, mi) = meters(&c);
                    assert!(!(mi > 0.0));
                    if m > 0.0 {
                        pos.push(format!("w{w:?} sc{scale} off{off} len{len} s*{s_star}: m {m:e} ref {r:e} above_ref {}, metered {:e}", m > r, m * len));
                    }
                }
            }
        }
    }
    println!("{} positive f64 meters on exact stalls", pos.len());
    for p in pos.iter().take(20) {
        println!("{p}");
    }
}

/// Dump meters on a fixed corpus (for an A/B against a mutated join).
#[test]
fn dump_meters_for_ab() {
    let Ok(path) = std::env::var("R4540_DUMP") else { return };
    let mut rng = fuzz::Rng::from_seed(0x4540);
    let mut out = String::new();
    for case in 0..3000 {
        let deg = 2 + case % 5;
        let npts = deg + 1 + rng.below(10);
        let mut pts: Vec<Point3<f64>> = Vec::new();
        // Mix: random nets, near-straight nets, and swaying nets.
        for i in 0..npts {
            let base = match case % 3 {
                0 => Point3::new(rng.range(-2.0, 2.0), rng.range(-2.0, 2.0), rng.range(-2.0, 2.0)),
                1 => Point3::new(i as f64 + rng.range(-0.3, 0.3), rng.range(-0.3, 0.3), 0.0),
                _ => Point3::new(if i % 2 == 0 { -1.0 } else { 1.0 } * rng.range(0.1, 2.0), 0.0, i as f64),
            };
            pts.push(base);
        }
        let interior = npts - deg - 1;
        let mut knots = vec![0.0; deg + 1];
        let mut v = 0.0;
        for _ in 0..interior {
            v += rng.range(0.2, 1.2);
            knots.push(v);
        }
        v += rng.range(0.2, 1.2);
        knots.extend(std::iter::repeat_n(v, deg + 1));
        let kv = KnotVector::clamped(knots, deg).unwrap();
        let c = NurbsCurve3::new(kv, pts, vec![1.0; npts]).unwrap();
        let (m, mi) = meters(&c);
        out.push_str(&format!("{case} {m:e} {mi:e}\n"));
    }
    std::fs::write(path, out).unwrap();
}

/// Scale extremes on a stall: derivative coefficients near the f64
/// underflow / overflow of `‖sum‖²`.
#[test]
fn a_stall_at_extreme_scales() {
    let w = Vec3::new(3.1, 0.7, -1.3);
    let v = Vec3::new(-0.2, 1.9, 0.6);
    for scale in [1.0e-170, 1.0e-160, 1.0e-150, 1.0e150, 1.0e160] {
        for k in [0, 1, 7, 16, 17, 31] {
            let s_star = k as f64 / 32.0;
            for p in [2usize, 3] {
                let c = if p == 2 {
                    // C′(s) = (s − s*)·w, degree 2.
                    let q0 = w * (-s_star * scale);
                    let q1 = w * ((1.0 - s_star) * scale);
                    let p0 = Point3::new(0.0, 0.0, 0.0);
                    let p1 = p0 + q0 * 0.5;
                    let p2 = p1 + q1 * 0.5;
                    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
                    NurbsCurve3::new(kv, vec![p0, p1, p2], vec![1.0; 3]).unwrap()
                } else {
                    stall_bezier(3, s_star, w * scale, v * scale, 0.0, 1.0)
                };
                let (m, mi) = meters(&c);
                if m > 0.0 || mi > 0.0 {
                    println!("EXTREME p{p} scale {scale:e} s* {s_star}: m {m:e} mi {mi:e}");
                }
            }
        }
    }
}
