//! Review r1 probes for PR 4344 (two spheres crossing off every edge cut
//! in). Every build is read through `common::differential::outcome`
//! (tiers 2 and 3′, the certificate, the legal-operand check, volume)
//! against a lens volume derived here independently of the radical
//! plane (the textbook closed form in `R`, `r`, `d`), and tessellated
//! with `mesh::validate::check_mesh`. Each line also carries the face,
//! edge and vertex counts and how many faces lie on each sphere, so a
//! needless meridian is visible. Rows print and fail only on a BAD or
//! wrong body, so a refusal is a printed line, not a red row.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::f64::consts::PI;

use geom_core::{Affine3, Point3, Tol, Vec3};
use sweep::test_support::{ball_poled_y, brick, finished};
use topo::{AtRestBody, BooleanDeclarations, BooleanError, BooleanResult};

use crate::common::differential::outcome;

fn ballv(r: f64) -> f64 {
    4.0 / 3.0 * PI * r.powi(3)
}

/// The lens of balls `big`, `small` at centre distance `d`, in the
/// textbook form that never forms the radical plane.
fn lens(big: f64, small: f64, d: f64) -> f64 {
    if d >= big + small {
        return 0.0;
    }
    if d <= (big - small).abs() {
        return ballv(big.min(small));
    }
    PI * (big + small - d).powi(2)
        * (d * d + 2.0 * d * small - 3.0 * small * small + 2.0 * d * big + 6.0 * small * big
            - 3.0 * big * big)
        / (12.0 * d)
}

fn capv(r: f64, h: f64) -> f64 {
    PI * h * h * (3.0 * r - h) / 3.0
}

/// A `y`-poled ball turned by `turn` about its centre, then placed at `c`.
fn ball_turned(r: f64, c: Vec3<f64>, turn: Option<(Vec3<f64>, f64)>) -> AtRestBody<f64> {
    let tol = Tol::witness();
    let mut at = ball_poled_y(r, Vec3::new(0.0, 0.0, 0.0), tol);
    if let Some((axis, angle)) = turn {
        let m = Affine3::rotation_about_axis(Point3::origin(), axis / axis.norm(), angle);
        at = topo::transform_rigid(&at, &m, tol).unwrap();
    }
    let moved = topo::transform_rigid(&at, &Affine3::translation(c), tol).unwrap();
    finished("the ball", moved, tol)
}

fn ball(r: f64, c: Vec3<f64>) -> AtRestBody<f64> {
    ball_turned(r, c, None)
}

fn shape(body: &topo::Body<f64>) -> String {
    let mut spheres: Vec<((i64, i64, i64, i64), usize)> = Vec::new();
    for (_, fd) in body.faces() {
        if let Some(geom::Surface::Sphere { center, radius, .. }) = body.get_surface(fd.surface) {
            let k = |x: f64| (x * 1e6).round() as i64;
            let key = (k(center.x), k(center.y), k(center.z), k(*radius));
            match spheres.iter_mut().find(|(s, _)| *s == key) {
                Some((_, n)) => *n += 1,
                None => spheres.push((key, 1)),
            }
        }
    }
    let per: Vec<String> = spheres
        .iter()
        .map(|(k, n)| format!("r{}:{n}", k.3 as f64 / 1e6))
        .collect();
    format!(
        "F{} E{} V{} S{} sph[{}]",
        body.faces().count(),
        body.edges().count(),
        body.vertices().count(),
        body.shells().count(),
        per.join(",")
    )
}

/// One op's line; returns whether it is wrong (BAD, a wrong empty, an
/// unmeasured body, or a mesh that does not check).
fn line(
    pose: &str,
    op: &str,
    r: Result<BooleanResult<f64>, BooleanError>,
    want: f64,
    tol: Tol,
) -> bool {
    let extra = match &r {
        Ok(res) => match res.body() {
            Some(bb) => {
                let mesh = match mesh::tessellate(&bb.body, 5e-3, tol) {
                    Ok(m) => match mesh::validate::check_mesh(&m) {
                        Ok(()) => "mesh=ok".to_string(),
                        Err(e) => format!("mesh=BAD {e:?}"),
                    },
                    Err(e) => format!("mesh=ERR {:.80}", format!("{e:?}")),
                };
                format!("{} {mesh}", shape(&bb.body))
            }
            None => String::new(),
        },
        Err(_) => String::new(),
    };
    let o = outcome(r, want, tol);
    let wrong = o.contains("BAD")
        || o.contains("WRONG")
        || o.contains("UNMEASURED")
        || extra.contains("mesh=BAD");
    println!("R1PROBE {pose} | {op} | {o} | {extra}");
    wrong
}

