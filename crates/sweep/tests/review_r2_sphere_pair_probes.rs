//! Review r2 probes for PR 4344 (two spheres crossing in a circle no
//! edge reaches). Every row is `#[ignore]`d and prints one line per run,
//! read through `common::differential::outcome`, plus the body's face
//! census (faces, sphere faces, seam edges, edges between two distinct
//! faces on one sphere) and `check_mesh`. Run on base and head and diffed.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::HashMap;
use std::f64::consts::PI;

use geom_core::{Affine3, Mat3, Tol, Vec3};
use sweep::test_support::{ball_poled_y, brick, finished};
use topo::{AtRestBody, BooleanError, BooleanResult};

use crate::common::differential::outcome;

fn tol() -> Tol {
    Tol::witness()
}

/// A ball of radius `r` at `c`, its polar axis along `p` and its seam
/// meridian toward `u` (`u ⟂ p`, both unit).
fn ball_frame(r: f64, c: Vec3<f64>, p: Vec3<f64>, u: Vec3<f64>) -> AtRestBody<f64> {
    let (p, u) = (p / p.norm(), u / u.norm());
    let u = (u - p * u.dot(p)) / (u - p * u.dot(p)).norm();
    let at = ball_poled_y(r, Vec3::new(0.0, 0.0, 0.0), tol());
    let m = Affine3::from_parts(Mat3::from_cols(u, p, u.cross(p)), c);
    finished("ball", topo::transform_rigid(&at, &m, tol()).unwrap(), tol())
}

fn unit_x() -> Vec3<f64> {
    Vec3::new(1.0, 0.0, 0.0)
}
fn unit_y() -> Vec3<f64> {
    Vec3::new(0.0, 1.0, 0.0)
}
fn unit_z() -> Vec3<f64> {
    Vec3::new(0.0, 0.0, 1.0)
}

fn ball_volume(r: f64) -> f64 {
    4.0 / 3.0 * PI * r.powi(3)
}

/// The standard closed-form lens of two crossing balls (NOT the
/// suite's cap sum): `π (R + r − d)² (d² + 2dr − 3r² + 2dR + 6rR − 3R²) / 12d`.
fn lens(rr: f64, r: f64, d: f64) -> f64 {
    if d >= rr + r {
        return 0.0;
    }
    if d <= (rr - r).abs() {
        return ball_volume(rr.min(r));
    }
    PI * (rr + r - d).powi(2)
        * (d * d + 2.0 * d * r - 3.0 * r * r + 2.0 * d * rr + 6.0 * r * rr - 3.0 * rr * rr)
        / (12.0 * d)
}

/// Faces, sphere faces, seam edges (one face both sides), and edges
/// between two DISTINCT faces whose surfaces are one sphere.
fn census(body: &AtRestBody<f64>) -> String {
    let hes: HashMap<_, _> = body.half_edges().collect();
    let loops: HashMap<_, _> = body.loops().collect();
    let faces: HashMap<_, _> = body.faces().collect();
    let sphere = |f| match body.get_surface(faces[&f].surface) {
        Some(geom::Surface::Sphere { center, radius, .. }) => {
            Some((center.x, center.y, center.z, *radius))
        }
        _ => None,
    };
    let n_faces = faces.len();
    let n_sph = faces.keys().filter(|&&f| sphere(f).is_some()).count();
    let (mut seam, mut same) = (0, 0);
    for (_, e) in body.edges() {
        let fp = loops[&hes[&e.he_plus].parent_loop].face;
        let fm = loops[&hes[&e.he_minus].parent_loop].face;
        if fp == fm {
            seam += 1;
        } else if let (Some(a), Some(b)) = (sphere(fp), sphere(fm)) {
            let close = |x: f64, y: f64| (x - y).abs() < 1e-9;
            if close(a.0, b.0) && close(a.1, b.1) && close(a.2, b.2) && close(a.3, b.3) {
                same += 1;
            }
        }
    }
    let mesh = match mesh::tessellate(body, 0.05, tol()) {
        Ok(m) => match mesh::validate::check_mesh(&m) {
            Ok(()) => "mesh=ok".to_string(),
            Err(e) => format!("mesh=BAD {:?}", format!("{e:?}").chars().take(60).collect::<String>()),
        },
        Err(e) => format!("mesh=ERR {}", format!("{e:?}").chars().take(60).collect::<String>()),
    };
    format!("faces={n_faces} sph={n_sph} seam={seam} samesph={same} {mesh}")
}

