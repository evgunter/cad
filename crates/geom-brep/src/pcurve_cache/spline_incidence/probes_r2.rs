//! Review R2 probes for PR 6498 (not for merge).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, missing_docs)]

use geom::NurbsSurface;
use geom_core::spline::KnotVector;
use geom_core::{Band, Bounds, Interval, Point3, Real, Vec3};

use super::{Net, Read};

type Rng = test_utils::fuzz::Rng;

fn knots(rng: &mut Rng, p: usize, nint: usize) -> KnotVector {
    let mut interior: Vec<f64> = (0..rng.below(nint + 1)).map(|_| rng.range(0.05, 0.95)).collect();
    interior.sort_by(f64::total_cmp);
    let mut v = vec![0.0; p + 1];
    for x in interior {
        v.extend(core::iter::repeat_n(x, 1 + rng.below(p)));
    }
    v.extend(core::iter::repeat_n(1.0, p + 1));
    KnotVector::clamped(v, p).unwrap()
}

/// Harsher patches: degrees to 5, weights to 8 decades, far placement.
fn patch(rng: &mut Rng) -> (NurbsSurface<f64>, String) {
    let (pu, pv) = (1 + rng.below(5), 1 + rng.below(5));
    let (ku, kv) = (knots(rng, pu, 3), knots(rng, pv, 3));
    let (nu, nv) = (ku.control_count(), kv.control_count());
    let scale = 10f64.powf(rng.range(-3.0, 3.0));
    let far = [0.0, 10.0, 1e3, 1e5][rng.below(4)];
    let offset = Vec3::new(rng.range(-1.0, 1.0), rng.range(-1.0, 1.0), rng.range(-1.0, 1.0)) * scale * far;
    let mode = ["smooth", "folded", "pole", "flat", "saddle", "spike"][rng.below(6)];
    let mut control = Vec::with_capacity(nu * nv);
    for i in 0..nu {
        for j in 0..nv {
            let (x, y) = (i as f64 / nu as f64, j as f64 / nv as f64);
            let z = match mode {
                "folded" => rng.range(-1.0, 1.0),
                "flat" => rng.range(-1.0, 1.0) * 1e-9,
                "saddle" => (x - 0.5) * (y - 0.5) * 4.0,
                "spike" if i == nu / 2 && j == nv / 2 => 30.0,
                _ => 0.3 * (3.0 * x).sin() * (2.0 * y).cos() + rng.range(-0.05, 0.05),
            };
            let (x, y) = match mode {
                "folded" => (x + rng.range(-0.5, 0.5), y + rng.range(-0.5, 0.5)),
                "pole" if i == 0 => (0.0, 0.5),
                _ => (x, y),
            };
            // A tilt so nothing is axis aligned.
            let p = Vec3::new(x, y, z);
            let r = Vec3::new(0.6 * p.x + 0.8 * p.z, p.y, -0.8 * p.x + 0.6 * p.z);
            let r = Vec3::new(r.x, 0.6 * r.y - 0.8 * r.z, 0.8 * r.y + 0.6 * r.z);
            control.push(Point3::origin() + r * scale + offset);
        }
    }
    let decades = [0.0, 2.0, 4.0, 8.0][rng.below(4)];
    let weights: Vec<f64> = (0..nu * nv)
        .map(|_| 10f64.powf(rng.range(-decades / 2.0, decades / 2.0)))
        .collect();
    let what = format!("{mode} deg ({pu},{pv}) n ({nu},{nv}) scale {scale:e} far {far:e} wdec {decades}");
    (NurbsSurface::new(ku, kv, control, weights).unwrap(), what)
}

fn dist(a: Point3<f64>, b: Point3<f64>) -> f64 {
    (a - b).norm()
}

