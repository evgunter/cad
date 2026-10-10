//! Review probes for PR 4518 (`Affine3::rotate_point_about_axis`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::cast_precision_loss)]

use geom_core::{Affine3, Bounds, Dual64, Interval, Point3, Real, Vec3};

/// A small deterministic generator (xorshift).
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * self.next()
    }
}

fn old<T: geom_core::Real>(q: Point3<T>, n: Vec3<T>, a: T, p: Point3<T>) -> Point3<T> {
    Affine3::rotation_about_axis(q, n, a).transform_point(p)
}

/// Origin axis: new == old, bit for bit, at f64 / Dual64 / Interval.
#[test]
fn review_origin_axis_bit_identity() {
    let mut r = Rng(0x9e37_79b9_7f4a_7c15);
    let (mut f, mut fz, mut d, mut dz, mut i, mut total) = (0, 0, 0, 0, 0, 0);
    let mut first = Vec::new();
    for k in 0..200_000 {
        let pick = |r: &mut Rng, s: f64| {
            let v = r.range(-s, s);
            if r.next() < 0.15 { 0.0 } else if r.next() < 0.05 { -0.0 } else { v }
        };
        let scale = [1.0, 1e3, 1e6][k % 3];
        let p = [pick(&mut r, scale), pick(&mut r, scale), pick(&mut r, scale)];
        let n = [pick(&mut r, 1.0), pick(&mut r, 1.0), r.range(0.1, 1.0)];
        let ang = match k % 7 {
            0 => 0.0,
            1 => core::f64::consts::PI,
            2 => core::f64::consts::TAU,
            _ => r.range(-7.0, 7.0),
        };
        total += 1;
        let o = Point3::new(0.0, 0.0, 0.0);
        let pp = Point3::new(p[0], p[1], p[2]);
        let nn = Vec3::new(n[0], n[1], n[2]);
        let a = Affine3::rotate_point_about_axis(o, nn, ang, pp);
        let b = old(o, nn, ang, pp);
        for (x, y) in [(a.x, b.x), (a.y, b.y), (a.z, b.z)] {
            if x.to_bits() != y.to_bits() {
                if x == y { fz += 1 } else { f += 1; if first.len() < 5 { first.push(format!("f64 {p:?} {n:?} {ang}: {x:e} vs {y:e}")); } }
            }
        }
        // Dual, varying the angle.
        let dl = |v: f64| Dual64::constant(v);
        let od = Point3::new(dl(0.0), dl(0.0), dl(0.0));
        let pd = Point3::new(dl(p[0]), dl(p[1]), dl(p[2]));
        let nd = Vec3::new(dl(n[0]), dl(n[1]), dl(n[2]));
        let ad = Dual64::variable(ang);
        let a = Affine3::rotate_point_about_axis(od, nd, ad, pd);
        let b = old(od, nd, ad, pd);
        for (x, y) in [(a.x, b.x), (a.y, b.y), (a.z, b.z)] {
            for (u, v) in [(x.value, y.value), (x.deriv, y.deriv)] {
                if u.to_bits() != v.to_bits() {
                    if u == v { dz += 1 } else { d += 1; if first.len() < 10 { first.push(format!("dual {p:?} {n:?} {ang}: {u:e} vs {v:e}")); } }
                }
            }
        }
        let il = Interval::from_f64;
        let oi = Point3::new(il(0.0), il(0.0), il(0.0));
        let pi = Point3::new(il(p[0]), il(p[1]), il(p[2]));
        let ni = Vec3::new(il(n[0]), il(n[1]), il(n[2]));
        let ai = il(ang);
        let a = Affine3::rotate_point_about_axis(oi, ni, ai, pi);
        let b = old(oi, ni, ai, pi);
        for (x, y) in [(a.x, b.x), (a.y, b.y), (a.z, b.z)] {
            if x.lo() != y.lo() || x.hi() != y.hi() {
                i += 1;
                if first.len() < 15 { first.push(format!("iv {p:?} {n:?} {ang}: {x:?} vs {y:?}")); }
            }
        }
    }
    println!("origin axis over {total}: f64 value mismatches {f}, f64 signed-zero-only {fz}, dual {d}, dual signed-zero-only {dz}, interval endpoint mismatches {i}");
    for s in &first { println!("  {s}"); }
}

