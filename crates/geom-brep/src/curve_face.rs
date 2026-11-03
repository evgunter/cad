//! **The curve × face meeting table**: how a carrier span and a face
//! meet (C5's curve × face clause, `crates/geom-brep/README.md`).
//!
//! - **One total table** ([`route`]): one exhaustive match over the
//!   span's [`CurveKind`] and the face's [`SurfaceKind`], with no
//!   wildcard arm, naming the door that reads the span against the
//!   face's interior. An arm that is not built says so
//!   ([`Route::implemented`]) and names what it lacks, and its caller
//!   refuses typed with that routing.
//! - **The boundary first** ([`read`], and [`boundary`] its answer for
//!   the span's interior): before any arm, the span is
//!   read against each of the face's boundary edges through the
//!   curve-pair table ([`crate::curve_pair::meet`]), each boundary
//!   edge's start vertex on the span before its meeting. A shared run
//!   along a boundary edge is a coincidence, decided by that reading and
//!   never by surface arithmetic; a point meeting there is a meeting on
//!   the face's boundary. Only a span the boundary leaves undecided
//!   ([`Boundary::Clear`]) is read against the face's interior.
//!
//! The pass is given no topology and no surface: the boundary is its
//! spans, in the face's walk order, and its lone vertices, so it serves
//! every face kind alike.

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
    /// split. Of several, a lone vertex's first, else the one the walk
    /// reaches first, which need not be the least `t`.
    Meets {
        /// The parameter on the span.
        t: T,
        /// The point.
        point: Point3<T>,
        /// Where it sits on the boundary.
        at: OnBoundary<T>,
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
    Fault {
        /// The carrier misread, or the readings that contradict.
        what: &'static str,
    },
}

/// Where a meeting of the span sits on the face's boundary
/// ([`BoundaryMeeting`]).
#[derive(Clone, Copy, Debug)]
pub enum OnBoundary<T: geom_core::Real> {
    /// The start vertex of the edge at this index in the walk.
    Vertex(usize),
    /// The lone vertex at this index.
    Lone(usize),
    /// On the edge at this index in the walk, where its span reads the
    /// meeting: inside it, or at one of its ends.
    Edge(usize, Hit<T>),
}

/// A meeting of the span with the face's boundary: its point, where it
/// sits on the span, and where on the boundary.
#[derive(Clone, Copy, Debug)]
pub struct BoundaryMeeting<T: geom_core::Real> {
    /// The point.
    pub point: Point3<T>,
    /// Where it sits on the span.
    pub on_span: Hit<T>,
    /// Where it sits on the boundary.
    pub on_boundary: OnBoundary<T>,
}

/// **Every meeting of a span with a face's boundary** ([`read`]), in the
/// order the pass reads them.
#[derive(Clone, Debug)]
pub struct BoundaryReading<T: geom_core::Real> {
    /// The meetings: each vertex read on the span, then each point
    /// meeting of an edge the span shares no curve with.
    pub meetings: Vec<BoundaryMeeting<T>>,
    /// The edges, by index in the walk, whose carrier the pose read as the
    /// span's own curve, each with whether they share a run of positive
    /// length ([`shares_a_run`]). Their point meetings are the run's
    /// witnesses and are not in `meetings`.
    pub shared: Vec<(usize, bool)>,
}

/// A boundary member the pass could not read against the span, and the
/// stretch of the span outside which it meets nothing
/// ([`Span::window_near`]).
#[derive(Clone, Copy, Debug)]
pub struct Undecided<T: geom_core::Real> {
    /// The span's parameters, `(lo, hi)`, the member's meetings lie
    /// within.
    pub window: (T, T),
    /// Why the member was not read.
    pub why: BoundaryRefusal,
}

/// **The boundary pass read past its members' refusals** ([`located`]):
/// the meetings of every member it read, and every member it did not.
#[derive(Clone, Debug)]
pub struct Located<T: geom_core::Real> {
    /// The members read, as [`read`] gives them.
    pub reading: BoundaryReading<T>,
    /// The members not read, in the pass's order.
    pub undecided: Vec<Undecided<T>>,
}

