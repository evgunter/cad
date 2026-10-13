//! Reviewer probes, PR 4938 (periodic-chart closure decision).
//! A differential: prints every case's verdict so base and head diff.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::props::{curved_face_loops, LoopEdge, PropsError};
use geom_core::{Band, Bounds, Decide, Interval, Point3, Real, Tol, Vec3};
use std::f64::consts::{PI, TAU};

#[derive(Clone, Copy)]
struct Frame {
    o: [f64; 3],
    z: [f64; 3],
    x: [f64; 3],
}
fn add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] { [a[0] + b[0], a[1] + b[1], a[2] + b[2]] }
fn mul(a: [f64; 3], s: f64) -> [f64; 3] { [a[0] * s, a[1] * s, a[2] * s] }
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
fn norm(a: [f64; 3]) -> [f64; 3] { let n = (a[0]*a[0]+a[1]*a[1]+a[2]*a[2]).sqrt(); mul(a, 1.0/n) }
impl Frame {
    fn y(&self) -> [f64; 3] { cross(self.z, self.x) }
    fn radial(&self, u: f64) -> [f64; 3] { add(mul(self.x, u.cos()), mul(self.y(), u.sin())) }
}
fn p<T: Real>(a: [f64; 3]) -> Point3<T> { Point3::new(a[0], a[1], a[2]).map(T::from_f64) }
fn v<T: Real>(a: [f64; 3]) -> Vec3<T> { Vec3::new(a[0], a[1], a[2]).map(T::from_f64) }
fn edge<T: Real>(c: Curve3<T>, a: f64, b: f64) -> LoopEdge<T> {
    let (t0, t1, fwd) = if a < b { (a, b, true) } else { (b, a, false) };
    LoopEdge::hand_built(c, T::from_f64(t0), T::from_f64(t1), fwd, 0, 0)
}

fn frames() -> Vec<(&'static str, Frame)> {
    let tz = norm([0.3, -0.5, 0.81]);
    let tx = norm(cross([0.0, 0.0, 1.0], tz));
    vec![
        ("std", Frame { o: [0.0; 3], z: [0.0, 0.0, 1.0], x: [1.0, 0.0, 0.0] }),
        ("tilt_off", Frame { o: [3.7, -12.25, 0.4], z: tz, x: tx }),
    ]
}

// ---- cylinder (radius RC) ----
const RC: f64 = 0.7;
fn cyl<T: Real>(f: Frame) -> Surface<T> {
    Surface::Cylinder { origin: p(f.o), axis: v(f.z), radius: T::from_f64(RC), u_ref: v(f.x) }
}
fn crim<T: Real>(f: Frame, z: f64, u0: f64, u1: f64) -> LoopEdge<T> {
    edge(Curve3::Circle { center: p(add(f.o, mul(f.z, z))), axis: v(f.z), radius: T::from_f64(RC), u_ref: v(f.x) }, u0, u1)
}
fn ruling<T: Real>(f: Frame, u: f64, z0: f64, z1: f64) -> LoopEdge<T> {
    edge(Curve3::Line { origin: p(add(f.o, mul(f.radial(u), RC))), dir: v(f.z) }, z0, z1)
}
/// rectangle u in [u0,u0+w], z in [z0,z1]; ccw in chart; gap: the second edge starts gap higher.
fn crect<T: Real>(f: Frame, u0: f64, w: f64, z0: f64, z1: f64, gap: f64) -> Vec<LoopEdge<T>> {
    vec![crim(f, z0, u0, u0 + w), ruling(f, u0 + w, z0 + gap, z1), crim(f, z1, u0 + w, u0), ruling(f, u0, z1, z0)]
}

// ---- torus ----
const RR: f64 = 0.020;
const R0: f64 = 0.005;
fn tor<T: Real>(f: Frame) -> Surface<T> {
    Surface::Torus { center: p(f.o), axis: v(f.z), major_radius: T::from_f64(RR), minor_radius: T::from_f64(R0), u_ref: v(f.x) }
}
fn trim<T: Real>(f: Frame, vv: f64, u0: f64, u1: f64) -> LoopEdge<T> {
    edge(Curve3::Circle { center: p(add(f.o, mul(f.z, R0 * vv.sin()))), axis: v(f.z), radius: T::from_f64(RR + R0 * vv.cos()), u_ref: v(f.x) }, u0, u1)
}
fn tmer<T: Real>(f: Frame, u: f64, v0: f64, v1: f64) -> LoopEdge<T> {
    let rad = f.radial(u);
    edge(Curve3::Circle { center: p(add(f.o, mul(rad, RR))), axis: v(cross(rad, f.z)), radius: T::from_f64(R0), u_ref: v(rad) }, v0, v1)
}
fn trect<T: Real>(f: Frame, u0: f64, w: f64, v0: f64, v1: f64, gap: f64) -> Vec<LoopEdge<T>> {
    vec![trim(f, v0, u0, u0 + w), tmer(f, u0 + w, v0 + gap / R0, v1), trim(f, v1, u0 + w, u0), tmer(f, u0, v1, v0)]
}

