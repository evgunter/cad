//! Reviewer rows (CLEAVE dr51, PR 3892): grazes of curved walls by a
//! split plane, beyond the PR's fixtures — cylinder bosses and round
//! holes at off-axis azimuths, cone bosses and conical sockets at
//! off-axis azimuths, near-graze offsets, an obround's flat face, a
//! full cone grazed through its apex, and the Interval lane. Every row
//! holds the one claim that matters: an `Ok` answers the closed-form
//! volumes; a refusal passes (and is printed).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol, Vec3};
use profile::{Profile, RawLoop, SketchPlane, test_support::bulge_loop};
use sweep::{Extrusion, extrude};
use topo::splitting::{SplitPart, SplitPlane, split};
use topo::{Body, mass_properties, validate_closed};

type Loop = Vec<((f64, f64), f64)>;

fn extruded(loops: Vec<Loop>) -> Body<f64> {
    let lps = loops
        .into_iter()
        .map(|l| {
            bulge_loop(
                l.into_iter()
                    .map(|((x, y), b)| (Point2::new(x, y), b))
                    .collect(),
            )
        })
        .collect();
    let vp = Profile::new(SketchPlane::xy(), lps)
        .validate(Tol::witness())
        .unwrap();
    extrude(&vp, Extrusion::Distance(1.0), Tol::witness())
        .unwrap()
        .body
}