fn line(tag: &str, r: Result<BooleanResult<f64>, BooleanError>, want: f64) {
    let cen = match &r {
        Ok(BooleanResult::Body(bb)) => census(&bb.body),
        _ => String::new(),
    };
    println!("{tag} => {} | {cen}", outcome(r, want, tol()));
}

/// Every op in both orders, against `(va, vb, shared)`.
fn six(tag: &str, a: &AtRestBody<f64>, b: &AtRestBody<f64>, (va, vb, sh): (f64, f64, f64)) {
    line(&format!("{tag} a∪b"), topo::union(a, b, tol()), va + vb - sh);
    line(&format!("{tag} b∪a"), topo::union(b, a, tol()), va + vb - sh);
    line(&format!("{tag} a∖b"), topo::subtract(a, b, tol()), va - sh);
    line(&format!("{tag} b∖a"), topo::subtract(b, a, tol()), vb - sh);
    line(&format!("{tag} a∩b"), topo::intersect(a, b, tol()), sh);
    line(&format!("{tag} b∩a"), topo::intersect(b, a, tol()), sh);
}

/// A unit vector perpendicular to `n`, turned by `phi` about it.
fn perp(n: Vec3<f64>, phi: f64) -> Vec3<f64> {
    let n = n / n.norm();
    let seed = if n.x.abs() < 0.9 { unit_x() } else { unit_y() };
    let e1 = (seed - n * seed.dot(n)) / (seed - n * seed.dot(n)).norm();
    let e2 = n.cross(e1);
    e1 * phi.cos() + e2 * phi.sin()
}

/// P1: non-parallel charts. The unit ball at the origin with pole
/// `perp(n, 0)`, the other ball (radius `r`) at `d·n` with pole
/// `perp(n, phi)`: both seams on the great circle normal to `n`, so the
/// circle stays off both, while the charts' axes differ by `phi`.
#[test]
#[ignore = "review probe"]
fn r2_nonparallel_charts() {
    let dirs = [
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.3, -0.2, 0.93),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(-0.7, 0.1, 0.2),
    ];
    for (i, n) in dirs.iter().enumerate() {
        let n = *n / n.norm();
        for phi in [0.0, 0.3, 1.1, PI / 2.0, 2.5] {
            for (r, t) in [(0.3, 0.5), (0.7, 0.3), (1.5, 0.6), (0.05, 0.5)] {
                let (lo, hi) = ((1.0f64 - r).abs(), 1.0 + r);
                let d = lo + t * (hi - lo);
                let pa = perp(n, 0.0);
                let a = ball_frame(1.0, Vec3::new(0.0, 0.0, 0.0), pa, pa.cross(n));
                let pb = perp(n, phi);
                let b = ball_frame(r, n * d, pb, pb.cross(n));
                six(
                    &format!("P1 dir{i} phi={phi:.3} r={r} d={d:.4}"),
                    &a,
                    &b,
                    (ball_volume(1.0), ball_volume(r), lens(1.0, r, d)),
                );
            }
        }
    }
}

