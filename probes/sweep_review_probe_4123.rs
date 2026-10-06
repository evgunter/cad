//! Reviewer probe for PR #4123 (not for merge): booleans of partial-turn
//! sphere bodies against bricks, every op, both orders, three scales,
//! two poses; each result sampled by `point_in_solid` against an
//! independent membership oracle (the profile revolved through `Θ`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::TAU;

use crate::revolve_common::{axis_y, validated};
use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use profile::{ArcSweep, bulge_from_center, test_support::bulge_loop};
use sweep::{Revolution, revolve};
use topo::boolean::BooleanOp;
use topo::{AtRestBody, Body, SolidContainment, point_in_solid, transform_rigid};

struct Fx {
    name: &'static str,
    chain: Vec<(Point2<f64>, Option<Point2<f64>>)>,
    theta: f64,
    y: (f64, f64),
    radius_at: fn(f64) -> f64,
}

const RB: f64 = 0.9;
fn ball(y: f64) -> f64 {
    (RB * RB - y * y).max(0.0).sqrt()
}

fn fixtures() -> Vec<Fx> {
    let mut out = Vec::new();
    for theta in [0.7, 3.5, 5.9] {
        out.push(Fx {
            name: "cap to the south pole",
            chain: vec![
                (Point2::new(0.0, -RB), Some(Point2::new(0.0, 0.0))),
                (Point2::new(ball(0.4), 0.4), None),
                (Point2::new(0.0, 0.4), None),
            ],
            theta,
            y: (-RB, 0.4),
            radius_at: |y| if y <= 0.4 { ball(y) } else { 0.0 },
        });
    }
    for theta in [1.5, 5.9] {
        out.push(Fx {
            name: "two-rim zone",
            chain: vec![
                (Point2::new(0.0, -0.6), None),
                (Point2::new(ball(-0.6), -0.6), Some(Point2::new(0.0, 0.0))),
                (Point2::new(ball(0.3), 0.3), None),
                (Point2::new(0.0, 0.3), None),
            ],
            theta,
            y: (-0.6, 0.3),
            radius_at: ball,
        });
    }
    out
}

fn build(f: &Fx, s: f64) -> Body<f64> {
    let n = f.chain.len();
    let at = |p: Point2<f64>| Point2::new(p.x * s, p.y * s);
    let chain = (0..n)
        .map(|i| {
            let (p, c) = f.chain[i];
            let q = f.chain[(i + 1) % n].0;
            let bulge = c.map_or(0.0, |c| bulge_from_center(at(p), at(q), at(c), ArcSweep::Ccw));
            (at(p), bulge)
        })
        .collect();
    revolve(&validated(vec![bulge_loop(chain)]), axis_y(), Revolution::Partial(f.theta), Tol::witness())
        .unwrap()
        .body
}

fn sense(body: &Body<f64>, theta: f64) -> f64 {
    let (s, c) = theta.sin_cos();
    for (_, p) in body.vertex_points() {
        let rho = p.x.hypot(p.z);
        if rho > 1e-9 * 1e-3 && (p.x - rho * c).abs() < 1e-9 * rho.max(1.0) && p.z.abs() > 1e-6 * rho {
            return (p.z / (rho * s)).signum();
        }
    }
    panic!("no end-plane vertex")
}

/// Unit-scale body-frame membership of the revolved body.
fn in_a(f: &Fx, sigma: f64, p: Point3<f64>) -> bool {
    let rho = p.x.hypot(p.z);
    let mut phi = (sigma * p.z).atan2(p.x);
    if phi < 0.0 {
        phi += TAU;
    }
    p.y >= f.y.0 && p.y <= f.y.1 && rho <= (f.radius_at)(p.y) && phi <= f.theta
}

type Brick = ((f64, f64), (f64, f64), (f64, f64));
fn in_b(b: &Brick, p: Point3<f64>) -> bool {
    (b.0.0..=b.0.1).contains(&p.x) && (b.1.0..=b.1.1).contains(&p.y) && (b.2.0..=b.2.1).contains(&p.z)
}

