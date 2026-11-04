//! Delta-verifier probe for PR 6485 (fix pass U). Not for merge.
//!
//! U2's provenance question: a STORED projected row on a circle or a
//! line, re-checked at rest (or certified at a door that was handed the
//! row), whose image is not the edge carrier's — the carrier itself on
//! the chart, so the incidence read passes — refuses at check 4 on a
//! term other than `FidelityU`/`FidelityV`. The orchestrator's ruling:
//! such a row keeps its own term's ending (a defect at rest); only the
//! mint's own fresh image takes the lane's gap.

use core::f64::consts::TAU;

use geom_core::{Bounds, Interval};

use super::harmonic_sup_rows::{band, zero};
use super::*;

fn rotate_about(p: Point3<f64>, o: Point3<f64>, k: Vec3<f64>, ang: f64) -> Point3<f64> {
    let v = p - o;
    let (s, c) = ang.sin_cos();
    o + (v * c + k.cross(v) * s + k * (k.dot(v) * (1.0 - c)))
}
fn rotate_dir(v: Vec3<f64>, k: Vec3<f64>, ang: f64) -> Vec3<f64> {
    let (s, c) = ang.sin_cos();
    v * c + k.cross(v) * s + k * (k.dot(v) * (1.0 - c))
}

/// The carrier turned about the chart's axis by `ang`: on a surface of
/// revolution, exactly as on the chart as before.
fn turned(carrier: &Curve3<f64>, surface: &Surface<f64>, ang: f64) -> Curve3<f64> {
    let (o, k) = match *surface {
        Surface::Cylinder { origin, axis, .. } => (origin, axis),
        Surface::Cone { apex, axis, .. } => (apex, axis),
        Surface::Torus { center, axis, .. } | Surface::Sphere { center, axis, .. } => {
            (center, axis)
        }
        _ => unreachable!(),
    };
    let k = k.normalize();
    match carrier {
        Curve3::Circle(geom::Circle {
            frame:
                geom::Frame {
                    origin: center,
                    axis: n,
                    u_ref: x,
                },
            radius,
        }) => Curve3::circle(
            rotate_about(*center, o, k, ang),
            rotate_dir(*n, k, ang),
            *radius,
            rotate_dir(*x, k, ang),
        ),
        Curve3::Line(geom::Line { origin, dir }) => {
            Curve3::line(rotate_about(*origin, o, k, ang), rotate_dir(*dir, k, ang))
        }
        _ => unreachable!(),
    }
}

fn describe(scalar: &str, what: &str, got: &Result<String, PcurveCertifyError>) {
    use crate::recourse::Reading;
    match got {
        Ok(s) => println!("[delta-u2] {what} {scalar}: CERTIFIED {s}"),
        Err(e) => {
            let k: String = format!("{e:?}").chars().take(160).collect();
            println!("[delta-u2] {what} {scalar}: {k}");
            println!(
                "[delta-u2]     at rest: {}",
                e.ending(Reading::AtRest).chars().take(260).collect::<String>()
            );
        }
    }
}