fn rev<T: Real>(l: &[LoopEdge<T>]) -> Vec<LoopEdge<T>> {
    l.iter().rev().map(|e| { let mut e = e.clone(); e.forward = !e.forward; e }).collect()
}

fn show<T: Decide + Bounds>(r: Result<geom_brep::props::FaceContribution<T>, PropsError>, want: Option<f64>) -> String {
    match r {
        Ok(c) => {
            let a = 0.5 * (c.area.lo() + c.area.hi());
            match want {
                Some(w) if (a - w).abs() <= 1e-6 * w => "OK(area=want)".to_string(),
                Some(w) => format!("OK(area={a:.6e} WANT {w:.6e} ratio {:.4})", a / w),
                None => format!("OK(area={a:.6e})"),
            }
        }
        Err(PropsError::NotIsoRectangle { what }) => format!("NotIsoRectangle({})", &what[..what.len().min(40)]),
        Err(e) => format!("{e:?}").chars().take(90).collect(),
    }
}

fn sweep<T: Decide + Bounds>(tag: &str, eps: f64, out: &mut Vec<String>) {
    let band = Band::linear_at(Tol::witness(), eps).unwrap();
    let mut row = |name: String, r: String| out.push(format!("{tag} eps={eps:e} {name} => {r}"));
    for (fname, f) in frames() {
        for &u0 in &[-TAU, -1.0, 0.0, TAU - 1.0, TAU - 0.5, PI - 0.25, 2.0 * TAU - 0.5, 5.0 * TAU + 0.1] {
            for &w in &[1.0, 3.5, 6.0] {
                for &g in &[0.0, 2.0, 3.5, 6.0, 100.0] {
                    let gap = g * eps;
                    for &flip in &[false, true] {
                        let mut l = crect::<T>(f, u0, w, 0.0, 1.0, gap);
                        if flip { l = rev(&l); }
                        row(format!("cyl {fname} u0={u0:.3} w={w} gap={g}e flip={flip}"),
                            show(curved_face_loops(&cyl::<T>(f), &[&l], !flip, band), Some(RC * w)));
                        let mut l = trect::<T>(f, u0, w, 0.2, 0.7, gap);
                        if flip { l = rev(&l); }
                        let want = R0 * w * (RR * 0.5 + R0 * (0.7f64.sin() - 0.2f64.sin()));
                        row(format!("tor {fname} u0={u0:.3} w={w} gap={g}e flip={flip}"),
                            show(curved_face_loops(&tor::<T>(f), &[&l], !flip, band), Some(want)));
                        // torus rectangle across v seam
                        let mut l = trect::<T>(f, u0, w, -0.4 + TAU * 0.0, 0.5, gap);
                        if flip { l = rev(&l); }
                        let want = R0 * w * (RR * 0.9 + R0 * (0.5f64.sin() - (-0.4f64).sin()));
                        row(format!("torvseam {fname} u0={u0:.3} w={w} gap={g}e flip={flip}"),
                            show(curved_face_loops(&tor::<T>(f), &[&l], !flip, band), Some(want)));
                    }
                }
            }
            // full-turn seam faces (one loop, k=0)
            let l = crect::<T>(f, u0, TAU, 0.0, 1.0, 0.0);
            row(format!("cyl seamband {fname} u0={u0:.3}"), show(curved_face_loops(&cyl::<T>(f), &[&l], true, band), Some(RC * TAU)));
            let l = trect::<T>(f, u0, TAU, 0.2, 0.7, 0.0);
            let want = R0 * TAU * (RR * 0.5 + R0 * (0.7f64.sin() - 0.2f64.sin()));
            row(format!("tor seamband {fname} u0={u0:.3}"), show(curved_face_loops(&tor::<T>(f), &[&l], true, band), Some(want)));
            // two-loop band (cylinder), rims stored at u0
            let lo = vec![crim::<T>(f, 0.0, u0, u0 + TAU)];
            let hi = vec![crim::<T>(f, 1.0, u0 + TAU, u0)];
            row(format!("cyl band2 {fname} u0={u0:.3}"), show(curved_face_loops(&cyl::<T>(f), &[&lo, &hi], true, band), Some(RC * TAU)));
            // band with hole
            let hole = rev(&crect::<T>(f, u0 + 1.0, 0.5, 0.25, 0.75, 0.0));
            row(format!("cyl band2+hole {fname} u0={u0:.3}"), show(curved_face_loops(&cyl::<T>(f), &[&lo, &hi, &hole], true, band), Some(RC * (TAU - 0.25))));
            row(format!("cyl band2+hole-first {fname} u0={u0:.3}"), show(curved_face_loops(&cyl::<T>(f), &[&hole, &lo, &hi], true, band), Some(RC * (TAU - 0.25))));
            // rect with hole
            let outer = crect::<T>(f, u0, 3.0, 0.0, 1.0, 0.0);
            row(format!("cyl rect+hole {fname} u0={u0:.3}"), show(curved_face_loops(&cyl::<T>(f), &[&outer, &hole], true, band), Some(RC * (3.0 - 0.25))));
            // torus band (legal, inventory)
            let tlo = vec![trim::<T>(f, 0.2, u0, u0 + TAU)];
            let thi = vec![trim::<T>(f, 0.7, u0 + TAU, u0)];
            row(format!("tor band2 {fname} u0={u0:.3}"), show(curved_face_loops(&tor::<T>(f), &[&tlo, &thi], true, band), None));
            let tm1 = vec![tmer::<T>(f, u0, 0.0, TAU)];
            let tm2 = vec![tmer::<T>(f, u0 + 1.0, TAU, 0.0)];
            row(format!("tor vband2 {fname} u0={u0:.3}"), show(curved_face_loops(&tor::<T>(f), &[&tm1, &tm2], true, band), None));
            // torus v seam band: one loop, rims at v=0 and v=tau
            for &w in &[1.0, TAU] {
                let l = vec![trim::<T>(f, 0.0, u0, u0 + w), tmer(f, u0 + w, 0.0, TAU), trim(f, TAU, u0 + w, u0), tmer(f, u0, TAU, 0.0)];
                row(format!("tor vseamband w={w:.3} {fname} u0={u0:.3}"), show(curved_face_loops(&tor::<T>(f), &[&l], true, band), Some(R0 * w * RR * TAU)));
                let l = vec![trim::<T>(f, 1.0, u0, u0 + w), tmer(f, u0 + w, 1.0, 1.0 + TAU), trim(f, 1.0 + TAU, u0 + w, u0), tmer(f, u0, 1.0 + TAU, 1.0)];
                row(format!("tor vseamband1 w={w:.3} {fname} u0={u0:.3}"), show(curved_face_loops(&tor::<T>(f), &[&l], true, band), Some(R0 * w * RR * TAU)));
            }
            // ---- defects ----
            row(format!("cyl lone-rim {fname} u0={u0:.3}"), show(curved_face_loops(&cyl::<T>(f), &[&lo], true, band), None));
            row(format!("tor lone-rim {fname} u0={u0:.3}"), show(curved_face_loops(&tor::<T>(f), &[&tlo], true, band), None));
            row(format!("tor lone-mer {fname} u0={u0:.3}"), show(curved_face_loops(&tor::<T>(f), &[&tm1], true, band), None));
            let lo2 = vec![crim::<T>(f, 0.5, u0, u0 + TAU)];
            row(format!("cyl same-way {fname} u0={u0:.3}"), show(curved_face_loops(&cyl::<T>(f), &[&lo, &lo2], true, band), None));
            let tlo2 = vec![trim::<T>(f, 0.5, u0, u0 + TAU)];
            row(format!("tor same-way {fname} u0={u0:.3}"), show(curved_face_loops(&tor::<T>(f), &[&tlo, &tlo2], true, band), None));
            let twice = vec![crim::<T>(f, 0.0, u0, u0 + TAU), crim::<T>(f, 0.0, u0, u0 + TAU)];
            let twiceb = vec![crim::<T>(f, 1.0, u0 + TAU, u0), crim::<T>(f, 1.0, u0 + TAU, u0)];
            row(format!("cyl pm2 {fname} u0={u0:.3}"), show(curved_face_loops(&cyl::<T>(f), &[&twice, &twiceb], true, band), None));
            row(format!("cyl twice-lone {fname} u0={u0:.3}"), show(curved_face_loops(&cyl::<T>(f), &[&twice], true, band), None));
            // one rim edge spanning 2 turns
            let span2 = vec![crim::<T>(f, 0.0, u0, u0 + 2.0 * TAU)];
            let span2b = vec![crim::<T>(f, 1.0, u0 + 2.0 * TAU, u0)];
            row(format!("cyl pm2-oneedge {fname} u0={u0:.3}"), show(curved_face_loops(&cyl::<T>(f), &[&span2, &span2b], true, band), None));
            let ttw = vec![trim::<T>(f, 0.2, u0, u0 + TAU), trim::<T>(f, 0.2, u0, u0 + TAU)];
            let ttwb = vec![trim::<T>(f, 0.7, u0 + TAU, u0), trim::<T>(f, 0.7, u0 + TAU, u0)];
            row(format!("tor pm2 {fname} u0={u0:.3}"), show(curved_face_loops(&tor::<T>(f), &[&ttw, &ttwb], true, band), None));
            // seam-band face whose rim overruns: u0..u0+tau+1 rims with seam ruling at u0+1 (k=1 lone, W)
            let ov = vec![crim::<T>(f, 0.0, u0, u0 + TAU + 1.0), ruling(f, u0 + 1.0, 0.0, 1.0), crim(f, 1.0, u0 + TAU + 1.0, u0), ruling(f, u0, 1.0, 0.0)];
            row(format!("cyl overrun {fname} u0={u0:.3}"), show(curved_face_loops(&cyl::<T>(f), &[&ov], true, band), None));
            // half-turn sweeps: rim u0..u0+pi, ruling, rim back u0+pi .. u0 (closed, fine) vs W near pi
            let hp = vec![crim::<T>(f, 0.0, u0, u0 + PI), ruling(f, u0 + PI, 0.0, 1.0), crim(f, 1.0, u0 + PI, u0 + 2.0 * PI), ruling(f, u0, 1.0, 0.0)];
            row(format!("cyl W~tau-open? {fname} u0={u0:.3}"), show(curved_face_loops(&cyl::<T>(f), &[&hp], true, band), None));
        }
    }
}