/// Bricks in the unit body frame, around azimuth `phi` (in the revolve's
/// own sense) at radius `r`, height `y`, half-size `h`.
fn brick_at(sigma: f64, phi: f64, r: f64, y: f64, h: f64) -> Brick {
    let (x, z) = (r * phi.cos(), sigma * r * phi.sin());
    ((x - h, x + h), (y - h, y + h), (z - h, z + h))
}

fn bricks(f: &Fx, sigma: f64) -> Vec<(&'static str, Brick)> {
    let gap = TAU - f.theta;
    let mid = f.theta + 0.5 * gap;
    let ym = 0.5 * (f.y.0 + f.y.1);
    let rr = (f.radius_at)(ym);
    vec![
        // In the missing sector, inside the latitude ring the zone claims.
        ("gap", brick_at(sigma, mid, 0.75 * rr, ym, (0.3 * rr * (0.5 * gap).sin()).min(0.1))),
        // Across the sphere face, mid-window.
        ("face", brick_at(sigma, 0.5 * f.theta, rr, ym, 0.12)),
        // Just outside the sphere face in the gap, past the window's end.
        ("gap-rim", brick_at(sigma, f.theta + 0.25 * gap, rr * 1.02, ym, (0.2 * rr * (0.25 * gap).sin()).min(0.08))),
        // Across the start end-plane and the sphere.
        ("seam", ((0.5 * rr, 1.1 * rr), (ym - 0.1, ym + 0.1), (-0.15, 0.15))),
        // Across the far end-plane and the sphere.
        ("far-end", brick_at(sigma, f.theta, rr, ym, 0.12)),
        // Across the top rim, mid-window.
        ("rim", brick_at(sigma, 0.5 * f.theta, 0.8 * (f.radius_at)(f.y.1 - 1e-9).max(0.3), f.y.1, 0.1)),
        // Around the bottom of the body.
        ("bottom", ((-0.15, 0.15), (f.y.0 - 0.1, f.y.0 + 0.12), (-0.15, 0.15))),
    ]
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
}

