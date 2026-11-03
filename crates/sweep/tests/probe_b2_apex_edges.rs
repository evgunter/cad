//! Reviewer probe (PR 6424): box edges through, ending near, and passing
//! near a cone's apex at generic directions, every op, both orders, both
//! cone forms, held to a numerical volume oracle and a membership lattice.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::wrap_tip::{NORTH, one_wrap_edge};
use core::f64::consts::PI;
use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{finished, revolved_about_y};
use topo::{BooleanResult, SolidContainment, intersect, point_in_solid, subtract, union};

fn tol() -> Tol {
    Tol::witness()
}

fn cone(alpha: f64) -> topo::Body<f64> {
    let r = alpha.to_radians().tan();
    revolved_about_y(
        vec![(Point2::new(0.0, 0.0), 0.0), (Point2::new(r, 0.0), 0.0), (Point2::new(0.0, 1.0), 0.0)],
        Revolution::Full,
        tol(),
    )
}

fn forms(alpha: f64) -> [(&'static str, topo::Body<f64>); 2] {
    [
        ("tipped", one_wrap_edge(cone(alpha), true).0),
        ("revolved", crate::revolve_common::needle(alpha, None, tol()).into_body()),
    ]
}

type Bx = ((f64, f64), (f64, f64), (f64, f64));

/// Volume of cone ∩ m(box), numerically: slices in y, a fine disk polygon
/// clipped by the box's six half-spaces.
fn oracle(alpha: f64, bx: Bx, m: Affine3<f64>) -> f64 {
    let r = alpha.to_radians().tan();
    let lin = |v: Vec3<f64>| m.transform_vec(v);
    let o = m.transform_point(Point3::origin());
    // Half-spaces n·p ≤ d in world coordinates.
    let mut hs = Vec::new();
    for (axis, (lo, hi)) in [(Vec3::new(1.0, 0.0, 0.0), bx.0), (Vec3::new(0.0, 1.0, 0.0), bx.1), (Vec3::new(0.0, 0.0, 1.0), bx.2)] {
        let n = lin(axis);
        let base = n.dot(o - Point3::origin());
        hs.push((n, base + hi));
        hs.push((-n, -(base + lo)));
    }
    let nseg = 1024usize;
    let fix = (2.0 * PI / (nseg as f64 * (2.0 * PI / nseg as f64).sin())).sqrt();
    let slices = 2000usize;
    let mut vol = 0.0;
    for k in 0..slices {
        let y = (k as f64 + 0.5) / slices as f64;
        let rho = r * (1.0 - y) * fix;
        let mut poly: Vec<(f64, f64)> = (0..nseg)
            .map(|i| {
                let t = 2.0 * PI * i as f64 / nseg as f64;
                (rho * t.cos(), rho * t.sin())
            })
            .collect();
        for &(n, d) in &hs {
            let dd = d - n.y * y;
            let f = |p: (f64, f64)| n.x * p.0 + n.z * p.1 - dd;
            let mut out = Vec::with_capacity(poly.len() + 4);
            for i in 0..poly.len() {
                let (p, q) = (poly[i], poly[(i + 1) % poly.len()]);
                let (fp, fq) = (f(p), f(q));
                if fp <= 0.0 {
                    out.push(p);
                }
                if (fp <= 0.0) != (fq <= 0.0) {
                    let s = fp / (fp - fq);
                    out.push((p.0 + s * (q.0 - p.0), p.1 + s * (q.1 - p.1)));
                }
            }
            poly = out;
            if poly.is_empty() {
                break;
            }
        }
        let mut a = 0.0;
        for i in 0..poly.len() {
            let (p, q) = (poly[i], poly[(i + 1) % poly.len()]);
            a += p.0 * q.1 - q.0 * p.1;
        }
        vol += (a / 2.0).abs() / slices as f64;
    }
    vol
}

fn cone_region(r: f64) -> impl Fn(Point3<f64>) -> f64 {
    move |p: Point3<f64>| (r * (1.0 - p.y) - p.x.hypot(p.z)).min(p.y).min(1.0 - p.y)
}

fn box_region(bx: Bx, m: Affine3<f64>) -> impl Fn(Point3<f64>) -> f64 {
    let back = m.inverse();
    move |p| {
        let q = back.transform_point(p);
        (q.x - bx.0.0).min(bx.0.1 - q.x).min(q.y - bx.1.0).min(bx.1.1 - q.y).min(q.z - bx.2.0).min(bx.2.1 - q.z)
    }
}

/// Runs every op both orders; returns (built ok, refused list, wrong list).
fn census(what: &str, alpha: f64, bx: Bx, m: Affine3<f64>, out: &mut Vec<String>, wrong: &mut Vec<String>) {
    let r = alpha.to_radians().tan();
    let vc = PI * r * r / 3.0;
    let vb = (bx.0.1 - bx.0.0) * (bx.1.1 - bx.1.0) * (bx.2.1 - bx.2.0);
    let share = oracle(alpha, bx, m);
    let b = finished(
        "box",
        topo::transform_rigid(&topo::test_support::brick(bx.0, bx.1, bx.2, tol()), &geom_core::test_support::rigid(&m, tol()), tol()).unwrap(),
        tol(),
    );
    let cr = cone_region(r);
    let br = box_region(bx, m);
    for (form, body) in forms(alpha) {
        let c = finished("c", body, tol());
        for (order, (x, y), (vx, vy), cone_first) in [("cone first", (&c, &b), (vc, vb), true), ("box first", (&b, &c), (vb, vc), false)] {
            let rx = |p: Point3<f64>| if cone_first { cr(p) } else { br(p) };
            let ry = |p: Point3<f64>| if cone_first { br(p) } else { cr(p) };
            let ops: [(&str, Box<dyn Fn() -> Result<BooleanResult<f64>, topo::BooleanError>>, f64, Box<dyn Fn(Point3<f64>) -> f64>); 4] = [
                ("∩", Box::new(|| intersect(x, y, tol())), share, Box::new(|p| rx(p).min(ry(p)))),
                ("∪", Box::new(|| union(x, y, tol())), vx + vy - share, Box::new(|p| rx(p).max(ry(p)))),
                ("x∖y", Box::new(|| subtract(x, y, tol())), vx - share, Box::new(|p| rx(p).min(-ry(p)))),
                ("y∖x", Box::new(|| subtract(y, x, tol())), vy - share, Box::new(|p| ry(p).min(-rx(p)))),
            ];
            for (op, run, want, region) in ops {
                let label = format!("{what} {form} {order} {op}");
                let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| run()));
                match res {
                    Err(_) => wrong.push(format!("{label}: PANIC")),
                    Ok(Err(e)) => out.push(format!("{label}: REFUSE {:?}", e).chars().take(200).collect()),
                    Ok(Ok(BooleanResult::Empty)) => {
                        if want.abs() > 1e-6 { wrong.push(format!("{label}: EMPTY want {want}")) } else { out.push(format!("{label}: ok-empty")) }
                    }
                    Ok(Ok(BooleanResult::Body(body))) => {
                        let mp = topo::mass_properties(&body.body, tol(), topo::test_support::pinned_resolution(tol()));
                        let v = match mp.map(|m| m.met()) {
                            Ok(Ok(e)) => e.volume,
                            other => {
                                wrong.push(format!("{label}: BUILT but volume unmeasurable {other:?}").chars().take(300).collect());
                                continue;
                            }
                        };
                        if (v - want).abs() > 2e-5 * want.abs().max(1.0) {
                            wrong.push(format!("{label}: VOLUME {v} want {want}"));
                            continue;
                        }
                        if let Err(e) = topo::validate_geometric(&body.body, tol()) {
                            wrong.push(format!("{label}: TIER3 {e:?}").chars().take(300).collect());
                            continue;
                        }
                        let band = Band::linear(tol()).unwrap();
                        let n = 6;
                        let mut bad = 0;
                        for i in 0..n {
                            for j in 0..n {
                                for k in 0..n {
                                    let f = |t: usize, o: f64| (t as f64 + o) / (n as f64 - 0.63);
                                    let p = Point3::new(-2.5 + 5.0 * f(i, 0.11), -0.25 + 2.1 * f(j, 0.23), -2.5 + 5.0 * f(k, 0.07));
                                    let s = region(p);
                                    if s.abs() < 5e-3 {
                                        continue;
                                    }
                                    let want = if s > 0.0 { SolidContainment::In } else { SolidContainment::Out };
                                    if point_in_solid(&body.body, p, band, tol()) != Ok(want) {
                                        bad += 1;
                                    }
                                }
                            }
                        }
                        if bad > 0 {
                            wrong.push(format!("{label}: MEMBERSHIP {bad} points"));
                        } else {
                            out.push(format!("{label}: ok"));
                        }
                    }
                }
            }
        }
    }
}

