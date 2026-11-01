//! Two star-profile prisms sharing a vertical corner edge, one flank of
//! `b` lying along a flank line of `a`, either way round, pushed through
//! the identity, a rigid motion or a projective map (planes stay planes).
//! Where an op leaves one flank of each operand at the corner, collinear
//! and on one side, the result's two walls there lie on one plane with
//! one orientation: the boolean glues them into one face, so the corner
//! edge does not survive as scaffolding. The closed form is each convex
//! piece's image volume by the divergence theorem.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point3, Tol};
use sweep::test_support::finished;
use topo::test_support::{FaceGeometry, describe_as_intersections, prism_ops};
use topo::{AtRestBody, Body, BooleanError, BooleanResult};

use crate::common::differential::{area, clip_convex, outcome};

fn tol() -> Tol {
    Tol::witness()
}

type M = fn(f64, f64, f64) -> [f64; 3];

fn rot(p: [f64; 3]) -> [f64; 3] {
    // Rodrigues about (1, 2, 3)/√14 by 0.7 rad, then a translation.
    let s14 = 14f64.sqrt();
    let k = [1.0 / s14, 2.0 / s14, 3.0 / s14];
    let (c, s) = (0.7f64.cos(), 0.7f64.sin());
    let kx = [
        k[1] * p[2] - k[2] * p[1],
        k[2] * p[0] - k[0] * p[2],
        k[0] * p[1] - k[1] * p[0],
    ];
    let kd = k[0] * p[0] + k[1] * p[1] + k[2] * p[2];
    let r: Vec<f64> = (0..3)
        .map(|i| p[i] * c + kx[i] * s + k[i] * kd * (1.0 - c))
        .collect();
    [r[0] + 0.3, r[1] - 0.2, r[2] + 0.5]
}
fn map_rot(x: f64, y: f64, z: f64) -> [f64; 3] {
    rot([x, y, z])
}
fn map_proj(x: f64, y: f64, z: f64) -> [f64; 3] {
    let w = 1.0 + 0.06 * x - 0.04 * y + 0.09 * z;
    rot([x / w, y / w, z / w])
}
fn map_id(x: f64, y: f64, z: f64) -> [f64; 3] {
    [x, y, z]
}

/// The image volume of convex `c × [z0, z1]` under `m`.
fn image_volume(c: &[(f64, f64)], z: (f64, f64), m: M) -> f64 {
    let n = c.len();
    let bot: Vec<[f64; 3]> = c.iter().map(|&(x, y)| m(x, y, z.0)).collect();
    let top: Vec<[f64; 3]> = c.iter().map(|&(x, y)| m(x, y, z.1)).collect();
    let det = |a: [f64; 3], b: [f64; 3], d: [f64; 3]| {
        a[0] * (b[1] * d[2] - b[2] * d[1]) - a[1] * (b[0] * d[2] - b[2] * d[0])
            + a[2] * (b[0] * d[1] - b[1] * d[0])
    };
    let mut v = 0.0;
    // Top (CCW from +z, outward), bottom reversed, sides as two tris.
    for i in 1..n - 1 {
        v += det(top[0], top[i], top[i + 1]);
        v -= det(bot[0], bot[i], bot[i + 1]);
    }
    for i in 0..n {
        let j = (i + 1) % n;
        v += det(bot[i], bot[j], top[j]);
        v += det(bot[i], top[j], top[i]);
    }
    v / 6.0
}

/// An irregular star wedge from `phi` spanning `theta` degrees: the
/// fan triangles, each at most 50°, rim radii `r0` at the first flank,
/// `r1` at the last, wobbling between.
fn star(phi: f64, theta: f64, r0: f64, r1: f64) -> Vec<[(f64, f64); 3]> {
    let k = (theta / 50.0).ceil().max(1.0) as usize;
    let at = |i: usize| {
        let f = i as f64 / k as f64;
        let wob = if i == 0 || i == k {
            0.0
        } else {
            0.18 * (2.3 * i as f64 + 0.4).sin()
        };
        let r = (r0 + (r1 - r0) * f) * (1.0 + wob);
        let t = (phi + theta * f).to_radians();
        (r * t.cos(), r * t.sin())
    };
    (0..k).map(|i| [(0.0, 0.0), at(i), at(i + 1)]).collect()
}

fn outline(t: &[[(f64, f64); 3]]) -> Vec<(f64, f64)> {
    let mut p = vec![(0.0, 0.0), t[0][1]];
    p.extend(t.iter().map(|x| x[2]));
    p
}

const AZ: (f64, f64) = (0.0, 1.0);
const BZ: (f64, f64) = (-0.4, 0.7);
const OPS: [&str; 6] = ["I_ab", "I_ba", "U_ab", "U_ba", "S_ab", "S_ba"];

struct Pose {
    a: AtRestBody<f64>,
    b: AtRestBody<f64>,
    want: [f64; 6],
}

