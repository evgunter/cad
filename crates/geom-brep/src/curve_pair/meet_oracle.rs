//! **The walk as the closed forms' differential oracle**, outside the
//! band: a counterexample search over line, circle and ellipse pairs
//! drawn to meet, to near-miss, and to sit near the poses the closed forms
//! decide by a `Zero` (near-tangent coplanar circles, circles in nearly
//! parallel planes). Three checks, each apart from the reader's own
//! arithmetic:
//!
//! - where both decide whether the interiors meet (an answer, or a
//!   meeting decided before a refusal), they agree, unless the spans'
//!   least distance is in the band;
//! - where a draw is built to meet inside both spans, each answer says so;
//! - every meeting the closed forms place on a line or a circle lies on
//!   it.
//!
//! The walk cannot clear two circles that stay within a few bands of each
//! other round a long arc, and runs past its budget there: on the grazing
//! and tilted draws it is the built-in meeting that checks the closed
//! forms.

#![allow(clippy::unwrap_used, clippy::panic)]

test_utils::gated_to![
    "crates/geom-brep/src/curve_pair.rs",
    "crates/geom-brep/src/curve_pair/",
    "crates/geom-brep/src/carrier_speed.rs",
    "crates/geom-brep/src/half_angle.rs",
    "crates/geom-core/src/bisect.rs",
];

use core::f64::consts::{PI, TAU};

use geom::Curve3;
use geom_core::{Point3, Vec3};
use test_utils::fuzz;

use super::meet_rows::{ZERO, band, distance, interiors_meet, u_for};
use super::{MeetRefusal, Meetings, Span, meet};

/// A random unit vector.
fn direction(rng: &mut fuzz::Rng) -> Vec3<f64> {
    loop {
        let v = Vec3::new(
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
        );
        if v.norm() > 0.2 && v.norm() < 1.0 {
            return v.normalize();
        }
    }
}

/// A random carrier of kind `kind` (0 line, 1 circle, 2 ellipse) through
/// `x`, and its parameter there.
fn through(rng: &mut fuzz::Rng, kind: usize, x: Point3<f64>) -> (Curve3<f64>, f64) {
    if kind == 0 {
        let dir = direction(rng);
        let t = rng.range(-2.0, 2.0);
        return (
            Curve3::Line {
                origin: x - dir * t,
                dir,
            },
            t,
        );
    }
    let axis = direction(rng);
    let u = {
        let w = direction(rng);
        (w - axis * w.dot(axis)).normalize()
    };
    let v = axis.cross(u);
    let a = rng.log_uniform(0.3, 3.0);
    let b = if kind == 1 {
        a
    } else {
        a * rng.range(0.2, 0.9)
    };
    let t = rng.range(-PI, PI);
    let center = x - u * (a * t.cos()) - v * (b * t.sin());
    let carrier = if kind == 1 {
        Curve3::Circle {
            center,
            axis,
            radius: a,
            u_ref: u,
        }
    } else {
        Curve3::Ellipse {
            center,
            axis,
            semi_u: a,
            semi_v: b,
            u_ref: u,
        }
    };
    (carrier, t)
}

/// A span about `t`: a run each side, a conic's under half a turn each.
fn about(rng: &mut fuzz::Rng, conic: bool, t: f64) -> (f64, f64) {
    let most = if conic { 1.4 } else { 2.0 };
    (t - rng.range(0.1, most), t + rng.range(0.1, most))
}

/// The least of `f` over `[lo, hi]` by golden-section search: its
/// argument and value.
fn golden(mut lo: f64, mut hi: f64, f: impl Fn(f64) -> f64) -> (f64, f64) {
    let g = (5f64.sqrt() - 1.0) / 2.0;
    for _ in 0..80 {
        let (x1, x2) = (hi - g * (hi - lo), lo + g * (hi - lo));
        if f(x1) < f(x2) {
            hi = x2;
        } else {
            lo = x1;
        }
    }
    let t = (lo + hi) / 2.0;
    (t, f(t))
}

/// The least of `f` over `[t0, t1]`: sampled, then polished about the
/// best sample.
fn least(t0: f64, t1: f64, samples: usize, f: impl Fn(f64) -> f64) -> (f64, f64) {
    let at = |k: usize| t0 + (t1 - t0) * (k as f64) / (samples as f64);
    let (mut k_best, mut best) = (0, f64::INFINITY);
    for k in 0..=samples {
        let d = f(at(k));
        if d < best {
            (k_best, best) = (k, d);
        }
    }
    golden(
        at(k_best.saturating_sub(1)),
        at((k_best + 1).min(samples)),
        f,
    )
}

