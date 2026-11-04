//! Review probes for PR 6527 (not for merge).
use geom::{Curve3, Surface};
use geom_core::{Interval, Point3, Real, Vec3};
use std::io::Write;

use super::harmonic_sup_rows::{band, zero};
use super::{chart_distance, line_incidence, read_incidence};

fn verdicts(line: geom::Line<f64>, span: (f64, f64), s: &Surface<f64>) -> String {
    let f = Interval::from_f64;
    let c = Curve3::Line(line);
    let a = read_incidence(&c, span.0, span.1, s, band());
    let b = read_incidence(&c.map_scalar(f), f(span.0), f(span.1), &s.map_scalar(f), band());
    let fr = line_incidence::critical_fractions(&line, s, span);
    let mid = chart_distance(s, c.eval(0.5 * (span.0 + span.1)));
    format!("fractions {fr:?}; mid distance {mid:e}\n  f64: {a:?}\n  Interval: {b:?}")
}

#[test]
fn probe_spindle_axis_line() {
    let u_ref = Vec3::new(1.0, 0.0, 0.0);
    for (big, axis, origin_shift) in [
        (0.3, Vec3::new(0.0, 0.0, 1.0), 0.0),
        (-0.3, Vec3::new(0.0, 0.0, 1.0), 0.0),
        (0.3, Vec3::new(0.0, 0.0, 1.0), 1e-12),
        (0.3, Vec3::new(0.1, -0.2, 1.0).normalize(), 0.0),
        (0.0, Vec3::new(0.0, 0.0, 1.0), 0.0),
    ] {
        let c = Point3::new(0.0, 0.0, 0.0);
        let s = Surface::Spindle { center: c, axis, major_radius: big, minor_radius: 0.5, u_ref };
        let tip = (0.25f64 - big * big).sqrt();
        let line = geom::Line {
            origin: c - axis * tip + Vec3::new(origin_shift, 0.0, 0.0),
            dir: axis * (2.0 * tip),
        };
        println!("R {big} axis {axis:?} shift {origin_shift}: {}", verdicts(line, (0.0, 1.0), &s));
    }
    // the torus with R > r: line along the axis (fine) and the sphere through centre
    let s = Surface::Sphere { center: Point3::origin(), radius: 0.5, axis: Vec3::new(0.0, 0.0, 1.0), u_ref };
    let line = geom::Line { origin: Point3::new(0.0, 0.0, -0.5), dir: Vec3::new(0.0, 0.0, 1.0) };
    println!("sphere diameter: {}", verdicts(line, (0.0, 1.0), &s));
}

fn unitv(rng: &mut test_utils::fuzz::Rng) -> Vec3<f64> {
    loop {
        let v = Vec3::new(rng.range(-1.0, 1.0), rng.range(-1.0, 1.0), rng.range(-1.0, 1.0));
        if v.norm() > 0.1 && v.norm() <= 1.0 {
            return v.normalize();
        }
    }
}

