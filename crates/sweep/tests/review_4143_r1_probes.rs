//! Review probes for PR 4143 (predicate 2's reach), lane
//! `band-dual-4143-r1`. Each row prints its verdict; the asserts pin
//! only what the reviewer measured.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::time::Instant;

use geom_core::{Band, Point2, Point3, Tol};
use sweep::Revolution;
use sweep::blend::{BlendError, BlendRequest, fillet_edges};
use sweep::test_support::{band_reach, corners, revolved_about_y, rim_arcs_at};
use topo::{Body, ContactRecords, mass_properties, validate_geometric, validate_pseudomanifold};

use crate::common::cavity::{brick, cut, edges_with_corners, rod};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).expect("the witness band")
}

fn fuse(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    let t = tol();
    let a = sweep::test_support::finished("a", a.clone(), t);
    let b = sweep::test_support::finished("b", b.clone(), t);
    topo::union(&a, &b, t)
        .expect("union")
        .body()
        .expect("material")
        .body
        .clone()
        .into_body()
}

/// Run the fillet; on a build, report tier 3, tier 3' and the volume.
fn run(what: &str, body: &Body<f64>, edges: &[topo::EdgeKey], r: f64) -> Result<f64, BlendError> {
    match fillet_edges(body, edges, r, tol()) {
        Ok(f) => {
            let t3 = validate_geometric(&f.body, tol());
            let t3p = validate_pseudomanifold(&f.body, &ContactRecords::default(), tol());
            let v = mass_properties(&f.body, tol()).map(|m| m.volume).unwrap();
            eprintln!(
                "{what} r={r}: BUILT tier3 {t3:?} tier3' {:?} V {v}",
                t3p.is_ok()
            );
            assert_eq!(t3, Ok(()), "{what}: tier 3");
            if let Err(es) = &t3p {
                assert!(
                    es.iter()
                        .all(|e| format!("{e:?}").starts_with("CensusUndecidable")),
                    "{what}: tier 3' {t3p:?}"
                );
            }
            Ok(v)
        }
        Err(e) => {
            eprintln!("{what} r={r}: REFUSED {:?}", e.error);
            Err(e.error)
        }
    }
}

/// Solid cylinder `ρ ≤ 1`, `y ∈ [0, 2]`, with a sealed box void at
/// `x ∈ [x0, x0 + 0.1]`, `y ∈ [y0, y0 + 0.1]`, `z ∈ [-0.05, 0.05]`.
fn drum_with_void(x0: f64, y0: f64) -> Body<f64> {
    let drum = revolved_about_y(
        corners(&[(0.0, 0.0), (1.0, 0.0), (1.0, 2.0), (0.0, 2.0)]),
        Revolution::Full,
        tol(),
    );
    let void = sweep::test_support::brick((x0, x0 + 0.1), (y0, y0 + 0.1), (-0.05, 0.05), tol());
    cut("void", &drum, &void)
}

/// Spandrel removed by a convex rim fillet of radius `r` at outer
/// radius `big`: Pappus.
fn rim_removed(big: f64, r: f64) -> f64 {
    let pi = std::f64::consts::PI;
    let area = r * r * (1.0 - pi / 4.0);
    let off = r * (10.0 - 3.0 * pi) / (3.0 * (4.0 - pi));
    2.0 * pi * (big - off) * area
}

