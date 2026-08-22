//! **Boundary iso-curve extraction** for tensor-product NURBS
//! surfaces (M6-3, the loft/sweep assembly's seam substrate).
//!
//! A *clamped* surface's boundary iso-curves are **rows of its own
//! control net, verbatim**: `S(0, v)` is the spline over `knots_v`
//! whose control points are the first u-row (`P[0·nv + j]`) with the
//! matching weights, and likewise for `u = 1` (last u-row) and
//! `v = 0` / `v = 1` (first/last v-columns) over `knots_u`. No
//! arithmetic is performed — extraction is a **copy**, so the
//! extracted curve is exact structure (C6), not an approximation, and
//! two walls that share a control row share the curve bit for bit.
//!
//! Interior iso-curves (`u = c`, `0 < c < 1`) need a de Boor collapse
//! and are deliberately NOT here: nothing in the loft assembly mints
//! one (walls meet at their u-boundaries by construction), and an
//! unconsumed general extractor would be untested machinery. The
//! function that first needs it brings it.
//!
//! # Why this lives in `geom-brep` and not beside the payloads
//!
//! **Iso-curve extraction belongs to the EdgeGeometry layer, not to
//! the evaluator layer**, and that is a placement rule rather than an
//! accident of which crate the types used to sit in. Extraction is the
//! step that turns one entity's data into *another entity's carrier*:
//! its whole purpose is to hand a curve to an edge, which is what this
//! layer is for. `geom` answers "what is this locus and what does it
//! evaluate to"; it does not know that a surface row is about to
//! become an edge's geometry, and giving it a door that produces
//! carriers would make the evaluator layer aware of the B-rep above
//! it.

use geom::NurbsCurve3;
use geom::NurbsSurface;
use geom_core::Real;
use geom_core::spline::SplineError;

/// The `u = 0` (`end = false`) or `u = 1` (`end = true`) boundary
/// iso-curve of a clamped surface: the first/last u-row of the control
/// net over `knots_v`, weights matching.
///
/// # Errors
///
/// [`SplineError`] — unreachable for a surface that already validated
/// (the row's counts match `knots_v` by the surface's own
/// construction), surfaced rather than swallowed (D4 ¶2).
pub fn boundary_iso_u<T: Real>(
    s: &NurbsSurface<T>,
    end: bool,
) -> Result<NurbsCurve3<T>, SplineError> {
    let (nu, nv) = s.control_counts();
    let base = if end { (nu - 1) * nv } else { 0 };
    let control = s.control()[base..base + nv].to_vec();
    let weights = s.weights()[base..base + nv].to_vec();
    NurbsCurve3::new(s.knots_v().clone(), control, weights)
}

/// The `v = 0` (`end = false`) or `v = 1` (`end = true`) boundary
/// iso-curve of a clamped surface: the first/last v-column of the
/// control net over `knots_u`, weights matching.
///
/// # Errors
///
/// [`SplineError`] — unreachable as [`boundary_iso_u`]'s.
pub fn boundary_iso_v<T: Real>(
    s: &NurbsSurface<T>,
    end: bool,
) -> Result<NurbsCurve3<T>, SplineError> {
    let (nu, nv) = s.control_counts();
    let offset = if end { nv - 1 } else { 0 };
    let mut control = Vec::with_capacity(nu);
    let mut weights = Vec::with_capacity(nu);
    for iu in 0..nu {
        control.push(s.control()[iu * nv + offset]);
        weights.push(s.weights()[iu * nv + offset]);
    }
    NurbsCurve3::new(s.knots_u().clone(), control, weights)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::Point3;
    use geom_core::spline::KnotVector;

    use super::*;

    /// A 3×2 (u×v) bilinear-ish surface: boundary extraction matches
    /// dense evaluation along each boundary.
    #[test]
    fn boundary_isos_match_surface_evaluation() {
        let ku = KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap();
        let kv = KnotVector::unit_segment(1);
        // Row-major iu·nv + iv, nv = 2.
        let control = vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.5),
            Point3::new(1.0, 0.0, 0.2),
            Point3::new(1.0, 1.0, 0.9),
            Point3::new(2.0, 0.0, -0.3),
            Point3::new(2.0, 1.0, 0.1),
        ];
        let s = NurbsSurface::<f64>::new(ku, kv, control, vec![1.0; 6]).unwrap();
        let u0 = boundary_iso_u(&s, false).unwrap();
        let u1 = boundary_iso_u(&s, true).unwrap();
        let v0 = boundary_iso_v(&s, false).unwrap();
        let v1 = boundary_iso_v(&s, true).unwrap();
        for i in 0..=8 {
            let t = f64::from(i) / 8.0;
            assert!(u0.eval(t).distance(s.eval(0.0, t)) < 1e-15);
            assert!(u1.eval(t).distance(s.eval(1.0, t)) < 1e-15);
            assert!(v0.eval(t).distance(s.eval(t, 0.0)) < 1e-15);
            assert!(v1.eval(t).distance(s.eval(t, 1.0)) < 1e-15);
        }
    }
}
