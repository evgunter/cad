//! Reviewer probes for PR #3978 (dual lane r1): carrier touches swept
//! across face edges, three scales, a rotation, and results reused, each
//! built body checked against an analytic CSG oracle by Monte Carlo
//! `point_in_solid` and a closed-form volume. A refusal is recorded, a
//! wrong body panics.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use crate::common::approx::band;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane};
use sweep::{ExtrudeSide, Extrusion, Revolution, extrude};
use topo::{Body, BooleanError, BooleanOp, SolidContainment, point_in_solid};

/// An analytic solid: `depth > 0` inside (a CSG lower bound on the
/// distance to the boundary, good enough to skip near-boundary samples).
#[derive(Clone)]
enum Csg {
    Ball(Vec3<f64>, f64),
    Brick([f64; 3], [f64; 3]),
    /// axis point, unit direction, radius, along-axis range
    Rod(Vec3<f64>, Vec3<f64>, f64, (f64, f64)),
    U(Box<Csg>, Box<Csg>),
    I(Box<Csg>, Box<Csg>),
    D(Box<Csg>, Box<Csg>),
}

impl Csg {
    fn depth(&self, p: Vec3<f64>) -> f64 {
        match self {
            Csg::Ball(c, r) => r - (p - *c).norm(),
            Csg::Brick(lo, hi) => {
                let a = [p.x, p.y, p.z];
                (0..3)
                    .map(|i| (a[i] - lo[i]).min(hi[i] - a[i]))
                    .fold(f64::INFINITY, f64::min)
            }
            Csg::Rod(o, d, r, (t0, t1)) => {
                let w = p - *o;
                let t = w.dot(*d);
                let radial = (w - *d * t).norm();
                (r - radial).min(t - t0).min(t1 - t)
            }
            Csg::U(a, b) => a.depth(p).max(b.depth(p)),
            Csg::I(a, b) => a.depth(p).min(b.depth(p)),
            Csg::D(a, b) => a.depth(p).min(-b.depth(p)),
        }
    }
    fn bbox(&self) -> (Vec3<f64>, Vec3<f64>) {
        let mn = |a: Vec3<f64>, b: Vec3<f64>| Vec3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
        let mx = |a: Vec3<f64>, b: Vec3<f64>| Vec3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));
        match self {
            Csg::Ball(c, r) => (*c - Vec3::new(*r, *r, *r), *c + Vec3::new(*r, *r, *r)),
            Csg::Brick(lo, hi) => (Vec3::new(lo[0], lo[1], lo[2]), Vec3::new(hi[0], hi[1], hi[2])),
            Csg::Rod(o, d, r, (t0, t1)) => {
                let (p, q) = (*o + *d * *t0, *o + *d * *t1);
                let rr = Vec3::new(*r, *r, *r);
                (mn(p, q) - rr, mx(p, q) + rr)
            }
            Csg::U(a, b) => {
                let ((a0, a1), (b0, b1)) = (a.bbox(), b.bbox());
                (mn(a0, b0), mx(a1, b1))
            }
            Csg::I(a, b) => {
                let ((a0, a1), (b0, b1)) = (a.bbox(), b.bbox());
                (mx(a0, b0), mn(a1, b1))
            }
            Csg::D(a, _) => a.bbox(),
        }
    }
    fn of(op: BooleanOp, a: &Csg, b: &Csg) -> Csg {
        let (a, b) = (Box::new(a.clone()), Box::new(b.clone()));
        match op {
            BooleanOp::Union => Csg::U(a, b),
            BooleanOp::Intersect => Csg::I(a, b),
            BooleanOp::Subtract => Csg::D(a, b),
        }
    }
}

struct Shape {
    body: Body<f64>,
    csg: Csg,
}

fn rot() -> Affine3<f64> {
    Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.3, -0.5, 0.8), 0.7)
}

fn ball(r: f64, c: Vec3<f64>) -> Body<f64> {
    let b = sweep::test_support::revolved_about_y(
        vec![(Point2::new(0.0, -r), 1.0), (Point2::new(0.0, r), 0.0)],
        Revolution::Full,
        Tol::witness(),
    );
    topo::transform_rigid(&b, &Affine3::translation(c), Tol::witness()).unwrap()
}

