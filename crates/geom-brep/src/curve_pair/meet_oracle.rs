//! **The walk as the closed forms' differential oracle**, and the
//! band's contract checked apart from both: a counterexample search over
//! line, circle and ellipse pairs drawn to meet, to miss inside the zero
//! band, to miss outside it, and to sit near the poses the closed forms
//! decide by a `Zero` (near-tangent coplanar circles, circles in nearly
//! parallel planes, curves crossing at a wide angle in nearly parallel
//! planes). The checks, each apart from the reader's own arithmetic:
//!
//! - **a decided "no" is never in the band**: a draw built with its least
//!   distance inside the zero band inside both spans never reads as
//!   decided apart, and where the closed forms decide "no" and the walk
//!   does not agree, the spans' least distance (sampled and polished
//!   independently) is past the zero band;
//! - every meeting the closed forms decide lies on both spans, by an
//!   independent distance (exact on a line or a circle, sampled and
//!   polished on an ellipse).
//!
//! **Where the walk is vacuous.** It cannot clear two circles that stay
//! within a few bands of each other round a long arc, and runs past its
//! budget there: on most grazing and tilted draws it refuses, and the
//! comparison with it is empty. There the built-in meeting and the
//! independent least distance are what check the closed forms.
//!
//! A pinned draw (`the_closed_forms_decide_most_draws`) holds the
//! reader to deciding: a reader that refused everywhere would pass every
//! check above.

#![allow(clippy::unwrap_used, clippy::panic)]

test_utils::gated_to![
    "crates/geom-brep/src/curve_pair.rs",
    "crates/geom-brep/src/curve_pair/",
    "crates/geom/src/carrier_speed.rs",
    "crates/geom-brep/src/half_angle.rs",
    "crates/geom-core/src/bisect.rs",
];

use core::f64::consts::{PI, TAU};

use geom::Curve3;
use geom_core::{Point3, Vec3};
use test_utils::fuzz;

use super::meet_rows::{ZERO, band, distance, u_for};
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
        a * rng.log_uniform(1e-3, 0.9)
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
    let g = geom_core::GOLDEN_SECTION;
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
/// from the reader.
fn least_distance(a: &Span<'_, f64>, b: &Span<'_, f64>) -> f64 {
    let to_b = |q: Point3<f64>| {
        let (t0, t1) = b.params();
        least(t0, t1, 200, |t| (b.carrier().eval(t) - q).norm()).1
    };
    let (t0, t1) = a.params();
    least(t0, t1, 500, |t| to_b(a.carrier().eval(t))).1
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
/// `apart` along a random direction: within the zero band, their least
/// distance is in it there.
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
        family: if apart == 0.0 {
            "crossing"
        } else if apart <= ZERO {
            "near-miss"
        } else {
            "apart"
        },
        a: (a, about(rng, ka > 0, ta)),
        b: (b, about(rng, kb > 0, tb)),
        meets: apart <= ZERO,
    }
}

/// A carrier of kind `kind` (0 line, 1 circle, 2 ellipse) in the plane
/// through `x` square to `n`, running along the unit `t` there, and its
/// parameter at `x`.
fn along(
    rng: &mut fuzz::Rng,
    kind: usize,
    x: Point3<f64>,
    n: Vec3<f64>,
    t: Vec3<f64>,
) -> (Curve3<f64>, f64) {
    if kind == 0 {
        let at = rng.range(-2.0, 2.0);
        return (
            Curve3::Line {
                origin: x - t * at,
                dir: t,
            },
            at,
        );
    }
    let a = rng.log_uniform(0.3, 3.0);
    let b = if kind == 1 {
        a
    } else {
        a * rng.log_uniform(1e-3, 0.9)
    };
    let theta = rng.range(-PI, PI);
    // The tangent at `theta` is `cos α·û + sin α·v̂`: turn the frame so it
    // is `t`.
    let alpha = (b * theta.cos()).atan2(-a * theta.sin());
    let u = t * alpha.cos() - n.cross(t) * alpha.sin();
    let v = n.cross(u);
    let center = x - u * (a * theta.cos()) - v * (b * theta.sin());
    let carrier = if kind == 1 {
        Curve3::Circle {
            center,
            axis: n,
            radius: a,
            u_ref: u,
        }
    } else {
        Curve3::Ellipse {
            center,
            axis: n,
            semi_u: a,
            semi_v: b,
            u_ref: u,
        }
    };
    (carrier, theta)
}

