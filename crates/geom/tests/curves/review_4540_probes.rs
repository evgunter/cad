//! Review probes for PR 4540 (piece assembly of the integral speed
//! meter). Exact-construction stalls and near-stalls, at piece and span
//! boundaries, short / long spans, far from the origin.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(clippy::neg_cmp_op_on_partial_ord, clippy::cast_precision_loss)]

use geom::NurbsCurve3;
use geom_core::spline::KnotVector;
use geom_core::{Bounds, Interval, Point3, Real, Vec3};

fn binom(n: usize, k: usize) -> f64 {
    let mut r = 1.0;
    for i in 0..k {
        r = r * (n - i) as f64 / (i + 1) as f64;
    }
    r
}

/// Power coefficients of `(s - c)^m`, length `q + 1`.
fn shifted_power(c: f64, m: usize, q: usize) -> Vec<f64> {
    let mut a = vec![0.0; q + 1];
    for i in 0..=m {
        a[i] = binom(m, i) * (-c).powi((m - i) as i32);
    }
    a
}

/// Bernstein coefficients (times `mul`) of a degree-`q` power polynomial.
fn bern(a: &[f64], q: usize, mul: f64) -> Vec<f64> {
    (0..=q)
        .map(|k| {
            (0..=k)
                .map(|i| binom(k, i) * (mul / binom(q, i)) * a[i])
                .sum()
        })
        .collect()
}

/// One Bezier span on `[0, len]` whose derivative is `p·mul·g(t/len)` with
/// `g = (gx, gy, gz)` in power form. `None` if the f64 net does not
/// reproduce the differences exactly (then the stall is not exact).
fn bezier(g: [&[f64]; 3], p: usize, len: f64, off: f64) -> Option<NurbsCurve3<f64>> {
    let q = p - 1;
    let mul = [2.0, 2.0, 2.0, 6.0, 12.0, 60.0][q.min(5)];
    let b: Vec<Vec<f64>> = g.iter().map(|a| bern(a, q, mul)).collect();
    let mut pts = vec![Point3::new(off, off, off)];
    for k in 0..=q {
        let last = *pts.last().unwrap();
        let d = Vec3::new(b[0][k] * len, b[1][k] * len, b[2][k] * len);
        let next = last + d;
        let e = next - last;
        if e.x != d.x || e.y != d.y || e.z != d.z {
            return None;
        }
        pts.push(next);
    }
    let mut knots = vec![0.0; p + 1];
    knots.extend(vec![len; p + 1]);
    NurbsCurve3::<f64>::new(
        KnotVector::clamped(knots, p).unwrap(),
        pts,
        vec![1.0; p + 1],
    )
    .ok()
}

fn meters(c: &NurbsCurve3<f64>) -> (f64, Interval) {
    let ci = c.map_scalar(<Interval as Real>::from_f64);
    (c.speed_lower_bound().get(), ci.speed_lower_bound().get())
}

/// Touching stalls (no sign change of any component's projection on the
/// piece direction) at every piece boundary and inside pieces.
#[test]
fn touching_stalls_at_piece_boundaries_refuse() {
    let mut f64_pos = Vec::new();
    let mut checked = 0;
    for p in 3..=6 {
        let q = p - 1;
        for num in 0..=64 {
            let c = f64::from(num) / 64.0;
            let gx = shifted_power(c, 2, q);
            let gy = if q >= 3 {
                shifted_power(c, 3, q)
            } else {
                vec![0.0; q + 1]
            };
            let gz = vec![0.0; q + 1];
            for len in [1.0, 2f64.powi(-20), 2f64.powi(20), 3.0, 0.1] {
                for off in [0.0, 2f64.powi(27), -1e8] {
                    let Some(curve) = bezier([&gx, &gy, &gz], p, len, off) else {
                        continue;
                    };
                    checked += 1;
                    let (m, mi) = meters(&curve);
                    assert!(
                        !(mi.lo() > 0.0),
                        "p {p} stall at {c} len {len} off {off}: Interval {mi:?} certifies"
                    );
                    if m > 0.0 {
                        f64_pos.push((p, c, len, off, m));
                    }
                }
            }
        }
    }
    eprintln!(
        "checked {checked}; f64 positive at a stall: {}",
        f64_pos.len()
    );
    for r in f64_pos.iter().take(20) {
        eprintln!("  {r:?}");
    }
}

