//! **The ring composite against itself and against dense sampling**, a
//! fuzzer (shape 1): random spline spans against every analytic form,
//! each read at `f64` and at `Interval` through the table
//! ([`crate::curve_face::cross()`]). Two oracles, neither the walk's own
//! arithmetic:
//!
//! - **the two scalars agree** where both answer: the same interior
//!   crossings, within `10⁻⁶` in parameter, and the same end meetings;
//! - **a clear sign change is a meeting**: where the surface's implicit
//!   residual, sampled densely along the span at `f64`, changes sign
//!   between two samples each a hundred escalation widths off the
//!   surface, an answer at either scalar holds a meeting between them.
//!
//! The arm's measurement against the interval composite it replaced
//! (PR 7801's body) is not kept: its oracle is the retired arm. These are
//! the parts of it that stand without one.

#![allow(clippy::unwrap_used, clippy::panic, clippy::cast_precision_loss)]

use geom::{Curve3, NurbsCurve3, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Band, Interval, Point3, Vec3};
use test_utils::fuzz;

use crate::curve_face::{OnSpan, cross};
use crate::curve_pair::{Span, SpanEnd};

test_utils::gated_to![
    "crates/geom-brep/src/curve_face/composite.rs",
    "crates/geom-brep/src/curve_face/composite/",
    "crates/geom-brep/src/curve_face/cross.rs",
    "crates/geom-core/src/spline/canonical_rows.rs",
    "crates/geom-core/src/spline/derivative.rs",
];

fn frame(o: Point3<f64>) -> geom::Frame<f64> {
    geom::Frame::new(o, Vec3::unit_z(), Vec3::unit_x())
}

/// One of each analytic form about the carriers' run.
fn surface(k: usize) -> Surface<f64> {
    let at = Point3::new(0.5, 0.0, 0.0);
    match k % 5 {
        0 => Surface::Plane {
            frame: frame(Point3::origin()),
        },
        1 => Surface::Cylinder {
            frame: frame(at),
            radius: 0.4,
        },
        2 => Surface::Sphere {
            frame: frame(at),
            radius: 0.45,
        },
        3 => Surface::Cone {
            frame: frame(Point3::new(0.5, 0.0, -0.8)),
            half_angle: 0.4,
        },
        _ => Surface::Torus {
            frame: frame(at),
            major_radius: 0.35,
            minor_radius: 0.12,
        },
    }
}

/// A clamped cubic over `[0, 1]` with `n` control points, `x` rising
/// across `[0, 1]`, `y` and `z` drawn within `scale`, rational where
/// `rational`.
fn carrier(rng: &mut fuzz::Rng, n: usize, rational: bool, scale: f64) -> NurbsCurve3<f64> {
    let p = 3;
    let mut knots = vec![0.0; p + 1];
    knots.extend((1..n - p).map(|k| k as f64 / (n - p) as f64));
    knots.extend([1.0; 4]);
    let control = (0..n)
        .map(|i| {
            Point3::new(
                i as f64 / (n - 1) as f64,
                rng.range(-scale, scale),
                rng.range(-scale, scale),
            )
        })
        .collect();
    let weights = (0..n)
        .map(|_| if rational { rng.range(0.5, 2.0) } else { 1.0 })
        .collect();
    NurbsCurve3::new(KnotVector::clamped(knots, p).unwrap(), control, weights).unwrap()
}

/// What an answer holds: its interior crossings, and a meeting at each
/// end.
#[derive(Debug, PartialEq)]
struct Read {
    inside: Vec<f64>,
    ends: [bool; 2],
}

fn read_of<T: geom_core::Real>(
    meetings: Vec<crate::curve_face::Meeting<T>>,
    at: impl Fn(T) -> f64,
) -> Read {
    let mut read = Read {
        inside: Vec::new(),
        ends: [false; 2],
    };
    for m in meetings {
        match m.on_span {
            OnSpan::Inside { t } => read.inside.push(at(t)),
            OnSpan::End(SpanEnd::Start) => read.ends[0] = true,
            OnSpan::End(SpanEnd::Stop) => read.ends[1] = true,
        }
    }
    read
}

fn at_f64(net: &NurbsCurve3<f64>, span: (f64, f64), s: &Surface<f64>, band: Band) -> Option<Read> {
    let curve = Curve3::Nurbs(std::sync::Arc::new(net.clone()));
    let found = cross(&Span::new(&curve, span), s, band).ok()?;
    Some(read_of(found.meetings, |t| t))
}