/// Dumps lines shaped unlike the PR's hunt for an mpmath supremum.
#[test]
fn dump_cases() {
    let path = std::env::var("REVIEW_DUMP").unwrap_or("/tmp/claude-0/cases.jsonl".into());
    let mut out = std::fs::File::create(path).unwrap();
    let mut rng = test_utils::fuzz::pinned("review 6527", 6527);
    let n: usize = std::env::var("REVIEW_N").ok().and_then(|s| s.parse().ok()).unwrap_or(4000);
    let z = zero();
    let f = Interval::from_f64;
    for i in 0..n {
        let axis = if rng.below(3) == 0 { Vec3::new(0.0, 0.0, 1.0) } else { unitv(&mut rng) };
        let c = if rng.below(3) == 0 { Point3::origin() } else { Point3::new(rng.range(-2.0, 2.0), rng.range(-2.0, 2.0), rng.range(-2.0, 2.0)) };
        let u_ref = { let t = unitv(&mut rng); (t - axis * t.dot(axis)).normalize() };
        let kind = rng.below(6);
        let (s, kname, p1, p2, singular): (Surface<f64>, &str, f64, f64, Vec<Point3<f64>>) = match kind {
            0 => { let a = [0.5, 0.05, 1.5, 1.0e-3][rng.below(4)]; (Surface::Cone { apex: c, axis, half_angle: a, u_ref }, "cone", a, 0.0, vec![c]) }
            1 => { let (bg, r) = [(1.2, 0.4), (0.5, 0.5), (1.0, 0.999), (2.0, 1e-3)][rng.below(4)]; (Surface::Torus { center: c, axis, major_radius: bg, minor_radius: r, u_ref }, "torus", bg, r, vec![c]) }
            2 => { let (bg, r) = [(0.3, 0.5), (-0.3, 0.5), (0.499, 0.5), (1e-3, 0.5), (-0.499, 0.5)][rng.below(5)];
                   let tip = (r * r - bg * bg).sqrt();
                   (Surface::Spindle { center: c, axis, major_radius: bg, minor_radius: r, u_ref }, "spindle", bg, r, vec![c + axis * tip, c - axis * tip, c]) }
            3 => (Surface::Sphere { center: c, radius: 0.7, axis, u_ref }, "sphere", 0.7, 0.0, vec![c]),
            4 => (Surface::Cylinder { origin: c, axis, radius: 0.7, u_ref }, "cylinder", 0.7, 0.0, vec![c]),
            _ => (Surface::Plane { origin: c, normal: axis, u_ref }, "plane", 0.0, 0.0, vec![c]),
        };
        // shapes: (a) chord between two wall points + offset, (b) through a singular
        // point along axis / along a generator-ish / in the perpendicular plane,
        // (c) tangent at a wall point, (d) near-axis nearly parallel.
        let w = |rng: &mut test_utils::fuzz::Rng| s.eval(rng.range(-3.0, 3.0), rng.range(-1.0, 1.0));
        let off = |rng: &mut test_utils::fuzz::Rng| unitv(rng) * (z * rng.log_uniform(1e-2, 1e4));
        let (o, d) = match rng.below(5) {
            0 => { let (a, b) = (w(&mut rng), w(&mut rng)); (a + off(&mut rng), b - a) }
            1 => { let sp = singular[rng.below(singular.len())]; let e = unitv(&mut rng);
                   let dir = match rng.below(3) { 0 => axis, 1 => (e - axis * e.dot(axis)).normalize(), _ => (w(&mut rng) - sp).normalize() };
                   (sp + off(&mut rng) - dir * rng.range(0.0, 1.0), dir + unitv(&mut rng) * (z * rng.log_uniform(1e-3, 1e3))) }
            2 => { let (uu, vv) = (rng.range(-3.0, 3.0), rng.range(-1.0, 1.0)); let p = s.eval(uu, vv);
                   let dir = (s.deriv_u(uu, vv).normalize() * rng.range(-1.0, 1.0) + s.deriv_v(uu, vv).normalize() * rng.range(-1.0, 1.0)).normalize();
                   (p + off(&mut rng) - dir * rng.range(0.0, 1.0), dir) }
            3 => { let e = unitv(&mut rng); let side = (e - axis * e.dot(axis)).normalize();
                   (c + side * (z * rng.log_uniform(1e-2, 1e5)) - axis * rng.range(0.0, 1.5), axis + side.cross(axis) * (z * rng.log_uniform(1e-3, 1e4))) }
            _ => (w(&mut rng), unitv(&mut rng)),
        };
        if !(d.norm() > 0.0 && d.norm().is_finite() && o.x.is_finite()) { continue; }
        let d = d * rng.log_uniform(0.1, 10.0);
        let line = geom::Line { origin: o, dir: d };
        let span = (rng.range(-0.2, 0.3) * [1.0, 10.0, 0.01][rng.below(3)], rng.range(0.5, 2.0) * [1.0, 10.0, 0.01][rng.below(3)]);
        if !(span.1 > span.0) { continue; }
        let carrier = Curve3::Line(line);
        let fr = line_incidence::critical_fractions(&line, &s, span);
        let read = [0.0, 1.0].into_iter().chain(fr.iter().copied())
            .map(|l| chart_distance(&s, carrier.eval(span.0 + (span.1 - span.0) * l))).fold(0.0, f64::max);
        let v64 = read_incidence(&carrier, span.0, span.1, &s, band());
        let vi = read_incidence(&carrier.map_scalar(f), f(span.0), f(span.1), &s.map_scalar(f), band());
        let tag = |r: &Result<(), super::PcurveCertifyError>| match r { Ok(()) => "on".to_string(), Err(super::PcurveCertifyError::CarrierOffChart { .. }) => "off".into(), Err(e) => format!("{:?}", std::mem::discriminant(e)).chars().filter(|c| c.is_alphanumeric()).collect::<String>().replace("Discriminant", "esc") };
        writeln!(out, "{{\"i\":{i},\"kind\":\"{kname}\",\"c\":[{:e},{:e},{:e}],\"a\":[{:e},{:e},{:e}],\"p1\":{p1:e},\"p2\":{p2:e},\"o\":[{:e},{:e},{:e}],\"d\":[{:e},{:e},{:e}],\"span\":[{:e},{:e}],\"read\":{read:e},\"fr\":{fr:?},\"v64\":\"{}\",\"vi\":\"{}\",\"zero\":{z:e},\"esc\":{:e}}}",
            c.x, c.y, c.z, axis.x, axis.y, axis.z, o.x, o.y, o.z, d.x, d.y, d.z, span.0, span.1, tag(&v64), tag(&vi), band().escalate()).unwrap();
    }
}

