//! Reviewer probes for PR #3980 (lane reach-dual3980-r2). Every oracle
//! is a closed form computed here, never the kernel. Each test prints a
//! table and panics at the end with every disagreement it saw.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::approx::band;
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

#[derive(Clone, Copy, Debug, PartialEq)]
enum Layout {
    ArcSplit,
    FullTurn,
}

/// A shaft in a bore at scale `s`. `rb` the bore radius, `rs` the
/// shaft's, `off` the shaft's offset off the axis, `tilt` its tilt (rad)
/// about an axis normal to the bore's through the bore's middle.
#[derive(Clone, Copy, Debug)]
struct Fx {
    layout: Layout,
    s: f64,
    rb: f64,
    rs: f64,
    len: f64,
    span: (f64, f64),
    deg: f64,
    off: f64,
    tilt: f64,
}

impl Fx {
    fn wall(self) -> f64 {
        self.s
    }
    fn base(self) -> f64 {
        self.s
    }
    fn collar_volume(self) -> f64 {
        let o = self.rb + self.wall();
        PI * (o * o - self.rb * self.rb) * self.len
    }
    fn shaft_volume(self) -> f64 {
        PI * self.rs * self.rs * self.span.1
    }
    /// The length of the shaft inside the collar's slab.
    fn overlap(self) -> f64 {
        let (z0, h) = self.span;
        let lo = z0.max(self.base());
        let hi = (z0 + h).min(self.base() + self.len);
        (hi - lo).max(0.0)
    }
    /// The true ∩ volume when the shaft is only offset or re-radiused
    /// (no tilt): the shaft's disk outside the bore's disk, times the
    /// overlap, provided the shaft stays inside the collar's outer wall.
    fn true_intersection(self) -> Option<f64> {
        if self.tilt != 0.0 {
            return None;
        }
        let (r1, r2, d) = (self.rs, self.rb, self.off);
        // area of the shaft disk minus the lens it shares with the bore disk
        let lens = if d >= r1 + r2 {
            0.0
        } else if d <= (r1 - r2).abs() {
            PI * r1.min(r2).powi(2)
        } else {
            let a1 = ((d * d + r1 * r1 - r2 * r2) / (2.0 * d * r1)).acos();
            let a2 = ((d * d + r2 * r2 - r1 * r1) / (2.0 * d * r2)).acos();
            r1 * r1 * a1 + r2 * r2 * a2
                - 0.5 * ((-d + r1 + r2) * (d + r1 - r2) * (d - r1 + r2) * (d + r1 + r2)).sqrt()
        };
        Some((PI * r1 * r1 - lens).max(0.0) * self.overlap())
    }

    fn collar(self) -> Body<f64> {
        let (r, outer) = (self.rb, self.rb + self.wall());
        match self.layout {
            Layout::ArcSplit => extruded(
                sketch_at(self.base()),
                vec![
                    three_arc(Point2::new(0.0, 0.0), outer, 0.0),
                    three_arc(Point2::new(0.0, 0.0), r, 0.0),
                ],
                self.len,
                Tol::witness(),
            ),
            Layout::FullTurn => {
                let (y0, y1) = (self.base(), self.base() + self.len);
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
                revolve(&vp, axis, Revolution::Full, Tol::witness())
                    .unwrap()
                    .body
            }
        }
    }

    fn shaft(self) -> Body<f64> {
        let (z0, h) = self.span;
        let peg = extruded(
            sketch_at(z0),
            vec![three_arc(Point2::new(self.off, 0.0), self.rs, self.deg)],
            h,
            Tol::witness(),
        );
        let mid = self.base() + 0.5 * self.len;
        let peg = if self.tilt == 0.0 {
            peg
        } else {
            let t = Affine3::rotation_about_axis(
                Point3::new(0.0, 0.0, mid),
                Vec3::new(0.0, 1.0, 0.0),
                self.tilt,
            );
            topo::transform_rigid(&peg, &t, Tol::witness()).unwrap()
        };
        match self.layout {
            Layout::ArcSplit => peg,
            Layout::FullTurn => {
                let up = Affine3::rotation_about_axis(
                    Point3::new(0.0, 0.0, 0.0),
                    Vec3::new(1.0, 0.0, 0.0),
                    -FRAC_PI_2,
                );
                topo::transform_rigid(&peg, &up, Tol::witness()).unwrap()
            }
        }
    }

