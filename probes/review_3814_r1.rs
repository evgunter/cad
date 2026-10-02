//! Reviewer probes, PR #3814 (lane reach-dual3814-r1). Not a suite row:
//! run by `#[path]`-including this file from `crates/sweep/tests/all.rs`.
//!
//! The oracle is independent of the kernel: point membership is the
//! closed-form annulus/disc test in the fixture's local frame, sampled
//! by a deterministic LCG and asked of `point_in_solid` on the result;
//! volumes against the closed forms.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use crate::common::three_arc;
use core::f64::consts::{FRAC_PI_2, PI};
use geom_core::{Affine3, Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use sweep::test_support::{extruded, sketch_at};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{
    Body, BooleanDeclarations, BooleanOp, BooleanResult, ContactClass, FacePairDeclaration,
    SolidContainment, mass_properties, point_in_solid,
};

#[derive(Clone, Copy, Debug)]
struct Fx {
    s: f64,   // scale
    r: f64,   // bore radius (unit)
    rr: f64,  // outer radius (unit)
    rs: f64,  // shaft radius (unit)
    ecc: f64, // shaft axis offset along x (unit)
    tilt: f64, // shaft tilt about z (rad), about its own base centre
}

impl Fx {
    fn std(r: f64, rr: f64) -> Self {
        Fx { s: 1.0, r, rr, rs: r, ecc: 0.0, tilt: 0.0 }
    }
}

fn collar(f: Fx) -> Body<f64> {
    let s = f.s;
    let lp = ProfileLoop::polygon([
        Point2::new(f.r * s, 1.0 * s),
        Point2::new(f.rr * s, 1.0 * s),
        Point2::new(f.rr * s, 2.0 * s),
        Point2::new(f.r * s, 2.0 * s),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp]).validate(Tol::witness()).unwrap();
    let axis = RevolveAxis { origin: Point2::new(0.0, 0.0), dir: Vec2::new(0.0, 1.0) };
    revolve(&vp, axis, Revolution::Full, Tol::witness()).unwrap().body
}

fn shaft(f: Fx, deg: f64, y0: f64, h: f64) -> Body<f64> {
    let s = f.s;
    let peg = extruded(
        sketch_at(y0 * s),
        vec![three_arc(Point2::new(f.ecc * s, 0.0), f.rs * s, deg)],
        h * s,
        Tol::witness(),
    );
    let up = Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), -FRAC_PI_2);
    let b = topo::transform_rigid(&peg, &up, Tol::witness()).unwrap();
    if f.tilt == 0.0 {
        return b;
    }
    let t = Affine3::rotation_about_axis(Point3::new(f.ecc * s, 1.5 * s, 0.0), Vec3::new(0.0, 0.0, 1.0), f.tilt);
    topo::transform_rigid(&b, &t, Tol::witness()).unwrap()
}

fn placed(b: &Body<f64>, pose: &Affine3<f64>) -> Body<f64> {
    topo::transform_rigid(b, pose, Tol::witness()).unwrap()
}

/// Every cylinder pair within 1e-3 relative of radius r: a sloppy
/// author declares Rest on a shaft that does not quite fit.
fn decls(a: &Body<f64>, b: &Body<f64>, r: f64) -> BooleanDeclarations {
    let walls = |body: &Body<f64>| -> Vec<topo::FaceKey> {
        body.faces()
            .filter(|(_, fc)| matches!(body.get_surface(fc.surface),
                Some(geom::Surface::Cylinder { radius, .. }) if (radius - r).abs() < 1e-3 * r))
            .map(|(k, _)| k)
            .collect()
    };
    let mut d = BooleanDeclarations::none();
    for &fa in &walls(a) {
        for &fb in &walls(b) {
            d.coincident_faces.push(FacePairDeclaration::new(fa, fb, ContactClass::Rest));
        }
    }
    d
}

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
}

