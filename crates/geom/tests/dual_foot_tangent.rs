//! **A characterization suite: it pins what the projection doors do
//! WRONG at `Dual64`, and it is meant to go red.**
//!
//! `crates/geom/src/projection.rs`'s `mid` freezes the Newton foot
//! parameter as `f64`, so at a dual scalar every quantity built on `t*`
//! is differentiated at fixed `t*` and loses its `dt*/dp` term. The two
//! `Projection` types state which field that reaches; **issue #874**
//! carries the defect and its dispositions. Until one is chosen, the
//! claim in those docs is prose, and prose is what this file converts
//! into something a merge can falsify.
//!
//! **The day #874 is fixed, the rows marked WRONG below fail.** That is
//! the point: the fixer gets a red that names the issue, rather than a
//! silent behaviour change under a doc comment nobody re-reads. The same
//! flipped-pin shape `geom-core`'s decoration rows use.
//!
//! It lives directly under `tests/` rather than in `tests/curves/` or
//! `tests/surfaces/` because its subject is the shared interior
//! `projection` module, and because the two halves share one defect —
//! split across the two group directories they would drift apart.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{NurbsCurve3, NurbsSurface};
use geom_core::spline::KnotVector;
use geom_core::{Dual64, Point3, Real};

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

/// The query point is FIXED while the carrier slides, so the true
/// derivatives are exactly derivable by hand: the perpendicular foot of
/// `(1, 1, 0)` onto a line along x stays at `x = 1` whatever `s` is, and
/// the orthogonality residual is identically zero. **Both true values
/// are `0`; both reported tangents are not.**
#[test]
fn curve_projection_foot_and_orthogonality_tangents_are_wrong_at_dual() {
    let curve = sliding_line::<Dual64>(Dual64::new(0.0, 1.0));
    let q = Point3::new(
        Dual64::constant(1.0),
        Dual64::constant(1.0),
        Dual64::constant(0.0),
    );
    let r = curve.project(q).unwrap();

    // The VALUE channel is the f64 run's, bit for bit (D9). This half
    // must never move, #874 or not.
    let plain = sliding_line::<f64>(0.0)
        .project(Point3::new(1.0, 1.0, 0.0))
        .unwrap();
    assert_eq!(r.t.to_bits(), plain.t.to_bits(), "D9: t is the f64 run's");
    assert_eq!(r.foot.x.value.to_bits(), plain.foot.x.to_bits());
    assert_eq!(r.distance.value.to_bits(), plain.distance.to_bits());
    assert_eq!(
        r.orthogonality.value.to_bits(),
        plain.orthogonality.to_bits()
    );

    // WRONG (#874): true d(foot.x)/ds = 0, because the foot of a fixed
    // point on a sliding line does not move. Reported as 1 — the whole
    // control net's motion, with none of the compensating dt*/ds.
    assert_eq!(
        r.foot.x.deriv.to_bits(),
        1.0f64.to_bits(),
        "#874 fixed? the reported foot tangent is no longer the frozen-t* partial — \
         update this suite and the Projection docs together"
    );

    // WRONG (#874): true d(orthogonality)/ds = 0 — the residual is
    // identically zero as s varies. Reported as 4.
    assert_eq!(
        r.orthogonality.deriv.to_bits(),
        4.0f64.to_bits(),
        "#874 fixed? the reported orthogonality tangent is no longer the frozen-t* partial"
    );

    // RIGHT, and non-obviously so: distance's dropped coefficient is the
    // cosine acceptance quantity, which is zero at this exactly-orthogonal
    // foot. It is bounded only on the cosine exit and only to within ε₂
    // (Projection3's docs) — this fixture takes that exit.
    assert_eq!(r.distance.deriv.to_bits(), 0.0f64.to_bits());
}

/// The surface twin. Same defect, same shape, two parameters.
#[test]
fn surface_projection_foot_and_orthogonality_tangents_are_wrong_at_dual() {
    let surface = sliding_patch::<Dual64>(Dual64::new(0.0, 1.0));
    let q = Point3::new(
        Dual64::constant(1.0),
        Dual64::constant(0.5),
        Dual64::constant(1.0),
    );
    let r = surface.project(q).unwrap();

    let plain = sliding_patch::<f64>(0.0)
        .project(Point3::new(1.0, 0.5, 1.0))
        .unwrap();
    assert_eq!(r.u.to_bits(), plain.u.to_bits(), "D9: u is the f64 run's");
    assert_eq!(r.v.to_bits(), plain.v.to_bits(), "D9: v is the f64 run's");
    assert_eq!(r.foot.x.value.to_bits(), plain.foot.x.to_bits());
    assert_eq!(r.distance.value.to_bits(), plain.distance.to_bits());

    // WRONG (#874), for the curve half's reasons in two parameters.
    assert_eq!(
        r.foot.x.deriv.to_bits(),
        1.0f64.to_bits(),
        "#874 fixed? the reported foot tangent is no longer the frozen-(u*, v*) partial"
    );
    assert_eq!(
        r.orthogonality_u.deriv.to_bits(),
        4.0f64.to_bits(),
        "#874 fixed? the reported u-orthogonality tangent is no longer the frozen partial"
    );
    assert_eq!(r.distance.deriv.to_bits(), 0.0f64.to_bits());
}
