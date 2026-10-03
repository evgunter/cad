//! Fitting-stack acceptance (M5 PR 4 spec §c): interpolation hits the
//! data, the approximation loop honors its tolerance with a certified
//! achieved bound, the D9 bit-replay pin (same data twice ⇒ identical
//! structure and bits), and the 2-D pcurve lane riding the same
//! generic core.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use geom::{NurbsCurve2, NurbsCurve3};
use geom_core::{Point2, Point3};

const R: f64 = 2.5;

fn arc_samples(n: usize) -> Vec<Point3<f64>> {
    (0..n)
        .map(|k| {
            #[allow(clippy::cast_precision_loss)]
            let theta = core::f64::consts::FRAC_PI_2 * (k as f64) / ((n - 1) as f64);
            let (s, c) = theta.sin_cos();
            Point3::new(0.5 + R * c, 1.0 + R * s, -0.25)
        })
        .collect()
}

/// Chord parameters recomputed the fit's way, for hitting the data at
/// its own collocation sites.
fn chord_params(pts: &[Point3<f64>]) -> Vec<f64> {
    let mut chords = Vec::new();
    let mut total = 0.0;
    for pair in pts.windows(2) {
        let d = (pair[1] - pair[0]).norm();
        chords.push(d);
        total += d;
    }
    let mut params = vec![0.0];
    let mut acc = 0.0;
    for d in &chords {
        acc += d / total;
        params.push(acc);
    }
    params[pts.len() - 1] = 1.0;
    params
}

#[test]
fn interpolation_passes_through_every_sample() {
    let pts = arc_samples(17);
    let params = chord_params(&pts);
    let curve = NurbsCurve3::interpolate(&pts, 3).unwrap();
    for (q, u) in pts.iter().zip(params.iter()) {
        let d = curve.eval(*u).distance(*q);
        assert!(d < 1e-11, "|C(ū) − Q| = {d:e}");
    }
    // Between samples the cubic interpolant of a circle stays close to
    // the locus (h⁴ scale) — sanity, not a certificate.
    for k in 0..=256 {
        let t = f64::from(k) / 256.0;
        let p = curve.eval(t);
        let dist = ((p.x - 0.5).powi(2) + (p.y - 1.0).powi(2)).sqrt();
        assert!(
            (dist - R).abs() < 1e-5,
            "off-locus by {:e}",
            (dist - R).abs()
        );
    }
}

#[test]
fn bit_replay_fitting_twice_is_identical_structure_and_bits() {
    let pts = arc_samples(33);
    let a = NurbsCurve3::approximate(&pts, 3, 1e-2).unwrap();
    let b = NurbsCurve3::approximate(&pts, 3, 1e-2).unwrap();
    assert_eq!(a.bound.to_bits(), b.bound.to_bits());
    assert_eq!(a.refit_applied, b.refit_applied);
    assert_eq!(a.curve.degree(), b.curve.degree());
    assert_eq!(a.curve.knots().knots().len(), b.curve.knots().knots().len());
    for (ka, kb) in a
        .curve
        .knots()
        .knots()
        .iter()
        .zip(b.curve.knots().knots().iter())
    {
        assert_eq!(ka.to_bits(), kb.to_bits());
    }
    for (pa, pb) in a.curve.control().iter().zip(b.curve.control().iter()) {
        assert_eq!(pa.x.to_bits(), pb.x.to_bits());
        assert_eq!(pa.y.to_bits(), pb.y.to_bits());
        assert_eq!(pa.z.to_bits(), pb.z.to_bits());
    }
    for (wa, wb) in a.curve.weights().iter().zip(b.curve.weights().iter()) {
        assert_eq!(wa.to_bits(), wb.to_bits());
    }
}

#[test]
fn approximation_respects_tolerance_and_compresses() {
    let pts = arc_samples(65);
    let tol = 1e-2;
    let fit = NurbsCurve3::approximate(&pts, 3, tol).unwrap();
    assert!(
        fit.bound <= tol,
        "achieved {:e} > tolerance {tol:e}",
        fit.bound
    );
    assert!(
        fit.bound > 0.0,
        "no removal accepted at a coarse tolerance?"
    );
    // The Type-2 loop's point: far fewer control points than samples.
    assert!(
        fit.curve.knots().control_count() < pts.len() / 2,
        "no compression: {} control points from {} samples",
        fit.curve.knots().control_count(),
        pts.len()
    );
    // The certified bound really does bound the data deviation.
    let params = chord_params(&pts);
    let slack = 1e-12; // fp of the steering-grade bound arithmetic
    for (q, u) in pts.iter().zip(params.iter()) {
        let d = fit.curve.eval(*u).distance(*q);
        assert!(
            d <= fit.bound + slack,
            "data deviation {d:e} exceeds achieved bound {:e}",
            fit.bound
        );
    }
}

#[test]
fn tighter_tolerance_yields_tighter_bound_and_more_structure() {
    let pts = arc_samples(65);
    let coarse = NurbsCurve3::approximate(&pts, 3, 1e-2).unwrap();
    let fine = NurbsCurve3::approximate(&pts, 3, 1e-6).unwrap();
    assert!(fine.bound <= 1e-6 && coarse.bound <= 1e-2);
    assert!(coarse.bound > 1e-3, "coarse loop accepted nothing?");
    assert!(
        fine.curve.knots().control_count() >= coarse.curve.knots().control_count(),
        "tighter tolerance kept fewer control points"
    );
}

#[test]
fn pcurve_2d_fitting_rides_the_same_machinery() {
    // A 2-D parabola-ish sample set (the pcurve shape: parameter-space
    // data).
    let pts: Vec<Point2<f64>> = (0..33)
        .map(|k| {
            let u = f64::from(k) / 32.0;
            Point2::new(u, 0.25 + u * (1.0 - u))
        })
        .collect();
    let curve = NurbsCurve2::interpolate(&pts, 3).unwrap();
    let params = chord_params2(&pts);
    for (q, u) in pts.iter().zip(params.iter()) {
        let p = curve.eval(*u);
        let d = ((p.x - q.x).powi(2) + (p.y - q.y).powi(2)).sqrt();
        assert!(d < 1e-11);
    }
    let fit = NurbsCurve2::approximate(&pts, 3, 1e-2).unwrap();
    assert!(fit.bound <= 1e-2);
    assert!(fit.curve.knots().control_count() < pts.len());
}

fn chord_params2(pts: &[Point2<f64>]) -> Vec<f64> {
    let mut total = 0.0;
    let mut chords = Vec::new();
    for pair in pts.windows(2) {
        let d = (pair[1] - pair[0]).norm();
        chords.push(d);
        total += d;
    }
    let mut params = vec![0.0];
    let mut acc = 0.0;
    for d in &chords {
        acc += d / total;
        params.push(acc);
    }
    params[pts.len() - 1] = 1.0;
    params
}