/// Membership in the local (unposed, unit) frame; None within `m` of a boundary.
fn in_collar(f: Fx, p: [f64; 3], m: f64) -> Option<bool> {
    let rho = (p[0] * p[0] + p[2] * p[2]).sqrt();
    let d = [rho - f.r, f.rr - rho, p[1] - 1.0, 2.0 - p[1]];
    if d.iter().any(|x| x.abs() < m) {
        return None;
    }
    Some(d.iter().all(|&x| x > 0.0))
}

fn in_shaft(f: Fx, p: [f64; 3], y0: f64, h: f64, m: f64) -> Option<bool> {
    // undo tilt about (ecc, 1.5, 0) around z
    let (c, sn) = (f.tilt.cos(), f.tilt.sin());
    let (x, y) = (p[0] - f.ecc, p[1] - 1.5);
    let (x, y) = (c * x + sn * y, -sn * x + c * y);
    let (x, y, z) = (x, y + 1.5, p[2]);
    let rho = (x * x + z * z).sqrt();
    let d = [f.rs - rho, y - y0, y0 + h - y];
    if d.iter().any(|v| v.abs() < m) {
        return None;
    }
    Some(d.iter().all(|&v| v > 0.0))
}

/// Ask `point_in_solid` on `body` (posed by `pose`, scale s) at n points;
/// return the count of disagreements with `want`, and of non-answers.
fn sample(
    body: &Body<f64>, f: Fx, pose: &Affine3<f64>, n: usize, seed: u64,
    want: &dyn Fn([f64; 3]) -> Option<bool>,
) -> (usize, usize, usize) {
    let mut g = Lcg(seed);
    let ext = f.rr.max(f.r + f.ecc.abs() + 0.1) + 0.1;
    let (mut bad, mut unans, mut asked) = (0, 0, 0);
    let band = crate::common::approx::band();
    while asked < n {
        let p = [(2.0 * g.next() - 1.0) * ext, 0.3 + 2.4 * g.next(), (2.0 * g.next() - 1.0) * ext];
        let Some(w) = want(p) else { continue };
        asked += 1;
        let q = pose.transform_point(Point3::new(p[0] * f.s, p[1] * f.s, p[2] * f.s));
        match point_in_solid(body, q, band, Tol::witness()) {
            Ok(SolidContainment::In) if w => {}
            Ok(SolidContainment::Out) if !w => {}
            Ok(_) => bad += 1,
            Err(_) => unans += 1,
        }
    }
    (bad, unans, asked)
}

fn vol(b: &Body<f64>) -> f64 {
    mass_properties(b, Tol::witness()).unwrap().volume
}

fn poses() -> Vec<(&'static str, Affine3<f64>)> {
    crate::common::poses::poses()
}

fn tier3(b: &topo::BooleanBody<f64>) -> bool {
    topo::validate_geometric(&b.body, Tol::witness()).is_ok()
        && topo::validate_pseudomanifold(&b.body, &b.contacts, Tol::witness()).is_ok()
}

/// Run one union and judge it against the oracle. Returns a short verdict
/// string; "WRONG" anywhere is a silent wrong body.
fn judge_union(f: Fx, deg: f64, y0: f64, h: f64, pose: &Affine3<f64>, swap: bool, n: usize) -> String {
    let c = placed(&collar(f), pose);
    let p = placed(&shaft(f, deg, y0, h), pose);
    let (a, b) = if swap { (&p, &c) } else { (&c, &p) };
    let d = decls(a, b, f.r * f.s);
    match topo::union_with(a, b, &d, Tol::witness()) {
        Err(e) => format!("refused {e:?}"),
        Ok(BooleanResult::Empty) => "WRONG empty".into(),
        Ok(BooleanResult::Body(bb)) => {
            let s3 = f.s.powi(3);
            // closed form only for the fitted, untilted shaft
            let cyl_in = (y0 + h).min(2.0) - y0.max(1.0);
            let want = s3 * (PI * (f.rr * f.rr - f.r * f.r) + PI * f.rs * f.rs * h
                - if f.rs > f.r { PI * (f.rs * f.rs - f.r * f.r) * cyl_in.max(0.0) } else { 0.0 });
            let got = vol(&bb.body);
            let fitted = f.ecc == 0.0 && f.tilt == 0.0;
            let (bad, un, asked) = sample(&bb.body, f, pose, n, 7 + deg as u64, &|q| {
                let m = 1e-6;
                match (in_collar(f, q, m), in_shaft(f, q, y0, h, m)) {
                    (Some(x), Some(y)) => Some(x || y),
                    _ => None,
                }
            });
            let shells = bb.body.shells().count();
            let rel = (got - want).abs() / want;
            let vol_ok = !fitted || rel < 1e-9;
            let ok = bad == 0 && vol_ok && tier3(&bb) && shells == 1;
            format!(
                "{} body: vol rel {rel:.1e}{} shells {shells} tier3+census {} pis bad {bad}/{asked} unans {un}",
                if ok { "ok" } else { "WRONG" },
                if fitted { "" } else { " (no closed form)" },
                tier3(&bb)
            )
        }
    }
}