fn build(profile: &[(f64, f64)], z: (f64, f64), m: M) -> Body<f64> {
    let mut body = Body::<f64>::new();
    prism_ops(
        &mut body,
        profile,
        z,
        |x, y, zz| {
            let p = m(x, y, zz);
            Point3::new(p[0], p[1], p[2])
        },
        FaceGeometry::Certified,
        tol(),
    );
    describe_as_intersections(&mut body, tol());
    body
}

fn planar_pose(theta_a: f64, theta_b: f64, phi_b: f64, m: M) -> Pose {
    let ta = star(0.0, theta_a, 2.3, 1.4);
    let tb = star(phi_b, theta_b, 0.9, 1.6);
    let a = build(&outline(&ta), AZ, m);
    let b = build(&outline(&tb), BZ, m);
    let vol = |t: &[[(f64, f64); 3]], z| t.iter().map(|x| image_volume(x, z, m)).sum::<f64>();
    let (va, vb) = (vol(&ta, AZ), vol(&tb, BZ));
    let zc = (AZ.0.max(BZ.0), AZ.1.min(BZ.1));
    let vi: f64 = ta
        .iter()
        .flat_map(|p| tb.iter().map(move |q| (p, q)))
        .map(|(p, q)| {
            let c = clip_convex(p, q);
            if c.len() < 3 || area(&c).abs() < 1e-15 {
                0.0
            } else {
                image_volume(&c, zc, m)
            }
        })
        .sum();
    Pose {
        a: finished("a", a, tol()),
        b: finished("b", b, tol()),
        want: [vi, vi, va + vb - vi, va + vb - vi, va - vi, vb - vi],
    }
}

fn run(p: &Pose, op: &str) -> Result<BooleanResult<f64>, BooleanError> {
    let (a, b, t) = (&p.a, &p.b, tol());
    match op {
        "I_ab" => topo::intersect(a, b, t),
        "I_ba" => topo::intersect(b, a, t),
        "U_ab" => topo::union(a, b, t),
        "U_ba" => topo::union(b, a, t),
        "S_ab" => topo::subtract(a, b, t),
        _ => topo::subtract(b, a, t),
    }
}

fn caught(f: impl FnOnce() -> String) -> String {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)).unwrap_or_else(|_| "PANIC".into())
}

const MAPS: [(&str, M); 3] = [("id", map_id), ("rot", map_rot), ("proj", map_proj)];

/// The battery's poses (`θa θb φb`) where one op leaves the corner a
/// single half-turn, one flank from each operand: `b ∖ a` for the first
/// four, `a ∖ b` for the next four, `a ∪ b` for four, `a ∩ b` for six.
const HALF_TURN_POSES: [(f64, f64, f64); 18] = [
    (40.0, 200.0, 180.0),
    (40.0, 200.0, 20.0),
    (100.0, 200.0, 180.0),
    (100.0, 200.0, 80.0),
    (230.0, 170.0, 180.0),
    (230.0, 170.0, 240.0),
    (300.0, 170.0, 180.0),
    (300.0, 170.0, 310.0),
    (40.0, 170.0, 10.0),
    (40.0, 170.0, 220.0),
    (100.0, 170.0, 10.0),
    (100.0, 170.0, 280.0),
    (230.0, 200.0, -20.0),
    (230.0, 200.0, 410.0),
    (230.0, 290.0, -110.0),
    (230.0, 290.0, 410.0),
    (300.0, 200.0, -20.0),
    (300.0, 200.0, 480.0),
];

#[test]
fn a_flank_touch_glues_the_half_turn_it_leaves() {
    for (mn, m) in MAPS {
        for (theta_a, theta_b, phi_b) in HALF_TURN_POSES {
            let p = planar_pose(theta_a, theta_b, phi_b, m);
            for (op, want) in OPS.iter().zip(p.want) {
                let line = outcome(run(&p, op), want, tol());
                assert!(
                    line.starts_with("OK SOUND") || line == "EMPTY ok",
                    "{mn} {theta_a} {theta_b} {phi_b} {op}: {line}"
                );
            }
        }
    }
}

/// Every flank of `b` along or opposite each flank line of `a`. A
/// half-turn `a` (`θa = 180`) is no finished body, its corner edge
/// scaffolding between two walls on one plane, so it is not posed.
#[test]
#[ignore = "sweep battery: cargo test --release -p sweep --test all flank_touch_battery -- --ignored --nocapture"]
fn flank_touch_battery() {
    for (mn, m) in MAPS {
        for theta_a in [40.0, 100.0, 230.0, 300.0] {
            for theta_b in [25.0, 170.0, 200.0, 290.0] {
                for along in [0.0, theta_a, 180.0, theta_a + 180.0] {
                    for phi_b in [along, along - theta_b] {
                        let line = caught(|| {
                            let p = planar_pose(theta_a, theta_b, phi_b, m);
                            OPS.iter()
                                .zip(p.want)
                                .map(|(op, want)| {
                                    format!(
                                        "FT {mn} {theta_a} {theta_b} {phi_b} {op} => {}\n",
                                        caught(|| outcome(run(&p, op), want, tol()))
                                    )
                                })
                                .collect()
                        });
                        print!("{line}");
                    }
                }
            }
        }
    }
}