    /// Probe points in the frame before the pose, with the closed-form
    /// membership in collar and shaft: (point, in collar, in shaft).
    fn samples(self) -> Vec<(Point3<f64>, bool, bool)> {
        let (z0, h) = self.span;
        let mut out = Vec::new();
        let mid_c = self.base() + 0.5 * self.len;
        let lo = z0.max(self.base());
        let hi = (z0 + h).min(self.base() + self.len);
        let mid_o = 0.5 * (lo + hi);
        let w = self.rb + 0.5 * self.wall();
        for k in 0..6 {
            let th = 0.37 + k as f64 * 1.047;
            out.push((Point3::new(w * th.cos(), w * th.sin(), mid_c), true, false));
            // inside the shaft, inside the bore
            let q = 0.6 * self.rs;
            out.push((Point3::new(q * th.cos(), q * th.sin(), mid_o), false, true));
        }
        // the shaft outside the collar's slab, if any
        if z0 < self.base() - 1e-9 * self.s {
            out.push((Point3::new(0.0, 0.0, 0.5 * (z0 + self.base())), false, true));
        }
        // outside both
        out.push((Point3::new(0.0, 0.0, self.base() + self.len + 3.0 * self.s), false, false));
        out.push((Point3::new(self.rb + 2.0 * self.wall(), 0.0, mid_c), false, false));
        out
            .into_iter()
            .map(|(p, a, b)| {
                let p = match self.layout {
                    Layout::ArcSplit => p,
                    // the -90° turn about x takes z to y: (x, y, z) -> (x, z, -y)
                    Layout::FullTurn => Point3::new(p.x, p.z, -p.y),
                };
                (p, a, b)
            })
            .collect()
    }
}

fn cyl_faces(body: &Body<f64>, r: f64, rel: f64) -> Vec<topo::FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Cylinder { radius, .. }) if (radius - r).abs() <= rel * r
            )
        })
        .map(|(k, _)| k)
        .collect()
}

/// Every bore wall x shaft wall declared Rest (radius ra on `a`, rb on
/// `b`), plus the continuations.
fn decls(a: &Body<f64>, ra: f64, b: &Body<f64>, rb: f64) -> BooleanDeclarations {
    let mut d = match topo::flush::find_flush_candidates(a, b, Tol::witness()) {
        Ok(found) => {
            let picked: Vec<_> = found
                .into_iter()
                .filter(|f| f.class == topo::BooleanCoincidence::Continuation)
                .collect();
            topo::flush::declare_all(&picked)
        }
        Err(_) => BooleanDeclarations::none(),
    };
    for &fa in &cyl_faces(a, ra, 1e-9) {
        for &fb in &cyl_faces(b, rb, 1e-9) {
            d.coincident_faces
                .push(FacePairDeclaration::new(fa, fb, ContactClass::Rest));
        }
    }
    d
}

fn volume(b: &Body<f64>) -> f64 {
    mass_properties(b, Tol::witness()).unwrap().volume
}

fn pose(s: f64) -> Affine3<f64> {
    let r = Affine3::rotation_about_axis(
        Point3::new(0.3 * s, -0.7 * s, 0.2 * s),
        Vec3::new(-2.0, 0.5, 1.5).normalize(),
        2.3,
    );
    Affine3::translation(Vec3::new(4.0 * s, -3.0 * s, 7.0 * s)) * r
}

fn placed(b: &Body<f64>, p: &Affine3<f64>) -> Body<f64> {
    topo::transform_rigid(b, p, Tol::witness()).unwrap()
}

enum Ans {
    Empty,
    Body(f64, usize, Body<f64>, Vec<String>),
    Refused(String),
}

fn run(op: BooleanOp, a: &Body<f64>, b: &Body<f64>, d: &BooleanDeclarations) -> Ans {
    let tol = Tol::witness();
    match topo::boolean_op_with(op, a, b, d, topo::SweepStrategy::Realized, tol) {
        Ok(BooleanResult::Empty) => Ans::Empty,
        Ok(BooleanResult::Body(bb)) => {
            let mut notes = Vec::new();
            if topo::validate_geometric(&bb.body, tol).is_err() {
                notes.push("tier3 FAILS".into());
            }
            if topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol).is_err() {
                notes.push("census FAILS".into());
            }
            Ans::Body(volume(&bb.body), bb.body.shells().count(), bb.body, notes)
        }
        Err(e) => {
            let s = format!("{e:?}");
            Ans::Refused(s.chars().take(90).collect())
        }
    }
}

fn pis(b: &Body<f64>, q: Point3<f64>) -> String {
    match point_in_solid(b, q, band(), Tol::witness()) {
        Ok(SolidContainment::In) => "In".into(),
        Ok(SolidContainment::Out) => "Out".into(),
        Ok(SolidContainment::OnBoundary) => "On".into(),
        Err(e) => format!("err {e:?}").chars().take(40).collect(),
    }
}

