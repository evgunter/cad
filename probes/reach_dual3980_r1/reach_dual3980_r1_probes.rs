//! Reviewer probes for PR #3980 (lane reach-dual3980-r1). The oracle is
//! closed form (cylinder / annulus / sphere volumes, the two-circle lens),
//! never the kernel. Every probe prints a `PROBE` line so the same file run
//! on main and on the head gives a verdict differential.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::approx::band;
use crate::common::three_arc;
use core::f64::consts::{FRAC_PI_2, PI};
use geom_core::{Affine3, Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use sweep::test_support::{extruded, sketch_at};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{
    Body, BooleanDeclarations, BooleanError, BooleanOp, BooleanResult, ContactClass,
    FacePairDeclaration, SolidContainment, mass_properties, point_in_solid,
};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Layout {
    ArcSplit,
    FullTurn,
}

/// A collar (bore `rb`, wall `wall`, z in [base, base+len]) on the z axis,
/// and a shaft of radius `rs`, span (z0, h), azimuth `deg`, axis offset
/// `dx` along x and tilt `tilt` radians about the x axis through its middle.
#[derive(Clone, Copy, Debug)]
struct Cell {
    layout: Layout,
    rb: f64,
    rs: f64,
    wall: f64,
    base: f64,
    len: f64,
    z0: f64,
    h: f64,
    deg: f64,
    dx: f64,
    tilt: f64,
}

impl Cell {
    fn outer(self) -> f64 {
        self.rb + self.wall
    }
    fn collar(self) -> Body<f64> {
        let (r, outer) = (self.rb, self.outer());
        match self.layout {
            Layout::ArcSplit => extruded(
                sketch_at(self.base),
                vec![
                    three_arc(Point2::new(0.0, 0.0), outer, 0.0),
                    three_arc(Point2::new(0.0, 0.0), r, 0.0),
                ],
                self.len,
                Tol::witness(),
            ),
            Layout::FullTurn => {
                // Revolved about y over y in [-(base+len), -base], then a
                // quarter turn about x takes y to z... y -> z needs +pi/2:
                // (0,y,0) -> (0,0,y). So build over y in [base, base+len].
                let (y0, y1) = (self.base, self.base + self.len);
                let lp = ProfileLoop::polygon([
                    Point2::new(r, y0),
                    Point2::new(outer, y0),
                    Point2::new(outer, y1),
                    Point2::new(r, y1),
                ]);
                let vp = Profile::new(SketchPlane::xy(), vec![lp])
                    .validate(Tol::witness())
                    .unwrap();
                let axis = RevolveAxis {
                    origin: Point2::new(0.0, 0.0),
                    dir: Vec2::new(0.0, 1.0),
                };
                let body = revolve(&vp, axis, Revolution::Full, Tol::witness())
                    .unwrap()
                    .body;
                let up = Affine3::rotation_about_axis(
                    Point3::origin(),
                    Vec3::new(1.0, 0.0, 0.0),
                    FRAC_PI_2,
                );
                topo::transform_rigid(&body, &up, Tol::witness()).unwrap()
            }
        }
    }
    fn shaft(self) -> Body<f64> {
        let peg = extruded(
            sketch_at(self.z0),
            vec![three_arc(Point2::new(self.dx, 0.0), self.rs, self.deg)],
            self.h,
            Tol::witness(),
        );
        if self.tilt == 0.0 {
            return peg;
        }
        let mid = Point3::new(self.dx, 0.0, self.z0 + 0.5 * self.h);
        let t = Affine3::rotation_about_axis(mid, Vec3::new(1.0, 0.0, 0.0), self.tilt);
        topo::transform_rigid(&peg, &t, Tol::witness()).unwrap()
    }
    fn collar_vol(self) -> f64 {
        PI * (self.outer().powi(2) - self.rb * self.rb) * self.len
    }
    fn shaft_vol(self) -> f64 {
        PI * self.rs * self.rs * self.h
    }
    /// Axial overlap of the shaft span with the collar span.
    fn overlap(self) -> f64 {
        let lo = self.z0.max(self.base);
        let hi = (self.z0 + self.h).min(self.base + self.len);
        (hi - lo).max(0.0)
    }
    /// The closed-form ∩ volume, for an untilted shaft inside the outer
    /// radius: (shaft disk minus its overlap with the bore disk) × overlap.
    fn meet_vol(self) -> Option<f64> {
        if self.tilt != 0.0 {
            return None;
        }
        let area = PI * self.rs * self.rs - disk_overlap(self.rs, self.rb, self.dx.abs());
        Some(area.max(0.0) * self.overlap())
    }
}

/// Area of the intersection of two disks, radii r1 r2, centres d apart.
fn disk_overlap(r1: f64, r2: f64, d: f64) -> f64 {
    if d >= r1 + r2 {
        return 0.0;
    }
    if d <= (r1 - r2).abs() {
        return PI * r1.min(r2).powi(2);
    }
    let a1 = ((d * d + r1 * r1 - r2 * r2) / (2.0 * d * r1)).acos();
    let a2 = ((d * d + r2 * r2 - r1 * r1) / (2.0 * d * r2)).acos();
    r1 * r1 * a1 + r2 * r2 * a2
        - 0.5 * ((-d + r1 + r2) * (d + r1 - r2) * (d - r1 + r2) * (d + r1 + r2)).sqrt()
}

fn faces_on(body: &Body<f64>, r: f64, sphere: bool) -> Vec<topo::FaceKey> {
    body.faces()
        .filter(|(_, f)| match body.get_surface(f.surface) {
            Some(geom::Surface::Cylinder { radius, .. }) if !sphere => {
                (radius - r).abs() < r * 1e-12
            }
            Some(geom::Surface::Sphere { radius, .. }) if sphere => {
                (radius - r).abs() < r * 1e-12
            }
            _ => false,
        })
        .map(|(k, _)| k)
        .collect()
}

fn continuations(a: &Body<f64>, b: &Body<f64>) -> BooleanDeclarations {
    match topo::flush::find_flush_candidates(a, b, Tol::witness()) {
        Ok(found) => {
            let picked: Vec<_> = found
                .into_iter()
                .filter(|f| f.class == topo::BooleanCoincidence::Continuation)
                .collect();
            topo::flush::declare_all(&picked)
        }
        Err(_) => BooleanDeclarations::none(),
    }
}

/// Every (`a` face at `ra`) × (`b` face at `rb`) declared Rest, plus
/// every continuation.
fn rest_decls(a: &Body<f64>, ra: f64, b: &Body<f64>, rb: f64, sphere: bool) -> BooleanDeclarations {
    let mut d = if sphere { BooleanDeclarations::none() } else { continuations(a, b) };
    let (fa, fb) = (faces_on(a, ra, sphere), faces_on(b, rb, sphere));
    assert!(!fa.is_empty() && !fb.is_empty(), "walls found: {} {}", fa.len(), fb.len());
    for &x in &fa {
        for &y in &fb {
            d.coincident_faces
                .push(FacePairDeclaration::new(x, y, ContactClass::Rest));
        }
    }
    d
}

fn vol(b: &Body<f64>) -> f64 {
    mass_properties(b, Tol::witness()).unwrap().volume
}

fn err_name(e: &BooleanError) -> String {
    let s = format!("{e:?}");
    let cut = s.find([' ', '{', '(']).unwrap_or(s.len());
    s[..cut].to_string()
}

fn summary(r: &Result<BooleanResult<f64>, BooleanError>) -> String {
    match r {
        Ok(BooleanResult::Empty) => "Empty".into(),
        Ok(BooleanResult::Body(bb)) => {
            format!("Body(vol={:.12e}, shells={})", vol(&bb.body), bb.body.shells().count())
        }
        Err(e) => format!("Err({})", err_name(e)),
    }
}

fn run(op: BooleanOp, a: &Body<f64>, b: &Body<f64>, d: &BooleanDeclarations) -> Result<BooleanResult<f64>, BooleanError> {
    topo::boolean_op_with(op, a, b, d, topo::SweepStrategy::Realized, Tol::witness())
}

/// Collected failures (wrong bodies, refusals inside the claim's domain).
#[derive(Default)]
struct Log {
    wrong: Vec<String>,
    refused: Vec<String>,
    max_rel: f64,
    answers: usize,
}

impl Log {
    /// Holds `r` to the closed form `want` (None = empty), shells `shells`
    /// (if Some), point samples `pts` (local -> expected).
    fn check(
        &mut self,
        tag: &str,
        r: &Result<BooleanResult<f64>, BooleanError>,
        want: Option<f64>,
        shells: Option<usize>,
        pts: &[(Point3<f64>, SolidContainment)],
        scale3: f64,
    ) {
        println!("PROBE {tag} => {}", summary(r));
        match (r, want) {
            (Err(e), _) => self.refused.push(format!("{tag}: {}", err_name(e))),
            (Ok(BooleanResult::Empty), None) => self.answers += 1,
            (Ok(BooleanResult::Empty), Some(w)) => {
                self.wrong.push(format!("{tag}: Empty, want volume {w:e}"))
            }
            (Ok(BooleanResult::Body(bb)), None) => self
                .wrong
                .push(format!("{tag}: body vol {:e}, want empty", vol(&bb.body))),
            (Ok(BooleanResult::Body(bb)), Some(w)) => {
                self.answers += 1;
                let got = vol(&bb.body);
                let rel = (got - w).abs() / scale3;
                self.max_rel = self.max_rel.max(rel);
                if rel > 1e-9 {
                    self.wrong.push(format!("{tag}: vol {got:e} vs oracle {w:e} (rel {rel:e})"));
                }
                if let Some(n) = shells {
                    let got_n = bb.body.shells().count();
                    if got_n != n {
                        self.wrong.push(format!("{tag}: {got_n} shells, want {n}"));
                    }
                }
                if let Err(e) = topo::validate_geometric(&bb.body, Tol::witness()) {
                    self.wrong.push(format!("{tag}: tier 3 {e:?}"));
                }
                for &(p, want_c) in pts {
                    match point_in_solid(&bb.body, p, band(), Tol::witness()) {
                        Ok(c) if c == want_c => {}
                        Err(e) => self.refused.push(format!(
                            "{tag}: point query {}",
                            format!("{e:?}").split(['{', '(']).next().unwrap_or("")
                        )),
                        other => self
                            .wrong
                            .push(format!("{tag}: point {p:?} -> {other:?}, want {want_c:?}")),
                    }
                }
            }
        }
    }
}

fn poses(s: f64) -> [(&'static str, Affine3<f64>); 2] {
    [
        ("id", Affine3::identity()),
        (
            "rot+shift",
            Affine3::translation(Vec3::new(3.0 * s, -2.0 * s, 5.0 * s))
                * Affine3::rotation_about_axis(
                    Point3::new(-0.2 * s, 0.1 * s, 0.4 * s),
                    Vec3::new(-2.0, 1.0, 0.5).normalize(),
                    2.3,
                ),
        ),
    ]
}

fn placed(b: &Body<f64>, p: &Affine3<f64>) -> Body<f64> {
    topo::transform_rigid(b, p, Tol::witness()).unwrap()
}

/// The fixture pose itself may refuse (at ε 1e-12, a large-scale rigid
/// map's certificate); that cell is skipped and counted, never the op's.
fn try_placed(b: &Body<f64>, p: &Affine3<f64>) -> Option<Body<f64>> {
    topo::transform_rigid(b, p, Tol::witness()).ok()
}

/// Claim 1, widened: other radii, lengths, partial insertion, an odd
/// azimuth, three scales, a rotated-and-translated pose; every op in both
/// orders; point samples; results reused as operands.
#[test]
fn claim1_widened_rest_matrix() {
    use SolidContainment::{In, OnBoundary, Out};
    let mut log = Log::default();
    for s in [1e-3, 1.0, 1e3] {
        for layout in [Layout::ArcSplit, Layout::FullTurn] {
            for (r, wall, len) in [(0.3, 0.45, 1.7), (2.0, 0.6, 0.4)] {
                let base = 0.7;
                let spans = [
                    ("through", base - 0.3 * len, 1.6 * len),
                    ("half-in", base - 0.5 * len, len),
                    ("blind-top", base + 0.25 * len, 1.5 * len),
                    ("inside", base + 0.2 * len, 0.5 * len),
                ];
                for (span, z0, h) in spans {
                    let c = Cell {
                        layout,
                        rb: r * s,
                        rs: r * s,
                        wall: wall * s,
                        base: base * s,
                        len: len * s,
                        z0: z0 * s,
                        h: h * s,
                        deg: 37.0,
                        dx: 0.0,
                        tilt: 0.0,
                    };
                    let Ok((c0, s0)) = std::panic::catch_unwind(|| (c.collar(), c.shaft())) else {
                        println!("SKIP fixture build refused: c1 s{s:e} {layout:?} r{r} len{len} {span}");
                        continue;
                    };
                    let s3 = s * s * s;
                    let lo = c.z0.max(c.base);
                    let hi = (c.z0 + c.h).min(c.base + c.len);
                    let zc = 0.5 * (lo + hi);
                    let ang = 1.234_f64;
                    let at = |rad: f64, z: f64| Point3::new(rad * ang.cos(), rad * ang.sin(), z);
                    let p_wall = at(c.rb + 0.5 * c.wall, c.base + 0.5 * c.len);
                    let p_shaft = at(0.5 * c.rb, c.z0 + 0.5 * c.h);
                    let p_contact = at(c.rb, zc);
                    let p_far = at(0.0, c.base - 3.0 * c.len - c.h);
                    for (pose_name, pose) in poses(s) {
                        let tag0 = format!("c1 s{s:e} {layout:?} r{r} len{len} {span} {pose_name}");
                        let (Some(cb), Some(sb)) = (try_placed(&c0, &pose), try_placed(&s0, &pose)) else {
                            println!("SKIP fixture pose refused: {tag0}");
                            continue;
                        };
                        let tp = |p: Point3<f64>| pose.transform_point(p);
                        let (cs, sc) = (
                            rest_decls(&cb, c.rb, &sb, c.rs, false),
                            rest_decls(&sb, c.rs, &cb, c.rb, false),
                        );
                        for (ord, a, b, d) in [("c∩s", &cb, &sb, &cs), ("s∩c", &sb, &cb, &sc)] {
                            let r = run(BooleanOp::Intersect, a, b, d);
                            log.check(&format!("{tag0} {ord}"), &r, None, None, &[], s3);
                        }
                        let r = run(BooleanOp::Subtract, &cb, &sb, &cs);
                        log.check(
                            &format!("{tag0} c∖s"),
                            &r,
                            Some(c.collar_vol()),
                            Some(1),
                            &[
                                (tp(p_wall), In),
                                (tp(p_shaft), Out),
                                (tp(p_contact), OnBoundary),
                                (tp(p_far), Out),
                            ],
                            s3,
                        );
                        // Reuse: (c ∖ s) ∩ s, and (c ∖ s) ∪ s, under fresh decls.
                        if let Ok(BooleanResult::Body(bb)) = &r {
                            let d2 = rest_decls(&bb.body, c.rb, &sb, c.rs, false);
                            let r2 = run(BooleanOp::Intersect, &bb.body, &sb, &d2);
                            log.check(&format!("{tag0} (c∖s)∩s"), &r2, None, None, &[], s3);
                            let r3 = run(BooleanOp::Union, &bb.body, &sb, &d2);
                            log.check(
                                &format!("{tag0} (c∖s)∪s"),
                                &r3,
                                Some(c.collar_vol() + c.shaft_vol()),
                                Some(1),
                                &[(tp(p_wall), In), (tp(p_shaft), In), (tp(p_contact), In)],
                                s3,
                            );
                        }
                        let r = run(BooleanOp::Subtract, &sb, &cb, &sc);
                        log.check(
                            &format!("{tag0} s∖c"),
                            &r,
                            Some(c.shaft_vol()),
                            Some(1),
                            &[
                                (tp(p_wall), Out),
                                (tp(p_shaft), In),
                                (tp(p_contact), OnBoundary),
                            ],
                            s3,
                        );
                        if let Ok(BooleanResult::Body(bb)) = &r {
                            let d2 = rest_decls(&bb.body, c.rs, &cb, c.rb, false);
                            let r2 = run(BooleanOp::Subtract, &bb.body, &cb, &d2);
                            log.check(
                                &format!("{tag0} (s∖c)∖c"),
                                &r2,
                                Some(c.shaft_vol()),
                                Some(1),
                                &[],
                                s3,
                            );
                        }
                        let skip_union = span == "blind-top" || span == "inside";
                        if !(skip_union && layout == Layout::FullTurn) {
                            for (ord, a, b, d) in [("c∪s", &cb, &sb, &cs), ("s∪c", &sb, &cb, &sc)] {
                                let r = run(BooleanOp::Union, a, b, d);
                                log.check(
                                    &format!("{tag0} {ord}"),
                                    &r,
                                    Some(c.collar_vol() + c.shaft_vol()),
                                    Some(1),
                                    &[(tp(p_wall), In), (tp(p_shaft), In), (tp(p_contact), In)],
                                    s3,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    report("claim1", &log, true);
}

fn report(name: &str, log: &Log, strict_refusals: bool) {
    println!(
        "SUMMARY {name}: answers {} max_rel {:e} wrong {} refused {}",
        log.answers,
        log.max_rel,
        log.wrong.len(),
        log.refused.len()
    );
    for w in &log.wrong {
        println!("WRONG {w}");
    }
    for r in &log.refused {
        println!("REFUSED {r}");
    }
    assert!(log.wrong.is_empty(), "{name}: {} wrong answers", log.wrong.len());
    if strict_refusals {
        assert!(log.refused.is_empty(), "{name}: {} refusals", log.refused.len());
    }
}

/// Claim 2: a declared Rest whose geometry is not a Rest. Any typed
/// refusal is fine; a body must match the closed form; ∩ must never
/// answer Empty where the parts overlap.
#[test]
fn claim2_lying_rest_declarations() {
    let mut log = Log::default();
    let base_cell = Cell {
        layout: Layout::ArcSplit,
        rb: 0.5,
        rs: 0.5,
        wall: 1.0,
        base: 1.0,
        len: 1.0,
        z0: 0.5,
        h: 2.0,
        deg: 0.0,
        dx: 0.0,
        tilt: 0.0,
    };
    let mut cells = Vec::new();
    for layout in [Layout::ArcSplit, Layout::FullTurn] {
        for d in [1e-3, -1e-3, 1e-6, -1e-6, 1e-7, -1e-7, 2e-8, -2e-8] {
            cells.push((format!("{layout:?} radius {d:+e}"), Cell { layout, rs: 0.5 + d, ..base_cell }));
        }
        for d in [1e-3, 1e-6, 1e-7, 2e-8] {
            cells.push((format!("{layout:?} off-axis {d:e}"), Cell { layout, dx: d, ..base_cell }));
        }
        for t in [1e-3, 1e-6, 1e-8] {
            cells.push((format!("{layout:?} tilt {t:e}"), Cell { layout, tilt: t, ..base_cell }));
        }
    }
    for (name, c) in cells {
        let (cb, sb) = (c.collar(), c.shaft());
        let (cs, sc) = (
            rest_decls(&cb, c.rb, &sb, c.rs, false),
            rest_decls(&sb, c.rs, &cb, c.rb, false),
        );
        let meet = c.meet_vol();
        let tag0 = format!("c2 {name}");
        for (ord, a, b, d) in [("c∩s", &cb, &sb, &cs), ("s∩c", &sb, &cb, &sc)] {
            let r = run(BooleanOp::Intersect, a, b, d);
            lie_check(&mut log, &format!("{tag0} {ord}"), &r, meet.map(|m| if m > 0.0 { Some(m) } else { None }), c.tilt != 0.0);
        }
        let r = run(BooleanOp::Subtract, &cb, &sb, &cs);
        lie_check(&mut log, &format!("{tag0} c∖s"), &r, meet.map(|m| Some(c.collar_vol() - m)), c.tilt != 0.0);
        let r = run(BooleanOp::Subtract, &sb, &cb, &sc);
        lie_check(&mut log, &format!("{tag0} s∖c"), &r, meet.map(|m| Some(c.shaft_vol() - m)), c.tilt != 0.0);
        let r = run(BooleanOp::Union, &cb, &sb, &cs);
        lie_check(&mut log, &format!("{tag0} c∪s"), &r, meet.map(|m| Some(c.collar_vol() + c.shaft_vol() - m)), c.tilt != 0.0);
    }
    // Spheres: ball radius r+δ, and a ball off-centre by δ, in a cavity r.
    for (name, rr, off) in [
        ("ball +1e-3", 0.5 + 1e-3, 0.0),
        ("ball -1e-3", 0.5 - 1e-3, 0.0),
        ("ball +1e-7", 0.5 + 1e-7, 0.0),
        ("ball -1e-7", 0.5 - 1e-7, 0.0),
        ("ball off 1e-3", 0.5, 1e-3),
        ("ball off 1e-7", 0.5, 1e-7),
    ] {
        let h = hollow(0.5, 1.0);
        let b = placed(&ball(rr), &Affine3::translation(Vec3::new(off, 0.0, 0.0)));
        let (hb, bh) = (rest_decls(&h, 0.5, &b, rr, true), rest_decls(&b, rr, &h, 0.5, true));
        let v = |r: f64| 4.0 / 3.0 * PI * r * r * r;
        // ∩ for a concentric ball: the shell between 0.5 and rr, if rr > 0.5.
        let meet = if off == 0.0 { Some(if rr > 0.5 { v(rr) - v(0.5) } else { 0.0 }) } else { None };
        let tag0 = format!("c2 {name}");
        for (ord, x, y, d) in [("h∩b", &h, &b, &hb), ("b∩h", &b, &h, &bh)] {
            let r = run(BooleanOp::Intersect, x, y, d);
            lie_check(&mut log, &format!("{tag0} {ord}"), &r, meet.map(|m| if m > 0.0 { Some(m) } else { None }), off != 0.0);
        }
        let r = run(BooleanOp::Subtract, &h, &b, &hb);
        lie_check(&mut log, &format!("{tag0} h∖b"), &r, meet.map(|m| Some(v(1.0) - v(0.5) - m)), off != 0.0);
        let r = run(BooleanOp::Subtract, &b, &h, &bh);
        lie_check(&mut log, &format!("{tag0} b∖h"), &r, meet.map(|m| Some(v(rr) - m)), off != 0.0);
        let r = run(BooleanOp::Union, &h, &b, &hb);
        lie_check(&mut log, &format!("{tag0} h∪b"), &r, meet.map(|m| Some(v(1.0) - v(0.5) + v(rr) - m)), off != 0.0);
    }
    report("claim2", &log, false);
}

/// `want`: None = no closed form (any body must at least be non-empty for
/// ∩ — `nonempty_meet`); Some(None) = empty; Some(Some(v)) = volume v.
fn lie_check(
    log: &mut Log,
    tag: &str,
    r: &Result<BooleanResult<f64>, BooleanError>,
    want: Option<Option<f64>>,
    overlap_unknown: bool,
) {
    match want {
        Some(w) => log.check(tag, r, w, None, &[], 1.0),
        None => {
            println!("PROBE {tag} => {}", summary(r));
            match r {
                Err(e) => log.refused.push(format!("{tag}: {}", err_name(e))),
                Ok(BooleanResult::Empty) if tag.contains('∩') && overlap_unknown => {
                    log.wrong.push(format!("{tag}: Empty, but the parts overlap"))
                }
                Ok(_) => log.wrong.push(format!("{tag}: built with no oracle: {}", summary(r))),
            }
        }
    }
}

fn ball(r: f64) -> Body<f64> {
    sweep::test_support::revolved_about_y(
        vec![(Point2::new(0.0, -r), 1.0), (Point2::new(0.0, r), 0.0)],
        Revolution::Full,
        Tol::witness(),
    )
}

fn hollow(r: f64, outer: f64) -> Body<f64> {
    match topo::subtract(&ball(outer), &ball(r), Tol::witness()) {
        Ok(BooleanResult::Body(bb)) => bb.body,
        other => panic!("the hollow ball builds: {other:?}"),
    }
}

/// Ball in a cavity, three scales and a rotated pose; point samples on
/// the contact sphere (In for ∪), and results reused.
#[test]
fn claim1_ball_in_cavity_scales_and_points() {
    use SolidContainment::{In, OnBoundary, Out};
    let mut log = Log::default();
    let v = |r: f64| 4.0 / 3.0 * PI * r * r * r;
    for s in [1e-3, 1.0, 1e3] {
        for (r, outer) in [(0.35, 0.9), (1.7, 1.9)] {
            let (r, outer) = (r * s, outer * s);
            for (pose_name, pose) in poses(s) {
                let tp = |p: Point3<f64>| pose.transform_point(p);
                let tag_skip = format!("c1ball s{s:e} r{r:e} {pose_name}");
                let (Some(h), Some(b)) = (try_placed(&hollow(r, outer), &pose), try_placed(&ball(r), &pose)) else {
                    println!("SKIP fixture pose refused: {tag_skip}");
                    continue;
                };
                let (hb, bh) = (rest_decls(&h, r, &b, r, true), rest_decls(&b, r, &h, r, true));
                let tag0 = format!("c1ball s{s:e} r{r:e} {pose_name}");
                let s3 = s * s * s;
                let d = Vec3::new(0.3, -0.5, 0.81).normalize();
                let at = |rad: f64| Point3::origin() + d * rad;
                let (p_ball, p_shell, p_contact, p_out) =
                    (at(0.5 * r), at(0.5 * (r + outer)), at(r), at(2.0 * outer));
                for (ord, x, y, dd) in [("h∪b", &h, &b, &hb), ("b∪h", &b, &h, &bh)] {
                    let rr = run(BooleanOp::Union, x, y, dd);
                    log.check(
                        &format!("{tag0} {ord}"),
                        &rr,
                        Some(v(outer)),
                        Some(1),
                        &[(tp(p_ball), In), (tp(p_shell), In), (tp(p_contact), In), (tp(p_out), Out)],
                        s3,
                    );
                    if let Ok(BooleanResult::Body(bb)) = &rr {
                        // Reuse: (h ∪ b) ∖ b re-hollows it — a real cut, the
                        // union's material straddles the sphere; undeclared.
                        let r2 = topo::subtract(&bb.body, &b, Tol::witness());
                        log.check(
                            &format!("{tag0} ({ord})∖b"),
                            &r2,
                            Some(v(outer) - v(r)),
                            Some(2),
                            &[(tp(p_ball), Out), (tp(p_shell), In)],
                            s3,
                        );
                    }
                    let rr = run(BooleanOp::Intersect, x, y, dd);
                    log.check(&format!("{tag0} {ord} ∩"), &rr, None, None, &[], s3);
                }
                let rr = run(BooleanOp::Subtract, &h, &b, &hb);
                log.check(
                    &format!("{tag0} h∖b"),
                    &rr,
                    Some(v(outer) - v(r)),
                    Some(2),
                    &[(tp(p_ball), Out), (tp(p_shell), In), (tp(p_contact), OnBoundary)],
                    s3,
                );
                let rr = run(BooleanOp::Subtract, &b, &h, &bh);
                log.check(
                    &format!("{tag0} b∖h"),
                    &rr,
                    Some(v(r)),
                    Some(1),
                    &[(tp(p_ball), In), (tp(p_shell), Out), (tp(p_contact), OnBoundary)],
                    s3,
                );
            }
        }
    }
    report("claim1ball", &log, true);
}

/// Claim 3 corpus: cases off the declared-Rest path whose verdicts must
/// not move. Shared-recipe-source pairs (rung 1, undeclared), a second
/// shell nested in the collar wall, a continuation, and undeclared
/// equal-radius pairs. Prints verdicts only (diffed main vs head) plus
/// closed-form checks where the answer is known.
#[test]
fn claim3_differential_corpus() {
    let mut log = Log::default();
    let tol = Tol::witness();
    // (a) shared recipe source: brick ∖ cyl and brick ∩ cyl, then every op.
    let brick = sweep::test_support::brick::<f64>((-1.0, 1.0), (-1.0, 1.0), (0.0, 1.0), tol);
    let cyl = extruded(
        sketch_at(-0.5),
        vec![three_arc(Point2::new(0.1, 0.0), 0.4, 13.0)],
        2.0,
        tol,
    );
    let (outside, inside) = (
        topo::subtract(&brick, &cyl, tol),
        topo::intersect(&brick, &cyl, tol),
    );
    println!("PROBE c3 brick∖cyl => {}", summary(&outside));
    println!("PROBE c3 brick∩cyl => {}", summary(&inside));
    if let (Ok(BooleanResult::Body(o)), Ok(BooleanResult::Body(i))) = (&outside, &inside) {
        let (vo, vi) = (4.0 - PI * 0.16, PI * 0.16);
        let none = BooleanDeclarations::none();
        for (name, op, x, y, want) in [
            ("out∩in", BooleanOp::Intersect, &o.body, &i.body, None),
            ("in∩out", BooleanOp::Intersect, &i.body, &o.body, None),
            ("out∖in", BooleanOp::Subtract, &o.body, &i.body, Some(vo)),
            ("in∖out", BooleanOp::Subtract, &i.body, &o.body, Some(vi)),
            ("out∪in", BooleanOp::Union, &o.body, &i.body, Some(4.0)),
        ] {
            let r = run(op, x, y, &none);
            log.check(&format!("c3 shared-source {name}"), &r, want, None, &[], 1.0);
        }
        // and cyl (the source itself) against the outside part.
        for (name, op, x, y) in [
            ("out∩cyl", BooleanOp::Intersect, &o.body, &cyl),
            ("out∖cyl", BooleanOp::Subtract, &o.body, &cyl),
            ("cyl∖out", BooleanOp::Subtract, &cyl, &o.body),
        ] {
            let r = run(op, x, y, &none);
            println!("PROBE c3 shared-source {name} => {}", summary(&r));
        }
    }
    // (b) a collar with a declared shaft, plus a second B shell (a small
    // ball) buried inside the collar wall: ∩ must be that ball.
    let c = Cell {
        layout: Layout::ArcSplit,
        rb: 0.5,
        rs: 0.5,
        wall: 1.0,
        base: 1.0,
        len: 1.0,
        z0: 0.5,
        h: 2.0,
        deg: 0.0,
        dx: 0.0,
        tilt: 0.0,
    };
    let (cb, sb) = (c.collar(), c.shaft());
    let pebble = placed(&ball(0.2), &Affine3::translation(Vec3::new(1.0, 0.0, 1.5)));
    let two = match topo::union(&sb, &pebble, tol) {
        Ok(BooleanResult::Body(bb)) => Some(bb.body),
        other => {
            println!("PROBE c3 shaft∪pebble => {}", summary(&other));
            None
        }
    };
    if let Some(two) = two {
        let pv = 4.0 / 3.0 * PI * 0.008;
        let d = rest_decls(&cb, 0.5, &two, 0.5, false);
        let d2 = rest_decls(&two, 0.5, &cb, 0.5, false);
        let r = run(BooleanOp::Intersect, &cb, &two, &d);
        log.check("c3 pebble c∩(s+p)", &r, Some(pv), None, &[], 1.0);
        let r = run(BooleanOp::Subtract, &cb, &two, &d);
        log.check("c3 pebble c∖(s+p)", &r, Some(c.collar_vol() - pv), Some(2), &[], 1.0);
        let r = run(BooleanOp::Subtract, &two, &cb, &d2);
        log.check("c3 pebble (s+p)∖c", &r, Some(c.shaft_vol()), Some(1), &[], 1.0);
    }
    // (c) undeclared: the same mate with no declarations at all.
    let none = BooleanDeclarations::none();
    for op in [BooleanOp::Intersect, BooleanOp::Subtract, BooleanOp::Union] {
        let r = run(op, &cb, &sb, &none);
        println!("PROBE c3 undeclared mate {op:?} => {}", summary(&r));
    }
    // (d) a continuation: two coaxial equal shafts stacked end to end,
    // their walls declared Continuation (aligned) — must stay as on main.
    let s1 = Cell { z0: 0.0, h: 1.0, ..c }.shaft();
    let s2 = Cell { z0: 0.5, h: 1.0, ..c }.shaft();
    let mut dc = BooleanDeclarations::none();
    for &x in &faces_on(&s1, 0.5, false) {
        for &y in &faces_on(&s2, 0.5, false) {
            dc.coincident_faces.push(FacePairDeclaration::new(
                x,
                y,
                topo::BooleanCoincidence::Continuation,
            ));
        }
    }
    for op in [BooleanOp::Intersect, BooleanOp::Subtract, BooleanOp::Union] {
        let r = run(op, &s1, &s2, &dc);
        println!("PROBE c3 overlapping coaxial shafts, Continuation {op:?} => {}", summary(&r));
    }
    // (e) a ball in a cavity, undeclared.
    let (h, b) = (hollow(0.5, 1.0), ball(0.5));
    for op in [BooleanOp::Intersect, BooleanOp::Subtract, BooleanOp::Union] {
        let r = run(op, &h, &b, &none);
        println!("PROBE c3 undeclared ball-in-cavity {op:?} => {}", summary(&r));
    }
    report("claim3", &log, false);
}

/// Claim 4 (mutant M7, the exemption keyed on the A face alone): the
/// bore that rests on a declared shaft also touches an UNDECLARED rod of
/// the same operand along a line (internally tangent). The bore × rod
/// pair is not settled, so it must still be asked its own question.
#[test]
fn claim4_rest_plus_undeclared_tangent_rod() {
    let tol = Tol::witness();
    let mut log = Log::default();
    let c = Cell {
        layout: Layout::ArcSplit,
        rb: 0.5,
        rs: 0.5,
        wall: 1.0,
        base: 1.0,
        len: 1.0,
        z0: 0.5,
        h: 1.0,
        deg: 0.0,
        dx: 0.0,
        tilt: 0.0,
    };
    for (rod_r, gap) in [(0.1, 0.0), (0.1, 1e-10), (0.1, 3e-9), (0.1, 1e-8), (0.1, 1e-6), (0.1, 0.05)] {
        let (cb, sb) = (c.collar(), c.shaft());
        let rod = extruded(
            sketch_at(1.6),
            vec![three_arc(Point2::new(0.5 - rod_r - gap, 0.0), rod_r, 0.0)],
            0.3,
            tol,
        );
        let two = match topo::union(&sb, &rod, tol) {
            Ok(BooleanResult::Body(bb)) => bb.body,
            other => panic!("shaft ∪ rod: {}", summary(&other)),
        };
        let rod_v = PI * rod_r * rod_r * 0.3;
        let tag0 = format!("c4 rod {rod_r} gap {gap}");
        let d = rest_decls(&cb, 0.5, &two, 0.5, false);
        let d2 = rest_decls(&two, 0.5, &cb, 0.5, false);
        let r = run(BooleanOp::Intersect, &cb, &two, &d);
        log.check(&format!("{tag0} c∩B"), &r, None, None, &[], 1.0);
        let r = run(BooleanOp::Subtract, &cb, &two, &d);
        log.check(&format!("{tag0} c∖B"), &r, Some(c.collar_vol()), Some(1), &[], 1.0);
        let r = run(BooleanOp::Subtract, &two, &cb, &d2);
        log.check(&format!("{tag0} B∖c"), &r, Some(c.shaft_vol() + rod_v), Some(2), &[], 1.0);
    }
    report("claim4", &log, false);
}

/// Claim 4 (mutants M4/M6 on the sphere arm's "every reaching face"
/// qualifier): a ball in a cavity with only SOME of its sphere pairs
/// declared. The undeclared opposed pairs are not settled, so the scan
/// must still ask the sphere pair's question for them.
#[test]
fn claim4_ball_in_cavity_partly_declared() {
    let tol = Tol::witness();
    let mut log = Log::default();
    let (h, b) = (hollow(0.5, 1.0), ball(0.5));
    let (fh, fb) = (faces_on(&h, 0.5, true), faces_on(&b, 0.5, true));
    println!("PROBE c4p faces: hollow {} ball {}", fh.len(), fb.len());
    let v = |r: f64| 4.0 / 3.0 * PI * r * r * r;
    let all: Vec<(usize, usize)> =
        (0..fh.len()).flat_map(|i| (0..fb.len()).map(move |j| (i, j))).collect();
    for k in 0..all.len() {
        // every pair but the k-th
        let mut d = BooleanDeclarations::none();
        for (n, &(i, j)) in all.iter().enumerate() {
            if n != k {
                d.coincident_faces
                    .push(FacePairDeclaration::new(fh[i], fb[j], ContactClass::Rest));
            }
        }
        let tag0 = format!("c4p all-but-{k}");
        let r = run(BooleanOp::Intersect, &h, &b, &d);
        log.check(&format!("{tag0} h∩b"), &r, None, None, &[], 1.0);
        let r = run(BooleanOp::Subtract, &h, &b, &d);
        log.check(&format!("{tag0} h∖b"), &r, Some(v(1.0) - v(0.5)), Some(2), &[], 1.0);
        let r = run(BooleanOp::Union, &h, &b, &d);
        log.check(&format!("{tag0} h∪b"), &r, Some(v(1.0)), Some(1), &[], 1.0);
    }
    let _ = tol;
    report("claim4p", &log, false);
}
