//! Review-2 probe (scratch, not for merge): dump random conic-arc reach
//! cases with `span_reach_from`, `lever_from` and a replica of the circle
//! crest, for an independent high-precision truth in Python (mpmath).
#![allow(clippy::panic, clippy::cast_precision_loss, missing_docs)]

use geom::Curve3;
use geom_brep::Reach;
use geom_core::{Point3, Vec3};
use std::io::Write;
use test_utils::fuzz;

fn log(g: &mut fuzz::Rng, a: f64, b: f64) -> f64 {
    (a.ln() + (b.ln() - a.ln()) * g.unit()).exp()
}
fn unit(g: &mut fuzz::Rng) -> Vec3<f64> {
    loop {
        let v = Vec3::new(g.range(-1., 1.), g.range(-1., 1.), g.range(-1., 1.));
        let n = v.norm();
        if n > 0.2 && n < 1.0 {
            return v / n;
        }
    }
}

#[test]
#[ignore = "review probe"]
fn r2_dump_reach_cases() {
    let path = std::env::var("R2_DUMP").expect("R2_DUMP");
    let n: usize = std::env::var("R2_N").map(|s| s.parse().unwrap()).unwrap_or(20000);
    let mut out = std::io::BufWriter::new(std::fs::File::create(path).unwrap());
    let mut g = fuzz::pinned("r2_reach", std::env::var("R2_SEED").map(|s| s.parse().unwrap()).unwrap_or(7));
    for _ in 0..n {
        let r0 = log(&mut g, 1e-6, 1e6);
        let center = Point3::origin() + unit(&mut g) * [0.0, 1.0, 1e3, 1e6][g.below(4)] * if g.unit() < 0.5 { 1.0 } else { r0 };
        let axis = unit(&mut g);
        let u_ref = { let v = unit(&mut g); (v - axis * v.dot(axis)).normalize() };
        let v_ref = axis.cross(u_ref);
        let ellipse = g.unit() < 0.25;
        let (a, b) = if ellipse {
            let ecc: f64 = [0.3, 0.9, 0.999, 0.999_999][g.below(4)];
            let (mut a, mut b) = (r0, r0 * (1.0 - ecc * ecc).sqrt());
            if g.unit() < 0.2 { core::mem::swap(&mut a, &mut b); }
            if g.unit() < 0.1 { a = -a; }
            (a, b)
        } else {
            (if g.unit() < 0.1 { -r0 } else { r0 }, r0)
        };
        let carrier = if ellipse {
            Curve3::Ellipse { center, axis, major: a, minor: b, u_ref }
        } else {
            Curve3::Circle { center, axis, radius: a, u_ref }
        };
        let big = a.abs().max(b.abs());
        // pivot
        let tm = g.range(-10.0, 10.0);
        let w_in = (carrier.eval(tm) - center).normalize();
        let pivot = match g.below(9) {
            0 => carrier.eval(tm),
            1 => center,
            2 => center + axis * big * log(&mut g, 1e-6, 1e6) * if g.unit() < 0.5 { 1.0 } else { -1.0 },
            3 => center + w_in * big * log(&mut g, 1e-9, 1e6),
            4 => center + w_in * big * log(&mut g, 1e-3, 1e3) + axis * big * log(&mut g, 1e-6, 1e3),
            5 => center + unit(&mut g) * big * log(&mut g, 1e-3, 1e6),
            6 => carrier.eval(tm) + w_in * big * g.range(-1e-6, 1e-6),
            7 => center + w_in * big * (1.0 + g.range(-1e-9, 1e-9)) ,
            _ => center + axis * big * g.range(-1e-12, 1e-12) + w_in * big * log(&mut g, 1e-12, 1e-6),
        };
        // crest angle (circle formula: farthest point direction -w_in)
        let w = pivot - center;
        let tstar = (-(a * v_ref.dot(w))).atan2(-(a * u_ref.dot(w)));
        let span = match g.below(6) {
            0 => g.range(6.0, 20.0),
            1 => core::f64::consts::TAU,
            _ => log(&mut g, 1e-9, 6.3),
        };
        let sign = if g.unit() < 0.4 { -1.0 } else { 1.0 };
        let delta = [0.0, 1e-15, -1e-15, 1e-12, -1e-12, 1e-8, -1e-8][g.below(7)];
        let wrap = 2.0 * core::f64::consts::PI * [-2.0, -1.0, 0.0, 1.0, 3.0][g.below(5)];
        let (t0, t1) = match g.below(4) {
            0 => { let t0 = tstar + delta + wrap; (t0, t0 + sign * span) }
            1 => { let t1 = tstar + delta + wrap; (t1 - sign * span, t1) }
            2 => { let t0 = tstar + wrap - sign * span * g.unit(); (t0, t0 + sign * span) }
            _ => { let t0 = g.range(-20.0, 20.0); (t0, t0 + sign * span) }
        };
        let reach = Reach::Span { carrier: carrier.clone(), t0, t1 };
        let got = reach.span_reach_from(pivot);
        let lever = reach.lever_from(pivot);
        // replica of the circle crest & cap at f64
        let h = w.dot(axis);
        let rho = (w - axis * h).norm();
        let crest = (h.powi(2) + (a.abs() + rho).powi(2)).sqrt();
        let cap = w.norm() + a.abs();
        writeln!(
            out,
            "{} {:e} {:e} {:e} {:e} {:e} {:e} {:e} {:e} {:e} {:e} {:e} {:e} {:e} {:e} {:e} {:e} {:e} {:e} {:e} {:e}",
            u8::from(ellipse),
            center.x, center.y, center.z, axis.x, axis.y, axis.z, u_ref.x, u_ref.y, u_ref.z,
            a, b, t0, t1, pivot.x, pivot.y, pivot.z, got, lever, crest, cap
        )
        .unwrap();
    }
}
