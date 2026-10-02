//! Reviewer probe (reach-dual3805-r2). Not for merge.
//!
//! 1. `ellipse_roots` fuzzed against the bisected sign changes of the TRUE
//!    distance (my own, not the kernel's residual) on eccentric ellipses,
//!    spheres and tilted walls, plus near-section (first-harmonic arm) poses.
//! 2. `conic_arc_residual_range` / `conic_residual_extremes` against a
//!    dense sampling of my own residual, sphere / wall / torus / plane.
//! 3. The frame premise: a swapped (`minor > major`) or negative-major
//!    ellipse, which the mint certifies, fed to the same enclosures.

#![allow(clippy::unwrap_used, clippy::panic, clippy::cast_precision_loss)]

use core::f64::consts::{PI, TAU};

use geom_core::{Band, Point3, Tol, Vec3};

use super::circle_roots::CircleRoots;
use super::ellipse_roots::ellipse_roots;

struct Lcg(u64);
impl Lcg {
    fn f(&mut self) -> f64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
    fn r(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * self.f()
    }
    fn unit(&mut self) -> Vec3<f64> {
        loop {
            let v = Vec3::new(self.r(-1.0, 1.0), self.r(-1.0, 1.0), self.r(-1.0, 1.0));
            let n = v.norm();
            if n > 0.2 && n < 1.0 {
                return v / n;
            }
        }
    }
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn ell(c: Point3<f64>, n: Vec3<f64>, u: Vec3<f64>, a: f64, b: f64) -> geom::Curve3<f64> {
    let u = (u - n * u.dot(n)).normalize();
    geom::Curve3::Ellipse {
        center: c,
        axis: n,
        major: a,
        minor: b,
        u_ref: u,
    }
}

fn pt(e: &geom::Curve3<f64>, t: f64) -> Point3<f64> {
    let geom::Curve3::Ellipse {
        center,
        axis,
        major,
        minor,
        u_ref,
    } = *e
    else {
        unreachable!()
    };
    let v = axis.cross(u_ref);
    center + u_ref * (major * t.cos()) + v * (minor * t.sin())
}

#[derive(Clone, Copy)]
enum S {
    Sphere(Point3<f64>, f64),
    Wall(Point3<f64>, Vec3<f64>, f64),
    Torus(Point3<f64>, Vec3<f64>, f64, f64),
    Plane(Point3<f64>, Vec3<f64>),
}

fn surf(s: S) -> geom::Surface<f64> {
    let x = Vec3::new(1.0, 0.0, 0.0);
    let y = Vec3::new(0.0, 1.0, 0.0);
    let perp = |a: Vec3<f64>| {
        let t = if a.dot(x).abs() < 0.9 { x } else { y };
        (t - a * t.dot(a)).normalize()
    };
    match s {
        S::Sphere(c, r) => geom::Surface::Sphere {
            center: c,
            radius: r,
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: x,
        },
        S::Wall(o, a, r) => geom::Surface::Cylinder {
            origin: o,
            axis: a,
            radius: r,
            u_ref: perp(a),
        },
        S::Torus(c, a, big, small) => geom::Surface::Torus {
            center: c,
            axis: a,
            major_radius: big,
            minor_radius: small,
            u_ref: perp(a),
        },
        S::Plane(o, n) => geom::Surface::Plane {
            origin: o,
            normal: n,
            u_ref: perp(n),
        },
    }
}

/// True signed distance (my own).
fn dist(s: S, p: Point3<f64>) -> f64 {
    match s {
        S::Sphere(c, r) => (p - c).norm() - r,
        S::Wall(o, a, r) => {
            let w = p - o;
            (w - a * w.dot(a)).norm() - r
        }
        S::Torus(c, a, big, small) => {
            let w = p - c;
            let h = w.dot(a);
            let rho = (w - a * h).norm();
            (rho - big).hypot(h) - small
        }
        S::Plane(o, n) => (p - o).dot(n),
    }
}

/// Linearized residual matching the kernel's units (my own algebra).
fn resid(s: S, p: Point3<f64>) -> f64 {
    match s {
        S::Sphere(c, r) => ((p - c).norm_squared() - r * r) / (2.0 * r),
        S::Wall(o, a, r) => {
            let w = p - o;
            ((w - a * w.dot(a)).norm_squared() - r * r) / (2.0 * r)
        }
        S::Torus(c, a, big, small) => {
            let w = p - c;
            let h = w.dot(a);
            let rho = (w - a * h).norm();
            ((rho - big).powi(2) + h * h - small * small) / (2.0 * small)
        }
        S::Plane(o, n) => (p - o).dot(n),
    }
}

/// Oracle: roots (bisected) of the true distance over a full turn about
/// `mid`, plus the smallest |extremum| of the distance that does not
/// change sign (a near-tangency the grid cannot split).
fn oracle(e: &geom::Curve3<f64>, s: S, mid: f64) -> (Vec<f64>, f64) {
    let f = |t: f64| dist(s, pt(e, t));
    let n = 40_000u32;
    let at = |k: u32| mid - PI + TAU * f64::from(k) / f64::from(n);
    let mut roots = Vec::new();
    let mut near = f64::INFINITY;
    let mut prev = f(at(0));
    for k in 0..n {
        let (mut a, mut b) = (at(k), at(k + 1));
        let fb = f(b);
        // local extremum of |f| between k-1..k+1
        let fa = prev;
        if fa.signum() != fb.signum() {
            for _ in 0..100 {
                let m = 0.5 * (a + b);
                if f(m).signum() == f(a).signum() {
                    a = m;
                } else {
                    b = m;
                }
            }
            roots.push(0.5 * (a + b));
        }
        near = near.min(fb.abs());
        prev = fb;
    }
    // near-tangency: minimum of |f| at points where it does not cross;
    // conservatively we report min |f| over grid excluding within 1e-3
    // rad of a root.
    let mut near2 = f64::INFINITY;
    for k in 0..n {
        let t = at(k);
        if roots.iter().all(|r| (r - t).abs() > 2e-3) {
            near2 = near2.min(f(t).abs());
        }
    }
    let _ = near;
    (roots, near2)
}

#[derive(Default, Debug)]
struct Tally {
    certified_ok: usize,
    miss_ok: usize,
    on_ok: usize,
    uncertain: usize,
    escalated: usize,
    near_tangent_skips: usize,
    skips: Vec<String>,
    fails: Vec<String>,
}

fn judge(label: &str, e: &geom::Curve3<f64>, s: S, t0: f64, t1: f64, tally: &mut Tally) {
    let mid = 0.5 * (t0 + t1);
    let got = ellipse_roots(e, t0, t1, &surf(s), band());
    let (truth, near) = oracle(e, s, mid);
    let eps = Tol::witness().eps();
    let tangentish = near < 1e3 * eps;
    match got {
        Err(_) => tally.escalated += 1,
        Ok(CircleRoots::Uncertain) => tally.uncertain += 1,
        Ok(CircleRoots::CountDisagrees) => tally.fails.push(format!("{label}: CountDisagrees")),
        Ok(CircleRoots::Miss) => {
            if truth.is_empty() {
                tally.miss_ok += 1;
            } else {
                // A miss with crossings: wrong unless every crossing pair is
                // within the band (a sliver of depth ≤ eps).
                let depth = (0..4000)
                    .map(|k| dist(s, pt(e, mid - PI + TAU * f64::from(k) / 4000.0)))
                    .fold(0.0_f64, |m, d| m.max(-d).max(0.0).max(m));
                let height = (0..4000)
                    .map(|k| dist(s, pt(e, mid - PI + TAU * f64::from(k) / 4000.0)))
                    .fold(0.0_f64, f64::max);
                let excursion = depth.min(height);
                if excursion > 10.0 * eps {
                    tally.fails.push(format!(
                        "{label}: MISS but oracle has {} roots, excursion {excursion:e}",
                        truth.len()
                    ));
                } else {
                    tally.near_tangent_skips += 1;
                    tally.skips.push(format!("{label}: MISS, {} roots, excursion {excursion:e}", truth.len()));
                }
            }
        }
        Ok(CircleRoots::OnSurface) => {
            let worst = (0..4000)
                .map(|k| dist(s, pt(e, TAU * f64::from(k) / 4000.0)).abs())
                .fold(0.0, f64::max);
            if worst > 10.0 * eps {
                tally
                    .fails
                    .push(format!("{label}: OnSurface but max |d| = {worst:e}"));
            } else {
                tally.on_ok += 1;
            }
        }
        Ok(CircleRoots::Certified { count, thetas }) => {
            let mut g = thetas[..count].to_vec();
            g.sort_by(f64::total_cmp);
            if g.len() != truth.len() {
                if tangentish {
                    tally.near_tangent_skips += 1;
                    let on: Vec<f64> = g.iter().map(|&t| dist(s, pt(e, t))).collect();
                    let between = if g.len() == 2 { dist(s, pt(e, 0.5 * (g[0] + g[1]))) } else { f64::NAN };
                    let away = if g.len() == 2 { dist(s, pt(e, 0.5 * (g[0] + g[1]) + PI)) } else { f64::NAN };
                    tally.skips.push(format!("{label}: certified {} vs oracle {}, near {near:e}; d(roots) {on:?}, d(mid) {between:e}, d(away) {away:e}, gap {:e}", g.len(), truth.len(), if g.len()==2 {g[1]-g[0]} else {0.0}));
                } else {
                    tally.fails.push(format!(
                        "{label}: certified {} roots, oracle {} ({truth:?} vs {g:?}), near {near:e}",
                        g.len(),
                        truth.len()
                    ));
                }
                return;
            }
            // Placement: each certified root within arc-length 10 eps of
            // the oracle's (in metres via the speed bound a).
            let mut bad = None;
            for (x, y) in g.iter().zip(&truth) {
                let off = dist(s, pt(e, *x)).abs();
                if (x - y).abs() * 2.0 > 1e-6 && off > 10.0 * eps {
                    bad = Some((*x, *y, off));
                }
                if (x - mid).abs() > PI + 1e-12 {
                    bad = Some((*x, mid, -1.0));
                }
            }
            match bad {
                None => tally.certified_ok += 1,
                Some((x, y, off)) => tally
                    .fails
                    .push(format!("{label}: root {x} vs oracle {y}, off-surface {off:e}")),
            }
        }
    }
}

#[test]
fn probe_r2_ellipse_roots_fuzz() {
    let mut rng = Lcg(0xC0FFEE);
    let mut tally = Tally::default();
    for k in 0..3000 {
        let a = rng.r(0.05, 3.0);
        let b = a * rng.r(0.03, 0.999);
        let c = Point3::new(rng.r(-1.0, 1.0), rng.r(-1.0, 1.0), rng.r(-1.0, 1.0));
        let n = rng.unit();
        let e = ell(c, n, rng.unit(), a, b);
        let t0 = rng.r(-7.0, 7.0);
        let t1 = t0 + rng.r(0.1, TAU);
        let near_pt = pt(&e, rng.r(0.0, TAU));
        let s = match k % 3 {
            0 => {
                let sc = near_pt + rng.unit() * rng.r(0.0, a);
                // radius near the distance to some ellipse point: crossings
                let r = (pt(&e, rng.r(0.0, TAU)) - sc).norm() * rng.r(0.9, 1.1);
                S::Sphere(sc, r.max(1e-3))
            }
            1 => {
                let ax = rng.unit();
                let o = near_pt + rng.unit() * rng.r(0.0, a);
                let w = pt(&e, rng.r(0.0, TAU)) - o;
                let r = (w - ax * w.dot(ax)).norm() * rng.r(0.9, 1.1);
                S::Wall(o, ax, r.max(1e-3))
            }
            _ => {
                // Near-section: wall whose tilted section is (almost) e.
                // Wall axis tilted off n by small angle, radius ~ b.
                let exact = k % 2 == 0;
                let tilt = if exact { 0.0 } else { rng.r(0.0, 1e-6) };
                let ax = (n + rng.unit() * tilt).normalize();
                let shift = if exact { Vec3::new(0.0, 0.0, 0.0) } else { rng.unit() * rng.r(0.0, 1e-5) * b };
                // Exact section would be the wall about axis n' with radius
                // b when the ellipse is the section of a wall tilted by
                // acos(b/a) — here use a wall with axis making angle
                // acos(b/a) with n, in the (n, u) plane.
                let geom::Curve3::Ellipse { u_ref, .. } = e else {
                    unreachable!()
                };
                let phi = (b / a).acos();
                let wall_axis = (n * phi.cos() + u_ref * phi.sin()).normalize();
                let wall_axis = (wall_axis + (ax - n)).normalize();
                let rr = if exact { b * [1.0, 1.0 + 1e-3, 1.0 - 1e-3, 1.0 + 1e-11][k % 4 / 2 + (k / 6) % 2 * 2] } else { b * rng.r(1.0 - 1e-7, 1.0 + 1e-7) };
                S::Wall(c + shift, wall_axis, rr)
            }
        };
        judge(&format!("case {k}"), &e, s, t0, t1, &mut tally);
    }
    println!("{tally:#?}");
    assert!(tally.fails.is_empty(), "{} fails", tally.fails.len());
}

/// Enclosure check: every dense sample of my residual inside the range.
fn enclosure_fails(e: &geom::Curve3<f64>, s: S, t0: f64, t1: f64) -> Option<String> {
    let conic = geom_brep::Conic::of(e).unwrap();
    let sf = surf(s);
    let mut out = None;
    if let Some((lo, hi)) = geom_brep::conic_arc_residual_range(&sf, &conic, t0, t1) {
        let n = 20_000;
        for k in 0..=n {
            let t = t0 + (t1 - t0) * f64::from(k) / f64::from(n);
            let r = resid(s, pt(e, t));
            let slack = 1e-12 * (1.0 + r.abs());
            if r < lo - slack || r > hi + slack {
                out = Some(format!("arc [{t0},{t1}] t {t}: {r:e} outside [{lo:e}, {hi:e}]"));
                break;
            }
        }
    }
    if let Some((lo, hi)) = geom_brep::conic_residual_extremes(&sf, &conic) {
        for k in 0..=20_000 {
            let t = TAU * f64::from(k) / 20_000.0;
            let r = resid(s, pt(e, t));
            let slack = 1e-12 * (1.0 + r.abs());
            if r < lo - slack || r > hi + slack {
                out = Some(format!("carrier t {t}: {r:e} outside [{lo:e}, {hi:e}]"));
                break;
            }
        }
    }
    out
}

#[test]
fn probe_r2_enclosures_fuzz() {
    let mut rng = Lcg(0xBEEF);
    let mut fails = Vec::new();
    for k in 0..1500 {
        let a = rng.r(0.05, 3.0);
        let b = a * rng.r(0.01, 0.999);
        let c = Point3::new(rng.r(-1.0, 1.0), rng.r(-1.0, 1.0), rng.r(-1.0, 1.0));
        let e = ell(c, rng.unit(), rng.unit(), a, b);
        let near_pt = pt(&e, rng.r(0.0, TAU));
        let s = match k % 4 {
            0 => S::Sphere(near_pt + rng.unit() * rng.r(0.0, a), rng.r(0.05, 3.0)),
            1 => S::Wall(near_pt + rng.unit() * rng.r(0.0, a), rng.unit(), rng.r(0.05, 3.0)),
            2 => {
                let big = rng.r(0.3, 3.0);
                S::Torus(near_pt + rng.unit() * rng.r(0.0, a), rng.unit(), big, big * rng.r(0.05, 0.95))
            }
            _ => S::Plane(near_pt + rng.unit() * rng.r(0.0, a), rng.unit()),
        };
        // Spans: whole turn, short arcs near the major/minor vertices.
        for (t0, t1) in [
            (0.0, TAU),
            (-0.05, 0.05),
            (PI / 2.0 - 0.05, PI / 2.0 + 0.05),
            (rng.r(-3.0, 3.0), rng.r(3.0, 6.0)),
        ] {
            if let Some(f) = enclosure_fails(&e, s, t0, t1) {
                fails.push(format!("case {k}: {f}"));
            }
        }
    }
    for f in fails.iter().take(10) {
        println!("{f}");
    }
    assert!(fails.is_empty(), "{} enclosure failures", fails.len());
}

/// The frame premise: ellipses the mint certifies (tier3_tests:
/// `the_elliptic_lever_is_the_larger_semi_axis_magnitude`) but whose
/// stored frame violates `major ≥ minor > 0`.
#[test]
fn probe_r2_swapped_frame() {
    let mut rng = Lcg(0xABCD);
    let mut fails = 0usize;
    let mut tries = 0usize;
    let mut first = None;
    for _ in 0..3000 {
        let big = rng.r(0.5, 2.0);
        let small = big * rng.r(0.05, 0.6);
        // Stored with minor > major (the same geometric ellipse as
        // (small, big) on a rotated frame).
        let c = Point3::new(rng.r(-1.0, 1.0), rng.r(-1.0, 1.0), rng.r(-1.0, 1.0));
        let e = ell(c, rng.unit(), rng.unit(), small, big);
        let torus_big = rng.r(0.3, 2.0);
        let s = S::Torus(
            pt(&e, rng.r(0.0, TAU)) + rng.unit() * rng.r(0.0, 0.5),
            rng.unit(),
            torus_big,
            torus_big * rng.r(0.1, 0.9),
        );
        for (t0, t1) in [(0.0, TAU), (0.3, 1.3)] {
            tries += 1;
            if let Some(f) = enclosure_fails(&e, s, t0, t1) {
                fails += 1;
                first.get_or_insert(f);
            }
        }
    }
    println!("swapped frame: {fails} enclosure failures in {tries}; first {first:?}");
}

/// An ellipse lying exactly on a tilted wall (its own section), random
/// frames: what does the door say?
#[test]
fn probe_r2_exact_section() {
    let mut rng = Lcg(0x5EC7);
    let mut seen = std::collections::BTreeMap::<String, usize>::new();
    let mut shown = 0;
    for _ in 0..400 {
        let a = rng.r(0.05, 3.0);
        let b = a * rng.r(0.03, 0.999);
        let c = Point3::new(rng.r(-1.0, 1.0), rng.r(-1.0, 1.0), rng.r(-1.0, 1.0));
        let n = rng.unit();
        let e = ell(c, n, rng.unit(), a, b);
        let geom::Curve3::Ellipse { u_ref, .. } = e else { unreachable!() };
        let phi = (b / a).acos();
        let wall_axis = (n * phi.cos() + u_ref * phi.sin()).normalize();
        let s = S::Wall(c, wall_axis, b);
        let worst = (0..2000)
            .map(|k| dist(s, pt(&e, TAU * f64::from(k) / 2000.0)).abs())
            .fold(0.0, f64::max);
        let got = ellipse_roots(&e, 0.0, 1.0, &surf(s), band());
        let key = match &got {
            Ok(CircleRoots::Certified { count, .. }) => format!("Certified{count}"),
            Ok(x) => format!("{x:?}").chars().take(12).collect(),
            Err(e) => format!("Err {e:?}").chars().take(60).collect(),
        };
        if !key.starts_with("OnSurface") && shown < 6 {
            shown += 1;
            let conic = geom_brep::Conic::of(&e).unwrap();
            let geom::Surface::Cylinder { origin, axis, radius, .. } = surf(s) else { unreachable!() };
            let h = geom_brep::conic_cylinder_harmonics(&conic, origin, axis, radius);
            println!("a {a:.4} b {b:.4} worst |d| {worst:e}: {key}; h {h:?}");
        }
        *seen.entry(key).or_default() += 1;
    }
    println!("{seen:?}");
}

/// Near-tangent poses: the sphere (or wall) radius set to the carrier's
/// extreme distance plus a small `δ`, so a close root pair is born or not.
#[test]
fn probe_r2_near_tangent() {
    let mut rng = Lcg(0x7A9);
    let mut tally = Tally::default();
    for k in 0..1200 {
        let a = rng.r(0.1, 2.0);
        let b = a * rng.r(0.05, 0.95);
        let c = Point3::new(rng.r(-1.0, 1.0), rng.r(-1.0, 1.0), rng.r(-1.0, 1.0));
        let n = rng.unit();
        let e = ell(c, n, rng.unit(), a, b);
        let o = pt(&e, rng.r(0.0, TAU)) + rng.unit() * rng.r(0.1, 1.5) * a;
        let ax = rng.unit();
        let wall = k % 2 == 1;
        // extreme (min or max) of the distance to o (or to the axis line)
        let raw = |t: f64| {
            let w = pt(&e, t) - o;
            if wall { (w - ax * w.dot(ax)).norm() } else { w.norm() }
        };
        let want_max = k % 4 >= 2;
        let mut best = 0.0;
        let mut bv = if want_max { f64::NEG_INFINITY } else { f64::INFINITY };
        for j in 0..20_000 {
            let t = TAU * f64::from(j) / 20_000.0;
            let v = raw(t);
            if (want_max && v > bv) || (!want_max && v < bv) {
                bv = v;
                best = t;
            }
        }
        // golden refine
        let (mut lo, mut hi) = (best - TAU / 20_000.0, best + TAU / 20_000.0);
        for _ in 0..200 {
            let m1 = lo + (hi - lo) * 0.382;
            let m2 = lo + (hi - lo) * 0.618;
            let better = if want_max { raw(m1) > raw(m2) } else { raw(m1) < raw(m2) };
            if better { hi = m2 } else { lo = m1 }
        }
        let ext = raw(0.5 * (lo + hi));
        if ext < 1e-3 {
            continue;
        }
        let delta = [1e-3, 1e-5, 1e-7, 1e-8, 1e-9, 1e-10, 1e-11][k % 7] * if k % 3 == 0 { -1.0 } else { 1.0 };
        let r = ext + delta;
        let s = if wall { S::Wall(o, ax, r) } else { S::Sphere(o, r) };
        let t0 = rng.r(-4.0, 4.0);
        judge(&format!("nt {k} wall {wall} max {want_max} δ {delta:e}"), &e, s, t0, t0 + 1.0, &mut tally);
    }
    println!("{tally:#?}");
    assert!(tally.fails.is_empty());
}
