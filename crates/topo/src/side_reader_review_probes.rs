//! Reviewer probes (second review of PR 6958): a boxed brick whose top
//! cap TILTS across the box (its stored normal is a function of the box
//! parameter), upright and inside out; every face's own aim and random
//! rays must read no side no member has.

#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use super::*;
use crate::construction::FaceSurface;
use crate::test_support_fixtures::{FaceGeometry, brick, prism_ops};
use geom::Surface;
use geom_core::{Interval, ParamSymbol, Real as _, Sym, SymBudget, Tol};

type Boxed = Sym<Interval>;
const SQUARE: [(f64, f64); 4] = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];

/// The brick over the unit square, its top edge at `x = 1` at height
/// `top` and at `x = 0` at height 1: the top cap tilts about `y`.
fn tilted(top: Boxed, inside_out: bool, tol: Tol) -> Body<Boxed> {
    let iv = Boxed::from_f64;
    let twin = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
    let mut body = Body::<Boxed>::new();
    prism_ops(
        &mut body,
        &SQUARE,
        (0.0, 1.0),
        |x, y, z| Point3::new(iv(x), iv(y), if z == 1.0 && x == 1.0 { top } else { iv(z) }),
        FaceGeometry::Declined,
        tol,
    );
    let sign = if inside_out { -1.0 } else { 1.0 };
    for (face, _) in twin.faces() {
        let Surface::Plane { normal, u_ref, .. } = *twin.proven_face(face).surface() else {
            panic!("planar");
        };
        let corner = body.proven_face(face).edges()[0].ends[0];
        let origin = *body
            .get_vertex(corner)
            .and_then(|v| body.get_point(v.point))
            .expect("shared keys");
        let (normal, u_ref) = if normal.z > 0.5 {
            // (1 - t, 0, 1) / |.|
            let a = iv(1.0) - top;
            let len = (a * a + iv(1.0)).sqrt();
            (
                Vec3::new(a / len, iv(0.0), iv(1.0) / len) * iv(sign),
                Vec3::new(iv(0.0), iv(1.0), iv(0.0)),
            )
        } else {
            (normal.map(|x| iv(sign * x)), u_ref.map(iv))
        };
        body.set_face_surface(
            face,
            FaceSurface::New {
                surface: Surface::Plane {
                    origin,
                    normal,
                    u_ref,
                },
                sense: true,
            },
        )
        .expect("the face takes its plane");
    }
    body
}

fn over<R>(top: (f64, f64), inside_out: bool, read: impl FnOnce(&Body<Boxed>, Band) -> R) -> R {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("band");
    let budget = SymBudget {
        max_terms: 64,
        max_degree: 8,
    };
    geom_core::sym::with_session(budget, || {
        let t = if top.0 == top.1 {
            Boxed::from_f64(top.0)
        } else {
            Sym::param_over(
                ParamSymbol::new(test_utils::symbol_id("review_tilted_top")),
                Interval::from_bounds(top.0, top.1),
                top.0,
                top.1,
            )
        };
        read(&tilted(t, inside_out, tol), band)
    })
    .0
}

fn kind(r: &str) -> &'static str {
    if r.contains("Side(In)") {
        "in"
    } else if r.contains("Side(Out)") {
        "out"
    } else if r.contains("SetAside") {
        "aside"
    } else {
        "other"
    }
}