fn rod_z(r: f64, (x, y): (f64, f64), (z0, z1): (f64, f64)) -> Body<f64> {
    let tol = Tol::witness();
    let lp = profile::circle(Point2::new(x, y), r, tol).unwrap();
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp.into()]).validate(tol).unwrap();
    extrude(
        &profile,
        Extrusion::Distance {
            depth: z1 - z0,
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .unwrap()
    .body
}

/// A rod about an arbitrary axis: a z rod at the origin, rigidly carried.
fn rod(r: f64, o: Vec3<f64>, d: Vec3<f64>, (t0, t1): (f64, f64)) -> Shape {
    let z = Vec3::new(0.0, 0.0, 1.0);
    let axis = z.cross(d);
    let ang = z.dot(d).clamp(-1.0, 1.0).acos();
    let turn = if axis.norm() < 1e-15 {
        if z.dot(d) > 0.0 {
            Affine3::identity()
        } else {
            Affine3::rotation_about_axis(Point3::origin(), Vec3::new(1.0, 0.0, 0.0), PI)
        }
    } else {
        Affine3::rotation_about_axis(Point3::origin(), axis / axis.norm(), ang)
    };
    let body = topo::transform_rigid(
        &rod_z(r, (0.0, 0.0), (t0, t1)),
        &(Affine3::translation(o) * turn),
        Tol::witness(),
    )
    .unwrap();
    Shape {
        body,
        csg: Csg::Rod(o, d, r, (t0, t1)),
    }
}

fn sball(r: f64, c: Vec3<f64>) -> Shape {
    Shape {
        body: ball(r, c),
        csg: Csg::Ball(c, r),
    }
}

fn brick(lo: [f64; 3], hi: [f64; 3]) -> Shape {
    Shape {
        body: sweep::test_support::brick(
            (lo[0], hi[0]),
            (lo[1], hi[1]),
            (lo[2], hi[2]),
            Tol::witness(),
        ),
        csg: Csg::Brick(lo, hi),
    }
}

fn boolean(op: BooleanOp, a: &Body<f64>, b: &Body<f64>) -> Result<Option<Body<f64>>, BooleanError> {
    let r = match op {
        BooleanOp::Union => topo::boolean::union(a, b, Tol::witness()),
        BooleanOp::Intersect => topo::boolean::intersect(a, b, Tol::witness()),
        BooleanOp::Subtract => topo::boolean::subtract(a, b, Tol::witness()),
    }?;
    Ok(r.body().map(|b| b.body.clone()))
}

fn combine(op: BooleanOp, a: &Shape, b: &Shape) -> Shape {
    Shape {
        body: boolean(op, &a.body, &b.body)
            .unwrap_or_else(|e| panic!("fixture {op:?}: {e:?}"))
            .expect("fixture nonempty"),
        csg: Csg::of(op, &a.csg, &b.csg),
    }
}

fn carry(s: &Shape, m: &Affine3<f64>, inv: &Affine3<f64>) -> Shape {
    // The oracle is evaluated in the original frame: points are mapped back.
    let _ = inv;
    Shape {
        body: topo::transform_rigid(&s.body, m, Tol::witness()).unwrap(),
        csg: s.csg.clone(),
    }
}

/// Deterministic LCG in [0, 1).
fn rng(state: &mut u64) -> f64 {
    *state = state
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1_442_695_040_888_963_407);
    (*state >> 11) as f64 / (1u64 << 53) as f64
}

/// Checks `body` against `want` by Monte Carlo over `[lo, hi]` (in the
/// oracle's frame; `to_body` carries a sample into the body's frame).
/// Returns a mismatch description or None.
fn mc_check(
    body: &Body<f64>,
    want: &Csg,
    (lo, hi): (Vec3<f64>, Vec3<f64>),
    to_body: &Affine3<f64>,
    skip: f64,
    n: usize,
) -> Option<String> {
    let mut s = 0x9e37_79b9_7f4a_7c15u64;
    let (mut inside, mut checked, mut bad) = (0usize, 0usize, Vec::new());
    let mut undecided = 0usize;
    for _ in 0..n {
        let p = Vec3::new(
            lo.x + (hi.x - lo.x) * rng(&mut s),
            lo.y + (hi.y - lo.y) * rng(&mut s),
            lo.z + (hi.z - lo.z) * rng(&mut s),
        );
        let dpt = want.depth(p);
        if dpt.abs() < skip {
            continue;
        }
        let q = to_body.transform_point(Point3::new(p.x, p.y, p.z));
        let got = point_in_solid(body, q, band(), Tol::witness());
        checked += 1;
        let ok = match got {
            Ok(SolidContainment::In) => dpt > 0.0,
            Ok(SolidContainment::Out) => dpt < 0.0,
            Ok(SolidContainment::OnBoundary) => false,
            Err(_) => {
                undecided += 1;
                continue;
            }
        };
        if dpt > 0.0 {
            inside += 1;
        }
        if !ok && bad.len() < 3 {
            bad.push(format!("{p:?} depth {dpt:.3e} got {got:?}"));
        }
    }
    assert!(
        undecided * 5 <= n,
        "too many undecided point_in_solid samples ({undecided}/{n})"
    );
    assert!(inside >= 10, "too few interior samples ({inside})");
    if bad.is_empty() {
        None
    } else {
        Some(format!("checked {checked} (inside {inside}): {bad:?}"))
    }
}