/// A near-stall: `g = ((s-c)^2, eps, 0)`; true min speed `p·mul·eps`.
#[test]
fn near_stalls_meter_at_most_the_speed() {
    let mut worst_f64: f64 = 0.0;
    let mut bad_f64 = 0;
    for p in 3..=6 {
        let q = p - 1;
        let mul = [2.0, 2.0, 2.0, 6.0, 12.0, 60.0][q.min(5)];
        for num in 0..=256 {
            let c = f64::from(num) / 256.0;
            let gx = shifted_power(c, 2, q);
            for eps in [2f64.powi(-40), 2f64.powi(-30), 2f64.powi(-50)] {
                let mut gy = vec![0.0; q + 1];
                gy[0] = eps;
                let gz = vec![0.0; q + 1];
                for len in [1.0, 2f64.powi(-20), 2f64.powi(20)] {
                    let Some(curve) = bezier([&gx, &gy, &gz], p, len, 0.0) else {
                        continue;
                    };
                    // exact true min speed (d/dt = 1/len)·p·mul·eps·len... the
                    // net was built as diffs b·len, so C' = p·mul·g(s).
                    let truth = p as f64 * mul * eps;
                    let (m, mi) = meters(&curve);
                    assert!(
                        !(mi.lo() > truth),
                        "p {p} c {c} eps {eps} len {len}: Interval {mi:?} > {truth}"
                    );
                    if m > truth {
                        bad_f64 += 1;
                        worst_f64 = worst_f64.max((m - truth) / truth);
                    }
                }
            }
        }
    }
    eprintln!("f64 above truth: {bad_f64}, worst relative excess {worst_f64:e}");
}

/// Two spans joined C0 (interior multiplicity p) with a touching stall
/// on the left of the joint, and C1 (multiplicity p-1) both sides.
#[test]
fn stalls_at_span_boundaries_refuse() {
    for p in 2..=5 {
        let q = p - 1;
        for mult in [p - 1, p] {
            if mult == 0 {
                continue;
            }
            // Left span: g = (1-s)^2 * e_x for q>=2, (1-s) e_x for q=1.
            // Build a two-span curve by knot insertion-free construction:
            // take a single Bezier of the stall at s = 1/2 and give it
            // an interior knot at 1/2 of multiplicity `mult` via
            // `insert_knot` if available; otherwise use refine.
            let m = if q >= 2 { 2 } else { 1 };
            let gx = shifted_power(0.5, m, q);
            let gy = if q >= 3 {
                shifted_power(0.5, 3, q)
            } else {
                vec![0.0; q + 1]
            };
            let gz = vec![0.0; q + 1];
            let curve = bezier([&gx, &gy, &gz], p, 1.0, 0.0).unwrap();
            let refined = curve.refine_knots(&vec![0.5; mult]).unwrap();
            for c in [&curve, &refined] {
                let (mf, mi) = meters(c);
                eprintln!("p {p} mult {mult}: f64 {mf:e} interval {mi:?}");
                assert!(!(mi.lo() > 0.0), "p {p} mult {mult}: {mi:?}");
            }
        }
    }
}

/// Pre-existing: `du = hi - lo` is formed in f64 before entering `T`.
/// A line whose second span length rounds down meters above its speed
/// at the Interval scalar.
#[test]
fn knot_difference_rounding_at_interval() {
    let mut hits = 0;
    for i in 1..2000 {
        let x = f64::from(i) * 0.000_731_3 + 1e-3;
        let du = 1.0 - x;
        // exact (1 - x) vs du: is du below the real difference?
        let e = Interval::from_f64(1.0) - Interval::from_f64(x);
        let bb = du - 1.0;
        let err = (1.0 - (du - bb)) + (-x - bb);
        if !(err > 0.0) {
            continue;
        }
        // P0 = 0, P1 = 2x (speed 2 on span 1), P2 = P1 + du (speed du/(1-x) < 1 on span 2).
        let p1 = 0.0;
        let p2 = du;
        let kv = KnotVector::clamped(vec![0.0, 0.0, x, 1.0, 1.0], 1).unwrap();
        let c = NurbsCurve3::<f64>::new(
            kv,
            vec![
                Point3::new(-2.0 * x, 0.0, 0.0),
                Point3::new(p1, 0.0, 0.0),
                Point3::new(p2, 0.0, 0.0),
            ],
            vec![1.0; 3],
        )
        .unwrap();
        let (_m, mi) = meters(&c);
        // true speed on span 2 = du / (1 - x) < 1 (exact real), meter lo:
        let true_speed_hi = (Interval::from_f64(du) / e).hi();
        if mi.lo() > true_speed_hi {
            hits += 1;
            if hits < 4 {
                eprintln!(
                    "x {x}: interval meter lo {:e} > true speed <= {:e}",
                    mi.lo(),
                    true_speed_hi
                );
            }
        }
    }
    eprintln!("interval meter above true speed on {hits} lines");
}
