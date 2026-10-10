//! Design-fork probe, round 2: the three spellings under REALISTIC input
//! widths — a rotated placement (the point pays the placement's
//! rounding), an axis that came through a transform (ulps), and both —
//! against the synthetic wide axis. Prints tables; asserts nothing.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::TAU;

use crate::shared::interval::iv;
use geom_brep::{MappedCurve, SweepRange};
use geom_core::{Affine3, Bounds, Interval, Mat3, Point2, Point3, Real, Vec3};

fn width(e: Interval) -> f64 {
    e.hi() - e.lo()
}
fn pw(p: Point3<Interval>) -> f64 {
    width(p.x).max(width(p.y)).max(width(p.z))
}

#[derive(Clone, Copy)]
enum Spelling {
    Shipped,
    AtPoint,
    AtAxis,
}

fn turn<T: Real>(sp: Spelling, p: Point3<T>, q: Point3<T>, n: Vec3<T>, theta: T) -> Point3<T> {
    match sp {
        Spelling::Shipped => Affine3::rotation_about_axis(q, n, theta).transform_point(p),
        Spelling::AtPoint => p - Mat3::identity_minus_rotation_about(n, theta) * (p - q),
        Spelling::AtAxis => q + Mat3::rotation_about(n, theta) * (p - q),
    }
}

fn parts(c: &MappedCurve<Interval>) -> (Point3<Interval>, Point3<Interval>, Vec3<Interval>, Interval, SweepRange<Interval>) {
    let MappedCurve::RevolvedPoint {
        point,
        place,
        axis_origin,
        axis_dir,
        angle,
        range,
    } = *c
    else {
        panic!("a revolved point")
    };
    let p = place.transform_point(Point3::new(point.x, point.y, iv(0.0)));
    (p, axis_origin, axis_dir, angle, range)
}

fn eval_as(sp: Spelling, c: &MappedCurve<Interval>, s: f64) -> Point3<Interval> {
    let (p, q, n, angle, range) = parts(c);
    turn(sp, p, q, n, range.at(iv(s)) * angle)
}

/// A rim whose placement is a ROTATED frame (tilted 0.7 rad about a
/// skew axis, then translated to `at`), so the placed point pays the
/// placement's rounding, as a sweep's rim does. The axis is the
/// placement's image of the sketch-plane axis, computed the same way.
fn tilted_rim(at: [f64; 3], axis_half: f64) -> MappedCurve<Interval> {
    let [x, y, z] = at;
    let tilt = Affine3::rotation_about_axis(
        Point3::origin(),
        Vec3::new(iv(0.2), iv(1.0), iv(-0.4)),
        iv(0.7),
    );
    let place = Affine3::translation(Vec3::new(iv(x), iv(y), iv(z))) * tilt;
    let w = |c: Interval| Interval::from_bounds(c.lo() - axis_half, c.hi() + axis_half);
    let q = place.transform_point(Point3::new(iv(1.0), iv(2.0), iv(0.0)));
    MappedCurve::RevolvedPoint {
        point: Point2::new(iv(2.0), iv(2.0)),
        place,
        axis_origin: Point3::new(w(q.x), w(q.y), w(q.z)),
        axis_dir: place.transform_vec(Vec3::new(iv(0.0), iv(0.0), iv(1.0))),
        angle: iv(TAU),
        range: SweepRange::whole(),
    }
}

/// The exact-input rim (translation placement, exact axis), the
/// fixture `revolved_point_anchor.rs` drives.
fn flat_rim(at: [f64; 3], axis_half: f64) -> MappedCurve<Interval> {
    let [x, y, z] = at;
    let w = |c: f64| Interval::from_bounds(c - axis_half, c + axis_half);
    MappedCurve::RevolvedPoint {
        point: Point2::new(iv(2.0), iv(2.0)),
        place: Affine3::translation(Vec3::new(iv(x), iv(y), iv(z))),
        axis_origin: Point3::new(w(1.0 + x), w(2.0 + y), w(z)),
        axis_dir: Vec3::new(iv(0.0), iv(0.0), iv(1.0)),
        angle: iv(TAU),
        range: SweepRange::whole(),
    }
}

fn chain(k: usize) -> (Interval, Interval) {
    #[allow(clippy::cast_precision_loss)]
    let a = 0.37 + 0.011 * ((k * 7) % 5) as f64;
    if k % 2 == 0 { (iv(0.3), iv(0.7)) } else { (iv(a), iv(1.0)) }
}

