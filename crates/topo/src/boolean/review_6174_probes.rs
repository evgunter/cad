//! Review probes for PR 6174 (`flush/aimed-ray-reach`), not part of the
//! PR: the arc ball's containment, the centred quadratics' roots against
//! exact arithmetic (dumped for `scripts`-free Python checking), and the
//! order margin's sign.
#![allow(clippy::panic, clippy::print_stdout, clippy::unwrap_used)]

use super::*;
use geom_core::{Band, Tol};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// A small deterministic generator.
struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
    fn range(&mut self, a: f64, b: f64) -> f64 {
        a + (b - a) * self.next()
    }
    fn unit(&mut self) -> Vec3<f64> {
        loop {
            let v = Vec3::new(
                self.range(-1.0, 1.0),
                self.range(-1.0, 1.0),
                self.range(-1.0, 1.0),
            );
            let n = v.norm();
            if n > 0.1 && n < 1.0 {
                return v / n;
            }
        }
    }
}

/// **Every sampled point of every arc lies in its arc ball**: ellipses
/// of eccentricity up to 1e8, spans from 1e-9 to past a full turn,
/// through either apsis, placed up to 1e7 m out.
#[test]
fn review_arc_ball_holds_every_arc() {
    let mut g = Lcg(6174);
    let mut worst: f64 = 0.0;
    let mut fails = Vec::new();
    for case in 0..20_000 {
        let a = 10f64.powf(g.range(-3.0, 8.0));
        let b = a * 10f64.powf(g.range(-8.0, 0.0));
        let (a, b) = if g.next() < 0.5 { (a, b) } else { (b, a) };
        let axis = g.unit();
        let u = {
            let t = g.unit();
            let p = t - axis * t.dot(axis);
            p / p.norm()
        };
        let center = Point3::origin() + g.unit() * 10f64.powf(g.range(-2.0, 7.0));
        let carrier = if case % 7 == 0 {
            geom::Curve3::Circle {
                center,
                axis,
                radius: a,
                u_ref: u,
            }
        } else {
            geom::Curve3::Ellipse {
                center,
                axis,
                semi_u: a,
                semi_v: b,
                u_ref: u,
            }
        };
        let apsis = [0.0, core::f64::consts::FRAC_PI_2, core::f64::consts::PI][case % 3];
        let width = 10f64.powf(g.range(-9.0, 0.9));
        let t0 = if case % 2 == 0 {
            apsis - width * g.next()
        } else {
            g.range(-7.0, 7.0)
        };
        let span = (t0, t0 + width);
        let Some((mid, radius)) =
            crate::splitting::containment::conic_arc_ball(&carrier, span)
        else {
            panic!("a conic has a ball");
        };
        for k in 0..=256 {
            let t = span.0 + (span.1 - span.0) * f64::from(k) / 256.0;
            let gap = (carrier.eval(t) - mid).norm();
            worst = worst.max(gap / radius);
            if gap > radius {
                fails.push(format!(
                    "case {case} a {a:e} b {b:e} span {span:?} t {t}: {gap:e} > {radius:e}"
                ));
            }
        }
    }
    println!("REVIEW-ARC-BALL worst gap/radius {worst}; {} fails", fails.len());
    for f in fails.iter().take(10) {
        println!("REVIEW-ARC-BALL fail {f}");
    }
    assert!(fails.is_empty());
}

