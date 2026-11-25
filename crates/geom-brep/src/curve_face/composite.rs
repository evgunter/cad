//! **The spline span × analytic face arm** of the curve × face table
//! ([`super::Door::SplineComposite`]): the face's canonical residual
//! composed along the span's NURBS carrier, the ring composite `f ∘ C`
//! ([`geom_core::spline::compose::CanonicalSurface::piece_rows`], in
//! `geom_core::spline::canonical_rows`), read
//! piece by piece at the scalar ([`walk`]'s module docs). It reads a
//! plane, cylinder, cone, sphere or torus, metered through the form's
//! distance divisor.
//!
//! The carrier's net is written in the surface's chart frame, so the
//! composite is a rational Bernstein form `f = N / D`, `D` a power of the
//! carrier's weight function and so positive: the carrier's door admits
//! no other weight. Every reading is of `f` itself, metered: a value of
//! it over an upper bound on the form's distance divisor over the net's
//! hull (the plane's is one) bounds the point's distance from the surface
//! from below, and that is what is decided against the band
//! `implicit_residual` is decided against. The divisor is a factor of `f`
//! at each point, `|f(p)| ≤ d(p)·divisor(p)` with `d` the distance from
//! the form's zero set (`projected::Meters::divisor`), not a bound along
//! a path to it, so the hull, which holds every point of the span, is the
//! region it is read over. **A piece never straddles a knot**: the
//! pieces start as the carrier's knot spans cut at the span's ends, and
//! every later piece is a part of one, so each row is one polynomial and
//! a derivative read on it is the derivative there.
//!
//! Every meeting is then placed. One within the band of a **crease** (an
//! interior knot of multiplicity at least the degree,
//! `KnotStructure::crease_runs`), where the carrier has no tangent,
//! refuses, read in metres from the crease's point. A touch refuses. A
//! crossing must read ON the surface. Its place against the span's ends
//! and the crossing before it is the table's ([`super::cross::place`]).
//!
//! The walk is `geom_core::bisect::bisect_within`, with its piece budget
//! past the knot spans it starts from. A crossing the body stores is a
//! proposal, admitted at the storing site
//! ([`crate::NurbsLane::admit_crossing`]).

use geom::Surface;
use geom_core::spline::compose::CanonicalSurface;
use geom_core::{Indeterminate, Interval, Point3, Real, Vec3};

use crate::pcurve_cache::projected::Meters;
use crate::roots::Unsettled;

