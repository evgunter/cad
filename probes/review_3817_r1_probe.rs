//! Reviewer probe for PR #3817 (lane reach-dual3817-r1). An oracle that
//! reads only the body's stored boundary (edge carriers + face sense) and
//! the two input spheres: every result face is checked, by Monte Carlo on
//! each sphere, to cover exactly the region the boolean's set algebra
//! says, with the right outward side. Point-in-face is a winding number
//! of the face's sampled loops under a stereographic projection, so it
//! never reads a chart, a window, an arc-side verdict, or props.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom::{Curve3, Surface};
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::{Body, BooleanOp};

fn ball(r: f64, c: Vec3<f64>, rot: Option<(Vec3<f64>, f64)>) -> Body<f64> {
    let mut b = revolved_about_y(
        vec![(Point2::new(0.0, -r), 1.0), (Point2::new(0.0, r), 0.0)],
        Revolution::Full,
        Tol::witness(),
    );
    if let Some((ax, ang)) = rot {
        let m = Affine3::rotation_about_axis(Point3::origin(), ax / ax.norm(), ang);
        b = topo::transform_rigid(&b, &m, Tol::witness()).unwrap();
    }
    topo::transform_rigid(&b, &Affine3::translation(c), Tol::witness()).unwrap()
}

fn cap(r: f64, h: f64) -> f64 {
    PI * h * h * (3.0 * r - h) / 3.0
}
fn lens(r1: f64, r2: f64, d: f64) -> f64 {
    if d >= r1 + r2 {
        return 0.0;
    }
    if d <= (r1 - r2).abs() {
        return 4.0 / 3.0 * PI * r1.min(r2).powi(3);
    }
    let x = (d * d + r1 * r1 - r2 * r2) / (2.0 * d);
    cap(r1, r1 - x) + cap(r2, r2 - (d - x))
}

struct Lcg(u64);
impl Lcg {
    fn f(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
    fn unit(&mut self) -> Vec3<f64> {
        loop {
            let v = Vec3::new(2.0 * self.f() - 1.0, 2.0 * self.f() - 1.0, 2.0 * self.f() - 1.0);
            let n = v.norm();
            if n > 0.1 && n <= 1.0 {
                return v / n;
            }
        }
    }
}

/// A face as the oracle sees it: sphere, outward sign, sampled loops.
struct OFace {
    c: Point3<f64>,
    r: f64,
    sigma: f64,
    loops: Vec<Vec<Point3<f64>>>,
}

fn faces_of(body: &Body<f64>) -> Vec<OFace> {
    let mut out = Vec::new();
    for (_fk, face) in body.faces() {
        let Some(Surface::Sphere { center, radius, .. }) = body.get_surface(face.surface) else {
            panic!("non-sphere face {:?}", body.get_surface(face.surface));
        };
        let mut loops = Vec::new();
        for lk in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
            let lp = body.get_loop(lk).unwrap();
            let topo::entity::LoopBoundary::Cycle { first: he0 } = lp.boundary else {
                continue;
            };
            let mut pts = Vec::new();
            for he in body.loop_cycle(he0).unwrap() {
                let h = body.get_half_edge(he).unwrap();
                let e = body.get_edge(h.edge).unwrap();
                let Some(topo::null::CurveGeom::Certified(cv)) = body.get_curve_geom(e.curve) else {
                    continue;
                };
                let (t0, t1) = cv.params();
                let fwd = e.he_plus == he;
                let n = 600;
                for i in 0..n {
                    let s = i as f64 / n as f64;
                    let t = if fwd { t0 + (t1 - t0) * s } else { t1 - (t1 - t0) * s };
                    pts.push(cv.carrier().eval(t));
                }
            }
            if !pts.is_empty() {
                loops.push(pts);
            }
        }
        out.push(OFace {
            c: *center,
            r: *radius,
            sigma: if face.sense { 1.0 } else { -1.0 },
            loops,
        });
    }
    out
}