/// Stored rows of a carrier `A`, re-checked against the edge carrier `B`
/// = `A` turned about the chart's axis (so `B` is on the chart as much as
/// `A` is, and the stored row is not `B`'s): at `f64` and at `Interval`,
/// through `recertify` (the at-rest door) on a cache certified for `A`.
#[test]
fn review_6485_delta_stored_foreign_row_at_rest() {
    let f = Interval::from_f64;
    let b = band();
    let mut rows: Vec<(String, Surface<f64>, Curve3<f64>, (f64, f64))> = Vec::new();
    // A cylinder's tilted circle of in-band radius change (none of its
    // classes), via the PR's own fixtures' shapes.
    let axis = Vec3::new(0.2, 0.1, 1.0).normalize();
    let u_ref = axis.cross(Vec3::new(0.3, 1.0, -0.2)).normalize();
    let cone = Surface::Cone {
        apex: Point3::new(0.3, -0.2, 0.5),
        axis,
        half_angle: 1.1,
        u_ref,
    };
    // A cone section circle (no rim) in band: generic tilted circle near
    // the cone, minted through chart_pcurve_over below.
    let torus = Surface::Torus {
        center: Point3::new(0.3, -0.2, 0.5),
        axis,
        major_radius: 2.0,
        minor_radius: 0.5,
        u_ref,
    };
    // A crest circle of the torus (centre `delta` off the axis).
    let crest = {
        let Some((o, e)) = projected::chart_frame(&torus) else {
            unreachable!()
        };
        let delta = (2.0 * 0.5 * 0.4 * zero()).sqrt();
        let c = o + e[0] * (0.3_f64.cos() * delta) + e[1] * (0.3_f64.sin() * delta) + e[2] * 0.5;
        Curve3::circle(c, e[2], 2.0, e[0])
    };
    rows.push(("torus crest".into(), torus.clone(), crest, (1.0, 3.9)));
    // A cone ruling read clear of the apex, anchored in band off it.
    {
        let Some((o, e)) = projected::chart_frame(&cone) else {
            unreachable!()
        };
        let (sa, ca) = 1.1_f64.sin_cos();
        let g = e[0] * (sa * 0.7_f64.cos()) + e[1] * (sa * 0.7_f64.sin()) + e[2] * ca;
        let line = Curve3::line(o + e[2] * (0.5 * zero() / ca), g);
        rows.push(("cone ruling".into(), cone.clone(), line, (0.5, 2.0)));
    }
    // A cone tilted section circle (the rows' `cone_section_circle`
    // shape, tilted so it stands ~0.4 zero off).
    {
        let Some((o, e)) = projected::chart_frame(&cone) else {
            unreachable!()
        };
        let (alpha, h) = (1.1_f64, 2.0_f64);
        let dist = |c: &Curve3<f64>| {
            (0..2048)
                .map(|i| {
                    let p = c.eval(f64::from(i) * TAU / 2048.0) - o;
                    let (x, y, z) = (p.dot(e[0]), p.dot(e[1]), p.dot(e[2]));
                    (x.hypot(y) * alpha.cos() - z.abs() * alpha.sin()).abs()
                })
                .fold(0.0, f64::max)
        };
        let section = |tau: f64| {
            let (sa, ca) = alpha.sin_cos();
            let (st, ct) = tau.sin_cos();
            let n = Vec3::new(0.0, -st, ct);
            let vertex = |s_phi: f64| {
                let g = Vec3::new(0.0, sa * s_phi, ca);
                g * (h * ct / n.dot(g))
            };
            let (p, q) = (vertex(1.0), vertex(-1.0));
            let m = (p + q) * 0.5;
            let semi_major = (p - q).norm() * 0.5;
            let semi_minor = ((m.z * sa / ca).powi(2) - m.y.powi(2)).sqrt();
            let v = (p - q).normalize();
            let w = |a: Vec3<f64>| e[0] * a.x + e[1] * a.y + e[2] * a.z;
            let (uu, vv) = (w(Vec3::unit_x()), w(v));
            Curve3::circle(o + w(m), uu.cross(vv).normalize(), 0.5 * (semi_major + semi_minor), uu)
        };
        let probe = 1e-3;
        let tau = probe * (0.4 * zero() / dist(&section(probe))).sqrt();
        rows.push(("cone tilted section".into(), cone.clone(), section(tau), (2.0, 4.5)));
    }

    let mut reds = Vec::new();
    for (what, surface, a, span) in rows {
        // The mint's own image of A, certified for A (the stored row).
        let image = chart_pcurve_over(&a, span.0, span.1, &surface, b);
        let image_i = chart_pcurve_over(
            &a.map_scalar(f),
            f(span.0),
            f(span.1),
            &surface.map_scalar(f),
            b,
        );
        let (Ok(image), Ok(image_i)) = (image, image_i) else {
            println!("[delta-u2] {what}: no mint for A");
            continue;
        };
        println!(
            "[delta-u2] {what}: A's image projected? {} / {}",
            image.projected().is_some(),
            image_i.projected().is_some()
        );
        let Ok(cache) = PcurveCache::certify(image, span.0, span.1, &a, &surface, b) else {
            println!("[delta-u2] {what}: A's row does not certify at f64");
            continue;
        };
        let Ok(cache_i) = PcurveCache::certify(
            image_i,
            f(span.0),
            f(span.1),
            &a.map_scalar(f),
            &surface.map_scalar(f),
            b,
        ) else {
            println!("[delta-u2] {what}: A's row does not certify at Interval");
            continue;
        };
        for ang in [1e-6, 1e-4, 1e-2] {
            let bb = turned(&a, &surface, ang);
            let tag = format!("{what}, turned {ang:e}");
            let r64 = cache
                .recertify(&bb, span, &surface, b, None)
                .map(|c| format!("{:e}", c.envelope));
            let ri = cache_i
                .recertify(
                    &bb.map_scalar(f),
                    (f(span.0), f(span.1)),
                    &surface.map_scalar(f),
                    b,
                    None,
                )
                .map(|c| format!("{:e}", c.envelope.hi()));
            describe("f64", &tag, &r64);
            describe("Interval", &tag, &ri);
            if let Err(e) = &ri {
                let ending = e.ending(crate::recourse::Reading::AtRest);
                if ending.starts_with(geom_core::NOT_YET_ENDING) {
                    reds.push(format!("{tag} Interval: {e:?} ends \"{ending}\""));
                }
            }
        }
    }
    assert!(
        reds.is_empty(),
        "a stored row that is not the edge carrier's ends \"not yet\" at rest (U2 provenance):\n{}",
        reds.join("\n")
    );
}