/// Runs every op in both orders. Built bodies: validate, closed mesh,
/// Monte Carlo against the CSG oracle. Returns the outcome row.
fn sweep_ops(label: &str, a: &Shape, b: &Shape, frame: &Affine3<f64>, scale: f64) -> String {
    let tol = Tol::witness();
    let bx = Csg::U(Box::new(a.csg.clone()), Box::new(b.csg.clone())).bbox();
    let mut row = format!("{label}:");
    for op in [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract] {
        for (ord, x, y) in [("AB", a, b), ("BA", b, a)] {
            let want = Csg::of(op, &x.csg, &y.csg);
            match boolean(op, &x.body, &y.body) {
                Err(e) => {
                    let s = format!("{e:?}");
                    let short: String = s.chars().take(60).collect();
                    row += &format!(" [{op:?}{ord} REFUSE {short}]");
                }
                Ok(None) => {
                    // Empty: the oracle must have no deep interior.
                    let m = mc_check_empty(&want, bx, 4000, 1e-6 * scale);
                    assert!(m.is_none(), "{label} {op:?}{ord}: EMPTY but oracle inside: {m:?}");
                    row += &format!(" [{op:?}{ord} empty]");
                }
                Ok(Some(body)) => {
                    assert_eq!(topo::validate(&body), Ok(()), "{label} {op:?}{ord} validate");
                    assert_eq!(topo::validate_closed(&body), Ok(()), "{label} {op:?}{ord} closed");
                    assert_eq!(
                        topo::validate_geometric(&body, tol),
                        Ok(()),
                        "{label} {op:?}{ord} geometric"
                    );
                    let (l, h) = want.bbox();
                    let pad = (h - l) * 0.05;
                    if let Some(bad) = mc_check(&body, &want, (l - pad, h + pad), frame, (1e-6 * scale).max(20.0 * Tol::witness().eps()), 3000) {
                        panic!("{label} {op:?}{ord}: WRONG BODY {bad}");
                    }
                    row += &format!(" [{op:?}{ord} ok]");
                }
            }
        }
    }
    row
}

fn mc_check_empty(want: &Csg, (lo, hi): (Vec3<f64>, Vec3<f64>), n: usize, skip: f64) -> Option<String> {
    let mut s = 7u64;
    for _ in 0..n {
        let p = Vec3::new(
            lo.x + (hi.x - lo.x) * rng(&mut s),
            lo.y + (hi.y - lo.y) * rng(&mut s),
            lo.z + (hi.z - lo.z) * rng(&mut s),
        );
        if want.depth(p) > skip {
            return Some(format!("{p:?}"));
        }
    }
    None
}

fn ident() -> Affine3<f64> {
    Affine3::identity()
}

const NECK: f64 = (1.96 + 1.0 - 0.64) / 2.8; // unit sphere's trim height in the snowman

fn on_unit(uy: f64) -> Vec3<f64> {
    Vec3::new(0.0, uy, (1.0 - uy * uy).sqrt())
}

/// The poses at scale `s`, each swept across its face's edge.
type Pose = (String, Box<dyn Fn() -> (Shape, Shape)>);