/// Is `q` (on the face's sphere) inside the face? Stereographic from `pole`.
fn in_face(f: &OFace, q: Point3<f64>, pole: Vec3<f64>) -> bool {
    let a = if pole.x.abs() < 0.9 { Vec3::new(1.0, 0.0, 0.0) } else { Vec3::new(0.0, 1.0, 0.0) };
    let e1 = pole.cross(a);
    let e1 = e1 / e1.norm();
    let e2 = pole.cross(e1);
    let proj = |p: Point3<f64>| {
        let u = (p - f.c) / f.r;
        let d = 1.0 - u.dot(pole);
        (u.dot(e1) / d, u.dot(e2) / d)
    };
    let (qx, qy) = proj(q);
    let mut wind = 0.0;
    let mut area = 0.0;
    for lp in &f.loops {
        let pp: Vec<(f64, f64)> = lp.iter().map(|&p| proj(p)).collect();
        for i in 0..pp.len() {
            let (x0, y0) = pp[i];
            let (x1, y1) = pp[(i + 1) % pp.len()];
            area += x0 * y1 - x1 * y0;
            let a0 = (y0 - qy).atan2(x0 - qx);
            let a1 = (y1 - qy).atan2(x1 - qx);
            let mut da = a1 - a0;
            while da > PI {
                da -= 2.0 * PI;
            }
            while da < -PI {
                da += 2.0 * PI;
            }
            wind += da;
        }
    }
    // e1 × e2 = pole × a × ... ; orientation handled by testing both
    // conventions against the untouched ball (see `orientation_sign`).
    let w = (wind / (2.0 * PI)).round();
    let v = -ORIENT * f.sigma * w;
    let ap = -ORIENT * f.sigma * area;
    v == if ap > 0.0 { 1.0 } else { 0.0 }
}

/// +1 if e1×e2 points along the pole (calibrated on an untouched ball).
const ORIENT: f64 = 1.0;

#[derive(Clone, Copy)]
struct Sph {
    c: Point3<f64>,
    r: f64,
}

/// Monte Carlo over both spheres: returns (mismatches, samples, foreign faces).
fn oracle(body: &Body<f64>, a: Sph, b: Sph, op: BooleanOp, a_minus_b: bool, seed: u64) -> (usize, usize) {
    let faces = faces_of(body);
    for f in &faces {
        let on_a = (f.c - a.c).norm() < 1e-9 * a.r.max(1.0) && (f.r - a.r).abs() < 1e-9 * a.r.max(1.0);
        let on_b = (f.c - b.c).norm() < 1e-9 * b.r.max(1.0) && (f.r - b.r).abs() < 1e-9 * b.r.max(1.0);
        assert!(on_a || on_b, "face on a foreign sphere c={:?} r={}", f.c, f.r);
    }
    let mut rng = Lcg(seed);
    let mut bad = 0;
    let mut n = 0;
    let section_d = |p: Point3<f64>| {
        // distance from p to the other sphere's surface
        ((p - a.c).norm() - a.r).abs().min(((p - b.c).norm() - b.r).abs())
    };
    for (me, other, is_a) in [(a, b, true), (b, a, false)] {
        for _ in 0..4000 {
            let q = me.c + rng.unit() * me.r;
            let din = (q - other.c).norm() - other.r; // <0 inside other
            if din.abs() < 2e-3 * other.r || section_d(q) > 1.0 && false {
                continue;
            }
            let inside_other = din < 0.0;
            // expected: on boundary? with sigma (+1 radial out, -1 in)
            let expect: Option<f64> = match op {
                BooleanOp::Union => (!inside_other).then_some(1.0),
                BooleanOp::Intersect => inside_other.then_some(1.0),
                BooleanOp::Subtract => {
                    // a_minus_b: result = A ∖ B (A the first operand)
                    let first = is_a == a_minus_b;
                    if first { (!inside_other).then_some(1.0) } else { inside_other.then_some(-1.0) }
                }
            };
            let pole = rng.unit();
            let mut hits = Vec::new();
            for f in &faces {
                let same = (f.c - me.c).norm() < 1e-9 * me.r.max(1.0) && (f.r - me.r).abs() < 1e-9 * me.r.max(1.0);
                if same && in_face(f, q, pole) {
                    hits.push(f.sigma);
                }
            }
            n += 1;
            let ok = match expect {
                None => hits.is_empty(),
                Some(s) => hits.len() == 1 && hits[0] == s,
            };
            if !ok {
                bad += 1;
            }
        }
    }
    (bad, n)
}

fn run(op: BooleanOp, a: &Body<f64>, b: &Body<f64>) -> Result<topo::BooleanResult<f64>, topo::BooleanError> {
    match op {
        BooleanOp::Union => topo::boolean::union(a, b, Tol::witness()),
        BooleanOp::Intersect => topo::boolean::intersect(a, b, Tol::witness()),
        BooleanOp::Subtract => topo::boolean::subtract(a, b, Tol::witness()),
    }
}