// ---------------------------------------------------------------------
// U1: a dump for an independent 50-digit check (review-probes/6485-delta-hp.py).

fn d_world(s: &Surface<f64>, p: Vec3<f64>) -> Point3<f64> {
    let Some((o, e)) = projected::chart_frame(s) else { unreachable!() };
    o + e[0] * p.x + e[1] * p.y + e[2] * p.z
}
fn d_dir(s: &Surface<f64>, v: Vec3<f64>) -> Vec3<f64> {
    d_world(s, v) - d_world(s, Vec3::zero())
}
fn d_circle(s: &Surface<f64>, c: Vec3<f64>, u: Vec3<f64>, v: Vec3<f64>, r: f64) -> Curve3<f64> {
    let (u, v) = (d_dir(s, u), d_dir(s, v));
    Curve3::circle(d_world(s, c), u.cross(v).normalize(), r, u.normalize())
}
/// The cone's tilted section's nearest circle (the rows' shape), tilt `tau`.
fn d_section(s: &Surface<f64>, alpha: f64, h: f64, tau: f64) -> Curve3<f64> {
    let (sa, ca) = alpha.sin_cos();
    let (st, ct) = tau.sin_cos();
    let n = Vec3::new(0.0, -st, ct);
    let sigma = h.signum();
    let vertex = |s_phi: f64| {
        let g = Vec3::new(0.0, sa * s_phi, ca) * sigma;
        g * (h * ct / n.dot(g))
    };
    let (p, q) = (vertex(1.0), vertex(-1.0));
    let m = (p + q) * 0.5;
    let semi_major = (p - q).norm() * 0.5;
    let semi_minor = ((m.z * sa / ca).powi(2) - m.y.powi(2)).sqrt();
    d_circle(s, m, Vec3::unit_x(), (p - q).normalize(), 0.5 * (semi_major + semi_minor))
}

fn fmt_v(v: Vec3<f64>) -> String {
    format!("{:?},{:?},{:?}", v.x, v.y, v.z)
}
fn fmt_p(v: Point3<f64>) -> String {
    format!("{:?},{:?},{:?}", v.x, v.y, v.z)
}
fn fmt_surface(s: &Surface<f64>) -> String {
    match *s {
        Surface::Cylinder { origin, axis, radius, .. } => format!("cyl;{};{};{radius:?}", fmt_p(origin), fmt_v(axis)),
        Surface::Cone { apex, axis, half_angle, .. } => format!("cone;{};{};{half_angle:?}", fmt_p(apex), fmt_v(axis)),
        Surface::Torus { center, axis, major_radius, minor_radius, .. } => {
            format!("torus;{};{};{major_radius:?};{minor_radius:?}", fmt_p(center), fmt_v(axis))
        }
        _ => unreachable!(),
    }
}
fn fmt_carrier(c: &Curve3<f64>) -> String {
    match c {
        Curve3::Circle(geom::Circle { frame: geom::Frame { origin: center, axis, u_ref }, radius }) => {
            format!("circle;{};{};{};{radius:?}", fmt_p(*center), fmt_v(*axis), fmt_v(*u_ref))
        }
        Curve3::Line(geom::Line { origin, dir }) => format!("line;{};{}", fmt_p(*origin), fmt_v(*dir)),
        _ => unreachable!(),
    }
}