/// A tight upper bound on q's distance from the chart: grid + pattern
/// search in f64, the best parameter enclosed at Interval.
fn upper(chart: &NurbsSurface<f64>, lifted: &NurbsSurface<Interval>, q: Point3<f64>) -> (f64, f64, f64) {
    let ((u0, u1), (v0, v1)) = (chart.knot_values_u().domain(), chart.knot_values_v().domain());
    let g = 48;
    let (mut bu, mut bv, mut bd) = (u0, v0, f64::INFINITY);
    for i in 0..=g {
        for j in 0..=g {
            let (u, v) = (u0 + (u1 - u0) * i as f64 / g as f64, v0 + (v1 - v0) * j as f64 / g as f64);
            let d = dist(chart.eval(u, v), q);
            if d < bd {
                (bu, bv, bd) = (u, v, d);
            }
        }
    }
    let (mut su, mut sv) = ((u1 - u0) / g as f64, (v1 - v0) / g as f64);
    for _ in 0..400 {
        let mut moved = false;
        for (du, dv) in [(1.0, 0.0), (-1.0, 0.0), (0.0, 1.0), (0.0, -1.0), (1.0, 1.0), (-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0)] {
            let (u, v) = ((bu + du * su).clamp(u0, u1), (bv + dv * sv).clamp(v0, v1));
            let d = dist(chart.eval(u, v), q);
            if d < bd {
                (bu, bv, bd) = (u, v, d);
                moved = true;
            }
        }
        if !moved {
            su *= 0.5;
            sv *= 0.5;
        }
    }
    let qi = q.map(<Interval as Real>::from_f64);
    let hi = (qi - lifted.eval(<Interval as Real>::from_f64(bu), <Interval as Real>::from_f64(bv)))
        .norm()
        .hi();
    (hi, bu, bv)
}

fn normal(chart: &NurbsSurface<f64>, u: f64, v: f64) -> Vec3<f64> {
    let ((u0, u1), (v0, v1)) = (chart.knot_values_u().domain(), chart.knot_values_v().domain());
    let h = 1e-6;
    let (ua, ub) = ((u - h).max(u0), (u + h).min(u1));
    let (va, vb) = ((v - h).max(v0), (v + h).min(v1));
    let du = chart.eval(ub, v) - chart.eval(ua, v);
    let dv = chart.eval(u, vb) - chart.eval(u, va);
    let n = du.cross(dv);
    if n.norm() > 0.0 && n.norm().is_finite() { n.normalize() } else { Vec3::new(0.0, 0.0, 1.0) }
}

/// **Every Off floor is below a tight upper bound on the distance**, at
/// bands (1e-12, 1e-11) and (1e-9, 1e-8), at f64 and Interval, samples
/// along the normal near the band and tangent-moved.
#[test]
fn r2_floor_below_tight_distance() {
    let mut rng = test_utils::fuzz::start("r2-spline-floor");
    let cases: usize = std::env::var("R2_CASES").ok().and_then(|s| s.parse().ok()).unwrap_or(300);
    let interval = std::env::var("R2_INTERVAL").is_ok();
    let (mut off, mut on, mut uns, mut bad, mut worst) = (0usize, 0usize, 0usize, 0usize, 0.0f64);
    for case in 0..cases {
        let (chart, what) = patch(&mut rng);
        let lifted = chart.map_scalar(<Interval as Real>::from_f64);
        let band = if case % 2 == 0 { Band::new(1e-12, 1e-11).unwrap() } else { Band::new(1e-9, 1e-8).unwrap() };
        let ((u0, u1), (v0, v1)) = (chart.knot_values_u().domain(), chart.knot_values_v().domain());
        let (u, v) = (rng.range(u0, u1), rng.range(v0, v1));
        let foot = chart.eval(u, v);
        let size = {
            let (lo, hi) = super::shadow_box(chart.control());
            (0..3).map(|k| (hi[k] - lo[k]).powi(2)).sum::<f64>().sqrt()
        };
        let kind = rng.below(4);
        let q = match kind {
            // along the normal, near the band
            0 | 1 => foot + normal(&chart, u, v) * (band.escalate() * 10f64.powf(rng.range(-1.0, 3.0))),
            // along the normal, a fraction of the size
            2 => foot + normal(&chart, u, v) * (size * 10f64.powf(rng.range(-8.0, 0.0))),
            // random direction
            _ => foot + Vec3::new(rng.range(-1.0, 1.0), rng.range(-1.0, 1.0), rng.range(-1.0, 1.0)) * (size * 10f64.powf(rng.range(-6.0, 0.5))),
        };
        let net = Net::of(&chart, band).expect("cuts");
        let r = net.read(q, band);
        let floor = match r {
            Read::Off(f) => { off += 1; Some(f) }
            Read::On => { on += 1; None }
            Read::Unsettled(c) => { uns += 1; if c.margin.is_invalid() { eprintln!("INVALID f64 case {case} kind {kind} {what}"); } None }
        };
        let ifloor = if interval {
            let qi = q.map(<Interval as Real>::from_f64);
            match Net::of(&lifted, band).expect("cuts").read(qi, band) {
                Read::Off(f) => Some(f.lo()),
                Read::Unsettled(c) => { if c.margin.is_invalid() { eprintln!("INVALID Interval case {case} kind {kind} {what}"); } None }
                _ => None,
            }
        } else { None };
        if floor.is_some() || ifloor.is_some() {
            let (up, _, _) = upper(&chart, &lifted, q);
            for (lane, f) in [("f64", floor), ("Interval", ifloor)] {
                if let Some(f) = f {
                    let ratio = f / up;
                    if ratio > worst { worst = ratio; }
                    if f > up {
                        bad += 1;
                        eprintln!("COUNTEREXAMPLE case {case} {lane}: floor {f:e} > dist {up:e} (band {:e}) kind {kind} {what} q {q:?}", band.zero());
                    }
                }
            }
        }
    }
    eprintln!("[r2] off {off} on {on} unsettled {uns} bad {bad} worst floor/dist {worst}");
    assert_eq!(bad, 0);
}

