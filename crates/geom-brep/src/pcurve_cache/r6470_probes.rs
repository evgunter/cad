//! PR 6470 review (E2) probes: a counterexample search on the projected
//! line lane's check-4 envelope, against a frame-relative residual.
use geom_core::{Bounds, Interval};
use test_utils::fuzz;

use super::harmonic_sup_rows::band;
use super::*;

fn unit3(s: &mut fuzz::Rng) -> Vec3<f64> {
    loop {
        let v = Vec3::new(s.range(-1.0, 1.0), s.range(-1.0, 1.0), s.range(-1.0, 1.0));
        let n = v.norm();
        if n > 0.2 && n < 1.0 {
            return v * (1.0 / n);
        }
    }
}

/// Frame-relative residual sup of `image` against the line `o + q + d·t`
/// on the cylinder `(o, axis, R, u_ref)`, sampled densely.
#[allow(clippy::too_many_arguments)]
fn residual(
    image: &Pcurve<f64>,
    (t0, t1): (f64, f64),
    q: Vec3<f64>,
    d: Vec3<f64>,
    axis: Vec3<f64>,
    u_ref: Vec3<f64>,
    r: f64,
) -> f64 {
    let cv = axis.cross(u_ref);
    let n = 1 << 14;
    (0..=n)
        .map(|i| {
            let t = t0 + (t1 - t0) * f64::from(i) / f64::from(n);
            let p = image.eval(t);
            let s = (u_ref * p.x.cos() + cv * p.x.sin()) * r + axis * p.y;
            (s - (q + d * t)).norm()
        })
        .fold(0.0, f64::max)
}

