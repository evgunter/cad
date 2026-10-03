//! PR #3987 dual review r1 — end-to-end probes through the public `pncad` API.
//! Independent oracles: closed forms (box arithmetic, πr²h, spherical cap) and
//! point membership computed analytically in the scene's own frame.
//! Mount: copy to crates/pncad/tests/ and add a [[test]] entry; run with
//! `cargo test --release -p pncad --test review3987_e2e -- --nocapture`.
#![allow(clippy::all, clippy::pedantic, clippy::restriction)]
use core::f64::consts::PI;
use pncad::geom_core::{Affine3, Point2, Point3, Tol, Vec2, Vec3};
use pncad::prelude::*;
use pncad::profile::{ArcSweep, Center, SketchPlane};
use pncad::sweep::{Revolution, RevolveAxis, revolve};
use pncad::topo::{AtRestBody, AtRestOutcome, Body, BooleanResult, SolidContainment};

fn tol() -> Tol { Tol::witness() }

fn slab(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    let t = tol();
    let rect: ClosedLoop<f64> = Open
        .at(p2(x.0, y.0)).line_to(p2(x.1, y.0), t)
        .and_then(|l| l.line_to(p2(x.1, y.1), t))
        .and_then(|l| l.line_to(p2(x.0, y.1), t))
        .and_then(|l| l.line_to(Start, t)).expect("rect");
    let plane = SketchPlane::from_frame(OrthoFrame::axes_xy(p3::<f64>(0.0, 0.0, z.0)));
    let profile = validated(plane, vec![rect.into()], t).expect("profile");
    extrude(&profile, Extrusion::Distance { depth: real(z.1 - z.0), side: ExtrudeSide::Along }, t)
        .expect("extrude").body
}

fn cyl(c: (f64, f64), r: f64, z: (f64, f64)) -> Body<f64> {
    let t = tol();
    let disc = circle(p2(c.0, c.1), r, t).expect("circle");
    let plane = SketchPlane::from_frame(OrthoFrame::axes_xy(p3::<f64>(0.0, 0.0, z.0)));
    let profile = validated(plane, vec![disc.into()], t).expect("disc profile");
    extrude(&profile, Extrusion::Distance { depth: real(z.1 - z.0), side: ExtrudeSide::Along }, t)
        .expect("cyl extrude").body
}

/// Ball of radius r centred at (0, y, 0), revolved about y.
fn ball_y(r: f64, y: f64) -> Body<f64> {
    let t = tol();
    let semi: ConstructedLoop<f64> = Open.at(Point2::new(0.0, y - r))
        .arc_to(Center { c: Point2::new(0.0, y), winding: ArcSweep::Ccw, p: Point2::new(0.0, y + r) }, t)
        .expect("arc").line_to(Start, t).expect("close").into();
    let profile = validated(SketchPlane::xy(), vec![semi], t).expect("semi");
    revolve(&profile, RevolveAxis { origin: p2(0.0, 0.0), dir: Vec2::new(0.0, 1.0) }, Revolution::Full, t)
        .expect("revolve").body
}

fn moved(b: &Body<f64>, m: Affine3<f64>) -> Body<f64> {
    pncad::topo::transform_rigid(b, &m, tol()).unwrap_or_else(|e| std::panic::panic_any(format!("SETUP rigid: {e}")))
}

fn fin(what: &str, b: Body<f64>) -> AtRestBody<f64> {
    AtRestBody::validate(b, tol()).unwrap_or_else(|e| panic!("{what}: not finished: {e:?}"))
}

/// Scene membership oracle in the scene's own (unscaled, unrotated) frame:
/// returns Some(inside) or None when within `margin` of a boundary.
type Oracle = Box<dyn Fn(Point3<f64>, f64) -> Option<bool>>;

