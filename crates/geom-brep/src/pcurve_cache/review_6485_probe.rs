//! Review probe for PR 6485 (E4): an adversarial counterexample search
//! of the projected envelope for circles on the cylinder, cone and torus
//! and lines on the cone, minted DIRECTLY through `projected::project`
//! (bypassing the class gates), at deviations from 0 to far off, so the
//! envelope is read as an upper bound wherever it does not refuse.

use core::f64::consts::TAU;

use geom_core::{Bounds, Interval};
use test_utils::fuzz;

use super::harmonic_sup_rows::band;
use super::*;

fn world(surface: &Surface<f64>, p: Vec3<f64>) -> Point3<f64> {
    let Some((o, e)) = projected::chart_frame(surface) else {
        unreachable!()
    };
    o + e[0] * p.x + e[1] * p.y + e[2] * p.z
}

fn world_dir(surface: &Surface<f64>, v: Vec3<f64>) -> Vec3<f64> {
    world(surface, v) - world(surface, Vec3::zero())
}

fn framed_circle(s: &Surface<f64>, c: Vec3<f64>, n: Vec3<f64>, r: f64) -> Curve3<f64> {
    let n = n.normalize();
    let x = n.cross(Vec3::new(0.37, -0.81, 0.45)).normalize();
    let (wn, wx) = (world_dir(s, n).normalize(), world_dir(s, x).normalize());
    Curve3::circle(world(s, c), wn, r, wx)
}

/// Densely sampled `|S(P(t)) − C(t)|`, refined around the max.
fn residual(image: &Pcurve<f64>, (t0, t1): (f64, f64), c: &Curve3<f64>, s: &Surface<f64>) -> f64 {
    let at = |t: f64| {
        let q = image.eval(t);
        s.eval(q.x, q.y).distance(c.eval(t))
    };
    let n = 8192;
    let mut best = (0.0, t0);
    for i in 0..=n {
        let t = t0 + (t1 - t0) * f64::from(i) / f64::from(n);
        let v = at(t);
        if v > best.0 {
            best = (v, t);
        }
    }
    let h = (t1 - t0) / f64::from(n);
    for j in 0..=400 {
        let t = (best.1 - h + 2.0 * h * f64::from(j) / 400.0).clamp(t0, t1);
        best.0 = best.0.max(at(t));
    }
    best.0
}

fn envelope(
    image: &Pcurve<f64>,
    image_i: &Pcurve<Interval>,
    carrier: &Curve3<f64>,
    (t0, t1): (f64, f64),
    surface: &Surface<f64>,
) -> (Result<f64, PcurveCertifyError>, Result<f64, PcurveCertifyError>) {
    let f = Interval::from_f64;
    let (Some(p), Some(p_i)) = (image.projected(), image_i.projected()) else {
        unreachable!()
    };
    let a = projected::projected_envelope(
        p,
        t0,
        t1,
        &p.chart_box(t0, t1),
        carrier,
        surface,
        band(),
        None,
    )
    .map(|(t, _)| t.total());
    let b = projected::projected_envelope(
        p_i,
        f(t0),
        f(t1),
        &p_i.chart_box(f(t0), f(t1)),
        &carrier.map_scalar(f),
        &surface.map_scalar(f),
        band(),
        None,
    )
    .map(|(t, _)| t.total().hi());
    (a, b)
}

