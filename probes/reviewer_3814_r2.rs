//! Reviewer probes for PR #3814 (lane reach-dual3814-r2). Each case
//! builds a collar (full-turn revolve) and a three-arc shaft, runs a
//! boolean, and classifies the outcome against an independent closed
//! form plus a Monte Carlo `point_in_solid` oracle: RIGHT, REFUSED (typed
//! error, printed), or WRONG. Only WRONG fails.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use crate::common::three_arc;
use core::f64::consts::{FRAC_PI_2, PI};
use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use sweep::test_support::{extruded, sketch_at};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{
    Body, BooleanDeclarations, BooleanOp, BooleanResult, ContactClass, FacePairDeclaration,
    SolidContainment, mass_properties,
};

#[derive(Clone, Copy, Debug)]
struct Scene {
    r: f64,      // bore radius
    rr: f64,     // collar outer radius
    cy: (f64, f64),
    sr: f64,     // shaft radius
    deg: f64,    // shaft first ruling azimuth
    sy: f64,     // shaft y0
    sh: f64,     // shaft height
    ecc: f64,    // shaft centre offset along local x (sketch x)
    tilt: f64,   // shaft tilt (rad) about z through the collar's mid-height on axis
    s: f64,      // global scale
}

fn base() -> Scene {
    Scene { r: 0.5, rr: 1.5, cy: (1.0, 2.0), sr: 0.5, deg: 0.0, sy: 0.5, sh: 2.0, ecc: 0.0, tilt: 0.0, s: 1.0 }
}

fn collar(sc: &Scene) -> Body<f64> {
    let s = sc.s;
    let lp = ProfileLoop::polygon([
        Point2::new(sc.r * s, sc.cy.0 * s),
        Point2::new(sc.rr * s, sc.cy.0 * s),
        Point2::new(sc.rr * s, sc.cy.1 * s),
        Point2::new(sc.r * s, sc.cy.1 * s),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp]).validate(Tol::witness()).unwrap();
    let axis = RevolveAxis { origin: Point2::new(0.0, 0.0), dir: Vec2::new(0.0, 1.0) };
    revolve(&vp, axis, Revolution::Full, Tol::witness()).unwrap().body
}

fn shaft(sc: &Scene) -> Body<f64> {
    let s = sc.s;
    let peg = extruded(
        sketch_at(sc.sy * s),
        vec![three_arc(Point2::new(sc.ecc * s, 0.0), sc.sr * s, sc.deg)],
        sc.sh * s,
        Tol::witness(),
    );
    let up = Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), -FRAC_PI_2);
    let b = topo::transform_rigid(&peg, &up, Tol::witness()).unwrap();
    if sc.tilt == 0.0 {
        b
    } else {
        let mid = 0.5 * (sc.cy.0 + sc.cy.1) * s;
        let t = Affine3::rotation_about_axis(Point3::new(0.0, mid, 0.0), Vec3::new(0.0, 0.0, 1.0), sc.tilt);
        topo::transform_rigid(&b, &t, Tol::witness()).unwrap()
    }
}

fn walls(b: &Body<f64>, r: f64) -> Vec<topo::FaceKey> {
    b.faces()
        .filter(|(_, f)| matches!(b.get_surface(f.surface),
            Some(geom::Surface::Cylinder { radius, .. }) if (radius - r).abs() < 1e-6 * r.max(1.0)))
        .map(|(k, _)| k)
        .collect()
}

fn decls(a: &Body<f64>, ra: f64, b: &Body<f64>, rb: f64) -> BooleanDeclarations {
    let mut d = BooleanDeclarations::none();
    for &fa in &walls(a, ra) {
        for &fb in &walls(b, rb) {
            d.coincident_faces.push(FacePairDeclaration::new(fa, fb, ContactClass::Rest));
        }
    }
    d
}