/// P2: a small ball near the pole of the big ball's chart. The big ball
/// is `y`-poled with its seam toward `-x`... the small ball's centre is
/// at angle `theta` from `+y` in the `y`–`x` plane (away from the seam),
/// at depth `d`.
#[test]
#[ignore = "review probe"]
fn r2_near_the_pole() {
    for theta_deg in [0.0, 0.5, 2.0, 5.0, 10.0, 20.0, 35.0] {
        for (r, d) in [(0.2, 1.0), (0.3, 0.9), (0.05, 1.0)] {
            let th = f64::to_radians(theta_deg);
            let n = Vec3::new(th.sin(), th.cos(), 0.0);
            let a = ball_frame(1.0, Vec3::new(0.0, 0.0, 0.0), unit_y(), -unit_x());
            // The small ball poled across the centre line, its seam
            // on the far side of the circle's great circle.
            let pb = n.cross(unit_z());
            let b = ball_frame(r, n * d, pb, unit_z());
            six(
                &format!("P2 theta={theta_deg} r={r} d={d}"),
                &a,
                &b,
                (ball_volume(1.0), ball_volume(r), lens(1.0, r, d)),
            );
        }
    }
}

/// P3: the circle passing within a band of the big ball's pole: the
/// small ball's circle on the big sphere has angular radius `alpha`;
/// its centre sits `alpha + delta` off the pole, so the circle passes
/// `delta` (angular) from the pole without holding it.
#[test]
#[ignore = "review probe"]
fn r2_circle_grazing_the_pole() {
    let r = 0.3f64;
    let d = 1.0f64;
    // Circle on the unit sphere: centre direction n, foot s along n.
    let s = (d * d + 1.0 - r * r) / (2.0 * d);
    let alpha = s.acos();
    for delta in [1e-2, 1e-4, 1e-6, 1e-8, 1e-10, 0.0, -1e-6] {
        let th = alpha + delta;
        let n = Vec3::new(th.sin(), th.cos(), 0.0);
        let a = ball_frame(1.0, Vec3::new(0.0, 0.0, 0.0), unit_y(), -unit_x());
        let pb = n.cross(unit_z());
        let b = ball_frame(r, n * d, pb, unit_z());
        six(
            &format!("P3 delta={delta:e}"),
            &a,
            &b,
            (ball_volume(1.0), ball_volume(r), lens(1.0, r, d)),
        );
    }
}

/// P4: three balls. `b` is the union of two disjoint small balls that
/// each cross the unit ball off every edge, so the unit ball's one face
/// takes two cut-ins from two partners: at two longitudes, on ONE
/// longitude at two latitudes, and in mirror positions.
#[test]
#[ignore = "review probe"]
fn r2_one_face_two_partners() {
    let a = ball_frame(1.0, Vec3::new(0.0, 0.0, 0.0), unit_y(), unit_x());
    let r = 0.25;
    let d = 1.0;
    let placements: [(&str, Vec3<f64>, Vec3<f64>); 5] = [
        ("z and -z", Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 0.0, -1.0)),
        ("z and -x", Vec3::new(0.0, 0.0, 1.0), Vec3::new(-1.0, 0.0, 0.0)),
        (
            "one longitude",
            Vec3::new(0.0, 0.5f64.sin(), 0.5f64.cos()),
            Vec3::new(0.0, -(0.5f64.sin()), 0.5f64.cos()),
        ),
        (
            "tilted pair",
            Vec3::new(-0.3, 0.4, 0.8),
            Vec3::new(-0.6, -0.5, -0.4),
        ),
        (
            "neighbours",
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(-(0.6f64.sin()), 0.0, 0.6f64.cos()),
        ),
    ];
    for (name, n1, n2) in placements {
        let (n1, n2) = (n1 / n1.norm(), n2 / n2.norm());
        let mk = |n: Vec3<f64>| {
            let p = perp(n, 0.4);
            ball_frame(r, n * d, p, p.cross(n))
        };
        let b = match topo::union(&mk(n1), &mk(n2), tol()) {
            Ok(BooleanResult::Body(bb)) => bb.body.clone(),
            other => {
                println!("P4 {name}: the partners' union refused {:?}", other.err());
                continue;
            }
        };
        let gap = (n1 * d - n2 * d).norm();
        let vb = 2.0 * ball_volume(r) - lens(r, r, gap);
        six(
            &format!("P4 {name}"),
            &a,
            &b,
            (ball_volume(1.0), vb, 2.0 * lens(1.0, r, d)),
        );
    }
}