#[test]
fn review_a_tilting_top_reads_no_wrong_side() {
    let mut rng = test_utils::fuzz::start("review_tilted_top");
    let eps = Tol::witness().eps();
    let iv = Boxed::from_f64;
    for inside_out in [false, true] {
        let wrong = if inside_out { "in_wrong_out" } else { "in" };
        let wrong_kind = if inside_out { "out" } else { "in" };
        let _ = wrong;
        let mut tally = std::collections::BTreeMap::<&str, usize>::new();
        for half in [1e-4, 1e-3, 0.01, 0.05, 0.2, 0.45] {
            let top = (1.0 - half, 1.0 + half);
            let own = over(top, inside_out, each_aim_report);
            for r in &own {
                *tally.entry(kind(r)).or_default() += 1;
                assert!(
                    kind(r) != wrong_kind,
                    "own aim io={inside_out} top {top:?} ε {eps:e}: {r}"
                );
            }
            let draws: Vec<([f64; 3], [f64; 3])> = (0..60)
                .map(|j| {
                    let aim = if j % 2 == 0 {
                        [
                            rng.range(-0.3, 1.3),
                            rng.range(-0.3, 1.3),
                            rng.range(-0.5, 1.6),
                        ]
                    } else {
                        // on the moving top edge / top cap / right wall at a member
                        let h = rng.range(top.0, top.1);
                        let u = rng.unit();
                        match rng.below(4) {
                            0 => [1.0, u, h],
                            1 => [u, u * 0.0 + rng.unit(), 1.0 + u * (h - 1.0)],
                            2 => [1.0, u, u * h],
                            _ => [1.0, if rng.below(2) == 0 { 0.0 } else { 1.0 }, h],
                        }
                    };
                    let normal = match rng.below(6) {
                        0 => [1.0, 0.0, 0.0],
                        1 => [0.0, 0.0, 1.0],
                        _ => {
                            let v = [
                                rng.range(-1.0, 1.0),
                                rng.range(-1.0, 1.0),
                                rng.range(-1.0, 1.0),
                            ];
                            let n = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt().max(1e-3);
                            [v[0] / n, v[1] / n, v[2] / n]
                        }
                    };
                    (aim, normal)
                })
                .collect();
            let reads = over(top, inside_out, |body, band| {
                let face = body.faces().next().unwrap().0;
                draws
                    .iter()
                    .map(|&(a, n)| {
                        ray_at_report(
                            body,
                            face,
                            Point3::new(iv(a[0]), iv(a[1]), iv(a[2])),
                            Vec3::new(iv(n[0]), iv(n[1]), iv(n[2])),
                            band,
                        )
                    })
                    .collect::<Vec<_>>()
            });
            for (r, d) in reads.iter().zip(&draws) {
                *tally.entry(kind(r)).or_default() += 1;
                assert!(
                    kind(r) != wrong_kind,
                    "ray io={inside_out} top {top:?} ε {eps:e} {d:?}: {r}"
                );
            }
            println!(
                "ε {eps:e} io={inside_out} ±{half}: own {:?}",
                own.iter().map(|r| kind(r)).collect::<Vec<_>>()
            );
        }
        println!("ε {eps:e} io={inside_out} tally {tally:?}");
    }
}

/// A grid of aims on the top cap's highest member, near every top edge,
/// with near-vertical normals, over the plain brick (top height boxed)
/// and the tilted one: no `In`. Built to have power over the first
/// review's MAJOR class (gap undecided read as met in band).
#[test]
fn review_a_grid_beside_every_moving_top_edge_reads_no_in() {
    let eps = Tol::witness().eps();
    let iv = Boxed::from_f64;
    let mut tally = std::collections::BTreeMap::<String, usize>::new();
    for tilt in [false, true] {
        for (lo, hi) in [(0.2, 0.3), (0.05, 0.3), (0.7, 1.3), (0.9, 1.1)] {
            let mut aims = Vec::new();
            for &m in &[0.002, 0.005, 0.01, 0.02, 0.04] {
                for &s in &[0.1, 0.5, 0.9] {
                    aims.push([m, s]);
                    aims.push([1.0 - m, s]);
                    aims.push([s, m]);
                    aims.push([s, 1.0 - m]);
                }
            }
            let normals = [
                [0.0, 0.0, 1.0],
                [0.05, 0.0, 0.998_749],
                [-0.05, 0.0, 0.998_749],
                [0.0, 0.05, 0.998_749],
                [0.0, -0.05, 0.998_749],
            ];
            let read = |body: &Body<Boxed>, band: Band| {
                let face = body.faces().next().unwrap().0;
                let mut out = Vec::new();
                for a in &aims {
                    let z = if tilt { 1.0 + a[0] * (hi - 1.0) } else { hi };
                    for n in &normals {
                        out.push((
                            *a,
                            *n,
                            ray_at_report(
                                body,
                                face,
                                Point3::new(iv(a[0]), iv(a[1]), iv(z)),
                                Vec3::new(iv(n[0]), iv(n[1]), iv(n[2])),
                                band,
                            ),
                        ));
                    }
                }
                out
            };
            let reads = if tilt {
                over((lo, hi), false, read)
            } else {
                super::boxed_brick_rows::over_pub((lo, hi), read)
            };
            for (a, n, r) in &reads {
                *tally.entry(format!("tilt={tilt} {}", kind(r))).or_default() += 1;
                assert!(
                    kind(r) != "in",
                    "tilt {tilt} top [{lo},{hi}] ε {eps:e} aim {a:?} n {n:?}: {r}"
                );
            }
        }
    }
    println!("ε {eps:e} grid tally {tally:?}");
}