/// Checks one op's answer against `want` (None = empty) and the samples
/// against `member`; returns a failure line or None.
fn judge(
    tag: &str,
    ans: &Ans,
    want: Option<f64>,
    scale: f64,
    samples: &[(Point3<f64>, bool)],
    allow_refusal: bool,
) -> Option<String> {
    match (ans, want) {
        (Ans::Empty, None) => None,
        (Ans::Empty, Some(w)) if w <= 1e-9 * scale => None,
        (Ans::Empty, Some(w)) => Some(format!("{tag}: EMPTY, true volume {w:e}")),
        (Ans::Refused(e), _) => {
            if allow_refusal {
                None
            } else {
                Some(format!("{tag}: refused {e}"))
            }
        }
        (Ans::Body(v, _, body, notes), w) => {
            let w = w.unwrap_or(0.0);
            let mut bad = Vec::new();
            if (v - w).abs() > 1e-9 * scale {
                bad.push(format!("volume {v:.15e} vs true {w:.15e} (rel {:e})", (v - w).abs() / scale));
            }
            bad.extend(notes.iter().cloned());
            for (q, inside) in samples {
                let got = pis(body, *q);
                let ok = if *inside { got == "In" } else { got == "Out" };
                if !ok {
                    bad.push(format!("pis({:.4},{:.4},{:.4})={got} want {}", q.x, q.y, q.z, if *inside { "In" } else { "Out" }));
                }
            }
            if bad.is_empty() {
                None
            } else {
                Some(format!("{tag}: {}", bad.join("; ")))
            }
        }
    }
}

fn describe(a: &Ans) -> String {
    match a {
        Ans::Empty => "Empty".into(),
        Ans::Body(v, sh, _, n) => format!("Body vol {v:.12e} shells {sh} {}", n.join(",")),
        Ans::Refused(e) => format!("Refused {e}"),
    }
}

/// Every op in both orders for a fixture; `exact` = the declaration is
/// true, so every op must build. Returns failures.
fn every_op(fx: Fx, posed: bool, exact: bool, tag: &str) -> Vec<String> {
    let p = if posed { pose(fx.s) } else { Affine3::translation(Vec3::new(0.0, 0.0, 0.0)) };
    let (c, s) = (placed(&fx.collar(), &p), placed(&fx.shaft(), &p));
    let (cs, sc) = (decls(&c, fx.rb, &s, fx.rs), decls(&s, fx.rs, &c, fx.rb));
    let scale = fx.collar_volume().max(fx.shaft_volume());
    let tint = fx.true_intersection();
    let samp: Vec<(Point3<f64>, bool, bool)> = fx
        .samples()
        .into_iter()
        .map(|(q, a, b)| (p.transform_point(q), a, b))
        .collect();
    let sel = |f: fn(bool, bool) -> bool| -> Vec<(Point3<f64>, bool)> {
        if fx.off != 0.0 || fx.tilt != 0.0 || fx.rs != fx.rb {
            // the closed-form membership no longer holds near the wall; only far samples
            samp.iter()
                .filter(|(q, _, _)| {
                    let _ = q;
                    true
                })
                .map(|&(q, a, b)| (q, f(a, b)))
                .filter(|_| false)
                .collect()
        } else {
            samp.iter().map(|&(q, a, b)| (q, f(a, b))).collect()
        }
    };
    let mut fails = Vec::new();
    let mut line = format!("{tag}:");

    for (name, op, x, y, d, want, mem) in [
        ("c∩s", BooleanOp::Intersect, &c, &s, &cs, tint.map(Some).unwrap_or(None), sel(|a, b| a && b)),
        ("s∩c", BooleanOp::Intersect, &s, &c, &sc, tint.map(Some).unwrap_or(None), sel(|a, b| a && b)),
        ("c∖s", BooleanOp::Subtract, &c, &s, &cs, tint.map(|t| fx.collar_volume() - t), sel(|a, b| a && !b)),
        ("s∖c", BooleanOp::Subtract, &s, &c, &sc, tint.map(|t| fx.shaft_volume() - t), sel(|a, b| b && !a)),
        ("c∪s", BooleanOp::Union, &c, &s, &cs, tint.map(|t| fx.collar_volume() + fx.shaft_volume() - t), sel(|a, b| a || b)),
        ("s∪c", BooleanOp::Union, &s, &c, &sc, tint.map(|t| fx.collar_volume() + fx.shaft_volume() - t), sel(|a, b| a || b)),
    ] {
        let ans = run(op, x, y, d);
        line.push_str(&format!(" [{name} {}]", describe(&ans)));
        if exact {
            // the declaration is true: the closed form
            let w = match name {
                "c∩s" | "s∩c" => None,
                "c∖s" => Some(fx.collar_volume()),
                "s∖c" => Some(fx.shaft_volume()),
                _ => Some(fx.collar_volume() + fx.shaft_volume()),
            };
            // union of a blind shaft in a full-turn bore is a known refusal
            let known = name.contains('∪') && fx.layout == Layout::FullTurn && tag.contains("blind");
            if let Some(f) = judge(&format!("{tag} {name}"), &ans, w, scale, &mem, known) {
                fails.push(f);
            }
        } else if let Some(want) = want {
            let want_opt = if name.contains('∩') { Some(want) } else { Some(want) };
            if let Some(f) = judge(&format!("{tag} {name}"), &ans, want_opt, scale, &[], true) {
                fails.push(f);
            }
        } else if let Ans::Empty = ans {
            if name.contains('∩') {
                fails.push(format!("{tag} {name}: EMPTY on an interfering (tilted) shaft"));
            }
        } else if let Ans::Body(v, ..) = ans {
            let closed = match name {
                "c∖s" => Some(fx.collar_volume()),
                "s∖c" => Some(fx.shaft_volume()),
                _ => None,
            };
            if let Some(cv) = closed {
                if (v - cv).abs() <= 1e-12 * scale {
                    fails.push(format!("{tag} {name}: the CLOSED FORM {v:e} on an interfering (tilted) shaft"));
                }
            }
        }
    }
    println!("{line}");
    fails
}