#[test]
fn review_6485_delta_hp_dump() {
    use std::io::Write;
    let Ok(path) = std::env::var("RP_HP_DUMP") else { return };
    let mut out = std::fs::File::create(path).unwrap();
    let b = band();
    let f = Interval::from_f64;
    let mut rows: Vec<(String, Surface<f64>, Curve3<f64>, (f64, f64))> = Vec::new();
    for far in [0.0, 1e3, 1e4, 1e5] {
        let at = Point3::new(0.3 + far, -0.7 - 0.6 * far, 0.2 + 0.5 * far);
        let axis = Vec3::new(0.2, -0.4, 1.0).normalize();
        let u_ref = axis.cross(Vec3::unit_x()).normalize();
        // Thin and near-horn tori: crest circles and tilted parallels.
        for (major, ratio) in [(1.0, 1e-1), (1.0, 1e-3), (50.0, 1e-3), (5.0, 1e-2), (2.0, 1.0 - 1e-5), (1.0, 1.0 - 1e-3)] {
            let minor = major * ratio;
            let s = Surface::Torus { center: at, axis, major_radius: major, minor_radius: minor, u_ref };
            for target in [1e-12, 1e-10, 5e-10, 9e-10] {
                let delta = (2.0 * minor * target).sqrt();
                let c = Vec3::new(0.7 * delta, 0.714 * delta, minor);
                rows.push((format!("torus crest R{major} r/R{ratio:e} t{target:e} far{far:e}"), s.clone(),
                    d_circle(&s, c, Vec3::unit_x(), Vec3::unit_y(), major), (0.0, TAU)));
                let theta = 0.9_f64;
                let (z, rho) = (minor * theta.sin(), major + minor * theta.cos());
                let tau = (target / rho).asin();
                let (st, ct) = tau.sin_cos();
                rows.push((format!("torus parallel R{major} r/R{ratio:e} t{target:e} far{far:e}"), s.clone(),
                    d_circle(&s, Vec3::new(0.0, 0.0, z), Vec3::unit_x(), Vec3::new(0.0, ct, st), rho), (0.5, 4.0)));
            }
        }
        // Needle and flat cones, near the apex: tilted sections and rulings.
        for alpha in [0.003, 0.02, 0.8, 1.55, 1.567] {
            let s = Surface::Cone { apex: at, axis, half_angle: alpha, u_ref };
            for h in [1e-4, 1e-2, 1.0, -0.5] {
                for tau in [1e-7, 1e-5, 1e-4] {
                    rows.push((format!("cone section a{alpha} h{h:e} tau{tau:e} far{far:e}"), s.clone(),
                        d_section(&s, alpha, h, tau), (0.0, TAU)));
                }
            }
            let (sa, ca) = alpha.sin_cos();
            for (anchor, start) in [(0.0, 1e-6), (3e-10, 1e-3), (-5e-10, 1e-4), (0.0, 0.5)] {
                let g = Vec3::new(sa * 0.7_f64.cos(), sa * 0.7_f64.sin(), ca);
                let line = Curve3::line(d_world(&s, Vec3::new(0.0, 0.0, anchor)), d_dir(&s, g));
                rows.push((format!("cone line a{alpha} anchor{anchor:e} start{start:e} far{far:e}"), s.clone(),
                    line, (start, start + 2.0)));
            }
        }
    }
    let (mut n, mut minted) = (0, 0);
    for (what, s, c, (t0, t1)) in rows {
        n += 1;
        let p = projected::project(&c, Some((t0, t1)), &s, b);
        let pi = projected::project(&c.map_scalar(f), Some((f(t0), f(t1))), &s.map_scalar(f), b);
        let (Ok(p), Ok(pi)) = (p, pi) else {
            writeln!(out, "#nomint|{what}").unwrap();
            continue;
        };
        minted += 1;
        let e = projected::projected_envelope(&p, t0, t1, &p.chart_box(t0, t1), &c, &s, b, None);
        let ei = projected::projected_envelope(&pi, f(t0), f(t1), &pi.chart_box(f(t0), f(t1)), &c.map_scalar(f), &s.map_scalar(f), b, None);
        let e64 = e.as_ref().map(|(t, _)| format!("{:?}", t.total())).unwrap_or_else(|e| format!("ERR {e:?}"));
        let inc64 = e.as_ref().map(|(t, _)| format!("{:?}", t.0[EnvelopeTerm::Incidence.slot()])).unwrap_or_default();
        let eI = ei.as_ref().map(|(t, _)| format!("{:?}", t.total().hi())).unwrap_or_else(|e| format!("ERR {e:?}"));
        let incI = ei.as_ref().map(|(t, _)| format!("{:?}", t.0[EnvelopeTerm::Incidence.slot()].hi())).unwrap_or_default();
        let image = Pcurve::Projected(Box::new(p));
        // f64-sampled residual, for comparison.
        let sup = (0..=4096).map(|i| {
            let t = t0 + (t1 - t0) * f64::from(i) / 4096.0;
            let q = image.eval(t);
            s.eval(q.x, q.y).distance(c.eval(t))
        }).fold(0.0, f64::max);
        writeln!(out, "{what}|{}|{}|{t0:?}|{t1:?}|{e64}|{eI}|{inc64}|{incI}|{sup:?}", fmt_surface(&s), fmt_carrier(&c)).unwrap();
    }
    println!("[hp-dump] rows {n}, minted {minted}");
}