/// Every op in both orders; `shared` the volume `a` and `b` share.
fn six(pose: &str, a: &AtRestBody<f64>, b: &AtRestBody<f64>, va: f64, vb: f64, shared: f64) -> usize {
    let (tol, none) = (Tol::witness(), BooleanDeclarations::none());
    let mut bad = 0;
    for (op, want, out) in [
        ("a∪b", va + vb - shared, topo::union_with(a, b, &none, tol)),
        ("b∪a", va + vb - shared, topo::union_with(b, a, &none, tol)),
        ("a∖b", va - shared, topo::subtract_with(a, b, &none, tol)),
        ("b∖a", vb - shared, topo::subtract_with(b, a, &none, tol)),
        ("a∩b", shared, topo::intersect_with(a, b, &none, tol)),
        ("b∩a", shared, topo::intersect_with(b, a, &none, tol)),
    ] {
        bad += usize::from(line(pose, op, out, want, tol));
    }
    bad
}

fn done(family: &str, bad: usize) {
    println!("R1PROBE-SUMMARY {family} bad={bad}");
    assert_eq!(bad, 0, "{family}: {bad} wrong bodies");
}

/// Claim 1: the small ball's chart turned off the unit ball's, so the
/// two cut meridian planes are not one plane.
#[test]
fn r1_charts_not_parallel() {
    let mut bad = 0;
    let unit = ball(1.0, Vec3::new(0.0, 0.0, 0.0));
    let x = Vec3::new(1.0, 0.0, 0.0);
    let z = Vec3::new(0.0, 0.0, 1.0);
    let xy = Vec3::new(1.0, 1.0, 0.0);
    let xyz = Vec3::new(1.0, -2.0, 3.0);
    for (r, d) in [(0.3, 0.95), (0.7, 1.2), (1.0, 1.5)] {
        for c in [
            Vec3::new(0.0, 0.0, d),
            Vec3::new(0.0, 0.3, 0.9).normalize() * d,
            Vec3::new(-0.4, 0.2, 0.8).normalize() * d,
        ] {
            for (name, turn) in [
                ("z30", (z, 30f64.to_radians())),
                ("z90", (z, 90f64.to_radians())),
                ("x45", (x, 45f64.to_radians())),
                ("xy60", (xy, 60f64.to_radians())),
                ("xyz110", (xyz, 110f64.to_radians())),
            ] {
                let pose = format!("nonpar r={r} c={:.3},{:.3},{:.3} turn={name}", c.x, c.y, c.z);
                let small = ball_turned(r, c, Some(turn));
                bad += six(&pose, &unit, &small, ballv(1.0), ballv(r), lens(1.0, r, d));
            }
        }
    }
    done("charts_not_parallel", bad);
}

/// Claim 1: a small ball crossing the unit ball near either pole of its
/// chart, its circle's polar distance stepping across the pole's band.
/// The small ball is `z`-poled (turned 90° about x) so its own circle
/// stays off its seam.
#[test]
fn r1_near_a_pole() {
    let mut bad = 0;
    let unit = ball(1.0, Vec3::new(0.0, 0.0, 0.0));
    let (r, d) = (0.3f64, 1.2f64);
    let s = (d * d + 1.0 - r * r) / (2.0 * d);
    let alpha = s.acos();
    let x = Vec3::new(1.0, 0.0, 0.0);
    for pole in [1.0f64, -1.0] {
        for off in [
            0.3, 0.1, 1e-3, 1e-5, 1e-7, 1e-9, 1e-11, -1e-11, -1e-9, -1e-7, -1e-5, -1e-3, -0.05,
        ] {
            let th = alpha + off;
            let c = Vec3::new(0.0, pole * th.cos(), th.sin()) * d;
            let pose = format!("pole={pole} theta-alpha={off:e}");
            let small = ball_turned(r, c, Some((x, PI / 2.0)));
            bad += six(&pose, &unit, &small, ballv(1.0), ballv(r), lens(1.0, r, d));
        }
    }
    done("near_a_pole", bad);
}