fn spans(base: f64, len: f64) -> Vec<(&'static str, (f64, f64))> {
    vec![
        ("through", (base - 0.5 * len, 2.0 * len)),
        ("flush", (base, len)),
        ("proud below", (base - 0.3 * len, 0.8 * len)),
        ("inside", (base + 0.25 * len, 0.5 * len)),
        ("blind", (base + 0.4 * len, len)),
    ]
}

#[test]
fn r2_widened_matrix_every_op_at_three_scales() {
    let mut fails = Vec::new();
    for s in [1e-3, 1.0, 1e3] {
        for (r, len) in [(0.05, 0.4), (2.7, 1.9)] {
            for layout in [Layout::ArcSplit, Layout::FullTurn] {
                for deg in [0.0, 37.0] {
                    for (sn, (z0, h)) in spans(s, len * s) {
                        for posed in [false, true] {
                            let fx = Fx {
                                layout,
                                s,
                                rb: r * s,
                                rs: r * s,
                                len: len * s,
                                span: (z0, h),
                                deg,
                                off: 0.0,
                                tilt: 0.0,
                            };
                            let tag = format!("{layout:?} s{s} r{r} L{len} {sn} az{deg} posed{posed}");
                            fails.extend(every_op(fx, posed, true, &tag));
                        }
                    }
                }
            }
        }
    }
    for f in &fails {
        println!("FAIL {f}");
    }
    assert!(fails.is_empty(), "{} failures", fails.len());
}

#[test]
fn r2_lying_declarations() {
    let mut fails = Vec::new();
    for s in [1e-3, 1.0, 1e3] {
        for layout in [Layout::ArcSplit, Layout::FullTurn] {
            let base = Fx {
                layout,
                s,
                rb: 0.5 * s,
                rs: 0.5 * s,
                len: s,
                span: (0.5 * s, 2.0 * s),
                deg: 0.0,
                off: 0.0,
                tilt: 0.0,
            };
            let mut cases: Vec<(String, Fx)> = Vec::new();
            for d in [1e-2, 1e-4, 1e-6, 1e-8, 1e-10] {
                cases.push((format!("off {d:e}"), Fx { off: d * s, ..base }));
                cases.push((format!("radius +{d:e}"), Fx { rs: (0.5 + d) * s, ..base }));
                cases.push((format!("radius -{d:e}"), Fx { rs: (0.5 - d) * s, ..base }));
            }
            for t in [1e-2, 1e-4, 1e-6, 1e-8] {
                cases.push((format!("tilt {t:e}"), Fx { tilt: t, ..base }));
            }
            // a far shaft, declared Rest against the bore anyway
            cases.push(("far".into(), Fx { off: 5.0 * s, ..base }));
            for (name, fx) in cases {
                for posed in [false, true] {
                    let tag = format!("{layout:?} s{s} {name} posed{posed}");
                    fails.extend(every_op(fx, posed, false, &tag));
                }
            }
        }
    }
    for f in &fails {
        println!("FAIL {f}");
    }
    assert!(fails.is_empty(), "{} failures", fails.len());
}

/// Results reused as operands.
#[test]
fn r2_results_reused() {
    let mut fails = Vec::new();
    for layout in [Layout::ArcSplit, Layout::FullTurn] {
        for s in [1e-3, 1.0, 1e3] {
            let fx = Fx {
                layout,
                s,
                rb: 0.4 * s,
                rs: 0.4 * s,
                len: 1.3 * s,
                span: (0.6 * s, 2.0 * s),
                deg: 0.0,
                off: 0.0,
                tilt: 0.0,
            };
            let p = pose(s);
            let (c, sh) = (placed(&fx.collar(), &p), placed(&fx.shaft(), &p));
            let scale = fx.collar_volume();
            let tag = format!("{layout:?} s{s}");
            let Ans::Body(_, _, cms, _) = run(BooleanOp::Subtract, &c, &sh, &decls(&c, fx.rb, &sh, fx.rs)) else {
                fails.push(format!("{tag}: c∖s did not build"));
                continue;
            };
            // (c ∖ s) ∖ s again, ∩ s, ∪ s
            let d = decls(&cms, fx.rb, &sh, fx.rs);
            for (n, op, want) in [
                ("(c∖s)∖s", BooleanOp::Subtract, Some(fx.collar_volume())),
                ("(c∖s)∩s", BooleanOp::Intersect, None),
                ("(c∖s)∪s", BooleanOp::Union, Some(fx.collar_volume() + fx.shaft_volume())),
            ] {
                let a = run(op, &cms, &sh, &d);
                println!("{tag} {n}: {}", describe(&a));
                if let Some(f) = judge(&format!("{tag} {n}"), &a, want, scale, &[], false) {
                    fails.push(f);
                }
            }
            // s ∖ c then ∩ c again and ∪ c
            let Ans::Body(_, _, smc, _) = run(BooleanOp::Subtract, &sh, &c, &decls(&sh, fx.rs, &c, fx.rb)) else {
                fails.push(format!("{tag}: s∖c did not build"));
                continue;
            };
            let d = decls(&smc, fx.rs, &c, fx.rb);
            for (n, op, want) in [
                ("(s∖c)∩c", BooleanOp::Intersect, None),
                ("(s∖c)∖c", BooleanOp::Subtract, Some(fx.shaft_volume())),
            ] {
                let a = run(op, &smc, &c, &d);
                println!("{tag} {n}: {}", describe(&a));
                if let Some(f) = judge(&format!("{tag} {n}"), &a, want, scale, &[], false) {
                    fails.push(f);
                }
            }
            // a second shaft into the c ∖ s result's bore from the other order
            let d = decls(&sh, fx.rs, &cms, fx.rb);
            let a = run(BooleanOp::Subtract, &sh, &cms, &d);
            println!("{tag} s∖(c∖s): {}", describe(&a));
            if let Some(f) = judge(&format!("{tag} s∖(c∖s)"), &a, Some(fx.shaft_volume()), scale, &[], false) {
                fails.push(f);
            }
        }
    }
    for f in &fails {
        println!("FAIL {f}");
    }
    assert!(fails.is_empty(), "{} failures", fails.len());
}

