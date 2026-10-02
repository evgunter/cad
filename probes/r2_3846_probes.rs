//! Reviewer probes, PR 3846 (reach-dual3846-r2). Widened poses of the
//! mid-edge tangency, every op, both orders, against an independent
//! oracle: closed-form volumes and an analytic membership test sampled
//! through `point_in_solid`. Any built body that disagrees is a WRONG
//! row; refusals are tallied, not failed, except where a row says so.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use profile::{Open, ProfileLoop, RawLoop};
use sweep::test_support::{extruded, sketch_at};
use topo::{
    Body, BooleanCoincidence, BooleanDeclarations, BooleanError, BooleanResult,
    FacePairDeclaration, SolidContainment,
};

fn tol() -> Tol {
    Tol::witness()
}

/// A rounded rectangle footprint `[x0, x0+w] × [y0, y0+h]`, corners `r`
/// (0 = sharp), extruded over `[z0, z0+t]`. All lengths in model units
/// BEFORE the scale `s`.
#[derive(Clone, Copy, Debug)]
struct Slab {
    x0: f64,
    y0: f64,
    w: f64,
    h: f64,
    r: f64,
    z0: f64,
    t: f64,
}

impl Slab {
    fn area(&self) -> f64 {
        self.w * self.h - 4.0 * (1.0 - core::f64::consts::FRAC_PI_4) * self.r * self.r
    }
    fn vol(&self) -> f64 {
        self.area() * self.t
    }
    /// Membership with a margin `d`: `None` within `d` of the boundary.
    fn has(&self, p: [f64; 3], d: f64) -> Option<bool> {
        let [x, y, z] = p;
        let (lx, ly) = (x - self.x0, y - self.y0);
        let mut g = [
            lx,
            self.w - lx,
            ly,
            self.h - ly,
            z - self.z0,
            self.z0 + self.t - z,
        ]
        .into_iter()
        .fold(f64::INFINITY, f64::min);
        if self.r > 0.0 {
            let cx = lx.clamp(self.r, self.w - self.r);
            let cy = ly.clamp(self.r, self.h - self.r);
            if cx != lx && cy != ly {
                let dist = ((lx - cx).powi(2) + (ly - cy).powi(2)).sqrt();
                g = g.min(self.r - dist);
            }
        }
        if g.abs() <= d { None } else { Some(g > 0.0) }
    }
    fn body(&self, s: f64) -> Body<f64> {
        let t = tol();
        let (x0, y0, w, h, r) = (self.x0 * s, self.y0 * s, self.w * s, self.h * s, self.r * s);
        let outline: ProfileLoop<f64> = if self.r == 0.0 {
            ProfileLoop::polygon([
                Point2::new(x0, y0),
                Point2::new(x0 + w, y0),
                Point2::new(x0 + w, y0 + h),
                Point2::new(x0, y0 + h),
            ])
        } else {
            Open.at(Point2::new(x0 + w / 2.0, y0))
                .toward(1.0, 0.0, t)
                .unwrap()
                .fillet(r, t)
                .unwrap()
                .at(Point2::new(x0 + w, y0 + h / 2.0), t)
                .unwrap()
                .toward(0.0, 1.0, t)
                .unwrap()
                .fillet(r, t)
                .unwrap()
                .at(Point2::new(x0 + w / 2.0, y0 + h), t)
                .unwrap()
                .toward(-1.0, 0.0, t)
                .unwrap()
                .fillet(r, t)
                .unwrap()
                .at(Point2::new(x0, y0 + h / 2.0), t)
                .unwrap()
                .toward(0.0, -1.0, t)
                .unwrap()
                .fillet(r, t)
                .unwrap()
                .to(profile::Start, t)
                .unwrap()
                .into()
        };
        extruded(sketch_at(self.z0 * s), vec![outline], self.t * s, tol())
    }
}