/// `f64` differential (claim 16): point containment over fixture bodies
/// at random points and at points in band of their edges and vertices,
/// each result as text, written to `$REVIEW_DIFF_OUT`. Run once at the
/// head and once with the merge base's `solid_contain`/`census`.
#[test]
fn review_f64_point_containment_differential() {
    use crate::boolean::point_in_solid;
    use crate::test_support_fixtures::{holed_block, mapped_cube, prism};
    let Ok(path) = std::env::var("REVIEW_DIFF_OUT") else {
        return;
    };
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let k = tol.k() * tol.eps();
    let mut bodies: Vec<(&str, Body<f64>)> = vec![
        (
            "brick",
            brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol),
        ),
        ("holed", holed_block::<f64>(3.0, &[1.0, 2.0], tol)),
        (
            "skew cube",
            mapped_cube::<f64>(
                |x, y, z| Point3::new(x + 0.3 * y + 0.1 * z, y + 0.2 * z, z + 0.15 * x),
                tol,
            ),
        ),
    ];
    for (name, profile) in [
        (
            "L",
            vec![
                (0.0, 0.0),
                (1.0, 0.0),
                (1.0, 0.4),
                (0.4, 0.4),
                (0.4, 1.0),
                (0.0, 1.0),
            ],
        ),
        ("tri", vec![(0.0, 0.0), (1.0, 0.0), (0.3, 0.9)]),
        (
            "needle",
            vec![(0.0, 0.0), (1.0, 0.0), (1.0, 0.001), (0.0, 0.001)],
        ),
    ] {
        bodies.push((name, prism::<f64>(&profile, 0.7, tol).body));
    }
    let mut rng = test_utils::fuzz::start("review_f64_diff");
    let mut out = String::new();
    for (name, body) in &bodies {
        let mut pts = Vec::new();
        let verts: Vec<Point3<f64>> = body
            .vertices()
            .filter_map(|(_, v)| body.get_point(v.point).copied())
            .collect();
        for _ in 0..300 {
            pts.push(Point3::new(
                rng.range(-0.5, 3.5),
                rng.range(-0.5, 1.5),
                rng.range(-0.5, 1.5),
            ));
        }
        for a in &verts {
            for b in &verts {
                for s in [0.0, 0.5, 0.25] {
                    let p = *a + (*b - *a) * s;
                    for o in [0.0, 0.3 * k, k, 3.0 * k, 1e-3] {
                        let d = Vec3::new(
                            rng.range(-1.0, 1.0),
                            rng.range(-1.0, 1.0),
                            rng.range(-1.0, 1.0),
                        );
                        pts.push(p + d * o);
                    }
                }
            }
        }
        for p in pts {
            let r = point_in_solid(body, p, band, tol);
            out.push_str(&format!("{name} {:?} {:?}\n", (p.x, p.y, p.z), r));
        }
    }
    std::fs::write(path, out).unwrap();
}