fn ball(r: f64) -> Body<f64> {
    sweep::test_support::revolved_about_y(
        vec![(Point2::new(0.0, -r), 1.0), (Point2::new(0.0, r), 0.0)],
        Revolution::Full,
        Tol::witness(),
    )
}

fn sphere_faces(body: &Body<f64>, r: f64) -> Vec<topo::FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Sphere { radius, .. }) if (radius - r).abs() <= 1e-9 * r
            )
        })
        .map(|(k, _)| k)
        .collect()
}

/// A ball in a cavity whose ball is off-centre or re-radiused, declared Rest.
#[test]
fn r2_ball_in_cavity_lying_and_scaled() {
    let mut fails = Vec::new();
    let tol = Tol::witness();
    let bv = |r: f64| 4.0 / 3.0 * PI * r * r * r;
    for s in [1e-3, 1.0, 1e3] {
        let (r, outer) = (0.6 * s, 1.4 * s);
        let hollow = match topo::subtract(&ball(outer), &ball(r), tol) {
            Ok(BooleanResult::Body(bb)) => bb.body,
            other => panic!("hollow: {:?}", other.err()),
        };
        // (name, ball radius, offset, true ∩ volume if known)
        let mut cases: Vec<(String, f64, f64, Option<f64>)> = vec![("exact".into(), r, 0.0, Some(0.0))];
        for d in [1e-2, 1e-5, 1e-8, 1e-10] {
            // radius r(1+d) ball: true ∩ = shell volume
            cases.push((format!("radius +{d:e}"), r * (1.0 + d), 0.0, Some(bv(r * (1.0 + d)) - bv(r))));
            cases.push((format!("radius -{d:e}"), r * (1.0 - d), 0.0, Some(0.0)));
            // offset by d·r: true ∩ = ball minus lens of two equal spheres at distance e
            let e = d * r;
            let lens = PI / 12.0 * (4.0 * r + e) * (2.0 * r - e).powi(2);
            cases.push((format!("off {d:e}"), r, e, Some(bv(r) - lens)));
        }
        for (name, rr, e, tint) in cases {
            let p = pose(s);
            let h = placed(&hollow, &p);
            let b0 = ball(rr);
            let b0 = topo::transform_rigid(&b0, &Affine3::translation(Vec3::new(e, 0.0, 0.0)), tol).unwrap();
            let b = placed(&b0, &p);
            let mut hb = BooleanDeclarations::none();
            let mut bh = BooleanDeclarations::none();
            for &fh in &sphere_faces(&h, r) {
                for &fb in &sphere_faces(&b, rr) {
                    hb.coincident_faces.push(FacePairDeclaration::new(fh, fb, ContactClass::Rest));
                    bh.coincident_faces.push(FacePairDeclaration::new(fb, fh, ContactClass::Rest));
                }
            }
            let scale = bv(outer);
            let t = tint.unwrap();
            let hv = bv(outer) - bv(r);
            let tag = format!("ball s{s} {name}");
            let mut line = format!("{tag}:");
            for (n, op, x, y, d, want) in [
                ("h∩b", BooleanOp::Intersect, &h, &b, &hb, t),
                ("b∩h", BooleanOp::Intersect, &b, &h, &bh, t),
                ("h∖b", BooleanOp::Subtract, &h, &b, &hb, hv - t),
                ("b∖h", BooleanOp::Subtract, &b, &h, &bh, bv(rr) - t),
                ("h∪b", BooleanOp::Union, &h, &b, &hb, hv + bv(rr) - t),
                ("b∪h", BooleanOp::Union, &b, &h, &bh, hv + bv(rr) - t),
            ] {
                let a = run(op, x, y, d);
                line.push_str(&format!(" [{n} {}]", describe(&a)));
                let want = if n.contains('∩') && t == 0.0 { None } else { Some(want) };
                let mut samples = Vec::new();
                if name == "exact" {
                    let inb = p.transform_point(Point3::new(0.1 * s, 0.2 * s, -0.1 * s));
                    let inh = p.transform_point(Point3::new(0.0, 1.0 * s, 0.0));
                    let out = p.transform_point(Point3::new(0.0, 3.0 * s, 0.0));
                    let (ib, ih) = match n {
                        "h∖b" => (false, true),
                        "b∖h" => (true, false),
                        "h∪b" | "b∪h" => (true, true),
                        _ => (false, false),
                    };
                    samples = vec![(inb, ib), (inh, ih), (out, false)];
                }
                if let Some(f) = judge(&format!("{tag} {n}"), &a, want, scale, &samples, name != "exact") {
                    fails.push(f);
                }
            }
            println!("{line}");
        }
    }
    for f in &fails {
        println!("FAIL {f}");
    }
    assert!(fails.is_empty(), "{} failures", fails.len());
}