#[test]
fn probe_spindle_axis_line_mint() {
    let f = Interval::from_f64;
    let u_ref = Vec3::new(1.0, 0.0, 0.0);
    for big in [0.3, -0.3] {
        let axis = Vec3::new(0.0, 0.0, 1.0);
        let c = Point3::origin();
        let s = Surface::Spindle { center: c, axis, major_radius: big, minor_radius: 0.5, u_ref };
        let tip = (0.25f64 - big * big).sqrt();
        for span in [(0.0, 1.0), (0.1, 0.9)] {
            let line = geom::Line { origin: c - axis * tip, dir: axis * (2.0 * tip) };
            let carrier = Curve3::Line(line);
            let m = super::chart_pcurve_over(&carrier, span.0, span.1, &s, band());
            println!("R {big} span {span:?} mint f64: {m:?}");
            if let Ok(im) = m.clone() {
                println!("   certify f64: {:?}", super::PcurveCache::certify(im, span.0, span.1, &carrier, &s, band()).map(|_| "CERTIFIED"));
            }
            let ci = carrier.map_scalar(f);
            let si = s.map_scalar(f);
            let mi = super::chart_pcurve_over(&ci, f(span.0), f(span.1), &si, band());
            println!("   mint Interval: {mi:?}");
            println!("   door: {}", verdicts(line, span, &s));
        }
    }
}