/// Claim 1: one ball takes two cut-ins from two partners (the partner
/// operand is two disjoint small balls), with the two cuts on different
/// meridian planes, and on the two halves of one plane.
#[test]
fn r1_three_balls() {
    let tol = Tol::witness();
    let mut bad = 0;
    let unit = ball(1.0, Vec3::new(0.0, 0.0, 0.0));
    for (name, p, q) in [
        ("+z,-x", Vec3::new(0.0, 0.0, 0.95), Vec3::new(-0.95, 0.0, 0.0)),
        ("+z,-z", Vec3::new(0.0, 0.0, 0.95), Vec3::new(0.0, 0.0, -0.95)),
        ("+z,tilt", Vec3::new(0.0, 0.0, 0.95), Vec3::new(-0.6, 0.5, -0.5).normalize() * 0.95),
    ] {
        let pair = topo::union(&ball(0.3, p), &ball(0.3, q), tol);
        let pair = match pair {
            Ok(r) => r.body().unwrap().body.clone(),
            Err(e) => {
                println!("R1PROBE three {name} | partner union | ERR {e:?}");
                continue;
            }
        };
        let pose = format!("three {name}");
        bad += six(&pose, &unit, &pair, ballv(1.0), 2.0 * ballv(0.3), 2.0 * lens(1.0, 0.3, 0.95));
    }
    // One ball crossing two balls of the other operand that overlap each
    // other (a lens union) where neither circle meets the lens's own.
    let lensu = topo::union(&ball(1.0, Vec3::new(0.0, 0.0, 0.0)), &ball(1.0, Vec3::new(1.4, 0.0, 0.0)), tol)
        .unwrap()
        .body()
        .unwrap()
        .body
        .clone();
    let small = ball(0.3, Vec3::new(0.7, 0.0, 0.0) + Vec3::new(0.0, 0.0, 0.95));
    // The small ball at (0.7, 0, 0.95) is 1.181 from each centre; it
    // reaches the lens circle (centre (0.7,0,0), radius 0.714) only if
    // its distance to the circle is < 0.3: the nearest circle point is
    // (0.7,0,0.714), 0.236 away — so it crosses the lens's own circle,
    // and the crossing layer, not the scan, answers. Volume by
    // inclusion–exclusion is not closed-form here; only shape is read.
    let tolv = Tol::witness();
    let none = BooleanDeclarations::none();
    let out = topo::union_with(&lensu, &small, &none, tolv);
    println!(
        "R1PROBE three lens-circle | a∪b (shape only) | {}",
        match &out {
            Ok(r) => r.body().map(|b| shape(&b.body)).unwrap_or_default(),
            Err(e) => format!("ERR {:.110}", format!("{e:?}")),
        }
    );
    done("three_balls", bad);
}

/// Claim 3: radius ratios of 1e-3 and 1e3, crossing at the middle of the
/// range and near both tangencies.
#[test]
fn r1_radius_ratios() {
    let mut bad = 0;
    let unit = ball(1.0, Vec3::new(0.0, 0.0, 0.0));
    for r in [1e-3, 1e3] {
        let (lo, hi) = ((1.0f64 - r).abs(), 1.0 + r);
        for t in [0.5, 0.1, 0.9, 1e-3, 1.0 - 1e-3] {
            let d = lo + t * (hi - lo);
            let pose = format!("ratio r={r:e} t={t}");
            let small = ball(r, Vec3::new(0.0, 0.0, d));
            bad += six(&pose, &unit, &small, ballv(1.0), ballv(r), lens(1.0, r, d));
        }
    }
    done("radius_ratios", bad);
}

/// Claim 3: equal radii nearly coincident (the cancellation `R² − r²`
/// would show), and radii a hair apart.
#[test]
fn r1_near_coincident() {
    let mut bad = 0;
    let unit = ball(1.0, Vec3::new(0.0, 0.0, 0.0));
    for r in [1.0, 1.0 + 1e-7, 1.0 - 1e-7] {
        for d in [1e-2, 1e-3, 1e-4, 1e-5] {
            if d <= (1.0f64 - r).abs() {
                continue;
            }
            let pose = format!("coinc r={r} d={d:e}");
            let small = ball(r, Vec3::new(0.0, 0.0, d));
            bad += six(&pose, &unit, &small, ballv(1.0), ballv(r), lens(1.0, r, d));
        }
    }
    done("near_coincident", bad);
}