#[test]
fn probe_realistic_widths() {
    const NEAR: [f64; 3] = [0.0, 0.0, 3.0];
    const FAR: [f64; 3] = [1000.0, -700.0, 300.0];
    println!("\n== Interval widths per sample; shipped / at-point / at-axis");
    for (label, mk) in [
        ("flat, exact axis        ", (|at| flat_rim(at, 0.0)) as fn([f64; 3]) -> MappedCurve<Interval>),
        ("tilted, axis via place  ", |at| tilted_rim(at, 0.0)),
        ("flat, axis ±1e-9 (synth)", |at| flat_rim(at, 1e-9)),
        ("tilted, axis ±1e-9      ", |at| tilted_rim(at, 1e-9)),
    ] {
        for (where_, at) in [("near", NEAR), ("far ", FAR)] {
            let c = mk(at);
            let (p, q, ..) = parts(&c);
            let mut row = format!("{label} {where_} w(p)={:.1e} w(q)={:.1e}", pw(p), pw(q));
            for s in [0.0, 0.25, 0.5, 1.0] {
                row += &format!(
                    " | s={s}: {:.1e} / {:.1e} / {:.1e}",
                    pw(eval_as(Spelling::Shipped, &c, s)),
                    pw(eval_as(Spelling::AtPoint, &c, s)),
                    pw(eval_as(Spelling::AtAxis, &c, s))
                );
            }
            // 64 alternating inexact splits, widest of s = 0, 1/2, 1.
            let mut r = c;
            for k in 0..64 {
                let (s0, s1) = chain(k);
                r = r.restrict(s0, s1);
            }
            let worst = |sp| {
                [0.0, 0.5, 1.0]
                    .into_iter()
                    .map(|s| pw(eval_as(sp, &r, s)))
                    .fold(0.0, f64::max)
            };
            row += &format!(
                " | N=64: {:.1e} / {:.1e} / {:.1e}",
                worst(Spelling::Shipped),
                worst(Spelling::AtPoint),
                worst(Spelling::AtAxis)
            );
            println!("{row}");
        }
    }
}

/// f64 samples for an exact reference computed outside: prints the
/// inputs and each spelling's output at full precision, for the
/// mpmath script to score.
#[test]
fn probe_f64_samples_for_exact_reference() {
    let raw = Vec3::new(0.3, 0.2, 1.0);
    let n = raw * (1.0 / raw.norm());
    let x = Vec3::new(1.0, 0.0, 0.0);
    let u0 = x - n * n.dot(x);
    let u_ref = u0 * (1.0 / u0.norm());
    let v_ref = n.cross(u_ref);
    let cases: [(&str, Point3<f64>, f64, f64); 5] = [
        ("origin R=2.3e6 half", Point3::origin(), 2.3e6, core::f64::consts::PI),
        ("origin R=2.3e6 full", Point3::origin(), 2.3e6, TAU),
        ("origin R=1e3 half", Point3::origin(), 1.0e3, core::f64::consts::PI),
        ("far 1e3 R=1 full", Point3::new(1000.0, -700.0, 300.0), 1.0, TAU),
        ("far 1e5 R=1 full", Point3::new(1.0e5, 3.0e4, -2.0e4), 1.0, TAU),
    ];
    println!("\nEXACTREF n {:e} {:e} {:e}", n.x, n.y, n.z);
    for (label, q, radius, angle) in cases {
        for az in [0.0_f64, 0.5, 1.0, 2.8] {
            let p = q + (u_ref * az.cos() + v_ref * az.sin()) * radius;
            for i in 0..17 {
                let theta = angle * f64::from(i) / 16.0;
                let a = turn(Spelling::Shipped, p, q, n, theta);
                let b = turn(Spelling::AtPoint, p, q, n, theta);
                let c = turn(Spelling::AtAxis, p, q, n, theta);
                println!(
                    "EXACTREF {label}|{az}|{:e} {:e} {:e}|{:e} {:e} {:e}|{theta:e}|{:e} {:e} {:e}|{:e} {:e} {:e}|{:e} {:e} {:e}",
                    p.x, p.y, p.z, q.x, q.y, q.z, a.x, a.y, a.z, b.x, b.y, b.z, c.x, c.y, c.z
                );
            }
        }
    }
}