/// The chart_incidence row's radius-48 near-rim tilted circle on the cone
/// at α 1.55 (h 1, tilted `0.5·ε/(h tan α)`): its envelope's parts.
#[test]
fn review_6485_delta_radius48_parts() {
    let b = band();
    let e = geom_core::Tol::witness().eps();
    let f = Interval::from_f64;
    let (alpha, h) = (1.55_f64, 1.0_f64);
    let theta = 0.5 * e / (h * alpha.tan());
    let t = alpha.tan().powi(2);
    let (s, c) = theta.sin_cos();
    let a = c * c - t * s * s;
    let u = Vec3::new(c, 0.0, -s);
    let centre = Point3::new(0.0, 0.0, h) + u * (-t * h * s / a);
    let k = h * alpha.tan() * c;
    let (major, minor) = (k / a, k / a.sqrt());
    let circle = Curve3::circle(centre, Vec3::new(s, 0.0, c), (major * minor).sqrt(), u);
    let cone = Surface::Cone { apex: Point3::origin(), axis: Vec3::unit_z(), half_angle: alpha, u_ref: Vec3::unit_x() };
    println!("[r48] eps {e:e} zero {:e}", b.zero());
    println!("[r48] carrier {}", fmt_carrier(&circle));
    println!("[r48] surface {}", fmt_surface(&cone));
    let Some(conic) = circle.conic() else { unreachable!() };
    let fs = crate::implicit::conic_form_sup(&cone, conic).unwrap();
    let hh = crate::implicit::conic_cone_harmonics(conic, Point3::origin(), Vec3::unit_z(), alpha);
    println!("[r48] form_sup harmonics {:e} rounding {:e}; per {:e} terms {:e} floor {:e}", fs.harmonics, fs.rounding, hh.per, hh.terms, hh.floor);
    let p = projected::project(&circle, Some((0.0, TAU)), &cone, b);
    match &p {
        Ok(p) => {
            let env = projected::projected_envelope(p, 0.0, TAU, &p.chart_box(0.0, TAU), &circle, &cone, b, None);
            match env {
                Ok((t, _)) => {
                    for term in EnvelopeTerm::ALL { println!("[r48]   f64 {term:?} {:e}", t.0[term.slot()]); }
                    println!("[r48]   f64 total {:e}", t.total());
                }
                Err(e) => println!("[r48] f64 envelope err {e:?}"),
            }
        }
        Err(e) => println!("[r48] project err {e:?}"),
    }
    let ci = circle.map_scalar(f);
    let si = cone.map_scalar(f);
    if let Ok(pi) = projected::project(&ci, Some((f(0.0), f(TAU))), &si, b) {
        if let Ok((t, _)) = projected::projected_envelope(&pi, f(0.0), f(TAU), &pi.chart_box(f(0.0), f(TAU)), &ci, &si, b, None) {
            println!("[r48]   Interval Incidence hi {:e} total hi {:e}", t.0[EnvelopeTerm::Incidence.slot()].hi(), t.total().hi());
            for term in EnvelopeTerm::ALL { println!("[r48]   I {term:?} [{:e}, {:e}]", t.0[term.slot()].lo(), t.0[term.slot()].hi()); }
        }
    }
    let got = chart_pcurve_over(&circle, 0.0, TAU, &cone, b)
        .and_then(|im| PcurveCache::certify(im, 0.0, TAU, &circle, &cone, b).map(|c| c.certificate().envelope));
    println!("[r48] mint+certify f64: {got:?}");
}

