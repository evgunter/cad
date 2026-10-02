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
    /// The edge is of the plane × NURBS class and `scalar`'s policy
    /// holds no lane (a dual, DL1).
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
