//! Delta-review probe for PR 3805 (fix pass). Included into
//! `crates/topo/src/boolean/mod.rs` with
//! `#[cfg(test)] #[path = "../../../../probes/reach-delta-3805/fuzz_3805.rs"] mod fuzz_3805;`
//! Run: `cargo test -p topo --lib fuzz_3805 -- --nocapture`.
//! Oracle: the TRUE Euclidean distance (sphere |p-c|-r, wall rho-r),
//! sampled densely and bisected; touches found by golden-section on |d|.
#![allow(clippy::all, clippy::pedantic, clippy::unwrap_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use super::circle_roots::CircleRoots;
use geom_core::{Band, Point3, Vec3};
use test_utils::fuzz;

fn unit(rng: &mut fuzz::Rng) -> Vec3<f64> {
    loop {
        let v = Vec3::new(rng.range(-1.0, 1.0), rng.range(-1.0, 1.0), rng.range(-1.0, 1.0));
        if v.norm() > 0.2 && v.norm() < 1.0 {
            return v.normalize();
        }
    }
}
fn perp(rng: &mut fuzz::Rng, t: Vec3<f64>) -> Vec3<f64> {
    let t = t.normalize();
    let v = unit(rng);
    let w = v - t * v.dot(t);
    if w.norm() < 1e-3 { perp(rng, t) } else { w.normalize() }
}

pub(super) fn distance(s: &geom::Surface<f64>, p: Point3<f64>) -> f64 {
    match *s {
        geom::Surface::Sphere { center, radius, .. } => (p - center).norm() - radius,
        geom::Surface::Cylinder { origin, axis, radius, .. } => {
            let w = p - origin;
            (w - axis * w.dot(axis)).norm() - radius
        }
        _ => unreachable!(),
    }
}

pub(super) fn call(
    c: &geom::Curve3<f64>,
    t0: f64,
    t1: f64,
    s: &geom::Surface<f64>,
    band: Band,
) -> Result<CircleRoots<f64>, super::BooleanError> {
    match (c, s) {
        (geom::Curve3::Ellipse { .. }, _) => super::ellipse_roots::ellipse_roots(c, t0, t1, s, band),
        (geom::Curve3::Circle { .. }, geom::Surface::Cylinder { .. }) => {
            super::circle_cylinder::circle_cylinder_roots(c, t0, t1, s, band)
        }
        (geom::Curve3::Circle { .. }, geom::Surface::Sphere { .. }) => {
            super::circle_sphere::circle_sphere_roots(c, t0, t1, s, band)
        }
        _ => unreachable!(),
    }
}

pub(super) struct Truth {
    pub roots: Vec<f64>,
    pub closest: f64,
    pub max_abs: f64,
}

pub(super) fn truth(c: &geom::Curve3<f64>, s: &geom::Surface<f64>, mid: f64, dense: usize) -> Truth {
    let d = |t: f64| distance(s, c.eval(t));
    let ts: Vec<f64> = (0..=dense).map(|k| mid - PI + TAU * k as f64 / dense as f64).collect();
    let ds: Vec<f64> = ts.iter().map(|&t| d(t)).collect();
    let mut roots = Vec::new();
    let mut closest = f64::INFINITY;
    let mut max_abs: f64 = 0.0;
    for k in 0..dense {
        max_abs = max_abs.max(ds[k].abs());
        if ds[k] == 0.0 {
            roots.push(ts[k]);
            closest = 0.0;
            continue;
        }
        if ds[k].signum() != ds[k + 1].signum() && ds[k + 1] != 0.0 {
            let (mut lo, mut hi) = (ts[k], ts[k + 1]);
            let slo = ds[k].signum();
            for _ in 0..80 {
                let m = 0.5 * (lo + hi);
                if d(m).signum() == slo { lo = m } else { hi = m }
            }
            roots.push(0.5 * (lo + hi));
            closest = 0.0;
        }
    }
    // Local minima of |d| with no sampled sign change: refine the signed
    // extremum by golden section; an extremum of the opposite sign is a
    // root pair the sampling stepped over, bisected on each side.
    for k in 1..dense {
        let (a, b, cc) = (ds[k - 1].abs(), ds[k].abs(), ds[k + 1].abs());
        if b <= a && b <= cc && ds[k - 1].signum() == ds[k].signum() && ds[k].signum() == ds[k + 1].signum() {
            let sg = ds[k].signum();
            let f = |t: f64| sg * d(t);
            let (mut lo, mut hi) = (ts[k - 1], ts[k + 1]);
            let g = 0.618_033_988_749_894_9;
            for _ in 0..120 {
                let x1 = hi - g * (hi - lo);
                let x2 = lo + g * (hi - lo);
                if f(x1) < f(x2) { hi = x2 } else { lo = x1 }
            }
            let ts_ = 0.5 * (lo + hi);
            let v = d(ts_);
            closest = closest.min(v.abs()).min(b);
            if v.signum() != sg && v != 0.0 {
                for (mut l, mut h) in [(ts[k - 1], ts_), (ts_, ts[k + 1])] {
                    let sl = d(l).signum();
                    for _ in 0..80 {
                        let m = 0.5 * (l + h);
                        if d(m).signum() == sl { l = m } else { h = m }
                    }
                    roots.push(0.5 * (l + h));
                }
            }
        }
    }
    roots.sort_by(|x, y| x.partial_cmp(y).unwrap());
    closest = closest.min(ds.iter().fold(f64::INFINITY, |m, x| m.min(x.abs())));
    Truth { roots, closest, max_abs }
}

