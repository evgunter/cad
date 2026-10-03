//! Reviewer probes for PR #3978 (lane reach-dual3978-r2): carrier
//! touches certified off a face on the no-crossings path. Every body
//! is checked against an oracle written here from the radii alone —
//! a closed-form volume and a Monte Carlo of `point_in_solid` against
//! analytic signed distances — never against the kernel. Widened past
//! the PR's poses: other radii, scales ×1e-3/×1/×1e3, a rigid tilt,
//! both orders, every op, gap ladders toward the face's edge, results
//! reused as operands. Outcomes are printed (`--nocapture`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane};
use sweep::{ExtrudeSide, Extrusion, Revolution, extrude};
use topo::{Body, BooleanError, BooleanOp, SolidContainment};

type Sd = Box<dyn Fn(Point3<f64>) -> f64>;

fn ball(r: f64, c: Vec3<f64>) -> Body<f64> {
    let b = sweep::test_support::revolved_about_y(
        vec![(Point2::new(0.0, -r), 1.0), (Point2::new(0.0, r), 0.0)],
        Revolution::Full,
        Tol::witness(),
    );
    topo::transform_rigid(&b, &Affine3::translation(c), Tol::witness()).unwrap()
}
fn sd_ball(r: f64, c: Vec3<f64>) -> Sd {
    Box::new(move |p: Point3<f64>| (p - Point3::origin() - c).norm() - r)
}

fn rod_z(r: f64, (x, y): (f64, f64), (z0, z1): (f64, f64)) -> Body<f64> {
    let tol = Tol::witness();
    let lp = profile::circle(Point2::new(x, y), r, tol).unwrap();
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp.into()]).validate(tol).unwrap();
    extrude(
        &profile,
        Extrusion::Distance { depth: z1 - z0, side: ExtrudeSide::Along },
        tol,
    )
    .unwrap()
    .body
}
fn sd_rod_z(r: f64, (x, y): (f64, f64), (z0, z1): (f64, f64)) -> Sd {
    Box::new(move |p: Point3<f64>| {
        let rad = ((p.x - x).powi(2) + (p.y - y).powi(2)).sqrt() - r;
        let ax = (z0 - p.z).max(p.z - z1);
        rad.max(ax)
    })
}
/// A z rod turned a quarter about y: `(x, y, z) -> (z, y, -x)`.
fn quarter_y() -> Affine3<f64> {
    Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 1.0, 0.0), core::f64::consts::FRAC_PI_2)
}
/// A rod along x through `(y, z)`, `x ∈ [x0, x1]`.
fn rod_x(r: f64, (y, z): (f64, f64), (x0, x1): (f64, f64)) -> (Body<f64>, Sd) {
    let b = topo::transform_rigid(&rod_z(r, (-z, y), (x0, x1)), &quarter_y(), Tol::witness()).unwrap();
    let sd: Sd = Box::new(move |p: Point3<f64>| {
        let rad = ((p.y - y).powi(2) + (p.z - z).powi(2)).sqrt() - r;
        rad.max((x0 - p.x).max(p.x - x1))
    });
    (b, sd)
}
fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> (Body<f64>, Sd) {
    let b = sweep::test_support::brick(x, y, z, Tol::witness());
    let sd: Sd = Box::new(move |p: Point3<f64>| {
        let f = |v: f64, (a, b): (f64, f64)| (a - v).max(v - b);
        f(p.x, x).max(f(p.y, y)).max(f(p.z, z))
    });
    (b, sd)
}

fn boolean(op: BooleanOp, a: &Body<f64>, b: &Body<f64>) -> Result<topo::BooleanResult<f64>, BooleanError> {
    let t = Tol::witness();
    match op {
        BooleanOp::Union => topo::boolean::union(a, b, t),
        BooleanOp::Intersect => topo::boolean::intersect(a, b, t),
        BooleanOp::Subtract => topo::boolean::subtract(a, b, t),
    }
}
fn combine(op: BooleanOp, a: f64, b: f64) -> f64 {
    match op {
        BooleanOp::Union => a.min(b),
        BooleanOp::Intersect => a.max(b),
        BooleanOp::Subtract => a.max(-b),
    }
}

/// A deterministic LCG in [0, 1).
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
}

