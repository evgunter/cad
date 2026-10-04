//! Review lane r2 of PR #4008 (the join's nearest partner by germ arm,
//! then chord): differential probes. `#[ignore]`d; each prints one line
//! per pose so two trees can be diffed.
//!
//! - `r2_steep_ellipse_battery`: a thin plate `x ∈ [x1, x2]`,
//!   `y ∈ [−2, 2]`, `z ∈ [0, t]` pierced by a round rod whose axis is
//!   tilted `θ` from `z` about `y`, so the plate's caps cut the rod's
//!   wall in ellipses of aspect `1/cos θ`. At `θ ≥ 45°` the chord from
//!   a site near a minor end is NOT monotone along the half-turn the
//!   germ runs into, which is the regime `j3r2_tilted_battery`
//!   (`θ ≤ 0.3`) never reaches. Oracle: the slab's slices are ellipses
//!   against the plate's rectangle, i.e. (scaled by `cos θ` along `x`)
//!   a disc against a rectangle, integrated in `z` by Simpson.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::Body;

use crate::common::differential::outcome;

fn tol() -> Tol {
    Tol::witness()
}

const R: f64 = 0.5;

fn plate(x: (f64, f64), t: f64) -> Body<f64> {
    let lp = bulge_loop(
        [(x.0, -2.0), (x.1, -2.0), (x.1, 2.0), (x.0, 2.0)]
            .into_iter()
            .map(|(a, b)| (Point2::new(a, b), 0.0))
            .collect(),
    );
    let p = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol())
        .unwrap();
    extrude(
        &p,
        Extrusion::Distance {
            depth: t,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body
}

/// The rod: a disc of radius `R` (its two vertices at sketch angle
/// `phi`), on a plane tilted `theta` about `y`, centred so its axis
/// meets `(0, 0, t/2)`, of length `len`.
fn rod(theta: f64, phi: f64, t: f64, len: f64) -> Body<f64> {
    let (s, c) = phi.sin_cos();
    let lp = bulge_loop(vec![
        (Point2::new(R * c, R * s), 1.0),
        (Point2::new(-R * c, -R * s), 1.0),
    ]);
    let rot = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 1.0, 0.0), theta);
    let n = rot.transform_vec(Vec3::new(0.0, 0.0, 1.0));
    let p0 = Point3::new(0.0, 0.0, t / 2.0) - n * (len / 2.0);
    let place = Affine3::translation(p0 - Point3::origin()) * rot;
    let p = Profile::new(SketchPlane::new(place), vec![lp])
        .validate(tol())
        .unwrap();
    extrude(
        &p,
        Extrusion::Distance {
            depth: len,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body
}

fn sector(r: f64, a: (f64, f64), b: (f64, f64)) -> f64 {
    let cross = a.0 * b.1 - a.1 * b.0;
    let dot = a.0 * b.0 + a.1 * b.1;
    r * r * cross.atan2(dot) / 2.0
}

/// Disc of radius `r` about the origin against a CCW polygon (Green).
fn disc_clip_area(r: f64, poly: &[(f64, f64)]) -> f64 {
    let n = poly.len();
    let mut total = 0.0;
    for i in 0..n {
        let (a, b) = (poly[i], poly[(i + 1) % n]);
        let d = (b.0 - a.0, b.1 - a.1);
        let (qa, qb, qc) = (
            d.0 * d.0 + d.1 * d.1,
            2.0 * (a.0 * d.0 + a.1 * d.1),
            a.0 * a.0 + a.1 * a.1 - r * r,
        );
        let disc = qb * qb - 4.0 * qa * qc;
        let hits: Vec<f64> = if disc <= 0.0 {
            Vec::new()
        } else {
            let root = disc.sqrt();
            [(-qb - root) / (2.0 * qa), (-qb + root) / (2.0 * qa)]
                .into_iter()
                .filter(|t| *t > 0.0 && *t < 1.0)
                .collect()
        };
        let at = |t: f64| (a.0 + t * d.0, a.1 + t * d.1);
        let mut from = a;
        let mut inside = a.0.hypot(a.1) <= r;
        let piece = |p: (f64, f64), q: (f64, f64), inside: bool| {
            if inside {
                (p.0 * q.1 - p.1 * q.0) / 2.0
            } else {
                sector(r, p, q)
            }
        };
        for t in hits {
            let p = at(t);
            total += piece(from, p, inside);
            from = p;
            inside = !inside;
        }
        total += piece(from, b, inside);
    }
    total
}

/// The rod ∩ plate volume: slices are ellipses (semi-axes `R/cos θ`
/// along `x`, `R` along `y`) centred at `x = (z − t/2)·tan θ`.
fn want_i(theta: f64, x: (f64, f64), t: f64) -> f64 {
    let (c, tn) = (theta.cos(), theta.tan());
    let slice = |z: f64| {
        let xc = (z - t / 2.0) * tn;
        let (a, b) = ((x.0 - xc) * c, (x.1 - xc) * c);
        disc_clip_area(R, &[(a, -2.0), (b, -2.0), (b, 2.0), (a, 2.0)]) / c
    };
    let n = 4000;
    let h = t / n as f64;
    let mut s = slice(0.0) + slice(t);
    for k in 1..n {
        s += slice(k as f64 * h) * if k % 2 == 1 { 4.0 } else { 2.0 };
    }
    s * h / 3.0
}

#[test]
fn r2_the_steep_ellipse_oracle_holds_at_a_plate_wider_than_the_rod() {
    // A plate the rod crosses whole: the slab's volume is the oblique
    // prism's, `π R² t / cos θ`.
    let (theta, t) = (1.2, 0.05);
    let w = want_i(theta, (-3.0, 3.0), t);
    let closed = core::f64::consts::PI * R * R * t / theta.cos();
    assert!((w - closed).abs() < 1e-10, "{w} vs {closed}");
}

#[test]
#[ignore = "differential battery; run with --ignored"]
fn r2_steep_ellipse_battery() {
    for theta_deg in [30.0f64, 50.0, 60.0, 70.0, 78.0] {
        let theta = theta_deg.to_radians();
        let a_major = R / theta.cos();
        for t in [0.02, 0.1] {
            for x2f in [-0.3, -0.1, 0.1] {
                for x1f in [-0.6, -0.8, -0.95] {
                    let x = (x1f * a_major, x2f * R);
                    for phi in [0.0, core::f64::consts::FRAC_PI_2, 0.4] {
                        for mirror in [false, true] {
                            let xs = if mirror { (-x.1, -x.0) } else { x };
                            let pl = plate(xs, t);
                            let len = 2.0 * ((R + t) / theta.cos() + 1.0);
                            let rd = rod(theta, phi, t, len);
                            let va = (xs.1 - xs.0) * 4.0 * t;
                            let vb = topo::mass_properties(&rd, tol()).unwrap().volume;
                            let vi = want_i(theta, xs, t);
                            let tag = format!(
                                "R2E th={theta_deg} t={t} x=({:.4},{:.4}) phi={phi:.2} m={mirror}",
                                xs.0, xs.1
                            );
                            for (op, w_ab, w_ba) in [
                                ("U", va + vb - vi, va + vb - vi),
                                ("S", va - vi, vb - vi),
                                ("I", vi, vi),
                            ] {
                                for (order, want) in [("AB", w_ab), ("BA", w_ba)] {
                                    let (l, r) = if order == "AB" {
                                        (&pl, &rd)
                                    } else {
                                        (&rd, &pl)
                                    };
                                    let got = match op {
                                        "U" => topo::union(l, r, tol()),
                                        "S" => topo::subtract(l, r, tol()),
                                        _ => topo::intersect(l, r, tol()),
                                    };
                                    println!("{tag} {op} {order} => {}", outcome(got, want, tol()));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// A U plate (`[−1, 1]² × [0, t]` less the slot `|x − c| < w`, `y > −0.2`,
/// open at `y = 1`) and an upright rod about the origin: the open row
/// `join-ranks-conic-facing-germs-by-chord`'s fixture shape, a circle cut
/// into two arcs on ONE face pair by a slot narrower than the prongs.
fn u_plate(c: f64, w: f64, t: f64) -> (Body<f64>, Vec<(f64, f64)>) {
    let poly = vec![
        (-1.0, -1.0),
        (1.0, -1.0),
        (1.0, 1.0),
        (c + w, 1.0),
        (c + w, -0.2),
        (c - w, -0.2),
        (c - w, 1.0),
        (-1.0, 1.0),
    ];
    let lp = bulge_loop(
        poly.iter()
            .map(|&(a, b)| (Point2::new(a, b), 0.0))
            .collect(),
    );
    let p = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol())
        .unwrap();
    let body = extrude(
        &p,
        Extrusion::Distance {
            depth: t,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body;
    (body, poly)
}

#[test]
#[ignore = "differential battery; run with --ignored"]
fn r2_u_plate_battery() {
    let t = 0.1;
    for c in [0.0, 0.1, 0.25, -0.3] {
        for w in [0.03, 0.12] {
            for phi in [0.0, core::f64::consts::FRAC_PI_2, 0.4, 2.0] {
                let (pl, poly) = u_plate(c, w, t);
                let rd = rod(0.0, phi, t, 3.0);
                let va = crate::common::differential::area(&poly) * t;
                let vb = topo::mass_properties(&rd, tol()).unwrap().volume;
                let vi = disc_clip_area(R, &poly) * t;
                let tag = format!("R2U c={c} w={w} phi={phi:.2}");
                for (op, w_ab, w_ba) in [
                    ("U", va + vb - vi, va + vb - vi),
                    ("S", va - vi, vb - vi),
                    ("I", vi, vi),
                ] {
                    for (order, want) in [("AB", w_ab), ("BA", w_ba)] {
                        let (l, r) = if order == "AB" {
                            (&pl, &rd)
                        } else {
                            (&rd, &pl)
                        };
                        let got = match op {
                            "U" => topo::union(l, r, tol()),
                            "S" => topo::subtract(l, r, tol()),
                            _ => topo::intersect(l, r, tol()),
                        };
                        println!("{tag} {op} {order} => {}", outcome(got, want, tol()));
                    }
                }
            }
        }
    }
}