/// P5: radius ratios 1e-3 and 1e3 and the near-tangent regime through
/// `sphere_pair_cut`'s cancellation: the unit ball against `(r, d·z)`.
#[test]
#[ignore = "review probe"]
fn r2_extreme_ratios() {
    for r in [1e-3, 3e-3, 1e3, 1e-2, 50.0] {
        let (lo, hi) = ((1.0f64 - r).abs(), 1.0 + r);
        for t in [1e-6, 0.01, 0.5, 0.99, 1.0 - 1e-6] {
            let d = lo + t * (hi - lo);
            let a = ball_frame(1.0, Vec3::new(0.0, 0.0, 0.0), unit_y(), unit_x());
            let b = ball_frame(r, unit_z() * d, unit_y(), unit_x());
            six(
                &format!("P5 r={r:e} t={t:e} d={d:.12}"),
                &a,
                &b,
                (ball_volume(1.0), ball_volume(r), lens(1.0, r, d)),
            );
        }
    }
}

/// P6: face census against the crossing layer's answer of the SAME
/// point sets: the plain witness (seams off the circle, scan path)
/// beside the same balls turned so each seam crosses the circle
/// (crossing-layer path). Volumes match by construction.
#[test]
#[ignore = "review probe"]
fn r2_face_census_scan_vs_crossing_layer() {
    for (r, d) in [(0.3, 0.95), (0.7, 0.9), (1.5, 1.2)] {
        let o = Vec3::new(0.0, 0.0, 0.0);
        let want = (ball_volume(1.0), ball_volume(r), lens(1.0, r, d));
        let a = ball_frame(1.0, o, unit_y(), unit_x());
        let b = ball_frame(r, unit_z() * d, unit_y(), unit_x());
        six(&format!("P6 scan r={r} d={d}"), &a, &b, want);
        // Poled along the centre line: every meridian, the seam
        // included, crosses the circle.
        let a = ball_frame(1.0, o, unit_z(), unit_x());
        let b = ball_frame(r, unit_z() * d, unit_z(), unit_x());
        six(&format!("P6 layer r={r} d={d}"), &a, &b, want);
    }
}

/// P7: the plane + sphere refusal: does `FallbackExtentUnsupported`
/// "rename" fire on a pose main built? A ball poking a slab face whose
/// OTHER operand also has a sphere it crosses in a circle that a seam
/// DOES reach (crossing layer), and a ball poking a slab in one op and
/// crossing the other operand's sphere off every edge.
#[test]
#[ignore = "review probe"]
fn r2_plane_and_sphere() {
    let o = Vec3::new(0.0, 0.0, 0.0);
    let unit = ball_frame(1.0, o, unit_y(), unit_x());
    for (zb, zs) in [(0.5, -0.95), (0.8, -0.9), (-0.5, -0.95)] {
        let slab = finished(
            "slab",
            brick((-3.0, 3.0), (-3.0, 3.0), (zb, zb + 3.0), tol()),
            tol(),
        );
        for (pole, tag) in [(unit_y(), "seam off"), (unit_z(), "seam on")] {
            let small = ball_frame(0.3, unit_z() * zs, pole, unit_x());
            let tool = match topo::union(&slab, &small, tol()) {
                Ok(BooleanResult::Body(bb)) => bb.body.clone(),
                other => {
                    println!("P7 tool refused {:?}", other.err());
                    continue;
                }
            };
            // Volumes: slab ∩ unit is a cap of height 1 - zb (zb ≥ -1).
            let slab_cap = if zb > -1.0 {
                PI * (1.0 - zb).powi(2) * (3.0 - (1.0 - zb)) / 3.0
            } else {
                ball_volume(1.0)
            };
            let vt = 36.0 * 3.0 + ball_volume(0.3);
            let sh = slab_cap + lens(1.0, 0.3, zs.abs());
            six(
                &format!("P7 zb={zb} zs={zs} {tag}"),
                &unit,
                &tool,
                (ball_volume(1.0), vt, sh),
            );
        }
    }
}