/// Two carriers crossing at a wide angle in planes `τ` apart, the second
/// moved off the first square to both by a draw inside the zero band:
/// their least distance is in it, where neither crosses the other's
/// plane.
fn shallow(rng: &mut fuzz::Rng) -> Drawn {
    let x = Point3::new(
        rng.range(-3.0, 3.0),
        rng.range(-3.0, 3.0),
        rng.range(-3.0, 3.0),
    );
    let n1 = direction(rng);
    let in_plane = |rng: &mut fuzz::Rng, n: Vec3<f64>| {
        let w = direction(rng);
        (w - n * w.dot(n)).normalize()
    };
    let ta = in_plane(rng, n1);
    let hinge = in_plane(rng, n1);
    let tau = rng.log_uniform(1e-7, 1e-2);
    let n2 = n1 * tau.cos() + hinge.cross(n1) * tau.sin();
    let ta2 = (ta - n2 * ta.dot(n2)).normalize();
    let phi = rng.range(0.3, PI - 0.3);
    let tb = ta2 * phi.cos() + n2.cross(ta2) * phi.sin();
    let h = rng.range(0.0, 0.95) * ZERO;
    let (ka, kb) = (rng.below(3), rng.below(3));
    let (a, sa) = along(rng, ka, x, n1, ta);
    let (b, sb) = along(rng, kb, x + ta.cross(tb).normalize() * h, n2, tb);
    Drawn {
        family: "shallow",
        a: (a, about(rng, ka > 0, sa)),
        b: (b, about(rng, kb > 0, sb)),
        meets: true,
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

/// Whether an answer, or the meetings a refusal decided, say the spans
/// meet, at an end or inside: `None` for a refusal that decided none.
fn verdict(got: &Result<Meetings<f64>, MeetRefusal<f64>>) -> Option<bool> {
    match got {
        Ok(m) => Some(!m.points.is_empty()),
        Err(r) => (!r.found.is_empty()).then_some(true),
    }
}

/// `q`'s distance from the span, apart from the reader: exact on a line
/// (its segment) and a circle (its arc: the carrier's distance where the
/// foot's angle lies in the span, else the nearer end's), sampled and
/// polished on any other kind.
fn off_span(s: &Span<'_, f64>, q: Point3<f64>) -> f64 {
    let (t0, t1) = s.params();
    let ends = || {
        (s.carrier().eval(t0) - q)
            .norm()
            .min((s.carrier().eval(t1) - q).norm())
    };
    match *s.carrier() {
        Curve3::Line { origin, dir } => {
            let t = (q - origin).dot(dir) / dir.dot(dir);
            if (t0.min(t1)..=t0.max(t1)).contains(&t) {
                distance(s.carrier(), q)
            } else {
                ends()
            }
        }
        Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        } => {
            let w = q - center;
            let v = axis.cross(u_ref);
            let angle = (w.dot(v) * radius.signum()).atan2(w.dot(u_ref) * radius.signum());
            let mid = 0.5 * (t0 + t1);
            let t = mid + (angle - mid + PI).rem_euclid(TAU) - PI;
            if (t0.min(t1)..=t0.max(t1)).contains(&t) {
                distance(s.carrier(), q)
            } else {
                ends()
            }
        }
        ref c => least(t0, t1, 2000, |t| (c.eval(t) - q).norm()).1,
    }
}

/// What a run of draws measured.
#[derive(Default)]
struct Tally {
    /// Draws the closed forms answered, a decided meeting before a
    /// refusal included.
    decided: usize,
    /// Draws.
    drawn: usize,
    /// Draws both the closed forms and the walk decided.
    compared: usize,
    /// Meetings checked on a span.
    placed: usize,
    /// Refusals, by family, side and kind.
    refused: std::collections::BTreeMap<String, usize>,
}

/// One draw of family `i % 6`.
fn draw(rng: &mut fuzz::Rng, i: usize) -> Drawn {
    match i % 6 {
        0 => crossing(rng, 0.0),
        1 => {
            let apart = rng.log_uniform(1e-7, 1e-1);
            crossing(rng, apart)
        }
        2 => {
            let apart = rng.range(0.05, 0.95) * ZERO;
            crossing(rng, apart)
        }
        3 => grazing(rng),
        4 => tilted(rng),
        _ => shallow(rng),
    }
}

/// **The checks** (module docs) over `n` draws.
fn run(rng: &mut fuzz::Rng, n: usize) -> Tally {
    let mut tally = Tally::default();
    for i in 0..n {
        let drawn = draw(rng, i);
        let (a, b) = (
            Span::new(&drawn.a.0, drawn.a.1),
            Span::new(&drawn.b.0, drawn.b.1),
        );
        let label = format!("draw {i} ({}), {}", drawn.family, fuzz::replay());
        let closed = meet(&a, &b, band());
        tally.drawn += 1;
        let by_closed = verdict(&closed);
        tally.decided += usize::from(by_closed.is_some());
        assert!(
            !(drawn.meets && by_closed == Some(false)),
            "{label}: built with its least distance in the zero band, the closed forms decide \
             no meeting"
        );
        let decided = match &closed {
            Ok(m) => m.points.as_slice(),
            Err(r) => r.found.as_slice(),
        };
        for p in decided {
            for s in [&a, &b] {
                let off = off_span(s, p.point);
                assert!(off <= ZERO, "{label}: a meeting lies {off:e} off a span");
                tally.placed += 1;
            }
        }
        let walked = super::walk::meet(&a, &b, band());
        for (side, fault) in [
            ("closed", closed.as_ref().err().map(|r| r.fault)),
            ("walk", walked.as_ref().err().map(|r| r.fault)),
        ] {
            if let Some(fault) = fault {
                let kind = format!("{fault:?}");
                let kind = kind.split([' ', '{']).next().unwrap_or("").to_owned();
                *tally
                    .refused
                    .entry(format!("{} {side} {kind}", drawn.family))
                    .or_default() += 1;
            }
        }
        let by_walk = verdict(&walked);
        if let (Some(_), Some(_)) = (by_closed, by_walk) {
            tally.compared += 1;
            assert!(
                !drawn.meets || by_walk == Some(true),
                "{label}: built to meet, the walk reads no meeting"
            );
        }
        // A decided meeting lies on both spans (above). A decided "no"
        // the walk does not share is held to the least distance.
        if by_closed != Some(false) || by_walk == Some(false) {
            continue;
        }
        let least = least_distance(&a, &b);
        assert!(
            least > ZERO,
            "{label}: the closed forms decide no meeting, the walk {by_walk:?}, and the spans' \
             least distance is {least:e}"
        );
    }
    tally
}

/// **The checks** (module docs), a counterexample search.
#[test]
fn the_closed_forms_agree_with_the_walk_outside_the_band() {
    let mut rng = fuzz::start("curve_pair::meet_oracle");
    let t = run(&mut rng, fuzz::scaled(240));
    println!(
        "[curve_pair::meet_oracle] drawn {}, decided {}, compared {}, placed {}, refused {:?}",
        t.drawn, t.decided, t.compared, t.placed, t.refused
    );
}

/// **The closed forms decide most draws**, on a pinned draw: the checks
/// above pass a reader that refuses everywhere, and this does not.
#[test]
fn the_closed_forms_decide_most_draws() {
    let mut rng = fuzz::pinned("curve_pair::meet_oracle::decided", 0x5121_c0de);
    let t = run(&mut rng, 120);
    assert!(
        t.decided * 10 >= t.drawn * 7,
        "the closed forms decided {} of {} draws; refused {:?}",
        t.decided,
        t.drawn,
        t.refused
    );
}