/// Membership in the collar / shaft, in the UNPOSED frame, for points
/// clear of every boundary by `m` (else None).
fn in_collar(sc: &Scene, p: Point3<f64>, m: f64) -> Option<bool> {
    let s = sc.s;
    let rho = (p.x * p.x + p.z * p.z).sqrt();
    let d = [(rho - sc.r * s).abs(), (rho - sc.rr * s).abs(), (p.y - sc.cy.0 * s).abs(), (p.y - sc.cy.1 * s).abs()];
    let inside = rho > sc.r * s && rho < sc.rr * s && p.y > sc.cy.0 * s && p.y < sc.cy.1 * s;
    if d.iter().any(|&x| x < m) { None } else { Some(inside) }
}

fn in_shaft(sc: &Scene, p: Point3<f64>, m: f64) -> Option<bool> {
    let s = sc.s;
    // undo tilt
    let q = if sc.tilt == 0.0 {
        p
    } else {
        let mid = 0.5 * (sc.cy.0 + sc.cy.1) * s;
        Affine3::rotation_about_axis(Point3::new(0.0, mid, 0.0), Vec3::new(0.0, 0.0, 1.0), -sc.tilt).transform_point(p)
    };
    // shaft axis: y, centre (ecc*s, *, 0) after the -pi/2 about x (sketch y -> -z)
    let dx = q.x - sc.ecc * s;
    let rho = (dx * dx + q.z * q.z).sqrt();
    let d = [(rho - sc.sr * s).abs(), (q.y - sc.sy * s).abs(), (q.y - (sc.sy + sc.sh) * s).abs()];
    let inside = rho < sc.sr * s && q.y > sc.sy * s && q.y < (sc.sy + sc.sh) * s;
    if d.iter().any(|&x| x < m) { None } else { Some(inside) }
}

fn lcg(state: &mut u64) -> f64 {
    *state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    ((*state >> 11) as f64) / ((1u64 << 53) as f64)
}

#[derive(Debug, PartialEq)]
enum Verdict {
    Right,
    Refused(String),
    Wrong(String),
}

/// Run `op` on (collar, shaft) [or swapped], posed, and judge.
fn judge(sc: &Scene, op: BooleanOp, swap: bool, pose: &Affine3<f64>, samples: usize) -> Verdict {
    let tol = Tol::witness();
    let c = topo::transform_rigid(&collar(sc), pose, tol).unwrap();
    let p = topo::transform_rigid(&shaft(sc), pose, tol).unwrap();
    let (a, b, d) = if swap {
        (&p, &c, decls(&p, sc.sr * sc.s, &c, sc.r * sc.s))
    } else {
        (&c, &p, decls(&c, sc.r * sc.s, &p, sc.sr * sc.s))
    };
    let out = topo::boolean_op_with(op, a, b, &d, topo::SweepStrategy::Realized, tol);
    // the membership rule of the op over (a in collar?, b in shaft?)
    let rule = |ic: bool, is: bool| -> bool {
        let (ia, ib) = if swap { (is, ic) } else { (ic, is) };
        match op {
            BooleanOp::Union => ia || ib,
            BooleanOp::Intersect => ia && ib,
            BooleanOp::Subtract => ia && !ib,
        }
    };
    let s = sc.s;
    let lo = Point3::new(-sc.rr * s * 1.1, (sc.cy.0.min(sc.sy) - 0.2) * s, -sc.rr * s * 1.1);
    let hi = Point3::new(sc.rr * s * 1.1, (sc.cy.1.max(sc.sy + sc.sh) + 0.2) * s, sc.rr * s * 1.1);
    let bb = match out {
        Err(e) => return Verdict::Refused(format!("{e:?}").chars().take(160).collect()),
        Ok(BooleanResult::Empty) => {
            // empty is right iff no sampled point is in the oracle set
            let mut st = 7u64;
            for _ in 0..samples.max(2000) {
                let l = Point3::new(lo.x + (hi.x - lo.x) * lcg(&mut st), lo.y + (hi.y - lo.y) * lcg(&mut st), lo.z + (hi.z - lo.z) * lcg(&mut st));
                if let (Some(ic), Some(is)) = (in_collar(sc, l, (1e-6 * s).max(2e-5)), in_shaft(sc, l, (1e-6 * s).max(2e-5))) {
                    if rule(ic, is) {
                        return Verdict::Wrong(format!("Empty but {l:?} is in"));
                    }
                }
            }
            return Verdict::Right;
        }
        Ok(BooleanResult::Body(bb)) => bb,
    };
    let body = &bb.body;
    if let Err(e) = topo::validate_geometric(body, tol) {
        return Verdict::Wrong(format!("tier 3: {e:?}"));
    }
    if let Err(e) = topo::validate_pseudomanifold(body, &bb.contacts, tol) {
        return Verdict::Wrong(format!("census: {e:?}"));
    }
    let band = Band::linear(tol).unwrap();
    let mut st = 0x3814u64;
    let mut bad = 0usize;
    let mut first = String::new();
    let mut asked = 0usize;
    let mut refusals = 0usize;
    let mut oracle_vol = 0usize;
    let mut n = 0usize;
    while n < samples {
        let l = Point3::new(lo.x + (hi.x - lo.x) * lcg(&mut st), lo.y + (hi.y - lo.y) * lcg(&mut st), lo.z + (hi.z - lo.z) * lcg(&mut st));
        let (Some(ic), Some(is)) = (in_collar(sc, l, (1e-6 * s).max(2e-5)), in_shaft(sc, l, (1e-6 * s).max(2e-5))) else { continue };
        n += 1;
        let want = rule(ic, is);
        if want {
            oracle_vol += 1;
        }
        let w = pose.transform_point(l);
        match topo::point_in_solid(body, w, band, tol) {
            Ok(SolidContainment::In) if want => {}
            Ok(SolidContainment::Out) if !want => {}
            Ok(got) => {
                bad += 1;
                if first.is_empty() {
                    first = format!("{l:?}: got {got:?} want in={want}");
                }
            }
            Err(_) => refusals += 1,
        }
        asked += 1;
    }
    let _ = oracle_vol;
    let vol = mass_properties(body, tol).map(|m| m.volume).unwrap_or(f64::NAN);
    if bad > 0 {
        return Verdict::Wrong(format!("pis mismatches {bad}/{asked} (refused {refusals}); first {first}; vol {vol}"));
    }
    Verdict::Right
}