/// P8: the BAD rows' reasons: tier 3′'s errors on P4's two-lump
/// results, and the far-brick union's refusal on P5's slivers.
#[test]
#[ignore = "review probe"]
fn r2_bad_reasons() {
    let a = ball_frame(1.0, Vec3::new(0.0, 0.0, 0.0), unit_y(), unit_x());
    let (r, d) = (0.25, 1.0);
    let mk = |n: Vec3<f64>| {
        let p = perp(n, 0.4);
        ball_frame(r, n * d, p, p.cross(n))
    };
    let b = match topo::union(&mk(unit_z()), &mk(-unit_z()), tol()) {
        Ok(BooleanResult::Body(bb)) => bb.body.clone(),
        _ => panic!(),
    };
    for (op, out) in [
        ("b∖a", topo::subtract(&b, &a, tol())),
        ("a∩b", topo::intersect(&a, &b, tol())),
    ] {
        if let Ok(BooleanResult::Body(bb)) = out {
            println!(
                "P8 P4 z and -z {op}: shells={} t3p={:?}",
                bb.body.shells().count(),
                topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol())
                    .map_err(|e| format!("{e:?}").chars().take(300).collect::<String>())
            );
        }
    }
    // The same two-lump shape from two crossings the crossing layer sees
    // (poles along each centre line): is tier 3′ undecidable there too?
    let mkl = |n: Vec3<f64>| ball_frame(r, n * d, n, perp(n, 0.0));
    let al = ball_frame(1.0, Vec3::new(0.0, 0.0, 0.0), unit_z(), unit_x());
    let bl = match topo::union(&mkl(unit_z()), &mkl(-unit_z()), tol()) {
        Ok(BooleanResult::Body(bb)) => bb.body.clone(),
        _ => panic!(),
    };
    for (op, out) in [
        ("b∖a", topo::subtract(&bl, &al, tol())),
        ("a∩b", topo::intersect(&al, &bl, tol())),
    ] {
        match out {
            Ok(BooleanResult::Body(bb)) => println!(
                "P8 layer z and -z {op}: shells={} t3p={:?}",
                bb.body.shells().count(),
                topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol())
                    .map_err(|e| format!("{e:?}").chars().take(300).collect::<String>())
            ),
            other => println!("P8 layer {op}: {:?}", other.err()),
        }
    }
    for (r, d, op) in [(50.0, 49.000002, "a∖b"), (50.0, 50.999998, "a∩b")] {
        let a = ball_frame(1.0, Vec3::new(0.0, 0.0, 0.0), unit_y(), unit_x());
        let b = ball_frame(r, unit_z() * d, unit_y(), unit_x());
        let out = if op == "a∖b" {
            topo::subtract(&a, &b, tol())
        } else {
            topo::intersect(&a, &b, tol())
        };
        if let Ok(BooleanResult::Body(bb)) = out {
            let far = finished(
                "far",
                brick((50.0, 51.0), (50.0, 51.0), (50.0, 51.0), tol()),
                tol(),
            );
            let v = topo::mass_properties(&bb.body, tol()).map(|m| m.volume);
            println!(
                "P8 P5 r={r} d={d} {op}: v={v:?} want={} far-union={:?}",
                if op == "a∖b" { ball_volume(1.0) - lens(1.0, r, d) } else { lens(1.0, r, d) },
                topo::union(&bb.body, &far, tol()).err()
            );
        }
    }
}

/// P9: the PR's own near-tangency poses read through `outcome` (its
/// suite never asks the legal-operand question).
#[test]
#[ignore = "review probe"]
fn r2_near_tangency_through_outcome() {
    for r in [0.05, 0.3, 0.7, 1.5] {
        let (lo, hi) = ((1.0f64 - r).abs(), 1.0 + r);
        for d in [lo + 1e-4, lo + 1e-6, hi - 1e-4, hi - 1e-6] {
            let a = ball_frame(1.0, Vec3::new(0.0, 0.0, 0.0), unit_y(), unit_x());
            let b = ball_frame(r, unit_z() * d, unit_y(), unit_x());
            six(
                &format!("P9 r={r} d={d:.9}"),
                &a,
                &b,
                (ball_volume(1.0), ball_volume(r), lens(1.0, r, d)),
            );
        }
    }
}