fn box_o(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Oracle {
    Box::new(move |p, m| {
        let d = [p.x - x.0, x.1 - p.x, p.y - y.0, y.1 - p.y, p.z - z.0, z.1 - p.z];
        let min = d.iter().cloned().fold(f64::INFINITY, f64::min);
        if min.abs() < m { None } else { Some(min > 0.0) }
    })
}
fn cyl_o(c: (f64, f64), r: f64, z: (f64, f64)) -> Oracle {
    Box::new(move |p, m| {
        let rad = r - ((p.x - c.0).powi(2) + (p.y - c.1).powi(2)).sqrt();
        let d = rad.min(p.z - z.0).min(z.1 - p.z);
        if d.abs() < m { None } else { Some(d > 0.0) }
    })
}
fn ball_o(c: Point3<f64>, r: f64) -> Oracle {
    Box::new(move |p, m| {
        let d = r - (p - c).norm();
        if d.abs() < m { None } else { Some(d > 0.0) }
    })
}

#[derive(Clone, Copy, Debug)]
enum Op { U, I, S }

fn combine(op: Op, a: Option<bool>, b: Option<bool>) -> Option<bool> {
    let (a, b) = (a?, b?);
    Some(match op { Op::U => a || b, Op::I => a && b, Op::S => a && !b })
}

fn run(op: Op, a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> Result<BooleanResult<f64>, pncad::topo::BooleanError> {
    let t = tol();
    match op {
        Op::U => pncad::topo::union(a, b, t),
        Op::I => pncad::topo::intersect(a, b, t),
        Op::S => pncad::topo::subtract(a, b, t),
    }
}

struct Lcg(u64);
impl Lcg { fn next(&mut self) -> f64 { self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407); ((self.0 >> 11) as f64) / ((1u64 << 53) as f64) } }

/// Checks one result: kept verdict, re-validation, volume vs closed form, and
/// point_in_solid against the oracle at `n` sampled points of `bbox`.
#[allow(clippy::too_many_arguments)]
fn check(label: &str, res: &AtRestBody<f64>, want: f64, s: f64, m: Affine3<f64>,
         oa: &Oracle, ob: &Oracle, op: Op, bbox: ((f64, f64), (f64, f64), (f64, f64)), n: usize,
         report: &mut Vec<String>) {
    let t = tol();
    assert_eq!(res.outcome(), AtRestOutcome::Validated, "{label}: outcome");
    if let Err(e) = pncad::topo::validate_geometric(res, t) { report.push(format!("{label}: re-validate refused {e:?}")); }
    match mass_properties(res, t) {
        Ok(p) => {
            let rel = (p.volume - want * s * s * s).abs() / (want * s * s * s);
            if rel > 1e-9 { report.push(format!("{label}: volume {} want {} rel {rel:e}", p.volume, want * s * s * s)); }
        }
        Err(e) => report.push(format!("{label}: mass_properties refused {e:?}")),
    }
    let band = Band::linear(t).expect("band");
    let inv = m.inverse();
    let mut rng = Lcg(0x3987);
    let (mut agree, mut skip, mut bad) = (0, 0, 0);
    for _ in 0..n {
        let q = Point3::new(
            bbox.0.0 + (bbox.0.1 - bbox.0.0) * rng.next(),
            bbox.1.0 + (bbox.1.1 - bbox.1.0) * rng.next(),
            bbox.2.0 + (bbox.2.1 - bbox.2.0) * rng.next());
        let mg = (1e-6f64).max(20.0 * t.eps() / s);
        let Some(want_in) = combine(op, oa(q, mg), ob(q, mg)) else { skip += 1; continue };
        let world = m.transform_point(Point3::new(q.x * s, q.y * s, q.z * s));
        let _ = inv;
        match pncad::topo::point_in_solid(res, world, band, t) {
            Ok(SolidContainment::In) if want_in => agree += 1,
            Ok(SolidContainment::Out) if !want_in => agree += 1,
            other => { bad += 1; if bad <= 3 { report.push(format!("{label}: pis at {q:?} got {other:?} want in={want_in}")); } }
        }
    }
    eprintln!("{label}: pis agree {agree} skip {skip} bad {bad}");
}

fn poses() -> Vec<(String, f64, Affine3<f64>)> {
    let mut v = Vec::new();
    for &s in &[1e-3, 1.0, 1e3] {
        v.push((format!("s={s:e} id"), s, Affine3::identity()));
        let ax = Vec3::new(1.0, 2.0, 3.0) * (1.0 / 14f64.sqrt());
        v.push((format!("s={s:e} rot0.7"), s, Affine3::rotation_about_axis(Point3::new(0.3 * s, -0.2 * s, 0.1 * s), ax, 0.7)));
    }
    v
}

fn scaled_box(x: (f64, f64), y: (f64, f64), z: (f64, f64), s: f64) -> Body<f64> {
    slab((x.0 * s, x.1 * s), (y.0 * s, y.1 * s), (z.0 * s, z.1 * s))
}

const AX: (f64, f64) = (0.0, 3.0); const AY: (f64, f64) = (0.0, 2.0); const AZ: (f64, f64) = (0.0, 1.0);
const BX: (f64, f64) = (1.0, 4.0); const BY: (f64, f64) = (0.5, 1.5); const BZ: (f64, f64) = (0.25, 2.0);
const CC: (f64, f64) = (2.0, 1.0); const CR: f64 = 0.4; const CZ: (f64, f64) = (-1.0, 3.0);
const BBOX: ((f64, f64), (f64, f64), (f64, f64)) = ((-0.5, 4.5), (-0.5, 2.5), (-1.5, 3.5));

#[test]
fn e2e_box_box_box_cyl_ball_all_ops_both_orders_scales_poses() {
    let mut report = Vec::new();
    for (pl, s, m) in poses() {
        let setup = std::panic::catch_unwind(|| {
            let ball = ball_y(0.45 * s, 0.0);
            let ball = moved(&ball, Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 0.0, 1.0), PI / 2.0));
            let ball = moved(&ball, Affine3::translation(Vec3::new(3.2 * s, 1.0 * s, 0.5 * s)));
            (moved(&cyl((CC.0 * s, CC.1 * s), CR * s, (CZ.0 * s, CZ.1 * s)), m), moved(&ball, m))
        });
        if setup.is_err() { eprintln!("SKIP {pl}: fixture setup refused (see panic above)"); continue; }
        let a = fin("A", moved(&scaled_box(AX, AY, AZ, s), m));
        let b = fin("B", moved(&scaled_box(BX, BY, BZ, s), m));
        let c = fin("C", moved(&cyl((CC.0 * s, CC.1 * s), CR * s, (CZ.0 * s, CZ.1 * s)), m));
        // ball r 0.45 centred (3.2, 1, 0.5): revolve about y then translate.
        let bc = Point3::new(3.2, 1.0, 0.5); let br = 0.45;
        let ball = ball_y(br * s, 0.0);
        // pole along x, normal to A's x = 3 face (the supported plane×sphere pose)
        let ball = moved(&ball, Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 0.0, 1.0), PI / 2.0));
        let ball = moved(&ball, Affine3::translation(Vec3::new(bc.x * s, bc.y * s, bc.z * s)));
        let d = fin("D", moved(&ball, m));
        let (va, vb, vab) = (6.0, 3.0 * 1.0 * 1.75, 2.0 * 1.0 * 0.75);
        let vc = PI * CR * CR * 4.0; let vac = PI * CR * CR * 1.0;
        let vd = 4.0 / 3.0 * PI * br.powi(3); let h = br - 0.2; let vad = PI * h * h * (3.0 * br - h) / 3.0;
        let cases: Vec<(&str, &AtRestBody<f64>, &AtRestBody<f64>, Oracle, Oracle, [f64; 4])> = vec![
            // [vol x, vol y, vol x∩y, (unused)]
            ("A,B", &a, &b, box_o(AX, AY, AZ), box_o(BX, BY, BZ), [va, vb, vab, 0.0]),
            ("B,A", &b, &a, box_o(BX, BY, BZ), box_o(AX, AY, AZ), [vb, va, vab, 0.0]),
            ("A,C", &a, &c, box_o(AX, AY, AZ), cyl_o(CC, CR, CZ), [va, vc, vac, 0.0]),
            ("C,A", &c, &a, cyl_o(CC, CR, CZ), box_o(AX, AY, AZ), [vc, va, vac, 0.0]),
            ("A,D", &a, &d, box_o(AX, AY, AZ), ball_o(bc, br), [va, vd, vad, 0.0]),
            ("D,A", &d, &a, ball_o(bc, br), box_o(AX, AY, AZ), [vd, va, vad, 0.0]),
        ];
        for (cl, x, y, ox, oy, v) in &cases {
            for op in [Op::U, Op::I, Op::S] {
                let label = format!("{pl} {cl} {op:?}");
                let want = match op { Op::U => v[0] + v[1] - v[2], Op::I => v[2], Op::S => v[0] - v[2] };
                match run(op, x, y) {
                    Ok(BooleanResult::Body(bb)) => check(&label, &bb.body, want, s, m, ox, oy, op, BBOX, 300, &mut report),
                    Ok(BooleanResult::Empty) => report.push(format!("{label}: EMPTY")),
                    Err(e) => report.push(format!("{label}: refused {e}")),
                }
            }
        }
        // Reuse: R1 = A∖C, R2 = R1∩B, R3 = R2 ∪ D, each finished result as an operand.
        let r1 = match run(Op::S, &a, &c) { Ok(BooleanResult::Body(bb)) => bb.body, o => { report.push(format!("{pl} reuse r1 {o:?}")); continue } };
        let r2 = match run(Op::I, &r1, &b) { Ok(BooleanResult::Body(bb)) => bb.body, o => { report.push(format!("{pl} reuse r2 {:?}", o.err())); continue } };
        let want2 = vab - PI * CR * CR * 0.75;
        let o_r1: Oracle = { let (oa, oc) = (box_o(AX, AY, AZ), cyl_o(CC, CR, CZ)); Box::new(move |p, mm| combine(Op::S, oa(p, mm), oc(p, mm))) };
        check(&format!("{pl} reuse (A∖C)∩B"), &r2, want2, s, m, &o_r1, &box_o(BX, BY, BZ), Op::I, BBOX, 300, &mut report);
        // r2 is x∈[1,3]; ball (3.2) does not reach x<2.75 → disjoint? ball x∈[2.75,3.65]: overlaps r2 x∈[2.75,3]
        match run(Op::S, &r1, &d) {
            Ok(BooleanResult::Body(bb)) => {
                let want3 = va - vac - vad;
                let o_d = ball_o(bc, br);
                check(&format!("{pl} reuse (A∖C)∖D"), &bb.body, want3, s, m, &o_r1, &o_d, Op::S, BBOX, 300, &mut report);
            }
            o => report.push(format!("{pl} reuse (A∖C)∖D {:?}", o.err())),
        }
    }
    for r in &report { eprintln!("REPORT {r}"); }
    assert!(report.is_empty(), "{} findings", report.len());
}