fn draw(s: &mut fuzz::Rng) -> (usize, Surface<f64>, Curve3<f64>, (f64, f64), f64) {
    let scale = s.log_uniform(1e-2, 50.0);
    let far = if s.below(2) == 0 { 0.0 } else { s.log_uniform(1.0, 1000.0) };
    let place = Point3::new(far, -0.5 * far, 0.3 * far);
    let axis = Vec3::new(s.range(-1.0, 1.0), s.range(-1.0, 1.0), s.range(-1.0, 1.0)).normalize();
    let u_ref = axis.cross(Vec3::new(0.3, 1.0, -0.2)).normalize();
    let sign = |s: &mut fuzz::Rng| if s.below(2) == 0 { 1.0 } else { -1.0 };
    // A deviation scale: from nothing to well off.
    let dev = if s.below(3) == 0 { 0.0 } else { s.log_uniform(1e-14, 1e-2) * scale };
    let jitter = |s: &mut fuzz::Rng| {
        Vec3::new(s.range(-1.0, 1.0), s.range(-1.0, 1.0), s.range(-1.0, 1.0)) * dev
    };
    let chart = s.below(4);
    let (surface, carrier) = match chart {
        0 => {
            // Cone circle: near the apex or not, either nappe, any tilt.
            let alpha = s.range(0.02, 1.5);
            let surface = Surface::Cone { apex: place, axis, half_angle: alpha, u_ref };
            let h = if s.below(2) == 0 {
                s.log_uniform(1e-6, 1e-1)
            } else {
                s.range(0.1, 3.0)
            } * scale
                * sign(s);
            let tilt = if s.below(2) == 0 { 0.0 } else { s.log_uniform(1e-8, 1.4) };
            let n = Vec3::new(0.0, tilt.sin(), tilt.cos());
            let r = (h * alpha.tan()).abs() * (1.0 + s.range(-1.0, 1.0) * dev / scale);
            let c = Vec3::new(0.0, 0.0, h) + jitter(s);
            let carrier = framed_circle(&surface, c, n, r.abs().max(1e-12));
            (surface, carrier)
        }
        1 => {
            // Torus circle: thin tube to near-horn; crest, tilted parallel,
            // Villarceau-ish, random.
            let ratio = if s.below(3) == 0 {
                1.0 - s.log_uniform(1e-4, 0.1)
            } else {
                s.log_uniform(1e-3, 0.9)
            };
            let (major, minor) = (scale, scale * ratio);
            let surface = Surface::Torus {
                center: place,
                axis,
                major_radius: major,
                minor_radius: minor,
                u_ref,
            };
            let carrier = match s.below(3) {
                0 => {
                    // crest (or any parallel) with an offset centre.
                    let theta = if s.below(2) == 0 {
                        core::f64::consts::FRAC_PI_2 * sign(s)
                    } else {
                        s.range(-3.1, 3.1)
                    };
                    let (z, rho) = (minor * theta.sin(), major + minor * theta.cos());
                    let c = Vec3::new(0.0, 0.0, z) + jitter(s);
                    framed_circle(&surface, c, Vec3::unit_z(), rho + s.range(-1.0, 1.0) * dev)
                }
                1 => {
                    let theta = s.range(-3.1, 3.1);
                    let (z, rho) = (minor * theta.sin(), major + minor * theta.cos());
                    let tau = s.log_uniform(1e-9, 1.0);
                    framed_circle(
                        &surface,
                        Vec3::new(0.0, 0.0, z) + jitter(s),
                        Vec3::new(0.0, -tau.sin(), tau.cos()),
                        rho,
                    )
                }
                _ => {
                    // Villarceau: plane through centre tilted by asin(r/R),
                    // circle of radius R centred r off the axis.
                    let tau = (minor / major).asin();
                    let n = Vec3::new(0.0, -tau.sin(), tau.cos());
                    let c = Vec3::new(minor * sign(s), 0.0, 0.0) + jitter(s);
                    framed_circle(&surface, c, n, major + s.range(-1.0, 1.0) * dev)
                }
            };
            (surface, carrier)
        }
        2 => {
            // Cylinder: circles of all radii and tilts near the wall.
            let radius = scale;
            let surface = Surface::Cylinder { origin: place, axis, radius, u_ref };
            let carrier = if s.below(2) == 0 {
                // A tilted rim.
                let tau = s.log_uniform(1e-9, 1.0);
                framed_circle(
                    &surface,
                    Vec3::new(0.0, 0.0, s.range(-1.0, 1.0) * scale) + jitter(s),
                    Vec3::new(0.0, -tau.sin(), tau.cos()),
                    radius * (1.0 + s.range(-1.0, 1.0) * tau * tau),
                )
            } else {
                let phi = s.range(-3.1, 3.1);
                let (sp, cp) = phi.sin_cos();
                let tangent = Vec3::new(-sp, cp, 0.0);
                let psi = s.range(-1.5, 1.5);
                let n = (tangent * psi.cos() + Vec3::unit_z() * psi.sin())
                    .cross(Vec3::unit_z() * psi.cos() - tangent * psi.sin());
                framed_circle(
                    &surface,
                    Vec3::new(cp * radius, sp * radius, 0.0) + jitter(s),
                    n,
                    s.log_uniform(1e-9, 1.0) * scale,
                )
            };
            (surface, carrier)
        }
        _ => {
            // Cone line: through or near the apex, near a ruling.
            let alpha = s.range(0.02, 1.5);
            let surface = Surface::Cone { apex: place, axis, half_angle: alpha, u_ref };
            let (sa, ca) = alpha.sin_cos();
            let phi = s.range(-3.1, 3.1);
            let g = Vec3::new(sa * phi.cos(), sa * phi.sin(), ca) * sign(s);
            let near = if s.below(2) == 0 { 0.0 } else { s.log_uniform(1e-9, 1e-1) * scale };
            let tilt = jitter(s) / scale;
            let p0 = Vec3::new(s.range(-1.0, 1.0), s.range(-1.0, 1.0), s.range(-1.0, 1.0)) * near;
            let carrier = Curve3::line(world(&surface, p0), world_dir(&surface, g + tilt));
            let t0 = if s.below(2) == 0 {
                -s.log_uniform(1e-6, 1.0) * scale
            } else {
                s.log_uniform(1e-6, 1.0) * scale
            };
            let t1 = t0 + s.log_uniform(1e-3, 3.0) * scale;
            return (chart, surface, carrier, (t0, t1), scale + far);
        }
    };
    let t0 = s.range(-4.0, 4.0);
    let t1 = t0 + if s.below(4) == 0 { TAU } else { s.range(0.05, 6.0) };
    (chart, surface, carrier, (t0, t1), scale + far)
}