/// P10: the full refusals of P5's refusing poses (a ∪ b).
#[test]
#[ignore = "review probe"]
fn r2_ratio_refusals_in_full() {
    for (r, t) in [
        (1e-2, 1e-6),
        (1e-2, 1.0 - 1e-6),
        (1e-3, 0.5),
        (1e-3, 1e-6),
        (3e-3, 0.5),
        (3e-3, 1e-6),
        (1e3, 0.5),
    ] {
        let (lo, hi) = ((1.0f64 - r).abs(), 1.0 + r);
        let d = lo + t * (hi - lo);
        let a = ball_frame(1.0, Vec3::new(0.0, 0.0, 0.0), unit_y(), unit_x());
        let b = ball_frame(r, unit_z() * d, unit_y(), unit_x());
        let out = topo::union(&a, &b, tol());
        println!("P10 r={r:e} t={t:e}: {:?}", out.err());
    }
}

/// P11: P2's ring refusals in full, and the same pose with the big
/// ball's pole turned perpendicular to the centre line (P1's shape)
/// and with the small ball's chart turned instead.
#[test]
#[ignore = "review probe"]
fn r2_pole_ring_in_full() {
    let th = f64::to_radians(35.0);
    let n = Vec3::new(th.sin(), th.cos(), 0.0);
    for (r, d) in [(0.2, 1.0), (0.3, 0.9)] {
        let pb = n.cross(unit_z());
        let b = ball_frame(r, n * d, pb, unit_z());
        let a_y = ball_frame(1.0, Vec3::new(0.0, 0.0, 0.0), unit_y(), -unit_x());
        let a_z = ball_frame(1.0, Vec3::new(0.0, 0.0, 0.0), unit_z(), -unit_x());
        let want = (ball_volume(1.0), ball_volume(r), lens(1.0, r, d));
        println!("P11 r={r} a∪b full: {:?}", topo::union(&a_y, &b, tol()).err());
        println!("P11 r={r} a∩b full: {:?}", topo::intersect(&a_y, &b, tol()).err());
        six(&format!("P11 bigpole⟂ r={r}"), &a_z, &b, want);
        // The small ball poled along the centre line instead (its seam
        // then crosses the circle: the crossing layer's door).
        let b_n = ball_frame(r, n * d, n, unit_z());
        six(&format!("P11 smallpole∥ r={r}"), &a_y, &b_n, want);
    }
}

/// P12: which pose shape rings. The big ball's pole tilted `beta`
/// toward the centre line `n` inside the plane `(pa, n)`; the small
/// ball poled `pa` (cuts coplanar) or `perp(n, 1.1)` (cuts not).
#[test]
#[ignore = "review probe"]
fn r2_coplanar_cuts_with_tilted_pole() {
    for dir in [Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.3, -0.2, 0.93)] {
        let n = dir / dir.norm();
        let pa = perp(n, 0.0);
        for beta_deg in [0.0, 5.0, 35.0, 60.0] {
            let beta = f64::to_radians(beta_deg);
            let p = pa * beta.cos() + n * beta.sin();
            let a = ball_frame(1.0, Vec3::new(0.0, 0.0, 0.0), p, pa.cross(n));
            for (r, d) in [(0.2, 1.0), (0.3, 0.9), (0.7, 0.9)] {
                let want = (ball_volume(1.0), ball_volume(r), lens(1.0, r, d));
                for (tag, pb) in [("coplanar", pa), ("apart", perp(n, 1.1))] {
                    let b = ball_frame(r, n * d, pb, pb.cross(n));
                    six(
                        &format!("P12 n={:.2},{:.2},{:.2} beta={beta_deg} r={r} {tag}", n.x, n.y, n.z),
                        &a,
                        &b,
                        want,
                    );
                }
            }
        }
    }
}

