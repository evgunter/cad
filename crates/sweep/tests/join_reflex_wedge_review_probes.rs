//! **Review probes for PR 3962** (edge-edge membership at a reflex
//! dihedral wedge), shaped apart from `join_reflex_wedge_probes.rs`:
//!
//! - `rv_planar_battery`: irregular star profiles (every rim vertex at
//!   its own radius, the two flanks of unequal length) sharing the
//!   corner at the origin, pushed through a rigid rotation or a
//!   projective map (planes stay planes, so a prism becomes a general
//!   polyhedron with a tilted, non-vertical common line and
//!   non-parallel side edges). The closed form is exact: each convex
//!   piece's image volume by the divergence theorem over its mapped
//!   vertices. `b` straddles `a`'s bottom (`z ∈ [−0.4, 0.7]`).
//!   Wedge angles include a half-turn exactly and near 0 and 2π.
//! - `rv_coplanar_battery`: `b`'s flank along / opposite each flank of
//!   `a`, declared and undeclared, both operand orders.
//! - `rv_curved_battery`: a keyhole whose common-edge flanks are a
//!   plane and a convex cylinder (a 270° wedge), against a fan prism;
//!   closed form from polygon ∩ disc areas.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol, Vec3};
use topo::test_support::{FaceGeometry, describe_as_intersections, flush_declarations, prism_ops};
use topo::{Body, BooleanDeclarations, BooleanError, BooleanResult};

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
/// The tall `b` z-range: `a`'s whole edge inside `b`'s, so no vertex of
/// `b` sits on `a`'s curved face.
const BZT: (f64, f64) = (-0.4, 1.4);
const OPS: [&str; 6] = ["I_ab", "I_ba", "U_ab", "U_ba", "S_ab", "S_ba"];

struct Pose {
    a: Body<f64>,
    b: Body<f64>,
    d: (BooleanDeclarations, BooleanDeclarations),
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

fn planar_pose(theta_a: f64, theta_b: f64, phi_b: f64, m: M, declare: bool) -> Pose {
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
    let d = if declare {
        (
            flush_declarations(&a, &b, tol()),
            flush_declarations(&b, &a, tol()),
        )
    } else {
        Default::default()
    };
    Pose {
        a,
        b,
        d,
        want: [vi, vi, va + vb - vi, va + vb - vi, va - vi, vb - vi],
    }
}

fn run(p: &Pose, op: &str) -> Result<BooleanResult<f64>, BooleanError> {
    let (a, b, (dab, dba), t) = (&p.a, &p.b, &p.d, tol());
    match op {
        "I_ab" => topo::intersect_with(a, b, dab, t),
        "I_ba" => topo::intersect_with(b, a, dba, t),
        "U_ab" => topo::union_with(a, b, dab, t),
        "U_ba" => topo::union_with(b, a, dba, t),
        "S_ab" => topo::subtract_with(a, b, dab, t),
        _ => topo::subtract_with(b, a, dba, t),
    }
}

fn caught(f: impl FnOnce() -> String) -> String {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)).unwrap_or_else(|_| "PANIC".into())
}

const MAPS: [(&str, M); 3] = [("id", map_id), ("rot", map_rot), ("proj", map_proj)];
const THETA_A: [f64; 9] = [2.0, 40.0, 100.0, 179.0, 180.0, 181.0, 230.0, 300.0, 358.0];
const THETA_B: [f64; 5] = [25.0, 170.0, 200.0, 290.0, 350.0];