fn revolved(pts: &[(f64, f64)]) -> Body<f64> {
    use crate::revolve_common::{axis_y, validated};
    let lp = profile::ProfileLoop::polygon(pts.iter().map(|&(x, y)| Point2::new(x, y)));
    sweep::revolve(
        &validated(vec![lp]),
        axis_y(),
        sweep::Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

fn plane3(o: Point3<f64>, n: Vec3<f64>) -> SplitPlane<f64> {
    topo::test_support::split_plane(o, n / n.norm(), Tol::witness())
}

fn volume(label: &str, part: &SplitPart<f64>) -> Option<f64> {
    part.body().map(|b| {
        assert_eq!(validate_closed(b), Ok(()), "{label}");
        mass_properties(b, Tol::witness()).unwrap().volume
    })
}

fn close(got: Option<f64>, want: Option<f64>, rel: f64) -> bool {
    match (got, want) {
        (None, None) => true,
        (Some(g), Some(w)) => (g - w).abs() <= rel * w.max(1.0),
        _ => false,
    }
}

/// Split; an Ok must match `want` (above, below); a refusal is
/// returned as its Debug text. `Ok(())` means answered right.
fn check(
    label: &str,
    body: &Body<f64>,
    p: &SplitPlane<f64>,
    want: (Option<f64>, Option<f64>),
    rel: f64,
) -> Result<(), String> {
    match split(body, p, Tol::witness()) {
        Ok(r) => {
            let got = (volume(label, &r.above), volume(label, &r.below));
            assert!(
                close(got.0, want.0, rel) && close(got.1, want.1, rel),
                "WRONG ANSWER {label}: {got:?}, want {want:?}"
            );
            // A split has no declaration channel: an answered graze
            // must not carry an undeclared tangent (knife) edge. Only
            // read where the operand declares none of its own.
            let declared = topo::contact_marks(body, Tol::witness())
                .expect("operand tier-3 valid")
                .iter()
                .any(|(_, m)| *m == topo::ContactMark::Tangent);
            for part in [&r.above, &r.below].into_iter().filter(|_| !declared) {
                if let Some(b) = part.body() {
                    let knives = topo::contact_marks(b, Tol::witness())
                        .expect("tier-3 valid")
                        .iter()
                        .filter(|(_, m)| **m == topo::ContactMark::Tangent)
                        .count();
                    assert_eq!(knives, 0, "UNDECLARED KNIFE EDGE {label}");
                }
            }
            Ok(())
        }
        Err(e) => Err(format!("{e:?}")),
    }
}

fn short(e: &str) -> String {
    let head: String = e.chars().take(90).collect();
    match e.find("predicate: ") {
        Some(i) => format!("{head} … {}", &e[i..e.len().min(i + 50)]),
        None => head,
    }
}

/// Azimuths away from the seam and the quarter points.
const THETAS: [f64; 6] = [0.3, 1.1, 2.0, 2.9, 4.0, 5.5];

/// Cylinder boss r = 0.5, grazed from outside at off-axis azimuths,
/// both normals: the whole π/4 on the material side.
#[test]
fn dr51_cylinder_boss_off_axis_azimuths() {
    let disc = extruded(vec![vec![((-0.5, 0.0), 1.0), ((0.5, 0.0), 1.0)]]);
    let v = std::f64::consts::PI / 4.0;
    let mut refused = vec![];
    for t in THETAS {
        let (c, s) = (t.cos(), t.sin());
        for sg in [1.0, -1.0] {
            let label = format!("cyl boss θ={t} s={sg}");
            let p = plane3(
                Point3::new(0.5 * c, 0.5 * s, 0.0),
                Vec3::new(sg * c, sg * s, 0.0),
            );
            let want = if sg > 0.0 {
                (None, Some(v))
            } else {
                (Some(v), None)
            };
            if let Err(e) = check(&label, &disc, &p, want, 1e-9) {
                refused.push(format!("{label}: {}", short(&e)));
            }
        }
    }
    assert!(
        refused.is_empty(),
        "convex cylinder grazes refused: {refused:#?}"
    );
}

/// Area of the square [-2,2]² on the side n·x > d.
fn square_beyond(n: (f64, f64), d: f64) -> f64 {
    rect_beyond((-2.0, -2.0, 2.0, 2.0), n, d)
}

/// Area of the rectangle (x0, y0, x1, y1) on the side n·x > d.
fn rect_beyond(rc: (f64, f64, f64, f64), n: (f64, f64), d: f64) -> f64 {
    let sq = [(rc.0, rc.1), (rc.2, rc.1), (rc.2, rc.3), (rc.0, rc.3)];
    let f = |p: (f64, f64)| n.0 * p.0 + n.1 * p.1 - d;
    let mut poly = vec![];
    for i in 0..4 {
        let (a, b) = (sq[i], sq[(i + 1) % 4]);
        let (fa, fb) = (f(a), f(b));
        if fa > 0.0 {
            poly.push(a);
        }
        if (fa > 0.0) != (fb > 0.0) {
            let t = fa / (fa - fb);
            poly.push((a.0 + t * (b.0 - a.0), a.1 + t * (b.1 - a.1)));
        }
    }
    let mut area = 0.0;
    for i in 0..poly.len() {
        let (a, b) = (poly[i], poly[(i + 1) % poly.len()]);
        area += a.0 * b.1 - b.0 * a.1;
    }
    area.abs() / 2.0
}

/// A round hole r = 0.5 in a 4 × 4 plate, grazed from inside at
/// off-axis azimuths: never the hole on the wrong side.
#[test]
fn dr51_round_hole_off_axis_azimuths() {
    let outer = vec![
        ((-2.0, -2.0), 0.0),
        ((2.0, -2.0), 0.0),
        ((2.0, 2.0), 0.0),
        ((-2.0, 2.0), 0.0),
    ];
    let hole = vec![((0.5, 0.0), 1.0), ((-0.5, 0.0), 1.0)];
    let body = extruded(vec![outer, hole]);
    let total = 16.0 - std::f64::consts::PI / 4.0;
    let mut answered = vec![];
    for t in THETAS
        .iter()
        .copied()
        .chain([0.0, std::f64::consts::FRAC_PI_2])
    {
        let (c, s) = (t.cos(), t.sin());
        let far = square_beyond((c, s), 0.5);
        for sg in [1.0, -1.0] {
            let label = format!("hole θ={t} s={sg}");
            let p = plane3(
                Point3::new(0.5 * c, 0.5 * s, 0.0),
                Vec3::new(sg * c, sg * s, 0.0),
            );
            let want = if sg > 0.0 {
                (Some(far), Some(total - far))
            } else {
                (Some(total - far), Some(far))
            };
            match check(&label, &body, &p, want, 1e-9) {
                Ok(()) => answered.push(label),
                Err(e) => println!("{label}: refused {}", short(&e)),
            }
        }
    }
    println!("hole answered: {answered:?}");
}

/// Cone frusta (both nappes) grazed from outside at off-axis
/// azimuths: 7π/12 whole on the material side.
#[test]
fn dr51_cone_boss_off_axis_azimuths() {
    let v = 7.0 * std::f64::consts::PI / 12.0;
    let mut refused = vec![];
    for (name, body, r0, slope) in [
        (
            "narrowing",
            revolved(&[(0.0, 0.0), (1.0, 0.0), (0.5, 1.0), (0.0, 1.0)]),
            1.0,
            -0.5,
        ),
        (
            "widening",
            revolved(&[(0.0, 0.0), (0.5, 0.0), (1.0, 1.0), (0.0, 1.0)]),
            0.5,
            0.5,
        ),
    ] {
        for t in THETAS {
            let u = (t.cos(), t.sin());
            for sg in [1.0, -1.0] {
                let label = format!("{name} θ={t} s={sg}");
                let p = plane3(
                    Point3::new(r0 * u.0, 0.0, r0 * u.1),
                    Vec3::new(sg * u.0, -sg * slope, sg * u.1),
                );
                let want = if sg > 0.0 {
                    (None, Some(v))
                } else {
                    (Some(v), None)
                };
                if let Err(e) = check(&label, &body, &p, want, 1e-9) {
                    refused.push(format!("{label}: {}", short(&e)));
                }
            }
        }
    }
    println!("cone boss refusals: {refused:#?}");
}

/// A conical socket (the narrowing cone as a through hole of an r = 3
/// cylinder) grazed from inside at off-axis azimuths.
#[test]
fn dr51_conical_socket_off_axis_azimuths() {
    use std::f64::consts::PI;
    let body = revolved(&[(1.0, 0.0), (3.0, 0.0), (3.0, 1.0), (0.5, 1.0)]);
    let segment = |y: f64| {
        let c = 1.0 - y / 2.0;
        9.0 * (c / 3.0).acos() - c * (9.0 - c * c).sqrt()
    };
    let m = 2000;
    let h = 1.0 / f64::from(m);
    let beyond = (0..=m)
        .map(|i| {
            let w = if i == 0 || i == m {
                1.0
            } else if i % 2 == 1 {
                4.0
            } else {
                2.0
            };
            w * segment(f64::from(i) * h)
        })
        .sum::<f64>()
        * h
        / 3.0;
    let rest = 9.0 * PI - 7.0 * PI / 12.0 - beyond;
    for t in THETAS {
        let u = (t.cos(), t.sin());
        for sg in [1.0, -1.0] {
            let label = format!("socket θ={t} s={sg}");
            let p = plane3(
                Point3::new(u.0, 0.0, u.1),
                Vec3::new(sg * u.0, sg * 0.5, sg * u.1),
            );
            let want = if sg > 0.0 {
                (Some(beyond), Some(rest))
            } else {
                (Some(rest), Some(beyond))
            };
            match check(&label, &body, &p, want, 1e-7) {
                Ok(()) => println!("{label}: answered"),
                Err(e) => println!("{label}: refused {}", short(&e)),
            }
        }
    }
}

/// Near grazes: the plane δ inside / outside the cylinder's top ruling,
/// both normals. Inside, the truth is the circular segment; outside,
/// the whole on one side. Tolerance-scale offsets may land whole.
#[test]
fn dr51_cylinder_near_graze_offsets() {
    let disc = extruded(vec![vec![((-0.5, 0.0), 1.0), ((0.5, 0.0), 1.0)]]);
    let r: f64 = 0.5;
    let v = std::f64::consts::PI / 4.0;
    for d in [1e-13, 1e-11, 1e-10, 1e-9, 1e-8, 1e-7, 1e-6, 1e-5, 1e-4] {
        for (inside, off) in [(true, r - d), (false, r + d)] {
            for sg in [1.0, -1.0] {
                let label = format!("δ={d:e} inside={inside} s={sg}");
                let p = plane3(Point3::new(0.0, off, 0.0), Vec3::new(0.0, sg, 0.0));
                let seg = if inside {
                    r * r * (off / r).acos() - off * (r * r - off * off).sqrt()
                } else {
                    0.0
                };
                // above (+y side) holds `seg`.
                let (up, down) = (seg, v - seg);
                let opt = |x: f64| if x > 0.0 { Some(x) } else { None };
                let want = if sg > 0.0 {
                    (opt(up), opt(down))
                } else {
                    (opt(down), opt(up))
                };
                match split(&disc, &p, Tol::witness()) {
                    Ok(res) => {
                        let got = (volume(&label, &res.above), volume(&label, &res.below));
                        let exact = close(got.0, want.0, 1e-9) && close(got.1, want.1, 1e-9);
                        // landing whole is acceptable only where the
                        // missing segment is below tolerance scale.
                        let whole_ok = seg < 1e-12
                            && (got.0.is_none() || got.1.is_none())
                            && (got.0.unwrap_or(0.0) + got.1.unwrap_or(0.0) - v).abs() < 1e-9;
                        println!("{label}: {got:?} want {want:?} exact={exact} whole={whole_ok}");
                        assert!(
                            exact || whole_ok,
                            "WRONG ANSWER {label}: {got:?} want {want:?}"
                        );
                    }
                    Err(e) => println!("{label}: refused {}", short(&format!("{e:?}"))),
                }
            }
        }
    }
}

/// A full cone (apex on the body) grazed along a ruling: the tangent
/// plane passes through the apex vertex.
#[test]
fn dr51_full_cone_graze_through_apex() {
    let cone = revolved(&[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)]);
    let v = std::f64::consts::PI / 3.0;
    for t in [0.0f64, 0.7, 2.0] {
        let u: (f64, f64) = (t.cos(), t.sin());
        for sg in [1.0, -1.0] {
            let label = format!("full cone θ={t} s={sg}");
            // ρ = 1 − y: outward gradient ∝ (u, 1).
            let p = plane3(
                Point3::new(u.0, 0.0, u.1),
                Vec3::new(sg * u.0, sg, sg * u.1),
            );
            let want = if sg > 0.0 {
                (None, Some(v))
            } else {
                (Some(v), None)
            };
            match check(&label, &cone, &p, want, 1e-9) {
                Ok(()) => println!("{label}: answered whole"),
                Err(e) => println!("{label}: refused {}", short(&e)),
            }
        }
    }
}

/// The convex ruling graze at `T = Interval`.
#[test]
fn dr51_cylinder_graze_at_interval() {
    use crate::common::interval::{iv, p2, p3, v3};
    use geom_core::{Bounds, Interval};
    let lp = bulge_loop(vec![(p2(-0.5, 0.0), iv(1.0)), (p2(0.5, 0.0), iv(1.0))]);
    let vp = Profile::new(SketchPlane::<Interval>::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let body = extrude(&vp, Extrusion::Distance(iv(1.0)), Tol::witness())
        .unwrap()
        .body;
    for (o, n) in [
        (p3(0.0, 0.5, 0.0), v3(0.0, 1.0, 0.0)),
        (p3(0.5, 0.0, 0.0), v3(1.0, 0.0, 0.0)),
        (p3(0.0, 0.5, 0.0), v3(0.0, -1.0, 0.0)),
    ] {
        let plane = topo::test_support::split_plane(o, n, Tol::witness());
        match split(&body, &plane, Tol::witness()) {
            Ok(r) => {
                let a = r
                    .above
                    .body()
                    .map(|b| mass_properties(b, Tol::witness()).unwrap().volume);
                let b = r
                    .below
                    .body()
                    .map(|b| mass_properties(b, Tol::witness()).unwrap().volume);
                println!(
                    "interval graze: above {:?} below {:?}",
                    a.map(|x| (x.lo(), x.hi())),
                    b.map(|x| (x.lo(), x.hi()))
                );
                let whole = a.or(b).unwrap();
                let pi4 = std::f64::consts::PI / 4.0;
                assert!(a.is_none() || b.is_none(), "two-sided");
                assert!(whole.lo() - 1e-9 <= pi4 && pi4 <= whole.hi() + 1e-9);
            }
            Err(e) => println!("interval graze refused {}", short(&format!("{e:?}"))),
        }
    }
}

/// Booleans on grazing poses: a cylinder boss and a box whose face is
/// tangent to the wall along a ruling (union, subtract, intersect).
#[test]
fn dr51_boolean_grazing_box() {
    let disc = extruded(vec![vec![((-0.5, 0.0), 1.0), ((0.5, 0.0), 1.0)]]);
    let bx = extruded(vec![vec![
        ((-1.0, 0.5), 0.0),
        ((1.0, 0.5), 0.0),
        ((1.0, 1.5), 0.0),
        ((-1.0, 1.5), 0.0),
    ]]);
    let pi4 = std::f64::consts::PI / 4.0;
    let vol = |b: &Body<f64>| mass_properties(b, Tol::witness()).unwrap().volume;
    match topo::subtract(&disc, &bx, Tol::witness()) {
        Ok(r) => {
            let v = r.body().map(|b| vol(&b.body));
            println!("cyl − tangent box: {v:?}");
            assert!(v.is_some_and(|v| (v - pi4).abs() < 1e-9));
        }
        Err(e) => println!("cyl − tangent box refused {}", short(&format!("{e:?}"))),
    }
    match topo::union(&disc, &bx, Tol::witness()) {
        Ok(r) => {
            let v = r.body().map(|b| vol(&b.body));
            println!("cyl ∪ tangent box: {v:?}");
            assert!(v.is_some_and(|v| (v - pi4 - 2.0).abs() < 1e-9));
        }
        Err(e) => println!("cyl ∪ tangent box refused {}", short(&format!("{e:?}"))),
    }
    match topo::intersect(&disc, &bx, Tol::witness()) {
        Ok(r) => {
            let v = r.body().map(|b| vol(&b.body));
            println!("cyl ∩ tangent box: {v:?}");
            assert!(v.is_none_or(|v| v.abs() < 1e-9));
        }
        Err(e) => println!("cyl ∩ tangent box refused {}", short(&format!("{e:?}"))),
    }
}

fn seg(r: f64, d: f64) -> f64 {
    r * r * (d / r).acos() - d * (r * r - d * d).sqrt()
}

/// Mixed vertices: the graze shares its rim vertex with a planar face
/// the plane crosses transversally. A boss (r = 0.5) on a disc
/// (r = 2), and a counterbored hole (r = 0.5 under r = 1), revolved
/// about y; the plane tangent to the r = 0.5 wall at several azimuths.
#[test]
fn dr51_graze_on_a_stepped_body() {
    use std::f64::consts::PI;
    let boss = revolved(&[
        (0.0, 0.0),
        (2.0, 0.0),
        (2.0, 1.0),
        (0.5, 1.0),
        (0.5, 2.0),
        (0.0, 2.0),
    ]);
    let bore = revolved(&[
        (0.5, 0.0),
        (2.0, 0.0),
        (2.0, 2.0),
        (1.0, 2.0),
        (1.0, 1.0),
        (0.5, 1.0),
    ]);
    let far = seg(2.0, 0.5);
    let boss_total = 4.0 * PI + PI / 4.0;
    let bore_beyond = far + (far - seg(1.0, 0.5));
    let bore_total = 8.0 * PI - PI / 4.0 - PI;
    let mut answered = vec![];
    let mut refused = vec![];
    for (name, body, beyond, total) in [
        ("boss", &boss, far, boss_total),
        ("bore", &bore, bore_beyond, bore_total),
    ] {
        for t in [0.0f64, 0.3, 1.1, 2.0, PI, 4.0] {
            let u = (t.cos(), t.sin());
            for sg in [1.0, -1.0] {
                let label = format!("{name} θ={t} s={sg}");
                let p = plane3(
                    Point3::new(0.5 * u.0, 0.5, 0.5 * u.1),
                    Vec3::new(sg * u.0, 0.0, sg * u.1),
                );
                let want = if sg > 0.0 {
                    (Some(beyond), Some(total - beyond))
                } else {
                    (Some(total - beyond), Some(beyond))
                };
                match check(&label, body, &p, want, 1e-7) {
                    Ok(()) => answered.push(label),
                    Err(e) => refused.push(format!("{label}: {}", short(&e))),
                }
            }
        }
    }
    println!("stepped answered: {answered:#?}\nstepped refused: {refused:#?}");
}

/// A 6 × 4 slab (height 1), its corners rounded r = 0.5 through the
/// PATHS fillet door (declared tangent joints): smooth edges between
/// the flats and the corner walls. The plane coplanar with a flat the
/// NE corner continues (φ = 0, π/2), and tangent to the corner wall at
/// interior angles: convex, so it lands whole on the material side.
#[test]
fn dr51_rounded_slab_corner_grazes() {
    use profile::{Open, Start};
    let (w, h, r, t) = (6.0, 4.0, 0.5, Tol::witness());
    let lp: profile::ProfileLoop<f64> = Open
        .at(Point2::new(w / 2.0, 0.0))
        .toward(1.0, 0.0, t)
        .unwrap()
        .fillet(r, t)
        .unwrap()
        .at(Point2::new(w, h / 2.0), t)
        .unwrap()
        .toward(0.0, 1.0, t)
        .unwrap()
        .fillet(r, t)
        .unwrap()
        .at(Point2::new(w / 2.0, h), t)
        .unwrap()
        .toward(-1.0, 0.0, t)
        .unwrap()
        .fillet(r, t)
        .unwrap()
        .at(Point2::new(0.0, h / 2.0), t)
        .unwrap()
        .toward(0.0, -1.0, t)
        .unwrap()
        .fillet(r, t)
        .unwrap()
        .to(Start, t)
        .unwrap()
        .into();
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(t)
        .unwrap();
    let body = extrude(&vp, Extrusion::Distance(1.0), t).unwrap().body;
    let v = w * h - (4.0 - std::f64::consts::PI) * r * r;
    let c = (w - r, h - r);
    for phi in [
        0.0f64,
        0.3,
        std::f64::consts::FRAC_PI_4,
        1.2,
        std::f64::consts::FRAC_PI_2,
    ] {
        let n = (phi.cos(), phi.sin());
        for sg in [1.0, -1.0] {
            let label = format!("rounded φ={phi} s={sg}");
            let p = plane3(
                Point3::new(c.0 + r * n.0, c.1 + r * n.1, 0.0),
                Vec3::new(sg * n.0, sg * n.1, 0.0),
            );
            let want = if sg > 0.0 {
                (None, Some(v))
            } else {
                (Some(v), None)
            };
            match check(&label, &body, &p, want, 1e-9) {
                Ok(()) => println!("{label}: answered whole"),
                Err(e) => println!("{label}: refused {}", short(&e)),
            }
        }
    }
}

/// The same rounded outline as a hole in a 10 × 8 plate: grazed from
/// inside along its NE corner wall (concave, smooth edges to the flats)
/// and coplanar with the flats the corner continues.
#[test]
fn dr51_rounded_hole_corner_grazes() {
    use profile::{Open, Start};
    let (w, h, r, t) = (6.0, 4.0, 0.5, Tol::witness());
    let hole: profile::ProfileLoop<f64> = Open
        .at(Point2::new(w / 2.0, 0.0))
        .toward(1.0, 0.0, t)
        .unwrap()
        .fillet(r, t)
        .unwrap()
        .at(Point2::new(w, h / 2.0), t)
        .unwrap()
        .toward(0.0, 1.0, t)
        .unwrap()
        .fillet(r, t)
        .unwrap()
        .at(Point2::new(w / 2.0, h), t)
        .unwrap()
        .toward(-1.0, 0.0, t)
        .unwrap()
        .fillet(r, t)
        .unwrap()
        .at(Point2::new(0.0, h / 2.0), t)
        .unwrap()
        .toward(0.0, -1.0, t)
        .unwrap()
        .fillet(r, t)
        .unwrap()
        .to(Start, t)
        .unwrap()
        .into();
    let rc = (-2.0, -2.0, 8.0, 6.0);
    let outer = profile::ProfileLoop::polygon([
        Point2::new(rc.0, rc.1),
        Point2::new(rc.2, rc.1),
        Point2::new(rc.2, rc.3),
        Point2::new(rc.0, rc.3),
    ]);
    let vp = match Profile::new(SketchPlane::xy(), vec![outer, hole]).validate(t) {
        Ok(vp) => vp,
        Err(e) => panic!("profile: {e:?}"),
    };
    let body = extrude(&vp, Extrusion::Distance(1.0), t).unwrap().body;
    let total = 80.0 - (w * h - (4.0 - std::f64::consts::PI) * r * r);
    let c = (w - r, h - r);
    for phi in [
        0.0f64,
        0.3,
        std::f64::consts::FRAC_PI_4,
        1.2,
        std::f64::consts::FRAC_PI_2,
    ] {
        let n = (phi.cos(), phi.sin());
        let d = n.0 * (c.0 + r * n.0) + n.1 * (c.1 + r * n.1);
        let far = rect_beyond(rc, n, d);
        for sg in [1.0, -1.0] {
            let label = format!("rounded hole φ={phi} s={sg}");
            let p = plane3(
                Point3::new(c.0 + r * n.0, c.1 + r * n.1, 0.0),
                Vec3::new(sg * n.0, sg * n.1, 0.0),
            );
            let want = if sg > 0.0 {
                (Some(far), Some(total - far))
            } else {
                (Some(total - far), Some(far))
            };
            match check(&label, &body, &p, want, 1e-9) {
                Ok(()) => println!("{label}: answered"),
                Err(e) => println!("{label}: refused {}", short(&e)),
            }
        }
    }
}