/// Near-incident lines: through a cone apex or spindle tip along the
/// tangent cone, shifted/tilted by band-sized amounts; short tangent
/// segments on curved walls whose sagitta is band-sized.
#[test]
fn dump_near() {
    let path = std::env::var("REVIEW_DUMP").unwrap_or("/tmp/claude-0/near.jsonl".into());
    let mut out = std::fs::File::create(path).unwrap();
    let mut rng = test_utils::fuzz::pinned("review 6527 near", 6528);
    let n: usize = std::env::var("REVIEW_N").ok().and_then(|s| s.parse().ok()).unwrap_or(4000);
    let z = zero();
    let f = Interval::from_f64;
    for i in 0..n {
        let axis = if rng.below(2) == 0 { Vec3::new(0.0, 0.0, 1.0) } else { unitv(&mut rng) };
        let c = if rng.below(2) == 0 { Point3::origin() } else { Point3::new(rng.range(-2.0, 2.0), rng.range(-2.0, 2.0), rng.range(-2.0, 2.0)) };
        let e = { let t = unitv(&mut rng); (t - axis * t.dot(axis)).normalize() };
        let kind = rng.below(4);
        let (s, kname, p1, p2, tip, gdir): (Surface<f64>, &str, f64, f64, Point3<f64>, Vec3<f64>) = match kind {
            0 => { let a = [0.5, 0.05, 1.5, 1e-3, 1.0][rng.below(5)]; let sg = if rng.below(2) == 0 { 1.0 } else { -1.0 };
                   (Surface::Cone { apex: c, axis, half_angle: a, u_ref: e }, "cone", a, 0.0, c, axis * (sg * a.cos()) + e * a.sin()) }
            1 => { let (bg, r) = [(0.3, 0.5), (-0.3, 0.5), (0.499, 0.5), (1e-3, 0.5), (-0.499, 0.5), (0.1, 0.5), (0.4999999, 0.5)][rng.below(7)];
                   let tp = (r * r - bg * bg).sqrt(); let sg = if rng.below(2) == 0 { 1.0 } else { -1.0 };
                   (Surface::Spindle { center: c, axis, major_radius: bg, minor_radius: r, u_ref: e }, "spindle", bg, r, c + axis * (sg * tp), (e * tp + axis * (sg * bg)).normalize()) }
            2 => { let (bg, r) = [(1.2, 0.4), (0.5, 0.5), (2.0, 1e-3), (0.5, 0.4999999), (1.0, 0.9)][rng.below(5)];
                   (Surface::Torus { center: c, axis, major_radius: bg, minor_radius: r, u_ref: e }, "torus", bg, r, c, axis) }
            _ => (Surface::Sphere { center: c, radius: 0.7, axis, u_ref: e }, "sphere", 0.7, 0.0, c, axis),
        };
        let axis_shape = rng.below(4) == 0;
        let (o, d, span) = if axis_shape {
            let e2 = { let t = unitv(&mut rng); (t - axis * t.dot(axis)).normalize() };
            let off = match rng.below(3) { 0 => 0.0, 1 => z * rng.log_uniform(1e-6, 1e3), _ => 1e-300 };
            let tilt = match rng.below(3) { 0 => 0.0, 1 => z * rng.log_uniform(1e-6, 1e3), _ => 1e-300 };
            let a = rng.range(0.0, 1.5); let b = rng.range(0.0, 1.5);
            let sp = if rng.below(3) == 0 && matches!(kind, 1) { let tp = (p2 * p2 - p1 * p1).sqrt(); (-tp, tp) } else { (-a, b) };
            (c + e2 * off, axis + e2.cross(axis) * tilt, sp)
        } else if kind <= 1 && rng.below(3) != 0 {
            let side = unitv(&mut rng);
            let shift = side * (z * rng.log_uniform(1e-2, 1e3));
            let tilt = unitv(&mut rng) * (z * rng.log_uniform(1e-3, 1e2));
            let a = rng.log_uniform(1e-4, 1.0);
            let b = rng.log_uniform(1e-4, 1.0);
            let (a, b) = if rng.below(4) == 0 { (-a * 0.5, -a * 0.1) } else { (-a, b) };
            (tip + shift, gdir + tilt, (a, b))
        } else {
            let (uu, vv) = (rng.range(-3.0, 3.0), rng.range(-1.0, 1.0));
            let p = s.eval(uu, vv);
            let (du, dv) = (s.deriv_u(uu, vv), s.deriv_v(uu, vv));
            if !(du.norm() > 1e-9 && dv.norm() > 1e-9) { continue; }
            let nrm = du.cross(dv).normalize();
            let dir = (du.normalize() * rng.range(-1.0, 1.0) + dv.normalize() * rng.range(-1.0, 1.0)).normalize();
            let l = (z * rng.log_uniform(1.0, 1e4)).sqrt() * rng.log_uniform(0.3, 3.0);
            (p + nrm * (z * rng.range(-3.0, 3.0)), dir, (-l * rng.range(0.0, 1.0), l * rng.range(0.1, 1.0)))
        };
        if !(d.norm().is_finite() && d.norm() > 0.0 && o.x.is_finite()) { continue; }
        let line = geom::Line { origin: o, dir: d };
        let carrier = Curve3::Line(line);
        let fr = line_incidence::critical_fractions(&line, &s, span);
        let read = [0.0, 1.0].into_iter().chain(fr.iter().copied())
            .map(|l| chart_distance(&s, carrier.eval(span.0 + (span.1 - span.0) * l))).fold(0.0, f64::max);
        let v64 = read_incidence(&carrier, span.0, span.1, &s, band());
        let vi = read_incidence(&carrier.map_scalar(f), f(span.0), f(span.1), &s.map_scalar(f), band());
        let tag = |r: &Result<(), super::PcurveCertifyError>| match r { Ok(()) => "on".to_string(), Err(super::PcurveCertifyError::CarrierOffChart { .. }) => "off".into(), Err(super::PcurveCertifyError::Escalated { .. }) => "esc".into(), Err(_) => "other".into() };
        writeln!(out, "{{\"i\":{i},\"kind\":\"{kname}\",\"c\":[{:e},{:e},{:e}],\"a\":[{:e},{:e},{:e}],\"p1\":{p1:e},\"p2\":{p2:e},\"o\":[{:e},{:e},{:e}],\"d\":[{:e},{:e},{:e}],\"span\":[{:e},{:e}],\"read\":{read:e},\"fr\":{fr:?},\"v64\":\"{}\",\"vi\":\"{}\",\"zero\":{z:e},\"esc\":{:e}}}",
            c.x, c.y, c.z, axis.x, axis.y, axis.z, o.x, o.y, o.z, d.x, d.y, d.z, span.0, span.1, tag(&v64), tag(&vi), band().escalate()).unwrap();
    }
}