#[derive(Default, Debug)]
pub(super) struct Tally {
    pub certified: usize,
    pub miss: usize,
    pub uncertain: usize,
    pub on: usize,
    pub escalated: usize,
    pub disagrees: usize,
    pub off_band: Vec<String>,
    pub count_wrong: Vec<String>,
    pub bad_miss: Vec<String>,
    pub near_miss_gap: usize,
    pub worst_off_over_eps: f64,
}

pub(super) fn judge(
    tally: &mut Tally,
    label: &str,
    c: &geom::Curve3<f64>,
    s: &geom::Surface<f64>,
    t0: f64,
    t1: f64,
    eps: f64,
    oracle_noise: f64,
) {
    let band = Band::new(eps, 10.0 * eps).unwrap();
    let got = call(c, t0, t1, s, band);
    let mid = 0.5 * (t0 + t1);
    let tr = truth(c, s, mid, 20_000);
    match got {
        Err(_) => tally.escalated += 1,
        Ok(CircleRoots::Uncertain) => tally.uncertain += 1,
        Ok(CircleRoots::CountDisagrees) => tally.disagrees += 1,
        Ok(CircleRoots::OnSurface) => {
            tally.on += 1;
            if tr.max_abs > eps + oracle_noise {
                tally.off_band.push(format!("{label}: OnSurface but max |d| {:e}", tr.max_abs));
            }
        }
        Ok(CircleRoots::Miss) => {
            tally.miss += 1;
            if !tr.roots.is_empty() || tr.closest <= eps {
                tally.bad_miss.push(format!(
                    "{label}: Miss with {} true roots, closest {:e}",
                    tr.roots.len(),
                    tr.closest
                ));
            } else if tr.closest <= 10.0 * eps {
                tally.near_miss_gap += 1;
            }
        }
        Ok(CircleRoots::Certified { count, thetas }) => {
            tally.certified += 1;
            for &t in &thetas[..count] {
                let off = distance(s, c.eval(t)).abs();
                tally.worst_off_over_eps = tally.worst_off_over_eps.max(off / eps);
                if off > eps + oracle_noise {
                    tally.off_band.push(format!("{label}: root {t} lies {off:e} off (eps {eps:e})"));
                }
            }
            if count != tr.roots.len() {
                tally.count_wrong.push(format!(
                    "{label}: certified {count}, true {} (closest {:e}) roots {:?} vs {:?}",
                    tr.roots.len(),
                    tr.closest,
                    &thetas[..count],
                    tr.roots
                ));
            }
        }
    }
}