/// A body against its oracle: tiers, closed mesh, volume, and a Monte
/// Carlo of `point_in_solid` against `sd` (points within `skip` of a
/// boundary are not asked). `frame` maps the oracle's frame to the body's.
fn check_body(label: &str, body: &Body<f64>, want: f64, sd: &dyn Fn(Point3<f64>) -> f64, frame: &Affine3<f64>, scale: f64, bbox: ([f64; 3], [f64; 3])) {
    let tol = Tol::witness();
    assert_eq!(topo::validate(body), Ok(()), "{label}: validate");
    assert_eq!(topo::validate_closed(body), Ok(()), "{label}: validate_closed");
    assert_eq!(topo::validate_geometric(body, tol), Ok(()), "{label}: validate_geometric");
    let m = mesh::tessellate(body, 1e-3 * scale, tol).unwrap_or_else(|e| panic!("{label}: tessellate {e:?}"));
    assert_eq!(mesh::validate::check_mesh(&m), Ok(()), "{label}: mesh");
    let p = topo::mass_properties(body, tol).unwrap_or_else(|e| panic!("{label}: mass {e:?}"));
    assert!((p.volume - want).abs() <= 1e-8 * want.abs().max(scale.powi(3) * 1e-3), "{label}: volume {} vs oracle {want}", p.volume);
    let band = Band::linear(tol).unwrap();
    let mut rng = Rng(0x3978);
    let (lo, hi) = bbox;
    let (mut asked, mut bad) = (0, 0);
    for _ in 0..120 {
        let q = Point3::new(
            lo[0] + (hi[0] - lo[0]) * rng.next(),
            lo[1] + (hi[1] - lo[1]) * rng.next(),
            lo[2] + (hi[2] - lo[2]) * rng.next(),
        );
        let s = sd(q);
        if s.abs() < 1e-4 * scale {
            continue;
        }
        let qb = frame.transform_point(q);
        match topo::point_in_solid(body, qb, band, tol) {
            Ok(SolidContainment::In) => {
                asked += 1;
                if s > 0.0 { bad += 1; eprintln!("{label}: PIS In at {q:?}, oracle out ({s})"); }
            }
            Ok(SolidContainment::Out) => {
                asked += 1;
                if s < 0.0 { bad += 1; eprintln!("{label}: PIS Out at {q:?}, oracle in ({s})"); }
            }
            other => eprintln!("{label}: PIS {other:?} at {q:?} (sd {s})"),
        }
    }
    assert_eq!(bad, 0, "{label}: {bad} of {asked} Monte Carlo points disagree");
}

/// What one pose did under every op in both orders.
#[derive(Default, Debug)]
struct Tally {
    built: usize,
    refused: Vec<String>,
}

struct Pose {
    a: Body<f64>,
    sa: Sd,
    va: f64,
    b: Body<f64>,
    sb: Sd,
    vb: f64,
    /// `true` when B lies inside A; else the two are apart.
    nested: bool,
    bbox: ([f64; 3], [f64; 3]),
}

/// Every op, both orders: a build is checked against the oracle; a
/// refusal is recorded with its payload's head.
fn run(label: &str, pose: &Pose, frame: &Affine3<f64>, scale: f64) -> Tally {
    let tol = Tol::witness();
    let a = topo::transform_rigid(&pose.a, frame, tol).unwrap();
    let b = topo::transform_rigid(&pose.b, frame, tol).unwrap();
    let meet = if pose.nested { pose.vb } else { 0.0 };
    let mut t = Tally::default();
    for (op, ab) in [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract].into_iter().flat_map(|o| [(o, true), (o, false)]) {
        let (x, y, sx, sy, vx, vy) = if ab { (&a, &b, &pose.sa, &pose.sb, pose.va, pose.vb) } else { (&b, &a, &pose.sb, &pose.sa, pose.vb, pose.va) };
        let want = match op {
            BooleanOp::Union => pose.va + pose.vb - meet,
            BooleanOp::Intersect => meet,
            BooleanOp::Subtract => vx - meet,
        };
        let _ = vy;
        let name = format!("{label} {op:?} {}", if ab { "A·B" } else { "B·A" });
        match boolean(op, x, y) {
            Ok(r) => match r.body() {
                Some(out) => {
                    let sd = |p: Point3<f64>| combine(op, sx(p), sy(p));
                    check_body(&name, &out.body, want, &sd, frame, scale, pose.bbox);
                    t.built += 1;
                }
                None => {
                    assert!(want.abs() <= 1e-9 * scale.powi(3), "{name}: empty vs {want}");
                    t.built += 1;
                }
            },
            Err(e) => {
                let s = format!("{e:?}");
                t.refused.push(format!("{name}: {}", &s[..s.len().min(160)]));
            }
        }
    }
    eprintln!("{label}: built {} / 6; refused {:?}", t.built, t.refused);
    t
}