#[test]
#[ignore = "review probe"]
fn probe_booleans_against_partial_turn_sphere_bodies() {
    let tol = Tol::witness();
    let (mut ok, mut refused, mut wrong, mut samples) = (0, 0, 0, 0);
    let mut refusals: std::collections::BTreeMap<String, usize> = Default::default();
    let scales: Vec<f64> = std::env::var("PROBE_SCALES")
        .map(|v| v.split(',').map(|x| x.parse().unwrap()).collect())
        .unwrap_or(vec![1e-3, 1.0, 1e3]);
    for f in fixtures() {
        let sigma = sense(&build(&f, 1.0), f.theta);
        for &s in &scales {
            if s < 1e4 * tol.eps() {
                continue;
            }
            for (pose, map) in [
                ("upright", Affine3::identity()),
                (
                    "skew",
                    Affine3::translation(Vec3::new(0.3 * s, -0.7 * s, 0.2 * s))
                        * Affine3::rotation_about_axis(Point3::origin(), Vec3::new(1.0, 2.0, 3.0), 1.1),
                ),
            ] {
                let a = match transform_rigid(&build(&f, s), &map, tol)
                    .map_err(|e| format!("{e:?}"))
                    .and_then(|b| AtRestBody::validate(b, tol).map_err(|e| format!("{e:?}")))
                {
                    Ok(a) => a,
                    Err(e) => {
                        eprintln!("STAND-DOWN A {} {} s={s} {pose}: {e:.200}", f.name, f.theta);
                        continue;
                    }
                };
                for (bname, bb) in bricks(&f, sigma) {
                    let sc = |(lo, hi): (f64, f64)| (lo * s, hi * s);
                    let b = match transform_rigid(&topo::test_support::brick(sc(bb.0), sc(bb.1), sc(bb.2), tol), &map, tol)
                        .map_err(|e| format!("{e:?}"))
                        .and_then(|b| AtRestBody::validate(b, tol).map_err(|e| format!("{e:?}")))
                    {
                        Ok(b) => b,
                        Err(e) => {
                            eprintln!("STAND-DOWN B {bname} s={s} {pose}: {e:.200}");
                            continue;
                        }
                    };
                    for (opname, op, swap) in [
                        ("A∪B", BooleanOp::Union, false),
                        ("A∩B", BooleanOp::Intersect, false),
                        ("A−B", BooleanOp::Subtract, false),
                        ("B−A", BooleanOp::Subtract, true),
                        ("B∪A", BooleanOp::Union, true),
                    ] {
                        let what = format!("{} Θ={} {bname} {opname} s={s} {pose}", f.name, f.theta);
                        let (x, y) = if swap { (&b, &a) } else { (&a, &b) };
                        let res = match op {
                            BooleanOp::Union => topo::union(x, y, tol),
                            BooleanOp::Intersect => topo::intersect(x, y, tol),
                            BooleanOp::Subtract => topo::subtract(x, y, tol),
                        };
                        let truth = |p: Point3<f64>| {
                            let (ia, ib) = (in_a(&f, sigma, p), in_b(&bb, p));
                            match (opname, ia, ib) {
                                ("A∪B" | "B∪A", ia, ib) => ia || ib,
                                ("A∩B", ia, ib) => ia && ib,
                                ("A−B", ia, ib) => ia && !ib,
                                (_, ia, ib) => ib && !ia,
                            }
                        };
                        let body = match res {
                            Ok(topo::BooleanResult::Body(out)) => Some(out.body),
                            Ok(other) => {
                                let d = format!("{other:?}");
                                if d.starts_with("Empty") { None } else { panic!("{what}: unexpected {d:.200}") }
                            }
                            Err(e) => {
                                refused += 1;
                                let k = format!("{:?}", e.kind());
                                *refusals.entry(k.clone()).or_default() += 1;
                                eprintln!("REFUSED {what}: {e:.300}");
                                continue;
                            }
                        };
                        // Sample the joint box, away from every boundary.
                        let lo = Point3::new(bb.0.0.min(-RB), bb.1.0.min(f.y.0), bb.2.0.min(-RB));
                        let hi = Point3::new(bb.0.1.max(RB), bb.1.1.max(f.y.1), bb.2.1.max(RB));
                        let mut rng = Rng(0x4123 ^ (s.to_bits()));
                        let mut bad_here = 0;
                        let mut n = 0;
                        // Points biased into the brick and the body.
                        while n < 48 {
                            let pick = |rng: &mut Rng, a: f64, b: f64| a + (b - a) * rng.next();
                            let p = if n % 2 == 0 {
                                Point3::new(pick(&mut rng, bb.0.0, bb.0.1), pick(&mut rng, bb.1.0, bb.1.1), pick(&mut rng, bb.2.0, bb.2.1))
                            } else {
                                Point3::new(pick(&mut rng, lo.x, hi.x), pick(&mut rng, lo.y, hi.y), pick(&mut rng, lo.z, hi.z))
                            };
                            let t = truth(p);
                            let d = 2e-3;
                            if [Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()]
                                .iter()
                                .any(|e| truth(p + *e * d) != t || truth(p - *e * d) != t)
                            {
                                continue;
                            }
                            n += 1;
                            samples += 1;
                            let q = map.transform_point(Point3::new(p.x * s, p.y * s, p.z * s));
                            let got = match &body {
                                None => Ok(SolidContainment::Out),
                                Some(b) => point_in_solid(b, q, band(), tol),
                            };
                            let want = if t { SolidContainment::In } else { SolidContainment::Out };
                            match got {
                                Ok(g) if g == want => {}
                                Ok(g) => {
                                    bad_here += 1;
                                    eprintln!("WRONG {what}: {p:?} oracle {want:?} kernel {g:?}");
                                }
                                Err(e) => eprintln!("PIS-REFUSED {what}: {e:.150}"),
                            }
                        }
                        if bad_here > 0 {
                            wrong += 1;
                        } else {
                            ok += 1;
                        }
                    }
                }
            }
        }
    }
    eprintln!("ok {ok}, refused {refused}, wrong {wrong}, samples {samples}; refusals {refusals:?}");
    assert_eq!(wrong, 0);
}