fn poses(s: f64) -> Vec<Pose> {
    let v = move |x: f64, y: f64, z: f64| Vec3::new(x, y, z) * s;
    let mut out: Vec<Pose> = Vec::new();
    let snowman = move || combine(BooleanOp::Union, &sball(s, v(0., 0., 0.)), &sball(0.8 * s, v(0., 1.4, 0.)));
    let lens = move || combine(BooleanOp::Intersect, &sball(s, v(0., 0., 0.)), &sball(0.8 * s, v(0., 1.4, 0.)));
    // snowman ⊃ ball(0.4) internally tangent at u: off face iff u_y > NECK
    for dy in [0.09, 1e-2, 1e-4, 1e-6, 0.0, -1e-4, -0.1] {
        let u = on_unit(NECK + dy);
        out.push((
            format!("s{s:e} snowman⊃ball0.4 uy=NECK{dy:+e}"),
            Box::new(move || (snowman(), sball(0.4 * s, u * (0.6 * s)))),
        ));
    }
    // lens, ball(0.3) externally tangent at u: off face iff u_y < NECK
    for dy in [-0.2, -1e-2, -1e-4, -1e-6, 0.0, 1e-4, 0.05] {
        let u = on_unit(NECK + dy);
        out.push((
            format!("s{s:e} lens,ball0.3 uy=NECK{dy:+e}"),
            Box::new(move || (lens(), sball(0.3 * s, u * (1.3 * s)))),
        ));
    }
    // unit ball, brick with plane y=1 face x in [d, d+1]
    for d in [0.5, 1e-2, 1e-4, 1e-6, 0.0, -1e-6, -0.5] {
        out.push((
            format!("s{s:e} ball,brick x0={d:+e}"),
            Box::new(move || (sball(s, v(0., 0., 0.)), brick([d * s, s, -s], [(d + 1.0) * s, 2.0 * s, s]))),
        ));
    }
    // unit ball, rod along x at (y,z)=(0,1.5) r .5 from x0
    for d in [0.5, 1e-2, 1e-4, 1e-6, 0.0, -0.5] {
        out.push((
            format!("s{s:e} ball,rod x0={d:+e}"),
            Box::new(move || (sball(s, v(0., 0., 0.)), rod(0.5 * s, v(0., 0., 1.5), Vec3::new(1., 0., 0.), (d * s, 2.0 * s)))),
        ));
    }
    // skew rods: x rod through origin, z rod through (0,1) from z0
    for d in [0.2, 1e-2, 1e-4, 1e-6, 0.0, -1.0] {
        out.push((
            format!("s{s:e} rodx,rodz z0={d:+e}"),
            Box::new(move || (rod(0.5 * s, v(0., 0., 0.), Vec3::new(1., 0., 0.), (-2.0 * s, 2.0 * s)), rod(0.5 * s, v(0., 1., 0.), Vec3::new(0., 0., 1.), (d * s, 3.0 * s)))),
        ));
    }
    // a holed plate: brick above y=1 with a y-hole of radius h about the touch
    for h in [0.5, 1e-2, 1e-3, 1e-5] {
        out.push((
            format!("s{s:e} ball,holed-plate h={h:e}"),
            Box::new(move || {
                let plate = brick([-s, s, -s], [s, 2.0 * s, s]);
                let hole = rod(h * s, v(0., 0.5, 0.), Vec3::new(0., 1., 0.), (0.0, 2.0 * s));
                (sball(s, v(0., 0., 0.)), combine(BooleanOp::Subtract, &plate, &hole))
            }),
        ));
    }
    out
}

fn build(label: &str, f: &dyn Fn() -> (Shape, Shape)) -> Option<(Shape, Shape)> {
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));
    if r.is_err() {
        eprintln!("ROW {label}: FIXTURE REFUSED (not this PR's surface)");
    }
    r.ok()
}

fn run_scale(s: f64) {
    let mut rows = Vec::new();
    for (label, f) in poses(s) {
        let Some((a, b)) = build(&label, &*f) else { continue };
        rows.push(sweep_ops(&label, &a, &b, &ident(), s));
    }
    for r in &rows {
        eprintln!("ROW {r}");
    }
}

#[test]
fn r1_sweep_scale_1() {
    run_scale(1.0);
}

#[test]
fn r1_sweep_scale_milli() {
    run_scale(1e-3);
}

#[test]
fn r1_sweep_scale_kilo() {
    run_scale(1e3);
}