#[test]
fn review_4938_differential() {
    let mut out = Vec::new();
    for eps in [1e-6, 1e-9, 1e-12] {
        sweep::<f64>("f64", eps, &mut out);
    }
    sweep::<Interval>("iv", 1e-9, &mut out);
    let path = std::env::var("PROBE_OUT").unwrap_or_else(|_| "/tmp/probe_out.txt".into());
    std::fs::write(path, out.join("\n")).unwrap();
}

/// Siblings out of the PR's scope: a ±2 pair (each loop two whole rims)
/// on a cone frustum and on a sphere zone, and the legal single-turn band.
#[test]
fn review_4938_pm2_siblings() {
    let band = Band::linear_at(Tol::witness(), 1e-9).unwrap();
    let f = Frame { o: [0.0; 3], z: [0.0, 0.0, 1.0], x: [1.0, 0.0, 0.0] };
    let ta = 0.5f64.tan();
    let cone = Surface::<f64>::Cone { apex: p(f.o), axis: v(f.z), half_angle: 0.5, u_ref: v(f.x) };
    let rim = |h: f64, rad: f64, a: f64, b: f64| edge::<f64>(Curve3::Circle { center: p([0.0, 0.0, h]), axis: v(f.z), radius: rad, u_ref: v(f.x) }, a, b);
    let one = (vec![rim(1.0, ta, 0.0, TAU)], vec![rim(2.0, 2.0 * ta, TAU, 0.0)]);
    let two = (vec![rim(1.0, ta, 0.0, TAU), rim(1.0, ta, 0.0, TAU)], vec![rim(2.0, 2.0 * ta, TAU, 0.0), rim(2.0, 2.0 * ta, TAU, 0.0)]);
    for (name, (a, b)) in [("cone band", one.clone()), ("cone pm2", two.clone())] {
        for sense in [true, false] {
            let r = curved_face_loops(&cone, &[&a, &b], sense, band);
            eprintln!("REVIEW4938 {name} sense={sense}: {}", show(r, None));
        }
    }
    let sph = Surface::<f64>::Sphere { center: p(f.o), radius: 1.0, axis: v(f.z), u_ref: v(f.x) };
    let lat = |la: f64, a: f64, b: f64| rim(la.sin(), la.cos(), a, b);
    let one = (vec![lat(0.2, 0.0, TAU)], vec![lat(0.7, TAU, 0.0)]);
    let two = (vec![lat(0.2, 0.0, TAU), lat(0.2, 0.0, TAU)], vec![lat(0.7, TAU, 0.0), lat(0.7, TAU, 0.0)]);
    for (name, (a, b)) in [("sphere band", one), ("sphere pm2", two)] {
        for sense in [true, false] {
            let r = curved_face_loops(&sph, &[&a, &b], sense, band);
            eprintln!("REVIEW4938 {name} sense={sense}: {}", show(r, None));
        }
    }
}