fn ball_volume(r: f64) -> f64 { 4.0 / 3.0 * PI * r.powi(3) }
fn cap_volume(r: f64, h: f64) -> f64 { PI * h.powi(2) * (3.0 * r - h) / 3.0 }
fn lens_volume(r1: f64, r2: f64, d: f64) -> f64 {
    let x = (d.powi(2) + r1.powi(2) - r2.powi(2)) / (2.0 * d);
    cap_volume(r1, r1 - x) + cap_volume(r2, r2 - (d - x))
}
fn dir(x: f64, y: f64, z: f64) -> Vec3<f64> { let v = Vec3::new(x, y, z); v / v.norm() }
fn v(x: f64, y: f64, z: f64) -> Vec3<f64> { Vec3::new(x, y, z) }

/// A snowman of radii `r1` at 0 and `r2` at `y = d`, scaled by `s`.
fn snowman(s: f64, (r1, r2, d): (f64, f64, f64)) -> (Body<f64>, Sd, f64) {
    let a = ball(r1 * s, v(0.0, 0.0, 0.0));
    let b = ball(r2 * s, v(0.0, d * s, 0.0));
    let body = boolean(BooleanOp::Union, &a, &b).unwrap().body().unwrap().body.clone();
    let (sa, sb) = (sd_ball(r1 * s, v(0.0, 0.0, 0.0)), sd_ball(r2 * s, v(0.0, d * s, 0.0)));
    let vol = (ball_volume(r1) + ball_volume(r2) - lens_volume(r1, r2, d)) * s.powi(3);
    (body, Box::new(move |p| sa(p).min(sb(p))), vol)
}
fn lens(s: f64, (r1, r2, d): (f64, f64, f64)) -> (Body<f64>, Sd, f64) {
    let a = ball(r1 * s, v(0.0, 0.0, 0.0));
    let b = ball(r2 * s, v(0.0, d * s, 0.0));
    let body = boolean(BooleanOp::Intersect, &a, &b).unwrap().body().unwrap().body.clone();
    let (sa, sb) = (sd_ball(r1 * s, v(0.0, 0.0, 0.0)), sd_ball(r2 * s, v(0.0, d * s, 0.0)));
    (body, Box::new(move |p| sa(p).max(sb(p))), lens_volume(r1, r2, d) * s.powi(3))
}
fn bx(s: f64, lo: [f64; 3], hi: [f64; 3]) -> ([f64; 3], [f64; 3]) {
    (lo.map(|x| x * s), hi.map(|x| x * s))
}

const SCALES: [f64; 3] = [1e-3, 1.0, 1e3];
fn frames() -> Vec<(&'static str, Affine3<f64>)> {
    let tilt = Affine3::rotation_about_axis(Point3::origin(), dir(0.3, -0.5, 0.8), 0.7)
        * Affine3::rotation_about_axis(Point3::origin(), dir(1.0, 0.2, 0.0), -1.1);
    vec![("id", Affine3::identity()), ("tilt", tilt)]
}

