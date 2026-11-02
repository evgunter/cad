//! R1 review probe (not for merge).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::*;
use crate::link::{FaceSector, TangentFan};
use geom_core::{Tol, Vec3};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn ball(center: Point3<f64>, r: f64, axis: Vec3<f64>) -> geom::Surface<f64> {
    let u = axis.cross(Vec3::new(0.3, -0.7, 0.2)).normalize();
    geom::Surface::Sphere {
        center,
        radius: r,
        axis,
        u_ref: u,
    }
}

fn whole(
    s: geom::Surface<f64>,
    outward: Vec3<f64>,
    p: Point3<f64>,
    along: Vec3<f64>,
) -> FaceSector<f64> {
    FaceSector::tangent(
        s,
        true,
        outward,
        p,
        TangentFan::Inside { reach: 1.0, along },
        band(),
    )
    .unwrap()
}

/// Two balls kissing at a point that is (to rounding) each one's pole on
/// a tilted axis: the tangent planes are one plane, so first order must
/// refuse (Unreadable). Report what `around` gives and the verdict.
#[test]
fn r1_kissing_balls_at_a_tilted_pole() {
    let mut bad = Vec::new();
    let raws = [
        [1.0, 2.0, 3.0],
        [0.3, -0.9, 0.1],
        [-2.0, 0.5, 7.0],
        [1.0, 1.0, 1.0],
        [0.1, 0.2, -0.97],
    ];
    for (i, raw) in raws.iter().enumerate() {
        let a = Vec3::new(raw[0], raw[1], raw[2]).normalize();
        for far in [0.0, 1e3] {
            let p = Point3::new(0.3 + far, -0.2 + 2.0 * far, 0.1);
            let r = 0.7;
            for socket in [false, true] {
                let ca = p - a * r;
                let (cb, rb) = if socket {
                    (p - a * (2.0 * r), 2.0 * r)
                } else {
                    (p + a * r, r)
                };
                let sa = ball(ca, r, a);
                let sb = ball(cb, rb, a);
                let na = (p - ca).normalize();
                // A ball's outward is away from its centre; a socket's toward it.
                let nb = if socket {
                    -(p - cb).normalize()
                } else {
                    (p - cb).normalize()
                };
                let (Some(ua), Some(ub)) = (around(&sa, p), around(&sb, p)) else {
                    eprintln!("case {i} far {far} socket {socket}: around None");
                    continue;
                };
                let tilt = (ua.dot(na).abs(), ub.dot(nb).abs());
                let link =
                    Link::of(p, vec![whole(sa, na, p, ua), whole(sb, nb, p, ub)]).unwrap();
                let v = rest::one_material(&link, band());
                eprintln!("case {i} far {far} socket {socket}: tilt {tilt:?} -> {v:?}");
                if v != TouchVerdict::Unread(TouchUnread::Unreadable) {
                    bad.push(format!(
                        "case {i} far {far} socket {socket}: tilt {tilt:?} -> {v:?}"
                    ));
                }
            }
        }
    }
    assert!(bad.is_empty(), "{bad:#?}");
}

fn flat(n: Vec3<f64>, p: Point3<f64>) -> geom::Surface<f64> {
    geom::Surface::Plane {
        origin: p,
        normal: n,
        u_ref: n.cross(Vec3::new(0.3, -0.7, 0.2)).normalize(),
    }
}

/// A ball at its tilted pole against a whole planar face: on the floor
/// (one plane; first order cannot decide) and crossing it (a plane
/// tilted 0.3 rad off the tangent plane through the pole: Crossing).
#[test]
fn r1_ball_pole_against_a_floor_and_a_crossing_plane() {
    let mut bad = Vec::new();
    let raws = [
        [1.0, 2.0, 3.0],
        [0.3, -0.9, 0.1],
        [-2.0, 0.5, 7.0],
        [0.1, 0.2, -0.97],
    ];
    for (i, raw) in raws.iter().enumerate() {
        let a = Vec3::new(raw[0], raw[1], raw[2]).normalize();
        let side = a.cross(Vec3::new(0.3, -0.7, 0.2)).normalize();
        let p = Point3::new(0.3, -0.2, 0.1);
        let r = 0.7;
        // The ball sits above the floor: centre along +a, outward at p is -a.
        let cb = p + a * r;
        let sb = ball(cb, r, a);
        let nb = (p - cb).normalize();
        let Some(ub) = around(&sb, p) else {
            eprintln!("case {i}: around None");
            continue;
        };
        for (what, n, want_cross) in [
            ("floor", a, false),
            (
                "tilted",
                (a * 0.3f64.cos() + side * 0.3f64.sin()).normalize(),
                true,
            ),
        ] {
            let fl = flat(n, p);
            let uf = n.cross(side).normalize();
            for order in [false, true] {
                let mut v = vec![whole(sb.clone(), nb, p, ub), whole(fl.clone(), n, p, uf)];
                if order {
                    v.reverse();
                }
                let link = Link::of(p, v).unwrap();
                let got = rest::one_material(&link, band());
                eprintln!(
                    "case {i} {what} order {order}: tilt {} -> {got:?}",
                    ub.dot(nb).abs()
                );
                let ok = if want_cross {
                    got == TouchVerdict::Crossing
                } else {
                    !matches!(got, TouchVerdict::Rest(_) | TouchVerdict::Crossing)
                };
                if !ok {
                    bad.push(format!("case {i} {what} order {order}: {got:?}"));
                }
            }
        }
    }
    assert!(bad.is_empty(), "{bad:#?}");
}