#[test]
fn review_6485_envelope_search() {
    let mut s = fuzz::start("pcurve_cache::review_6485_probe");
    let mut tally = [[0usize; 4]; 4];
    let mut worst = [0.0f64; 4];
    let mut reds = Vec::new();
    let f = Interval::from_f64;
    for trial in 0..fuzz::scaled(2000) {
        let (chart, surface, carrier, (t0, t1), size) = draw(&mut s);
        let b = band();
        let (Ok(p), Ok(p_i)) = (
            projected::project(&carrier, Some((t0, t1)), &surface, b),
            projected::project(
                &carrier.map_scalar(f),
                Some((f(t0), f(t1))),
                &surface.map_scalar(f),
                b,
            ),
        ) else {
            tally[chart][0] += 1;
            continue;
        };
        let (image, image_i) = (Pcurve::Projected(Box::new(p)), Pcurve::Projected(Box::new(p_i)));
        let (e, ei) = envelope(&image, &image_i, &carrier, (t0, t1), &surface);
        if e.is_err() && ei.is_err() {
            tally[chart][1] += 1;
            continue;
        }
        tally[chart][2] += 1;
        let sup = residual(&image, (t0, t1), &carrier, &surface);
        let noise = 256.0 * f64::EPSILON * size;
        for (lane, env) in [("f64", e.as_ref().ok()), ("Interval", ei.as_ref().ok())] {
            let Some(&env) = env else { continue };
            if sup > 1e4 * noise {
                worst[chart] = worst[chart].max(sup / env);
            }
            if env < sup - noise {
                reds.push(format!(
                    "trial {trial} chart {chart} {lane}: envelope {env:e} < residual {sup:e} \
                     (noise {noise:e}); span ({t0}, {t1}); surface {surface:?}; carrier {carrier:?}"
                ));
            }
        }
        if env_under_zero(&e) && sup > b.zero() {
            tally[chart][3] += 1;
        }
    }
    for (chart, t) in tally.iter().enumerate() {
        println!(
            "[probe] chart {chart}: {} no image, {} refused, {} read, {} f64 under zero while off; worst sup/env {:.3}",
            t[0], t[1], t[2], t[3], worst[chart]
        );
    }
    for r in reds.iter().take(20) {
        println!("[red] {r}");
    }
    println!("{}", fuzz::replay());
    assert!(reds.is_empty(), "{} counterexamples", reds.len());
}

