//! Reviewer R1 probes for PR 5227.
//!
//! Red on 04b5385628: `probe_a_line_through_the_plane_meets_the_circle_twice`,
//! `probe_a2_seam_elsewhere`, `probe_d_two_found_meetings_near_one_root_leave_the_other_roots_meetings`
//! (main refuses `Unsettled(Meter)` on each). Green: `probe_b`, `probe_c` (3000 random coplanar ellipse pairs).
#![allow(clippy::unwrap_used, clippy::panic)]

use core::f64::consts::TAU;

use geom_core::Point3;

use super::meet_rows::{band, circle, line};
use super::{Span, meet};

/// A line crossing a circle's plane at a slope whose rise over the
/// circle's chord is in the band's gap, passing within the zero band of
/// the circle at both ends of the chord. Started at one end of the chord
/// (found first), the other end of the chord is a second meeting in its
/// own stretch (the chord's middle is a metre off the circle).
#[test]
fn probe_a_line_through_the_plane_meets_the_circle_twice() {
    let c = circle([0.0; 3], [0.0, 0.0, 1.0], 1.0);
    let h = 0.9e-9;
    let q = Point3::new(1.0, 0.0, h);
    let p = Point3::new(-1.0, 0.0, -h);
    let l = line(q.to_array(), (p - q).to_array());
    let whole = Span::new(&c, (0.0, TAU)).endless();
    let mut lost = Vec::new();
    for (label, t0) in [("starts at the chord's end", 0.0), ("starts before it", -1.0)] {
        let seg = Span::new(&l, (t0, 100.0));
        for (order, got) in [
            ("line first", meet(&seg, &whole, band())),
            ("circle first", meet(&whole, &seg, band())),
        ] {
            eprintln!("PROBE A {label} {order}: {got:?}");
            if let Ok(m) = &got {
                if !m.points.iter().any(|m| (m.point - p).norm() < 1e-6) {
                    lost.push(format!("{label} {order}"));
                }
            }
        }
    }
    assert!(lost.is_empty(), "the far end of the chord is lost: {lost:?}");
}

#[test]
fn probe_a_pose() {
    let c = circle([0.0; 3], [0.0, 0.0, 1.0], 1.0);
    let h = 0.9e-9;
    let q = Point3::new(1.0, 0.0, h);
    let p = Point3::new(-1.0, 0.0, -h);
    let l = line(q.to_array(), (p - q).to_array());
    let whole = Span::new(&c, (0.0, TAU)).endless();
    let seg = Span::new(&l, (-1.0, 100.0));
    eprintln!("PROBE POSE: {:?}", super::pose(&seg, &whole, band()));
}

/// Probe A with the circle's seam a quarter turn from the chord, so the
/// only end found first is the line's own start, on the circle.
#[test]
fn probe_a2_seam_elsewhere() {
    let c = geom::Curve3::Circle {
        center: Point3::origin(),
        axis: geom_core::Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: geom_core::Vec3::new(0.0, 1.0, 0.0),
    };
    let h = 0.9e-9;
    let q = Point3::new(1.0, 0.0, h);
    let p = Point3::new(-1.0, 0.0, -h);
    let l = line(q.to_array(), (p - q).to_array());
    let mut lost = Vec::new();
    for (label, ended) in [("endless", false), ("ended", true)] {
        let mut whole = Span::new(&c, (0.0, TAU));
        if !ended {
            whole = whole.endless();
        }
        for (t0, starts) in [(0.0, "at Q"), (-1.0, "before Q")] {
            let seg = Span::new(&l, (t0, 100.0));
            for (order, got) in [
                ("line first", meet(&seg, &whole, band())),
                ("circle first", meet(&whole, &seg, band())),
            ] {
                eprintln!("PROBE A2 {label} {starts} {order}: {got:?}");
                if let Ok(m) = &got {
                    if !m.points.iter().any(|m| (m.point - p).norm() < 1e-6) {
                        lost.push(format!("{label} {starts} {order}"));
                    }
                }
            }
        }
    }
    assert!(lost.is_empty(), "P lost: {lost:?}");
}