/// Claim 2: the plain witness through the cut-in, beside the same pose
/// with the small ball turned so its seam crosses the circle (the
/// ordinary crossing path), for the face and edge counts.
#[test]
fn r1_cut_in_against_ordinary() {
    let mut bad = 0;
    let unit = ball(1.0, Vec3::new(0.0, 0.0, 0.0));
    let c = Vec3::new(0.0, 0.0, 0.95);
    let cut = ball(0.3, c);
    // Turned 90° about x: z-poled, seam in the half-plane y = 0, x > 0,
    // which the small ball's circle (axis z) crosses.
    let ordinary = ball_turned(0.3, c, Some((Vec3::new(1.0, 0.0, 0.0), PI / 2.0)));
    bad += six("path=cut-in", &unit, &cut, ballv(1.0), ballv(0.3), lens(1.0, 0.3, 0.95));
    bad += six("path=ordinary", &unit, &ordinary, ballv(1.0), ballv(0.3), lens(1.0, 0.3, 0.95));
    // The operands themselves.
    println!("R1PROBE operand unit | {}", shape(&unit));
    println!("R1PROBE operand small | {}", shape(&cut));
    done("cut_in_against_ordinary", bad);
}

/// Claims 2 and 4: the re-chart beside a cut-in on another shell of the
/// same operand (two disjoint balls: one pokes a slab's plane face, the
/// other crosses a sphere face off every edge). `apply_recuts` carves
/// the poked ball's shell out and grafts it back; the cut-in names a
/// face of the other shell.
#[test]
fn r1_rechart_beside_a_cut_in_on_another_shell() {
    let tol = Tol::witness();
    let mut bad = 0;
    let two = match topo::union(
        &ball(1.0, Vec3::new(0.0, 0.0, 0.0)),
        &ball(1.0, Vec3::new(5.0, 0.0, 0.0)),
        tol,
    ) {
        Ok(r) => r.body().unwrap().body.clone(),
        Err(e) => {
            println!("R1PROBE rechart | operand | ERR {e:?}");
            return;
        }
    };
    let tool = match topo::union(
        &finished("slab", brick((-2.0, 2.0), (-2.0, 2.0), (0.5, 3.0), tol), tol),
        &ball(0.3, Vec3::new(5.0, 0.0, 0.95)),
        tol,
    ) {
        Ok(r) => r.body().unwrap().body.clone(),
        Err(e) => {
            println!("R1PROBE rechart | tool | ERR {e:?}");
            return;
        }
    };
    println!("R1PROBE rechart operand | {}", shape(&two));
    let shared = capv(1.0, 0.5) + lens(1.0, 0.3, 0.95);
    bad += six("rechart+cut other shell", &two, &tool, 2.0 * ballv(1.0), 40.0 + ballv(0.3), shared);
    done("rechart_beside", bad);
}

