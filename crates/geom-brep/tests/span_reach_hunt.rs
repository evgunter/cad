//! **A conic arc's span reach is never short of its farthest point**
//! (`Reach::span_reach_from`), hunted over random circles and ellipses
//! (eccentricities to 0.999999, a negative or swapped semi-axis), spans
//! from 1e-6 rad to two turns either way, and pivots on the arc, at its
//! centre, along and against its crests, far off its plane and just off
//! it. The farthest point is sampled and each local maximum refined by
//! golden section. The reach is never short of it to rounding, never
//! past it by more than a quarter's bulge, and never past the whole
//! turn ([`Reach::lever_from`]).

#![allow(clippy::panic, clippy::cast_precision_loss)]

test_utils::gated_to!["crates/geom-brep/src/extent.rs"];

use geom::Curve3;
use geom_brep::Reach;
use geom_core::{Point3, Vec3};
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

/// The farthest point of the arc from `p`: 1,000 samples, then each local
/// maximum refined by golden section.
fn farthest(c: &Curve3<f64>, t0: f64, t1: f64, p: Point3<f64>) -> f64 {
    let n = 1000;
    let d = |t: f64| (c.eval(t) - p).norm();
    let ts: Vec<f64> = (0..=n)
        .map(|k| t0 + (t1 - t0) * k as f64 / n as f64)
        .collect();
    let ds: Vec<f64> = ts.iter().map(|t| d(*t)).collect();
    let mut best = ds.iter().copied().fold(0.0, f64::max);
    for k in 1..n {
        if ds[k] >= ds[k - 1] && ds[k] >= ds[k + 1] {
            let (mut a, mut b) = (ts[k - 1], ts[k + 1]);
            for _ in 0..80 {
                let m1 = a + (b - a) * 0.381_966;
                let m2 = a + (b - a) * 0.618_034;
                if d(m1) < d(m2) {
                    a = m1;
                } else {
                    b = m2;
                }
            }
            best = best.max(d(0.5 * (a + b)));
        }
    }
    best
}

#[test]
fn a_conic_arcs_span_reach_is_never_short_of_its_farthest_point() {
    let mut g = fuzz::start("span_reach_hunt");
    for _ in 0..fuzz::scaled(100) {
        let r = [1e-3, 1.0, 1e3][g.below(3)];
        let center = Point3::origin() + unit(&mut g) * [0.0, 1.0, 1e3][g.below(3)];
        let axis = unit(&mut g);
        let u_ref = {
            let v = unit(&mut g);
            (v - axis * v.dot(axis)).normalize()
        };
        let ecc: f64 = [0.0, 0.5, 0.9, 0.99, 0.999, 0.999_999][g.below(6)];
        let (mut a, mut b) = (r, r * (1.0 - ecc * ecc).sqrt());
        if g.unit() < 0.2 {
            core::mem::swap(&mut a, &mut b);
        }
        if g.unit() < 0.1 {
            a = -a;
        }
        let carrier = if ecc == 0.0 && g.unit() < 0.5 {
            Curve3::Circle {
                center,
                axis,
                radius: r,
                u_ref,
            }
        } else {
            Curve3::Ellipse {
                center,
                axis,
                major: a,
                minor: b,
                u_ref,
            }
        };
        let big = a.abs().max(b.abs());
        let span = if g.unit() < 0.1 {
            g.range(6.0, 13.0)
        } else {
            log(&mut g, 1e-6, 6.3)
        };
        let t0 = g.range(-10.0, 10.0);
        let t1 = if g.unit() < 0.3 { t0 - span } else { t0 + span };
        let tm = t0 + (t1 - t0) * g.unit();
        let pivots = [
            carrier.eval(t0),
            carrier.eval(t1),
            carrier.eval(tm),
            center,
            center + (carrier.eval(tm) - center) * g.range(-50.0, 50.0),
            center - (carrier.eval(tm) - center) * log(&mut g, 1e-3, 1e6),
            center + unit(&mut g) * big * log(&mut g, 1e-3, 1e3),
            carrier.eval(tm) + axis * big * log(&mut g, 1e-6, 1e3),
            carrier.eval(tm) + u_ref * big * g.range(-1e-4, 1e-4),
        ];
        for p in pivots {
            let reach = Reach::Span {
                carrier: carrier.clone(),
                t0,
                t1,
            };
            let got = reach.span_reach_from(p);
            let far = farthest(&carrier, t0, t1, p);
            let scale = (center - Point3::origin()).norm() + (p - Point3::origin()).norm() + big;
            let tol = 16.0 * f64::EPSILON * scale;
            let slack = (1.0 - ((t1 - t0).abs().min(core::f64::consts::TAU) / 8.0).cos()) * big;
            let label = format!("{carrier:?} over [{t0}, {t1}] from {p:?}");
            assert!(
                got >= far - tol,
                "{label}: {got} falls short of the farthest point {far}; {}",
                fuzz::replay()
            );
            assert!(
                got <= far + slack + tol,
                "{label}: {got} passes the farthest point {far} by more than {slack}; {}",
                fuzz::replay()
            );
            assert!(
                got <= reach.lever_from(p),
                "{label}: {got} passes the whole turn; {}",
                fuzz::replay()
            );
        }
    }
}