/// One dumped row's parts: the form bound and the incidence term.
#[test]
fn review_6485_delta_row_parts() {
    let Ok(row) = std::env::var("RP_ROW") else { return };
    let b = band();
    let f = Interval::from_f64;
    let v = |s: &str| { let x: Vec<f64> = s.split(',').map(|t| t.parse().unwrap()).collect(); Vec3::new(x[0], x[1], x[2]) };
    let p = |s: &str| { let x = v(s); Point3::new(x.x, x.y, x.z) };
    let parts: Vec<&str> = row.split('|').collect();
    let sp: Vec<&str> = parts[1].split(';').collect();
    let cp: Vec<&str> = parts[2].split(';').collect();
    let axis = v(sp[2]);
    let u_ref = axis.cross(Vec3::unit_x()).normalize();
    let surface = match sp[0] {
        "cone" => Surface::Cone { apex: p(sp[1]), axis, half_angle: sp[3].parse().unwrap(), u_ref },
        "torus" => Surface::Torus { center: p(sp[1]), axis, major_radius: sp[3].parse().unwrap(), minor_radius: sp[4].parse().unwrap(), u_ref },
        _ => Surface::Cylinder { origin: p(sp[1]), axis, radius: sp[3].parse().unwrap(), u_ref },
    };
    let carrier = Curve3::circle(p(cp[1]), v(cp[2]), cp[4].parse().unwrap(), v(cp[3]));
    let (t0, t1): (f64, f64) = (parts[3].parse().unwrap(), parts[4].parse().unwrap());
    let conic = carrier.conic().unwrap();
    let fs = crate::implicit::conic_form_sup(&surface, conic).unwrap();
    println!("[parts] form harmonics {:e} rounding {:e} bound {:e}", fs.harmonics, fs.rounding, fs.bound());
    let ci = carrier.map_scalar(f); let si = surface.map_scalar(f);
    let fsi = crate::implicit::conic_form_sup(&si, ci.conic().unwrap()).unwrap();
    println!("[parts] Interval form bound hi {:e}", fsi.bound().hi());
    let pr = projected::project(&carrier, Some((t0, t1)), &surface, b).unwrap();
    let (t, _) = projected::projected_envelope(&pr, t0, t1, &pr.chart_box(t0, t1), &carrier, &surface, b, None).unwrap();
    println!("[parts] incidence {:e}  -> metres/form {:e}", t.0[EnvelopeTerm::Incidence.slot()], t.0[EnvelopeTerm::Incidence.slot()] / fs.bound());
    println!("[parts] carrier dbg {carrier:?}");
    if let Surface::Cone { apex, axis, half_angle, .. } = si {
        let h = crate::implicit::conic_cone_harmonics(ci.conic().unwrap(), apex, axis, half_angle);
        let mut best = 0.0_f64;
        let (sa, ca) = half_angle.sin_cos();
        for i in 0..4096 {
            let t = f(f64::from(i) * TAU / 4096.0);
            let q = ci.eval(t) - apex;
            let hgt = q.dot(axis);
            let rho = (q - axis * hgt).norm();
            let form = (rho * ca - hgt * sa) * (rho * ca + hgt * sa);
            let lm = if form.lo() > 0.0 { form.lo() } else if form.hi() < 0.0 { -form.hi() } else { 0.0 };
            best = best.max(lm);
        }
        println!("[parts] pointwise Interval |form| least-magnitude sup {best:e}");
        let fr = ci.conic().unwrap().frame();
        println!("[parts] I frame center {:?} u {:?} v {:?} semi {:?}", fr.origin, fr.u_ref, fr.v_ref(), ci.conic().unwrap().semi_axes());
        println!("[parts] I coeffs*per c0 {:?} c1 {:?} s1 {:?} c2 {:?} s2 {:?} per {:?}", h.c0 * h.per, h.c1 * h.per, h.s1 * h.per, h.c2 * h.per, h.s2 * h.per, h.per);
    }
}

