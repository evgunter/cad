//! Copied by review r1 of PR 4345 from `join/fan-end-one-spelling-review`
//! (167784d6), `six` adapted to the at-rest operand doors.
//!
//! **Review probes for PR 4004** (the fan end's one home, strut
//! facing's one rule). A prism corner `V`, reflex or convex, top or
//! bottom, touches the bottom face of a large tilted cube whose face
//! lies on the plane through `V` with unit normal `m`; `m` runs over a
//! Fibonacci sphere, so the poses reach the pierce's whole-orbit and
//! bare-bisector struts in every germ configuration a prism corner
//! offers. A second family puts a revolved tube's rim vertex (the lone
//! vertex of a closed circle edge) on the face.
//!
//! The oracle is independent of the PR's Simpson rule: the part of an
//! extrusion `R × [0, 1]` on `m`'s side is `∫_R clamp(L, 0, 1) dA` with
//! `L` linear, which is `∫_R L⁺ − ∫_R (L − 1)⁺`, each a half-plane clip
//! of `R` and its first moments (a disc's in closed form).
//!
//! Every line is `differential::outcome`: tiers 2 and 3′, the
//! certificate, a legal operand, and the volume.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::differential::{area, ccw, moments, outcome};
use geom_core::{Point2, Point3, Tol};
use topo::test_support::{mapped_cube, prism_z};
use topo::{Body, BooleanDeclarations};

fn tol() -> Tol {
    Tol::witness()
}

const SIDE: f64 = 10.0;

fn unit(m: [f64; 3]) -> [f64; 3] {
    let n = m.iter().map(|c| c * c).sum::<f64>().sqrt();
    m.map(|c| c / n)
}

/// The cube of side [`SIDE`] standing on the plane through `v` with
/// unit normal `m`, on `m`'s side, centred on `v`.
fn cube_beyond(v: [f64; 3], m: [f64; 3]) -> Body<f64> {
    let m = unit(m);
    let u = if m[0].abs() < 0.9 {
        unit([0.0, m[2], -m[1]])
    } else {
        unit([m[1], -m[0], 0.0])
    };
    let w = [
        m[1] * u[2] - m[2] * u[1],
        m[2] * u[0] - m[0] * u[2],
        m[0] * u[1] - m[1] * u[0],
    ];
    mapped_cube::<f64>(
        move |x, y, z| {
            let (a, b, c) = (SIDE * (x - 0.5), SIDE * (y - 0.5), SIDE * z);
            Point3::new(
                v[0] + a * u[0] + b * w[0] + c * m[0],
                v[1] + a * u[1] + b * w[1] + c * m[1],
                v[2] + a * u[2] + b * w[2] + c * m[2],
            )
        },
        tol(),
    )
}

/// `∫ (a·x + b·y + c)⁺ dA` over a polygon (any winding made CCW).
fn pos_part_poly(p: &[(f64, f64)], a: f64, b: f64, c: f64) -> f64 {
    let f = |q: (f64, f64)| a * q.0 + b * q.1 + c;
    let n = p.len();
    let mut kept = Vec::new();
    for i in 0..n {
        let (s, e) = (p[i], p[(i + 1) % n]);
        if f(s) >= 0.0 {
            kept.push(s);
        }
        if (f(s) >= 0.0) != (f(e) >= 0.0) {
            let t = f(s) / (f(s) - f(e));
            kept.push((s.0 + t * (e.0 - s.0), s.1 + t * (e.1 - s.1)));
        }
    }
    if kept.len() < 3 {
        return 0.0;
    }
    let (mx, my) = moments(&kept);
    a * mx + b * my + c * area(&kept)
}