/// Calibration: the oracle on untouched balls (and on a ball rotated).
#[test]
fn probe_oracle_calibrates_on_untouched_balls() {
    for rot in [None, Some((Vec3::new(1.0, 0.3, 0.2), 0.9))] {
        let c = Vec3::new(0.3, -0.2, 0.1);
        let b = ball(1.3, c, rot);
        let s = Sph { c: Point3::origin() + c, r: 1.3 };
        let far = Sph { c: Point3::new(100.0, 0.0, 0.0), r: 1.0 };
        // a lone ball is "A ∪ (far ball)" restricted to A's sphere.
        let faces = faces_of(&b);
        let mut rng = Lcg(7);
        let mut bad = 0;
        for _ in 0..2000 {
            let q = s.c + rng.unit() * s.r;
            let pole = rng.unit();
            let k = faces.iter().filter(|f| in_face(f, q, pole)).count();
            if k != 1 {
                bad += 1;
            }
        }
        let _ = far;
        assert_eq!(bad, 0, "calibration rot={rot:?}");
    }
}

fn pose_rows() -> Vec<(String, f64, Vec3<f64>, f64, Vec3<f64>, Option<(Vec3<f64>, f64)>, Option<(Vec3<f64>, f64)>)> {
    let base = Vec3::new(2.0, 2.0, 0.5);
    let mut v = Vec::new();
    for scale in [1e-3, 1.0, 1e3] {
        for (name, r1, r2, off) in [
            ("equal x", 1.0, 1.0, Vec3::new(1.4, 0.0, 0.0)),
            ("unequal xy", 1.0, 0.7, Vec3::new(0.9, 0.5, 0.0)),
            ("big second", 0.6, 1.0, Vec3::new(-0.7, 0.6, 0.0)),
            ("nearly tangent", 1.0, 0.8, Vec3::new(1.8 - 1e-3, 0.0, 0.0)),
            ("nearly inside", 1.0, 0.4, Vec3::new(0.6 + 1e-3, 0.1, 0.0)),
            ("deep inside", 1.0, 0.4, Vec3::new(0.3, 0.55, 0.0)),
            ("tiny", 1.0, 0.05, Vec3::new(1.0, 0.02, 0.0)),
            ("neg y", 1.0, 0.9, Vec3::new(0.3, -1.2, 0.0)),
        ] {
            v.push((
                format!("{name} ×{scale}"),
                r1 * scale,
                base * scale,
                r2 * scale,
                (base + off) * scale,
                None,
                None,
            ));
        }
    }
    v
}

fn exercise(rows: Vec<(String, f64, Vec3<f64>, f64, Vec3<f64>, Option<(Vec3<f64>, f64)>, Option<(Vec3<f64>, f64)>)>) {
    let mut report = Vec::new();
    let mut fails = Vec::new();
    for (name, r1, c1, r2, c2, rot1, rot2) in rows {
        let a = ball(r1, c1, rot1);
        let b = ball(r2, c2, rot2);
        let sa = Sph { c: Point3::origin() + c1, r: r1 };
        let sb = Sph { c: Point3::origin() + c2, r: r2 };
        let l = lens(r1, r2, (c2 - c1).norm());
        let (va, vb) = (4.0 / 3.0 * PI * r1.powi(3), 4.0 / 3.0 * PI * r2.powi(3));
        for (lab, op, x, y, amb, exp) in [
            ("∪", BooleanOp::Union, &a, &b, true, va + vb - l),
            ("∩", BooleanOp::Intersect, &a, &b, true, l),
            ("A∖B", BooleanOp::Subtract, &a, &b, true, va - l),
            ("B∖A", BooleanOp::Subtract, &b, &a, false, vb - l),
            ("B∪A", BooleanOp::Union, &b, &a, true, va + vb - l),
        ] {
            match run(op, x, y) {
                Err(e) => report.push(format!("{name} {lab}: REFUSED {e:?}")),
                Ok(out) => {
                    let Some(bb) = out.body() else {
                        report.push(format!("{name} {lab}: EMPTY"));
                        continue;
                    };
                    let body = &bb.body;
                    let v = topo::validate(body).is_ok()
                        && topo::validate_closed(body).is_ok()
                        && topo::validate_geometric(body, Tol::witness()).is_ok();
                    let vol = topo::mass_properties(body, Tol::witness()).map(|p| p.volume);
                    let (bad, n) = oracle(body, sa, sb, op, amb, 11);
                    let volok = match &vol {
                        Ok(v) => (v - exp).abs() <= 1e-8 * exp.abs().max(r1.powi(3)),
                        Err(_) => false,
                    };
                    let line = format!(
                        "{name} {lab}: valid={v} vol={vol:?} oracle={exp:.12e} mc_bad={bad}/{n}"
                    );
                    if !v || !volok || bad > n / 500 {
                        fails.push(line.clone());
                    }
                    report.push(line);
                }
            }
        }
    }
    for l in &report {
        eprintln!("{l}");
    }
    assert!(fails.is_empty(), "FAILS:\n{}", fails.join("\n"));
}

