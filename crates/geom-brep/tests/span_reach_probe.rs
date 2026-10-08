//! Reviewer probe (PR 4292): hunts `Reach::span_reach_from` short of the
//! arc's farthest point. Asserts nothing; prints counts.
#![allow(clippy::panic, clippy::too_many_lines, clippy::cast_precision_loss)]

use geom::Curve3;
use geom_brep::Reach;
use geom_core::{Point3, Vec3};

struct Rng(u64);
impl Rng {
    fn u(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
    fn r(&mut self, a: f64, b: f64) -> f64 { a + (b - a) * self.u() }
    fn log(&mut self, a: f64, b: f64) -> f64 { (a.ln() + (b.ln() - a.ln()) * self.u()).exp() }
    fn unit(&mut self) -> Vec3<f64> {
        loop {
            let v = Vec3::new(self.r(-1., 1.), self.r(-1., 1.), self.r(-1., 1.));
            let n = v.norm();
            if n > 0.2 && n < 1.0 { return v / n; }
        }
    }
}

/// The farthest point of the arc from `p`: 4000 samples, then each local
/// maximum refined by golden section.
fn farthest(c: &Curve3<f64>, t0: f64, t1: f64, p: Point3<f64>) -> f64 {
    let n = 4000;
    let d = |t: f64| (c.eval(t) - p).norm();
    let ts: Vec<f64> = (0..=n).map(|k| t0 + (t1 - t0) * k as f64 / n as f64).collect();
    let ds: Vec<f64> = ts.iter().map(|t| d(*t)).collect();
    let mut best = ds.iter().copied().fold(0.0, f64::max);
    for k in 1..n {
        if ds[k] >= ds[k - 1] && ds[k] >= ds[k + 1] {
            let (mut a, mut b) = (ts[k - 1], ts[k + 1]);
            for _ in 0..80 {
                let m1 = a + (b - a) * 0.381_966;
                let m2 = a + (b - a) * 0.618_034;
                if d(m1) < d(m2) { a = m1 } else { b = m2 }
            }
            best = best.max(d(0.5 * (a + b)));
        }
    }
    best
}

#[test]
#[ignore = "reviewer probe"]
fn span_reach_probe() {
    let mut rng = Rng(0x4292);
    let (mut n, mut short, mut over_slack, mut over_turn) = (0u64, 0u64, 0u64, 0u64);
    let mut worst_short = 0.0_f64;
    let mut worst_label = String::new();
    for _ in 0..40_000 {
        let r: f64 = [1e-3, 1.0, 1e3][(rng.u() * 3.0) as usize % 3];
        let offmag = [0.0, 1.0, 1e3][(rng.u() * 3.0) as usize % 3];
        let center = Point3::origin() + rng.unit() * offmag;
        let axis = rng.unit();
        let u_ref = { let v = rng.unit(); (v - axis * v.dot(axis)).normalize() };
        let ecc: f64 = [0.0, 0.5, 0.9, 0.99, 0.999, 0.999_999][(rng.u() * 6.0) as usize % 6];
        let flip = rng.u() < 0.2;
        let (mut a, mut b) = (r, r * (1.0 - ecc * ecc).sqrt());
        if flip { core::mem::swap(&mut a, &mut b); }
        if rng.u() < 0.1 { a = -a; }
        let carrier = if ecc == 0.0 && rng.u() < 0.5 {
            Curve3::Circle { center, axis, radius: r, u_ref }
        } else {
            Curve3::Ellipse { center, axis, major: a, minor: b, u_ref }
        };
        let big = a.abs().max(b.abs());
        let span = if rng.u() < 0.1 { rng.r(6.0, 13.0) } else { rng.log(1e-6, 6.3) };
        let t0 = rng.r(-10.0, 10.0);
        let t1 = if rng.u() < 0.3 { t0 - span } else { t0 + span };
        let tm = t0 + (t1 - t0) * rng.u();
        let pivots = [
            carrier.eval(t0),
            carrier.eval(t1),
            carrier.eval(tm),
            center,
            center + (carrier.eval(tm) - center) * rng.r(-50.0, 50.0),
            center - (carrier.eval(tm) - center) * rng.log(1e-3, 1e6),
            center + rng.unit() * big * rng.log(1e-3, 1e3),
            carrier.eval(tm) + axis * big * rng.log(1e-6, 1e3),
            carrier.eval(tm) + u_ref * big * rng.r(-1e-4, 1e-4),
        ];
        for p in pivots {
            n += 1;
            let reach = Reach::Span { carrier: carrier.clone(), t0, t1 };
            let got = reach.span_reach_from(p);
            let far = farthest(&carrier, t0, t1, p);
            let scale = (center - Point3::origin()).norm() + (p - Point3::origin()).norm() + big;
            let tol = 4.0 * f64::EPSILON * scale;
            let sh = (far - got) / scale;
            if far - got > tol {
                short += 1;
            }
            if sh > worst_short {
                worst_short = sh;
                worst_label = format!("{carrier:?} [{t0}, {t1}] from {p:?}: got {got} far {far}");
            }
            let s = (t1 - t0).abs().min(core::f64::consts::TAU);
            if got > far + (1.0 - (s / 8.0).cos()) * big + tol { over_slack += 1; }
            if got > reach.lever_from(p) { over_turn += 1; }
        }
    }
    println!("PROBE n={n} short={short} over_slack={over_slack} over_turn={over_turn}");
    println!("PROBE worst short / scale = {worst_short:e}: {worst_label}");
}