/// The least distance between the two spans, sampled and polished apart
/// from the reader: `None` where it lies within a millionth of a span
/// end, where an interior meeting is not the question.
fn least_distance(a: &Span<'_, f64>, b: &Span<'_, f64>) -> Option<f64> {
    let to_b = |q: Point3<f64>| {
        let (t0, t1) = b.params();
        least(t0, t1, 400, |t| (b.carrier().eval(t) - q).norm())
    };
    let (t0, t1) = a.params();
    let (ta, d) = least(t0, t1, 2000, |t| to_b(a.carrier().eval(t)).1);
    let tb = to_b(a.carrier().eval(ta)).0;
    let near_end = |t: f64, (s0, s1): (f64, f64)| (t - s0).abs().min((t - s1).abs()) < 1e-6;
    (!near_end(ta, a.params()) && !near_end(tb, b.params())).then_some(d)
}

/// One drawn pose: its family, two carriers with their spans, and
/// whether it is built to meet inside both spans.
struct Drawn {
    family: &'static str,
    a: (Curve3<f64>, (f64, f64)),
    b: (Curve3<f64>, (f64, f64)),
    meets: bool,
}

/// Whether `t` lies inside `(t0, t1)` by `margin`, on the branch about
/// its middle.
fn inside(t: f64, (t0, t1): (f64, f64), margin: f64) -> bool {
    let mid = (t0 + t1) / 2.0;
    let t = mid + (t - mid + PI).rem_euclid(TAU) - PI;
    t0 + margin < t && t < t1 - margin
}

/// Two random carriers through one point, the second moved off it by
/// `apart` along a random direction (zero: they meet there).
fn crossing(rng: &mut fuzz::Rng, apart: f64) -> Drawn {
    let x = Point3::new(
        rng.range(-3.0, 3.0),
        rng.range(-3.0, 3.0),
        rng.range(-3.0, 3.0),
    );
    let (ka, kb) = (rng.below(3), rng.below(3));
    let (a, ta) = through(rng, ka, x);
    let off = direction(rng) * apart;
    let (b, tb) = through(rng, kb, x + off);
    Drawn {
        family: if apart == 0.0 { "crossing" } else { "apart" },
        a: (a, about(rng, ka > 0, ta)),
        b: (b, about(rng, kb > 0, tb)),
        meets: apart == 0.0,
    }
}

/// Two circles in one plane, centres `d` apart, their radii's gap within
/// the zero band of `±d`: a near-tangent pair whose near extreme is
/// decided zero.
fn grazing(rng: &mut fuzz::Rng) -> Drawn {
    let axis = direction(rng);
    let u = u_for(axis);
    let c = Point3::new(
        rng.range(-3.0, 3.0),
        rng.range(-3.0, 3.0),
        rng.range(-3.0, 3.0),
    );
    let r = rng.log_uniform(0.3, 3.0);
    let d = rng.log_uniform(3e-8, 1e-4);
    let sign = if rng.below(2) == 0 { 1.0 } else { -1.0 };
    let r2 = r + sign * d + rng.range(-0.5, 0.5) * ZERO;
    let mk = |center: Point3<f64>, radius: f64| Curve3::Circle {
        center,
        axis,
        radius,
        u_ref: u,
    };
    let start = rng.range(-PI, PI);
    let (run_a, run_b) = (rng.range(PI, TAU - 0.1), rng.range(PI, TAU - 0.1));
    let (sa, sb) = ((start, start + run_a), (start - 1.0, start - 1.0 + run_b));
    // The near extreme, within half a zero of the other circle, is at
    // the same angle on both: `−π/2` with the larger second circle, `π/2`
    // with the smaller.
    let touch = -sign * PI / 2.0;
    Drawn {
        family: "grazing",
        a: (mk(c, r), sa),
        b: (mk(c + axis.cross(u) * d, r2), sb),
        meets: inside(touch, sa, 0.05) && inside(touch, sb, 0.05),
    }
}

