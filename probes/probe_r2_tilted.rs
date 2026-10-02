//! Reviewer probe (PR 3817, lane reach-dual3817-r2): tilted sphere
//! pairs, widened — rigidly re-posed whole scenes, scales, near
//! tangencies, pole-through sections, results reused as operands, and
//! `point_in_solid` sampled against an analytic oracle. Every body that
//! builds is held to the lens closed form (own derivation below); a
//! refusal is recorded with its payload.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Mat3, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::{Body, BooleanOp};

fn ball(r: f64, c: Vec3<f64>, rot: Mat3<f64>) -> Body<f64> {
    let b = revolved_about_y(
        vec![(Point2::new(0.0, -r), 1.0), (Point2::new(0.0, r), 0.0)],
        Revolution::Full,
        Tol::witness(),
    );
    topo::transform_rigid(&b, &Affine3::from_parts(rot, c), Tol::witness()).unwrap()
}

/// The lens as two caps beyond the radical plane (closed form; an
/// earlier Simpson slice oracle was too coarse at the sqrt ends).
fn lens(r1: f64, r2: f64, d: f64) -> f64 {
    if d >= r1 + r2 {
        return 0.0;
    }
    let x = (d * d + r1 * r1 - r2 * r2) / (2.0 * d);
    let cap = |r: f64, h: f64| PI * h * h * (3.0 * r - h) / 3.0;
    cap(r1, r1 - x) + cap(r2, r2 - (d - x))
}

fn run(op: BooleanOp, a: &Body<f64>, b: &Body<f64>) -> Result<topo::BooleanResult<f64>, topo::BooleanError> {
    match op {
        BooleanOp::Union => topo::boolean::union(a, b, Tol::witness()),
        BooleanOp::Intersect => topo::boolean::intersect(a, b, Tol::witness()),
        BooleanOp::Subtract => topo::boolean::subtract(a, b, Tol::witness()),
    }
}

#[derive(Default)]
struct Tally {
    built_ok: usize,
    refused: Vec<String>,
    wrong: Vec<String>,
}

fn inside(p: Point3<f64>, c: Vec3<f64>, r: f64) -> bool {
    let d = Vec3::new(p.x - c.x, p.y - c.y, p.z - c.z);
    d.norm() < r
}