/// **The boundary pass**: `span` read against a face's boundary, given
/// as its edges in the face's walk order (each edge's start vertex and
/// its span) and its lone vertices. Every member is read, so a refusal
/// anywhere on the boundary refuses the pass. Each vertex is read on the
/// span ([`Span::contact`]); each edge's start vertex before the edge's
/// span, which is read through [`crate::curve_pair::meet`]. A shared run
/// is read through its witnesses, of which one strictly inside the span
/// is a run of positive length, and leaves the run's ends to the vertex
/// readings, since every end of a run strictly inside the span is a
/// vertex of the boundary.
///
/// # Errors
///
/// [`BoundaryRefusal`], as its variants say: the first member's refusal
/// in the pass's order. A refusal's meetings are not read: a stretch
/// that escalates holds meetings whose stretch is undecided.
pub fn read<T: Decide>(
    span: &Span<'_, T>,
    edges: &[(Point3<T>, Span<'_, T>)],
    lone: &[Point3<T>],
    band: Band,
) -> Result<BoundaryReading<T>, BoundaryRefusal> {
    let pass = pass(span, edges, lone, band, false);
    pass.undecided
        .first()
        .map_or(Ok(pass.reading), |u| Err(u.why))
}

/// **The boundary pass, located** ([`read`]'s, read past each member's
/// refusal): a member it cannot read is [`Undecided`] over the window of
/// the span it can meet ([`Span::window_near`]), and every other member is
/// read as [`read`] reads it. So the reading is exact outside the
/// windows, and says nothing inside them.
pub fn located<T: Decide>(
    span: &Span<'_, T>,
    edges: &[(Point3<T>, Span<'_, T>)],
    lone: &[Point3<T>],
    band: Band,
) -> Located<T> {
    pass(span, edges, lone, band, true)
}

/// [`read`] and [`located`]'s one walk; `past` reads on past a member's
/// refusal.
fn pass<T: Decide>(
    span: &Span<'_, T>,
    edges: &[(Point3<T>, Span<'_, T>)],
    lone: &[Point3<T>],
    band: Band,
    past: bool,
) -> Located<T> {
    let (t0, t1) = span.params();
    let mid = geom::mid_param(t0, t1);
    let mut pass = Located {
        reading: BoundaryReading {
            meetings: Vec::new(),
            shared: Vec::new(),
        },
        undecided: Vec::new(),
    };
    let window = |cover: Option<geom::cover::Cover<T>>| span.window_near(cover.as_ref(), band);
    let vertex = |pass: &mut Located<T>, point: Point3<T>, on_boundary| {
        match span.contact(point, mid, band) {
            Ok(Some(on_span)) => pass.reading.meetings.push(BoundaryMeeting {
                point,
                on_span,
                on_boundary,
            }),
            Ok(None) => {}
            Err(refusal) => pass.undecided.push(Undecided {
                window: window(geom::cover::Cover::points(vec![point])),
                why: carrier(refusal),
            }),
        }
    };
    let mut met_edges = Vec::with_capacity(edges.len());
    let members = lone
        .iter()
        .enumerate()
        .map(|(i, &point)| (point, OnBoundary::Lone(i), None))
        .chain(
            edges
                .iter()
                .enumerate()
                .map(|(i, (start, edge))| (*start, OnBoundary::Vertex(i), Some((i, edge)))),
        );
    for (point, on_boundary, edge) in members {
        if !past && !pass.undecided.is_empty() {
            break;
        }
        vertex(&mut pass, point, on_boundary);
        let Some((i, edge)) = edge else { continue };
        if !past && !pass.undecided.is_empty() {
            break;
        }
        match meet(span, edge, band) {
            Ok(met) => met_edges.push((i, met)),
            Err(refusal) => {
                let (e0, e1) = edge.params();
                let cover = edge.carrier().cover_over(geom::cover::ParamWindow::new(e0, e1));
                pass.undecided.push(Undecided {
                    window: window(cover.ok()),
                    why: BoundaryRefusal::of(refusal.fault),
                });
            }
        }
    }
    for (i, met) in met_edges {
        if met.shared.is_some() {
            pass.reading.shared.push((i, shares_a_run(&met)));
            continue;
        }
        pass.reading
            .meetings
            .extend(met.points.iter().map(|m| BoundaryMeeting {
                point: m.point,
                on_span: m.on_a,
                on_boundary: OnBoundary::Edge(i, m.on_b),
            }));
    }
    pass
}

/// **The boundary pass's answer for a span's interior** ([`read`]): the
/// first meeting strictly inside the span, a lone vertex's before the
/// walk's and the walk's in its order, else a run shared with a boundary
/// edge, else clear.
///
/// # Errors
///
/// [`read`]'s.
pub fn boundary<T: Decide>(
    span: &Span<'_, T>,
    edges: &[(Point3<T>, Span<'_, T>)],
    lone: &[Point3<T>],
    band: Band,
) -> Result<Boundary<T>, BoundaryRefusal> {
    let reading = read(span, edges, lone, band)?;
    // Walk order: an edge's start vertex, then the edge's meetings, and
    // `meetings` holds every vertex before any edge's, the lone first.
    let first = reading
        .meetings
        .iter()
        .filter_map(|m| match m.on_span {
            Hit::On { t, .. } => Some(Boundary::Meets {
                t,
                point: m.point,
                at: m.on_boundary,
            }),
            Hit::End(_) => None,
        })
        .min_by_key(|b| match *b {
            Boundary::Meets {
                at: OnBoundary::Lone(_),
                ..
            } => 0,
            Boundary::Meets {
                at: OnBoundary::Vertex(edge) | OnBoundary::Edge(edge, _),
                ..
            } => edge + 1,
            Boundary::Along { .. } | Boundary::Clear => usize::MAX,
        });
    let along = reading.shared.iter().find(|(_, run)| *run).map(|&(i, _)| i);
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
            MeetFault::Desync => Self::Fault {
                what: "a carrier the meeting table misread",
            },
            MeetFault::Contradiction { readings } => Self::Fault { what: readings },
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
    refusal.escalation().map_or_else(
        |why| BoundaryRefusal::Unread(why.into()),
        BoundaryRefusal::Escalated,
    )
}

pub mod composite;
#[cfg(test)]
mod review_rows;
#[cfg(test)]
mod rows;
pub mod spline_face;