/// Multi-shell operands on the no-crossings path: A = the collar plus a
/// thin rod floating inside the shaft's material (no face of the rod
/// meets the shaft); B = the shaft, optionally with a cavity of its own
/// in the part above the collar. The whole-shell vertex probe must keep
/// or drop each shell by its own side.
#[test]
fn r2_multi_shell_operands() {
    let tol = Tol::witness();
    let mut fails = Vec::new();
    for layout in [Layout::ArcSplit] {
        for (sn, span) in [("through", (0.5, 2.0)), ("flush", (1.0, 1.0)), ("blind", (1.5, 1.0))] {
            let fx = Fx {
                layout,
                s: 1.0,
                rb: 0.5,
                rs: 0.5,
                len: 1.0,
                span,
                deg: 0.0,
                off: 0.0,
                tilt: 0.0,
            };
            let p = pose(1.0);
            let collar = fx.collar();
            let shaft = fx.shaft();
            // a rod of radius 0.1 on the axis, inside the shaft's span
            let (z0, h) = span;
            let rod = extruded(
                sketch_at(z0 + 0.2 * h),
                vec![three_arc(Point2::new(0.05, 0.02), 0.1, 10.0)],
                0.6 * h,
                tol,
            );
            let rod_v = PI * 0.01 * 0.6 * h;
            let a0 = match topo::union(&collar, &rod, tol) {
                Ok(BooleanResult::Body(bb)) => bb.body,
                other => panic!("collar + rod: {:?}", other.err()),
            };
            let (a, b) = (placed(&a0, &p), placed(&shaft, &p));
            let tag = format!("multi-shell {sn}");
            let ab = decls(&a, 0.5, &b, 0.5);
            let ba = decls(&b, 0.5, &a, 0.5);
            let scale = fx.collar_volume();
            for (n, op, x, y, d, want) in [
                ("A∩B", BooleanOp::Intersect, &a, &b, &ab, Some(rod_v)),
                ("B∩A", BooleanOp::Intersect, &b, &a, &ba, Some(rod_v)),
                ("A∖B", BooleanOp::Subtract, &a, &b, &ab, Some(fx.collar_volume())),
                ("B∖A", BooleanOp::Subtract, &b, &a, &ba, Some(fx.shaft_volume() - rod_v)),
                ("A∪B", BooleanOp::Union, &a, &b, &ab, Some(fx.collar_volume() + fx.shaft_volume())),
            ] {
                let r = run(op, x, y, d);
                println!("{tag} {n}: {}", describe(&r));
                if let Some(f) = judge(&format!("{tag} {n}"), &r, want, scale, &[], false) {
                    fails.push(f);
                }
            }
            // B with a cavity (a small ball) inside its material, away from the collar
            let zc = z0 + 0.5 * h;
            let cav = topo::transform_rigid(
                &ball(0.15),
                &Affine3::translation(Vec3::new(0.1, 0.0, zc)),
                tol,
            )
            .unwrap();
            let b1 = match topo::subtract(&shaft, &cav, tol) {
                Ok(BooleanResult::Body(bb)) => bb.body,
                other => panic!("shaft - cavity: {:?}", other.err()),
            };
            let cv = 4.0 / 3.0 * PI * 0.15f64.powi(3);
            let (c, b) = (placed(&collar, &p), placed(&b1, &p));
            let cb = decls(&c, 0.5, &b, 0.5);
            let bc = decls(&b, 0.5, &c, 0.5);
            for (n, op, x, y, d, want) in [
                ("C∩B'", BooleanOp::Intersect, &c, &b, &cb, None),
                ("C∖B'", BooleanOp::Subtract, &c, &b, &cb, Some(fx.collar_volume())),
                ("B'∖C", BooleanOp::Subtract, &b, &c, &bc, Some(fx.shaft_volume() - cv)),
                ("C∪B'", BooleanOp::Union, &c, &b, &cb, Some(fx.collar_volume() + fx.shaft_volume() - cv)),
            ] {
                let r = run(op, x, y, d);
                println!("{tag} {n}: {}", describe(&r));
                if let Some(f) = judge(&format!("{tag} {n}"), &r, want, scale, &[], false) {
                    fails.push(f);
                }
            }
        }
    }
    for f in &fails {
        println!("FAIL {f}");
    }
    assert!(fails.is_empty(), "{} failures", fails.len());
}