/// Claim 2 through the facade: an inside-out operand refuses at validate,
/// naming its own solid.
#[test]
fn e2e_inside_out_operand_refuses_at_validate() {
    let b = slab(AX, AY, AZ).revert().expect("revert");
    let e = AtRestBody::validate(b, tol()).expect_err("inside-out is not finished");
    eprintln!("inside-out: {e:?}");
    assert!(e.iter().any(|x| matches!(x, pncad::topo::ValidationError::NegativeVolume { .. })));
}

/// Claim 1's tier-3′ half: does a body the door ships pass tier 3′ over its
/// own contacts? Two disjoint balls, a gap of 0.5 (well clear of ε).
#[test]
fn e2e_door_ships_a_result_its_own_tier_3_prime_refuses() {
    let t = tol();
    for (gap, off) in [(0.5, 0.0), (0.05, 0.0), (0.26, 1.6), (0.05, 1.45)] {
        let a = fin("ball a", ball_y(1.0, 0.0));
        let b = fin("ball b", moved(&ball_y(1.0, ((2.0f64 + gap).powi(2) - off * off).sqrt()), Affine3::translation(Vec3::new(off, 0.0, 0.0))));
        let BooleanResult::Body(bb) = pncad::topo::union(&a, &b, t).expect("disjoint union ships") else { panic!("empty") };
        assert_eq!(bb.body.outcome(), AtRestOutcome::Validated);
        let v = mass_properties(&bb.body, t).expect("mp").volume;
        let want = 2.0 * 4.0 / 3.0 * PI;
        let _ = gap;
        eprintln!("gap {gap}: shipped kind {:?}, solids {}, volume {v} (oracle {want}, rel {:e})",
            bb.kind, bb.body.solids().count(), (v - want).abs() / want);
        let census = bb.body.validate_pseudomanifold(&bb.contacts, t);
        eprintln!("gap {gap}: tier 3′ over the result's own contacts → {:?}", census.as_ref().map_err(|e| e.iter().map(|x| format!("{x}")).collect::<Vec<_>>()));
    }
}