/// Claim 3: the plain witness and a non-parallel-chart pose at the
/// `Interval` scalar, at whatever ε the run commits.
#[test]
fn r1_interval() {
    use crate::common::interval::iv;
    use geom_core::{Bounds, Interval};
    let tol = Tol::witness();
    let mk = |r: f64, c: (f64, f64, f64), turn: Option<f64>| -> AtRestBody<Interval> {
        let mut at = ball_poled_y(iv(r), Vec3::new(iv(0.0), iv(0.0), iv(0.0)), tol);
        if let Some(a) = turn {
            let m = Affine3::rotation_about_axis(
                Point3::new(iv(0.0), iv(0.0), iv(0.0)),
                Vec3::new(iv(0.0), iv(0.0), iv(1.0)),
                iv(a),
            );
            at = topo::transform_rigid(&at, &m, tol).unwrap();
        }
        let at = topo::transform_rigid(
            &at,
            &Affine3::translation(Vec3::new(iv(c.0), iv(c.1), iv(c.2))),
            tol,
        )
        .unwrap();
        finished("the ball", at, tol)
    };
    let mut bad = 0;
    for (name, turn, c, d) in [
        ("plain", None, (0.0, 0.0, 0.95), 0.95),
        ("z30", Some(30f64.to_radians()), (0.0, 0.0, 0.95), 0.95),
        ("tilt", None, (0.02, 0.01, 0.9), (0.02f64 * 0.02 + 0.0001 + 0.81).sqrt()),
    ] {
        let a = mk(1.0, (0.0, 0.0, 0.0), None);
        let b = mk(0.3, c, turn);
        let shared = lens(1.0, 0.3, d);
        let (va, vb) = (ballv(1.0), ballv(0.3));
        for (op, want, out) in [
            ("a∪b", va + vb - shared, topo::union(&a, &b, tol)),
            ("b∪a", va + vb - shared, topo::union(&b, &a, tol)),
            ("a∖b", va - shared, topo::subtract(&a, &b, tol)),
            ("b∖a", vb - shared, topo::subtract(&b, &a, tol)),
            ("a∩b", shared, topo::intersect(&a, &b, tol)),
            ("b∩a", shared, topo::intersect(&b, &a, tol)),
        ] {
            let o = match out {
                Err(e) => format!("ERR {:.140}", format!("{e:?}")),
                Ok(r) => match r.body() {
                    None => "EMPTY WRONG".into(),
                    Some(bb) => {
                        let t3 = topo::validate_geometric(&bb.body, tol).is_ok();
                        match topo::mass_properties(&bb.body, tol) {
                            Ok(m) => {
                                let slack = 1e-9 * want.max(1.0);
                                let ok = m.volume.lo() - slack <= want && want <= m.volume.hi() + slack;
                                if !(ok && t3) {
                                    bad += 1;
                                }
                                format!(
                                    "OK {} t3={t3} v=[{:.12},{:.12}] want={want:.12}",
                                    if ok && t3 { "SOUND" } else { "BAD" },
                                    m.volume.lo(),
                                    m.volume.hi()
                                )
                            }
                            Err(e) => {
                                bad += 1;
                                format!("OK-UNMEASURED {e:?}")
                            }
                        }
                    }
                },
            };
            println!("R1PROBE interval {name} eps={:e} | {op} | {o}", tol.eps());
        }
    }
    done("interval", bad);
}

/// Claim 3 / 1: the circle in its own sphere's equatorial plane, which
/// holds that ball's seam (`s = 0`): the seam half-meridian then lies on
/// the other sphere, and nearby depths put the circle within ε of it.
/// Both sides: the small ball's great circle (`d² = 1 − r²`) and the
/// unit ball's (`r² = 1 + d²`).
#[test]
fn r1_seam_on_the_circle() {
    let mut bad = 0;
    let unit = ball(1.0, Vec3::new(0.0, 0.0, 0.0));
    for (side, r, d0) in [
        ("small", 0.3f64, (1.0f64 - 0.09).sqrt()),
        ("unit", (1.25f64).sqrt(), 0.5f64),
    ] {
        for off in [0.0, 1e-12, -1e-12, 1e-9, -1e-9, 1e-7, -1e-7, 1e-5, -1e-5] {
            let d = d0 + off;
            let pose = format!("seamcircle side={side} off={off:e}");
            let small = ball(r, Vec3::new(0.0, 0.0, d));
            bad += six(&pose, &unit, &small, ballv(1.0), ballv(r), lens(1.0, r, d));
        }
    }
    done("seam_on_the_circle", bad);
}

/// A ball turned so its seam plane is normal to `n` (its seam then never
/// meets a circle about `n`), placed at `c`.
fn ball_seam_normal_to(r: f64, c: Vec3<f64>, n: Vec3<f64>) -> AtRestBody<f64> {
    let z = Vec3::new(0.0, 0.0, 1.0);
    let n = n / n.norm();
    let axis = z.cross(n);
    if axis.norm() < 1e-12 {
        return ball(r, c);
    }
    ball_turned(r, c, Some((axis, z.dot(n).clamp(-1.0, 1.0).acos())))
}