/// The exemption is per PAIR: a bore face settled `Rest` against the
/// shaft's walls must still have its section with ANOTHER B face asked.
/// B = a blind shaft (filling the bore's lower half) plus a radial pin
/// at 60° that pierces the bore wall above the shaft and ends inside the
/// collar's material: the pin's wall meets the bore face in a closed
/// loop that no edge crosses. The true ∩ is the pin's part outside the
/// bore radius.
#[test]
fn r2_pin_beside_a_rest_mate() {
    let tol = Tol::witness();
    let (rb, rp, x0, x1, zp, az) = (0.5f64, 0.1f64, 0.2f64, 1.2f64, 1.75f64, 60f64.to_radians());
    let fx = Fx {
        layout: Layout::ArcSplit,
        s: 1.0,
        rb,
        rs: rb,
        len: 1.0,
        span: (0.5, 1.0),
        deg: 0.0,
        off: 0.0,
        tilt: 0.0,
    };
    let collar = fx.collar();
    let shaft = fx.shaft();
    let pin = extruded(
        sketch_at(x0),
        vec![three_arc(Point2::new(0.0, 0.0), rp, 30.0)],
        x1 - x0,
        tol,
    );
    // z -> x, then lift to zp and turn to the azimuth
    let to_x = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 1.0, 0.0), FRAC_PI_2);
    let lift = Affine3::translation(Vec3::new(0.0, 0.0, zp));
    let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 0.0, 1.0), az);
    let pin = topo::transform_rigid(&pin, &(turn * lift * to_x), tol).unwrap();
    // the oracle: ∫ over the pin's disk of (x1 - max(x0, sqrt(rb² - y²)))
    let n = 2000;
    let mut inside = 0.0;
    for i in 0..n {
        for j in 0..n {
            let y = -rp + (i as f64 + 0.5) * 2.0 * rp / n as f64;
            let z = -rp + (j as f64 + 0.5) * 2.0 * rp / n as f64;
            if y * y + z * z <= rp * rp {
                let xin = (rb * rb - y * y).sqrt().max(x0);
                inside += (x1 - xin) * (2.0 * rp / n as f64).powi(2);
            }
        }
    }
    let pin_v = PI * rp * rp * (x1 - x0);
    let b = match topo::union(&shaft, &pin, tol) {
        Ok(BooleanResult::Body(bb)) => bb.body,
        other => panic!("shaft + pin: {:?}", other.err()),
    };
    let mut fails = Vec::new();
    for (pose_name, p) in [("identity", Affine3::translation(Vec3::new(0.0, 0.0, 0.0))), ("posed", pose(1.0))] {
        let (c, bb) = (placed(&collar, &p), placed(&b, &p));
        let cb = decls(&c, rb, &bb, rb);
        let bc = decls(&bb, rb, &c, rb);
        let scale = fx.collar_volume();
        for (n, op, x, y, d, want) in [
            ("C∩B", BooleanOp::Intersect, &c, &bb, &cb, inside),
            ("B∩C", BooleanOp::Intersect, &bb, &c, &bc, inside),
            ("C∖B", BooleanOp::Subtract, &c, &bb, &cb, fx.collar_volume() - inside),
            ("B∖C", BooleanOp::Subtract, &bb, &c, &bc, fx.shaft_volume() + pin_v - inside),
            ("C∪B", BooleanOp::Union, &c, &bb, &cb, fx.collar_volume() + fx.shaft_volume() + pin_v - inside),
        ] {
            let r = run(op, x, y, d);
            println!("pin {pose_name} {n} (true {want:.9e}): {}", describe(&r));
            // a refusal is allowed; a wrong body is not (oracle quadrature ~1e-6)
            if let Ans::Body(v, ..) = &r {
                if (v - want).abs() > 1e-5 * scale {
                    fails.push(format!("pin {pose_name} {n}: WRONG BODY {v:e} vs {want:e}"));
                }
            }
            if let Ans::Empty = r {
                fails.push(format!("pin {pose_name} {n}: EMPTY vs {want:e}"));
            }
        }
    }
    for f in &fails {
        println!("FAIL {f}");
    }
    assert!(fails.is_empty(), "{} failures", fails.len());
}