/// Closed-form union volume for coaxial, fitting cases.
fn coaxial_union_volume(sc: &Scene) -> f64 {
    let s3 = sc.s.powi(3);
    let annulus = PI * (sc.rr * sc.rr - sc.r * sc.r) * (sc.cy.1 - sc.cy.0);
    let disc = PI * sc.sr * sc.sr * sc.sh;
    (annulus + disc) * s3
}

fn union_volume_check(sc: &Scene, pose: &Affine3<f64>, swap: bool) -> Result<f64, String> {
    let tol = Tol::witness();
    let c = topo::transform_rigid(&collar(sc), pose, tol).unwrap();
    let p = topo::transform_rigid(&shaft(sc), pose, tol).unwrap();
    let r = if swap {
        topo::union_with(&p, &c, &decls(&p, sc.sr * sc.s, &c, sc.r * sc.s), tol)
    } else {
        topo::union_with(&c, &p, &decls(&c, sc.r * sc.s, &p, sc.sr * sc.s), tol)
    };
    match r {
        Ok(BooleanResult::Body(bb)) => Ok(mass_properties(&bb.body, tol).unwrap().volume),
        Ok(BooleanResult::Empty) => Err("Empty".into()),
        Err(e) => Err(format!("{e:?}").chars().take(160).collect()),
    }
}

fn poses() -> Vec<(&'static str, Affine3<f64>)> {
    crate::common::poses::poses()
}

fn tally(rows: &[(String, Verdict)]) {
    let mut wrong = 0;
    for (tag, v) in rows {
        eprintln!("R2PROBE {tag}: {v:?}");
        if matches!(v, Verdict::Wrong(_)) {
            wrong += 1;
        }
    }
    let right = rows.iter().filter(|(_, v)| *v == Verdict::Right).count();
    eprintln!("R2PROBE TALLY right {right} refused {} wrong {wrong}", rows.len() - right - wrong);
    assert_eq!(wrong, 0, "a WRONG body shipped");
}