/// `∫ (a·x + b·y + c)⁺ dA` over the disc of radius `r` at the origin.
fn pos_part_disc(r: f64, a: f64, b: f64, c: f64) -> f64 {
    let g = (a * a + b * b).sqrt();
    if g == 0.0 {
        return c.max(0.0) * std::f64::consts::PI * r * r;
    }
    // g·(u − d), u along (a, b)/g, d = −c/g; ∫_{u ≥ d} 2(u − d)√(r² − u²).
    let d = (-c / g).clamp(-r, r);
    let s = (r * r - d * d).max(0.0).sqrt();
    let seg = r * r * (d / r).acos() - d * s;
    let first = 2.0 / 3.0 * s * s * s;
    let raw = g * (first - d * seg);
    // Beyond the disc on the near side the integrand stays linear.
    let dd = -c / g;
    if dd < -r {
        raw + g * (-r - dd) * std::f64::consts::PI * r * r
    } else {
        raw
    }
}

/// The volume of `R × [z0, z0 + 1]` (`R` in the `(p, q)` plane, the
/// extrusion along the third axis) where `mp·(p − vp) + mq·(q − vq) +
/// mz·(z − vz) ≥ 0`, given `∫_R (α·p + β·q + γ)⁺` as `pos`.
fn beyond(pos: &dyn Fn(f64, f64, f64) -> f64, v: [f64; 3], m: [f64; 3], z0: f64) -> f64 {
    let g0 = -(m[0] * v[0] + m[1] * v[1]);
    if m[2] == 0.0 {
        // The height is 1 where g ≥ 0: that part's area.
        let big = 1e9;
        return pos(m[0] * big, m[1] * big, g0 * big) - pos(m[0] * big, m[1] * big, g0 * big - 1.0);
    }
    // L = 1 − z*, z* = vz − z0 − g/mz with g = m0 p + m1 q + g0 (for mz > 0);
    // for mz < 0, L = z*.
    let (a, b, c) = if m[2] > 0.0 {
        (m[0] / m[2], m[1] / m[2], 1.0 - (v[2] - z0) + g0 / m[2])
    } else {
        (-m[0] / m[2], -m[1] / m[2], (v[2] - z0) - g0 / m[2])
    };
    pos(a, b, c) - pos(a, b, c - 1.0)
}

/// The prism corners: (name, profile, the corner).
fn profiles() -> Vec<(&'static str, Vec<(f64, f64)>, (f64, f64))> {
    let l = vec![
        (0.0, 0.0),
        (2.0, 0.0),
        (2.0, 1.0),
        (1.0, 1.0),
        (1.0, 2.0),
        (0.0, 2.0),
    ];
    let r315 = vec![
        (0.0, 0.0),
        (2.0, 2.0),
        (-2.0, 2.0),
        (-2.0, -2.0),
        (2.0, -2.0),
        (2.0, 0.0),
    ];
    // A shallow notch: reflex 200° at (1, 0.18).
    let shallow = vec![(0.0, 0.0), (1.0, 0.18), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)];
    // A sharp slot: reflex ~350° at (1, 1.8).
    let sharp = vec![
        (0.0, 0.0),
        (0.95, 0.0),
        (1.0, 1.8),
        (1.05, 0.0),
        (2.0, 0.0),
        (2.0, 2.0),
        (0.0, 2.0),
    ];
    vec![
        ("L-reflex", l.clone(), (1.0, 1.0)),
        ("L-convex", l, (0.0, 0.0)),
        ("r315", r315, (0.0, 0.0)),
        ("shallow", shallow, (1.0, 0.18)),
        ("sharp", sharp, (1.0, 1.8)),
    ]
}

fn sphere(n: usize) -> Vec<[f64; 3]> {
    let ga = std::f64::consts::PI * (3.0 - 5f64.sqrt());
    (0..n)
        .map(|i| {
            let y = 1.0 - 2.0 * (i as f64 + 0.5) / n as f64;
            let r = (1.0 - y * y).sqrt();
            let t = ga * i as f64;
            [r * t.cos(), r * t.sin(), y]
        })
        .collect()
}

