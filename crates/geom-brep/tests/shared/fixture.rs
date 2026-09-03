//! Exact rational NURBS carriers that more than one suite in this
//! crate builds, and the two clamped single-span knot vectors they
//! stand on.
//!
//! **What earns a place here.** A carrier belongs here when two suites
//! were building the SAME net — the same control points in the same
//! order with the same weights — and only spelling separated them. It
//! does NOT belong here when a suite's claim is that its carrier is one
//! the thing under test never saw: `cert7_r1_probes.rs`'s elliptic
//! wall, `cert7_r2_probes.rs`'s ellipse-of-revolution wall,
//! `offb_r2_probes.rs`'s skinned loft and `offset_fit.rs`'s bumpy patch
//! all stay where they are used, and the two reviewer rebuilds of the
//! quarter cylinder in `cert10_r1_probes.rs` and `cert10r2_probes.rs`
//! stay too, each saying at its own site why.

// The same allow every suite in this directory carries: these
// constructors are handed literal knot vectors and control nets that
// are valid by construction, and an `unwrap` that fires here is a
// broken fixture, which is exactly what a test wants to hear about
// loudly.
#![allow(clippy::unwrap_used)]

use core::f64::consts::FRAC_PI_2;

use geom::NurbsSurface;
use geom_core::Point3;
use geom_core::spline::KnotVector;

/// The clamped degree-2 single-Bézier knot vector.
pub(crate) fn kv2() -> KnotVector {
    KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap()
}

/// The clamped degree-1 single-span knot vector.
pub(crate) fn kv1() -> KnotVector {
    KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap()
}

/// The classical rational-quadratic arc weight for a sweep of
/// `sweep` radians: `cos(sweep/2)` (a quarter turn gives `√2/2`).
pub(crate) fn arc_weight(sweep: f64) -> f64 {
    (sweep * 0.5).cos()
}

/// A quarter cylinder of radius `r` and height `h` about `+z`, exact:
/// the u direction is the rational quadratic quarter circle
/// (weights `1, √2/2, 1` — the classical exact arc), the v direction
/// a linear translation.
pub(crate) fn quarter_cylinder(r: f64, h: f64) -> NurbsSurface<f64> {
    let s = arc_weight(FRAC_PI_2);
    let control = vec![
        Point3::new(r, 0.0, 0.0),
        Point3::new(r, 0.0, h),
        Point3::new(r, r, 0.0),
        Point3::new(r, r, h),
        Point3::new(0.0, r, 0.0),
        Point3::new(0.0, r, h),
    ];
    let weights = vec![1.0, 1.0, s, s, 1.0, 1.0];
    NurbsSurface::new(kv2(), kv1(), control, weights).unwrap()
}

/// A sphere band of radius `r` between latitudes `lat0` and `lat1`,
/// swept a quarter turn in longitude — exact: a rational quadratic
/// meridian arc revolved through the classical rational quadratic
/// quarter turn (A8.1's weight product), with no control row on the
/// axis, so the chart normal is regular everywhere on it.
pub(crate) fn sphere_band(r: f64, lat0: f64, lat1: f64) -> NurbsSurface<f64> {
    // Meridian: a rational quadratic arc through the sweep
    // `lat1 − lat0`, in the (x, z) half-plane.
    let theta = 0.5 * (lat1 - lat0);
    let wm = theta.cos();
    let a = (r * lat0.cos(), r * lat0.sin());
    let b = (r * lat1.cos(), r * lat1.sin());
    // The tangent-intersection control point: the midpoint direction
    // at radius `r / cos θ`.
    let mid = (a.0 + b.0, a.1 + b.1);
    let mlen = (mid.0 * mid.0 + mid.1 * mid.1).sqrt();
    let m = (mid.0 / mlen * r / wm, mid.1 / mlen * r / wm);
    let meridian = [(a.0, a.1, 1.0), (m.0, m.1, wm), (b.0, b.1, 1.0)];
    // Revolve a quarter turn about `+z` (A8.1): the row is
    // `(x, 0, z), (x, x, z), (0, x, z)` with weights
    // `w, w·cos45, w`.
    let wr = arc_weight(FRAC_PI_2);
    let mut control = Vec::with_capacity(9);
    let mut weights = Vec::with_capacity(9);
    for iu in 0..3 {
        for (x, z, w) in meridian {
            control.push(match iu {
                0 => Point3::new(x, 0.0, z),
                1 => Point3::new(x, x, z),
                _ => Point3::new(0.0, x, z),
            });
            weights.push(if iu == 1 { w * wr } else { w });
        }
    }
    NurbsSurface::new(kv2(), kv2(), control, weights).unwrap()
}