/// **The cut cells are the chart**: each Bézier cell's rational net,
/// evaluated by de Casteljau, matches the chart's own eval at the
/// mapped parameter; and after quartering.
#[test]
fn r2_cells_are_the_chart() {
    let mut rng = test_utils::fuzz::start("r2-cells");
    let band = Band::new(1e-9, 1e-8).unwrap();
    let mut worst = 0.0f64;
    for case in 0..200 {
        let (chart, what) = patch(&mut rng);
        let net = Net::of(&chart, band).expect("cuts");
        let (pu, pv) = (net.pu, net.pv);
        let size = {
            let (lo, hi) = super::shadow_box(chart.control());
            (0..3).map(|k| (hi[k] - lo[k]).powi(2)).sum::<f64>().sqrt().max(1e-300)
        };
        let mut cells: Vec<super::Cell<f64>> = net.cells.clone();
        // one level of quartering on the first cell, too
        let (a, b) = cells[0].split(pu, pv, true);
        for h in [a, b] {
            let (c, d) = h.split(pu, pv, false);
            cells.push(c);
            cells.push(d);
        }
        for cell in &cells {
            for _ in 0..4 {
                let (s, t) = (rng.range(0.0, 1.0), rng.range(0.0, 1.0));
                // de Casteljau on the homogeneous net
                let mut rows: Vec<[f64; 4]> = Vec::new();
                for a in 0..=pu {
                    let mut row: Vec<[f64; 4]> = (0..=pv).map(|b| cell.net[a * (pv + 1) + b]).collect();
                    for l in 1..=pv {
                        for i in 0..=pv - l {
                            row[i] = core::array::from_fn(|c| row[i][c] * (1.0 - t) + row[i + 1][c] * t);
                        }
                    }
                    rows.push(row[0]);
                }
                for l in 1..=pu {
                    for i in 0..=pu - l {
                        rows[i] = core::array::from_fn(|c| rows[i][c] * (1.0 - s) + rows[i + 1][c] * s);
                    }
                }
                let h = rows[0];
                let p = Point3::new(h[0] / h[3], h[1] / h[3], h[2] / h[3]) + net.centre;
                let [u0, u1, v0, v1] = cell.rect;
                let e = chart.eval(u0 + (u1 - u0) * s, v0 + (v1 - v0) * t);
                let rel = dist(p, e) / size;
                worst = worst.max(rel);
                assert!(rel < 1e-6, "case {case} {what}: cell point {p:?} vs chart {e:?} rel {rel:e}");
            }
        }
    }
    eprintln!("[r2-cells] worst rel {worst:e}");
}