/// V2: the cone harmonics' f64 coefficients (times `per`) against their
/// charge `rounding_charge(terms)`, for review-probes/6485-delta-v2.py to
/// compare with the exact coefficients of the same stored inputs.
#[test]
fn review_6485_delta_v2_coeff_dump() {
    use std::io::Write;
    let Ok(path) = std::env::var("RP_V2_DUMP") else { return };
    let mut out = std::fs::File::create(path).unwrap();
    let mut s = test_utils::fuzz::start("review_6485_delta_v2");
    for _ in 0..3000 {
        let alpha = match s.below(3) { 0 => s.log_uniform(1e-4, 0.05), 1 => s.range(1.52, 1.5707), _ => s.range(0.05, 1.5) };
        let far = if s.below(2) == 0 { 0.0 } else { s.log_uniform(1.0, 1e5) };
        let apex = Point3::new(0.3 + far, -0.7 - 0.6 * far, 0.2 + 0.5 * far);
        let axis = Vec3::new(s.range(-1.0, 1.0), s.range(-1.0, 1.0), s.range(-1.0, 1.0)).normalize();
        let u_ref = axis.cross(Vec3::new(0.3, 1.0, -0.2)).normalize();
        let cone = Surface::Cone { apex, axis, half_angle: alpha, u_ref };
        let h = s.log_uniform(1e-4, 1e3) * if s.below(2) == 0 { 1.0 } else { -1.0 };
        let tau = if s.below(2) == 0 { 0.0 } else { s.log_uniform(1e-9, 1e-2) };
        let c = d_section(&cone, alpha, h, tau);
        let conic = c.conic().unwrap();
        let hh = crate::implicit::conic_cone_harmonics(conic, apex, axis, alpha);
        let charge = geom_core::rounding_charge(hh.terms);
        writeln!(out, "{}|{}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}", fmt_surface(&cone), fmt_carrier(&c),
            hh.c0 * hh.per, hh.c1 * hh.per, hh.s1 * hh.per, hh.c2 * hh.per, hh.s2 * hh.per, charge, hh.terms, hh.per).unwrap();
    }
}

/// V2 red probe: a needle cone's tilted section (α 0.003, h 1, τ 1e-7,
/// apex 1e5 off the origin), whose 50-digit distance from the cone is
/// `2.7234227197864185e-12` m (review-probes/6485-delta-hp.py on
/// 6485-delta-hp-dump.txt's row). The f64 envelope must cover it.
#[test]
fn review_6485_delta_v2_needle_cone_f64_envelope_covers_the_50_digit_distance() {
    let far = 1e5;
    let at = Point3::new(0.3 + far, -0.7 - 0.6 * far, 0.2 + 0.5 * far);
    let axis = Vec3::new(0.2, -0.4, 1.0).normalize();
    let u_ref = axis.cross(Vec3::unit_x()).normalize();
    let alpha = 0.003;
    let s = Surface::Cone { apex: at, axis, half_angle: alpha, u_ref };
    let c = d_section(&s, alpha, 1.0, 1e-7);
    let exact = 2.7234227197864185e-12;
    let b = band();
    let p = projected::project(&c, Some((0.0, TAU)), &s, b).unwrap();
    let (t, _) = projected::projected_envelope(&p, 0.0, TAU, &p.chart_box(0.0, TAU), &c, &s, b, None).unwrap();
    let env = t.total();
    assert!(env >= exact, "f64 envelope {env:e} under the 50-digit distance {exact:e} (carrier {})", fmt_carrier(&c));
}