#[allow(clippy::too_many_arguments)]
fn pose(t: &mut Tally, label: &str, r1: f64, c1: Vec3<f64>, rot1: Mat3<f64>, r2: f64, c2: Vec3<f64>, rot2: Mat3<f64>) {
    let a = ball(r1, c1, rot1);
    let b = ball(r2, c2, rot2);
    let d = (c2 - c1).norm();
    let sh = lens(r1, r2, d);
    let (va, vb) = (4.0 / 3.0 * PI * r1.powi(3), 4.0 / 3.0 * PI * r2.powi(3));
    let ops: [(&str, BooleanOp, &Body<f64>, &Body<f64>, f64, fn(bool, bool) -> bool); 4] = [
        ("U", BooleanOp::Union, &a, &b, va + vb - sh, |x, y| x || y),
        ("I", BooleanOp::Intersect, &a, &b, sh, |x, y| x && y),
        ("A-B", BooleanOp::Subtract, &a, &b, va - sh, |x, y| x && !y),
        ("B-A", BooleanOp::Subtract, &b, &a, vb - sh, |x, y| y && !x),
    ];
    for (name, op, x, y, want, member) in ops {
        let lab = format!("{label} {name}");
        match run(op, x, y) {
            Err(e) => {
                let s = format!("{lab}: refused {e:?}");
                println!("{}", &s[..s.len().min(260)]);
                t.refused.push(s);
            }
            Ok(out) => {
                let Some(bd) = out.body() else {
                    println!("{lab}: EMPTY (want {want})");
                    if want > 1e-12 { t.wrong.push(format!("{lab}: empty, want {want}")); }
                    continue;
                };
                let body = &bd.body;
                let v1 = topo::validate(body);
                let v2 = topo::validate_closed(body);
                let v3 = topo::validate_geometric(body, Tol::witness());
                let mp = topo::mass_properties(body, Tol::witness());
                match (&v1, &v2, &v3, &mp) {
                    (Ok(()), Ok(()), Ok(()), Ok(p)) => {
                        let err = (p.volume - want).abs() / want.max(1e-300);
                        println!("{lab}: vol {:.15e} want {:.15e} rel {err:.2e} pad {}", p.volume, want, p.volume_pad);
                        if (p.volume - want).abs() > 1e-9 * want.max(1.0) {
                            t.wrong.push(format!("{lab}: volume {} want {want}", p.volume));
                        } else {
                            t.built_ok += 1;
                        }
                    }
                    _ => {
                        let s = format!("{lab}: built but validate {v1:?} closed {v2:?} geom {v3:?} mp {:?}", mp.as_ref().map(|p| p.volume));
                        println!("{}", &s[..s.len().min(400)]);
                        t.wrong.push(s);
                    }
                }
                // point_in_solid sampled against the oracle (first refusal recorded).
                let band = geom_core::Band::linear(Tol::witness()).unwrap();
                let mut seed = 99u64;
                let mut rnd = || {
                    seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
                    ((seed >> 11) as f64) / ((1u64 << 53) as f64)
                };
                let lo = Vec3::new(c1.x.min(c2.x) - r1.max(r2), c1.y.min(c2.y) - r1.max(r2), c1.z.min(c2.z) - r1.max(r2));
                let span = 2.0 * r1.max(r2) + (c2 - c1).norm();
                let mut mism = 0;
                for k in 0..24 {
                    let p = Point3::new(lo.x + span * rnd(), lo.y + span * rnd(), lo.z + span * rnd());
                    let truth = member(inside(p, c1, r1), inside(p, c2, r2));
                    match topo::point_in_solid(body, p, band, Tol::witness()) {
                        Ok(topo::SolidContainment::In) if !truth => mism += 1,
                        Ok(topo::SolidContainment::Out) if truth => mism += 1,
                        Ok(_) => {}
                        Err(e) => {
                            if k == 0 { println!("{lab}: point_in_solid refused {e:?}"); }
                            break;
                        }
                    }
                }
                if mism > 0 {
                    println!("{lab}: POINT_IN_SOLID mismatches {mism}");
                    t.wrong.push(format!("{lab}: point_in_solid wrong x{mism}"));
                }
            }
        }
    }
}

#[test]
fn probe_r2_tilted_sphere_pairs_widened() {
    let id = Mat3::identity();
    let base = Vec3::new(2.0, 2.0, 0.5);
    let mut t = Tally::default();
    let q = Mat3::rotation_about(Vec3::new(0.3, -0.8, 0.52).normalize(), 1.1);
    // Rigid re-posing of a whole seam-plane scene: the pair's relation is
    // unchanged, both charts rotate together.
    for (lab, s) in [("x1", 1.0), ("x1e-3", 1e-3), ("x1e3", 1e3)] {
        for (pl, r2, off) in [
            ("eq-x", 1.0, Vec3::new(1.4, 0.0, 0.0)),
            ("r.6", 0.6, Vec3::new(0.9, 0.0, 0.0)),
            ("r.5xy", 0.5, Vec3::new(0.8, 0.4, 0.0)),
        ] {
            pose(&mut t, &format!("rot-{lab}-{pl}"), s, q * (base * s), q, r2 * s, q * ((base + off) * s), q);
        }
    }
    // Pole-through: equal unit balls at offset (1,1,0): the radical plane
    // passes A's north pole and B's south pole. And just off it.
    for (pl, off) in [
        ("pole-through", Vec3::new(1.0, 1.0, 0.0)),
        ("pole-near+", Vec3::new(1.0, 1.0 + 1e-6, 0.0)),
        ("pole-near-", Vec3::new(1.0, 1.0 - 1e-6, 0.0)),
        ("pole-1e-3", Vec3::new(1.0, 1.001, 0.0)),
        ("pole-1e-2", Vec3::new(1.0, 1.01, 0.0)),
        ("pole-5e-2", Vec3::new(1.0, 1.05, 0.0)),
        ("pole-2e-1", Vec3::new(1.0, 1.2, 0.0)),
    ] {
        pose(&mut t, pl, 1.0, base, id, 1.0, base + off, id);
    }
    // Nearly tangent (outer) and nearly internally tangent pairs.
    for (pl, r2, dx) in [
        ("near-ext-1e-3", 0.5, 1.5 - 1e-3),
        ("near-ext-1e-6", 0.5, 1.5 - 1e-6),
        ("near-int-1e-3", 0.5, 0.5 + 1e-3),
        ("near-int-1e-6", 0.5, 0.5 + 1e-6),
        ("tiny", 0.02, 1.0),
        ("big-B", 3.0, 2.5),
    ] {
        pose(&mut t, pl, 1.0, base, id, r2, base + Vec3::new(dx, 0.0, 0.0), id);
    }
    // Each ball rotated differently, offset in a plane that is the seam
    // plane of neither chart -> pierce ring expected (typed).
    let q2 = Mat3::rotation_about(Vec3::new(1.0, 0.0, 0.0), 0.4);
    pose(&mut t, "diff-charts", 1.0, base, q, 0.8, base + Vec3::new(1.1, 0.2, -0.3), q2);
    // Results reused as an operand: (A ∖ B) ∪ C, C another tilted ball.
    {
        let a = ball(1.0, base, id);
        let b = ball(1.0, base + Vec3::new(1.4, 0.0, 0.0), id);
        let ab = topo::boolean::subtract(&a, &b, Tol::witness()).unwrap();
        let ab = ab.body().unwrap().body.clone();
        let c = ball(0.5, base + Vec3::new(-1.1, 0.0, 0.0), id);
        for (n, op) in [("reuse-U", BooleanOp::Union), ("reuse-I", BooleanOp::Intersect), ("reuse-S", BooleanOp::Subtract)] {
            match run(op, &ab, &c) {
                Ok(o) => {
                    let v = o.body().map(|b| topo::mass_properties(&b.body, Tol::witness()).map(|p| p.volume));
                    println!("{n}: built, volume {v:?}");
                }
                Err(e) => println!("{n}: refused {e:?}"),
            }
        }
    }
    println!("BUILT-OK {} REFUSED {} WRONG {}", t.built_ok, t.refused.len(), t.wrong.len());
    for w in &t.wrong {
        println!("WRONG: {}", &w[..w.len().min(300)]);
    }
    assert!(t.wrong.is_empty());
}

