//! M5 PR 9 (C12.3): `split_at` — NURBS carrier splitting by knot
//! insertion. The children must reproduce the parent exactly (in ℝ;
//! at f64, to roundoff) over their halves, keep the parent's
//! parameter, and refuse boundary/out-of-domain splits typed.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::NurbsCurve3;
use geom_core::Point3;
use geom_core::spline::KnotVector;

/// The gentle cubic the split-meter suite uses — the shape a fitted
/// SSI carrier has.
fn fitted_like_carrier() -> NurbsCurve3<f64> {
    let pts: Vec<Point3<f64>> = (0..=30)
        .map(|i| {
            let t = f64::from(i) / 30.0;
            Point3::new(t, 0.1 * (t * 4.0).sin(), 0.02 * t)
        })
        .collect();
    NurbsCurve3::<f64>::interpolate(&pts, 3).unwrap()
}

#[test]
fn children_reproduce_the_parent_on_their_halves() {
    let c = fitted_like_carrier();
    let (d0, d1) = c.domain();
    let u = d0 + (d1 - d0) * 0.37;
    let (a, b) = c.split_at(u).unwrap();
    assert_eq!(a.domain(), (d0, u), "child 1 keeps the parent parameter");
    assert_eq!(b.domain(), (u, d1), "child 2 keeps the parent parameter");
    for i in 0..=400 {
        let t = d0 + (d1 - d0) * f64::from(i) / 400.0;
        let p = c.eval(t);
        let q = if t <= u { a.eval(t) } else { b.eval(t) };
        assert!(
            p.distance(q) < 1e-12,
            "child deviates from parent at t = {t}: {:e}",
            p.distance(q)
        );
    }
    // The shared point is the split point, both sides.
    assert!(a.eval(u).distance(c.eval(u)) < 1e-13);
    assert!(b.eval(u).distance(c.eval(u)) < 1e-13);
}

#[test]
fn rational_split_is_evaluation_invariant() {
    // A rational quadratic (weights ≠ 1): the projective insertion
    // path must carry the weights, not just the points.
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let control = vec![
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(1.0, 1.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
    ];
    let w = std::f64::consts::FRAC_1_SQRT_2;
    let c = NurbsCurve3::<f64>::new(kv, control, vec![1.0, w, 1.0]).unwrap();
    let (a, b) = c.split_at(0.5).unwrap();
    for i in 0..=200 {
        let t = f64::from(i) / 200.0;
        let p = c.eval(t);
        let q = if t <= 0.5 { a.eval(t) } else { b.eval(t) };
        assert!(p.distance(q) < 1e-13, "rational split deviates at t = {t}");
        // The exact circle stays exact through the split.
        let r = (q.x * q.x + q.y * q.y).sqrt();
        assert!((r - 1.0).abs() < 1e-13, "off the circle at t = {t}: {r}");
    }
}

#[test]
fn split_at_an_existing_knot_and_repeated_splits() {
    let c = fitted_like_carrier();
    let (d0, d1) = c.domain();
    // A knot value strictly inside the domain (the interpolant has
    // interior knots at the chord parameters).
    let interior = c
        .knots()
        .knots()
        .iter()
        .copied()
        .find(|k| *k > d0 && *k < d1)
        .expect("an interpolated cubic has interior knots");
    let (a, b) = c.split_at(interior).unwrap();
    assert_eq!(a.domain().1, interior);
    // Splitting a child again still reproduces the parent.
    let (u0, u1) = b.domain();
    let mid = u0 + (u1 - u0) * 0.5;
    let (b1, b2) = b.split_at(mid).unwrap();
    for i in 0..=100 {
        let t = u0 + (u1 - u0) * f64::from(i) / 100.0;
        let q = if t <= mid { b1.eval(t) } else { b2.eval(t) };
        assert!(c.eval(t).distance(q) < 1e-12);
    }
}

#[test]
fn boundary_and_outside_splits_refuse_typed() {
    let c = fitted_like_carrier();
    let (d0, d1) = c.domain();
    for u in [d0, d1, d0 - 1.0, d1 + 1.0, f64::NAN] {
        let err = c.split_at(u);
        assert!(err.is_err(), "split at {u} must refuse");
    }
}