/// **On-chart samples of big patches at a 1e-12 band**: q is the
/// middle of the chart's Interval eval, so its distance is at most the
/// enclosure's reach; any Off with floor above that is false.
#[test]
fn r2_big_patch_on_chart_samples() {
    let mut rng = test_utils::fuzz::start("r2-big");
    let cases: usize = std::env::var("R2_CASES").ok().and_then(|s| s.parse().ok()).unwrap_or(300);
    let band = Band::new(1e-12, 1e-11).unwrap();
    let (mut off, mut on, mut uns, mut bad) = (0usize, 0usize, 0usize, 0usize);
    for case in 0..cases {
        let (chart0, what) = patch(&mut rng);
        // rescale to 1e3..1e6
        let s = 10f64.powf(rng.range(std::env::var("R2_SLO").ok().and_then(|x| x.parse().ok()).unwrap_or(3.0), 6.0));
        let (lo, hi) = super::shadow_box(chart0.control());
        let size = (0..3).map(|k| (hi[k] - lo[k]).powi(2)).sum::<f64>().sqrt();
        let f = s / size;
        let control: Vec<Point3<f64>> = chart0.control().iter().map(|p| Point3::new(p.x * f, p.y * f, p.z * f)).collect();
        let chart = NurbsSurface::new(
            chart0.knots_u_f64_interim().clone(),
            chart0.knots_v_f64_interim().clone(),
            control,
            chart0.weights().to_vec(),
        )
        .unwrap();
        let lifted = chart.map_scalar(<Interval as Real>::from_f64);
        let ((u0, u1), (v0, v1)) = (chart.knot_values_u().domain(), chart.knot_values_v().domain());
        let (u, v) = (rng.range(u0, u1), rng.range(v0, v1));
        let pi = lifted.eval(<Interval as Real>::from_f64(u), <Interval as Real>::from_f64(v));
        let mid = |x: Interval| 0.5 * (x.lo() + x.hi());
        let q = Point3::new(mid(pi.x), mid(pi.y), mid(pi.z));
        let lo_s: f64 = std::env::var("R2_SLO").ok().and_then(|x| x.parse().ok()).unwrap_or(3.0);
        let _ = lo_s;
        let q = if std::env::var("R2_D").is_ok() {
            q + normal(&chart, u, v) * (band.escalate() * rng.range(0.0, 4.0))
        } else { q };
        let qi = q.map(<Interval as Real>::from_f64);
        let upper = (qi - pi).norm().hi();
        match Net::of(&chart, band).expect("cuts").read(q, band) {
            Read::Off(fl) => {
                off += 1;
                if fl > upper {
                    bad += 1;
                    eprintln!("COUNTEREXAMPLE case {case}: f64 floor {fl:e} > dist {upper:e}, size {s:e} {what}");
                }
            }
            Read::On => on += 1,
            Read::Unsettled(_) => uns += 1,
        }
    }
    eprintln!("[r2-big] off {off} on {on} unsettled {uns} bad {bad}");
    assert_eq!(bad, 0);
}

fn ring<T: Real>() -> geom::Surface<T> {
    let h = 1.2 / 2.0;
    let mut ku = vec![0.0; 3];
    ku.extend([0.5; 2]);
    ku.extend([1.0; 3]);
    let (mut control, mut weights) = (Vec::new(), Vec::new());
    for j in 0..=4 {
        let theta = j as f64 * h / 2.0;
        let (r, w) = if j % 2 == 1 { (1.0 / (h / 2.0).cos(), (h / 2.0).cos()) } else { (1.0, 1.0) };
        for z in [0.0, 1.0] {
            control.push(Point3::new(r * theta.cos(), r * theta.sin(), z));
            weights.push(w);
        }
    }
    let net = NurbsSurface::new(
        KnotVector::clamped(ku, 2).unwrap(),
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        control,
        weights,
    )
    .unwrap();
    geom::Surface::Nurbs(std::sync::Arc::new(net.map_scalar(T::from_f64)))
}