/// Inclusion: wide axis / angle / point boxes contain the point
/// enclosures of samples inside them.
#[test]
fn review_interval_enclosure_contains_samples() {
    let mut r = Rng(0x1234_5678_9abc_def1);
    let mut bad = 0;
    let mut worst_ratio: f64 = 0.0;
    for k in 0..4000 {
        let scale = [1.0, 1e3, 1e5][k % 3];
        let w = [1e-12, 1e-6, 1e-2, 0.3][k % 4];
        let c = |r: &mut Rng, s: f64| r.range(-s, s);
        let qc = [c(&mut r, scale), c(&mut r, scale), c(&mut r, scale)];
        let pc = [qc[0] + c(&mut r, 3.0), qc[1] + c(&mut r, 3.0), qc[2] + c(&mut r, 3.0)];
        let nc = [c(&mut r, 1.0), c(&mut r, 1.0), r.range(0.5, 1.0)];
        let ac = c(&mut r, 7.0);
        let bx = |v: f64, h: f64| Interval::from_bounds(v - h, v + h);
        let wq = Point3::new(bx(qc[0], w), bx(qc[1], w), bx(qc[2], w));
        let wp = Point3::new(bx(pc[0], w), bx(pc[1], w), bx(pc[2], w));
        let wn = Vec3::new(bx(nc[0], w * 0.1), bx(nc[1], w * 0.1), bx(nc[2], w * 0.1));
        let wa = bx(ac, w);
        let wide = Affine3::rotate_point_about_axis(wq, wn, wa, wp);
        for _ in 0..20 {
            let s = |r: &mut Rng, v: f64, h: f64| Interval::from_f64(r.range(v - h, v + h));
            let q = Point3::new(s(&mut r, qc[0], w), s(&mut r, qc[1], w), s(&mut r, qc[2], w));
            let p = Point3::new(s(&mut r, pc[0], w), s(&mut r, pc[1], w), s(&mut r, pc[2], w));
            let n = Vec3::new(s(&mut r, nc[0], w * 0.1), s(&mut r, nc[1], w * 0.1), s(&mut r, nc[2], w * 0.1));
            let a = s(&mut r, ac, w);
            let pt = Affine3::rotate_point_about_axis(q, n, a, p);
            for (big, small) in [(wide.x, pt.x), (wide.y, pt.y), (wide.z, pt.z)] {
                if !(big.lo() <= small.lo() && small.hi() <= big.hi()) {
                    bad += 1;
                }
            }
        }
        // Width relative to the old composite spelling at the same boxes.
        let ow = old(wq, wn, wa, wp);
        let wd = |e: Interval| e.hi() - e.lo();
        let ratio = wd(wide.x).max(wd(wide.y)).max(wd(wide.z)) / wd(ow.x).max(wd(ow.y)).max(wd(ow.z));
        worst_ratio = worst_ratio.max(ratio);
    }
    println!("inclusion failures {bad}; worst new/old width ratio {worst_ratio:e}");
    assert_eq!(bad, 0);
}

/// Emit f64 cases for an mpmath reference (python), new vs old.
#[test]
fn review_emit_f64_cases() {
    let mut r = Rng(0xdead_beef_cafe_f00d);
    let mut out = String::new();
    for k in 0..3000 {
        let scale = [0.0, 1.0, 1e3, 1e5, 3.7e7][k % 5];
        let rad = [1e-3, 1.0, 1e3, 2.3e6][(k / 5) % 4];
        let c = |r: &mut Rng, s: f64| r.range(-s, s);
        let q = [c(&mut r, scale), c(&mut r, scale), c(&mut r, scale)];
        let p = [q[0] + c(&mut r, rad), q[1] + c(&mut r, rad), q[2] + c(&mut r, rad)];
        let n = [c(&mut r, 1.0), c(&mut r, 1.0), r.range(0.2, 1.0)];
        let a = c(&mut r, 7.0);
        let qq = Point3::new(q[0], q[1], q[2]);
        let pp = Point3::new(p[0], p[1], p[2]);
        let nn = Vec3::new(n[0], n[1], n[2]);
        let x = Affine3::rotate_point_about_axis(qq, nn, a, pp);
        let y = old(qq, nn, a, pp);
        let h = |v: f64| format!("{:016x}", v.to_bits());
        let vals: Vec<String> = q.iter().chain(&p).chain(&n).chain([a].iter())
            .chain([x.x, x.y, x.z, y.x, y.y, y.z].iter()).map(|v| h(*v)).collect();
        out.push_str(&vals.join(" "));
        out.push('\n');
    }
    std::fs::write(std::env::var("REVIEW_OUT").unwrap_or("/tmp/review_cases.txt".into()), out).unwrap();
}