/// P1: union widened — azimuths × radii × scales × poses × both orders,
/// volume against closed form AND Monte Carlo point_in_solid.
#[test]
fn p1_union_widened() {
    let mut rows = Vec::new();
    for (deg, r, rr, s) in [
        (17.0, 0.5, 1.5, 1.0),
        (119.99, 0.5, 1.5, 1.0),
        (180.0, 0.5, 1.5, 1.0),
        (300.0, 0.5, 1.5, 1.0),
        (45.0, 0.25, 0.4, 1.0),
        (45.0, 2.0, 3.5, 1.0),
        (45.0, 0.5, 1.5, 1e-3),
        (45.0, 0.5, 1.5, 1e3),
        (0.0, 0.5, 1.5, 1e3),
    ] {
        for (span, sy, sh) in [("through", 0.5, 2.0), ("flush", 1.0, 1.0), ("top-flush", 1.0, 1.7), ("bot-flush", 0.3, 1.7)] {
            let sc = Scene { r, rr, sr: r, deg, sy, sh, s, ..base() };
            for (pn, pose) in poses().into_iter().take(if s == 1.0 && r == 0.5 { 6 } else { 3 }) {
                for swap in [false, true] {
                    let tag = format!("deg {deg} r {r} s {s} {span} pose[{pn}] swap {swap}");
                    let v = match union_volume_check(&sc, &pose, swap) {
                        Err(e) => Verdict::Refused(e),
                        Ok(vol) => {
                            let want = coaxial_union_volume(&sc);
                            if (vol - want).abs() > 1e-9 * want {
                                Verdict::Wrong(format!("volume {vol} vs {want}"))
                            } else {
                                judge(&sc, BooleanOp::Union, swap, &pose, 300)
                            }
                        }
                    };
                    rows.push((tag, v));
                }
            }
        }
    }
    tally(&rows);
}

/// P2: misfits — eccentric shafts, a slimmer/fatter shaft, a tilted
/// shaft (by a hair and by a visible angle), all with the Rest
/// declared. Must refuse or be right.
#[test]
fn p2_misfits_refuse_or_are_right() {
    let mut rows = Vec::new();
    let cases: Vec<(&str, Scene)> = vec![
        ("ecc 0.1", Scene { ecc: 0.1, ..base() }),
        ("ecc 1e-7", Scene { ecc: 1e-7, ..base() }),
        ("ecc 1e-11", Scene { ecc: 1e-11, ..base() }),
        ("slim 0.49", Scene { sr: 0.49, ..base() }),
        ("fat 0.51", Scene { sr: 0.51, ..base() }),
        ("slim 0.5-1e-11", Scene { sr: 0.5 - 1e-11, ..base() }),
        ("tilt 1e-12", Scene { tilt: 1e-12, ..base() }),
        ("tilt 1e-9", Scene { tilt: 1e-9, ..base() }),
        ("tilt 1e-6", Scene { tilt: 1e-6, ..base() }),
        ("tilt 0.05", Scene { tilt: 0.05, ..base() }),
        ("tilt 1e-12 deg 60", Scene { tilt: 1e-12, deg: 60.0, ..base() }),
        ("ecc 0.1 flush", Scene { ecc: 0.1, sy: 1.0, sh: 1.0, ..base() }),
    ];
    for (name, sc) in cases {
        for (pn, pose) in poses().into_iter().take(2) {
            for op in [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract] {
                for swap in [false, true] {
                    let tag = format!("{name} {op:?} pose[{pn}] swap {swap}");
                    rows.push((tag, judge(&sc, op, swap, &pose, 300)));
                }
            }
        }
    }
    tally(&rows);
}