fn make_carrier(rng: &mut fuzz::Rng, scale: f64, circle: bool, any_frame: bool) -> geom::Curve3<f64> {
    let n = unit(rng);
    let u = unit(rng);
    let u_ref = (u - n * u.dot(n)).normalize();
    let center = Point3::new(rng.range(-1.0, 1.0) * scale, rng.range(-1.0, 1.0) * scale, rng.range(-1.0, 1.0) * scale);
    let big = rng.range(0.5, 5.0) * scale;
    if circle {
        return geom::Curve3::Circle { center, axis: n, radius: big, u_ref };
    }
    // eccentricity (ratio) up to 40, log-uniform
    let small = big / 10f64.powf(rng.range(0.0, 40f64.log10()));
    let (mut major, mut minor) = if !any_frame || rng.below(2) == 0 { (big, small) } else { (small, big) };
    if any_frame && rng.below(3) == 0 {
        major = -major;
    }
    if any_frame && rng.below(3) == 0 {
        minor = -minor;
    }
    geom::Curve3::Ellipse { center, axis: n, major, minor, u_ref }
}

/// mode 0: surface through a random carrier point (crossing); mode 1: a
/// surface tangent at a random point, pushed by a depth of a few bands.
fn make_surface(
    rng: &mut fuzz::Rng,
    c: &geom::Curve3<f64>,
    scale: f64,
    sphere: bool,
    mode: usize,
    eps: f64,
) -> geom::Surface<f64> {
    let th = rng.range(0.0, TAU);
    let p = c.eval(th);
    let h = 1e-7;
    let tang = (c.eval(th + h) - c.eval(th - h)).normalize();
    let r = scale * 10f64.powf(rng.range(-1.5, 1.0));
    let x = Vec3::new(1.0, 0.0, 0.0);
    let depth = if mode == 0 {
        0.0
    } else {
        let k = [0.0, 0.3, 0.9, 2.0, 5.0, 12.0, 40.0][rng.below(7)];
        if rng.below(2) == 0 { k * eps } else { -k * eps }
    };
    if mode == 0 {
        let w = unit(rng);
        if sphere {
            return geom::Surface::Sphere { center: p + unit(rng) * r, radius: r, axis: w, u_ref: (x - w * x.dot(w)).normalize() };
        }
        return geom::Surface::Cylinder {
            origin: p + unit(rng).cross(w).normalize() * r,
            axis: w,
            radius: r,
            u_ref: (x - w * x.dot(w)).normalize(),
        };
    }
    let nrm = perp(rng, tang);
    if sphere {
        let w = unit(rng);
        geom::Surface::Sphere { center: p + nrm * (r - depth), radius: r, axis: w, u_ref: (x - w * x.dot(w)).normalize() }
    } else {
        let w = perp(rng, nrm);
        geom::Surface::Cylinder { origin: p + nrm * (r - depth), axis: w, radius: r, u_ref: (x - w * x.dot(w)).normalize() }
    }
}

fn sweep(label: &str, circle: bool, any_frame: bool, per: usize) -> Vec<String> {
    let mut rng = fuzz::start(label);
    let mut report = Vec::new();
    let mut bad = Vec::new();
    for (sname, scale) in [("mm", 1e-3), ("m", 1.0), ("km", 1e3)] {
        for eps in [1e-9, 1e-6, 1e-12] {
            for mode in 0..2 {
                let mut tally = Tally::default();
                for i in 0..per {
                    let c = make_carrier(&mut rng, scale, circle, any_frame);
                    let sphere = i % 2 == 1;
                    let s = make_surface(&mut rng, &c, scale, sphere, mode, eps);
                    let t0 = rng.range(0.0, TAU);
                    let t1 = t0 + rng.range(0.1, TAU);
                    // oracle's own rounding: a few ulps of the coordinates
                    let oracle_noise = 64.0 * f64::EPSILON * scale * 10.0;
                    let lab = format!(
                        "{label} {sname} eps {eps:e} mode {mode} case {i} {} — {c:?} vs {s:?} on [{t0}, {t1}]",
                        if sphere { "sphere" } else { "wall" }
                    );
                    judge(&mut tally, &lab, &c, &s, t0, t1, eps, oracle_noise);
                }
                report.push(format!(
                    "{label} {sname} eps {eps:e} mode {mode}: cert {} miss {} unc {} on {} esc {} disagree {} | off_band {} count_wrong {} bad_miss {} near_miss(eps..10eps) {} worst_off/eps {:.3}",
                    tally.certified, tally.miss, tally.uncertain, tally.on, tally.escalated, tally.disagrees,
                    tally.off_band.len(), tally.count_wrong.len(), tally.bad_miss.len(), tally.near_miss_gap, tally.worst_off_over_eps
                ));
                bad.extend(tally.off_band.into_iter().take(3));
                bad.extend(tally.count_wrong.into_iter().take(3));
                bad.extend(tally.bad_miss.into_iter().take(3));
            }
        }
    }
    for l in &report {
        println!("{l}");
    }
    for b in &bad {
        println!("BAD {b}");
    }
    bad
}