/// Cost: read_incidence over lines on each chart (time this on head and the mutant).
#[test]
fn time_line_reads() {
    let f = Interval::from_f64;
    let mut rng = test_utils::fuzz::pinned("review 6527 time", 1);
    let axis = Vec3::new(0.1, -0.2, 1.0).normalize();
    let e = Vec3::new(1.0, 0.0, -0.1).normalize();
    let c = Point3::new(0.4, 0.1, -0.3);
    let charts = [
        Surface::Cylinder { origin: c, axis, radius: 0.7, u_ref: e },
        Surface::Cone { apex: c, axis, half_angle: 0.5, u_ref: e },
        Surface::Torus { center: c, axis, major_radius: 1.2, minor_radius: 0.4, u_ref: e },
        Surface::Spindle { center: c, axis, major_radius: 0.3, minor_radius: 0.5, u_ref: e },
    ];
    for s in &charts {
        let lines: Vec<_> = (0..2000).map(|_| { let (u, v) = (rng.range(-3.0, 3.0), rng.range(-1.0, 1.0)); geom::Line { origin: s.eval(u, v), dir: unitv(&mut rng) } }).collect();
        let si = s.map_scalar(f);
        let t = std::time::Instant::now();
        for l in &lines { let _ = read_incidence(&Curve3::Line(*l), -0.5, 0.5, s, band()); }
        let a = t.elapsed();
        let t = std::time::Instant::now();
        for l in &lines { let _ = read_incidence(&Curve3::Line(*l).map_scalar(f), f(-0.5), f(0.5), &si, band()); }
        println!("TIME {:?}: f64 {:?} interval {:?}", s.kind(), a, t.elapsed());
    }
}

/// The P2's spindle row: an apple spindle's meridian circle shifted off
/// its plane by `s`, read over an edge through a tip.
#[test]
fn probe_meridian_circle_past_a_tip() {
    let f = Interval::from_f64;
    let z = zero();
    let s = Surface::Spindle { center: Point3::origin(), axis: Vec3::new(0.0, 0.0, 1.0), major_radius: 0.3, minor_radius: 0.5, u_ref: Vec3::new(1.0, 0.0, 0.0) };
    let tip_theta = 0.8f64.atan2(0.6);
    for shift in [1e-7, 30.0 * z, 100.0 * z, 300.0 * z, 1e3 * z] {
        let circle = Curve3::Circle(geom::Circle {
            frame: geom::Frame { origin: Point3::new(0.3, shift, 0.0), axis: Vec3::new(0.0, 1.0, 0.0), u_ref: Vec3::new(-1.0, 0.0, 0.0) },
            radius: 0.5,
        });
        let at_tip = chart_distance(&s, circle.eval(tip_theta));
        for span in [(0.5, 1.3), (tip_theta - 0.37, tip_theta + 0.41)] {
            let door = read_incidence(&circle, span.0, span.1, &s, band());
            let door_i = read_incidence(&circle.map_scalar(f), f(span.0), f(span.1), &s.map_scalar(f), band());
            let edge = super::read_edge_incidence(&circle, span.0, span.1, &s, band());
            let mint = super::chart_pcurve_over(&circle, span.0, span.1, &s, band());
            let sh = |r: &Result<(), super::PcurveCertifyError>| format!("{:?}", r).chars().take(60).collect::<String>();
            println!("shift {shift:e} (at tip {at_tip:e} = {:.1} zero) span {span:?}:\n  door f64 {}\n  door I {}\n  edge {}\n  mint {}", at_tip / z, sh(&door), sh(&door_i), sh(&edge.map(|_| ())), format!("{:?}", mint.map(|_| "MINTED")).chars().take(90).collect::<String>());
        }
    }
}