fn env_under_zero(e: &Result<f64, PcurveCertifyError>) -> bool {
    e.as_ref().is_ok_and(|&x| x < band().zero())
}

/// Targeted: thin-tube tori, crest circles through the full mint path
/// (`chart_pcurve_over`) and `PcurveCache::certify` at f64; reports
/// every certificate whose envelope sits under the sampled residual,
/// and every one certified while the residual is past `zero`.
#[test]
fn review_6485_thin_torus_crest_f64() {
    let b = band();
    println!("[thin] zero {:e} escalate {:e}", b.zero(), b.escalate());
    let mut under = 0;
    let mut wrong = 0;
    let mut certified = 0;
    let mut worst_gap = 0.0f64;
    for &major in &[1.0, 5.0, 20.0, 50.0] {
        for &ratio in &[1e-2, 3e-3, 1e-3, 3e-4, 1e-4] {
            let minor = major * ratio;
            let surface = Surface::Torus {
                center: Point3::new(0.3, -0.7, 0.2),
                axis: Vec3::new(0.2, -0.4, 1.0).normalize(),
                major_radius: major,
                minor_radius: minor,
                u_ref: Vec3::new(0.2, -0.4, 1.0).normalize().cross(Vec3::unit_x()).normalize(),
            };
            for i in 0..40 {
                // the crest circle's distance is ~δ²/2r: sweep it across zero.
                let target = b.zero() * (0.2 + 0.05 * f64::from(i));
                let delta = (2.0 * minor * target).sqrt();
                let phi = 0.3 + 0.1 * f64::from(i);
                let c = Vec3::new(phi.cos() * delta, phi.sin() * delta, minor);
                let carrier = framed_circle(&surface, c, Vec3::unit_z(), major);
                let span = (0.0, TAU);
                let Ok(image) = chart_pcurve_over(&carrier, span.0, span.1, &surface, b) else {
                    continue;
                };
                if image.projected().is_none() {
                    continue;
                }
                let Ok(cache) = PcurveCache::certify(image.clone(), span.0, span.1, &carrier, &surface, b)
                else {
                    continue;
                };
                certified += 1;
                let env = cache.certificate().envelope;
                let sup = residual(&image, span, &carrier, &surface);
                let gap = sup - env;
                worst_gap = worst_gap.max(gap);
                if gap > 1e3 * f64::EPSILON * major {
                    under += 1;
                    println!("[thin-under] R {major} r {minor:e} target {target:e}: envelope {env:e} < residual {sup:e}");
                }
                if sup > b.zero() {
                    wrong += 1;
                    println!("[thin-wrong] R {major} r {minor:e}: certified at f64, envelope {env:e}, residual {sup:e} past zero");
                }
            }
        }
    }
    println!("[thin] certified {certified}, under {under}, past zero {wrong}, worst gap {worst_gap:e}");
}