#[test]
fn probe_ellipse_stored_order() {
    let bad = sweep("p3805_ellipse", false, false, fuzz::scaled(120));
    assert!(bad.is_empty(), "{} bad", bad.len());
}

#[test]
fn probe_ellipse_any_frame() {
    let bad = sweep("p3805_ellipse_anyframe", false, true, fuzz::scaled(120));
    assert!(bad.is_empty(), "{} bad", bad.len());
}

#[test]
fn probe_circle() {
    let bad = sweep("p3805_circle", true, false, fuzz::scaled(120));
    assert!(bad.is_empty(), "{} bad", bad.len());
}

/// The km-scale certified Miss the fuzz found (ellipse_anyframe km eps
/// 1e-12 mode 1 case 83), instrumented.
#[test]
fn probe_km_miss_case() {
    let c = geom::Curve3::Ellipse {
        center: Point3::new(-959.2639129088163, -367.02388074477653, 623.0471277842355),
        axis: Vec3::new(-0.31903108529555385, -0.8958001297855095, -0.3094532179367431),
        major: -3514.3154468033267,
        minor: -2361.48048802387,
        u_ref: Vec3::new(-0.1811201960417648, -0.2628702620852155, 0.9476785846989699),
    };
    let s = geom::Surface::Sphere {
        center: Point3::new(-2850.858173756086, -510.3082903169221, 2988.8851976595456),
        radius: 39.170514333982986,
        axis: Vec3::new(-0.40357028177242227, -0.5610344053254761, 0.7227526711892646),
        u_ref: Vec3::new(0.9149486475590463, -0.2474639572890499, 0.3187954864371759),
    };
    let (t0, t1) = (5.382086155163396, 9.981586155095057);
    let conic = geom_brep::Conic::of(&c).unwrap();
    let h = geom_brep::conic_sphere_harmonics(&conic, Point3::new(-2850.858173756086, -510.3082903169221, 2988.8851976595456), 39.170514333982986);
    let te = 10.142998946228167;
    let f = |t: f64| h.c0 + h.c1 * t.cos() + h.s1 * t.sin() + h.c2 * (2.0 * t).cos() + h.s2 * (2.0 * t).sin();
    println!("h = {h:?}");
    println!("noise = {:e}", super::circle_roots::rounding_charge(h.terms) / (2.0 * 39.170514333982986));
    println!("F(te) = {:e}, implicit_residual(te) = {:e}, dist(te) = {:e}", f(te), geom_brep::implicit_residual(&s, conic.point(te)), distance(&s, c.eval(te)));
    for eps in [1e-12, 3e-12, 1e-11] {
        let band = Band::new(eps, 10.0 * eps).unwrap();
        println!("eps {eps:e}: {:?}", call(&c, t0, t1, &s, band));
    }
}

/// The same geometric ellipse in its four stored frames (sign of major via
/// a flipped u_ref; order swapped via u_ref -> v_ref): the door's answer
/// and its `terms` must not depend on the frame.
#[test]
fn probe_km_miss_case_frames() {
    let n = Vec3::new(-0.31903108529555385, -0.8958001297855095, -0.3094532179367431);
    let u = Vec3::new(-0.1811201960417648, -0.2628702620852155, 0.9476785846989699);
    let center = Point3::new(-959.2639129088163, -367.02388074477653, 623.0471277842355);
    let (a, b) = (-3514.3154468033267, -2361.48048802387);
    let sc = Point3::new(-2850.858173756086, -510.3082903169221, 2988.8851976595456);
    let s = geom::Surface::Sphere {
        center: sc,
        radius: 39.170514333982986,
        axis: Vec3::new(-0.40357028177242227, -0.5610344053254761, 0.7227526711892646),
        u_ref: Vec3::new(0.9149486475590463, -0.2474639572890499, 0.3187954864371759),
    };
    let (t0, t1) = (5.382086155163396, 9.981586155095057);
    let band = Band::new(1e-12, 1e-11).unwrap();
    // theta is the same point in frames 0 and 1; frames 2/3 shift theta by pi/2,
    // so give them the whole-turn arc shifted too.
    let v = n.cross(u);
    let frames = [
        ("stored (-a,-b)", u, a, b, 0.0),
        ("positive (a,b), u and v flipped", -u, -a, -b, 0.0),
        ("positive, order swapped (b along v)", v * -1.0, -b, -a, 0.0),
    ];
    for (name, uu, maj, min, shift) in frames {
        let c = geom::Curve3::Ellipse { center, axis: if name.contains("swapped") { -n } else { n }, major: maj, minor: min, u_ref: uu };
        let conic = geom_brep::Conic::of(&c).unwrap();
        let h = geom_brep::conic_sphere_harmonics(&conic, sc, 39.170514333982986);
        // the same point set?
        let p0 = c.eval(10.142998946228167);
        let got = call(&c, t0 + shift, t1 + shift, &s, band);
        println!("{name}: terms {:e}, speed_hi {}, eval(te) dist {:e}, answer {got:?}", h.terms, conic.speed_hi(), distance(&s, p0));
    }
}