/// Off-face poses of every arm, at three scales and two frames: a build
/// must match the oracle; any refusal is reported.
#[test]
fn off_face_touches_widened() {
    let mut refusals = Vec::new();
    for s in SCALES {
        for (fname, frame) in frames() {
            let mut poses: Vec<(String, Pose)> = Vec::new();
            // Sphere × sphere inside, other radii: snowman(1, 0.7, 1.2), ball(0.25)
            // internally tangent to the unit sphere at a trimmed point.
            let (sm, ssm, vsm) = snowman(s, (1.0, 0.7, 1.2));
            let c = dir(0.2, 0.8, -0.3) * (0.75 * s);
            poses.push(("snowman⊃ball(.25)".into(), Pose { a: sm, sa: ssm, va: vsm, b: ball(0.25 * s, c), sb: sd_ball(0.25 * s, c), vb: ball_volume(0.25 * s), nested: true, bbox: bx(s, [-1.1, -1.1, -1.1], [1.1, 2.0, 1.1]) }));
            // Sphere × plane: a brick whose plane x = 1 touches the unit sphere
            // at (1, 0, 0), off the brick face (y ≥ 0.3): the snowman trims nothing
            // there, so this is the plane-face side.
            let (sm, ssm, vsm) = snowman(s, (1.0, 0.7, 1.2));
            let (br, sbr) = brick((1.0 * s, 2.0 * s), (0.3 * s, 1.5 * s), (-0.5 * s, 0.5 * s));
            poses.push(("snowman,brick x=1".into(), Pose { a: sm, sa: ssm, va: vsm, b: br, sb: sbr, vb: 1.2 * s.powi(3), nested: false, bbox: bx(s, [-1.1, -1.1, -1.1], [2.1, 2.0, 1.1]) }));
            // Sphere × sphere outside, off the lens face: lens(1, 0.9, 1.3).
            let (ln, sln, vln) = lens(s, (1.0, 0.9, 1.3));
            let c = dir(-0.4, -0.6, 0.7) * (1.35 * s);
            poses.push(("lens,ball(.35)".into(), Pose { a: ln, sa: sln, va: vln, b: ball(0.35 * s, c), sb: sd_ball(0.35 * s, c), vb: ball_volume(0.35 * s), nested: false, bbox: bx(s, [-1.4, -1.4, -1.4], [1.4, 1.4, 1.4]) }));
            // Sphere × cylinder outside, past the rod's end; rod r 0.3.
            let (rd, srd) = rod_x(0.3 * s, (0.0, 1.0 * s + 0.3 * s), (0.2 * s, 1.7 * s));
            poses.push(("ball,rod(.3)".into(), Pose { a: ball(s, v(0.0, 0.0, 0.0)), sa: sd_ball(s, v(0.0, 0.0, 0.0)), va: ball_volume(s), b: rd, sb: srd, vb: PI * 0.09 * 1.5 * s.powi(3), nested: false, bbox: bx(s, [-1.1, -1.1, -1.1], [1.8, 1.1, 1.7]) }));
            // Skew walls (unequal radii, 60° apart) touching past one rod's end.
            let (rx, srx) = rod_x(0.4 * s, (0.0, 0.0), (-1.5 * s, 1.5 * s));
            let r2 = 0.25 * s;
            let ax = dir(0.5, 0.0, 3f64.sqrt() / 2.0);
            let rot = Affine3::rotation_about_axis(Point3::origin(), v(0.0, 1.0, 0.0), -PI / 6.0);
            // A z rod through (0, 0.65) from z = 0.15 to 1.5, turned about y so its
            // axis runs along `ax`; its wall touches the x rod's at (0, 0.4, 0).
            let rz = topo::transform_rigid(&rod_z(r2, (0.0, 0.65 * s), (0.15 * s, 1.5 * s)), &rot, Tol::witness()).unwrap();
            let inv = rot.inverse();
            let srz0 = sd_rod_z(r2, (0.0, 0.65 * s), (0.15 * s, 1.5 * s));
            let srz: Sd = Box::new(move |p| srz0(inv.transform_point(p)));
            let _ = ax;
            poses.push(("rodx(.4),rod60(.25)".into(), Pose { a: rx, sa: srx, va: PI * 0.16 * 3.0 * s.powi(3), b: rz, sb: srz, vb: PI * 0.0625 * 1.35 * s.powi(3), nested: false, bbox: bx(s, [-1.6, -0.5, -1.6], [1.6, 1.0, 1.6]) }));
            for (name, pose) in &poses {
                let t = run(&format!("{name} s={s} {fname}"), pose, &frame, s);
                refusals.extend(t.refused);
            }
        }
    }
    eprintln!("REFUSALS ({}): {refusals:#?}", refusals.len());
}