/// Draws lines near and far from rulings, long spans, far origins, R
/// from band scale to large, azimuths on the seam; reports the worst
/// envelope/residual ratio and any draw whose certified envelope is
/// under the residual.
#[test]
fn r6470_line_envelope_counterexample_search() {
    let seed: u64 = std::env::var("R6470_SEED")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0x6470);
    let draws: usize = std::env::var("R6470_DRAWS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(4000);
    let mut s = fuzz::Rng::from_seed(seed);
    let b = band();
    let f = Interval::from_f64;
    let (mut projected, mut certified, mut certified_i, mut bad) = (0, 0, 0, 0);
    let mut worst = f64::INFINITY;
    let mut tally: std::collections::BTreeMap<String, usize> = Default::default();
    let mut log = String::new();
    let mut outcome = |draw: usize, what: String, params: &str| {
        use std::fmt::Write as _;
        let _ = writeln!(log, "{draw} {what} :: {params}");
    };
    for draw in 0..draws {
        let axis = unit3(&mut s);
        let u_ref = axis.cross(unit3(&mut s)).normalize();
        let cv = axis.cross(u_ref);
        let r = s.log_uniform(b.escalate() * 0.5, 1e4);
        let o = if s.below(3) == 0 {
            Point3::new(
                s.range(-1e6, 1e6),
                s.range(-1e6, 1e6),
                s.range(-1e6, 1e6),
            )
        } else {
            Point3::new(s.range(-2.0, 2.0), s.range(-2.0, 2.0), s.range(-2.0, 2.0))
        };
        let phi = match s.below(4) {
            0 => std::f64::consts::PI - s.log_uniform(1e-14, 1e-3),
            1 => -std::f64::consts::PI + s.log_uniform(1e-14, 1e-3),
            _ => s.range(-4.0, 4.0),
        };
        let radial = u_ref * phi.cos() + cv * phi.sin();
        let tang = axis.cross(radial);
        // Lean along the wall and out of it.
        let tilt = s.log_uniform(1e-14, 0.5);
        let out = if s.below(2) == 0 {
            0.0
        } else {
            s.log_uniform(1e-14, 1e-2) * if s.below(2) == 0 { 1.0 } else { -1.0 }
        };
        let off = s.range(-1.0, 1.0) * b.zero() * if s.below(4) == 0 { 10.0 } else { 0.9 };
        let speed = s.log_uniform(1e-2, 1e2);
        let dir = (axis + tang * tilt * if s.below(2) == 0 { 1.0 } else { -1.0 } + radial * out)
            * speed;
        let half = s.log_uniform(1e-4, 1e3) / speed;
        let c = s.range(-1.0, 1.0) * half;
        let span = (c - half, c + half);
        let at = o + radial * (r + off) + axis * s.range(-3.0, 3.0);
        let carrier = Curve3::line(at, dir);
        let q = at - o;
        // A stored image from a mildly moved line half the time.
        let moved = s.below(2) == 0;
        let (dp, dd) = if moved {
            (
                unit3(&mut s) * (b.zero() * s.range(0.0, 0.3)),
                unit3(&mut s) * (b.zero() * s.range(0.0, 0.3) / (half * speed).max(1.0)),
            )
        } else {
            (Vec3::zero(), Vec3::zero())
        };
        let stored_carrier = Curve3::line(at + dp, dir + dd);
        let params = format!("R={r:.3e} tilt={tilt:.3e} out={out:.3e} off={off:.3e} half={half:.3e} speed={speed:.3e} o={:.1e} phi={phi:.4} moved={moved}", (o - Point3::origin()).norm());
        let image = match chart_pcurve_over(&stored_carrier, span.0, span.1, &surface_of(o, axis, r, u_ref), b) {
            Ok(im) => im,
            Err(e) => {
                *tally.entry(format!("derive: {}", short(&e))).or_default() += 1;
                outcome(draw, format!("derive-refused {}", short(&e)), &params);
                continue;
            }
        };
        if image.projected().is_none() {
            *tally.entry("derive: closed form".into()).or_default() += 1;
            let surface = surface_of(o, axis, r, u_ref);
            let a = PcurveCache::certify(image.clone(), span.0, span.1, &carrier, &surface, b).is_ok();
            let bi = chart_pcurve_over(&stored_carrier.map_scalar(f), f(span.0), f(span.1), &surface.map_scalar(f), b)
                .ok()
                .and_then(|im| PcurveCache::certify(im, f(span.0), f(span.1), &carrier.map_scalar(f), &surface.map_scalar(f), b).ok())
                .is_some();
            outcome(draw, format!("closed f64={a} interval={bi}"), &params);
            continue;
        }
        projected += 1;
        let surface = surface_of(o, axis, r, u_ref);
        let sup = residual(&image, span, q, dir, axis, u_ref, r);
        let scale = (q.norm() + dir.norm() * span.0.abs().max(span.1.abs())).max(r);
        let slack = 64.0 * f64::EPSILON * scale;
        let mut res = (false, false);
        match PcurveCache::certify(image.clone(), span.0, span.1, &carrier, &surface, b) {
            Ok(cache) => {
                certified += 1;
                res.0 = true;
                let env = cache.certificate().envelope;
                if sup > 1e3 * slack && env / sup < worst {
                    worst = env / sup;
                    eprintln!("worst so far draw {draw}: env {env:e} residual {sup:e} slack {slack:e} R {r:e} tilt {tilt:e} out {out:e} half {half:e} speed {speed:e} moved {moved}");
                }
                if env < sup - slack {
                    bad += 1;
                    eprintln!(
                        "COUNTEREXAMPLE f64 draw {draw}: env {env:e} < residual {sup:e} \
                         (R {r:e}, tilt {tilt:e}, out {out:e}, off {off:e}, half {half:e}, speed {speed:e}, |o| {:e}, phi {phi})",
                        (o - Point3::origin()).norm()
                    );
                }
            }
            Err(e) => *tally.entry(format!("f64 certify: {}", short(&e))).or_default() += 1,
        }
        let image_i = chart_pcurve_over(
            &stored_carrier.map_scalar(f),
            f(span.0),
            f(span.1),
            &surface.map_scalar(f),
            b,
        );
        if let Ok(image_i) = image_i {
            match PcurveCache::certify(
                image_i,
                f(span.0),
                f(span.1),
                &carrier.map_scalar(f),
                &surface.map_scalar(f),
                b,
            ) {
                Ok(cache) => {
                    certified_i += 1;
                    res.1 = true;
                    let hi = cache.certificate().envelope.hi();
                    let ulps = (sup - hi) / (f64::EPSILON * scale);
                    if ulps > 0.5 {
                        eprintln!("INTERVAL UNDER draw {draw}: by {ulps:.2} eps*scale (hi {hi:e} sup {sup:e} R {r:e} |o| {:e} moved {moved})", (o - Point3::origin()).norm());
                    }
                    if hi < sup - slack {
                        bad += 1;
                        eprintln!(
                            "COUNTEREXAMPLE Interval draw {draw}: env {hi:e} < residual {sup:e} \
                             (R {r:e}, tilt {tilt:e}, out {out:e}, half {half:e}, |o| {:e})",
                            (o - Point3::origin()).norm()
                        );
                    }
                }
                Err(e) => *tally.entry(format!("Interval certify: {}", short(&e))).or_default() += 1,
            }
        } else if let Err(e) = image_i {
            *tally.entry(format!("Interval derive: {}", short(&e))).or_default() += 1;
        }
        outcome(draw, format!("projected f64={} interval={}", res.0, res.1), &params);
    }
    if let Ok(path) = std::env::var("R6470_OUT") {
        std::fs::write(path, &log).unwrap();
    }
    eprintln!(
        "seed {seed:#x}: draws {draws}, projected {projected}, certified f64 {certified}, \
         Interval {certified_i}, counterexamples {bad}, worst env/residual {worst:e}"
    );
    for (k, v) in &tally {
        eprintln!("  {v:6}  {k}");
    }
    assert_eq!(bad, 0);
}

fn surface_of(o: Point3<f64>, axis: Vec3<f64>, radius: f64, u_ref: Vec3<f64>) -> Surface<f64> {
    Surface::Cylinder {
        origin: o,
        axis,
        radius,
        u_ref,
    }
}

fn short(e: &PcurveCertifyError) -> String {
    let s = format!("{e:?}");
    let cut = s.find("cause").unwrap_or(s.len()).min(90);
    s.chars().take(cut).collect()
}