fn report(name: &str, out: Vec<String>, wrong: Vec<String>) {
    let ok = out.iter().filter(|s| s.ends_with(": ok") || s.ends_with("ok-empty")).count();
    let refused: Vec<_> = out.iter().filter(|s| s.contains("REFUSE")).collect();
    eprintln!("PROBE {name}: ops {} ok {ok} refused {} wrong {}", out.len() + wrong.len(), refused.len(), wrong.len());
    for s in &refused {
        eprintln!("  R {s}");
    }
    for s in &wrong {
        eprintln!("  W {s}");
    }
    assert!(wrong.is_empty(), "{name}: {} wrong", wrong.len());
}

fn lcg(seed: &mut u64) -> f64 {
    *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    ((*seed >> 11) as f64) / ((1u64 << 53) as f64)
}

fn rot(axis: Vec3<f64>, angle: f64, shift: Vec3<f64>) -> Affine3<f64> {
    Affine3::translation(NORTH - Point3::origin() + shift) * Affine3::rotation_about_axis(Point3::origin(), axis, angle)
}

/// An edge across the apex (local z through the origin, box's top y=0 and side x=0 holding it).
const ACROSS: Bx = ((0.0, 3.0), (-3.0, 0.0), (-3.0, 3.0));

fn generic(alpha: f64, seed0: u64) {
    let (mut out, mut wrong) = (Vec::new(), Vec::new());
    let mut seed = seed0;
    for i in 0..6 {
        let (u, v, w) = (lcg(&mut seed) * 2.0 - 1.0, lcg(&mut seed) * 2.0 - 1.0, lcg(&mut seed) * 2.0 - 1.0);
        let axis = Vec3::new(u, v, w);
        let angle = lcg(&mut seed) * 2.0 * PI;
        census(&format!("{alpha}° generic#{i}"), alpha, ACROSS, rot(axis, angle, Vec3::zero()), &mut out, &mut wrong);
    }
    report(&format!("generic {alpha}"), out, wrong);
}