/// Two circles cut from one sphere by planes tilted `θ` about a line
/// through it: they meet at the line's two points on the sphere.
fn tilted(rng: &mut fuzz::Rng) -> Drawn {
    let centre = Point3::new(
        rng.range(-3.0, 3.0),
        rng.range(-3.0, 3.0),
        rng.range(-3.0, 3.0),
    );
    let radius = rng.log_uniform(0.3, 3.0);
    let n1 = direction(rng);
    let hinge = u_for(n1);
    let theta = rng.log_uniform(1e-8, 1e-2);
    let n2 = n1 * theta.cos() + n1.cross(hinge) * theta.sin();
    let h = radius * rng.range(-0.8, 0.8);
    let foot = |n: Vec3<f64>| {
        let d = (n1 * h).dot(n);
        (centre + n * d, (radius * radius - d * d).sqrt())
    };
    let ((c1, r1), (c2, r2)) = (foot(n1), foot(n2));
    let mk = |center, axis: Vec3<f64>, radius| Curve3::Circle {
        center,
        axis,
        radius,
        u_ref: u_for(axis),
    };
    let start = rng.range(-PI, PI);
    let (a, b) = (mk(c1, n1, r1), mk(c2, n2, r2));
    let (sa, sb) = (
        (start, start + TAU - 0.05),
        (start + 1.0, start + 1.0 + TAU - 0.05),
    );
    // They meet where the hinge line crosses the sphere.
    let half = (radius * radius - h * h).sqrt();
    let meets = [half, -half].into_iter().any(|s| {
        let q = centre + n1 * h + hinge * s;
        let at = |c: &Curve3<f64>| c.param_near(q, 0.0).unwrap();
        inside(at(&a), sa, 1e-3) && inside(at(&b), sb, 1e-3)
    });
    Drawn {
        family: "tilted",
        a: (a, sa),
        b: (b, sb),
        meets,
    }
}

/// Whether an answer, or the meetings a refusal decided, say the
/// interiors meet: `None` for a refusal that decided none.
fn verdict(got: &Result<Meetings<f64>, MeetRefusal<f64>>) -> Option<bool> {
    match got {
        Ok(m) => Some(interiors_meet(m)),
        Err(r) => interiors_meet(&Meetings {
            shared: None,
            points: r.found.clone(),
        })
        .then_some(true),
    }
}

/// **The three checks** (module docs).
#[test]
fn the_closed_forms_agree_with_the_walk_outside_the_band() {
    let mut rng = fuzz::start("curve_pair::meet_oracle");
    let (mut compared, mut placed) = (0usize, 0usize);
    let mut refused = std::collections::BTreeMap::<String, usize>::new();
    for i in 0..fuzz::scaled(240) {
        let drawn = match i % 4 {
            0 => crossing(&mut rng, 0.0),
            1 => {
                let apart = rng.log_uniform(1e-7, 1e-1);
                crossing(&mut rng, apart)
            }
            2 => grazing(&mut rng),
            _ => tilted(&mut rng),
        };
        let (a, b) = (
            Span::new(&drawn.a.0, drawn.a.1),
            Span::new(&drawn.b.0, drawn.b.1),
        );
        let label = format!("draw {i} ({}), {}", drawn.family, fuzz::replay());
        let closed = meet(&a, &b, band());
        assert!(
            !(drawn.meets && verdict(&closed) == Some(false)),
            "{label}: built to meet, the closed forms read no meeting"
        );
        if let Ok(m) = &closed {
            for p in &m.points {
                for c in [&drawn.a.0, &drawn.b.0] {
                    if matches!(c, Curve3::Line { .. } | Curve3::Circle { .. }) {
                        let off = distance(c, p.point);
                        assert!(off <= ZERO, "{label}: a meeting lies {off:e} off a carrier");
                        placed += 1;
                    }
                }
            }
        }
        let walked = super::walk::meet(&a, &b, band());
        for (side, fault) in [
            ("closed", closed.as_ref().err().map(|r| r.fault)),
            ("walk", walked.as_ref().err().map(|r| r.fault)),
        ] {
            if let Some(fault) = fault {
                let kind = format!("{fault:?}");
                let kind = kind.split(['(', ' ']).next().unwrap_or("").to_owned();
                *refused
                    .entry(format!("{} {side} {kind}", drawn.family))
                    .or_default() += 1;
            }
        }
        let (Some(by_closed), Some(by_walk)) = (verdict(&closed), verdict(&walked)) else {
            continue;
        };
        assert!(
            !drawn.meets || by_walk,
            "{label}: built to meet, the walk reads no meeting"
        );
        compared += 1;
        if by_closed == by_walk {
            continue;
        }
        let Some(least) = least_distance(&a, &b) else {
            continue;
        };
        assert!(
            (0.1 * ZERO..100.0 * ZERO).contains(&least),
            "{label}: the closed forms read meet = {by_closed}, the walk {by_walk}, and the \
             spans' least distance is {least:e}"
        );
    }
    println!("[curve_pair::meet_oracle] compared {compared}, placed {placed}, refused {refused:?}");
}