/// Does `mesh::tessellate` take a plain ball, and bodies main builds on
/// the ordinary path (an `x` offset: the seam reaches the circle)?
#[test]
fn r1_mesh_baseline() {
    let tol = Tol::witness();
    let unit = ball(1.0, Vec3::new(0.0, 0.0, 0.0));
    for chordal in [5e-3, 2e-2] {
        println!(
            "R1PROBE meshbase unit ball chordal={chordal} | {:?}",
            mesh::tessellate(&unit, chordal, tol).map(|m| mesh::validate::check_mesh(&m))
        );
    }
    let mut bad = 0;
    for d in [0.95, 1.1] {
        let small = ball(0.3, Vec3::new(d, 0.0, 0.0));
        bad += six(&format!("ordinary-x d={d}"), &unit, &small, ballv(1.0), ballv(0.3), lens(1.0, 0.3, d));
        let small = ball(0.3, Vec3::new(-d, 0.0, 0.0));
        bad += six(&format!("ordinary-negx d={d}"), &unit, &small, ballv(1.0), ballv(0.3), lens(1.0, 0.3, d));
    }
    let slab = finished("slab", brick((-2.0, 2.0), (-2.0, 2.0), (0.5, 3.0), tol), tol);
    let z = ball_turned(1.0, Vec3::new(0.0, 0.0, 0.0), Some((Vec3::new(1.0, 0.0, 0.0), PI / 2.0)));
    let out = topo::subtract(&z, &slab, tol);
    bad += usize::from(line("meshbase ball-minus-slab", "a∖b", out, ballv(1.0) - capv(1.0, 0.5), tol));
    done("mesh_baseline", bad);
}

/// Three balls again, each partner turned so its seam plane is normal
/// to its own centre line (its circle stays off its seam), so the
/// partner pairs reach the scan rather than the crossing layer.
#[test]
fn r1_three_balls_off_both_seams() {
    let tol = Tol::witness();
    let mut bad = 0;
    let unit = ball(1.0, Vec3::new(0.0, 0.0, 0.0));
    let tilt = Vec3::new(-0.6, 0.5, -0.5).normalize() * 0.95;
    let tilt2 = Vec3::new(-0.3, -0.6, 0.6).normalize() * 0.95;
    for (name, p, q) in [
        ("+z,-x", Vec3::new(0.0, 0.0, 0.95), Vec3::new(-0.95, 0.0, 0.0)),
        ("+z,-z", Vec3::new(0.0, 0.0, 0.95), Vec3::new(0.0, 0.0, -0.95)),
        ("+z,tilt", Vec3::new(0.0, 0.0, 0.95), tilt),
        ("tilt,tilt2", tilt, tilt2),
        ("-x,tilt", Vec3::new(-0.95, 0.0, 0.0), tilt),
    ] {
        let pair = match topo::union(&ball_seam_normal_to(0.3, p, p), &ball_seam_normal_to(0.3, q, q), tol) {
            Ok(r) => r.body().unwrap().body.clone(),
            Err(e) => {
                println!("R1PROBE three2 {name} | partner union | ERR {e:?}");
                continue;
            }
        };
        let pose = format!("three2 {name}");
        bad += six(&pose, &unit, &pair, ballv(1.0), 2.0 * ballv(0.3), 2.0 * lens(1.0, 0.3, 0.95));
        // tier 3′ detail where it fails
        let none = BooleanDeclarations::none();
        for (op, out) in [
            ("b∖a", topo::subtract_with(&pair, &unit, &none, tol)),
            ("a∩b", topo::intersect_with(&unit, &pair, &none, tol)),
        ] {
            if let Ok(BooleanResult::Body(bb)) = out {
                if let Err(e) = topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol) {
                    println!("R1PROBE three2 {name} | {op} t3p detail | {:.400}", format!("{e:?}"));
                }
            }
        }
    }
    done("three_balls_off_both_seams", bad);
}