/// A: the convex top rim (circular spine, removes material) against a
/// sealed void near the rim (non-coaxial planes, and planes ⊥ the axis).
#[test]
fn a_convex_rim_against_a_void() {
    let r = 0.3;
    let pi = std::f64::consts::PI;
    // The void's far corner (0.95, 1.95) is 0.354 from the ball centre
    // (0.7, 1.7): inside the material the band removes.
    let body = drum_with_void(0.85, 1.85);
    let edges = rim_arcs_at(&body, 1.0, 2.0);
    assert!(!edges.is_empty());
    let e = run("A interfering", &body, &edges, r).expect_err("the band cuts the void");
    assert!(matches!(e, BlendError::FaceClearance { .. }), "{e:?}");
    // Control: the void deep inside.
    let body = drum_with_void(0.4, 1.0);
    let edges = rim_arcs_at(&body, 1.0, 2.0);
    let v = run("A control", &body, &edges, r).expect("clear");
    let want = 2.0 * pi - 0.1 * 0.1 * 0.1 - rim_removed(1.0, r);
    eprintln!("A control V {v} want {want} diff {}", v - want);
    assert!((v - want).abs() < 1e-6);
}

/// B: the void's nearest corner just outside the band's material: the
/// material is `{ρ ∈ [0.7, 1], y ∈ [1.7, 2]}` outside the ball of radius
/// `0.3` about `(0.7, 1.7)`; the corner `(x0 + 0.1, y0 + 0.1, ±0.05)`
/// with `ρ = hypot(x0 + 0.1, 0.05)`.
#[test]
fn b_a_void_just_clear_of_the_convex_rim() {
    let r = 0.3;
    for gap in [1e-2, 1e-3, 1e-4] {
        // Put the corner on the 45° ray from the ball centre at distance
        // r − gap (inside the ball, so outside the material).
        let d = (r - gap) / 2f64.sqrt();
        let (rho, y) = (0.7 + d, 1.7 + d);
        let x = (rho * rho - 0.05 * 0.05).sqrt();
        let body = drum_with_void(x - 0.1, y - 0.1);
        let edges = rim_arcs_at(&body, 1.0, 2.0);
        let _ = run(&format!("B gap {gap}"), &body, &edges, r);
    }
}

/// C: the concave foot of a round boss on a plate (circular spine, adds
/// material out to ρ = 0.5 + r on the plate), with a block wall standing
/// on the plate `gap` off the boss: its faces are planes parallel to the
/// band's axis (the sheet path's `n·a = 0`).
fn boss_and_wall(gap: f64) -> Body<f64> {
    let plate = brick(Point3::new(-3.0, -3.0, 0.0), Point3::new(3.0, 3.0, 1.0));
    let boss = rod(Point2::new(0.0, 0.0), 0.5, 0.5, 2.0);
    let wall = brick(
        Point3::new(0.5 + gap, -1.0, 0.5),
        Point3::new(0.9 + gap, 1.0, 2.0),
    );
    fuse(&fuse(&plate, &boss), &wall)
}

fn boss_foot(body: &Body<f64>) -> Vec<topo::EdgeKey> {
    edges_with_corners(body, |p| {
        (p.z - 1.0).abs() < 1e-9 && ((p.x * p.x + p.y * p.y).sqrt() - 0.5).abs() < 1e-9
    })
}

#[test]
fn c_a_concave_boss_foot_beside_a_wall() {
    let r = 0.2;
    for gap in [0.1, 0.19, 0.21, 0.3] {
        let body = boss_and_wall(gap);
        let edges = boss_foot(&body);
        eprintln!("C gap {gap}: {} foot edges", edges.len());
        let res = run(&format!("C gap {gap}"), &body, &edges, r);
        if gap < r {
            assert!(res.is_err(), "C gap {gap}: the band reaches the wall");
        }
    }
}

/// D: a ledge of width `w` between a convex drop (y = 0) and a concave
/// wall (y = w), both edges co-requested: two chains, apart, sharing the
/// ledge as a support.
fn ledge(w: f64) -> Body<f64> {
    let p = |x: f64, y: f64| Point2::new(x, y);
    crate::common::cavity::prism(
        &[
            p(0.0, 0.0),
            p(3.0, 0.0),
            p(3.0, 2.0),
            p(w, 2.0),
            p(w, 1.0),
            p(0.0, 1.0),
        ],
        0.0,
        4.0,
    )
}