fn findings(a: &Body<f64>, b: &Body<f64>) -> BooleanDeclarations {
    let found = topo::flush::find_flush_candidates(a, b, tol()).expect("the plates decide");
    let _ = BooleanCoincidence::REST;
    topo::flush::declare_all(&found)
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Op {
    Union,
    AmB,
    BmA,
    Meet,
}

/// The expected volume given the z-overlap volume `vi` (footprints
/// nested or z-disjoint, so `vi` is closed form).
fn expect(op: Op, va: f64, vb: f64, vi: f64) -> f64 {
    match op {
        Op::Union => va + vb - vi,
        Op::AmB => va - vi,
        Op::BmA => vb - vi,
        Op::Meet => vi,
    }
}

fn member(op: Op, a: Option<bool>, b: Option<bool>) -> Option<bool> {
    let (a, b) = (a?, b?);
    Some(match op {
        Op::Union => a || b,
        Op::AmB => a && !b,
        Op::BmA => b && !a,
        Op::Meet => a && b,
    })
}

struct Pose {
    name: String,
    p: Slab,
    q: Slab,
    /// The z-overlap volume (model units, before scale).
    vi: f64,
    s: f64,
    map: Option<Affine3<f64>>,
}

/// Runs a pose in both orders through every op; returns
/// (wrong rows, refusal rows, ok rows).
fn run(pose: &Pose, samples: usize) -> (Vec<String>, Vec<String>, usize) {
    let (mut wrong, mut refused, mut ok) = (Vec::new(), Vec::new(), 0usize);
    let mapped = |b: Body<f64>| match &pose.map {
        Some(m) => topo::transform_rigid(&b, m, tol()).expect("rigid"),
        None => b,
    };
    let (pb, qb) = (mapped(pose.p.body(pose.s)), mapped(pose.q.body(pose.s)));
    let s3 = pose.s.powi(3);
    let band = Band::linear(tol()).unwrap();
    for (order, (a, sa), (b, sb)) in [
        ("p is A", (&pb, pose.p), (&qb, pose.q)),
        ("q is A", (&qb, pose.q), (&pb, pose.p)),
    ] {
        let dab = findings(a, b);
        let dba = findings(b, a);
        for op in [Op::Union, Op::AmB, Op::BmA, Op::Meet] {
            let label = format!("{} / {order} / {op:?}", pose.name);
            let out = match op {
                Op::Union => topo::union_with(a, b, &dab, tol()),
                Op::AmB => topo::subtract_with(a, b, &dab, tol()),
                Op::BmA => topo::subtract_with(b, a, &dba, tol()),
                Op::Meet => topo::intersect_with(a, b, &dab, tol()),
            };
            let ev = expect(op, sa.vol(), sb.vol(), pose.vi) * s3;
            match out {
                Err(e) => refused.push(format!("{label}: {}", short(&e))),
                Ok(BooleanResult::Empty) => {
                    if ev.abs() > 1e-12 * s3 {
                        wrong.push(format!("{label}: Empty, expected {ev}"));
                    } else {
                        ok += 1;
                    }
                }
                Ok(BooleanResult::Body(bb)) => {
                    let v = topo::mass_properties(&bb.body, tol()).unwrap().volume;
                    let mut bad = Vec::new();
                    if (v - ev).abs() > 1e-9 * ev.abs().max(s3) {
                        bad.push(format!("volume {v} vs {ev}"));
                    }
                    if let Err(e) = topo::validate_geometric(&bb.body, tol()) {
                        bad.push(format!("tier 3 {e:?}"));
                    }
                    if let Err(e) = topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol()) {
                        bad.push(format!("tier 3' {e:?}"));
                    }
                    // Sampling: a jittered grid over the joint box.
                    let (lo, hi) = bbox(&sa, &sb);
                    let mut miss = 0;
                    let mut errs = 0;
                    let n = samples;
                    for i in 0..n {
                        let f = |k: usize, m: f64| ((i * k) as f64 * 0.618_033_988_749 + m).fract();
                        let lp = [
                            lo[0] + (hi[0] - lo[0]) * f(1, 0.1),
                            lo[1] + (hi[1] - lo[1]) * f(7, 0.3),
                            lo[2] + (hi[2] - lo[2]) * f(13, 0.7),
                        ];
                        let want = member(op, sa.has(lp, 1e-6), sb.has(lp, 1e-6));
                        let Some(want) = want else { continue };
                        let mut q = Point3::new(lp[0] * pose.s, lp[1] * pose.s, lp[2] * pose.s);
                        if let Some(m) = &pose.map {
                            q = m.transform_point(q);
                        }
                        match topo::point_in_solid(&bb.body, q, band, tol()) {
                            Ok(SolidContainment::In) if want => {}
                            Ok(SolidContainment::Out) if !want => {}
                            Ok(got) => {
                                miss += 1;
                                if miss <= 2 {
                                    bad.push(format!("pis {lp:?}: {got:?}, want in={want}"));
                                }
                            }
                            Err(_) => errs += 1,
                        }
                    }
                    if miss > 0 {
                        bad.push(format!("{miss} sample mismatches"));
                    }
                    if bad.is_empty() {
                        ok += 1;
                        if errs > 0 {
                            refused.push(format!("{label}: built OK but {errs} pis errors"));
                        }
                    } else {
                        wrong.push(format!("{label}: {}", bad.join("; ")));
                    }
                }
            }
        }
    }
    (wrong, refused, ok)
}