#[test]
fn r1_widened_unions() {
    let spans = [("through", 0.5, 2.0), ("flush", 1.0, 1.0), ("flush-lo", 1.0, 1.5), ("flush-hi", 0.5, 1.5)];
    let azimuths = [0.0, 1e-7, 7.3, 60.0, 119.9, 180.0, 243.7];
    let mut wrong = Vec::new();
    let mut tally = std::collections::BTreeMap::<String, usize>::new();
    for (pn, pose) in poses() {
        for &deg in &azimuths {
            for (sn, y0, h) in spans {
                for swap in [false, true] {
                    let v = judge_union(Fx::std(0.5, 1.5), deg, y0, h, &pose, swap, 60);
                    let key = v.split(':').next().unwrap().to_string();
                    let key = if key.starts_with("refused") { v.clone() } else { key };
                    *tally.entry(key).or_default() += 1;
                    if v.contains("WRONG") {
                        wrong.push(format!("{pn} az {deg} {sn} swap {swap}: {v}"));
                    }
                }
            }
        }
    }
    for (k, n) in &tally {
        eprintln!("[r1-widened] {n:4} × {k}");
    }
    for w in &wrong {
        eprintln!("[r1-widened] {w}");
    }
    assert!(wrong.is_empty());
}

#[test]
fn r1_radii_and_scales() {
    let mut wrong = Vec::new();
    for (r, rr) in [(0.5, 1.5), (0.3, 1.0), (1.7, 2.5), (0.05, 0.06)] {
        for s in [1e-3, 1.0, 1e3] {
            for (pn, pose) in poses().into_iter().take(3) {
                for deg in [0.0, 60.0, 200.0] {
                    for (sn, y0, h) in [("through", 0.5, 2.0), ("flush", 1.0, 1.0)] {
                        let f = Fx { s, ..Fx::std(r, rr) };
                        let v = judge_union(f, deg, y0, h, &pose, false, 30);
                        eprintln!("[r1-scale] r {r} R {rr} s {s} {pn} az {deg} {sn}: {v}");
                        if v.contains("WRONG") {
                            wrong.push(v);
                        }
                    }
                }
            }
        }
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}

#[test]
fn r1_misfit_shafts_refuse_or_are_right() {
    let mut wrong = Vec::new();
    let base = Fx::std(0.5, 1.5);
    let cases = [
        ("rs+1e-4", Fx { rs: 0.5 * (1.0 + 1e-4), ..base }),
        ("rs-1e-4", Fx { rs: 0.5 * (1.0 - 1e-4), ..base }),
        ("rs-1e-12", Fx { rs: 0.5 - 1e-12, ..base }),
        ("ecc 1e-4", Fx { ecc: 1e-4, ..base }),
        ("ecc 1e-9", Fx { ecc: 1e-9, ..base }),
        ("ecc 1e-13", Fx { ecc: 1e-13, ..base }),
        ("tilt 1e-5", Fx { tilt: 1e-5, ..base }),
        ("tilt 1e-9", Fx { tilt: 1e-9, ..base }),
        ("tilt 1e-13", Fx { tilt: 1e-13, ..base }),
    ];
    for (name, f) in cases {
        for (pn, pose) in poses().into_iter().take(2) {
            for deg in [0.0, 60.0] {
                for (sn, y0, h) in [("through", 0.5, 2.0), ("flush", 1.0, 1.0)] {
                    let v = judge_union(f, deg, y0, h, &pose, false, 60);
                    eprintln!("[r1-misfit] {name} {pn} az {deg} {sn}: {v}");
                    if v.contains("WRONG") {
                        wrong.push(format!("{name} {pn} az {deg} {sn}: {v}"));
                    }
                }
            }
        }
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}

#[test]
fn r1_blind_and_other_ops() {
    let f = Fx::std(0.5, 1.5);
    for (pn, pose) in poses().into_iter().take(2) {
        for deg in [0.0, 60.0] {
            for (y0, h) in [(0.5, 1.0), (1.5, 1.0), (1.2, 0.5), (1.0, 0.5)] {
                let v = judge_union(f, deg, y0, h, &pose, false, 40);
                eprintln!("[r1-blind] {pn} az {deg} y0 {y0} h {h}: {v}");
                assert!(!v.contains("WRONG"), "{v}");
            }
            for (y0, h) in [(0.5, 2.0), (1.0, 1.0)] {
                let c = placed(&collar(f), &pose);
                let p = placed(&shaft(f, deg, y0, h), &pose);
                for (op, a, b, ab) in [
                    (BooleanOp::Intersect, &c, &p, 0u8),
                    (BooleanOp::Subtract, &c, &p, 1),
                    (BooleanOp::Subtract, &p, &c, 2),
                    (BooleanOp::Intersect, &p, &c, 0),
                ] {
                    let d = decls(a, b, 0.5);
                    let out = topo::boolean_op_with(op, a, b, &d, topo::SweepStrategy::Realized, Tol::witness());
                    let v = match out {
                        Err(e) => format!("refused {e:?}"),
                        Ok(BooleanResult::Empty) => if ab == 0 { "ok empty".into() } else { "WRONG empty".into() },
                        Ok(BooleanResult::Body(bb)) => {
                            let (bad, un, n) = sample(&bb.body, f, &pose, 40, 3, &|q| {
                                match (in_collar(f, q, 1e-6), in_shaft(f, q, y0, h, 1e-6)) {
                                    (Some(x), Some(y)) => Some(match ab { 1 => x && !y, 2 => y && !x, _ => x && y }),
                                    _ => None,
                                }
                            });
                            format!("{} body pis bad {bad}/{n} unans {un}", if bad == 0 && ab != 0 { "ok" } else { "WRONG" })
                        }
                    };
                    eprintln!("[r1-ops] {pn} az {deg} y0 {y0} {op:?} {ab}: {v}");
                    assert!(!v.contains("WRONG"), "{v}");
                }
            }
        }
    }
}

#[test]
fn r1_results_reused() {
    let f = Fx::std(0.5, 1.5);
    for (pn, pose) in poses().into_iter().take(3) {
        for deg in [0.0, 60.0] {
            let c = placed(&collar(f), &pose);
            let p = placed(&shaft(f, deg, 0.5, 2.0), &pose);
            let u = match topo::union_with(&c, &p, &decls(&c, &p, 0.5), Tol::witness()) {
                Ok(BooleanResult::Body(bb)) => bb.body,
                o => panic!("{pn}: {:?}", o.err()),
            };
            // A second collar on the shaft's protruding top, y ∈ [2.1, 2.4].
            let lp = ProfileLoop::polygon([
                Point2::new(0.5, 2.1), Point2::new(1.0, 2.1), Point2::new(1.0, 2.4), Point2::new(0.5, 2.4),
            ]);
            let vp = Profile::new(SketchPlane::xy(), vec![lp]).validate(Tol::witness()).unwrap();
            let axis = RevolveAxis { origin: Point2::new(0.0, 0.0), dir: Vec2::new(0.0, 1.0) };
            let c2 = placed(&revolve(&vp, axis, Revolution::Full, Tol::witness()).unwrap().body, &pose);
            let want = PI * 2.0 + PI * 0.25 * 2.0 + PI * 0.75 * 0.3;
            for (a, b) in [(&u, &c2), (&c2, &u)] {
                let v = match topo::union_with(a, b, &decls(a, b, 0.5), Tol::witness()) {
                    Err(e) => format!("refused {e:?}"),
                    Ok(BooleanResult::Empty) => "WRONG empty".into(),
                    Ok(BooleanResult::Body(bb)) => {
                        let got = vol(&bb.body);
                        let (bad, un, n) = sample(&bb.body, f, &pose, 60, 11, &|q| {
                            let rho = (q[0] * q[0] + q[2] * q[2]).sqrt();
                            let m = 1e-6;
                            let near = [rho - 0.5, rho - 1.5, rho - 1.0, q[1] - 0.5, q[1] - 1.0, q[1] - 2.0, q[1] - 2.1, q[1] - 2.4, q[1] - 2.5];
                            if near.iter().any(|x| x.abs() < m) { return None; }
                            let shaft = rho < 0.5 && q[1] > 0.5 && q[1] < 2.5;
                            let c1 = rho > 0.5 && rho < 1.5 && q[1] > 1.0 && q[1] < 2.0;
                            let c2 = rho > 0.5 && rho < 1.0 && q[1] > 2.1 && q[1] < 2.4;
                            Some(shaft || c1 || c2)
                        });
                        let rel = (got - want).abs() / want;
                        format!("{} vol rel {rel:.1e} pis bad {bad}/{n} unans {un} tier3+census {}",
                            if rel < 1e-9 && bad == 0 { "ok" } else { "WRONG" }, tier3(&bb))
                    }
                };
                eprintln!("[r1-reuse] {pn} az {deg}: {v}");
                assert!(!v.contains("WRONG"), "{v}");
            }
        }
    }
}

#[test]
fn r1_grid() {
    let spans = [("through", 0.5, 2.0), ("flush", 1.0, 1.0), ("flush-lo", 1.0, 1.5), ("flush-hi", 0.5, 1.5)];
    for (pn, pose) in poses().into_iter().take(2) {
        for deg in [0.0, 1e-7, 7.3, 60.0, 119.9, 180.0, 243.7] {
            for (sn, y0, h) in spans {
                for swap in [false, true] {
                    let v = judge_union(Fx::std(0.5, 1.5), deg, y0, h, &pose, swap, 30);
                    let v: String = v.chars().take(70).collect();
                    eprintln!("[r1-grid] {:<8} az {deg:<6} {sn:<8} {}: {v}", &pn[..8.min(pn.len())], if swap { "P∪C" } else { "C∪P" });
                }
            }
        }
    }
}

/// A collar whose bore is TWO full-turn faces, split by a circle at
/// y = `ys` that no planar face touches: the only fixture shape on
/// which a ruling's crossing of a bore boundary is seen by nobody but
/// the on-carrier crossing step.
fn split_collar(ys: f64) -> Body<f64> {
    let lp = ProfileLoop::polygon([
        Point2::new(0.5, 1.0),
        Point2::new(1.5, 1.0),
        Point2::new(1.5, 2.0),
        Point2::new(0.5, 2.0),
        Point2::new(0.5, ys),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp]).validate(Tol::witness()).unwrap();
    let axis = RevolveAxis { origin: Point2::new(0.0, 0.0), dir: Vec2::new(0.0, 1.0) };
    revolve(&vp, axis, Revolution::Full, Tol::witness()).unwrap().body
}

#[test]
fn r1_split_bore() {
    let f = Fx::std(0.5, 1.5);
    let c0 = split_collar(1.5);
    let walls = c0.faces().filter(|(_, fc)| matches!(c0.get_surface(fc.surface), Some(geom::Surface::Cylinder { radius, .. }) if (radius - 0.5).abs() < 1e-9)).count();
    eprintln!("[r1-split] bore wall faces: {walls}");
    for (pn, pose) in poses() {
        for deg in [0.0, 60.0] {
            for (sn, y0, h) in [("through", 0.5, 2.0), ("flush", 1.0, 1.0), ("flush-lo", 1.0, 1.5)] {
                for swap in [false, true] {
                    let c = placed(&c0, &pose);
                    let p = placed(&shaft(f, deg, y0, h), &pose);
                    let (a, b) = if swap { (&p, &c) } else { (&c, &p) };
                    let v = match topo::union_with(a, b, &decls(a, b, 0.5), Tol::witness()) {
                        Err(e) => format!("refused {e:?}"),
                        Ok(BooleanResult::Empty) => "WRONG empty".into(),
                        Ok(BooleanResult::Body(bb)) => {
                            let want = PI * 2.0 + PI * 0.25 * h;
                            let got = vol(&bb.body);
                            let (bad, un, n) = sample(&bb.body, f, &pose, 60, 5, &|q| {
                                match (in_collar(f, q, 1e-6), in_shaft(f, q, y0, h, 1e-6)) {
                                    (Some(x), Some(y)) => Some(x || y),
                                    _ => None,
                                }
                            });
                            let rel = (got - want).abs() / want;
                            let t3 = tier3(&bb);
                            format!("{} vol {got} rel {rel:.1e} shells {} tier3+census {t3} pis bad {bad}/{n} unans {un}",
                                if rel < 1e-9 && bad == 0 && t3 { "ok" } else { "WRONG" }, bb.body.shells().count())
                        }
                    };
                    let v: String = v.chars().take(160).collect();
                    eprintln!("[r1-split] {:<8} az {deg} {sn} {}: {v}", &pn[..8.min(pn.len())], if swap { "P∪C" } else { "C∪P" });
                }
            }
        }
    }
}

/// Undeclared: a box whose bottom face is the tangent plane of a
/// cylinder, its corner vertices ON the cylinder — a planar sector on a
/// curved pierced face, the arm `vtxfac`'s gate keeps shut.
#[test]
fn r1_tangent_box_undeclared() {
    let tol = Tol::witness();
    for deg in [0.0, 90.0, 30.0] {
        let cyl = extruded(sketch_at(-1.0), vec![three_arc(Point2::new(0.0, 0.0), 0.5, deg)], 2.0, tol);
        for (bx, tag) in [((0.0, 1.0), "corner on the tangent line"), ((-0.3, 0.7), "straddling")] {
            let bk: Body<f64> = sweep::test_support::brick(bx, (0.5, 1.0), (-0.5, 0.5), tol);
            let vb = (bx.1 - bx.0) * 0.5 * 1.0;
            let vc = PI * 0.25 * 2.0;
            for (op, a, b, want) in [
                (BooleanOp::Union, &cyl, &bk, Some(vc + vb)),
                (BooleanOp::Union, &bk, &cyl, Some(vc + vb)),
                (BooleanOp::Intersect, &cyl, &bk, None),
                (BooleanOp::Subtract, &cyl, &bk, Some(vc)),
                (BooleanOp::Subtract, &bk, &cyl, Some(vb)),
            ] {
                let out = topo::boolean_op_with(op, a, b, &BooleanDeclarations::none(), topo::SweepStrategy::Realized, tol);
                let v = match (out, want) {
                    (Err(e), _) => format!("refused {e:?}"),
                    (Ok(BooleanResult::Empty), None) => "ok empty".into(),
                    (Ok(BooleanResult::Body(bb)), Some(w)) => {
                        let got = vol(&bb.body);
                        format!("{} vol {got} vs {w}", if (got - w).abs() < 1e-9 { "ok" } else { "WRONG" })
                    }
                    (Ok(r), _) => format!("WRONG {:?}", r.body().is_some()),
                };
                let v: String = v.chars().take(170).collect();
                eprintln!("[r1-tangent] az {deg} {tag} {op:?} {}: {v}", if core::ptr::eq(a, &cyl) { "cyl,box" } else { "box,cyl" });
            }
        }
    }
}