fn door<T: geom_core::Decide>(r: f64) -> Result<(), crate::PcurveCertifyError> {
    let band = Band::linear(geom_core::Tol::witness()).unwrap();
    let c = |x: f64| T::from_f64(x);
    let circle = geom::Curve3::circle(
        Point3::new(c(0.0), c(0.0), c(0.5)),
        Vec3::new(c(0.0), c(0.0), c(1.0)),
        c(r),
        Vec3::new(c(1.0), c(0.0), c(0.0)),
    );
    crate::read_edge_incidence(&circle, c(0.1), c(1.1), &ring::<T>(), band)
}

fn short(r: &Result<(), crate::PcurveCertifyError>) -> String {
    match r {
        Ok(()) => "On".into(),
        Err(crate::PcurveCertifyError::CarrierOffChart { .. }) => "Off".into(),
        Err(crate::PcurveCertifyError::Escalated { .. }) => "Escalated".into(),
        Err(e) => format!("{e:?}").chars().take(60).collect(),
    }
}

/// Timing of the door read against the ring wall, by distance in
/// escalates.
#[test]
fn r2_door_timing() {
    let band = Band::linear(geom_core::Tol::witness()).unwrap();
    eprintln!("band zero {:e} escalate {:e}", band.zero(), band.escalate());
    for k in [0.0, 0.5, 0.95, 1.05, 1.5, 3.0, 10.0, 100.0] {
        let r = 1.0 + k * band.escalate();
        let t = std::time::Instant::now();
        let a = door::<f64>(r);
        let tf = t.elapsed();
        let t = std::time::Instant::now();
        let b = door::<Interval>(r);
        let ti = t.elapsed();
        eprintln!("k {k:>6}: f64 {:>10} {:>8.3?}   Interval {:>10} {:>8.3?}", short(&a), tf, short(&b), ti);
    }
    // inside the wall (r < 1) by the same amounts
    for k in [1.5, 3.0, 10.0] {
        let r = 1.0 - k * band.escalate();
        let t = std::time::Instant::now();
        let a = door::<f64>(r);
        let tf = t.elapsed();
        let t = std::time::Instant::now();
        let b = door::<Interval>(r);
        let ti = t.elapsed();
        eprintln!("k -{k:>5}: f64 {:>10} {:>8.3?}   Interval {:>10} {:>8.3?}", short(&a), tf, short(&b), ti);
    }
}