fn bbox(a: &Slab, b: &Slab) -> ([f64; 3], [f64; 3]) {
    let lo = [
        a.x0.min(b.x0) - 0.1,
        a.y0.min(b.y0) - 0.1,
        a.z0.min(b.z0) - 0.1,
    ];
    let hi = [
        (a.x0 + a.w).max(b.x0 + b.w) + 0.1,
        (a.y0 + a.h).max(b.y0 + b.h) + 0.1,
        (a.z0 + a.t).max(b.z0 + b.t) + 0.1,
    ];
    (lo, hi)
}

fn short(e: &BooleanError) -> String {
    let s = format!("{e:?}");
    s.split(['{', '(']).next().unwrap_or("").trim().to_string()
}

fn slab(x0: f64, w: f64, h: f64, r: f64, z0: f64, t: f64) -> Slab {
    Slab {
        x0,
        y0: 0.0,
        w,
        h,
        r,
        z0,
        t,
    }
}

fn report(rows: &[Pose], samples: usize) -> (usize, usize, usize) {
    let (mut nw, mut nr, mut nok) = (0, 0, 0);
    for p in rows {
        let (w, r, ok) = run(p, samples);
        for x in &w {
            println!("WRONG  {x}");
        }
        for x in &r {
            println!("refuse {x}");
        }
        println!("pose {}: ok {ok}, wrong {}, refused {}", p.name, w.len(), r.len());
        nw += w.len();
        nr += r.len();
        nok += ok;
    }
    println!("TOTAL ok {nok} wrong {nw} refused {nr}");
    (nw, nr, nok)
}

/// P1: sharp over rounded and rounded over sharp, radii to the limits.
#[test]
fn p1_radii_both_stack_senses() {
    let mut rows = Vec::new();
    for r in [0.01, 0.1, 0.25, 0.5, 1.0, 1.5, 1.9, 1.999] {
        rows.push(Pose {
            name: format!("sharp over rounded r={r}"),
            p: slab(0.0, 6.0, 4.0, 0.0, 1.0, 1.0),
            q: slab(0.0, 6.0, 4.0, r, 0.0, 1.0),
            vi: 0.0,
            s: 1.0,
            map: None,
        });
        rows.push(Pose {
            name: format!("rounded over sharp r={r}"),
            p: slab(0.0, 6.0, 4.0, r, 1.0, 1.0),
            q: slab(0.0, 6.0, 4.0, 0.0, 0.0, 1.0),
            vi: 0.0,
            s: 1.0,
            map: None,
        });
    }
    let (nw, _, _) = report(&rows, 300);
    assert_eq!(nw, 0);
}

/// P2: mismatched radii, thin and thick plates.
#[test]
fn p2_mismatched_radii_and_thickness() {
    let mut rows = Vec::new();
    for (ru, rl) in [(0.1, 1.9), (1.9, 0.1), (1.0, 1.5), (0.5, 0.5001), (0.3, 0.5)] {
        rows.push(Pose {
            name: format!("r {ru} over r {rl}"),
            p: slab(0.0, 6.0, 4.0, ru, 1.0, 1.0),
            q: slab(0.0, 6.0, 4.0, rl, 0.0, 1.0),
            vi: 0.0,
            s: 1.0,
            map: None,
        });
    }
    for (tu, tl) in [(0.01, 1.0), (3.0, 0.05)] {
        rows.push(Pose {
            name: format!("sharp t={tu} over rounded t={tl}"),
            p: slab(0.0, 6.0, 4.0, 0.0, tl, tu),
            q: slab(0.0, 6.0, 4.0, 0.5, 0.0, tl),
            vi: 0.0,
            s: 1.0,
            map: None,
        });
    }
    let (nw, _, _) = report(&rows, 300);
    assert_eq!(nw, 0);
}