/// Why the arm gave no answer ([`CompositeRefusal::fault`]).
#[derive(Clone, Copy, Debug)]
pub enum CompositeRefusal {
    /// A decision the arm must read definite fell in the band's gap:
    /// whether a located crossing lies ON the surface or apart from a
    /// crease, or a piece's end, or every share of a piece, read in the gap
    /// where the carrier comes within the band of the surface.
    Escalated(Indeterminate),
    /// The walk reached a limit, or a reading it has no arm for:
    /// [`Unsettled::Tangency`] (the carrier decided within the zero band
    /// of the surface along a piece, at every share, or at a knot it
    /// leaves the way it came), [`Unsettled::Floor`] (a piece at the floor that its
    /// enclosure does not read past the band), [`Unsettled::Crease`],
    /// [`Unsettled::Meter`] (a located crossing reads definitely off the
    /// surface) or [`Unsettled::Exhausted`].
    Unsettled(Unsettled),
    /// The face's surface describes no frame or no positive meter: its
    /// chart frame does not read (a reference direction along its axis, a
    /// zero or non-finite direction), or the form's distance divisor is
    /// not shown positive over the net's hull. A surface tier 3 refuses
    /// ([`geom::Surface::representability_margins`]), read here because
    /// the arm does not re-validate its inputs.
    FaceDegenerate(&'static str),
    /// The composite could not be formed: the face has no canonical form
    /// (a spline face), a weight of the carrier is not shown positive, or
    /// the carrier does not cut to its Bézier pieces.
    Unread(&'static str),
}

impl CompositeRefusal {
    /// **What the refusal is to a reader of the curve × face table**: the
    /// one home of the mapping, read by every reader through the table's
    /// window ([`super::CrossWhy::Method`]).
    ///
    /// - [`Self::Escalated`] is a band reading: [`MeetFault::Escalated`],
    ///   ranked by its margin, which a finer tolerance can settle;
    /// - [`Self::Unsettled`] is the walk's limit or a reading it has no
    ///   arm for (a tangency, a crease, the floor, the budget):
    ///   [`MeetFault::Unsettled`], with no margin;
    /// - [`Self::FaceDegenerate`] and [`Self::Unread`] are inputs the arm
    ///   cannot form a composite from (a frame or a meter that does not
    ///   read, a weight not shown positive, a carrier that does not cut to
    ///   Bézier pieces): [`MeetFault::Unsupported`], a limit. Neither is a
    ///   kernel fault: the table routes only analytic faces here, and a
    ///   poisoned or degenerate input is the input's, not the reading's.
    #[must_use]
    pub fn fault(self) -> crate::curve_pair::MeetFault {
        use crate::curve_pair::MeetFault;
        match self {
            Self::Escalated(diag) => MeetFault::Escalated(diag),
            Self::Unsettled(why) => MeetFault::Unsettled(why),
            Self::FaceDegenerate(pose) | Self::Unread(pose) => MeetFault::Unsupported { pose },
        }
    }
}

impl From<Unsettled> for CompositeRefusal {
    fn from(why: Unsettled) -> Self {
        Self::Unsettled(why)
    }
}

/// [`framed_form`]'s answer: the points' coordinates in the chart frame,
/// the canonical form and its metres conversion.
pub(crate) type FramedForm = ([Vec<Interval>; 3], CanonicalSurface, Meters<Interval>);

/// A surface's canonical form and its metres conversion, with `points`
/// written in its chart frame in certification arithmetic, coordinate by
/// coordinate: the operands a composite against the surface reads, for
/// a carrier's net or a patch's ([`crate::patch_carrier`]) alike.
///
/// # Errors
///
/// [`CompositeRefusal::Unread`] off an analytic chart or for a point that
/// does not certify; [`CompositeRefusal::FaceDegenerate`] where the chart
/// frame does not.
pub(crate) fn framed_form<T: geom_core::Certifies>(
    surface: &Surface<T>,
    points: &[Point3<T>],
    cross: &impl Fn(T) -> Interval,
) -> Result<FramedForm, CompositeRefusal> {
    let unread = CompositeRefusal::Unread;
    let (origin, frame) = crate::pcurve_cache::projected::chart_frame(surface)
        .ok_or(unread("a spline face has no canonical form"))?;
    let point = |p: Point3<T>| Vec3::new(cross(p.x), cross(p.y), cross(p.z));
    let vector = |v: Vec3<T>| Vec3::new(cross(v.x), cross(v.y), cross(v.z));
    let o = point(origin);
    // The frame formed in certification arithmetic: the axis normalized,
    // and the reference direction made orthogonal to it and normalized.
    // Its enclosures hold an exactly orthonormal frame about the stored
    // axis, which the radial terms read; the stored twin is orthonormal
    // only to its rounding.
    let [u_ref, _, axis] = frame.map(vector);
    let e2 = axis.normalize();
    let radial = u_ref - e2 * u_ref.dot(e2);
    let e0 = radial.normalize();
    let e = [e0, e2.cross(e0), e2];
    if !e
        .iter()
        .all(|v| v.x.is_certified() && v.y.is_certified() && v.z.is_certified())
    {
        return Err(CompositeRefusal::FaceDegenerate(
            "the face's chart frame does not certify",
        ));
    }
    let framed: Vec<Vec3<Interval>> = points
        .iter()
        .map(|&p| {
            let g = point(p) - o;
            Vec3::new(g.dot(e[0]), g.dot(e[1]), g.dot(e[2]))
        })
        .collect();
    if !framed
        .iter()
        .all(|g| g.x.is_certified() && g.y.is_certified() && g.z.is_certified())
    {
        return Err(unread("a control point does not certify"));
    }
    let coords = [
        framed.iter().map(|g| g.x).collect::<Vec<_>>(),
        framed.iter().map(|g| g.y).collect(),
        framed.iter().map(|g| g.z).collect(),
    ];
    let (form, meters) = match *surface {
        Surface::Plane { .. } => (CanonicalSurface::Plane, Meters::Plane),
        Surface::Cylinder { radius, .. } => {
            let radius = cross(radius);
            (
                CanonicalSurface::Cylinder { radius },
                Meters::Cylinder { radius },
            )
        }
        Surface::Sphere { radius, .. } => {
            let radius = cross(radius);
            (
                CanonicalSurface::Sphere { radius },
                Meters::Sphere { radius },
            )
        }
        Surface::Cone { half_angle, .. } => {
            let (sin, cos) = crate::implicit::cone_sin_cos(half_angle, cross);
            (
                CanonicalSurface::Cone {
                    cos2: cos.powi(2),
                    sin2: sin.powi(2),
                },
                Meters::Cone {
                    sin,
                    cos,
                    locus: crate::pcurve_cache::projected::Locus::Carrier,
                },
            )
        }
        Surface::Torus {
            major_radius,
            minor_radius,
            ..
        } => {
            let (major, minor) = (cross(major_radius), cross(minor_radius));
            (
                CanonicalSurface::Torus { major, minor },
                Meters::Torus { major, minor },
            )
        }
        Surface::Nurbs(_) | Surface::Approx(_) => {
            return Err(unread("a spline face has no canonical form"));
        }
        Surface::Spindle { .. } => {
            return Err(unread("a spindle sheet's face has no composite lane yet"));
        }
    };
    Ok((coords, form, meters))
}

mod walk;
pub(in crate::curve_face) use walk::method;

#[cfg(test)]
mod rows;

#[cfg(test)]
mod crossing_parameter_rows;
