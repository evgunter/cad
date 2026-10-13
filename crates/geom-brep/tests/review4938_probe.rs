//! Reviewer probe for PR 4938: the closed-form closure decision over a
//! corpus of legal and defective cylinder/torus faces, at both scalars and
//! three ε. Prints one line per case; diffed head vs merge base.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::props::{LoopEdge, curved_face_loops};
use geom_core::{Band, Bounds, Interval, Point3, Real, Tol, Vec3};
use std::f64::consts::TAU;

#[derive(Clone, Copy)]
struct Frame { o: Point3<f64>, a: Vec3<f64>, e1: Vec3<f64>, e2: Vec3<f64> }

fn frame(tilted: bool, off: bool) -> Frame {
    let (a, e1) = if tilted {
        let a = Vec3::new(0.3, -0.5, 0.81);
        let a = a * (1.0 / a.norm());
        let t = Vec3::new(1.0, 0.2, 0.0);
        let e1 = t - a * t.dot(a);
        (a, e1 * (1.0 / e1.norm()))
    } else { (Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0)) };
    let o = if off { Point3::new(12.3, -7.1, 4.4) } else { Point3::new(0.0, 0.0, 0.0) };
    Frame { o, a, e1, e2: a.cross(e1) }
}

fn edge(c: Curve3<f64>, a: f64, b: f64) -> LoopEdge<f64> {
    let (t0, t1, f) = if a < b { (a, b, true) } else { (b, a, false) };
    LoopEdge::hand_built(c, t0, t1, f, 0, 0)
}

const R: f64 = 1.0;
fn cyl(f: Frame) -> Surface<f64> { Surface::Cylinder { origin: f.o, axis: f.a, radius: R, u_ref: f.e1 } }
fn crim(f: Frame, z: f64, u0: f64, u1: f64) -> LoopEdge<f64> {
    edge(Curve3::Circle { center: f.o + f.a * z, axis: f.a, radius: R, u_ref: f.e1 }, u0, u1)
}
fn ruling(f: Frame, u: f64, z0: f64, z1: f64) -> LoopEdge<f64> {
    edge(Curve3::Line { origin: f.o + (f.e1 * u.cos() + f.e2 * u.sin()) * R, dir: f.a }, z0, z1)
}
const RR: f64 = 0.02; const R0: f64 = 0.005;
fn tor(f: Frame) -> Surface<f64> { Surface::Torus { center: f.o, axis: f.a, major_radius: RR, minor_radius: R0, u_ref: f.e1 } }
fn trim(f: Frame, v: f64, u0: f64, u1: f64) -> LoopEdge<f64> {
    edge(Curve3::Circle { center: f.o + f.a * (R0 * v.sin()), axis: f.a, radius: RR + R0 * v.cos(), u_ref: f.e1 }, u0, u1)
}
fn tmer(f: Frame, u: f64, v0: f64, v1: f64) -> LoopEdge<f64> {
    let rad = f.e1 * u.cos() + f.e2 * u.sin();
    edge(Curve3::Circle { center: f.o + rad * RR, axis: f.a.cross(rad) * -1.0, radius: R0, u_ref: rad }, v0, v1)
}
/// reverse a loop
fn rev(l: Vec<LoopEdge<f64>>) -> Vec<LoopEdge<f64>> {
    l.into_iter().rev().map(|mut e| { e.forward = !e.forward; e }).collect()
}
/// a rectangle on the cylinder [ua,ub]x[z0,z1], junction 1 (end of bottom rim -> ruling) opened by gz in level, gu (m) in azimuth
fn crect(f: Frame, ua: f64, ub: f64, z0: f64, z1: f64, gz: f64, gu: f64) -> Vec<LoopEdge<f64>> {
    vec![crim(f, z0, ua, ub - gu / R), ruling(f, ub, z0 + gz, z1), crim(f, z1, ub, ua), ruling(f, ua, z1, z0)]
}
fn trect(f: Frame, ua: f64, ub: f64, v0: f64, v1: f64, gv: f64, gu: f64) -> Vec<LoopEdge<f64>> {
    let rho = RR + R0 * v0.cos();
    vec![trim(f, v0, ua, ub - gu / rho), tmer(f, ub, v0 + gv / R0, v1), trim(f, v1, ub, ua), tmer(f, ua, v1, v0)]
}

fn run<T: geom_core::Decide + Bounds>(s: &Surface<f64>, loops: &[Vec<LoopEdge<f64>>], band: Band, lift: impl Fn(f64) -> T + Copy) -> String {
    let s2 = s.map_scalar(lift);
    let ls: Vec<Vec<LoopEdge<T>>> = loops.iter().map(|l| l.iter().map(|e| LoopEdge::hand_built(e.carrier.map_scalar(lift), lift(e.t0), lift(e.t1), e.forward, 0, 0)).collect()).collect();
    let refs: Vec<&[LoopEdge<T>]> = ls.iter().map(Vec::as_slice).collect();
    match curved_face_loops(&s2, &refs, true, band) {
        Ok(c) => format!("Ok area={:.9e}", 0.5 * (c.area.lo() + c.area.hi())),
        Err(e) => format!("Err {e:?}"),
    }
}