fn at_interval(
    net: &NurbsCurve3<f64>,
    span: (f64, f64),
    s: &Surface<f64>,
    band: Band,
) -> Option<Read> {
    let lift = |x: f64| Interval::from_bounds(x, x);
    let curve = Curve3::Nurbs(std::sync::Arc::new(net.map_scalar(lift)));
    let surface = s.map_scalar(lift);
    let found = cross(
        &Span::new(&curve, (lift(span.0), lift(span.1))),
        &surface,
        band,
    )
    .ok()?;
    Some(read_of(found.meetings, |t| {
        geom::bracket_mid((geom_core::Bounds::lo(t), geom_core::Bounds::hi(t)))
    }))
}

/// The parameter runs, between dense samples, over which the implicit
/// residual changes sign with both samples clear of the surface.
fn clear_sign_changes(
    net: &NurbsCurve3<f64>,
    span: (f64, f64),
    s: &Surface<f64>,
    band: Band,
) -> Vec<(f64, f64)> {
    const SAMPLES: usize = 2000;
    let clear = 100.0 * band.escalate();
    let at = |k: usize| span.0 + (span.1 - span.0) * k as f64 / SAMPLES as f64;
    let r = |t: f64| crate::implicit::implicit_residual(s, net.eval(t));
    (0..SAMPLES)
        .filter_map(|k| {
            let (a, b) = (at(k), at(k + 1));
            let (ra, rb) = (r(a), r(b));
            (ra.abs() > clear && rb.abs() > clear && (ra > 0.0) != (rb > 0.0)).then_some((a, b))
        })
        .collect()
}

/// Whether `read` holds a meeting in `[a, b]` of the span `span`.
fn holds(read: &Read, span: (f64, f64), (a, b): (f64, f64)) -> bool {
    read.inside.iter().any(|&t| a <= t && t <= b)
        || read.ends[0] && a <= span.0
        || read.ends[1] && span.1 <= b
}

#[test]
fn the_ring_composite_agrees_across_scalars_and_meets_every_clear_sign_change() {
    let mut rng = fuzz::start("ring composite scalars and sampling");
    let mut bad = Vec::new();
    let mut answered = 0;
    let cases = fuzz::scaled(60);
    for case in 0..cases {
        let eps = [1e-6, 1e-9, 1e-12][rng.below(3)];
        let band = Band::new(eps, 10.0 * eps).unwrap();
        let s = surface(case);
        let rational = rng.below(3) == 0;
        let scale = [0.5, 0.2, 0.05][rng.below(3)];
        let count = 4 + rng.below(5);
        let net = carrier(&mut rng, count, rational, scale);
        let span = if rng.below(2) == 0 {
            (0.0, 1.0)
        } else {
            let a = rng.range(0.0, 0.4);
            (a, rng.range(a + 0.2, 1.0))
        };
        let label = format!("case {case} eps {eps:e} form {} span {span:?}", case % 5);
        let (real, enclosed) = (
            at_f64(&net, span, &s, band),
            at_interval(&net, span, &s, band),
        );
        if let (Some(x), Some(y)) = (&real, &enclosed) {
            let same = x.ends == y.ends
                && x.inside.len() == y.inside.len()
                && x.inside
                    .iter()
                    .zip(&y.inside)
                    .all(|(p, q)| (p - q).abs() <= 1e-6);
            if !same {
                bad.push(format!("{label}: f64 {x:?}, Interval {y:?}"));
            }
        }
        let changes = clear_sign_changes(&net, span, &s, band);
        for (scalar, answer) in [("f64", &real), ("Interval", &enclosed)] {
            let Some(answer) = answer else { continue };
            answered += 1;
            if let Some(run) = changes.iter().find(|&&run| !holds(answer, span, run)) {
                bad.push(format!(
                    "{label}: {scalar} answers {answer:?} with no meeting over the clear sign change {run:?}"
                ));
            }
        }
    }
    assert!(
        answered >= cases,
        "fewer answers than cases ({answered} of {cases}, {})",
        fuzz::replay()
    );
    assert!(
        bad.is_empty(),
        "{} disagreements ({}):\n{}",
        bad.len(),
        fuzz::replay(),
        bad.iter().take(6).cloned().collect::<Vec<_>>().join("\n")
    );
}