#[test]
fn d_a_ledge_with_a_convex_and_a_concave_edge() {
    let w = 1.0;
    let body = ledge(w);
    let mut both = edges_with_corners(&body, |p| (p.y - 1.0).abs() < 1e-9 && p.x.abs() < 1e-9);
    both.extend(edges_with_corners(&body, |p| {
        (p.y - 1.0).abs() < 1e-9 && (p.x - w).abs() < 1e-9
    }));
    assert_eq!(both.len(), 2, "two ledge edges");
    for r in [0.3, 0.45, 0.49, 0.5, 0.51, 0.6] {
        let req = BlendRequest {
            body: &body,
            edges: both.clone(),
            size: r,
        };
        eprintln!("D meter r {r}: {:?}", band_reach(&req, band()).err());
        let res = run(&format!("D r {r}"), &body, &both, r);
        if let Ok(v) = res {
            // Box volume less the convex spandrel plus the concave one.
            let pi = std::f64::consts::PI;
            let want = 4.0 * (3.0 * 1.0 + (3.0 - w) * 1.0);
            let sp = r * r * (1.0 - pi / 4.0) * 4.0;
            eprintln!("D r {r}: V {v} want {}", want);
            assert!((v - want).abs() < 1e-6 + 0.0 * sp, "D r {r}: V {v}");
        }
    }
}

/// E: cost. A staircase revolve with `n` steps: `2n` rims, every
/// rim filleted in one request, at f64 and at `Interval`.
fn stairs(n: usize) -> Vec<(f64, f64)> {
    let mut pts = vec![(0.0, 0.0)];
    for i in 0..n {
        let x = 4.0 - 3.0 * i as f64 / n as f64;
        let y = i as f64;
        pts.push((x, y));
        pts.push((x, y + 1.0));
    }
    pts.push((0.0, n as f64));
    pts
}

#[test]
#[ignore = "cost probe: run with --ignored --nocapture"]
fn e_cost_scaling() {
    use crate::common::interval::iv;
    use geom_core::Interval;
    for n in [2usize, 4, 8, 16] {
        let pts = stairs(n);
        let body = revolved_about_y(corners(&pts), Revolution::Full, tol());
        let mut edges = Vec::new();
        for (i, &(x, y)) in pts.iter().enumerate().skip(1) {
            if i == pts.len() - 1 || y == 0.0 {
                continue;
            }
            edges.extend(rim_arcs_at(&body, x, y));
        }
        let req = BlendRequest {
            body: &body,
            edges: edges.clone(),
            size: 0.05,
        };
        let t = Instant::now();
        let r = band_reach(&req, band());
        let f = t.elapsed();
        let ipts: Vec<(Point2<Interval>, Interval)> = pts
            .iter()
            .map(|&(x, y)| (Point2::new(iv(x), iv(y)), iv(0.0)))
            .collect();
        let ib =
            sweep::test_support::revolved_about_y_at::<Interval>(ipts, Revolution::Full, tol());
        let mut iedges = Vec::new();
        for (i, &(x, y)) in pts.iter().enumerate().skip(1) {
            if i == pts.len() - 1 || y == 0.0 {
                continue;
            }
            iedges.extend(rim_arcs_at(&ib, x, y));
        }
        let ireq = BlendRequest {
            body: &ib,
            edges: iedges,
            size: iv(0.05),
        };
        let t = Instant::now();
        let ir = band_reach(&ireq, band());
        let fi = t.elapsed();
        eprintln!(
            "E n {n}: links {} faces {}: f64 {f:?} {:?} | interval {fi:?} {:?}",
            edges.len(),
            body.faces().count(),
            r.is_ok(),
            ir.is_ok()
        );
    }
}