/// P3: scales and rigid motions.
#[test]
fn p3_scales_and_rotations() {
    let mut rows = Vec::new();
    for s in [1e-3, 1e3] {
        for r in [0.25, 1.0] {
            rows.push(Pose {
                name: format!("sharp over rounded r={r} x{s}"),
                p: slab(0.0, 6.0, 4.0, 0.0, 1.0, 1.0),
                q: slab(0.0, 6.0, 4.0, r, 0.0, 1.0),
                vi: 0.0,
                s,
                map: None,
            });
        }
    }
    for (axis, angle) in [
        (Vec3::new(0.0, 0.0, 1.0), 0.3),
        (Vec3::new(1.0, 2.0, 3.0), 0.7),
        (Vec3::new(1.0, 0.0, 0.0), core::f64::consts::FRAC_PI_2),
    ] {
        rows.push(Pose {
            name: format!("sharp over rounded r=0.5 rot {axis:?} {angle}"),
            p: slab(0.0, 6.0, 4.0, 0.0, 1.0, 1.0),
            q: slab(0.0, 6.0, 4.0, 0.5, 0.0, 1.0),
            vi: 0.0,
            s: 1.0,
            map: Some(Affine3::rotation_about_axis(
                Point3::new(0.3, -0.2, 0.5),
                axis,
                angle,
            )),
        });
    }
    let (nw, _, _) = report(&rows, 200);
    assert_eq!(nw, 0);
}

/// P4: offset footprints (the touch near an end, an end inside the
/// fillet's corner square, the end exactly on the tangent vertex) and
/// the sunk (z-overlapping) stack whose touch is mid-ruling.
#[test]
fn p4_offsets_and_sunk() {
    let r = 0.5;
    let mut rows = Vec::new();
    for dx in [-3.0, -0.25, 0.25, -(r / 2.0), r - 6.0 + 0.0, 0.5 - 6.0 + 1e-3] {
        rows.push(Pose {
            name: format!("sharp dx={dx} over rounded"),
            p: slab(dx, 6.0, 4.0, 0.0, 1.0, 1.0),
            q: slab(0.0, 6.0, 4.0, r, 0.0, 1.0),
            vi: 0.0,
            s: 1.0,
            map: None,
        });
    }
    // Sunk: the sharp plate z ∈ [0.5, 1.5] over the rounded z ∈ [0, 1];
    // the rounded footprint is inside the sharp one, overlap 0.5 thick.
    let q = slab(0.0, 6.0, 4.0, r, 0.0, 1.0);
    rows.push(Pose {
        name: "sunk sharp over rounded".into(),
        p: slab(0.0, 6.0, 4.0, 0.0, 0.5, 1.0),
        q,
        vi: q.area() * 0.5,
        s: 1.0,
        map: None,
    });
    let (nw, _, _) = report(&rows, 300);
    assert_eq!(nw, 0);
}

/// P5: results reused as operands: (sharp ∪ rounded) then a rounded
/// plate on top of the sharp one, and a sharp plate under the rounded,
/// in both orders.
#[test]
fn p5_results_reused() {
    let r = 0.5;
    let sharp = slab(0.0, 6.0, 4.0, 0.0, 1.0, 1.0).body(1.0);
    let rnd = slab(0.0, 6.0, 4.0, r, 0.0, 1.0).body(1.0);
    let d = findings(&sharp, &rnd);
    let Ok(BooleanResult::Body(u)) = topo::union_with(&sharp, &rnd, &d, tol()) else {
        panic!("the base union")
    };
    let u = u.body;
    let vu = 24.0 + slab(0.0, 6.0, 4.0, r, 0.0, 1.0).vol();
    for (name, c, vc) in [
        ("rounded on top", slab(0.0, 6.0, 4.0, 0.3, 2.0, 1.0), 0.0),
        ("sharp below", slab(0.0, 6.0, 4.0, 0.0, -1.0, 1.0), 0.0),
    ] {
        let cb = c.body(1.0);
        for (order, a, b, va, vb) in [
            ("U is A", &u, &cb, vu, c.vol()),
            ("U is B", &cb, &u, c.vol(), vu),
        ] {
            let dab = findings(a, b);
            let dba = findings(b, a);
            for (op, out) in [
                (Op::Union, topo::union_with(a, b, &dab, tol())),
                (Op::AmB, topo::subtract_with(a, b, &dab, tol())),
                (Op::BmA, topo::subtract_with(b, a, &dba, tol())),
                (Op::Meet, topo::intersect_with(a, b, &dab, tol())),
            ] {
                let ev = expect(op, va, vb, vc);
                let res = match out {
                    Ok(BooleanResult::Body(bb)) => {
                        let v = topo::mass_properties(&bb.body, tol()).unwrap().volume;
                        let val = topo::validate_geometric(&bb.body, tol()).is_ok()
                            && topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol())
                                .is_ok();
                        assert!(
                            (v - ev).abs() < 1e-9 * ev && val,
                            "{name} {order} {op:?}: {v} vs {ev}, valid {val}"
                        );
                        format!("ok {v}")
                    }
                    Ok(BooleanResult::Empty) => {
                        assert!(ev == 0.0, "{name} {order} {op:?} empty");
                        "empty".into()
                    }
                    Err(e) => format!("refuse {}", short(&e)),
                };
                println!("reuse {name} / {order} / {op:?}: {res}");
            }
        }
    }
}