/// Targeted: metre-scale ellipse stored with a NEGATIVE major, a small
/// surface grazing near a major vertex (|C0 - o| ~ |major|, so the signed
/// `terms` (|d| + major)^2 collapses to ~r^2), depth a few bands.
#[test]
fn probe_negative_major_metre_scale() {
    let mut rng = fuzz::start("p3805_negmajor");
    for eps in [1e-9, 1e-12] {
        let mut tally = Tally::default();
        let mut tally_pos = Tally::default();
        for i in 0..fuzz::scaled(800) {
            let n = unit(&mut rng);
            let u0 = unit(&mut rng);
            let u = (u0 - n * u0.dot(n)).normalize();
            let a = rng.range(0.5, 5.0);
            let b = a / rng.range(1.0, 40.0);
            let center = Point3::new(rng.range(-1.0, 1.0), rng.range(-1.0, 1.0), rng.range(-1.0, 1.0));
            let neg = geom::Curve3::Ellipse { center, axis: n, major: -a, minor: b, u_ref: -u };
            let pos = geom::Curve3::Ellipse { center, axis: n, major: a, minor: -b, u_ref: u };
            // NB pos = same point set as neg? -a*(-u) = a u; v' = n x (-u) = -v so b*(-v)... use minor -b with u: v = n x u, -b v. neg: b * (n x -u) = -b v. same.
            let th = if rng.below(2) == 0 { 0.0 } else { PI } + rng.range(-0.05, 0.05);
            let p = neg.eval(th);
            let tang = (neg.eval(th + 1e-7) - neg.eval(th - 1e-7)).normalize();
            let r = 10f64.powf(rng.range(-6.0, -2.0));
            let k = [0.0, 0.5, 2.0, 5.0, 12.0, 30.0, 100.0][rng.below(7)];
            let depth = if rng.below(2) == 0 { k * eps } else { -k * eps };
            let nrm = perp(&mut rng, tang);
            let x = Vec3::new(1.0, 0.0, 0.0);
            let s = if i % 2 == 0 {
                let w = unit(&mut rng);
                geom::Surface::Sphere { center: p + nrm * (r - depth), radius: r, axis: w, u_ref: (x - w * x.dot(w)).normalize() }
            } else {
                let w = perp(&mut rng, nrm);
                geom::Surface::Cylinder { origin: p + nrm * (r - depth), axis: w, radius: r, u_ref: (x - w * x.dot(w)).normalize() }
            };
            let t0 = rng.range(0.0, TAU);
            let t1 = t0 + rng.range(0.1, TAU);
            let noise = 64.0 * f64::EPSILON * 10.0;
            judge(&mut tally, &format!("neg eps {eps:e} case {i} r {r:e} k {k} — {neg:?} vs {s:?} on [{t0}, {t1}]"), &neg, &s, t0, t1, eps, noise);
            judge(&mut tally_pos, &format!("pos eps {eps:e} case {i}"), &pos, &s, t0, t1, eps, noise);
        }
        for (name, t) in [("negative major", &tally), ("positive major, same ellipse", &tally_pos)] {
            println!(
                "{name} eps {eps:e}: cert {} miss {} unc {} esc {} | off_band {} count_wrong {} bad_miss {} near_miss {}",
                t.certified, t.miss, t.uncertain, t.escalated, t.off_band.len(), t.count_wrong.len(), t.bad_miss.len(), t.near_miss_gap
            );
            for b in t.bad_miss.iter().chain(&t.off_band).chain(&t.count_wrong).take(4) {
                println!("BAD {} ... {}", b.chars().take(60).collect::<String>(), b.rsplit("]: ").next().unwrap());
            }
        }
    }
}