/// P3: blind shafts (ending inside the bore) from below and above, at
/// several azimuths; and ∩/∖ on the fitting mate. Records the refusal
/// payloads; any body must be right.
#[test]
fn p3_blind_and_other_ops() {
    let mut rows = Vec::new();
    for deg in [0.0, 60.0, 90.0] {
        for (name, sy, sh) in [("blind-below", 0.5, 1.0), ("blind-above", 1.5, 1.0), ("inside", 1.2, 0.5)] {
            let sc = Scene { deg, sy, sh, ..base() };
            for op in [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract] {
                for swap in [false, true] {
                    let tag = format!("deg {deg} {name} {op:?} swap {swap}");
                    rows.push((tag, judge(&sc, op, swap, &poses()[1].1, 300)));
                }
            }
        }
        for (name, sy, sh) in [("through", 0.5, 2.0), ("flush", 1.0, 1.0)] {
            let sc = Scene { deg, sy, sh, ..base() };
            for op in [BooleanOp::Intersect, BooleanOp::Subtract] {
                for swap in [false, true] {
                    let tag = format!("deg {deg} {name} {op:?} swap {swap}");
                    rows.push((tag, judge(&sc, op, swap, &poses()[0].1, 300)));
                }
            }
        }
    }
    tally(&rows);
}

/// P4: the union reused as an operand — subtract a box straddling the
/// mate, and union a second shaft stacked on top; point_in_solid on
/// each result against the composed oracle.
#[test]
fn p4_result_reused() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    for deg in [0.0, 60.0, 90.0] {
        let sc = Scene { deg, ..base() };
        let c = collar(&sc);
        let p = shaft(&sc);
        let u = match topo::union_with(&c, &p, &decls(&c, 0.5, &p, 0.5), tol) {
            Ok(BooleanResult::Body(bb)) => bb.body,
            other => panic!("deg {deg}: base union {:?}", other.err()),
        };
        // a box x in [0, 2], y in [0.8, 1.6], z in [-2, 2] (cuts the mate)
        let bx = extruded(
            sketch_at(-2.0),
            vec![ProfileLoop::polygon([
                Point2::new(0.0, 0.8),
                Point2::new(2.0, 0.8),
                Point2::new(2.0, 1.6),
                Point2::new(0.0, 1.6),
            ])],
            4.0,
            tol,
        );
        let r = topo::boolean_op_with(
            BooleanOp::Subtract,
            &u,
            &bx,
            &BooleanDeclarations::none(),
            topo::SweepStrategy::Realized,
            tol,
        );
        let in_box = |q: Point3<f64>| q.x > 0.0 && q.x < 2.0 && q.y > 0.8 && q.y < 1.6 && q.z > -2.0 && q.z < 2.0;
        let near_box = |q: Point3<f64>| {
            [q.x, q.x - 2.0, q.y - 0.8, q.y - 1.6].iter().any(|d| d.abs() < 1e-6)
        };
        match r {
            Err(e) => eprintln!("R2PROBE p4 deg {deg} u-box refused {e:?}"),
            Ok(BooleanResult::Empty) => panic!("deg {deg}: u-box empty"),
            Ok(BooleanResult::Body(bb)) => {
                topo::validate_geometric(&bb.body, tol).unwrap();
                let mut st = 99u64;
                let mut bad = 0;
                let mut n = 0;
                while n < 400 {
                    let l = Point3::new(-1.7 + 3.4 * lcg(&mut st), 0.3 + 2.4 * lcg(&mut st), -1.7 + 3.4 * lcg(&mut st));
                    let (Some(ic), Some(is)) = (in_collar(&sc, l, 1e-6), in_shaft(&sc, l, 1e-6)) else { continue };
                    if near_box(l) {
                        continue;
                    }
                    n += 1;
                    let want = (ic || is) && !in_box(l);
                    match topo::point_in_solid(&bb.body, l, band, tol) {
                        Ok(SolidContainment::In) if want => {}
                        Ok(SolidContainment::Out) if !want => {}
                        other => {
                            bad += 1;
                            eprintln!("R2PROBE p4 deg {deg} {l:?} {other:?} want {want}");
                        }
                    }
                }
                let vol = mass_properties(&bb.body, tol).unwrap().volume;
                eprintln!("R2PROBE p4 deg {deg} u-box body vol {vol} bad {bad}/{n}");
                assert_eq!(bad, 0);
            }
        }
    }
}

