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
//! **Iso-curve extraction belongs to the EdgeDescription layer, not to
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
use geom_core::k_stats::decide;
use geom_core::spline::SplineError;
use geom_core::{Band, Decide, Indeterminate, Margin, Real, Sign};

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

/// Why [`iso_boundary_row`] could not hand back a row.
#[derive(Clone, Debug)]
pub enum IsoRowError<T: Real> {
    /// `u` is not either end of the surface's own `u` domain. Interior
    /// iso-curves need a de Boor collapse, which this module does not
    /// build (module docs: the function that first needs one brings
    /// it).
    Interior {
        /// The `u` asked for, echoed as data.
        u: T,
        /// The surface's `u` domain.
        domain: (f64, f64),
    },
    /// The row is not valid spline structure — unreachable for a
    /// surface that already validated, surfaced rather than swallowed
    /// (D4 ¶2).
    Structure {
        /// The spline layer's typed refusal.
        source: SplineError,
    },
    /// The domain-endpoint coincidence test escalated.
    Escalated {
        /// The predicate-layer escalation.
        source: Indeterminate,
    },
}

impl<T: Real> core::fmt::Display for IsoRowError<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Interior { u, domain } => write!(
                f,
                "iso_boundary_row: u = {u:?} is interior to the chart's u domain {domain:?} —                  only the boundary rows extract, and an interior iso-curve needs a de Boor                  collapse this module does not build"
            ),
            Self::Structure { source } => {
                write!(
                    f,
                    "iso_boundary_row: the extracted row is not valid spline structure: {source}"
                )
            }
            Self::Escalated { source } => write!(f, "iso_boundary_row escalated: {source}"),
        }
    }
}

impl<T: Real> std::error::Error for IsoRowError<T> {}

/// **The `u = const` boundary row of a chart, selected by the stored
/// parameter** — [`boundary_iso_u`] with the end decided rather than
/// passed, which is what a consumer holding an
/// an iso chart image's `u` actually has.
///
/// Returns the row together with the DOMAIN endpoint it sits at, so
/// the caller re-states the description against the chart's own float
/// rather than the one it came in with.
///
/// The coincidence `u = u₀` / `u = u₁` is a named margined decide
/// (`iso_row_at_domain_end`), not an equality: a stored parameter and a
/// stored knot are two floats, and asking whether they name the same
/// chart line is a question with a band.
///
/// # Errors
///
/// [`IsoRowError`] — an interior `u`, an escalated coincidence, or a
/// row the spline layer refuses.
pub fn iso_boundary_row<T: Decide>(
    fit: &NurbsSurface<T>,
    u: T,
    band: Band,
) -> Result<(NurbsCurve3<T>, T), IsoRowError<T>> {
    let (u0, u1) = fit.knots_u().domain();
    let at = |end: f64| -> Result<bool, IsoRowError<T>> {
        let margin = Margin::of(u - T::from_f64(end));
        let sign = decide("iso_row_at_domain_end", margin, band)
            .map_err(|source| IsoRowError::Escalated { source })?;
        Ok(sign == Sign::Zero)
    };
    let end = if at(u0)? {
        (false, u0)
    } else if at(u1)? {
        (true, u1)
    } else {
        return Err(IsoRowError::Interior {
            u,
            domain: (u0, u1),
        });
    };
    let row = boundary_iso_u(fit, end.0).map_err(|source| IsoRowError::Structure { source })?;
    Ok((row, T::from_f64(end.1)))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::Point3;
    use geom_core::spline::KnotVector;

    use super::*;

    /// [`iso_boundary_row`] selects the row the stored parameter names,
    /// hands back the chart's OWN domain float for it, and refuses an
    /// interior parameter rather than approximating one.
    ///
    /// The row is asserted against the surface, not against a constant:
    /// it carries `knots_v` verbatim, which is the whole reason a
    /// consumer extracts instead of elevating and refining its own
    /// carrier into that space.
    /// The shared 3×2 (u×v) bilinear-ish fixture.
    fn surface() -> NurbsSurface<f64> {
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
        NurbsSurface::<f64>::new(ku, kv, control, vec![1.0; 6]).unwrap()
    }

    #[test]
    fn iso_boundary_row_selects_by_parameter_and_refuses_the_interior() {
        let s = surface();
        let band = geom_core::Band::linear(geom_core::Tol::witness()).unwrap();
        let (u0, u1) = s.knots_u().domain();

        let (first, at) = iso_boundary_row(&s, u0, band).expect("the u0 row extracts");
        assert_eq!(
            at, u0,
            "the description is re-stated at the chart's own float"
        );
        assert_eq!(first.knots().knots(), s.knots_v().knots());
        let want = boundary_iso_u(&s, false).unwrap();
        assert!(
            first
                .control()
                .iter()
                .zip(want.control())
                .all(|(a, b)| a.distance(*b) == 0.0),
            "the selected row IS the u0 row, point for point"
        );

        let (last, at) = iso_boundary_row(&s, u1, band).expect("the u1 row extracts");
        assert_eq!(at, u1);
        let want = boundary_iso_u(&s, true).unwrap();
        assert!(
            last.control()
                .iter()
                .zip(want.control())
                .all(|(a, b)| a.distance(*b) == 0.0),
            "and the u1 row for the other end"
        );

        let mid = (u0 + u1) * 0.5;
        let e = iso_boundary_row(&s, mid, band).expect_err("an interior u has no row");
        assert!(
            matches!(e, IsoRowError::Interior { u, domain } if u == mid && domain == (u0, u1)),
            "expected the interior refusal echoing the ask, got {e}"
        );
    }

    /// A 3×2 (u×v) bilinear-ish surface: boundary extraction matches
    /// dense evaluation along each boundary.
    #[test]
    fn boundary_isos_match_surface_evaluation() {
        let s = surface();
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