fn six(tag: &str, x: &Body<f64>, y: &Body<f64>, (vx, vy, shared): (f64, f64, f64)) {
    let d = BooleanDeclarations::default();
    let fin = |b: &Body<f64>| topo::test_support::finished("fan operand", b.clone(), tol());
    let (x, y) = (&fin(x), &fin(y));
    for (order, p, q, vp, vq) in [("xy", x, y, vx, vy), ("yx", y, x, vy, vx)] {
        for (op, r, want) in [
            ("U", topo::union_with(p, q, &d, tol()), vp + vq - shared),
            ("I", topo::intersect_with(p, q, &d, tol()), shared),
            ("S", topo::subtract_with(p, q, &d, tol()), vp - shared),
        ] {
            println!("FANREV {tag} {order} {op} => {}", outcome(r, want, tol()));
        }
    }
}

/// The battery: every prism corner against every normal, both orders,
/// ∪ ∩ ∖, top and bottom corners.
#[test]
#[ignore = "review battery; prints FANREV lines"]
fn fan_end_review_prism_battery() {
    let n: usize = std::env::var("FANREV_N")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(96);
    for (name, prof, k) in profiles() {
        let prof = ccw(prof);
        let prism = prism_z::<f64>(&prof, 0.0, 1.0, tol()).body;
        let vp = area(&prof);
        let pos = |a: f64, b: f64, c: f64| pos_part_poly(&prof, a, b, c);
        for (zname, z) in [("top", 1.0), ("bot", 0.0)] {
            let v = [k.0, k.1, z];
            for (i, m) in sphere(n).into_iter().enumerate() {
                let cube = cube_beyond(v, m);
                let shared = beyond(&pos, v, m, 0.0);
                let tag = format!("{name} {zname} m{i}=({:.3},{:.3},{:.3})", m[0], m[1], m[2]);
                six(&tag, &prism, &cube, (vp, SIDE.powi(3), shared));
            }
        }
    }
}

/// The oracle against the PR's Simpson rule on the L-prism, every normal.
#[test]
fn the_clamp_oracle_matches_a_brute_slice_sum() {
    let prof = vec![
        (0.0, 0.0),
        (2.0, 0.0),
        (2.0, 1.0),
        (1.0, 1.0),
        (1.0, 2.0),
        (0.0, 2.0),
    ];
    let pos = |a: f64, b: f64, c: f64| pos_part_poly(&prof, a, b, c);
    for m in sphere(64) {
        let v = [1.0, 1.0, 1.0];
        let got = beyond(&pos, v, m, 0.0);
        // Midpoint slices: area of the profile where m·(p − v) ≥ 0 at z.
        let k = 4000;
        let brute: f64 = (0..k)
            .map(|i| {
                let z = (i as f64 + 0.5) / k as f64;
                let c = m[2] * (z - v[2]) - m[0] * v[0] - m[1] * v[1];
                let big = 1e9;
                (pos(m[0] * big, m[1] * big, c * big) - pos(m[0] * big, m[1] * big, c * big - 1.0))
                    / k as f64
            })
            .sum();
        assert!((got - brute).abs() < 1e-5, "m={m:?}: {got} vs {brute}");
    }
}

/// A full revolve about `y` of the annulus `bore ≤ ρ ≤ r`, `y ∈ [0, h]`.
fn tube(bore: f64, r: f64, h: f64) -> Body<f64> {
    sweep::test_support::revolved_about_y(
        vec![
            (Point2::new(bore, 0.0), 0.0),
            (Point2::new(r, 0.0), 0.0),
            (Point2::new(r, h), 0.0),
            (Point2::new(bore, h), 0.0),
        ],
        sweep::Revolution::Full,
        tol(),
    )
}