/// One thin-torus crest circle: the f64 and Interval terms, side by side.
#[test]
fn review_6485_thin_torus_terms() {
    let b = band();
    let f = Interval::from_f64;
    for (major, minor, target) in [(50.0, 5e-3, 2e-10), (1.0, 1e-4, 8.5e-13), (20.0, 2e-2, 5e-10)] {
        let surface = Surface::Torus {
            center: Point3::new(0.3, -0.7, 0.2),
            axis: Vec3::new(0.2, -0.4, 1.0).normalize(),
            major_radius: major,
            minor_radius: minor,
            u_ref: Vec3::new(0.2, -0.4, 1.0).normalize().cross(Vec3::unit_x()).normalize(),
        };
        let delta = (2.0 * minor * target).sqrt();
        let c = Vec3::new(0.7 * delta, 0.71414284285 * delta, minor);
        let carrier = framed_circle(&surface, c, Vec3::unit_z(), major);
        let span = (0.0, TAU);
        let p = projected::project(&carrier, Some(span), &surface, b).unwrap();
        let p_i = projected::project(&carrier.map_scalar(f), Some((f(0.0), f(TAU))), &surface.map_scalar(f), b).unwrap();
        let (t, _) = projected::projected_envelope(&p, 0.0, TAU, &p.chart_box(0.0, TAU), &carrier, &surface, b, None).unwrap();
        let (ti, _) = projected::projected_envelope(&p_i, f(0.0), f(TAU), &p_i.chart_box(f(0.0), f(TAU)), &carrier.map_scalar(f), &surface.map_scalar(f), b, None).unwrap();
        let image = Pcurve::Projected(Box::new(p));
        let sup = residual(&image, span, &carrier, &surface);
        println!("[terms] R {major} r {minor:e} target {target:e}: residual {sup:e}");
        for term in EnvelopeTerm::ALL {
            println!("    {term:?}: f64 {:e}  Interval hi {:e}", t.0[term.slot()], ti.0[term.slot()].hi());
        }
        println!("    total: f64 {:e}  Interval hi {:e}", t.total(), ti.total().hi());
    }
}

/// `PcurveCache::certify` alone (no mint-time incidence read) on a
/// projected crest-circle row standing past `zero` on a thin torus.
#[test]
fn review_6485_thin_torus_certify_door_past_zero() {
    let b = band();
    let f = Interval::from_f64;
    let mut wrong = 0;
    for &major in &[1.0, 5.0, 20.0, 50.0] {
        for &ratio in &[1e-2, 1e-3, 1e-4] {
            let minor = major * ratio;
            let surface = Surface::Torus {
                center: Point3::new(0.3, -0.7, 0.2),
                axis: Vec3::new(0.2, -0.4, 1.0).normalize(),
                major_radius: major,
                minor_radius: minor,
                u_ref: Vec3::new(0.2, -0.4, 1.0).normalize().cross(Vec3::unit_x()).normalize(),
            };
            for i in 0..30 {
                let target = b.zero() * (1.2 + 0.2 * f64::from(i));
                let delta = (2.0 * minor * target).sqrt();
                let phi = 0.3 + 0.1 * f64::from(i);
                let c = Vec3::new(phi.cos() * delta, phi.sin() * delta, minor);
                let carrier = framed_circle(&surface, c, Vec3::unit_z(), major);
                let span = (0.0, TAU);
                let Ok(p) = projected::project(&carrier, Some(span), &surface, b) else { continue };
                let image = Pcurve::Projected(Box::new(p));
                let Ok(cache) = PcurveCache::certify(image.clone(), span.0, span.1, &carrier, &surface, b) else {
                    continue;
                };
                let sup = residual(&image, span, &carrier, &surface);
                let env = cache.certificate().envelope;
                let noise = 1e3 * f64::EPSILON * major;
                if sup > b.zero() + noise {
                    wrong += 1;
                    let interval = projected::project(&carrier.map_scalar(f), Some((f(0.0), f(TAU))), &surface.map_scalar(f), b)
                        .ok()
                        .map(|p| PcurveCache::certify(Pcurve::Projected(Box::new(p)), f(0.0), f(TAU), &carrier.map_scalar(f), &surface.map_scalar(f), b).is_ok());
                    println!("[door-wrong] R {major} r {minor:e}: f64 certify Ok, envelope {env:e}, residual {sup:e} = {:.2}·zero (Interval certifies: {interval:?})", sup / b.zero());
                }
            }
        }
    }
    println!("[door] wrong {wrong}");
}