// ---------------------------------------------------------------------
// Splits: cuts clear of / into the sphere face along random directions.
// ---------------------------------------------------------------------

use geom_core::UnitVec3;
use topo::splitting::{SplitPart, SplitPlane, split};

/// The sphere face's own support along unit `n` (unit body frame), by a
/// dense sample of its patch: centre origin, radius `RB`, latitudes
/// `[v0, v1]`, azimuths `[0, Θ]` in the revolve's sense `σ`.
fn face_support(f: &Fx, sigma: f64, n: Vec3<f64>) -> (f64, f64) {
    let v0 = (f.y.0 / RB).clamp(-1.0, 1.0).asin();
    let v1 = (f.y.1 / RB).clamp(-1.0, 1.0).asin();
    let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
    let k = 700;
    for i in 0..=k {
        let v = v0 + (v1 - v0) * f64::from(i) / f64::from(k);
        for j in 0..=k {
            let phi = f.theta * f64::from(j) / f64::from(k);
            let p = Vec3::new(RB * v.cos() * phi.cos(), RB * v.sin(), sigma * RB * v.cos() * phi.sin());
            let x = n.dot(p);
            lo = lo.min(x);
            hi = hi.max(x);
        }
    }
    (lo, hi)
}

#[test]
#[ignore = "review probe"]
fn probe_splits_of_partial_turn_sphere_bodies() {
    let tol = Tol::witness();
    let scales: Vec<f64> = std::env::var("PROBE_SCALES")
        .map(|v| v.split(',').map(|x| x.parse().unwrap()).collect())
        .unwrap_or(vec![1e-3, 1.0, 1e3]);
    let mut tally: std::collections::BTreeMap<String, usize> = Default::default();
    let mut wrong = 0;
    let mut rng = Rng(0xA2F1);
    let dirs: Vec<Vec3<f64>> = (0..24)
        .map(|_| loop {
            let v = Vec3::new(2.0 * rng.next() - 1.0, 2.0 * rng.next() - 1.0, 2.0 * rng.next() - 1.0);
            if v.norm() > 0.2 && v.norm() <= 1.0 {
                break v * (1.0 / v.norm());
            }
        })
        .collect();
    for f in fixtures() {
        let sigma = sense(&build(&f, 1.0), f.theta);
        let supports: Vec<(Vec3<f64>, (f64, f64))> = dirs.iter().map(|&n| (n, face_support(&f, sigma, n))).collect();
        for &s in &scales {
            if s < 1e4 * tol.eps() {
                continue;
            }
            for (pose, map) in [
                ("upright", Affine3::identity()),
                (
                    "skew",
                    Affine3::translation(Vec3::new(0.3 * s, -0.7 * s, 0.2 * s))
                        * Affine3::rotation_about_axis(Point3::origin(), Vec3::new(1.0, 2.0, 3.0), 1.1),
                ),
            ] {
                let Ok(a) = transform_rigid(&build(&f, s), &map, tol)
                    .map_err(|e| format!("{e:?}"))
                    .and_then(|b| AtRestBody::validate(b, tol).map_err(|e| format!("{e:?}")))
                else {
                    *tally.entry("stand-down body".into()).or_default() += 1;
                    continue;
                };
                for &(n, (lo, hi)) in &supports {
                    for (kind, d) in [
                        ("clear", hi + 1e-3),
                        ("clear", hi + 1e-5 + 30.0 * tol.eps() / s),
                        ("clear", lo - 1e-3),
                        ("clear", lo - 1e-5 - 30.0 * tol.eps() / s),
                        ("into", hi - 1e-3),
                        ("into", hi - 1e-5 - 30.0 * tol.eps() / s),
                        ("into", lo + 1e-3),
                        ("into", lo + 1e-5 + 30.0 * tol.eps() / s),
                    ] {
                        let what = format!("{} Θ={} s={s} {pose} n={n:?} d={d} ({kind})", f.name, f.theta);
                        let cut = SplitPlane {
                            origin: map.transform_point(Point3::new(n.x * d * s, n.y * d * s, n.z * d * s)),
                            normal: UnitVec3::new(map.transform_vec(n), topo::DATUM_UNIT_NORM, band()).unwrap(),
                        };
                        let res = match split(&a, &cut, tol) {
                            Ok(r) => r,
                            Err(e) => {
                                let m = format!("{e:?}");
                                let k = if m.contains("CurvedBooleanUnsupported") && m.contains("Sphere") {
                                    "gate(Sphere)"
                                } else {
                                    "other"
                                };
                                if kind == "clear" {
                                    eprintln!("CLEAR-REFUSED {what}: {m:.250}");
                                }
                                *tally.entry(format!("{kind} refused {k}")).or_default() += 1;
                                continue;
                            }
                        };
                        *tally.entry(format!("{kind} split")).or_default() += 1;
                        // Each half against the oracle.
                        let mut bad = 0;
                        for (part, sign) in [(&res.above, 1.0), (&res.below, -1.0)] {
                            let mut m = 0;
                            let mut tries = 0;
                            while m < 16 && tries < 4000 {
                                tries += 1;
                                // Sample near the face and the plane.
                                let p = Point3::new(
                                    (2.0 * rng.next() - 1.0) * RB,
                                    f.y.0 + (f.y.1 - f.y.0) * rng.next(),
                                    (2.0 * rng.next() - 1.0) * RB,
                                );
                                let truth = |p: Point3<f64>| in_a(&f, sigma, p) && sign * (n.dot(p - Point3::origin()) - d) > 0.0;
                                let t = truth(p);
                                let dd = 2e-3;
                                if [Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()]
                                    .iter()
                                    .any(|e| truth(p + *e * dd) != t || truth(p - *e * dd) != t)
                                {
                                    continue;
                                }
                                // Bias: half of the samples must be inside.
                                if m % 2 == 0 && !t {
                                    continue;
                                }
                                m += 1;
                                let q = map.transform_point(Point3::new(p.x * s, p.y * s, p.z * s));
                                let got = match part {
                                    SplitPart::Body(b) => point_in_solid(b, q, band(), tol),
                                    _ => Ok(SolidContainment::Out),
                                };
                                let want = if t { SolidContainment::In } else { SolidContainment::Out };
                                match got {
                                    Ok(g) if g == want => {}
                                    Ok(g) => {
                                        bad += 1;
                                        eprintln!("WRONG {what} half {sign}: {p:?} oracle {want:?} kernel {g:?}");
                                    }
                                    Err(e) => eprintln!("PIS-REFUSED {what}: {e:.120}"),
                                }
                            }
                        }
                        if bad > 0 {
                            wrong += 1;
                        }
                    }
                }
            }
        }
    }
    eprintln!("splits: {tally:?}; wrong {wrong}");
    assert_eq!(wrong, 0);
}

