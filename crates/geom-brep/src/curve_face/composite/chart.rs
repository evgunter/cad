//! **The ring composite's chart at any scalar** ([`chart`]): the one
//! home of the chart frame and canonical form that the walk at the
//! scalar (`super::walk`) and the certification readers
//! (`super::framed_form`) build on. Generic over every real scalar, the
//! walk's duals included, so it sits outside the certification module.

use geom::Surface;
use geom_core::spline::compose::CanonicalSurface;
use geom_core::{Point3, Real, Vec3};

use super::CompositeRefusal;
use crate::pcurve_cache::projected::Meters;

/// **A surface's chart at the scalar `S`** ([`chart`]): the chart frame's
/// origin and orthonormal axes, the canonical form, and its metres
/// conversion, the stored surface's scalars crossed into `S`.
pub(crate) struct Chart<S: Real> {
    /// The frame's origin.
    pub(crate) origin: Point3<S>,
    /// The frame's axes: the reference direction made orthogonal to the
    /// axis, the axis × it, and the axis, each normalized.
    pub(crate) axes: [Vec3<S>; 3],
    /// The canonical form `f` the composite reads.
    pub(crate) form: CanonicalSurface<S>,
    /// The form's metres conversion, on a cone against the carrier.
    pub(crate) meters: Meters<S>,
}

/// **The one home of a composite's chart**: the face's chart frame and
/// canonical form, every stored scalar crossed into `S` by `cross` before
/// the frame is formed, so at an enclosure scalar the axes enclose an
/// exactly orthonormal frame about the stored axis (the stored twin is
/// orthonormal only to its rounding) and a cone's sine and cosine
/// enclose the stored angle's ([`crate::implicit::cone_sin_cos`]).
/// `reads` says whether an axis coordinate is one the caller reads (a
/// certified enclosure, or no poison). The walk at the scalar
/// ([`super::walk::method`]) and the certification readers ([`super::framed_form`])
/// both build here.
///
/// # Errors
///
/// [`CompositeRefusal::Unread`] off an analytic chart (a spline face, a
/// spindle sheet); [`CompositeRefusal::FaceDegenerate`] where an axis
/// does not read.
pub(crate) fn chart<U: Real, S: Real>(
    surface: &Surface<U>,
    cross: impl Fn(U) -> S,
    reads: impl Fn(S) -> bool,
) -> Result<Chart<S>, CompositeRefusal> {
    let unread = CompositeRefusal::Unread;
    let (origin, frame) = crate::pcurve_cache::projected::chart_frame(surface)
        .ok_or(unread("a spline face has no canonical form"))?;
    let vector = |v: Vec3<U>| Vec3::new(cross(v.x), cross(v.y), cross(v.z));
    let origin = Point3::new(cross(origin.x), cross(origin.y), cross(origin.z));
    let [u_ref, _, axis] = frame.map(vector);
    let e2 = axis.normalize();
    let e0 = (u_ref - e2 * u_ref.dot(e2)).normalize();
    let axes = [e0, e2.cross(e0), e2];
    if !axes.iter().all(|v| reads(v.x) && reads(v.y) && reads(v.z)) {
        return Err(CompositeRefusal::FaceDegenerate(
            "the face's chart frame does not read",
        ));
    }
    let meters = Meters::of(
        surface,
        crate::pcurve_cache::projected::Locus::Carrier,
        &cross,
    )
    .ok_or(unread("a spline face has no canonical form"))?;
    let form = match meters {
        Meters::Plane => CanonicalSurface::Plane,
        Meters::Cylinder { radius } => CanonicalSurface::Cylinder { radius },
        Meters::Sphere { radius } => CanonicalSurface::Sphere { radius },
        Meters::Cone { sin, cos, .. } => CanonicalSurface::Cone {
            cos2: cos.powi(2),
            sin2: sin.powi(2),
        },
        Meters::Torus { major, minor } => CanonicalSurface::Torus { major, minor },
        Meters::Spindle { .. } => {
            return Err(unread("a spindle sheet's face has no composite lane yet"));
        }
    };
    Ok(Chart {
        origin,
        axes,
        form,
        meters,
    })
}