/// Interval reads of on-chart and near-band samples: how they end, and
/// whether any ends with an invalid (definite) margin.
#[test]
fn r2_interval_endings() {
    let mut rng = test_utils::fuzz::start("r2-iv");
    let cases: usize = std::env::var("R2_CASES").ok().and_then(|s| s.parse().ok()).unwrap_or(300);
    let mut tally = std::collections::BTreeMap::<String, usize>::new();
    for case in 0..cases {
        let (chart, what) = patch(&mut rng);
        let lifted = chart.map_scalar(<Interval as Real>::from_f64);
        let band = if case % 2 == 0 { Band::new(1e-12, 1e-11).unwrap() } else { Band::new(1e-9, 1e-8).unwrap() };
        let ((u0, u1), (v0, v1)) = (chart.knot_values_u().domain(), chart.knot_values_v().domain());
        let (u, v) = (rng.range(u0, u1), rng.range(v0, v1));
        let pi = lifted.eval(<Interval as Real>::from_f64(u), <Interval as Real>::from_f64(v));
        let mid = |x: Interval| 0.5 * (x.lo() + x.hi());
        let q = Point3::new(mid(pi.x), mid(pi.y), mid(pi.z));
        let k = rng.below(3);
        let d = [0.0, 0.5, 3.0][k] * band.escalate();
        let q = q + normal(&chart, u, v) * d;
        let qi = q.map(<Interval as Real>::from_f64);
        if let Read::Off(f) = Net::of(&chart, band).expect("cuts").read(q, band) {
            let hx = |x: f64| format!("\"{}\"", x.to_bits());
            let ku: Vec<String> = chart.knots_u_f64_interim().knots().iter().map(|&x| hx(x)).collect();
            let kv: Vec<String> = chart.knots_v_f64_interim().knots().iter().map(|&x| hx(x)).collect();
            let cp: Vec<String> = chart.control().iter().map(|p| format!("[{},{},{}]", hx(p.x), hx(p.y), hx(p.z))).collect();
            let w: Vec<String> = chart.weights().iter().map(|&x| hx(x)).collect();
            let (nu, nv) = chart.control_counts();
            eprintln!("F64JSON {{\"case\":{case},\"pu\":{},\"pv\":{},\"nu\":{nu},\"nv\":{nv},\"ku\":[{}],\"kv\":[{}],\"cp\":[{}],\"w\":[{}],\"u\":{},\"v\":{},\"q\":[{},{},{}],\"floor\":{},\"esc\":{}}}",
                chart.knots_u_f64_interim().degree(), chart.knots_v_f64_interim().degree(),
                ku.join(","), kv.join(","), cp.join(","), w.join(","), hx(u), hx(v), hx(q.x), hx(q.y), hx(q.z), hx(f), hx(band.escalate()));
        }
        let t = std::time::Instant::now();
        let r = Net::of(&lifted, band).expect("cuts").read(qi, band);
        let el = t.elapsed();
        let key = match r {
            Read::On => format!("d{k} On"),
            Read::Off(f) => {
                let upper = (qi - pi).norm().hi();
                let w = (pi.x.hi() - pi.x.lo()).max(pi.y.hi() - pi.y.lo()).max(pi.z.hi() - pi.z.lo());
                eprintln!("OFF case {case} d{k}: floor [{:e},{:e}] upper {upper:e} eval width {w:e} band {:e}/{:e} u {u} v {v} {what}", f.lo(), f.hi(), band.zero(), band.escalate());
                // f64 read of the same q
                let rf = Net::of(&chart, band).expect("cuts").read(q, band);
                eprintln!("   f64 read: {}", match rf { Read::On => "On".to_string(), Read::Off(x) => format!("Off {x:e}"), Read::Unsettled(_) => "Unsettled".to_string() });
                if f.lo() > upper { eprintln!("   COUNTEREXAMPLE: floor.lo > upper"); }
                {
                    let hx = |x: f64| format!("\"{}\"", x.to_bits());
                    let ku: Vec<String> = chart.knots_u_f64_interim().knots().iter().map(|&x| hx(x)).collect();
                    let kv: Vec<String> = chart.knots_v_f64_interim().knots().iter().map(|&x| hx(x)).collect();
                    let cp: Vec<String> = chart.control().iter().map(|p| format!("[{},{},{}]", hx(p.x), hx(p.y), hx(p.z))).collect();
                    let w: Vec<String> = chart.weights().iter().map(|&x| hx(x)).collect();
                    let (nu, nv) = chart.control_counts();
                    eprintln!("JSON {{\"case\":{case},\"pu\":{},\"pv\":{},\"nu\":{nu},\"nv\":{nv},\"ku\":[{}],\"kv\":[{}],\"cp\":[{}],\"w\":[{}],\"u\":{},\"v\":{},\"q\":[{},{},{}],\"floor\":{},\"esc\":{}}}",
                        chart.knots_u_f64_interim().degree(), chart.knots_v_f64_interim().degree(),
                        ku.join(","), kv.join(","), cp.join(","), w.join(","), hx(u), hx(v), hx(q.x), hx(q.y), hx(q.z), hx(f.lo()), hx(band.escalate()));
                }
                format!("d{k} Off")
            }
            Read::Unsettled(c) if c.margin.is_invalid() => {
                eprintln!("INVALID case {case} d{k} {what} name {:?}", c.predicate);
                format!("d{k} Unsettled-INVALID")
            }
            Read::Unsettled(_) => format!("d{k} Unsettled"),
        };
        if el.as_millis() > 200 {
            eprintln!("SLOW {el:?} case {case} d{k} {key} {what}");
        }
        *tally.entry(key).or_default() += 1;
    }
    eprintln!("[r2-iv] {tally:?}");
}