/// Gap ladders toward the face's boundary: the touch sits `δ` outside
/// the face, so the true gap between the bodies is about `δ²/2` (`δ²`
/// for the skew walls). A build must be correct; a build where the true
/// gap is below ε is reported as touching bodies built apart.
#[test]
fn gap_ladders_toward_the_face_edge() {
    let eps = Tol::witness().eps();
    for s in SCALES {
        for d in [1e-1, 1e-2, 1e-3, 3e-4, 1e-4, 3e-5, 1e-5, 0.0] {
            let dd = d * s;
            // Sphere × plane: brick x ∈ [δ, 1], bottom y = 1 touching ball(1) at (0,1,0).
            let (br, sbr) = brick((dd, s), (s, 2.0 * s), (-0.5 * s, 0.5 * s));
            let pose = Pose { a: ball(s, v(0.0, 0.0, 0.0)), sa: sd_ball(s, v(0.0, 0.0, 0.0)), va: ball_volume(s), b: br, sb: sbr, vb: (1.0 - d) * s.powi(3), nested: false, bbox: bx(s, [-1.1, -1.1, -1.1], [1.1, 2.1, 1.1]) };
            let gap = s - (s * s - dd * dd).sqrt();
            let t = run(&format!("LADDER plane s={s} δ={d} gap={gap:.2e}"), &pose, &Affine3::identity(), s);
            if gap < eps && t.built > 0 { eprintln!("!! plane δ={d} s={s}: built with true gap {gap:e} < ε"); }
            // Sphere × cylinder: rod r .5 along x at z = 1.5 from x = δ.
            let (rd, srd) = rod_x(0.5 * s, (0.0, 1.5 * s), (dd, 2.0 * s));
            let pose = Pose { a: ball(s, v(0.0, 0.0, 0.0)), sa: sd_ball(s, v(0.0, 0.0, 0.0)), va: ball_volume(s), b: rd, sb: srd, vb: PI * 0.25 * (2.0 - d) * s.powi(3), nested: false, bbox: bx(s, [-1.1, -1.1, -1.1], [2.1, 1.1, 2.1]) };
            let gap = (s * s + dd * dd).sqrt() - s;
            let t = run(&format!("LADDER cyl s={s} δ={d} gap={gap:.2e}"), &pose, &Affine3::identity(), s);
            if gap < eps && t.built > 0 { eprintln!("!! cyl δ={d} s={s}: built with true gap {gap:e} < ε"); }
            // Skew walls: z rod through (0,1) from z = δ.
            let (rx, srx) = rod_x(0.5 * s, (0.0, 0.0), (-2.0 * s, 2.0 * s));
            let pose = Pose { a: rx, sa: srx, va: PI * 0.25 * 4.0 * s.powi(3), b: rod_z(0.5 * s, (0.0, s), (dd, 3.0 * s)), sb: sd_rod_z(0.5 * s, (0.0, s), (dd, 3.0 * s)), vb: PI * 0.25 * (3.0 - d) * s.powi(3), nested: false, bbox: bx(s, [-2.1, -0.6, -0.6], [2.1, 1.6, 3.1]) };
            let gap = (0.25 * s * s + dd * dd).sqrt() - 0.5 * s;
            let t = run(&format!("LADDER skew s={s} δ={d} gap={gap:.2e}"), &pose, &Affine3::identity(), s);
            if gap < eps && t.built > 0 { eprintln!("!! skew δ={d} s={s}: built with true gap {gap:e} < ε"); }
        }
    }
}

/// A ball touching a half-rod's cylinder carrier from INSIDE the
/// carrier, at a point the half-rod trims away (y < 0): the inside
/// sphere × cylinder touch.
#[test]
fn ball_inside_a_cylinder_carrier_touching_where_the_half_rod_is_trimmed() {
    for s in SCALES {
        let (rd, srd) = rod_x(s, (0.0, 0.0), (-1.0 * s, 1.0 * s));
        let (br, sbr) = brick((-2.0 * s, 2.0 * s), (0.0, 2.0 * s), (-2.0 * s, 2.0 * s));
        let half = boolean(BooleanOp::Intersect, &rd, &br).unwrap().body().unwrap().body.clone();
        let sh: Sd = Box::new(move |p| srd(p).max(sbr(p)));
        let c = v(0.1 * s, -0.6 * s, 0.0);
        let pose = Pose { a: half, sa: sh, va: PI * s.powi(3), b: ball(0.4 * s, c), sb: sd_ball(0.4 * s, c), vb: ball_volume(0.4 * s), nested: false, bbox: bx(s, [-1.1, -1.1, -1.1], [1.1, 1.1, 1.1]) };
        let t = run(&format!("half-rod,ball inside-carrier s={s}"), &pose, &Affine3::identity(), s);
        eprintln!("inside-carrier s={s}: {t:?}");
    }
}