/// As `r2_pin_beside_a_rest_mate`, with a ball for the pin: a ball of
/// radius 0.15 centred at radial 0.45 (az 60°, z 1.75) bulges 0.1 into
/// the collar's wall. Its poles sit on z (inside the bore), its seam
/// meridian on the side toward the axis, so no edge of either operand
/// crosses a face. With `with_shaft` false, B is the ball alone.
#[test]
fn r2_ball_bulging_into_the_bore_wall_beside_a_rest_mate() {
    let tol = Tol::witness();
    let (rb, rr, rc, zc, az) = (0.5f64, 0.15f64, 0.45f64, 1.75f64, 60f64.to_radians());
    let fx = Fx {
        layout: Layout::ArcSplit,
        s: 1.0,
        rb,
        rs: rb,
        len: 1.0,
        span: (0.5, 1.0),
        deg: 0.0,
        off: 0.0,
        tilt: 0.0,
    };
    let collar = fx.collar();
    let shaft = fx.shaft();
    // ball(r) has poles on y and its seam in the plane z = 0 on the +x side
    // (assumed; the probe prints the seam position check below).
    // Rotate poles to z: -90° about x takes y to z.
    let up = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(1.0, 0.0, 0.0), FRAC_PI_2);
    let mut found = None;
    for seam_turn in [0.0f64, 90.0, 180.0, 270.0] {
        let spin = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 0.0, 1.0), seam_turn.to_radians());
        let place = Affine3::translation(Vec3::new(rc * az.cos(), rc * az.sin(), zc));
        let turn_az = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 0.0, 1.0), az);
        let t = place * turn_az * spin * up;
        let bl = topo::transform_rigid(&ball(rr), &t, tol).unwrap();
        // every edge sample's radial distance from the bore axis
        let mut max_r: f64 = 0.0;
        for (_, v) in bl.vertices() {
            let p = *bl.get_point(v.point).unwrap();
            max_r = max_r.max((p.x * p.x + p.y * p.y).sqrt());
        }
        println!("seam turn {seam_turn}: vertex max radial {max_r}");
        if found.is_none() && max_r < rb - 0.01 {
            found = Some((seam_turn, bl));
        }
    }
    let (seam_turn, bl) = found.expect("a seam orientation inside the bore");
    // oracle: the ball's part outside radius rb, by quadrature in the ball's own frame
    let n = 300;
    let h = 2.0 * rr / n as f64;
    let mut outside = 0.0;
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                let (x, y, z) = (
                    -rr + (i as f64 + 0.5) * h,
                    -rr + (j as f64 + 0.5) * h,
                    -rr + (k as f64 + 0.5) * h,
                );
                if x * x + y * y + z * z <= rr * rr {
                    let (px, py) = (rc + x, y);
                    if px * px + py * py > rb * rb {
                        outside += h * h * h;
                    }
                }
            }
        }
    }
    let bv = 4.0 / 3.0 * PI * rr.powi(3);
    println!("seam turn {seam_turn}: ball outside the bore radius, quadrature {outside:e} (± ~1e-5 rel)");
    let mut fails = Vec::new();
    for with_shaft in [false, true] {
        let b = if with_shaft {
            match topo::union(&shaft, &bl, tol) {
                Ok(BooleanResult::Body(bb)) => bb.body,
                other => panic!("shaft + ball: {:?}", other.err()),
            }
        } else {
            bl.clone()
        };
        let sv = if with_shaft { fx.shaft_volume() } else { 0.0 };
        for (pose_name, p) in [("identity", Affine3::translation(Vec3::new(0.0, 0.0, 0.0))), ("posed", pose(1.0))] {
            let (c, bb) = (placed(&collar, &p), placed(&b, &p));
            let cb = decls(&c, rb, &bb, rb);
            let bc = decls(&bb, rb, &c, rb);
            let scale = fx.collar_volume();
            for (n, op, x, y, d, want) in [
                ("C∩B", BooleanOp::Intersect, &c, &bb, &cb, outside),
                ("B∩C", BooleanOp::Intersect, &bb, &c, &bc, outside),
                ("C∖B", BooleanOp::Subtract, &c, &bb, &cb, fx.collar_volume() - outside),
                ("B∖C", BooleanOp::Subtract, &bb, &c, &bc, sv + bv - outside),
                ("C∪B", BooleanOp::Union, &c, &bb, &cb, fx.collar_volume() + sv + bv - outside),
            ] {
                let r = run(op, x, y, d);
                println!("bulge shaft={with_shaft} {pose_name} {n} (true {want:.6e}): {}", describe(&r));
                if let Ans::Body(v, ..) = &r {
                    if (v - want).abs() > 1e-4 * scale {
                        fails.push(format!("bulge shaft={with_shaft} {pose_name} {n}: WRONG BODY {v:e} vs {want:e}"));
                    }
                }
                if let Ans::Empty = r {
                    fails.push(format!("bulge shaft={with_shaft} {pose_name} {n}: EMPTY vs {want:e}"));
                }
            }
        }
    }
    for f in &fails {
        println!("FAIL {f}");
    }
    assert!(fails.is_empty(), "{} failures", fails.len());
}