/// Probe B: probe A with the line replaced by a great circle (R = 1000)
/// through the same two points, so the pair takes the transverse arm.
#[test]
fn probe_b_a_big_circle_through_the_plane_meets_the_circle_twice() {
    use geom_core::Vec3;
    let c = geom::Curve3::Circle {
        center: Point3::origin(),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::new(0.0, 1.0, 0.0),
    };
    let h = 0.9e-9;
    let q = Point3::new(1.0, 0.0, h);
    let p = Point3::new(-1.0, 0.0, -h);
    let big = 1000.0;
    let d = (p - q).normalize();
    let y = Vec3::new(0.0, 1.0, 0.0);
    let centre = Point3::origin() - y * (big * big - 1.0f64).sqrt();
    let axis = d.cross(y).normalize();
    let u_ref = (q - centre).normalize();
    let a = geom::Curve3::Circle { center: centre, axis, radius: big, u_ref };
    // `q` at θ = 0; `p` a little over 2e-3 rad on, in one sense or the other.
    let tp = {
        let w = p - centre;
        let v = axis.cross(u_ref);
        w.dot(v).atan2(w.dot(u_ref))
    };
    eprintln!("PROBE B tp = {tp}, |q-a| check {:?}", (a.eval(0.0) - q).norm());
    let span_a = if tp > 0.0 { (0.0, 0.1) } else { (-0.1, 0.0) };
    let sa = Span::new(&a, span_a);
    let whole = Span::new(&c, (0.0, TAU)).endless();
    let mut lost = Vec::new();
    for (order, got) in [
        ("big first", meet(&sa, &whole, band())),
        ("unit first", meet(&whole, &sa, band())),
    ] {
        eprintln!("PROBE B {order}: {got:?}");
        if let Ok(m) = &got {
            if !m.points.iter().any(|m| (m.point - p).norm() < 1e-6) {
                lost.push(order);
            }
        }
    }
    let mut f = super::Found::default();
    f.read(&sa, &whole, super::Candidate { point: a.eval(tp), ta: tp, tb: core::f64::consts::FRAC_PI_2 }, band());
    eprintln!("PROBE B p read alone: {:?}", f.points);
    assert!(lost.is_empty(), "P lost: {lost:?}");
}

/// Probe C: random coplanar ellipse pairs against a dense sign-change count.
#[test]
fn probe_c_coplanar_ellipses_random() {
    use core::f64::consts::PI;
    let mut seed: u64 = 0x5227_5227;
    let mut rnd = || {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((seed >> 11) as f64) / ((1u64 << 53) as f64)
    };
    let (mut ok, mut refused, mut bad) = (0, 0, 0);
    for i in 0..3000 {
        let skinny = i % 3 == 0;
        let mk = |rnd: &mut dyn FnMut() -> f64| {
            let a = 0.2 + 3.0 * rnd();
            let b = if skinny { a * (1e-3 + 0.05 * rnd()) } else { a * (0.1 + 0.9 * rnd()) };
            let ang = 2.0 * PI * rnd();
            let c = [2.0 * rnd() - 1.0, 2.0 * rnd() - 1.0];
            super::meet_rows::ellipse([c[0], c[1], 0.0], [0.0, 0.0, 1.0], [ang.cos(), ang.sin(), 0.0], a, b)
        };
        let x = mk(&mut rnd);
        let y = mk(&mut rnd);
        let (t0, t1) = (-PI + 0.3 * rnd(), PI - 0.3 * rnd());
        let (sx, sy) = (Span::new(&x, (t0, t1)), Span::new(&y, (-PI, PI)).endless());
        let level = |p: Point3<f64>| {
            let geom::Curve3::Ellipse { center, axis, semi_u, semi_v, u_ref } = y else { unreachable!() };
            let w = p - center;
            (w.dot(u_ref) / semi_u).powi(2) + (w.dot(axis.cross(u_ref)) / semi_v).powi(2) - 1.0
        };
        let n = 200_000;
        let mut changes = 0;
        let mut prev = level(x.eval(t0));
        for k in 1..=n {
            let t = t0 + (t1 - t0) * f64::from(k) / f64::from(n);
            let v = level(x.eval(t));
            if (v > 0.0) != (prev > 0.0) {
                changes += 1;
            }
            prev = v;
        }
        match meet(&sx, &sy, band()) {
            Ok(m) => {
                ok += 1;
                let on_both = m.points.iter().all(|p| {
                    [&x, &y].iter().all(|c| {
                        matches!(
                            c.on_carrier(p.point, c.param_near(p.point, 0.0).unwrap_or(0.0), band()),
                            Ok(geom::OnCarrier::On { .. })
                        )
                    })
                });
                if m.points.len() != changes || !on_both {
                    bad += 1;
                    eprintln!("PROBE C {i}: {} meetings vs {changes} sign changes, on both {on_both}: x {x:?} y {y:?} span {:?}", m.points.len(), (t0, t1));
                }
            }
            Err(_) => refused += 1,
        }
    }
    eprintln!("PROBE C ok {ok} refused {refused} bad {bad}");
    assert_eq!(bad, 0);
}