/// Results reused as operands: (snowman ∪ brick) then against a ball
/// nested at a trimmed touch, and the A∖B cavity body against a ball.
#[test]
fn results_reused_as_operands() {
    let s = 1.0;
    let (sm, ssm, vsm) = snowman(s, (1.0, 0.8, 1.4));
    let (br, sbr) = brick((1.5, 3.0), (1.0, 2.0), (-1.0, 1.0));
    let u = boolean(BooleanOp::Union, &sm, &br).expect("snowman ∪ brick").body().unwrap().body.clone();
    let su: Sd = Box::new(move |p| ssm(p).min(sbr(p)));
    let c = dir(0.0, 0.7, 0.3) * 0.6;
    let pose = Pose { a: u, sa: su, va: vsm + 3.0, b: ball(0.4, c), sb: sd_ball(0.4, c), vb: ball_volume(0.4), nested: true, bbox: bx(s, [-1.1, -1.1, -1.1], [3.1, 2.3, 1.1]) };
    run("(snowman∪brick)⊃ball(.4)", &pose, &Affine3::identity(), s);
    // The cavity: snowman ∖ ball(.4), then ∪ the same ball back.
    let (sm, ssm, vsm) = snowman(s, (1.0, 0.8, 1.4));
    let cav = boolean(BooleanOp::Subtract, &sm, &ball(0.4, c)).expect("cavity").body().unwrap().body.clone();
    let sb0 = sd_ball(0.4, c);
    let scav: Sd = Box::new(move |p| ssm(p).max(-sb0(p)));
    let pose = Pose { a: cav, sa: scav, va: vsm - ball_volume(0.4), b: ball(0.3, c), sb: sd_ball(0.3, c), vb: ball_volume(0.3), nested: false, bbox: bx(s, [-1.1, -1.1, -1.1], [1.1, 2.3, 1.1]) };
    run("cavity,ball(.3) inside the void", &pose, &Affine3::identity(), s);
}

/// Diagnostic: the skew pair at ×1e-3 in the identity frame, full payload.
#[test]
fn diag_skew_small() {
    for s in [1e-3, 2e-3, 5e-3, 1e-2, 1e-1] {
        let (rx, _) = rod_x(0.4 * s, (0.0, 0.0), (-1.5 * s, 1.5 * s));
        let rot = Affine3::rotation_about_axis(Point3::origin(), v(0.0, 1.0, 0.0), -PI / 6.0);
        let rz = topo::transform_rigid(&rod_z(0.25 * s, (0.0, 0.65 * s), (0.15 * s, 1.5 * s)), &rot, Tol::witness()).unwrap();
        eprintln!("DIAG s={s}: {:?}", boolean(BooleanOp::Union, &rx, &rz).map(|r| r.body().is_some()));
    }
}

/// The inside sphere × cylinder touch with the wall's box reaching the
/// ball: a rod with one quadrant notched out (`y < 0, z > 0`), and a
/// ball in the notch touching the wall's carrier from inside at
/// `(x, −1, 1)/√2`, which the notch removes.
#[test]
fn ball_in_a_notched_rod_touching_the_carrier_from_inside() {
    for s in SCALES {
        for (fname, frame) in frames() {
            let (rd, srd) = rod_x(s, (0.0, 0.0), (-1.0 * s, 1.0 * s));
            let (br, sbr) = brick((-2.0 * s, 2.0 * s), (-2.0 * s, 0.0), (0.0, 2.0 * s));
            let notched = boolean(BooleanOp::Subtract, &rd, &br).unwrap().body().unwrap().body.clone();
            let sh: Sd = Box::new(move |p| srd(p).max(-sbr(p)));
            let c = v(0.1 * s, -0.6 * s / 2f64.sqrt(), 0.6 * s / 2f64.sqrt());
            let pose = Pose { a: notched, sa: sh, va: 1.5 * PI * s.powi(3), b: ball(0.4 * s, c), sb: sd_ball(0.4 * s, c), vb: ball_volume(0.4 * s), nested: false, bbox: bx(s, [-1.1, -1.1, -1.1], [1.1, 1.1, 1.1]) };
            run(&format!("notched rod,ball inside-carrier s={s} {fname}"), &pose, &frame, s);
        }
    }
}