#[test]
fn probe_widened_poses() {
    exercise(pose_rows());
}

/// Charts re-posed: each ball rotated so its polar axis is off world y.
#[test]
fn probe_rotated_charts() {
    let mut rows = Vec::new();
    let base = Vec3::new(2.0, 2.0, 0.5);
    for (name, r2, off, rot1, rot2) in [
        ("A about x 90°, offset in A's seam?", 0.8, Vec3::new(1.2, 0.0, 0.0), Some((Vec3::new(1.0, 0.0, 0.0), PI / 2.0)), Some((Vec3::new(1.0, 0.0, 0.0), PI / 2.0))),
        ("both about z 30°", 0.8, Vec3::new(1.2, 0.3, 0.0), Some((Vec3::new(0.0, 0.0, 1.0), 0.52)), Some((Vec3::new(0.0, 0.0, 1.0), 0.52))),
        ("only B about z 40°", 0.8, Vec3::new(1.2, 0.0, 0.0), None, Some((Vec3::new(0.0, 0.0, 1.0), 0.7))),
        ("A about y 90°", 0.7, Vec3::new(1.1, 0.2, 0.0), Some((Vec3::new(0.0, 1.0, 0.0), PI / 2.0)), None),
        ("generic both", 0.9, Vec3::new(1.0, 0.4, 0.3), Some((Vec3::new(1.0, 2.0, 3.0), 0.8)), Some((Vec3::new(-2.0, 1.0, 0.5), 1.9))),
        ("A about y 33° offset z", 0.8, Vec3::new(0.0, 0.3, 1.2), Some((Vec3::new(0.0, 1.0, 0.0), 0.57)), None),
    ] {
        rows.push((name.to_string(), 1.0, base, r2, base + off, rot1, rot2));
    }
    exercise(rows);
}

/// A carved body reused as an operand, and point_in_solid on one.
#[test]
fn probe_reuse_and_classify() {
    let base = Vec3::new(2.0, 2.0, 0.5);
    let a = ball(1.0, base, None);
    let b = ball(0.8, base + Vec3::new(1.2, 0.3, 0.0), None);
    let u = run(BooleanOp::Union, &a, &b).unwrap();
    let ub = &u.body().unwrap().body;
    let q = Point3::origin() + base;
    eprintln!("point_in_solid: {:?}", topo::point_in_solid(ub, q, geom_core::Band::linear(Tol::witness()).unwrap(), Tol::witness()).map(|_| ()));
    for (lab, c) in [("far", Vec3::new(10.0, 0.0, 0.0)), ("third", base + Vec3::new(0.0, 1.2, 0.0)), ("inside", base + Vec3::new(0.1, 0.0, 0.0))] {
        let cball = ball(0.3, c, None);
        for op in [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract] {
            let r = run(op, ub, &cball);
            eprintln!("reuse {lab} {op:?}: {:?}", r.as_ref().map(|o| o.body().map(|b| topo::mass_properties(&b.body, Tol::witness()).map(|p| p.volume))).map_err(|e| e));
        }
    }
}

/// Is a sense bit that disagrees with a tilted-circle face's traversal
/// caught anywhere? `sphere_circle_loop`'s doc says the area-range check
/// fails such a loop; the complement is always in range.
#[test]
fn probe_flipped_sense_on_a_tilted_face() {
    let base = Vec3::new(2.0, 2.0, 0.5);
    let a = ball(1.0, base, None);
    let b = ball(0.8, base + Vec3::new(1.2, 0.3, 0.0), None);
    let u = run(BooleanOp::Union, &a, &b).unwrap();
    let ub = &u.body().unwrap().body;
    let v0 = topo::mass_properties(ub, Tol::witness()).unwrap().volume;
    for (fk, _f) in ub.faces() {
        let flipped = ub.flipped_face_sense_for_tests(fk).unwrap();
        let t1 = topo::validate(&flipped).is_ok();
        let t2 = topo::validate_closed(&flipped).is_ok();
        let t3 = topo::validate_geometric(&flipped, Tol::witness());
        let v = topo::mass_properties(&flipped, Tol::witness()).map(|p| p.volume);
        eprintln!("flip {fk:?}: t1={t1} t2={t2} t3={:?} vol={v:?} (true {v0})", t3.as_ref().map(|_| ()).map_err(|e| e.iter().map(|x| format!("{x:?}").chars().take(80).collect::<String>()).collect::<Vec<_>>()));
    }
}