/// P13: a seam on the cut's own great circle. The big ball poled
/// `pa ⟂ n` with its seam toward `-n` (the antipode of the half-meridian
/// its cut runs on), and/or the small ball poled `pa` with its seam
/// toward `+n` (likewise for its cut, which runs on the `-n` side).
#[test]
#[ignore = "review probe"]
fn r2_seam_on_the_cut_great_circle() {
    for dir in [Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.3, -0.2, 0.93)] {
        let n = dir / dir.norm();
        let pa = perp(n, 0.0);
        for (r, d) in [(0.2, 1.0), (0.3, 0.9), (0.7, 0.9), (1.5, 1.2)] {
            let want = (ball_volume(1.0), ball_volume(r), lens(1.0, r, d));
            for (tag, ua, ub) in [
                ("A-seam-on-cut", -n, pa.cross(n)),
                ("B-seam-on-cut", pa.cross(n), n),
                ("both", -n, n),
            ] {
                let a = ball_frame(1.0, Vec3::new(0.0, 0.0, 0.0), pa, ua);
                let b = ball_frame(r, n * d, pa, ub);
                six(
                    &format!("P13 n={:.2},{:.2},{:.2} r={r} {tag}", n.x, n.y, n.z),
                    &a,
                    &b,
                    want,
                );
            }
        }
    }
}

/// P14: D5 provenance of each face of `a ∪ b`, scan path vs crossing
/// layer, same point sets (P6's r = 0.7 pose).
#[test]
#[ignore = "review probe"]
fn r2_face_provenance() {
    let o = Vec3::new(0.0, 0.0, 0.0);
    for (tag, pole) in [("scan", unit_y()), ("layer", unit_z())] {
        let a = ball_frame(1.0, o, pole, unit_x());
        let b = ball_frame(0.7, unit_z() * 0.9, pole, unit_x());
        if let Ok(BooleanResult::Body(bb)) = topo::union(&a, &b, tol()) {
            for (f, _) in bb.body.faces() {
                let p = bb.body.provenance(topo::EntityId::Face(f));
                println!("P14 {tag} {f:?}: {}", format!("{p:?}").chars().take(160).collect::<String>());
            }
        }
    }
}

/// P15: `Interval` near tangency and at a 1e-2 ratio: whether the
/// radical-plane cut (`R − s` cancelling as `s → R`) still certifies.
#[test]
#[ignore = "review probe"]
fn r2_interval_near_tangency() {
    use crate::common::interval::iv;
    use geom_core::{Bounds, Interval};
    let ball_iv = |r: f64, z: f64| -> AtRestBody<Interval> {
        let at = ball_poled_y(iv(r), Vec3::new(iv(0.0), iv(0.0), iv(z)), tol());
        finished("ball", at, tol())
    };
    for (r, d) in [(0.3, 1.3 - 1e-4), (0.3, 0.7 + 1e-4), (0.01, 1.0), (0.3, 1.3 - 1e-6)] {
        let (a, b) = (ball_iv(1.0, 0.0), ball_iv(r, d));
        let sh = lens(1.0, r, d);
        for (op, want, out) in [
            ("a∪b", ball_volume(1.0) + ball_volume(r) - sh, topo::union(&a, &b, tol())),
            ("a∩b", sh, topo::intersect(&a, &b, tol())),
        ] {
            let line = match out {
                Ok(res) => match res.body() {
                    Some(bb) => {
                        let t3 = topo::validate_geometric(&bb.body, tol()).is_ok();
                        match topo::mass_properties(&bb.body, tol()) {
                            Ok(m) => format!(
                                "OK t3={t3} v=[{:.12}, {:.12}] want={want:.12} in={}",
                                m.volume.lo(),
                                m.volume.hi(),
                                m.volume.lo() - 1e-9 <= want && want <= m.volume.hi() + 1e-9
                            ),
                            Err(e) => format!("OK t3={t3} unmeasured {e:?}"),
                        }
                    }
                    None => "EMPTY".into(),
                },
                Err(e) => format!("ERR {}", format!("{e:?}").chars().take(200).collect::<String>()),
            };
            println!("P15 eps={} r={r} d={d} {op} => {line}", tol().eps());
        }
    }
}