/// The touch moved ONTO the face by `δ` (a genuine tangency of the two
/// boundaries): every op must refuse; a build is a touching pair built
/// as apart.
#[test]
fn touches_just_inside_the_face_refuse() {
    let mut built = Vec::new();
    for s in SCALES {
        for d in [-1e-1, -1e-3, -1e-5, -1e-7, -1e-9] {
            let dd = d * s;
            let (br, sbr) = brick((dd, s), (s, 2.0 * s), (-0.5 * s, 0.5 * s));
            let pose = Pose { a: ball(s, v(0.0, 0.0, 0.0)), sa: sd_ball(s, v(0.0, 0.0, 0.0)), va: ball_volume(s), b: br, sb: sbr, vb: (1.0 - d) * s.powi(3), nested: false, bbox: bx(s, [-1.1, -1.1, -1.1], [1.1, 2.1, 1.1]) };
            let t = run(&format!("ONFACE plane s={s} δ={d}"), &pose, &Affine3::identity(), s);
            if t.built > 0 { built.push(format!("plane s={s} δ={d}")); }
            let (rd, srd) = rod_x(0.5 * s, (0.0, 1.5 * s), (dd, 2.0 * s));
            let pose = Pose { a: ball(s, v(0.0, 0.0, 0.0)), sa: sd_ball(s, v(0.0, 0.0, 0.0)), va: ball_volume(s), b: rd, sb: srd, vb: PI * 0.25 * (2.0 - d) * s.powi(3), nested: false, bbox: bx(s, [-1.1, -1.1, -1.1], [2.1, 1.1, 2.1]) };
            let t = run(&format!("ONFACE cyl s={s} δ={d}"), &pose, &Affine3::identity(), s);
            if t.built > 0 { built.push(format!("cyl s={s} δ={d}")); }
            let (rx, srx) = rod_x(0.5 * s, (0.0, 0.0), (-2.0 * s, 2.0 * s));
            let pose = Pose { a: rx, sa: srx, va: PI * 0.25 * 4.0 * s.powi(3), b: rod_z(0.5 * s, (0.0, s), (dd, 3.0 * s)), sb: sd_rod_z(0.5 * s, (0.0, s), (dd, 3.0 * s)), vb: PI * 0.25 * (3.0 - d) * s.powi(3), nested: false, bbox: bx(s, [-2.1, -0.6, -0.6], [2.1, 1.6, 3.1]) };
            let t = run(&format!("ONFACE skew s={s} δ={d}"), &pose, &Affine3::identity(), s);
            if t.built > 0 { built.push(format!("skew s={s} δ={d}")); }
        }
    }
    assert!(built.is_empty(), "touching pairs built as apart: {built:?}");
}

/// Sphere × sphere across the lens's rim: a ball(0.3) outside the unit
/// ball touching it at polar angle `rim + η` from +y. `η > 0` is off the
/// lens face (gap about `η²`); `η < 0` is on it and must refuse.
#[test]
fn a_ball_touching_the_lens_across_its_rim() {
    let mut wrong = Vec::new();
    for s in SCALES {
        let (r1, r2, d): (f64, f64, f64) = (1.0, 0.8, 1.4);
        let rim = ((d * d + r1 * r1 - r2 * r2) / (2.0 * d) / r1).acos();
        for eta in [0.3, 0.05, 1e-2, 1e-3, 1e-4, 1e-5, 0.0, -1e-5, -1e-3, -0.05] {
            let (ln, sln, vln) = lens(s, (r1, r2, d));
            let a = rim + eta;
            let u = v(a.sin() * 0.7f64.cos(), a.cos(), a.sin() * 0.7f64.sin());
            let c = u * (1.3 * s);
            let pose = Pose { a: ln, sa: sln, va: vln, b: ball(0.3 * s, c), sb: sd_ball(0.3 * s, c), vb: ball_volume(0.3 * s), nested: false, bbox: bx(s, [-1.7, -1.7, -1.7], [1.7, 1.7, 1.7]) };
            let t = run(&format!("RIM s={s} η={eta}"), &pose, &Affine3::identity(), s);
            if eta <= 0.0 && t.built > 0 { wrong.push(format!("s={s} η={eta}")); }
        }
    }
    assert!(wrong.is_empty(), "on-face touches built: {wrong:?}");
}