/// P5: the swapped-order refusal, full payload and the result's faces.
#[test]
fn p5_swap_detail() {
    let tol = Tol::witness();
    for (deg, sy, sh) in [(60.0, 0.5, 2.0), (0.0, 1.0, 1.7), (0.0, 0.5, 2.0)] {
        let sc = Scene { deg, sy, sh, ..base() };
        let c = collar(&sc);
        let p = shaft(&sc);
        let r = topo::union_with(&p, &c, &decls(&p, 0.5, &c, 0.5), tol);
        eprintln!("R2PROBE p5 deg {deg} sy {sy} sh {sh}: {:?}", r.as_ref().err());
        let r2 = topo::union_with(&c, &p, &decls(&c, 0.5, &p, 0.5), tol);
        if let Ok(BooleanResult::Body(bb)) = r2 {
            let kinds: Vec<_> = bb.body.faces().map(|(_, f)| bb.body.get_surface(f.surface).map(|s| s.kind())).collect();
            eprintln!("R2PROBE p5 c∪p faces {} {:?}", kinds.len(), kinds);
            let lines = bb.body.edges().filter(|(_, e)| matches!(bb.body.get_curve_geom(e.curve), Some(topo::CurveGeom::Certified(c)) if matches!(c.carrier(), geom::Curve3::Line{..}))).count();
            eprintln!("R2PROBE p5 c∪p edges {} line edges {lines}", bb.body.edges().count());
        }
    }
}

/// P6: the established class (arc-split collar, mate2_common) in both
/// orders — is the swapped-order refusal this lane's or the class's?
#[test]
fn p6_established_class_both_orders() {
    use crate::mate2_common::{collar_at, peg_at, wall_decls};
    let tol = Tol::witness();
    for (cdeg, pdeg) in [(0.0, 0.0), (0.0, 60.0)] {
        for (z0, h) in [(0.5, 2.0), (1.0, 1.0), (1.0, 1.7)] {
            let c = collar_at(cdeg);
            let p = peg_at(pdeg, z0, h);
            let ab = topo::union_with(&c, &p, &wall_decls(&c, &p), tol).map(|r| r.body().map(|b| mass_properties(&b.body, tol).unwrap().volume));
            let ba = topo::union_with(&p, &c, &wall_decls(&p, &c), tol).map(|r| r.body().map(|b| mass_properties(&b.body, tol).unwrap().volume));
            let want = PI * (2.25 - 0.25) + PI * 0.25 * h;
            eprintln!("R2PROBE p6 collar {cdeg} peg {pdeg} z0 {z0} h {h} want {want}: c∪p {:?} | p∪c {:?}", ab, ba);
        }
    }
}