/// P6 (claim 3): a declared `Tangent` box whose wall edges graze the
/// fillet mid-ruling at a height where the plate has NO vertex and no
/// edge: nothing splits them, so the hold must answer as main did — a
/// typed refusal, or a correct body. And the same box undeclared.
#[test]
fn p6_unsplit_hold_reads_as_held() {
    let s2 = core::f64::consts::FRAC_1_SQRT_2;
    let (w, rr) = (6.0, 0.5);
    let touch = Point2::new(w - rr + rr * s2, rr - rr * s2);
    let at = |along: f64, out: f64| {
        Point2::new(touch.x + (along + out) * s2, touch.y + (along - out) * s2)
    };
    let plate = slab(0.0, 6.0, 4.0, rr, 0.0, 1.0).body(1.0);
    let fillet = plate
        .faces()
        .map(|(k, _)| k)
        .find(|&f| {
            matches!(
                plate.get_face(f).and_then(|x| plate.get_surface(x.surface)),
                Some(geom::Surface::Cylinder { origin, .. }) if origin.x > 3.0 && origin.y < 2.0
            )
        })
        .unwrap();
    for (z0, h) in [(0.25, 0.5), (0.25, 1.0), (-0.5, 1.0), (0.0, 1.0)] {
        let boxed = extruded(
            sketch_at(z0),
            vec![ProfileLoop::polygon([
                at(-0.5, 0.0),
                at(-0.5, 1.0),
                at(0.5, 1.0),
                at(0.5, 0.0),
            ])],
            h,
            tol(),
        );
        let wall = boxed
            .faces()
            .map(|(k, _)| k)
            .find(|&f| {
                matches!(
                    boxed.get_face(f).and_then(|x| boxed.get_surface(x.surface)),
                    Some(geom::Surface::Plane { normal, .. })
                        if (normal.x + s2).abs() < 1e-9 && (normal.y - s2).abs() < 1e-9
                )
            })
            .unwrap();
        let decl = |fa, fb| BooleanDeclarations {
            coincident_faces: vec![FacePairDeclaration::new(
                fa,
                fb,
                topo::ContactClass::Tangent,
            )],
            ..BooleanDeclarations::default()
        };
        let pv = slab(0.0, 6.0, 4.0, rr, 0.0, 1.0).vol();
        // box volume 1·h; overlap none (the box is outside the plate).
        for (order, a, b, fa, fb, va, vb) in [
            ("box A", &boxed, &plate, wall, fillet, h, pv),
            ("plate A", &plate, &boxed, fillet, wall, pv, h),
        ] {
            for (posture, dab, dba) in [
                ("declared", decl(fa, fb), decl(fb, fa)),
                (
                    "undeclared",
                    BooleanDeclarations::default(),
                    BooleanDeclarations::default(),
                ),
            ] {
                for (op, out) in [
                    (Op::Union, topo::union_with(a, b, &dab, tol())),
                    (Op::AmB, topo::subtract_with(a, b, &dab, tol())),
                    (Op::BmA, topo::subtract_with(b, a, &dba, tol())),
                    (Op::Meet, topo::intersect_with(a, b, &dab, tol())),
                ] {
                    let ev = expect(op, va, vb, 0.0);
                    let res = match out {
                        Ok(BooleanResult::Body(bb)) => {
                            let v = topo::mass_properties(&bb.body, tol()).unwrap().volume;
                            let val = topo::validate_geometric(&bb.body, tol()).is_ok()
                                && topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol())
                                    .is_ok();
                            let good = (v - ev).abs() < 1e-9 * ev.max(1.0) && val;
                            assert!(good, "z0={z0} h={h} {order} {posture} {op:?}: {v} vs {ev}");
                            format!("ok {v:.6}")
                        }
                        Ok(BooleanResult::Empty) => {
                            assert!(ev == 0.0);
                            "empty".into()
                        }
                        Err(e) => format!("refuse {}", short(&e)),
                    };
                    println!("box z0={z0} h={h} / {order} / {posture} / {op:?}: {res}");
                }
            }
        }
    }
}