/// Every pose rotated rigidly; the oracle stays in the original frame.
#[test]
fn r1_sweep_rotated() {
    let m = rot();
    let mut rows = Vec::new();
    for (label, f) in poses(1.0) {
        let Some((a, b)) = build(&label, &*f) else { continue };
        let (ra, rb) = (carry(&a, &m, &m), carry(&b, &m, &m));
        rows.push(sweep_ops(&format!("rot {label}"), &ra, &rb, &m, 1.0));
    }
    for r in &rows {
        eprintln!("ROW {r}");
    }
}

/// Results reused: snowman ∪ brick (off-face plane touch), then a nested
/// ball touching the unit sphere where trimmed, then a ball touching the
/// brick's top face's carrier off it.
#[test]
fn r1_results_reused() {
    let snow = combine(
        BooleanOp::Union,
        &sball(1.0, Vec3::new(0., 0., 0.)),
        &sball(0.8, Vec3::new(0., 1.4, 0.)),
    );
    let br = brick([1.5, 1.0, -1.0], [3.0, 2.0, 1.0]);
    let u1 = match boolean(BooleanOp::Union, &snow.body, &br.body) {
        Ok(Some(b)) => Shape {
            body: b,
            csg: Csg::of(BooleanOp::Union, &snow.csg, &br.csg),
        },
        other => panic!("u1: {other:?}"),
    };
    let inner = sball(0.4, on_unit(0.919) * 0.6);
    eprintln!("ROW {}", sweep_ops("reuse u1, inner", &u1, &inner, &ident(), 1.0));
    // a ball resting on the plane y=2 (brick top) at x=0, off the top face (x>=1.5)
    let top = sball(0.3, Vec3::new(0.0, 2.3, 0.0));
    eprintln!("ROW {}", sweep_ops("reuse u1, ball on y=2 off face", &u1, &top, &ident(), 1.0));
    // the same ball moved onto the top face
    let top_on = sball(0.3, Vec3::new(2.0, 2.3, 0.0));
    eprintln!("ROW {}", sweep_ops("reuse u1, ball on y=2 on face", &u1, &top_on, &ident(), 1.0));
}

/// In-band crossings: the plane y = 1 − h cuts the unit sphere in a
/// circle of radius √(2h) about (0, 1−h, 0); the face starts at x = d.
#[test]
fn r1_in_band_plane_crossing() {
    let eps = Tol::witness().eps();
    let mut rows = Vec::new();
    for hf in [0.5, 0.95, 5.0] {
        let h = hf * eps;
        let r = (2.0 * h).sqrt();
        for d in [0.5 * r, 2.0 * r, 1e-2] {
            let a = sball(1.0, Vec3::new(0., 0., 0.));
            let b = brick([d, 1.0 - h, -1.0], [d + 1.0, 2.0, 1.0]);
            rows.push(sweep_ops(&format!("inband h={h:e} circle r={r:.2e} x0={d:.2e}"), &a, &b, &ident(), 1.0));
        }
    }
    for r in &rows {
        eprintln!("ROW {r}");
    }
}

/// In-band sphere touches decided `Zero` but crossing by `h`: the inner
/// ball grows by `h` past the unit sphere (inside, where the snowman
/// trims it, so still nested in the union), and the outer ball moves
/// `h` into the lens's unit sphere where the lens trims it.
#[test]
fn r1_in_band_sphere_touches() {
    let eps = Tol::witness().eps();
    let snow = combine(
        BooleanOp::Union,
        &sball(1.0, Vec3::new(0., 0., 0.)),
        &sball(0.8, Vec3::new(0., 1.4, 0.)),
    );
    let lens = combine(
        BooleanOp::Intersect,
        &sball(1.0, Vec3::new(0., 0., 0.)),
        &sball(0.8, Vec3::new(0., 1.4, 0.)),
    );
    for hf in [-0.9, -0.5, 0.5, 0.9, 3.0] {
        let h = hf * eps;
        let u = on_unit(NECK + 0.09);
        let inner = sball(0.4 + h, u * 0.6);
        eprintln!("ROW {}", sweep_ops(&format!("inband snowman⊃ball h={h:e}"), &snow, &inner, &ident(), 1.0));
        let w = on_unit(NECK - 0.2);
        let outer = sball(0.3, w * (1.3 - h));
        eprintln!("ROW {}", sweep_ops(&format!("inband lens,ball h={h:e}"), &lens, &outer, &ident(), 1.0));
    }
}