/// E2: cost across non-coaxial faces: a plate with a k×k grid of
/// square posts, every post's four top edges filleted (f64).
#[test]
#[ignore = "cost probe: run with --ignored --nocapture"]
fn e2_cost_grid() {
    for k in [1usize, 2, 3, 4] {
        let mut body = brick(
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(2.0 * k as f64, 2.0 * k as f64, 1.0),
        );
        for i in 0..k {
            for j in 0..k {
                let (x, y) = (2.0 * i as f64 + 0.5, 2.0 * j as f64 + 0.5);
                body = fuse(
                    &body,
                    &brick(Point3::new(x, y, 0.5), Point3::new(x + 1.0, y + 1.0, 2.0)),
                );
            }
        }
        let edges = edges_with_corners(&body, |p| (p.z - 2.0).abs() < 1e-9);
        let req = BlendRequest {
            body: &body,
            edges: edges.clone(),
            size: 0.1,
        };
        let t = Instant::now();
        let r = band_reach(&req, band());
        eprintln!(
            "E2 k {k}: links {} faces {}: f64 {:?} {:?}",
            edges.len(),
            body.faces().count(),
            t.elapsed(),
            r.err()
        );
    }
}

/// A prism whose profile is a bulge loop, `z ∈ [z0, z1]`.
fn bulged(pts: Vec<(Point2<f64>, f64)>, z0: f64, z1: f64) -> Body<f64> {
    use profile::{Profile, RawLoop, test_support::bulge_loop};
    use sweep::{ExtrudeSide, Extrusion, extrude};
    let n = pts.len();
    let lp = bulge_loop(pts).with_tangent_joints((0..n).collect());
    let profile = Profile::new(crate::common::cavity::sketch_at(z0), vec![lp])
        .validate(tol())
        .expect("profile");
    extrude(
        &profile,
        Extrusion::Distance {
            depth: z1 - z0,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .expect("extrude")
    .body
}

/// G: an obround boss's top rim (line, half-arc, line, half-arc, G1 at
/// each junction): at a junction the straight link's "end face" is the
/// tangent cylinder, whose normal is ⊥ the spine, so the window's pad
/// `reach·|n×τ|/|n·τ|` divides by zero. A post stands in line with the
/// straight edge, 1.5 past the arc's far point.
#[test]
fn g_an_obround_rim_beside_an_in_line_post() {
    let p = |x: f64, y: f64| Point2::new(x, y);
    let plate = brick(Point3::new(-6.0, -3.0, 0.0), Point3::new(6.0, 3.0, 1.0));
    let boss = bulged(
        vec![
            (p(-1.0, -1.0), 0.0),
            (p(1.0, -1.0), 1.0),
            (p(1.0, 1.0), 0.0),
            (p(-1.0, 1.0), 1.0),
        ],
        0.5,
        2.0,
    );
    let rim_only = fuse(&plate, &boss);
    let rim =
        |b: &Body<f64>| edges_with_corners(b, |q| (q.z - 2.0).abs() < 1e-9 && q.x.abs() < 2.5);
    let e = rim(&rim_only);
    eprintln!("G rim edges {}", e.len());
    let _ = run("G no post", &rim_only, &e, 0.2);
    let post = brick(Point3::new(3.5, 0.8, 0.5), Point3::new(4.0, 1.2, 2.5));
    let body = fuse(&rim_only, &post);
    let e = rim(&body);
    let req = BlendRequest {
        body: &body,
        edges: e.clone(),
        size: 0.2,
    };
    eprintln!("G meter: {:?}", band_reach(&req, band()).err());
    let _ = run("G post", &body, &e, 0.2);
    // Latent over-refusal (fillet_edges refuses the mixed closed chain
    // upstream today): the meter refuses a post 1.5 off the band.
    assert!(
        band_reach(&req, band()).is_err(),
        "G: pinned latent over-refusal"
    );
}

/// H: a rounded-rectangle block (corner radius 1) with a thin through
/// hole 1.0 inboard of a corner arc's centre, on the diagonal: the hole
/// lies on the corner arc's circle, on the side away from the arc. The
/// top rim's band (r = 0.2) is ≥ 1.7 from the hole.
#[test]
fn h_a_hole_on_the_far_side_of_a_corner_arcs_circle() {
    let p = |x: f64, y: f64| Point2::new(x, y);
    let b = (std::f64::consts::PI / 8.0).tan();
    let block = bulged(
        vec![
            (p(1.0, 0.0), 0.0),
            (p(5.0, 0.0), b),
            (p(6.0, 1.0), 0.0),
            (p(6.0, 5.0), b),
            (p(5.0, 6.0), 0.0),
            (p(1.0, 6.0), b),
            (p(0.0, 5.0), 0.0),
            (p(0.0, 1.0), b),
        ],
        0.0,
        2.0,
    );
    let rim = |b: &Body<f64>| {
        edges_with_corners(b, |q| (q.z - 2.0).abs() < 1e-9 && (q.x - 3.0).abs() > 0.0)
    };
    let all_rim = |b: &Body<f64>| rim(b).into_iter().filter(|_| true).collect::<Vec<_>>();
    let e = all_rim(&block);
    eprintln!("H rim edges {}", e.len());
    let _ = run("H no hole", &block, &e, 0.2);
    let d = 1.0 / 2f64.sqrt();
    let hole = rod(Point2::new(5.0 - d, 5.0 - d), 0.1, -1.0, 3.0);
    let body = cut("hole", &block, &hole);
    let e: Vec<_> = all_rim(&body)
        .into_iter()
        .filter(|k| {
            // keep the outer rim only: the hole's own rim sits near (4.29, 4.29)
            let _ = k;
            true
        })
        .collect();
    let outer = edges_with_corners(&body, |q| {
        (q.z - 2.0).abs() < 1e-9
            && ((q.x - (5.0 - d)).powi(2) + (q.y - (5.0 - d)).powi(2)).sqrt() > 0.5
    });
    eprintln!("H rim edges with hole {} outer {}", e.len(), outer.len());
    let req = BlendRequest {
        body: &body,
        edges: outer.clone(),
        size: 0.2,
    };
    eprintln!("H meter: {:?}", band_reach(&req, band()).err());
    let _ = run("H hole", &body, &outer, 0.2);
    // Latent over-refusal: the corner arc's reach is its whole turn.
    assert!(
        band_reach(&req, band()).is_err(),
        "H: pinned latent over-refusal"
    );
}

/// C2: the boss foot's concave band against a wall HOVERING `lift`
/// above the plate, `gap` off the boss, as a second solid: no boundary
/// feature on the plate, so only the reach can see it.
#[test]
fn c2_a_concave_boss_foot_beside_a_hovering_wall() {
    let r = 0.2;
    // At height h above the plate the band reaches ρ = 0.5 + r − √(r² − (r − h)²).
    for (lift, gap) in [
        (0.05, 0.05),
        (0.05, 0.06),
        (0.05, 0.075),
        (0.05, 0.1),
        (0.25, 0.01),
    ] {
        let plate = brick(Point3::new(-3.0, -3.0, 0.0), Point3::new(3.0, 3.0, 1.0));
        let boss = rod(Point2::new(0.0, 0.0), 0.5, 0.5, 2.0);
        let wall = brick(
            Point3::new(0.5 + gap, -1.0, 1.0 + lift),
            Point3::new(0.9 + gap, 1.0, 2.5),
        );
        let body = fuse(&fuse(&plate, &boss), &wall);
        let edges = boss_foot(&body);
        let h = lift;
        let reach = 0.5 + r - (r * r - (r - h).powi(2)).max(0.0).sqrt();
        let what =
            format!("C2 lift {lift} gap {gap} (band reaches rho {reach:.4} at the wall's foot)");
        let res = run(&what, &body, &edges, r);
        if 0.5 + gap < reach {
            assert!(res.is_err(), "{what}: the band reaches the wall");
        }
    }
}

/// I: a D-shaped boss (a cylinder `ρ ≤ 1` cut flat at `x = 0.5`) on a
/// plate; its top rim ARC alone is filleted (an open circular chain cut
/// off at the flat). A post stands where the arc's circle would run if
/// it went on past the flat, `0.3` beyond the flat's plane.
#[test]
fn i_a_d_boss_arc_beside_a_post_past_its_flat() {
    let plate = brick(Point3::new(-3.0, -3.0, 0.0), Point3::new(3.0, 3.0, 1.0));
    let boss = cut(
        "flat",
        &rod(Point2::new(0.0, 0.0), 1.0, 0.5, 2.0),
        &brick(Point3::new(0.5, -2.0, 0.0), Point3::new(2.0, 2.0, 3.0)),
    );
    let base = fuse(&plate, &boss);
    let arc = |b: &Body<f64>| {
        edges_with_corners(b, |q| {
            (q.z - 2.0).abs() < 1e-9 && ((q.x * q.x + q.y * q.y).sqrt() - 1.0).abs() < 1e-9
        })
        .into_iter()
        .filter(|k| {
            !edges_with_corners(b, |q| (q.z - 2.0).abs() < 1e-9 && (q.x - 0.5).abs() < 1e-9)
                .contains(k)
        })
        .collect::<Vec<_>>()
    };
    let e = arc(&base);
    eprintln!("I arc edges {}", e.len());
    let _ = run("I no post", &base, &e, 0.2);
    for (x0, label) in [
        (0.8, "post at x 0.8..1.1"),
        (0.85, "post at x 0.85..1.15"),
        (1.3, "post at x 1.3..1.6"),
    ] {
        let post = brick(Point3::new(x0, -0.1, 0.5), Point3::new(x0 + 0.3, 0.1, 2.5));
        let body = fuse(&base, &post);
        let e = arc(&body);
        let req = BlendRequest {
            body: &body,
            edges: e.clone(),
            size: 0.2,
        };
        eprintln!("I {label} meter: {:?}", band_reach(&req, band()).err());
        let _ = run(&format!("I {label}"), &body, &e, 0.2);
    }
}

/// J: a circular-segment boss (`ρ ≤ 1`, `y ≤ −0.5`: one 120° arc edge on
/// top, cut off at the flat `y = −0.5`) on a plate, its arc filleted
/// alone; a post stands on the far side of the arc's circle, at
/// `y ∈ [0.85, 1.15]`, 1.35 past the flat's plane.
#[test]
fn j_a_segment_boss_arc_beside_a_post_across_its_circle() {
    let plate = brick(Point3::new(-3.0, -3.0, 0.0), Point3::new(3.0, 3.0, 1.0));
    let boss = cut(
        "flat",
        &rod(Point2::new(0.0, 0.0), 1.0, 0.5, 2.0),
        &brick(Point3::new(-2.0, -0.5, 0.0), Point3::new(2.0, 2.0, 3.0)),
    );
    let base = fuse(&plate, &boss);
    let arc = |b: &Body<f64>| {
        edges_with_corners(b, |q| {
            (q.z - 2.0).abs() < 1e-9 && ((q.x * q.x + q.y * q.y).sqrt() - 1.0).abs() < 1e-9
        })
        .into_iter()
        .filter(|k| {
            let e = b.get_edge(*k).unwrap();
            let c = b.get_curve_geom(e.curve).unwrap().certified().unwrap();
            matches!(c.carrier(), geom::Curve3::Circle { .. })
        })
        .collect::<Vec<_>>()
    };
    let e = arc(&base);
    eprintln!("J arc edges {}", e.len());
    let _ = run("J no post", &base, &e, 0.2);
    for (y0, label) in [(0.85, "post at y 0.85..1.15"), (1.5, "post at y 1.5..1.8")] {
        let post = brick(Point3::new(-0.1, y0, 0.5), Point3::new(0.1, y0 + 0.3, 2.5));
        let body = fuse(&base, &post);
        let e = arc(&body);
        let req = BlendRequest {
            body: &body,
            edges: e.clone(),
            size: 0.2,
        };
        eprintln!("J {label} meter: {:?}", band_reach(&req, band()).err());
        let _ = run(&format!("J {label}"), &body, &e, 0.2);
    }
}

/// K: probe A at `Interval`. The void's four walls are planes PARALLEL
/// to the rim band's axis, so `sheet_fn`'s plane arm divides `n·a = 0`
/// by `|n·a| = 0`: NaN at f64 (which `f64::max` drops), the empty
/// interval at `Interval`.
#[test]
fn k_the_convex_rim_against_a_void_at_interval() {
    use crate::common::interval::iv;
    use geom_core::{Interval, Real};
    let z = Interval::zero();
    let e = z / z.abs();
    let m = e.max(iv(-1.0));
    eprintln!(
        "K: 0/|0| = {e:?} (lo {} hi {}); max(that, -1) = {m:?} (lo {})",
        geom_core::Bounds::lo(e),
        geom_core::Bounds::hi(e),
        geom_core::Bounds::lo(m)
    );
    let t = tol();
    let drum = sweep::test_support::revolved_about_y_at::<Interval>(
        corners(&[(0.0, 0.0), (1.0, 0.0), (1.0, 2.0), (0.0, 2.0)]),
        Revolution::Full,
        t,
    );
    // B's near-clear void (its corner 0.01 inside the ball, so clear of
    // the material), placed as in `b_a_void_just_clear_of_the_convex_rim`.
    let d = (0.3 - 1e-2) / 2f64.sqrt();
    let (rho, y) = (0.7 + d, 1.7 + d);
    let xb = (rho * rho - 0.05 * 0.05).sqrt() - 0.1;
    // A void clear of the material but inside the reach's box, deep in
    // the drum: x ∈ [0.55, 0.65], y ∈ [1.75, 1.85].
    for (x0, y0, what) in [
        (0.85, 1.85, "interfering"),
        (0.4, 1.0, "control"),
        (xb, y - 0.1, "just clear (B gap 0.01)"),
        (0.55, 1.75, "clear, in the reach box"),
    ] {
        let void = sweep::test_support::brick::<Interval>(
            (x0, x0 + 0.1),
            (y0, y0 + 0.1),
            (-0.05, 0.05),
            t,
        );
        let a = topo::test_support::finished("drum", drum.clone(), t);
        let b = topo::test_support::finished("void", void, t);
        let body = topo::subtract(&a, &b, t)
            .expect("cut")
            .body()
            .expect("material")
            .body
            .clone()
            .into_body();
        let edges = rim_arcs_at(&body, 1.0, 2.0);
        let req = BlendRequest {
            body: &body,
            edges: edges.clone(),
            size: iv(0.3),
        };
        let meter = band_reach(&req, band());
        eprintln!("K {what}: meter {:?}", meter.as_ref().err());
        let built = fillet_edges(&body, &edges, iv(0.3), t);
        match &built {
            Ok(f) => eprintln!(
                "K {what}: BUILT tier3 {:?} V {:?}",
                validate_geometric(&f.body, t),
                mass_properties(&f.body, t).map(|m| m.volume)
            ),
            Err(e) => eprintln!("K {what}: REFUSED {:?}", e.error),
        }
        // f64 refuses the first FaceClearance and builds the other
        // three (probes A and B); Interval must agree. RED on d2fe38f5.
        if what == "interfering" {
            assert!(
                matches!(meter, Err(BlendError::FaceClearance { .. })),
                "K: the band cuts the void; f64 says FaceClearance: {meter:?}"
            );
        } else {
            assert!(meter.is_ok(), "K {what}: f64 builds this body: {meter:?}");
        }
    }
}