/// Tier 3′ detail for the `+z,-z` pose of `r1_three_balls`.
#[test]
fn r1_three_balls_t3p_detail() {
    let tol = Tol::witness();
    let unit = ball(1.0, Vec3::new(0.0, 0.0, 0.0));
    let pair = topo::union(&ball(0.3, Vec3::new(0.0, 0.0, 0.95)), &ball(0.3, Vec3::new(0.0, 0.0, -0.95)), tol)
        .unwrap()
        .body()
        .unwrap()
        .body
        .clone();
    println!("R1PROBE t3pdetail partner | {}", shape(&pair));
    let none = BooleanDeclarations::none();
    for (op, out) in [
        ("b∖a", topo::subtract_with(&pair, &unit, &none, tol)),
        ("a∩b", topo::intersect_with(&unit, &pair, &none, tol)),
        ("b∩a", topo::intersect_with(&pair, &unit, &none, tol)),
    ] {
        if let Ok(BooleanResult::Body(bb)) = out {
            println!(
                "R1PROBE t3pdetail {op} | {} | {:.600}",
                shape(&bb.body),
                format!("{:?}", topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol))
            );
            // Each lump alone, against the closed form of one lens / one cap.
            println!("R1PROBE t3pdetail {op} contacts | {:?}", bb.contacts);
        }
    }
    // The same two lumps built one at a time and unioned (a known path).
    let one = topo::intersect(&unit, &ball(0.3, Vec3::new(0.0, 0.0, 0.95)), tol);
    let two = topo::intersect(&unit, &ball(0.3, Vec3::new(0.0, 0.0, -0.95)), tol);
    if let (Ok(one), Ok(two)) = (one, two) {
        let (one, two) = (one.body().unwrap().body.clone(), two.body().unwrap().body.clone());
        let u = topo::union(&one, &two, tol);
        match u {
            Ok(BooleanResult::Body(bb)) => println!(
                "R1PROBE t3pdetail lens∪lens | {} | {:.400}",
                shape(&bb.body),
                format!("{:?}", topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol))
            ),
            other => println!("R1PROBE t3pdetail lens∪lens | {:.300}", format!("{other:?}")),
        }
    }
}

/// `outcome`'s volume test is absolute (1e-7), vacuous for a 1e-3 ball:
/// the same poses held to 1e-9 relative.
#[test]
fn r1_tiny_ball_relative_volume() {
    let (tol, none) = (Tol::witness(), BooleanDeclarations::none());
    let unit = ball(1.0, Vec3::new(0.0, 0.0, 0.0));
    let mut bad = 0;
    for r in [1e-3, 1e-2, 0.05] {
        let (lo, hi) = (1.0 - r, 1.0 + r);
        for t in [0.1, 0.25, 0.75, 0.9] {
            let d = lo + t * (hi - lo);
            let small = ball(r, Vec3::new(0.0, 0.0, d));
            let shared = lens(1.0, r, d);
            for (op, want, out) in [
                ("b∖a", ballv(r) - shared, topo::subtract_with(&small, &unit, &none, tol)),
                ("a∩b", shared, topo::intersect_with(&unit, &small, &none, tol)),
            ] {
                let got = out
                    .ok()
                    .and_then(|o| o.body().map(|b| topo::mass_properties(&b.body, tol).map(|m| m.volume)));
                let rel = match got {
                    Some(Ok(v)) => (v - want).abs() / want,
                    _ => f64::NAN,
                };
                if !(rel <= 1e-9) && !rel.is_nan() {
                    bad += 1;
                }
                println!("R1PROBE tinyrel r={r} t={t} | {op} | want={want:e} rel={rel:e}");
            }
        }
    }
    done("tiny_ball_relative_volume", bad);
}

/// Claim 2: the plain pose built with BOTH balls turned z-poled, so each
/// seam crosses the circle and the ordinary crossing layer answers,
/// beside the cut-in build, for the face/edge census.
#[test]
fn r1_cut_in_against_both_seams_crossing() {
    let mut bad = 0;
    let x = Vec3::new(1.0, 0.0, 0.0);
    let c = Vec3::new(0.0, 0.0, 0.95);
    let unit_z = ball_turned(1.0, Vec3::new(0.0, 0.0, 0.0), Some((x, PI / 2.0)));
    let small_z = ball_turned(0.3, c, Some((x, PI / 2.0)));
    let unit = ball(1.0, Vec3::new(0.0, 0.0, 0.0));
    let small = ball(0.3, c);
    let v = (ballv(1.0), ballv(0.3), lens(1.0, 0.3, 0.95));
    bad += six("census cut-in", &unit, &small, v.0, v.1, v.2);
    bad += six("census both-seams", &unit_z, &small_z, v.0, v.1, v.2);
    // Off-centre in x so the unit ball's latitude circle is crossed by
    // its seam but not centred on its pole.
    let c2 = Vec3::new(0.2, 0.0, 0.93);
    let small_z2 = ball_turned(0.3, c2, Some((x, PI / 2.0)));
    let small2 = ball(0.3, c2);
    let v2 = (ballv(1.0), ballv(0.3), lens(1.0, 0.3, c2.norm()));
    bad += six("census cut-in off", &unit, &small2, v2.0, v2.1, v2.2);
    bad += six("census both-seams off", &unit_z, &small_z2, v2.0, v2.1, v2.2);
    done("census", bad);
}
