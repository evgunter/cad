//! Design-fork probe (not a regression row): the spellings of a
//! revolved point's evaluation and of a restricted sketch segment,
//! measured side by side. Prints tables; asserts nothing but sanity.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI, TAU};

use crate::shared::interval::iv;
use geom::Curve3;
use geom_brep::{MappedCurve, SketchSegment, SweepRange};
use geom_core::{Affine3, Arc2, Bounds, Interval, Mat3, Point2, Point3, Real, Vec3};

fn width(e: Interval) -> f64 {
    e.hi() - e.lo()
}
fn pw(p: Point3<Interval>) -> f64 {
    width(p.x).max(width(p.y)).max(width(p.z))
}
fn pw2(p: Point2<Interval>) -> f64 {
    width(p.x).max(width(p.y))
}

/// The three spellings of "p turned by theta about the axis (q, n)".
#[derive(Clone, Copy)]
enum Spelling {
    /// `R·p + (I − R)·q` — the shipped affine map applied to the point.
    Shipped,
    /// `p − (I − R)·(p − q)` — anchored at the point.
    AtPoint,
    /// `q + R·(p − q)` — anchored at the axis.
    AtAxis,
}

fn turn<T: Real>(sp: Spelling, p: Point3<T>, q: Point3<T>, n: Vec3<T>, theta: T) -> Point3<T> {
    match sp {
        Spelling::Shipped => Affine3::rotation_about_axis(q, n, theta).transform_point(p),
        Spelling::AtPoint => p - Mat3::identity_minus_rotation_about(n, theta) * (p - q),
        Spelling::AtAxis => q + Mat3::rotation_about(n, theta) * (p - q),
    }
}

fn parts<T: Real>(c: &MappedCurve<T>) -> (Point3<T>, Point3<T>, Vec3<T>, T, SweepRange<T>) {
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
    let p = place.transform_point(Point3::new(point.x, point.y, T::zero()));
    (p, axis_origin, axis_dir, angle, range)
}

fn eval_as<T: Real>(sp: Spelling, c: &MappedCurve<T>, s: T) -> Point3<T> {
    let (p, q, n, angle, range) = parts(c);
    turn(sp, p, q, n, range.at(s) * angle)
}

fn rim_at(at: [f64; 3], half: f64) -> MappedCurve<Interval> {
    let [x, y, z] = at;
    let w = |c: f64| Interval::from_bounds(c - half, c + half);
    MappedCurve::RevolvedPoint {
        point: Point2::new(iv(2.0), iv(2.0)),
        place: Affine3::translation(Vec3::new(iv(x), iv(y), iv(z))),
        axis_origin: Point3::new(w(1.0 + x), w(2.0 + y), w(z)),
        axis_dir: Vec3::new(iv(0.0), iv(0.0), iv(1.0)),
        angle: iv(TAU),
        range: SweepRange::whole(),
    }
}

fn sampled(sp: Spelling, c: &MappedCurve<Interval>) -> f64 {
    [0.0, 0.5, 1.0]
        .into_iter()
        .map(|s| pw(eval_as(sp, c, iv(s))))
        .fold(0.0, f64::max)
}

fn crossing(k: usize) -> f64 {
    #[allow(clippy::cast_precision_loss)]
    let wobble = ((k * 7) % 5) as f64;
    0.37 + 0.011 * wobble
}

fn chain(name: &str, k: usize) -> (Interval, Interval) {
    let a = crossing(k);
    match name {
        "(0,1/2)" => (iv(0.0), iv(0.5)),
        "(1/2,1)" => (iv(0.5), iv(1.0)),
        "(0.3,0.7)" => (iv(0.3), iv(0.7)),
        "(a,1)" => (iv(a), iv(1.0)),
        "(1/4,3/4)" => (iv(0.25), iv(0.75)),
        _ => unreachable!(),
    }
}

