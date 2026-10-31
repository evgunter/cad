//! **The curve × face meeting table**: how a carrier span and a face
//! meet (C5's curve × face clause, `crates/geom-brep/README.md`).
//!
//! - **One total table** ([`route`]): one exhaustive match over the
//!   span's [`CurveKind`] and the face's [`SurfaceKind`], with no
//!   wildcard arm, naming the door that reads the span against the
//!   face's interior. An arm that is not built says so
//!   ([`Route::implemented`]) and names what it lacks, and its caller
//!   refuses typed with that routing.
//! - **The boundary first** ([`boundary`]): before any arm, the span is
//!   read against each of the face's boundary edges through the
//!   curve-pair table ([`crate::curve_pair::meet`]), each boundary
//!   edge's start vertex on the span before its meeting. A shared run
//!   along a boundary edge is a coincidence, decided by that reading and
//!   never by surface arithmetic; a point meeting there is a meeting on
//!   the face's boundary. Only a span the boundary leaves undecided
//!   ([`Boundary::Clear`]) is read against the face's interior.
//!
//! The pass is given no topology and no surface: the boundary is its
//! spans, in the face's walk order, so it serves every face kind alike.

use geom::{CurveKind, SurfaceKind};
use geom_core::{Band, Decide, Indeterminate, Point3};

use crate::curve_pair::{Hit, MeetFault, Meetings, Span, meet};

/// The door that reads a span against a face's interior, once its
/// boundary has left the span undecided.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Door {
    /// A line or a conic against a plane: the plane-crossing lane.
    PlaneCrossing,
    /// A line against a cylinder, a cone, a sphere or a torus: the
    /// certified roots of the surface's residual along the line.
    LineRoots,
    /// A circle or an ellipse against a cylinder, a cone or a sphere: the
    /// residual's trigonometric polynomial in the conic's angle.
    ConicQuadric,
    /// A circle or an ellipse against a torus.
    ConicTorus,
    /// A spline span against an analytic face: the face's residual
    /// composed along the span (the ring composite).
    SplineComposite,
    /// Any span against a spline face: enclosures on the span's pieces
    /// and the surface's cells, a Krawczyk contraction for one meeting.
    SplineFace,
    /// A spiric span against an analytic face: no producer reaches a
    /// boolean with one.
    Spiric,
}

/// How [`route`] reads a pair of kinds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Route {
    /// The door.
    pub door: Door,
    /// Whether the door reads the pair.
    pub implemented: bool,
    /// What the door is, or, unbuilt, what it lacks.
    pub note: &'static str,
}

/// **The table**: the door every pair of kinds is read by. Exhaustive,
/// with no wildcard arm: a new [`CurveKind`] or [`SurfaceKind`] is a
/// compile error here until it has a door.
#[must_use]
pub fn route(curve: CurveKind, surface: SurfaceKind) -> Route {
    use CurveKind::{Circle, Ellipse, Line, Nurbs, Spiric};
    use SurfaceKind::{Approx, Cone, Cylinder, Plane, Sphere, Torus};
    let built = |door, note| Route {
        door,
        implemented: true,
        note,
    };
    let unbuilt = |door, note| Route {
        door,
        implemented: false,
        note,
    };
    match (curve, surface) {
        (Line | Circle | Ellipse, Plane) => built(
            Door::PlaneCrossing,
            "the roots of the plane's residual along the carrier",
        ),
        (Line, Cylinder | Cone | Sphere | Torus) => built(
            Door::LineRoots,
            "the certified roots of the surface's residual along the line",
        ),
        (Circle | Ellipse, Cylinder | Cone | Sphere) => built(
            Door::ConicQuadric,
            "the residual's trigonometric polynomial in the conic's angle",
        ),
        (Circle | Ellipse, Torus) => built(
            Door::ConicTorus,
            "the torus residual along the conic, of degree two on a circle and four on an ellipse",
        ),
        (Nurbs, Plane | Cylinder | Cone | Sphere | Torus) => built(
            Door::SplineComposite,
            "the face's residual composed along the spline span, read on its pieces",
        ),
        (Line | Circle | Ellipse | Nurbs, SurfaceKind::Nurbs | Approx) => built(
            Door::SplineFace,
            "boxes of the span's parameter and the chart's, cleared past the band or \
             proved one transverse meeting by a Krawczyk contraction",
        ),
        (Spiric, SurfaceKind::Nurbs | Approx) => unbuilt(
            Door::SplineFace,
            "enclosures of a spiric span's pieces against the surface's cells, \
             deferred until its first producer \
             (work/reachhold/spiric-operand-edges-reopen-with-their-first-producer)",
        ),
        (Spiric, Plane | Cylinder | Cone | Sphere | Torus) => unbuilt(
            Door::Spiric,
            "a spiric operand edge, deferred until its first producer \
             (work/reachhold/spiric-operand-edges-reopen-with-their-first-producer)",
        ),
    }
}