/// `topo::split` of a y-poled ball by planes tilted against its chart:
/// the split lane's wall side reaches `select_arc_by_run_side` too
/// (chord_spec is shared). Each half against the cap closed form.
#[test]
fn probe_r2_split_ball_by_tilted_plane() {
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    let c = Vec3::new(0.3, -0.2, 0.1);
    let mut wrong = Vec::new();
    for (lab, n, off) in [
        ("x-normal", Vec3::new(1.0, 0.0, 0.0), 0.3),
        ("tilt30-seam", Vec3::new(0.5, 0.866_025_403_784_438_6, 0.0), 0.2),
        ("tilt-offseam", Vec3::new(0.6, 0.3, 0.742), 0.1),
        ("tilt-neg", Vec3::new(-0.7, 0.2, 0.0), -0.5),
        ("z-normal", Vec3::new(0.0, 0.0, 1.0), 0.4),
    ].map(|(l, n, o): (&str, Vec3<f64>, f64)| (l, n.normalize(), o)) {
        let b = ball(1.0, c, Mat3::identity());
        let origin = Point3::new(c.x + n.x * off, c.y + n.y * off, c.z + n.z * off);
        let plane = topo::SplitPlane { origin, normal: geom_core::UnitVec3::new(n, "probe", band).unwrap() };
        let h = 1.0 - off;
        let above_want = PI * h * h * (3.0 - h) / 3.0;
        let below_want = 4.0 / 3.0 * PI - above_want;
        match topo::split(&b, &plane, Tol::witness()) {
            Err(e) => println!("split {lab}: refused {e:?}"),
            Ok(r) => {
                for (side, part, want) in [("above", r.above.body(), above_want), ("below", r.below.body(), below_want)] {
                    let Some(body) = part else { println!("split {lab} {side}: empty"); continue };
                    let v3 = topo::validate_geometric(body, Tol::witness());
                    let mp = topo::mass_properties(body, Tol::witness()).map(|p| p.volume);
                    println!("split {lab} {side}: tier3 {v3:?} vol {mp:?} want {want}");
                    if let Ok(v) = mp { if (v - want).abs() > 1e-9 { wrong.push(format!("{lab} {side} {v} {want}")); } }
                }
            }
        }
    }
    assert!(wrong.is_empty(), "{wrong:?}");
}