/// Interval widths of the three spellings along split chains, near and
/// a thousand metres out.
#[test]
fn probe_interval_widths_by_spelling() {
    const NEAR: [f64; 3] = [0.0, 0.0, 3.0];
    const FAR: [f64; 3] = [1000.0, -700.0, 300.0];
    println!("\n== Interval widths (widest of s=0,1/2,1), N = splits; shipped / at-point / at-axis");
    for (label, at) in [("near", NEAR), ("far ", FAR)] {
        for name in ["(0,1/2)", "(1/2,1)", "(1/4,3/4)", "(0.3,0.7)", "(a,1)"] {
            let mut c = rim_at(at, 0.0);
            let mut row = String::new();
            for n in 0..=64usize {
                if [0, 1, 4, 16, 64].contains(&n) {
                    row += &format!(
                        " | N={n}: {:.2e} / {:.2e} / {:.2e}",
                        sampled(Spelling::Shipped, &c),
                        sampled(Spelling::AtPoint, &c),
                        sampled(Spelling::AtAxis, &c)
                    );
                }
                let (s0, s1) = chain(name, n);
                c = c.restrict(s0, s1);
            }
            println!("{label} {name:9}{row}");
        }
    }
    println!("\n== start sample on a WIDE axis origin (half-width 1e-9), near: shipped / at-point / at-axis");
    let c = rim_at(NEAR, 1.0e-9);
    for s in [0.0, 0.25] {
        println!(
            "eval({s}): {:.2e} / {:.2e} / {:.2e}",
            pw(eval_as(Spelling::Shipped, &c, iv(s))),
            pw(eval_as(Spelling::AtPoint, &c, iv(s))),
            pw(eval_as(Spelling::AtAxis, &c, iv(s)))
        );
    }
    println!("\n== whole-sweep widths at exact angles vs placement distance (unsplit rim, s=1/2)");
    for d in [0.0, 1.0, 10.0, 100.0, 1000.0, 1.0e5] {
        let c = rim_at([d, -0.7 * d, 0.3 * d], 0.0);
        println!(
            "|at|~{d:>8.0}: {:.2e} / {:.2e} / {:.2e}",
            pw(eval_as(Spelling::Shipped, &c, iv(0.5))),
            pw(eval_as(Spelling::AtPoint, &c, iv(0.5))),
            pw(eval_as(Spelling::AtAxis, &c, iv(0.5)))
        );
    }
}

/// f64 agreement with the carrier, in ulps of the coordinates' scale,
/// for an origin-axis meridian (the lune's shape) at several radii and
/// for a far-placed unit rim.
#[test]
fn probe_f64_agreement_by_spelling() {
    let raw = Vec3::new(0.3, 0.2, 1.0);
    let axis = raw * (1.0 / raw.norm());
    let x = Vec3::new(1.0, 0.0, 0.0);
    let u0 = x - axis * axis.dot(x);
    let u_ref = u0 * (1.0 / u0.norm());
    let v_ref = axis.cross(u_ref);
    println!("\n== f64 max |description − carrier|∞ over 17 samples, as (metres, ulps of scale); shipped / at-point / at-axis");
    let cases: Vec<(&str, Point3<f64>, f64, f64, f64)> = vec![
        ("origin axis, R=1, half turn", Point3::origin(), 1.0, -FRAC_PI_2, FRAC_PI_2),
        ("origin axis, R=1e3, half turn", Point3::origin(), 1.0e3, -FRAC_PI_2, FRAC_PI_2),
        ("origin axis, R=2.3e6, half turn (lune)", Point3::origin(), 2.3e6, -FRAC_PI_2, FRAC_PI_2),
        ("origin axis, R=2.3e6, quarter turn", Point3::origin(), 2.3e6, 0.0, FRAC_PI_2),
        ("origin axis, R=2.3e6, 0.1 rad", Point3::origin(), 2.3e6, 0.0, 0.1),
        ("origin axis, R=2.3e6, full turn", Point3::origin(), 2.3e6, 0.0, TAU),
        ("far centre (1e3), R=1, full turn", Point3::new(1000.0, -700.0, 300.0), 1.0, 0.0, TAU),
        ("far centre (1e5), R=1, full turn", Point3::new(1.0e5, 3.0e4, -2.0e4), 1.0, 0.0, TAU),
        ("far centre (1e3), R=1e3, half turn", Point3::new(1000.0, -700.0, 300.0), 1.0e3, -FRAC_PI_2, FRAC_PI_2),
    ];
    for (label, center, radius, t0, t1) in cases {
        for az in [0.0, 0.5, 1.0, 2.8] {
            let carrier = Curve3::Circle {
                center,
                axis: u_ref * az.sin() - v_ref * az.cos(),
                radius,
                u_ref: u_ref * az.cos() + v_ref * az.sin(),
            };
            let Curve3::Circle { axis: n, .. } = carrier else { unreachable!() };
            let p = carrier.eval(t0);
            let scale = center.x.abs().max(center.y.abs()).max(center.z.abs()).max(radius);
            let ulp = scale * f64::EPSILON;
            let worst = |sp: Spelling| {
                (0..17)
                    .map(|i| {
                        let s = f64::from(i) / 16.0;
                        let want = carrier.eval(t0 + (t1 - t0) * s);
                        let got = turn(sp, p, center, n, (t1 - t0) * s);
                        let d = got - want;
                        d.x.abs().max(d.y.abs()).max(d.z.abs())
                    })
                    .fold(0.0, f64::max)
            };
            let (a, b, c) = (worst(Spelling::Shipped), worst(Spelling::AtPoint), worst(Spelling::AtAxis));
            println!(
                "{label:42} az={az:3}: {a:.2e} ({:.1}) / {b:.2e} ({:.1}) / {c:.2e} ({:.1})",
                a / ulp,
                b / ulp,
                c / ulp
            );
        }
    }
}