/// What the boundary pass makes of a span ([`boundary`]).
#[derive(Clone, Copy, Debug)]
pub enum Boundary<T: geom_core::Real> {
    /// The span meets the boundary strictly inside itself, at `t` and
    /// `point`: a meeting on the face's boundary, where both edges are
    /// split. Of several, the one the walk reaches first, which need not
    /// be the least `t`. `edge` is the boundary edge's index in the walk.
    Meets {
        /// The parameter on the span.
        t: T,
        /// The point.
        point: Point3<T>,
        /// The boundary edge, by its index in the walk.
        edge: usize,
    },
    /// The span lies along the boundary: a boundary edge shares a run of
    /// positive length with it, and no boundary edge or vertex meets the
    /// span strictly inside it, so the run is the whole span.
    Along {
        /// The boundary edge, by its index in the walk.
        edge: usize,
    },
    /// Certified: the span's interior meets the boundary nowhere. The
    /// span's interior is the interior door's.
    Clear,
}

/// Why [`boundary`] gave no answer.
#[derive(Clone, Copy, Debug)]
pub enum BoundaryRefusal {
    /// Where a boundary vertex sits on the span fell in the band's gap.
    Escalated(Indeterminate),
    /// A decision of the meeting reading fell in the band's gap.
    Meeting(Indeterminate),
    /// The span and a boundary edge meet in band ([`MeetFault::Touch`]).
    Touch(Indeterminate),
    /// A reading could not run: a root finder, a meter or a walk that
    /// could not certify, or a pose with no closed form. Nothing is known
    /// about the span's interior.
    Unread(MeetFault),
    /// The meeting table misread a carrier, or two of its readings
    /// contradict: a kernel fault, never a pose.
    Fault(MeetFault),
}

/// **The boundary pass**: `span` read against each of a face's boundary
/// edges, given in the face's walk order as each edge's start vertex and
/// its span. Every edge is read, so a refusal anywhere on the boundary
/// refuses the pass. Each vertex is read on the span first
/// ([`Span::contact`]), then the edge's span through
/// [`crate::curve_pair::meet`]. A vertex on the span's interior, or a
/// point meeting there, is a meeting on the boundary; a shared run is
/// read through its witnesses, of which one strictly inside the span is a
/// run of positive length, and leaves the run's ends to the vertex
/// readings, since every end of a run strictly inside the span is a
/// vertex of the boundary.
///
/// # Errors
///
/// [`BoundaryRefusal`], as its variants say. A refusal's meetings are not
/// read: a stretch that escalates holds meetings whose stretch is
/// undecided.
pub fn boundary<T: Decide>(
    span: &Span<'_, T>,
    edges: &[(Point3<T>, Span<'_, T>)],
    band: Band,
) -> Result<Boundary<T>, BoundaryRefusal> {
    let (t0, t1) = span.params();
    let mid = geom::mid_param(t0, t1);
    let mut first = None;
    let mut along = None;
    for (i, (start, edge)) in edges.iter().enumerate() {
        match span.contact(*start, mid, band) {
            Ok(Some(Hit::On { t, .. })) => {
                first.get_or_insert(Boundary::Meets {
                    t,
                    point: *start,
                    edge: i,
                });
            }
            Ok(Some(Hit::End(_)) | None) => {}
            Err(refusal) => return Err(carrier(refusal)),
        }
        let met = meet(span, edge, band).map_err(|refusal| BoundaryRefusal::of(refusal.fault))?;
        if met.shared.is_some() {
            if shares_a_run(&met) {
                along.get_or_insert(i);
            }
            continue;
        }
        for m in &met.points {
            if let Hit::On { t, .. } = m.on_a {
                first.get_or_insert(Boundary::Meets {
                    t,
                    point: m.point,
                    edge: i,
                });
            }
        }
    }
    Ok(first.unwrap_or(along.map_or(Boundary::Clear, |edge| Boundary::Along { edge })))
}

impl BoundaryRefusal {
    /// A meeting reading's refusal, as the pass types it.
    #[must_use]
    pub fn of(fault: MeetFault) -> Self {
        match fault {
            MeetFault::Escalated(diag) => Self::Meeting(diag),
            MeetFault::Touch(diag) => Self::Touch(diag),
            MeetFault::Unsettled(_) | MeetFault::Unsupported { .. } => Self::Unread(fault),
            MeetFault::Desync | MeetFault::Contradiction { .. } => Self::Fault(fault),
        }
    }
}

/// **Whether two spans share a run of positive length**: the pose read
/// them as one curve, and a witness of the run lies strictly inside the
/// first span. Spans on one curve that share only an end (the rest of a
/// rim's circle) have witnesses at that end alone.
#[must_use]
pub fn shares_a_run<T: geom_core::Real>(met: &Meetings<T>) -> bool {
    met.shared.is_some() && met.points.iter().any(|m| matches!(m.on_a, Hit::On { .. }))
}

/// A point-on-span refusal of a vertex reading.
fn carrier(refusal: geom::CarrierRefusal) -> BoundaryRefusal {
    match crate::curve_pair::carrier_fault(refusal) {
        MeetFault::Escalated(diag) => BoundaryRefusal::Escalated(diag),
        fault => BoundaryRefusal::Unread(fault),
    }
}

pub mod composite;
#[cfg(test)]
mod review_rows;
#[cfg(test)]
mod rows;
pub mod spline_face;