#[test]
#[ignore = "review battery; --ignored --nocapture"]
fn rv_planar_battery() {
    for (mn, m) in MAPS {
        for theta_a in THETA_A {
            for theta_b in THETA_B {
                for k in 0..16 {
                    let phi_b = 11.0 + 23.0 * f64::from(k);
                    // Skip any flank of b within 0.5° of a flank line of a.
                    let near = [phi_b, phi_b + theta_b].iter().any(|&f| {
                        [0.0, theta_a].iter().any(|&g| {
                            let d = (f - g).rem_euclid(180.0);
                            d < 0.5 || d > 179.5
                        })
                    });
                    if near {
                        continue;
                    }
                    let line = caught(|| {
                        let p = planar_pose(theta_a, theta_b, phi_b, m, false);
                        OPS.iter()
                            .zip(p.want)
                            .map(|(op, want)| {
                                format!(
                                    "RV {mn} {theta_a} {theta_b} {phi_b} {op} => {}\n",
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

#[test]
#[ignore = "review battery; --ignored --nocapture"]
fn rv_coplanar_battery() {
    for (mn, m) in MAPS {
        for theta_a in [40.0, 100.0, 180.0, 230.0, 300.0] {
            for theta_b in [25.0, 170.0, 200.0, 290.0] {
                for along in [0.0, theta_a, 180.0, theta_a + 180.0] {
                    for phi_b in [along, along - theta_b] {
                        for declare in [true, false] {
                            let line = caught(|| {
                                let p = planar_pose(theta_a, theta_b, phi_b, m, declare);
                                OPS.iter()
                                    .zip(p.want)
                                    .map(|(op, want)| {
                                        format!("RVC {mn} {declare} {theta_a} {theta_b} {phi_b} {op} => {}\n",
                                            caught(|| outcome(run(&p, op), want, tol())))
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
}

// ---- curved flank ----

/// Area of convex CCW `poly` ∩ the disc (c, r).
fn poly_disc_area(poly: &[(f64, f64)], c: (f64, f64), r: f64) -> f64 {
    // Sum over edges of signed area of triangle(c, p, q) ∩ disc.
    let tri = |p: (f64, f64), q: (f64, f64)| -> f64 {
        let (px, py, qx, qy) = (p.0 - c.0, p.1 - c.1, q.0 - c.0, q.1 - c.1);
        let sector = |ax: f64, ay: f64, bx: f64, by: f64| {
            0.5 * r * r * (ax * by - ay * bx).atan2(ax * bx + ay * by)
        };
        let trian = |ax: f64, ay: f64, bx: f64, by: f64| 0.5 * (ax * by - ay * bx);
        let (dx, dy) = (qx - px, qy - py);
        let a = dx * dx + dy * dy;
        if a < 1e-30 {
            return 0.0;
        }
        let b = px * dx + py * dy;
        let cc = px * px + py * py - r * r;
        let disc = b * b - a * cc;
        let pin = cc <= 0.0;
        let qin = qx * qx + qy * qy <= r * r;
        if disc <= 0.0 {
            return sector(px, py, qx, qy);
        }
        let sq = disc.sqrt();
        let t1 = ((-b - sq) / a).clamp(0.0, 1.0);
        let t2 = ((-b + sq) / a).clamp(0.0, 1.0);
        let (ax, ay) = (px + t1 * dx, py + t1 * dy);
        let (bx, by) = (px + t2 * dx, py + t2 * dy);
        match (pin, qin) {
            (true, true) => trian(px, py, qx, qy),
            _ => {
                if t1 >= t2 {
                    sector(px, py, qx, qy)
                } else {
                    sector(px, py, ax, ay) + trian(ax, ay, bx, by) + sector(bx, by, qx, qy)
                }
            }
        }
    };
    let n = poly.len();
    (0..n).map(|i| tri(poly[i], poly[(i + 1) % n])).sum()
}

const KR: f64 = 0.6;

/// The keyhole: `[−2, 2] × [−2, 0]` with the upper half-disc of radius
/// `KR` centred `(−KR, 0)`; at the origin its flanks are the plane
/// `y = 0` and the cylinder, a 270° wedge.
fn keyhole() -> Vec<(Point2<f64>, f64)> {
    vec![
        (Point2::new(-2.0, -2.0), 0.0),
        (Point2::new(2.0, -2.0), 0.0),
        (Point2::new(2.0, 0.0), 0.0),
        (Point2::new(0.0, 0.0), 1.0),
        (Point2::new(-2.0 * KR, 0.0), 0.0),
        (Point2::new(-2.0, 0.0), 0.0),
    ]
}

fn curved_pose(theta_b: f64, phi_b: f64, rb: f64, tilt: bool, declare: bool) -> Pose {
    curved_pose_z(theta_b, phi_b, rb, tilt, declare, BZ)
}

fn curved_pose_z(
    theta_b: f64,
    phi_b: f64,
    rb: f64,
    tilt: bool,
    declare: bool,
    bz: (f64, f64),
) -> Pose {
    let (o, u, v) = if tilt {
        (
            Vec3::new(0.2, 0.1, -0.3),
            Vec3::new(0.8, 0.5, 0.2),
            Vec3::new(-0.3, 0.6, 0.7),
        )
    } else {
        (
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        )
    };
    let plane_at = |z0: f64| {
        let pl = sweep::test_support::sketch_from_axes(Point3::new(o.x, o.y, o.z), u, v, tol());
        let n = pl.normal();
        sweep::test_support::sketch_from_axes(
            Point3::new(o.x + n.x * z0, o.y + n.y * z0, o.z + n.z * z0),
            u,
            v,
            tol(),
        )
    };
    let tb = star(phi_b, theta_b, rb, rb);
    let bv: Vec<(Point2<f64>, f64)> = outline(&tb)
        .iter()
        .map(|&(x, y)| (Point2::new(x, y), 0.0))
        .collect();
    let a = sweep::test_support::prism_on(plane_at(AZ.0), keyhole(), AZ.1 - AZ.0, tol());
    let b = sweep::test_support::prism_on(plane_at(bz.0), bv, bz.1 - bz.0, tol());
    let half_disc = std::f64::consts::PI * KR * KR / 2.0;
    let area_a = 8.0 + half_disc;
    let area_b: f64 = tb.iter().map(|t| area(t)).sum();
    let common: f64 = tb
        .iter()
        .map(|t| {
            let low = clip_convex(t, &[(-2.0, -2.0), (2.0, -2.0), (2.0, 0.0), (-2.0, 0.0)]);
            let low = if low.len() < 3 { 0.0 } else { area(&low) };
            let up = clip_convex(t, &[(-9.0, 0.0), (9.0, 0.0), (9.0, 9.0), (-9.0, 9.0)]);
            let up = if up.len() < 3 {
                0.0
            } else {
                poly_disc_area(&up, (-KR, 0.0), KR)
            };
            low + up
        })
        .sum();
    let (ha, hb) = (AZ.1 - AZ.0, bz.1 - bz.0);
    let hc = AZ.1.min(bz.1) - AZ.0.max(bz.0);
    let (va, vb, vi) = (area_a * ha, area_b * hb, common * hc);
    let d = if declare {
        (
            flush_declarations(&a, &b, tol()),
            flush_declarations(&b, &a, tol()),
        )
    } else {
        Default::default()
    };
    Pose {
        a,
        b,
        d,
        want: [vi, vi, va + vb - vi, va + vb - vi, va - vi, vb - vi],
    }
}

#[test]
fn rv_oracles_agree() {
    // The disc oracle against a unit-disc quadrant and a full square.
    let q = poly_disc_area(
        &[(0.0, 0.0), (5.0, 0.0), (5.0, 5.0), (0.0, 5.0)],
        (0.0, 0.0),
        1.0,
    );
    assert!((q - std::f64::consts::FRAC_PI_4).abs() < 1e-12, "{q}");
    let s = poly_disc_area(
        &[(-0.1, -0.1), (0.1, -0.1), (0.1, 0.1), (-0.1, 0.1)],
        (0.0, 0.0),
        1.0,
    );
    assert!((s - 0.04).abs() < 1e-12, "{s}");
    let h = poly_disc_area(
        &[(-0.6, 0.0), (5.0, 0.0), (5.0, 5.0), (-0.6, 5.0)],
        (-0.6, 0.0),
        0.6,
    );
    assert!((h - std::f64::consts::PI * 0.09).abs() < 1e-12, "{h}");
    // The image volume of a unit cube under the identity.
    let c = image_volume(
        &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
        (0.0, 1.0),
        map_id,
    );
    assert!((c - 1.0).abs() < 1e-12, "{c}");
    let r = image_volume(
        &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
        (0.0, 1.0),
        map_rot,
    );
    assert!((r - 1.0).abs() < 1e-12, "{r}");
    // The projective oracle against the kernel on one operand alone.
    for m in [map_rot as M, map_proj] {
        let t = star(0.0, 300.0, 2.3, 1.4);
        let b = build(&outline(&t), AZ, m);
        let want: f64 = t.iter().map(|x| image_volume(x, AZ, m)).sum();
        let v = topo::mass_properties(&b, tol()).unwrap().volume;
        assert!((v - want).abs() < 1e-9, "{v} {want}");
    }
    let p = curved_pose(30.0, 200.0, 0.8, true, false);
    let v = topo::mass_properties(&p.a, tol()).unwrap().volume;
    assert!(
        (v - (8.0 + std::f64::consts::PI * KR * KR / 2.0)).abs() < 1e-9,
        "keyhole {v}"
    );
}

#[test]
#[ignore = "review battery; --ignored --nocapture"]
fn rv_curved_battery() {
    for tilt in [false, true] {
        for rb in [0.4, 1.0] {
            for theta_b in [25.0, 80.0, 170.0, 200.0, 290.0] {
                for k in 0..24 {
                    let phi_b = 15.0 * f64::from(k) + 3.0;
                    let declare = false;
                    let line = caught(|| {
                        let p = curved_pose(theta_b, phi_b, rb, tilt, declare);
                        OPS.iter()
                            .zip(p.want)
                            .map(|(op, want)| {
                                format!(
                                    "RVK {tilt} {rb} {theta_b} {phi_b} {op} => {}\n",
                                    caught(|| outcome(run(&p, op), want, tol()))
                                )
                            })
                            .collect()
                    });
                    print!("{line}");
                }
            }
            // b's flank along the plane (0°, 180°) or the cylinder's
            // tangent (90°), declared.
            for theta_b in [80.0, 200.0] {
                for phi_b in [0.0, -theta_b, 180.0, 180.0 - theta_b, 90.0, 90.0 - theta_b] {
                    let line = caught(|| {
                        let p = curved_pose(theta_b, phi_b, rb, tilt, true);
                        OPS.iter()
                            .zip(p.want)
                            .map(|(op, want)| {
                                format!(
                                    "RVKC {tilt} {rb} {theta_b} {phi_b} {op} => {}\n",
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

// ---- a curved flank's chord against its tangent ----

/// `a`: a plane flank (`y = 0`, from `(−2, 0)` to the origin) and a
/// cylinder flank leaving the origin at tangent heading `t0` degrees,
/// sweeping `sweep` degrees (negative: concave, curving right), radius 1.
/// The wedge by the tangent is `180 − t0`; the arc's chord heads
/// `t0 + sweep/2`.
fn chord_a(t0: f64, sweep: f64) -> (Vec<(Point2<f64>, f64)>, f64) {
    let ch = (t0 + sweep / 2.0).to_radians();
    let len = 2.0 * (sweep.abs() / 2.0).to_radians().sin();
    let q = (len * ch.cos(), len * ch.sin());
    let b = (sweep.to_radians() / 4.0).tan();
    let verts = vec![
        (Point2::new(-2.0, 0.0), 0.0),
        (Point2::new(0.0, 0.0), b),
        (Point2::new(q.0, q.1), 0.0),
        (Point2::new(2.5, q.1), 0.0),
        (Point2::new(2.5, 2.0), 0.0),
        (Point2::new(-2.0, 2.0), 0.0),
    ];
    let poly = [
        (-2.0, 0.0),
        (0.0, 0.0),
        q,
        (2.5, q.1),
        (2.5, 2.0),
        (-2.0, 2.0),
    ];
    let th = sweep.abs().to_radians();
    let seg = 0.5 * (th - th.sin());
    // A concave arc bows into the interior, a convex one out of it.
    let ar = area(&poly) + if sweep > 0.0 { seg } else { -seg };
    (verts, ar)
}

/// `b` a small fan at the origin, wholly in `y > 0` and inside radius
/// 0.3, where `a`'s arc stays above `y = 0`; its overlap with `a` is
/// the fan less the arc's disc (concave) or the fan within it (convex).
fn chord_pose(t0: f64, sweep: f64, phi_b: f64, theta_b: f64, bz: (f64, f64)) -> Pose {
    let (av, area_a) = chord_a(t0, sweep);
    let tb = star(phi_b, theta_b, 0.3, 0.3);
    let bv: Vec<(Point2<f64>, f64)> = outline(&tb)
        .iter()
        .map(|&(x, y)| (Point2::new(x, y), 0.0))
        .collect();
    let a = sweep::test_support::prism_at(av, AZ.0, AZ.1 - AZ.0, tol());
    let b = sweep::test_support::prism_at(bv, bz.0, bz.1 - bz.0, tol());
    // The disc: centre one unit from the origin, left of the tangent
    // for a convex arc, right for a concave one.
    let side = if sweep > 0.0 { 90.0 } else { -90.0 };
    let c = (
        (t0 + side).to_radians().cos(),
        (t0 + side).to_radians().sin(),
    );
    let area_b: f64 = tb.iter().map(|t| area(t)).sum();
    let in_disc: f64 = tb.iter().map(|t| poly_disc_area(t, c, 1.0)).sum();
    let common = if sweep > 0.0 {
        // y > 0 is a's except outside... only the disc part below the
        // chord matters; b sits in y > 0, all a's there but the bite.
        area_b
    } else {
        area_b - in_disc
    };
    let hc = AZ.1.min(bz.1) - AZ.0.max(bz.0);
    let (va, vb, vi) = (area_a * (AZ.1 - AZ.0), area_b * (bz.1 - bz.0), common * hc);
    let va_k = topo::mass_properties(&a, tol()).unwrap().volume;
    assert!((va - va_k).abs() < 1e-9, "a's closed form {va} vs {va_k}");
    Pose {
        a,
        b,
        d: Default::default(),
        want: [vi, vi, va + vb - vi, va + vb - vi, va - vi, vb - vi],
    }
}

#[test]
#[ignore = "review battery; --ignored --nocapture"]
fn rv_chord_battery() {
    // Concave flank at a convex wedge (160°, 170°, 175°), chord reading
    // reflex; and convex flank at a reflex wedge (200°), chord convex.
    for (t0, sweep) in [
        (20.0, -60.0),
        (10.0, -60.0),
        (5.0, -40.0),
        (20.0, -30.0),
        (-20.0, 60.0),
    ] {
        for bz in [BZ, BZT] {
            for phi_b in [2.0, 5.0, 10.0, 15.0, 25.0, 40.0] {
                for theta_b in [20.0, 60.0, 120.0] {
                    if phi_b + theta_b > 175.0 {
                        continue;
                    }
                    let line = caught(|| {
                        let p = chord_pose(t0, sweep, phi_b, theta_b, bz);
                        OPS.iter()
                            .zip(p.want)
                            .map(|(op, want)| {
                                format!(
                                    "RVH {} {t0} {sweep} {phi_b} {theta_b} {op} => {}\n",
                                    bz.1,
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

#[test]
#[ignore = "review battery; --ignored --nocapture"]
fn rv_curved_tall_battery() {
    for tilt in [false, true] {
        for rb in [0.4, 1.0] {
            for theta_b in [25.0, 80.0, 170.0, 200.0, 290.0] {
                for k in 0..24 {
                    let phi_b = 15.0 * f64::from(k) + 3.0;
                    let line = caught(|| {
                        let p = curved_pose_z(theta_b, phi_b, rb, tilt, false, BZT);
                        OPS.iter()
                            .zip(p.want)
                            .map(|(op, want)| {
                                format!(
                                    "RVKT {tilt} {rb} {theta_b} {phi_b} {op} => {}\n",
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