/// Probe D: a unit circle tilted 2e-8 about x̂ and an ellipse in z = 0
/// whose projection-crossings are at θ = ±0.03 and π ± 0.03, all within
/// 6e-10 of the circle's plane: four meetings. The circle's span ends
/// at θ = ±0.03, so the ends are two found meetings; the interior holds
/// the other two, near the height harmonic's other root.
#[test]
fn probe_d_two_found_meetings_near_one_root_leave_the_other_roots_meetings() {
    use geom_core::Vec3;
    let al = 2e-8_f64;
    let (s, c) = al.sin_cos();
    let a = geom::Curve3::Circle {
        center: Point3::origin(),
        axis: Vec3::new(0.0, -s, c),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let th = 0.03_f64;
    let big = th.cos() / (1.0 - (c * th.sin()).powi(2) / 0.81).sqrt();
    let b = geom::Curve3::Ellipse {
        center: Point3::origin(),
        axis: Vec3::new(0.0, 0.0, 1.0),
        semi_u: 0.9,
        semi_v: big,
        u_ref: Vec3::new(0.0, 1.0, 0.0),
    };
    let sa = Span::new(&a, (th, TAU - th));
    let mut fails = Vec::new();
    for (bl, sb) in [
        ("b endless", Span::new(&b, (0.0, TAU)).endless()),
        ("b half", Span::new(&b, (0.0, core::f64::consts::PI))),
    ] {
        for (order, got) in [("circle first", meet(&sa, &sb, band())), ("ellipse first", meet(&sb, &sa, band()))] {
            eprintln!("PROBE D {bl} {order}: {got:?}");
            if let Ok(m) = &got {
                let far = m.points.iter().filter(|p| p.point.x < 0.0).count();
                let want = if bl == "b endless" { 2 } else { 1 };
                if far != want {
                    fails.push(format!("{bl} {order}: {far} of {want} far meetings"));
                }
            }
        }
    }
    let sb = Span::new(&b, (0.0, TAU)).endless();
    for t in [core::f64::consts::PI - th, core::f64::consts::PI + th] {
        let q = a.eval(t);
        let tb = b.param_near(q, 0.0).unwrap_or(0.0);
        let mut f = super::Found::default();
        f.read(&sa, &sb, super::Candidate { point: q, ta: t, tb }, band());
        let polished = b.foot(q, tb).t();
        let mut g = super::Found::default();
        g.read(&sa, &sb, super::Candidate { point: q, ta: t, tb: polished }, band());
        eprintln!("PROBE D far point {t}: dist to ellipse foot {:e}; read {:?} / {:?} held {:?}", (b.eval(polished) - q).norm(), f.points.len(), g.points, g.held.len());
    }
    assert!(fails.is_empty(), "{fails:?}");
}