/// Claim 3: tangencies. An ellipse inside a wall about z touching it at
/// theta = +-pi/2 (its y semi-axis the wall radius, tilt lengthening it to
/// match), at r 50 and 500 m, eps 1e-9; and a circle of the wall's radius
/// tilted inside it at r 5 m, shifted by depth along x.
#[test]
fn probe_tangencies() {
    let wall = |r: f64| geom::Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: r,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    for eps in [1e-9, 1e-6, 1e-12] {
        let band = Band::new(eps, 10.0 * eps).unwrap();
        for r in [50.0, 500.0, 5.0] {
            for tilt in [0.0f64, 0.3, 1.0] {
                for k in [0.0, 0.5, -0.5, 3.0, -3.0, 20.0, -20.0, 1e3, -1e3] {
                    let depth = k * eps;
                    let n = Vec3::new(0.0, -tilt.sin(), tilt.cos());
                    let e = geom::Curve3::Ellipse {
                        center: Point3::new(0.0, 0.0, 0.0),
                        axis: n,
                        major: 0.6 * r,
                        minor: (r + depth) / tilt.cos(),
                        u_ref: Vec3::new(1.0, 0.0, 0.0),
                    };
                    let s = wall(r);
                    let got = call(&e, 0.0, PI, &s, band);
                    let tr = truth(&e, &s, PI / 2.0, 20_000);
                    let ok = match &got {
                        Ok(CircleRoots::Miss) => tr.roots.is_empty() && tr.closest > eps,
                        Ok(CircleRoots::Certified { count, thetas }) => {
                            *count == tr.roots.len() && thetas[..*count].iter().all(|&t| distance(&s, e.eval(t)).abs() <= eps)
                        }
                        _ => true,
                    };
                    println!(
                        "{} ellipse eps {eps:e} r {r} tilt {tilt} depth {depth:e}: {:?} (true roots {}, closest {:e})",
                        if ok { "ok " } else { "BAD" },
                        got.as_ref().map(|g| match g { CircleRoots::Certified { count, .. } => format!("Certified{count}"), o => format!("{o:?}") }).map_err(|_| "Err"),
                        tr.roots.len(),
                        tr.closest
                    );
                }
            }
        }
        // circle of the wall's radius tilted inside it
        for r in [5.0, 50.0, 500.0] {
            for k in [0.0, 0.5, 3.0, -3.0, 20.0, 5e-6 / eps, -5e-6 / eps] {
                let depth = k * eps;
                let tilt = 0.3f64;
                let c = geom::Curve3::Circle {
                    center: Point3::new(depth, 0.0, 0.0),
                    axis: Vec3::new(0.0, -tilt.sin(), tilt.cos()),
                    radius: r,
                    u_ref: Vec3::new(1.0, 0.0, 0.0),
                };
                let s = wall(r);
                let got = call(&c, -1.0, 1.0, &s, band);
                let tr = truth(&c, &s, 0.0, 20_000);
                let ok = match &got {
                    Ok(CircleRoots::Miss) => tr.roots.is_empty() && tr.closest > eps,
                    Ok(CircleRoots::Certified { count, thetas }) => {
                        *count == tr.roots.len() && thetas[..*count].iter().all(|&t| distance(&s, c.eval(t)).abs() <= eps)
                    }
                    _ => true,
                };
                println!(
                    "{} circle eps {eps:e} r {r} depth {depth:e}: {:?} (true roots {}, closest {:e})",
                    if ok { "ok " } else { "BAD" },
                    got.as_ref().map(|g| match g { CircleRoots::Certified { count, .. } => format!("Certified{count}"), o => format!("{o:?}") }).map_err(|e| format!("{e:?}").chars().take(60).collect::<String>()),
                    tr.roots.len(),
                    tr.closest
                );
            }
        }
    }
}
