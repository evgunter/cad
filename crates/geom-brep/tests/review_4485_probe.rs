//! Review probes for PR 4485 (not for merge).
use geom_core::{Bounds, interval::Interval};
use geom_core::interval::certification::Certification;
use geom_core::spline::KnotVector;

/// A degree-2 channel with a multiplicity-2 interior knot has a
/// DISCONTINUOUS first derivative, so its ladder's first level is
/// `Level::Coeffs` and orders 2 and 3 read as in-span zeros — but
/// `u'' = 8` on the second span. `∫ u·v' dt` with `v = t` is `1/6`.
#[test]
fn a_discontinuous_first_derivative_does_not_zero_the_second() {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let u = [0.0, 0.0, 0.0, 0.0, 1.0].map(Interval::point);
    let v = [0.0, 1.0].map(Interval::point);
    let (u, v) = (ku.with_coeffs(&u).unwrap(), kv.with_coeffs(&v).unwrap());
    let out = geom_brep::props::quad::bspline_green_integral(u, v, 0.0, 1.0, 64).unwrap();
    let truth = 1.0 / 6.0;
    assert!(
        out.lo() <= truth && truth <= out.hi(),
        "{:?} must bracket 1/6",
        (out.lo(), out.hi())
    );
}