/// **The centred roots, dumped for an exact check**: each line is the
/// inputs and the computed roots and slack, every float in Rust's
/// round-trip `{:?}`; the Python check computes the exact roots of the
/// same quadratic on the same floats.
#[test]
fn review_centred_roots_dump() {
    let mut g = Lcg(42);
    let mut out = String::new();
    let o = Point3::origin();
    for case in 0..6000 {
        let big = [1.0, 10.0, 1e3, 1e5, 1e7][case % 5];
        let r = [1e-3, 1.0, 1e3][(case / 5) % 3];
        let kind = (case / 15) % 3;
        let axis = g.unit();
        let center = o + g.unit() * g.range(0.0, 10.0);
        // A direction, and an impact parameter as a fraction of `r`,
        // grazing on some cases.
        let d = g.unit() * [1.0, 2.5, 0.3][case % 3];
        let frac = match case % 4 {
            0 => 0.0,
            1 => g.range(0.0, 1.0),
            2 => 1.0 - 10f64.powf(g.range(-12.0, -4.0)),
            _ => g.range(0.9, 0.9999),
        };
        // A point at the impact parameter off the centre, perpendicular
        // to `d` (and, for the wall, measured perpendicular to the axis
        // only roughly: the dump records what was asked).
        let perp = {
            let t = g.unit();
            let p = t - d * (t.dot(d) / d.norm_squared());
            p / p.norm()
        };
        let near = center + perp * (frac * r);
        let q = near - d * (big / d.norm());
        let dbg = |p: Point3<f64>| format!("{:?} {:?} {:?}", p.x, p.y, p.z);
        let dbv = |v: Vec3<f64>| format!("{:?} {:?} {:?}", v.x, v.y, v.z);
        match kind {
            0 => {
                for about in [About::Start, About::Nearest] {
                    if let Ok(WallRoots::Two([t0, t1], slack)) =
                        line_wall_roots(q, d, center, axis, r, 4.0 * big, band(), about)
                    {
                        out.push_str(&format!(
                            "wall {:?} {} {} {} {} {:?} {:?} {:?} {:?}\n",
                            about,
                            dbg(q),
                            dbv(d),
                            dbg(center),
                            dbv(axis),
                            r,
                            t0,
                            t1,
                            slack
                        ));
                    }
                }
            }
            1 => {
                for about in [About::Start, About::Nearest] {
                    if let Ok(WallRoots::Two([t0, t1], slack)) =
                        line_sphere_roots(q, d, center, r, band(), about)
                    {
                        out.push_str(&format!(
                            "sphere {:?} {} {} {} {:?} {:?} {:?} {:?}\n",
                            about,
                            dbg(q),
                            dbv(d),
                            dbg(center),
                            r,
                            t0,
                            t1,
                            slack
                        ));
                    }
                }
            }
            _ => {
                let half = g.range(0.05, 1.5);
                // The apex at `center`; the line passes at `near` — move
                // the near point up the axis so it crosses a nappe.
                let up = axis * (r * g.range(-3.0, 3.0));
                let q = q + up;
                let c = line_cone_centred(q, d, center, axis, half);
                let disc = c.disc();
                if disc > 0.0 {
                    let (ts, slack) = c.roots(disc);
                    out.push_str(&format!(
                        "cone Nearest {} {} {} {} {:?} {:?} {:?} {:?} {:?}\n",
                        dbg(q),
                        dbv(d),
                        dbg(center),
                        dbv(axis),
                        half,
                        half.cos(),
                        ts[0],
                        ts[1],
                        slack
                    ));
                }
            }
        }
    }
    let path = std::env::var("REVIEW_DUMP").unwrap_or_else(|_| "/dev/null".to_owned());
    std::fs::write(path, out).unwrap();
}

/// **A slack wider than the gap drives `pair`'s magnitude margin
/// negative**: `decide_magnitude` holds that margin nonnegative by
/// construction and panics past `-escalate`.
#[test]
fn review_pair_margin_can_go_negative() {
    let body = Body::<f64>::new();
    let face = FaceKey::default();
    let hit = |at: f64, slack: f64, count: i32| RayHit {
        count,
        face,
        other: None,
        at,
        slack,
        mark: None,
    };
    let (a, b) = (hit(5.0, 1e-7, 1), hit(5.0 + 1e-8, 1e-7, -1));
    let got = std::panic::catch_unwind(|| a.pair(&body, &b, "review_order", band()).is_ok());
    println!("REVIEW-PAIR panicked: {}", got.is_err());
    assert!(got.is_err(), "the margin's sign is held nonnegative");
}