// ---------------------------------------------------------------------
// Sphere × sphere: the partial-turn bodies against a whole ball placed
// in the azimuth gap, across the sphere face, and around the axis.
// ---------------------------------------------------------------------

fn whole_ball(c: Point3<f64>, r: f64, s: f64) -> Body<f64> {
    let at = |x: f64, y: f64| Point2::new(x * s, y * s);
    let chain = vec![
        (at(0.0, -r), bulge_from_center(at(0.0, -r), at(0.0, r), at(0.0, 0.0), ArcSweep::Ccw)),
        (at(0.0, r), 0.0),
    ];
    let b = revolve(&validated(vec![bulge_loop(chain)]), axis_y(), Revolution::Full, Tol::witness())
        .unwrap()
        .body;
    transform_rigid(&b, &Affine3::translation(Vec3::new(c.x * s, c.y * s, c.z * s)), Tol::witness()).unwrap()
}

#[test]
#[ignore = "review probe"]
fn probe_sphere_sphere_booleans() {
    let tol = Tol::witness();
    let (mut ok, mut refused, mut wrong) = (0, 0, 0);
    let mut refusals: std::collections::BTreeMap<String, usize> = Default::default();
    let scales: Vec<f64> = std::env::var("PROBE_SCALES")
        .map(|v| v.split(',').map(|x| x.parse().unwrap()).collect())
        .unwrap_or(vec![1e-3, 1.0, 1e3]);
    for f in fixtures() {
        let sigma = sense(&build(&f, 1.0), f.theta);
        let gap = TAU - f.theta;
        let ym = 0.5 * (f.y.0 + f.y.1);
        let rr = (f.radius_at)(ym);
        let on = |phi: f64, r: f64| Point3::new(r * phi.cos(), ym, sigma * r * phi.sin());
        let balls = [
            ("ball in gap", on(f.theta + 0.5 * gap, 0.7 * rr), (0.6 * rr * (0.5 * gap).sin()).min(0.25)),
            ("ball across face", on(0.5 * f.theta, rr), 0.2),
            ("ball outside gap rim", on(f.theta + 0.5 * gap, 1.05 * rr + 0.12), 0.1),
        ];
        for &s in &scales {
            if s < 1e4 * tol.eps() {
                continue;
            }
            let Ok(a) = AtRestBody::validate(build(&f, s), tol) else { continue };
            for (bname, c, r) in balls {
                let Ok(b) = AtRestBody::validate(whole_ball(c, r, s), tol) else {
                    eprintln!("STAND-DOWN {bname}");
                    continue;
                };
                for (opname, swap) in [("A∪B", false), ("A∩B", false), ("A−B", false), ("B−A", true)] {
                    let what = format!("{} Θ={} {bname} {opname} s={s}", f.name, f.theta);
                    let (x, y) = if swap { (&b, &a) } else { (&a, &b) };
                    let res = match opname {
                        "A∪B" => topo::union(x, y, tol),
                        "A∩B" => topo::intersect(x, y, tol),
                        _ => topo::subtract(x, y, tol),
                    };
                    let truth = |p: Point3<f64>| {
                        let (ia, ib) = (in_a(&f, sigma, p), (p - c).norm() <= r);
                        match opname {
                            "A∪B" => ia || ib,
                            "A∩B" => ia && ib,
                            "A−B" => ia && !ib,
                            _ => ib && !ia,
                        }
                    };
                    let body = match res {
                        Ok(topo::BooleanResult::Body(out)) => Some(out.body),
                        Ok(other) if format!("{other:?}").starts_with("Empty") => None,
                        Ok(other) => panic!("{what}: {other:?}"),
                        Err(e) => {
                            refused += 1;
                            *refusals.entry(format!("{:?}", e.kind())).or_default() += 1;
                            eprintln!("REFUSED {what}: {e:.200}");
                            continue;
                        }
                    };
                    let mut rng = Rng(0x55 ^ s.to_bits());
                    let mut bad = 0;
                    let mut n = 0;
                    while n < 48 {
                        let p = if n % 2 == 0 {
                            c + Vec3::new(2.0 * rng.next() - 1.0, 2.0 * rng.next() - 1.0, 2.0 * rng.next() - 1.0) * r
                        } else {
                            Point3::new((2.0 * rng.next() - 1.0) * RB, f.y.0 + (f.y.1 - f.y.0) * rng.next(), (2.0 * rng.next() - 1.0) * RB)
                        };
                        let t = truth(p);
                        let d = 2e-3;
                        if [Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()]
                            .iter()
                            .any(|e| truth(p + *e * d) != t || truth(p - *e * d) != t)
                        {
                            continue;
                        }
                        n += 1;
                        let q = Point3::new(p.x * s, p.y * s, p.z * s);
                        let got = match &body {
                            None => Ok(SolidContainment::Out),
                            Some(b) => point_in_solid(b, q, band(), tol),
                        };
                        let want = if t { SolidContainment::In } else { SolidContainment::Out };
                        match got {
                            Ok(g) if g == want => {}
                            Ok(g) => {
                                bad += 1;
                                eprintln!("WRONG {what}: {p:?} oracle {want:?} kernel {g:?}");
                            }
                            Err(e) => eprintln!("PIS-REFUSED {what}: {e:.120}"),
                        }
                    }
                    if bad > 0 { wrong += 1 } else { ok += 1 }
                }
            }
        }
    }
    eprintln!("sphere×sphere: ok {ok}, refused {refused}, wrong {wrong}; {refusals:?}");
    assert_eq!(wrong, 0);
}