/// Worst single-sample read time by lane over near-band samples, and a
/// whole 65-sample door read on the slowest patch found.
#[test]
fn r2_read_cost() {
    let mut rng = test_utils::fuzz::start("r2-cost");
    let cases: usize = std::env::var("R2_CASES").ok().and_then(|s| s.parse().ok()).unwrap_or(300);
    let mut worst = [(std::time::Duration::ZERO, String::new()), (std::time::Duration::ZERO, String::new())];
    let mut slowest: Option<(std::time::Duration, NurbsSurface<f64>, f64, f64, f64, Band)> = None;
    for case in 0..cases {
        let (chart, what) = patch(&mut rng);
        let lifted = chart.map_scalar(<Interval as Real>::from_f64);
        let band = Band::new(1e-9, 1e-8).unwrap();
        let ((u0, u1), (v0, v1)) = (chart.knot_values_u().domain(), chart.knot_values_v().domain());
        let (u, v) = (rng.range(u0, u1), rng.range(v0, v1));
        let d = rng.range(1.0, 4.0) * band.escalate();
        let q = chart.eval(u, v) + normal(&chart, u, v) * d;
        let t = std::time::Instant::now();
        let _ = Net::of(&chart, band).unwrap().read(q, band);
        let ef = t.elapsed();
        let t = std::time::Instant::now();
        let _ = Net::of(&lifted, band).unwrap().read(q.map(<Interval as Real>::from_f64), band);
        let ei = t.elapsed();
        if ef > worst[0].0 { worst[0] = (ef, format!("case {case} {what}")); }
        if ei > worst[1].0 { worst[1] = (ei, format!("case {case} {what}")); }
        let key = if std::env::var("R2_BY_IV").is_ok() { ei } else { ef }; if slowest.as_ref().is_none_or(|s| key > s.0) { slowest = Some((key, chart.clone(), u, v, d, band)); }
    }
    eprintln!("[r2-cost] worst f64 sample {:?} ({}), worst Interval sample {:?} ({})", worst[0].0, worst[0].1, worst[1].0, worst[1].1);
    // A door read: a line through 65 samples near the slowest sample's spot, along the
    // tangent, offset by d along the normal (a chord of the tangent plane: near band at its middle).
    let (_, chart, u, v, d, band) = slowest.unwrap();
    let n = normal(&chart, u, v);
    let p = chart.eval(u, v) + n * d;
    let ((u0, u1), _) = (chart.knot_values_u().domain(), chart.knot_values_v().domain());
    let h = (u1 - u0) * 1e-4;
    let tan = (chart.eval((u + h).min(u1), v) - chart.eval((u - h).max(u0), v)).normalize();
    let len = 1e-4 * (chart.eval(u1, v) - chart.eval(u0, v)).norm().max(1e-9);
    let surface = geom::Surface::Nurbs(std::sync::Arc::new(chart.clone()));
    let line = geom::Curve3::line(p - tan * len, tan);
    let t = std::time::Instant::now();
    let r = crate::read_edge_incidence(&line, 0.0, 2.0 * len, &surface, band);
    eprintln!("[r2-cost] f64 door read on it: {:?} -> {}", t.elapsed(), short(&r));
    let li = chart.map_scalar(<Interval as Real>::from_f64);
    let si = geom::Surface::Nurbs(std::sync::Arc::new(li));
    let c = |x: f64| <Interval as Real>::from_f64(x);
    let pi = (p - tan * len).map(c);
    let line_i = geom::Curve3::line(pi, tan.map(c));
    let t = std::time::Instant::now();
    let r = crate::read_edge_incidence(&line_i, c(0.0), c(2.0 * len), &si, band);
    eprintln!("[r2-cost] Interval door read on it: {:?} -> {}", t.elapsed(), short(&r));
}