/// P7: rims whose neighbour is CURVED (a convex round, a chamfer cone),
/// so no planar cap pre-splits the rim where the shaft's rulings pass
/// it — the closed-form meeting candidates are the only route.
#[test]
fn p7_curved_rim_neighbours() {
    use profile::test_support::bulge_loop;
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    type Member = Box<dyn Fn(f64, f64) -> Option<bool>>;
    let near = |d: f64| d.abs() < 1e-6;
    let mut rows = Vec::new();
    let variants: Vec<(&str, ProfileLoop<f64>, Member)> = vec![
        (
            "round top",
            bulge_loop(vec![
                (Point2::new(0.5, 1.0), 0.0),
                (Point2::new(1.5, 1.0), 0.0),
                (Point2::new(1.5, 2.3), 0.0),
                (Point2::new(0.8, 2.3), crate::common::bulge(Point2::new(0.8, 2.3), Point2::new(0.5, 2.0), Point2::new(0.9, 1.9))),
                (Point2::new(0.5, 2.0), 0.0),
            ]),
            Box::new(move |rho: f64, y: f64| {
                let dc = ((rho - 0.9).powi(2) + (y - 1.9).powi(2)).sqrt();
                if near(rho - 0.5) || near(rho - 1.5) || near(y - 1.0) || near(y - 2.3) || near(dc - 0.17f64.sqrt()) {
                    return None;
                }
                let in_rect = rho > 0.5 && rho < 1.5 && y > 1.0 && y < 2.3;
                let cut = rho < 0.8 && y > 2.0 && dc > 0.17f64.sqrt();
                Some(in_rect && !cut)
            }),
        ),
        (
            "chamfers",
            bulge_loop(vec![
                (Point2::new(0.8, 0.7), 0.0),
                (Point2::new(1.5, 0.7), 0.0),
                (Point2::new(1.5, 2.3), 0.0),
                (Point2::new(0.8, 2.3), 0.0),
                (Point2::new(0.5, 2.0), 0.0),
                (Point2::new(0.5, 1.0), 0.0),
            ]),
            Box::new(move |rho: f64, y: f64| {
                let c1 = rho - (0.5 + (y - 2.0));
                let c2 = rho - (0.5 + (1.0 - y));
                if near(rho - 0.5) || near(rho - 1.5) || near(y - 0.7) || near(y - 2.3) || near(c1) || near(c2) {
                    return None;
                }
                let in_rect = rho > 0.5 && rho < 1.5 && y > 0.7 && y < 2.3;
                let cut = (y > 2.0 && c1 < 0.0) || (y < 1.0 && c2 < 0.0);
                Some(in_rect && !cut)
            }),
        ),
    ];
    for (name, lp, member) in &variants {
        let vp = Profile::new(SketchPlane::xy(), vec![lp.clone()]).validate(tol).unwrap();
        let axis = RevolveAxis { origin: Point2::new(0.0, 0.0), dir: Vec2::new(0.0, 1.0) };
        let c0 = revolve(&vp, axis, Revolution::Full, tol).unwrap().body;
        for deg in [0.0, 60.0, 90.0] {
            for (sy, sh) in [(0.3, 2.4), (0.9, 1.0)] {
                let sc = Scene { deg, sy, sh, ..base() };
                for (pn, pose) in poses().into_iter().take(2) {
                    for swap in [false, true] {
                        let c = topo::transform_rigid(&c0, &pose, tol).unwrap();
                        let p = topo::transform_rigid(&shaft(&sc), &pose, tol).unwrap();
                        let tag = format!("{name} deg {deg} sy {sy} sh {sh} pose[{pn}] swap {swap}");
                        let r = if swap {
                            topo::union_with(&p, &c, &decls(&p, 0.5, &c, 0.5), tol)
                        } else {
                            topo::union_with(&c, &p, &decls(&c, 0.5, &p, 0.5), tol)
                        };
                        let v = match r {
                            Err(e) => Verdict::Refused(format!("{e:?}").chars().take(160).collect()),
                            Ok(BooleanResult::Empty) => Verdict::Wrong("Empty".into()),
                            Ok(BooleanResult::Body(bb)) => {
                                let want = volume_of(&c0) + PI * 0.25 * sh;
                                let got = volume_of(&bb.body);
                                let mut bad = String::new();
                                if (got - want).abs() > 1e-9 * want {
                                    bad = format!("volume {got} vs {want}");
                                }
                                if let Err(e) = topo::validate_geometric(&bb.body, tol) {
                                    bad = format!("{bad} tier3 {e:?}");
                                }
                                let mut st = 5u64;
                                let mut n = 0;
                                let mut mism = 0;
                                while n < 300 {
                                    let l = Point3::new(-1.6 + 3.2 * lcg(&mut st), 0.2 + 2.6 * lcg(&mut st), -1.6 + 3.2 * lcg(&mut st));
                                    let rho = (l.x * l.x + l.z * l.z).sqrt();
                                    let Some(ic) = member(rho, l.y) else { continue };
                                    let Some(is) = in_shaft(&sc, l, 1e-6) else { continue };
                                    n += 1;
                                    let want = ic || is;
                                    match topo::point_in_solid(&bb.body, pose.transform_point(l), band, tol) {
                                        Ok(SolidContainment::In) if want => {}
                                        Ok(SolidContainment::Out) if !want => {}
                                        _ => mism += 1,
                                    }
                                }
                                if mism > 0 {
                                    bad = format!("{bad} pis mismatches {mism}/300");
                                }
                                if bad.is_empty() { Verdict::Right } else { Verdict::Wrong(bad) }
                            }
                        };
                        rows.push((tag, v));
                    }
                }
            }
        }
    }
    tally(&rows);
}

fn volume_of(b: &Body<f64>) -> f64 {
    mass_properties(b, Tol::witness()).unwrap().volume
}
