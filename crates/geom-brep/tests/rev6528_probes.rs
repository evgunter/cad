//! Review probes for PR 6528 (the arc rim at unequal end weights).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::tol::band;
use geom::Curve3;
use geom::{NurbsSurface, Surface};
use geom_brep::{Pcurve, PcurveCache, pick_arc_rim};
use geom_core::spline::{KnotVector, Knots};
use geom_core::{Interval, Point3, Real, Vec3};

const R: f64 = 0.005;
const H: f64 = 0.01;

fn rim<T: Real>() -> Curve3<T> {
    let f = T::from_f64;
    Curve3::circle(
        Point3::new(f(0.0), f(0.0), f(0.0)),
        Vec3::new(f(0.0), f(0.0), f(1.0)),
        f(R),
        Vec3::new(f(1.0), f(0.0), f(0.0)),
    )
}

/// A one-span quarter wall on `u ∈ [0, 1]` with row weights `w`.
fn quarter<T: geom_core::spline::SpanLocate>(w: [T; 3]) -> Surface<T> {
    let f = T::from_f64;
    let mut control = Vec::new();
    let mut weights = Vec::new();
    for ((x, y), wk) in [(R, 0.0), (R, R), (0.0, R)].into_iter().zip(w) {
        for v in [0.0, H] {
            control.push(Point3::new(f(x), f(y), f(v)));
            weights.push(wk);
        }
    }
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    Surface::Nurbs(std::sync::Arc::new(
        NurbsSurface::new(Knots::from(ku).lift(f), Knots::from(kv).lift(f), control, weights).unwrap(),
    ))
}

/// A two-span half wall on `u ∈ [0, 1]` with row weights `w`.
fn half<T: geom_core::spline::SpanLocate>(w: [T; 5]) -> Surface<T> {
    let f = T::from_f64;
    let mut control = Vec::new();
    let mut weights = Vec::new();
    for ((x, y), wk) in [(R, 0.0), (R, R), (0.0, R), (-R, R), (-R, 0.0)]
        .into_iter()
        .zip(w)
    {
        for v in [0.0, H] {
            control.push(Point3::new(f(x), f(y), f(v)));
            weights.push(wk);
        }
    }
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    Surface::Nurbs(std::sync::Arc::new(
        NurbsSurface::new(Knots::from(ku).lift(f), Knots::from(kv).lift(f), control, weights).unwrap(),
    ))
}

fn verdict<T: geom_core::Decide>(wall: &Surface<T>, span: f64) -> String {
    let carrier = rim::<T>();
    let f = T::from_f64;
    match pick_arc_rim(&carrier, (f(0.0), f(span)), wall, band()) {
        Err(e) => format!("pick escalated {e:?}"),
        Ok(None) => "pick: none".into(),
        Ok(Some(image)) => {
            let ends = match &image {
                Pcurve::IsoArc { ends, .. } => ends.len(),
                _ => 99,
            };
            match PcurveCache::certify(image, f(0.0), f(span), &carrier, wall, band()) {
                Ok(c) => format!(
                    "CERTIFIED ends={ends} env={:?}",
                    c.certificate().envelope
                ),
                Err(e) => format!("refused ends={ends}: {e:?}"),
            }
        }
    }
}

/// Probe A: near-circle weights at one span (1, 1+δ, 2); the circle is
/// δ = 0. Prints verdicts; the parent compares against the true radial
/// residual computed outside.
#[test]
fn probe_a_near_circle_one_span() {
    let q = core::f64::consts::FRAC_PI_2;
    for d in [0.0, 1e-15, 1e-13, 1e-12, 1e-11, 1e-10, 1e-9, 1e-7, 1e-6, 1e-3] {
        let w = [1.0, 1.0 + d, 2.0];
        eprintln!("A δ={d:e}: {}", verdict(&quarter(w), q));
    }
    for (a, b) in [(1.0, 1e8), (1e8, 1.0), (1.0, 1e-8), (1.0, 1e15)] {
        let w1 = core::f64::consts::FRAC_1_SQRT_2 * (a * b as f64).sqrt();
        eprintln!("A λ² = {}: {}", a / b, verdict(&quarter([a, w1, b]), q));
    }
}

/// Probe B: two spans, ends (1, 2, 1), the second span's interior weight
/// off by δ (the first's at the circle).
#[test]
fn probe_b_near_circle_second_span() {
    let pi = core::f64::consts::PI;
    for d in [0.0, 1e-12, 1e-9, 1e-6, 1e-3] {
        let w = [1.0, 1.0, 2.0, 1.0 + d, 1.0];
        eprintln!("B δ={d:e}: {}", verdict(&half(w), pi));
    }
    // An unequal-ends row whose two spans carry DIFFERENT normalized
    // weights: span 1 a quarter, span 2 not (the half is not a circle).
    eprintln!(
        "B mixed: {}",
        verdict(&half([1.0, 1.0, 2.0, 1.2, 1.0]), pi)
    );
}

/// Probe C: at `Interval`. Point weights (1, 1, 2); a constant pattern
/// held at a NON-POINT scale `c`; and (1, 1, 2) scaled by `c`.
#[test]
fn probe_c_interval() {
    let q = core::f64::consts::FRAC_PI_2;
    let f = Interval::from_f64;
    let c = f(1.0) / f(3.0);
    eprintln!("C point (1,1,2): {}", verdict(&quarter([f(1.0), f(1.0), f(2.0)]), q));
    let s = f(core::f64::consts::FRAC_1_SQRT_2);
    eprintln!(
        "C constant pattern at non-point c: {}",
        verdict(&quarter([c, c * s, c]), q)
    );
    eprintln!(
        "C (1,1,2)·c: {}",
        verdict(&quarter([c, c, c * f(2.0)]), q)
    );
    // The same at f64 for comparison.
    let cf = 1.0 / 3.0;
    eprintln!(
        "C f64 constant pattern at c: {}",
        verdict(&quarter([cf, cf * core::f64::consts::FRAC_1_SQRT_2, cf]), q)
    );
}
