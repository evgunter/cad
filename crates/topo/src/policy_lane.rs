//! **Edge certification with the plane × NURBS lane read off the
//! scalar's policy**: the one home of a `topo` door's certification
//! through [`AtRestPolicy::nurbs_lane`], and of the refusal that names
//! the scalar where the policy holds no lane.
//!
//! `geom_brep`'s `certify_via`/`recertify_via` take `Option<NurbsLane>`
//! and answer [`CertifyError::NurbsLaneNotSupplied`] when handed `None`,
//! which is all that layer can know. Here the `None` is the policy's
//! answer, so the absence is the scalar's, and the result type says so:
//! [`ByPolicy::NoLane`] is not a [`CertifyError`], so a door cannot pass
//! it on as a certification failure without naming it.

use geom::Surface;
use geom_brep::{Certificate, CertifyError, EdgeCurve, EdgeCurveSpec};
use geom_core::{Band, Point3, Real};

use crate::geometry::SurfaceKey;
use crate::props::AtRestPolicy;

/// Why a certification through the scalar's policy did not certify.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum ByPolicy {
    /// The edge's certificate needs the NURBS lane and `scalar`'s
    /// policy holds none (a dual, DL1).
    NoLane {
        /// [`Real::NAME`] of the scalar the door ran at.
        scalar: &'static str,
    },
    /// Every other refusal, as the certifier raised it.
    Refused(CertifyError),
}

impl ByPolicy {
    fn of<T: Real>(error: CertifyError) -> Self {
        if error == CertifyError::NurbsLaneNotSupplied {
            Self::NoLane { scalar: T::NAME }
        } else {
            Self::Refused(error)
        }
    }
}

/// [`EdgeCurve::certify_via`] with `T`'s lane.
pub(crate) fn certify<T: AtRestPolicy>(
    spec: EdgeCurveSpec<T>,
    start: Point3<T>,
    end: Point3<T>,
    surfaces: impl Fn(SurfaceKey) -> Option<Surface<T>>,
    band: Band,
) -> Result<EdgeCurve<T>, ByPolicy> {
    EdgeCurve::certify_via(spec, start, end, surfaces, band, T::nurbs_lane())
        .map_err(ByPolicy::of::<T>)
}

/// [`EdgeCurve::recertify_via`] with `T`'s lane.
pub(crate) fn recertify<T: AtRestPolicy>(
    curve: &EdgeCurve<T>,
    start: Point3<T>,
    end: Point3<T>,
    surfaces: impl Fn(SurfaceKey) -> Option<Surface<T>>,
    band: Band,
) -> Result<Certificate<T>, ByPolicy> {
    curve
        .recertify_via(start, end, surfaces, band, T::nurbs_lane())
        .map_err(ByPolicy::of::<T>)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use std::sync::Arc;

    use geom::{Curve3, NurbsCurve3};
    use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, SurfacePair};
    use geom_core::spline::KnotVector;
    use geom_core::{Dual64, Real, Tol};

    use crate::props::AtRestPolicy;
    use crate::test_support_fixtures::{describe_as_intersections, geometric_cube};
    use crate::{EulerOpError, entity::EdgeKey};

    /// Re-carries one plane × plane edge of the unit cube at `T` as a
    /// degree-1 spline between its two planes (an analytic rung-3
    /// edge) through the attach door.
    fn respec_as_spline<T: AtRestPolicy + geom_core::Bounds>() -> Result<EdgeKey, EulerOpError> {
        let tol = Tol::witness();
        fn cube<S: AtRestPolicy>(tol: Tol) -> crate::Body<S> {
            let mut body = geometric_cube::<S>(tol).body;
            describe_as_intersections(&mut body, tol);
            body
        }
        let mut body = cube::<T>(tol);
        // The same cube at f64 (D9: the same keys) gives the edge's
        // corners as f64, which the carrier is lifted from.
        let reference = cube::<f64>(tol);
        let (edge_key, edge) = reference
            .edges()
            .next()
            .map(|(k, e)| (k, e.clone()))
            .unwrap();
        let (s1, s2) = crate::readback::edge_sides(&body, edge_key)
            .unwrap()
            .surfaces();
        let corner = |v| {
            *reference
                .get_point(reference.get_vertex(v).unwrap().point)
                .unwrap()
        };
        let p0 = corner(reference.get_half_edge(edge.he_plus).unwrap().start);
        let p1 = corner(reference.half_edge_end(edge.he_plus).unwrap());
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let carrier = NurbsCurve3::new(kv, vec![p0, p1], vec![1.0, 1.0])
            .unwrap()
            .map_scalar(T::from_f64);
        let spec = EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                pair: SurfacePair::new(s1, s2),
                witness: carrier.eval(T::from_f64(0.5)),
            },
            carrier: Curve3::Nurbs(Arc::new(carrier)),
            param_start: T::zero(),
            param_end: T::one(),
        };
        body.set_edge_curve(edge_key, spec, tol).map(|_| edge_key)
    }

    /// **The attach door takes the scalar's lane**: an analytic rung-3
    /// edge attaches at `f64`, and at the dual — whose policy holds no
    /// lane — refuses naming the edge and the scalar.
    #[test]
    fn the_attach_door_refuses_an_analytic_rung3_edge_at_the_dual() {
        respec_as_spline::<f64>().expect("f64 holds the lane");
        let dual = respec_as_spline::<Dual64>();
        assert!(
            matches!(
                dual,
                Err(EulerOpError::NurbsLaneUnsupported {
                    edge: Some(_),
                    scalar,
                }) if scalar == <Dual64 as Real>::NAME
            ),
            "{dual:?}"
        );
    }
}