#[test]
fn review4938_closed_form_corpus() {
    let mut out = Vec::new();
    for &eps in &[1e-6, 1e-9, 1e-12] {
        let band = Band::linear_at(Tol::witness(), eps).unwrap();
        for &(tilted, off) in &[(false, false), (true, true)] {
            let f = frame(tilted, off);
            let mut cases: Vec<(String, Surface<f64>, Vec<Vec<LoopEdge<f64>>>)> = Vec::new();
            let c = cyl(f); let t = tor(f);
            let (c, t) = (&c, &t);
            // gaps as multiples of eps
            for &g in &[0.0, 1.0, 2.0, 2.9, 3.5, 5.0, 20.0, 1e3] {
                for &(ua, ub, tag) in &[(TAU - 1.0, TAU, "seam"), (TAU + 0.5, TAU + 1.5, "periodoff"), (-1.0, 0.0, "neg"), (2.0, 3.0, "mid")] {
                    cases.push((format!("cyl {tag} gz={g}"), c.clone(), vec![crect(f, ua, ub, 0.0, 1.0, g * eps, 0.0)]));
                    cases.push((format!("cyl {tag} gu={g}"), c.clone(), vec![crect(f, ua, ub, 0.0, 1.0, 0.0, g * eps)]));
                    cases.push((format!("cyl {tag} rev gz={g}"), c.clone(), vec![rev(crect(f, ua, ub, 0.0, 1.0, g * eps, 0.0))]));
                    cases.push((format!("tor {tag} gv={g}"), t.clone(), vec![trect(f, ua, ub, 0.2, 0.7, g * eps, 0.0)]));
                    cases.push((format!("tor {tag} gu={g}"), t.clone(), vec![trect(f, ua, ub, 0.2, 0.7, 0.0, g * eps)]));
                    cases.push((format!("tor {tag} rev gv={g}"), t.clone(), vec![rev(trect(f, ua, ub, 0.2, 0.7, g * eps, 0.0))]));
                    cases.push((format!("tor {tag} vseam gv={g}"), t.clone(), vec![trect(f, ua, ub, TAU - 0.3, TAU + 0.4, g * eps, 0.0)]));
                }
                // band: lower rim forward, upper backward, split pieces, upper junction gap
                let lower = vec![crim(f, 0.0, 0.3, 2.0), crim(f, 0.0, 2.0, 0.3 + TAU)];
                let upper = vec![crim(f, 1.0, 5.0, 3.0 + g * eps), crim(f, 1.0, 3.0, 5.0 - TAU)];
                cases.push((format!("cyl band gu={g}"), c.clone(), vec![lower.clone(), upper.clone()]));
                cases.push((format!("cyl band rev gu={g}"), c.clone(), vec![rev(lower.clone()), rev(upper.clone())]));
                // band with a hole
                let hole = rev(crect(f, 1.0, 1.5, 0.3, 0.6, g * eps, 0.0));
                cases.push((format!("cyl band+hole gz={g}"), c.clone(), vec![lower.clone(), upper.clone(), hole.clone()]));
                cases.push((format!("cyl band hole-first gz={g}"), c.clone(), vec![hole.clone(), lower.clone(), upper.clone()]));
                // rect with hole
                cases.push((format!("cyl rect+hole gz={g}"), c.clone(), vec![crect(f, 0.5, 2.5, 0.0, 1.0, 0.0, 0.0), hole.clone()]));
                let thole = rev(trect(f, 1.0, 1.5, 0.3, 0.6, g * eps, 0.0));
                cases.push((format!("tor rect+hole gv={g}"), t.clone(), vec![trect(f, 0.5, 2.5, 0.0, 1.0, 0.0, 0.0), thole]));
            }
            // defects
            let full = |z: f64, fwd: bool, k: usize| -> Vec<LoopEdge<f64>> { (0..k).map(|_| if fwd { crim(f, z, 0.0, TAU) } else { crim(f, z, TAU, 0.0) }).collect() };
            cases.push(("cyl DEF lone rim".into(), c.clone(), vec![full(0.0, true, 1)]));
            cases.push(("cyl DEF same-way pair".into(), c.clone(), vec![full(0.0, true, 1), full(1.0, true, 1)]));
            cases.push(("cyl DEF +-2 pair".into(), c.clone(), vec![full(0.0, true, 2), full(1.0, false, 2)]));
            cases.push(("cyl DEF twice lone".into(), c.clone(), vec![full(0.0, true, 2)]));
            cases.push(("cyl DEF +2,-1,-1".into(), c.clone(), vec![full(0.0, true, 2), full(1.0, false, 1), full(2.0, false, 1)]));
            cases.push(("cyl legal band".into(), c.clone(), vec![full(0.0, true, 1), full(1.0, false, 1)]));
            cases.push(("cyl DEF three-rim (+1 -1 +1 -1?)".into(), c.clone(), vec![full(0.0, true, 1), full(1.0, false, 1), full(2.0, true, 1), full(3.0, false, 1)]));
            cases.push(("cyl DEF ++-- four".into(), c.clone(), vec![full(0.0, true, 1), full(1.0, true, 1), full(2.0, false, 1), full(3.0, false, 1)]));
            cases.push(("cyl DEF +--+ four".into(), c.clone(), vec![full(0.0, true, 1), full(1.0, false, 1), full(2.0, false, 1), full(3.0, true, 1)]));
            cases.push(("cyl DEF band + same-way hole ring".into(), c.clone(), vec![full(0.0, true, 1), full(1.0, false, 1), crect(f, 1.0, 1.5, 0.3, 0.6, 0.0, 0.0)]));
            cases.push(("cyl DEF open 0.1".into(), c.clone(), vec![crect(f, 0.0, 1.0, 0.0, 1.0, 0.1, 0.0)]));
            let tfull = |v: f64, fwd: bool, k: usize| -> Vec<LoopEdge<f64>> { (0..k).map(|_| if fwd { trim(f, v, 0.0, TAU) } else { trim(f, v, TAU, 0.0) }).collect() };
            cases.push(("tor DEF lone rim".into(), t.clone(), vec![tfull(0.2, true, 1)]));
            cases.push(("tor DEF same-way pair".into(), t.clone(), vec![tfull(0.2, true, 1), tfull(0.7, true, 1)]));
            cases.push(("tor DEF +-2 pair".into(), t.clone(), vec![tfull(0.2, true, 2), tfull(0.7, false, 2)]));
            cases.push(("tor band (legal)".into(), t.clone(), vec![tfull(0.2, true, 1), tfull(0.7, false, 1)]));
            let tmfull = |u: f64, fwd: bool, k: usize| -> Vec<LoopEdge<f64>> { (0..k).map(|_| if fwd { tmer(f, u, 0.0, TAU) } else { tmer(f, u, TAU, 0.0) }).collect() };
            cases.push(("tor DEF lone meridian".into(), t.clone(), vec![tmfull(0.2, true, 1)]));
            cases.push(("tor DEF meridian +-2".into(), t.clone(), vec![tmfull(0.2, true, 2), tmfull(0.9, false, 2)]));
            cases.push(("tor meridian band (legal)".into(), t.clone(), vec![tmfull(0.2, true, 1), tmfull(0.9, false, 1)]));
            cases.push(("tor DEF open".into(), t.clone(), vec![trect(f, 0.0, 1.0, 0.2, 0.7, 0.1 * R0, 0.0)]));
            for (name, s, loops) in &cases {
                let a = run::<f64>(s, loops, band, |x| x);
                let b = run::<Interval>(s, loops, band, Interval::from_f64);
                out.push(format!("eps={eps:e} tilt={tilted} | {name} | f64: {a} | iv: {b}"));
            }
        }
    }
    // Set PROBE_OUT to dump the table; diffed against the merge base 9326fa5f.
    if let Ok(path) = std::env::var("PROBE_OUT") {
        std::fs::write(path, out.join("\n")).unwrap();
    }
}

/// **Red on 55b187ac.** Four whole rims, wound `+ + − −` up the axis,
/// pass `|k| ≤ 1` and `Σk = 0` and measure `8π` (the middle band counted
/// twice); wound `+ − − +` they measure `0`. Neither is any face's
/// boundary: adjacent rims bounding one region wind opposite ways.
#[test]
fn review4938_four_rims_out_of_order_refuse() {
    let f = frame(false, false);
    let band = Band::linear_at(Tol::witness(), 1e-9).unwrap();
    let full = |z: f64, fwd: bool| vec![if fwd { crim(f, z, 0.0, TAU) } else { crim(f, z, TAU, 0.0) }];
    for (name, signs) in [("++--", [true, true, false, false]), ("+--+", [true, false, false, true])] {
        let loops: Vec<Vec<LoopEdge<f64>>> = signs.iter().enumerate().map(|(i, &s)| full(i as f64, s)).collect();
        let got = run::<f64>(&cyl(f), &loops, band, |x| x);
        assert!(got.starts_with("Err"), "{name}: {got}");
    }
}