fn ell_poly(pts: &[(f64, f64)]) -> ProfileLoop<f64> {
    ProfileLoop::polygon(pts.iter().map(|&(x, y)| Point2::new(x, y)).collect::<Vec<_>>())
}

/// P7 (claim 6, first filed item): the L of the filing, r = 1, both
/// orders; prints the payloads.
#[test]
fn p7_filed_uncovered_vertex_touch() {
    let pts = [(0.0, 0.0), (6.0, 0.0), (6.0, 2.0), (3.0, 2.0), (3.0, 4.0), (0.0, 4.0)];
    let t = tol();
    let mut path = Open.at(Point2::new(3.0, 0.0)).toward(1.0, 0.0, t).unwrap();
    for (c, (dx, dy)) in [
        (Point2::new(6.0, 1.0), (0.0, 1.0)),
        (Point2::new(4.5, 2.0), (-1.0, 0.0)),
        (Point2::new(3.0, 3.0), (0.0, 1.0)),
        (Point2::new(1.5, 4.0), (-1.0, 0.0)),
        (Point2::new(0.0, 2.0), (0.0, -1.0)),
    ] {
        path = path.fillet(1.0, t).unwrap().at(c, t).unwrap().toward(dx, dy, t).unwrap();
    }
    let rl: ProfileLoop<f64> = path.fillet(1.0, t).unwrap().to(profile::Start, t).unwrap().into();
    let rounded = extruded(sketch_at(0.0), vec![rl], 1.0, tol());
    let sharp = extruded(sketch_at(1.0), vec![ell_poly(&pts)], 1.0, tol());
    for (o, a, b) in [("sharp A", &sharp, &rounded), ("rounded A", &rounded, &sharp)] {
        let d = findings(a, b);
        println!("filed-1 {o}: {:?}", topo::union_with(a, b, &d, tol()).err());
    }
}

/// P8 (claim 6, second filed item): a box corner on the declared
/// tangent ruling, both directions of the box, both orders.
#[test]
fn p8_filed_box_corner_on_ruling() {
    let s2 = core::f64::consts::FRAC_1_SQRT_2;
    let rr = 0.5;
    let touch = Point2::new(6.0 - rr + rr * s2, rr - rr * s2);
    let at = |along: f64, out: f64| {
        Point2::new(touch.x + (along + out) * s2, touch.y + (along - out) * s2)
    };
    let plate = slab(0.0, 6.0, 4.0, rr, 0.0, 1.0).body(1.0);
    let fillet = plate
        .faces()
        .map(|(k, _)| k)
        .find(|&f| {
            matches!(
                plate.get_face(f).and_then(|x| plate.get_surface(x.surface)),
                Some(geom::Surface::Cylinder { origin, .. }) if origin.x > 3.0 && origin.y < 2.0
            )
        })
        .unwrap();
    for (lo, hi) in [(0.0, 1.0), (-1.0, 0.0)] {
        let boxed = extruded(
            sketch_at(0.0),
            vec![ProfileLoop::polygon([at(lo, 0.0), at(lo, 1.0), at(hi, 1.0), at(hi, 0.0)])],
            1.0,
            tol(),
        );
        let wall = boxed
            .faces()
            .map(|(k, _)| k)
            .find(|&f| {
                matches!(
                    boxed.get_face(f).and_then(|x| boxed.get_surface(x.surface)),
                    Some(geom::Surface::Plane { normal, .. })
                        if (normal.x + s2).abs() < 1e-9 && (normal.y - s2).abs() < 1e-9
                )
            })
            .unwrap();
        let decl = |fa, fb| BooleanDeclarations {
            coincident_faces: vec![FacePairDeclaration::new(fa, fb, topo::ContactClass::Tangent)],
            ..BooleanDeclarations::default()
        };
        for (o, a, b, d) in [
            ("box A", &boxed, &plate, decl(wall, fillet)),
            ("plate A", &plate, &boxed, decl(fillet, wall)),
        ] {
            println!(
                "filed-2 along [{lo},{hi}] {o}: ∪ {:?} | ∩ {:?}",
                topo::union_with(a, b, &d, tol()).err().map(|e| short(&e)),
                topo::intersect_with(a, b, &d, tol()).err().map(|e| short(&e))
            );
        }
    }
}