#[test]
fn generic_30() {
    generic(30.0, 1);
}
#[test]
fn generic_45() {
    generic(45.0, 2);
}
#[test]
fn generic_60() {
    generic(60.0, 3);
}

/// Edges in a meridian plane (rotation about local x), at elevations
/// inside, outside, and near a ruling.
fn meridian(alpha: f64) {
    let (mut out, mut wrong) = (Vec::new(), Vec::new());
    for e in [10.0f64, 35.0, 70.0, 85.0, -20.0, 90.0 - alpha + 3.0, 90.0 - alpha - 3.0, 90.0 - alpha] {
        for turn in [0.0, 1.1] {
            let m = Affine3::rotation_about_axis(NORTH, Vec3::new(0.0, 1.0, 0.0), turn) * rot(Vec3::new(1.0, 0.0, 0.0), e.to_radians(), Vec3::zero());
            census(&format!("{alpha}° meridian e{e} turn{turn}"), alpha, ACROSS, m, &mut out, &mut wrong);
        }
    }
    report(&format!("meridian {alpha}"), out, wrong);
}

#[test]
fn meridian_30() {
    meridian(30.0);
}
#[test]
fn meridian_45() {
    meridian(45.0);
}
#[test]
fn meridian_60() {
    meridian(60.0);
}

/// The across box shifted off the apex by k·ε (perpendicular to the edge),
/// and a corner box whose edge ends k·ε from the apex.
fn near(alpha: f64) {
    let (mut out, mut wrong) = (Vec::new(), Vec::new());
    let eps = tol().eps();
    let corner: Bx = ((0.0, 3.0), (-3.0, 0.0), (0.0, 3.0));
    for k in [0.3, 3.0, 30.0, 3e3, 3e5] {
        for (bxn, bx) in [("across", ACROSS), ("corner", corner)] {
            for (dn, d) in [("up", Vec3::new(0.3, 1.0, 0.2)), ("side", Vec3::new(1.0, -0.4, 0.7))] {
                let shift = d * (k * eps / d.norm());
                let m = rot(Vec3::new(0.3, 0.2, 1.0), 0.7, shift);
                census(&format!("{alpha}° {bxn} k{k} {dn}"), alpha, bx, m, &mut out, &mut wrong);
            }
        }
    }
    report(&format!("near {alpha}"), out, wrong);
}

#[test]
fn near_30() {
    near(30.0);
}
#[test]
fn near_45() {
    near(45.0);
}
#[test]
fn near_60() {
    near(60.0);
}

/// The PR's two boxes at turns that put the wrap seam on a box side.
fn seam_turns(alpha: f64) {
    let (mut out, mut wrong) = (Vec::new(), Vec::new());
    let across: Bx = ((0.0, 2.0), (-0.45, 0.0), (-2.0, 2.3));
    let along: Bx = ((0.0, 2.0), (-0.45, 0.6), (0.0, 2.1));
    for (bn, bx) in [("across", across), ("along", along)] {
        for turn in [0.0, 1e-7, PI / 2.0, PI, 1.5 * PI, -PI / 2.0, 2.0 * PI - 1e-7] {
            census(&format!("{alpha}° seam {bn} turn{turn:.3e}"), alpha, bx, rot(Vec3::new(0.0, 1.0, 0.0), turn, Vec3::zero()), &mut out, &mut wrong);
        }
    }
    report(&format!("seam {alpha}"), out, wrong);
}

#[test]
fn seam_30() {
    seam_turns(30.0);
}
#[test]
fn seam_45() {
    seam_turns(45.0);
}
#[test]
fn seam_60() {
    seam_turns(60.0);
}