/// A tube's outer top rim vertex `(r, 1, 0)` (the lone vertex of a
/// closed circle edge) on the cube's face, every normal, plus the
/// normals that hold the rim's tangent `±z`.
#[test]
#[ignore = "review battery; prints FANREV lines"]
fn fan_end_review_rim_battery() {
    let n: usize = std::env::var("FANREV_N")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(96);
    let (bore, r) = (0.5, 1.0);
    let t = tube(bore, r, 1.0);
    let vt = topo::mass_properties(&t, tol()).unwrap().volume;
    let exact = std::f64::consts::PI * (r * r - bore * bore);
    println!("FANREV tube volume {vt:.12} exact {exact:.12}");
    // Extrusion axis y; the region in (x, z).
    let pos = |a: f64, b: f64, c: f64| pos_part_disc(r, a, b, c) - pos_part_disc(bore, a, b, c);
    let mut normals = sphere(n);
    for k in 0..24 {
        let th = (k as f64 + 0.37) * std::f64::consts::TAU / 24.0;
        normals.push([th.cos(), th.sin(), 0.0]);
    }
    for (zname, yv) in [("top", 1.0), ("bot", 0.0)] {
        let v = [r, yv, 0.0];
        for (i, m) in normals.iter().enumerate() {
            let cube = cube_beyond(v, *m);
            // (x, z) region, axis y: permute to (p, q, axis) = (x, z, y).
            let shared = beyond(&pos, [v[0], v[2], v[1]], [m[0], m[2], m[1]], 0.0);
            let tag = format!("tube {zname} m{i}=({:.3},{:.3},{:.3})", m[0], m[1], m[2]);
            six(&tag, &t, &cube, (exact, SIDE.powi(3), shared));
        }
    }
}

/// The PR's Simpson oracle (`join_pierce_strut_facing.rs`, copied for
/// the comparison only) against the clamp oracle on its two poses and
/// on the sphere.
#[test]
fn the_prs_simpson_oracle_agrees_with_the_clamp_oracle() {
    const P: [(f64, f64); 6] = [
        (0.0, 0.0),
        (2.0, 0.0),
        (2.0, 1.0),
        (1.0, 1.0),
        (1.0, 2.0),
        (0.0, 2.0),
    ];
    let v = [1.0, 1.0, 1.0];
    let clipped_area = |a: f64, b: f64, c: f64| {
        let f = |p: (f64, f64)| a * p.0 + b * p.1 - c;
        let n = P.len();
        let mut kept = Vec::new();
        for i in 0..n {
            let (p, q) = (P[i], P[(i + 1) % n]);
            if f(p) >= 0.0 {
                kept.push(p);
            }
            if (f(p) >= 0.0) != (f(q) >= 0.0) {
                let t = f(p) / (f(p) - f(q));
                kept.push((p.0 + t * (q.0 - p.0), p.1 + t * (q.1 - p.1)));
            }
        }
        let k = kept.len();
        (0..k)
            .map(|i| kept[i].0 * kept[(i + 1) % k].1 - kept[(i + 1) % k].0 * kept[i].1)
            .sum::<f64>()
            / 2.0
    };
    let simpson = |m: [f64; 3]| -> f64 {
        let m = unit(m);
        let c = |z: f64| m[0] * v[0] + m[1] * v[1] - m[2] * (z - v[2]);
        let area = |z: f64| clipped_area(m[0], m[1], c(z));
        let mut cuts: Vec<f64> = P
            .iter()
            .map(|p| v[2] + (m[0] * (v[0] - p.0) + m[1] * (v[1] - p.1)) / m[2])
            .filter(|&z| z > 0.0 && z < 1.0)
            .chain([0.0, 1.0])
            .collect();
        cuts.sort_by(f64::total_cmp);
        cuts.windows(2)
            .map(|w| {
                (w[1] - w[0]) / 6.0 * (area(w[0]) + 4.0 * area((w[0] + w[1]) / 2.0) + area(w[1]))
            })
            .sum()
    };
    let prof = P.to_vec();
    let pos = |a: f64, b: f64, c: f64| pos_part_poly(&prof, a, b, c);
    let mut worst = 0.0f64;
    for m in [[1.0, 1.3, -0.7], [-1.0, -1.3, 0.7]]
        .into_iter()
        .chain(sphere(200))
    {
        let mu = unit(m);
        let (s, o) = (simpson(m), beyond(&pos, v, mu, 0.0));
        println!(
            "FANREV oracle m=({:.3},{:.3},{:.3}) simpson={s:.15} clamp={o:.15}",
            mu[0], mu[1], mu[2]
        );
        worst = worst.max((s - o).abs());
    }
    println!("FANREV oracle worst {worst:e}");
    assert!(worst < 1e-12, "{worst}");
}
