//! **The ground truth #874 is measured against, and the record of what
//! `Dual64` reported before the doors stopped admitting it.**
//!
//! `crates/geom/src/projection.rs`'s `mid` freezes the Newton foot
//! parameter as `f64`, so a dual scalar differentiates every quantity
//! built on `t*` at fixed `t*` and loses its `dt*/dp` term. The two
//! `Projection` types state which field that reaches and by how much;
//! **issue #874** carries the defect and its dispositions.
//!
//! **The doors no longer admit a dual.** `::project` and
//! `::project_from_seed` bound `Decide + CertifiedBounds`, so the wrong
//! tangent is unreachable rather than fixed. The eviction is pinned by
//! `compile_fail` rows in `crates/geom/src/projection.rs`'s module docs
//! — a doctest is only collected from a `lib` target, so it cannot live
//! in this file.
//!
//! **What this file still executes is the half that matters to a
//! fixer**: the TRUE derivatives, measured by central difference at
//! `f64`, on the same two fixtures. They are the numbers any #874 fix
//! must reproduce, and they do not need a dual to measure.
//!
//! **What `Dual64` reported, recorded because it can no longer be run
//! here** (measured 2026-08-21, before the tightening; the fixtures
//! below are the same ones):
//!
//! | quantity | true | `Dual64` reported |
//! |---|---|---|
//! | `d(foot.x)/ds`, curve and surface | `0` | `1` |
//! | `d(orthogonality)/ds`, curve; `d(orthogonality_u)/ds`, surface | `0` | `4` |
//! | `d(distance)/ds` | `0` | `0` — right, and only on the cosine exit |
//!
//! **A fixer loses the red-test workflow.** To reproduce those numbers
//! now, loosen the two doors' bound back to `T: Bounds` locally; that
//! is the first step of fixing #874, not an obstacle to it.
//!
//! It lives directly under `tests/` rather than in `tests/curves/` or
//! `tests/surfaces/` because its subject is the shared interior
//! `projection` module, and because the two halves share one defect —
//! split across the two group directories they would drift apart.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{NurbsCurve3, NurbsSurface};
use geom_core::spline::KnotVector;
use geom_core::{Point3, Real};

/// Degree-1 line from `(s, 0, 0)` to `(4 + s, 0, 0)` — the whole curve
/// slides along x with the seeded parameter `s`.
fn sliding_line<T: Real>(shift: T) -> NurbsCurve3<T> {
    let control = vec![
        Point3::new(shift, T::from_f64(0.0), T::from_f64(0.0)),
        Point3::new(T::from_f64(4.0) + shift, T::from_f64(0.0), T::from_f64(0.0)),
    ];
    let knots = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    NurbsCurve3::new(knots, control, vec![1.0, 1.0]).unwrap()
}

/// The bilinear patch in `z = 0` over `x ∈ [s, 4 + s]`, `y ∈ [-2, 2]` —
/// the surface twin of [`sliding_line`], sliding the same way.
fn sliding_patch<T: Real>(shift: T) -> NurbsSurface<T> {
    let z = T::from_f64(0.0);
    let p = |x: f64, y: f64| Point3::new(T::from_f64(x) + shift, T::from_f64(y), z);
    let control = vec![p(0.0, -2.0), p(0.0, 2.0), p(4.0, -2.0), p(4.0, 2.0)];
    let ku = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    NurbsSurface::new(ku, kv, control, vec![1.0, 1.0, 1.0, 1.0]).unwrap()
}

/// The step of the central differences below. Large enough that the
/// difference of two feet is not lost to cancellation, small enough
/// that the second-order term is far under the tolerances asserted.
const H: f64 = 1e-6;

/// The query point is FIXED while the carrier slides, so the true
/// derivatives are exactly derivable by hand: the perpendicular foot of
/// `(1, 1, 0)` onto a line along x stays at `x = 1` whatever `s` is, and
/// the orthogonality residual is identically zero. **Both true values
/// are `0`** — which is what makes the reported tangents in the module
/// header wrong, and what a #874 fix has to produce.
#[test]
fn the_true_curve_foot_and_orthogonality_derivatives_are_zero() {
    let at = |s: f64| {
        sliding_line::<f64>(s)
            .project(Point3::new(1.0, 1.0, 0.0))
            .unwrap()
    };
    let (lo, hi) = (at(-H), at(H));
    let d = |f: fn(&geom::Projection3<f64>) -> f64| (f(&hi) - f(&lo)) / (2.0 * H);

    assert!(
        d(|r| r.foot.x).abs() < 1e-6,
        "the foot of a fixed point on a sliding line does not move: got {}",
        d(|r| r.foot.x)
    );
    assert!(
        d(|r| r.orthogonality).abs() < 1e-6,
        "the orthogonality residual is identically zero as s varies: got {}",
        d(|r| r.orthogonality)
    );
    assert!(
        d(|r| r.distance).abs() < 1e-6,
        "the distance from a fixed point to a sliding line is stationary here: got {}",
        d(|r| r.distance)
    );
    // The foot really is at x = 1 and the residual really is zero, so
    // the derivatives above are zeros of a constant, not of noise.
    let mid = at(0.0);
    assert!((mid.foot.x - 1.0).abs() < 1e-9);
    assert!(mid.orthogonality.abs() < 1e-9);
}

/// The surface twin. Same fixture, same true derivatives, two
/// parameters.
#[test]
fn the_true_surface_foot_and_orthogonality_derivatives_are_zero() {
    let at = |s: f64| {
        sliding_patch::<f64>(s)
            .project(Point3::new(1.0, 0.5, 1.0))
            .unwrap()
    };
    let (lo, hi) = (at(-H), at(H));
    let d = |f: fn(&geom::SurfaceProjection<f64>) -> f64| (f(&hi) - f(&lo)) / (2.0 * H);

    assert!(d(|r| r.foot.x).abs() < 1e-6, "got {}", d(|r| r.foot.x));
    assert!(
        d(|r| r.orthogonality_u).abs() < 1e-6,
        "got {}",
        d(|r| r.orthogonality_u)
    );
    assert!(d(|r| r.distance).abs() < 1e-6, "got {}", d(|r| r.distance));

    let mid = at(0.0);
    assert!((mid.foot.x - 1.0).abs() < 1e-9);
    assert!(mid.orthogonality_u.abs() < 1e-9);
}