/// A restricted sketch segment: endpoints re-derived per split (the
/// tree) against a parameter window on the authored segment.
#[test]
fn probe_sketch_segment_window() {
    println!("\n== SketchSegment Interval widths (widest of s=0,1/2,1): re-derived endpoints / parameter window");
    let half_turn = |c: Point2<Interval>| SketchSegment::Arc {
        a: c + geom_core::Vec2::new(iv(1.0), iv(0.0)),
        b: c + geom_core::Vec2::new(iv(-1.0), iv(0.0)),
        arc: Arc2 {
            centre: c,
            radius: iv(1.0),
            sweep: iv(PI),
        },
    };
    let line = |c: Point2<Interval>| SketchSegment::Line {
        a: c + geom_core::Vec2::new(iv(1.0), iv(0.0)),
        b: c + geom_core::Vec2::new(iv(-1.0), iv(2.0)),
    };
    let sampled = |seg: &SketchSegment<Interval>, range: SweepRange<Interval>| {
        [0.0, 0.5, 1.0]
            .into_iter()
            .map(|s| pw2(seg.eval(range.at(iv(s)))))
            .fold(0.0, f64::max)
    };
    for (what, mk) in [("arc ", half_turn as fn(Point2<Interval>) -> SketchSegment<Interval>), ("line", line)] {
        for (label, c) in [("(0,0)      ", Point2::new(iv(0.0), iv(0.0))), ("(1000,-700)", Point2::new(iv(1000.0), iv(-700.0)))] {
            for name in ["(0,1/2)", "(0.3,0.7)", "(a,1)"] {
                let authored = mk(c);
                let mut derived = authored;
                let mut range = SweepRange::whole();
                let mut row = String::new();
                for n in 0..=64usize {
                    if [0, 1, 8, 64].contains(&n) {
                        row += &format!(
                            " | N={n}: {:.2e} / {:.2e}",
                            sampled(&derived, SweepRange::whole()),
                            sampled(&authored, range)
                        );
                    }
                    let (s0, s1) = chain(name, n);
                    derived = derived.restrict(s0, s1);
                    range = range.restrict(s0, s1);
                }
                println!("{what} centre {label} {name:9}{row}");
            }
        }
    }
    // f64: the window's eval(1) against the re-derived segment's stored b,
    // one split, on the far arc.
    let seg = SketchSegment::Arc {
        a: Point2::new(1001.0, -700.0),
        b: Point2::new(999.0, -700.0),
        arc: Arc2 {
            centre: Point2::new(1000.0, -700.0),
            radius: 1.0,
            sweep: PI,
        },
    };
    let sub = seg.restrict(0.3, 0.7);
    let win = SweepRange::whole().restrict(0.3, 0.7);
    let (d0, d1) = (sub.eval(0.0), seg.eval(win.at(0.0)));
    let (e0, e1) = (sub.eval(1.0), seg.eval(win.at(1.0)));
    println!(
        "f64 one split (0.3,0.7) far arc: start re-derived {:?} vs window {:?}; end {:?} vs {:?}",
        d0, d1, e0, e1
    );
}