/// P9 (claim 1/3 pin): the declared box beside the fillet at z ∈ [0.25,
/// 0.75] — its wall edges graze the fillet mid-ruling where the plate
/// has no vertex and no edge, so nothing splits them and the held pair
/// must answer its typed refusal in both orders. Red under "settle
/// accepts anything" and "settle ignores a still-interior fragment".
#[test]
fn p9_an_unsplit_held_pair_refuses_in_both_orders() {
    let s2 = core::f64::consts::FRAC_1_SQRT_2;
    let rr = 0.5;
    let touch = Point2::new(6.0 - rr + rr * s2, rr - rr * s2);
    let at = |along: f64, out: f64| {
        Point2::new(touch.x + (along + out) * s2, touch.y + (along - out) * s2)
    };
    let plate = slab(0.0, 6.0, 4.0, rr, 0.0, 1.0).body(1.0);
    let fillet = plate
        .faces()
        .map(|(k, _)| k)
        .find(|&f| {
            matches!(
                plate.get_face(f).and_then(|x| plate.get_surface(x.surface)),
                Some(geom::Surface::Cylinder { origin, .. }) if origin.x > 3.0 && origin.y < 2.0
            )
        })
        .unwrap();
    let boxed = extruded(
        sketch_at(0.25),
        vec![ProfileLoop::polygon([at(-0.5, 0.0), at(-0.5, 1.0), at(0.5, 1.0), at(0.5, 0.0)])],
        0.5,
        tol(),
    );
    let wall = boxed
        .faces()
        .map(|(k, _)| k)
        .find(|&f| {
            matches!(
                boxed.get_face(f).and_then(|x| boxed.get_surface(x.surface)),
                Some(geom::Surface::Plane { normal, .. })
                    if (normal.x + s2).abs() < 1e-9 && (normal.y - s2).abs() < 1e-9
            )
        })
        .unwrap();
    let decl = |fa, fb| BooleanDeclarations {
        coincident_faces: vec![FacePairDeclaration::new(fa, fb, topo::ContactClass::Tangent)],
        ..BooleanDeclarations::default()
    };
    for (o, a, b, d) in [
        ("box A", &boxed, &plate, decl(wall, fillet)),
        ("plate A", &plate, &boxed, decl(fillet, wall)),
    ] {
        let err = topo::union_with(a, b, &d, tol()).expect_err("an unsplit held pair refuses");
        println!("p9 {o}: {err:?}");
        assert!(matches!(err, BooleanError::CurvedPierceUnsupported { .. }), "{o}: {err:?}");
    }
}

/// P10: the payloads of the both-order refusals P1–P4 tallied.
#[test]
fn p10_payloads() {
    let r = 0.5;
    for (name, p, q) in [
        ("dx=0.25", slab(0.25, 6.0, 4.0, 0.0, 1.0, 1.0), slab(0.0, 6.0, 4.0, r, 0.0, 1.0)),
        ("0.5/0.5001", slab(0.0, 6.0, 4.0, 0.5, 1.0, 1.0), slab(0.0, 6.0, 4.0, 0.5001, 0.0, 1.0)),
    ] {
        let (a, b) = (p.body(1.0), q.body(1.0));
        for (o, a, b) in [("p A", &a, &b), ("q A", &b, &a)] {
            let d = findings(a, b);
            println!("p10 {name} {o}: {:?}", topo::union_with(a, b, &d, tol()).err());
        }
    }
}
