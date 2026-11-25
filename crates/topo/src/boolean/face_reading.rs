//! **The face reading** (C5's curve × face clause; GERM U4 §2.3): the
//! crossing reading `curve_face::cross` answers for a span against a
//! face's carrier, read through the face's trim, once, for every lane that
//! reads a curved face.
//!
//! - **A found meeting** strictly inside the span is placed in the trim
//!   (`reduce::place_crossing`: a cone's far nappe told off first, then
//!   `contain::placement_on_carrier`). One at an end of the span is that
//!   end's: the vertex rows own it.
//! - **A window** holds no point of the face when none of its pieces
//!   does (`window_off_face`, halving to a fixed depth); a piece standing
//!   past its reach from the carrier holds none, and any other holds none
//!   when its ball does not: let `m` be the piece's middle, `ℓ` its reach
//!   along the carrier at the carrier's speed bound
//!   (`geom_core::bisect::reach`), and `(dist, foot)` the surface's signed
//!   distance and foot at `C(m)` ([`distance`]). Every meeting the piece
//!   can hold lies within `ℓ + |dist|` of the foot (the distance is
//!   1-Lipschitz). That radius, widened by `escalate`, must decide below
//!   the carrier's reach ρ ([`reach`]), and then the ball meets the carrier
//!   in one disc; with every boundary vertex and edge of the face decided
//!   clear of the ball and its foot placed `Out`, the face holds none of
//!   that disc ([`ball_off_face`]). A window any piece of which the depth
//!   leaves held refuses the reading, naming the window and its why. A carrier with no distance and foot (a
//!   cone, a spline) refuses every window.
//! - **A window whose why is a fault** fails loud
//!   ([`BooleanError::ClassificationInvariant`]) and is never set aside.
//! - **A touch** is one meeting, its in-band arc: read as a window's ball,
//!   and off the face it is no meeting here. On the face it is the lane's
//!   touch treatment (a curved face's typed frontier).
//!
//! ρ is the carrier's reach, the distance from it to its medial axis: a
//! sphere's or a cylinder's radius, a ring torus's `min(r, R − r)`, and no
//! bound on a plane. A ball about a point of the carrier with radius below
//! ρ meets the carrier in one disc. With no vertex or edge of the face
//! inside the ball, the face holds all of that disc or none of it, and the
//! foot, placed `Out` of the face by its door
//! ([`crate::contain::curved_face_placement`]), says none. Every edge is
//! decided clear by a lower bound on its distance from the foot
//! ([`edge_clear_of_ball`]).

use geom_brep::curve_face::{
    CrossRefusal, CrossWhy, Crossings, MeetingKind, OnSpan, Pose, Undecided,
};
use geom_brep::curve_pair::Span;
use geom_core::ladder::{Answering, Climb, Rung, climb};
use geom_core::{Band, Bounds, Decide, Margin, Point3, Real, Sign, Vec3};

use super::BooleanError;
use super::{BooleanDecision, reduce};
use crate::body::Body;
use crate::boxes;
use crate::contain::{ContainError, CurvedPlacement, FaceContainment};
use crate::entity::{EdgeKey, FaceKey};
use crate::live::BoundaryMember;
use crate::validate::decide;

/// **What the face reading made of `cross`'s answer** ([`face_crossings`]).
pub(super) enum FaceCrossings<T: Real> {
    /// The pose's `Lies`: the span lies on the carrier.
    Lies,
    /// The pose's `Apart`: the span meets the carrier nowhere.
    Apart,
    /// Every window read off the face: the meetings in order, each
    /// placed, up to and including the first the face holds or that no
    /// placement reads. `windowed` says some window was read off.
    Met {
        /// The meetings.
        meetings: Vec<FaceMeeting<T>>,
        /// Whether some window was read off the face.
        windowed: bool,
    },
    /// A window the face may hold: the reading refuses it, by its why.
    Refused(Undecided<T, CrossWhy>),
}

/// **One meeting of the span with the carrier, read against the face.**
pub(super) enum FaceMeeting<T: Real> {
    /// At an end of the span: the end vertex's.
    End,
    /// A crossing strictly inside the span, at `t` and `p`, placed.
    Inside {
        /// Its parameter.
        t: T,
        /// Its point.
        p: Point3<T>,
        /// Where it stands against the face.
        placed: reduce::Placed,
    },
    /// A touch whose in-band arc's ball the face may hold.
    TouchOnFace,
}

/// **The face reading of `cross`'s answer for `span` against `face`**
/// (module docs).
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] for a window whose why is a
/// fault; a placement's or a ball's escalation; a stale face.
pub(super) fn face_crossings<T: Decide + Bounds>(
    y: &Body<T>,
    face: FaceKey,
    surface: &geom::Surface<T>,
    span: &Span<'_, T>,
    answer: Result<Crossings<T>, CrossRefusal<T>>,
    band: Band,
) -> Result<FaceCrossings<T>, BooleanError> {
    let (pose, mut meetings, windows) = match answer {
        Ok(Crossings {
            pose: Pose::Lies, ..
        }) => return Ok(FaceCrossings::Lies),
        Ok(Crossings {
            pose: Pose::Apart, ..
        }) => return Ok(FaceCrossings::Apart),
        Ok(Crossings { pose, meetings }) => (Some(pose), meetings, Vec::new()),
        Err(refusal) => (None, refusal.found, refusal.windows),
    };
    // A fault is never set aside, whichever window holds it.
    if let Some(what) = windows.iter().find_map(|w| w.why.fault()) {
        return Err(BooleanError::ClassificationInvariant { what });
    }
    let carrier = span.carrier();
    // Every window off the face, an identity: the first the face may hold
    // refuses, whatever an earlier one read ([`climb`], [`Answering::Any`]);
    // a window whose ball reads in band is kept by its standing.
    let held = climb(Answering::Any, windows.iter().copied(), |w| {
        Rung::holds(
            window_off_face(y, face, surface, carrier, w.window, band).map(|off| !off),
            w,
            BooleanError::refusal,
        )
    });
    match held {
        Climb::Answered { answer, .. } => return answer.map(FaceCrossings::Refused),
        Climb::Undecided(e) => return Err(e),
        Climb::Passed => {}
    }
    // The pose's singular point: `cross` answers the meetings other than the
    // apex, which the span crosses too where it holds it.
    if let Some(Pose::Through(apex)) = pose {
        let on_span = match span.place_param(apex.at, band) {
            Ok(geom_brep::curve_pair::Place::Inside) => Some(OnSpan::Inside { t: apex.at }),
            Ok(geom_brep::curve_pair::Place::End(end)) => Some(OnSpan::End(end)),
            Ok(geom_brep::curve_pair::Place::Past) => None,
            Err(diag) => {
                return Err(BooleanError::Escalated {
                    decision: BooleanDecision::Crossing(
                        geom_brep::edge_crossing::CrossingDecision::OnEdge,
                    ),
                    diag,
                });
            }
        };
        // First, as the root doors answered it ahead of the others.
        if let Some(on_span) = on_span {
            meetings.insert(
                0,
                geom_brep::curve_face::Meeting {
                    point: carrier.eval(apex.at),
                    on_span,
                    kind: MeetingKind::Cross,
                },
            );
        }
    }
    let nappe = reduce::cone_nappe(y, face, surface, band)?;
    let mut out = Vec::with_capacity(meetings.len());
    for m in meetings {
        match (m.on_span, m.kind) {
            (OnSpan::End(_), _) => out.push(FaceMeeting::End),
            (OnSpan::Inside { .. }, MeetingKind::Touch { arc }) => {
                if !window_off_face(y, face, surface, carrier, arc, band)? {
                    out.push(FaceMeeting::TouchOnFace);
                    break;
                }
            }
            (OnSpan::Inside { t }, MeetingKind::Cross) => {
                let placed = reduce::place_crossing(y, face, m.point, nappe, band)?;
                let decisive = matches!(
                    placed,
                    reduce::Placed::At(..) | reduce::Placed::Unplaced | reduce::Placed::NappeGap(_)
                );
                out.push(FaceMeeting::Inside {
                    t,
                    p: m.point,
                    placed,
                });
                if decisive {
                    break;
                }
            }
        }
    }
    Ok(FaceCrossings::Met {
        meetings: out,
        windowed: !windows.is_empty(),
    })
}

/// **How many times the face reading halves a window** ([`window_off_face`]):
/// eight, as the cluster reading it replaces did (`carrier_touch`'s
/// `CLUSTER_BUDGET`), so at most `2⁸` pieces. A window's piece is short
/// enough at that depth for its ball to fall below a carrier's reach
/// wherever the window is a touch or a graze's reach on a carrier of any
/// size the session box holds; a piece the depth leaves unsettled keeps
/// the window, and the reading refuses, as the cluster reading did past
/// its own budget.
const WINDOW_DEPTH: u32 = 8;

/// **Whether the window `(lo, hi)` of `carrier` holds no point of
/// `face`** (module docs), read piece by piece. A piece whose middle
/// stands past its own reach from the carrier by more than the band's
/// escalation holds no meeting (`curve_face_window_piece_clear`, in
/// metres: the distance is 1-Lipschitz along the carrier, so no point of
/// the piece is within the band of the surface); any other piece is read
/// by its ball ([`ball_off_face`]), and one the ball does not clear is
/// halved, to [`WINDOW_DEPTH`]. The window holds no point of the face
/// only when every piece does: the meetings it can hold lie in the union
/// of the pieces' discs, each off the face. A piece past the depth that
/// its ball does not clear keeps the window (`false`), whatever another
/// piece read; one whose reading escalated keeps it too, and its
/// escalation is the answer only where no piece is kept outright. A
/// carrier with no distance and foot here holds the window (`false`).
///
/// # Errors
///
/// [`ball_off_face`]'s, for a piece past the depth whose ball escalated.
pub(super) fn window_off_face<T: Decide + Bounds>(
    y: &Body<T>,
    face: FaceKey,
    surface: &geom::Surface<T>,
    carrier: &geom::Curve3<T>,
    (lo, hi): (T, T),
    band: Band,
) -> Result<bool, BooleanError> {
    let Some(rho) = reach(surface) else {
        return Ok(false);
    };
    let speed = geom::carrier_speed::speed_range(carrier).1;
    let escalate = T::from_f64(band.escalate());
    let mut kept = None;
    let mut pieces = vec![((lo, hi), 0_u32)];
    while let Some(((a, b), depth)) = pieces.pop() {
        let mid = geom::mid_param(a, b);
        let ell = geom_core::bisect::reach(speed, (a, b));
        let Some((dist, foot)) = distance(surface, carrier.eval(mid)) else {
            return Ok(false);
        };
        if matches!(
            decide(
                "curve_face_window_piece_clear",
                Margin::of(dist.abs() - ell - escalate),
                band
            ),
            Ok(Sign::Positive)
        ) {
            continue;
        }
        let read = ball_off_face(y, face, foot, ell + dist.abs() + escalate, rho, band);
        match read {
            Ok(true) => {}
            _ if depth < WINDOW_DEPTH => {
                pieces.push(((mid, b), depth + 1));
                pieces.push(((a, mid), depth + 1));
            }
            Ok(false) => return Ok(false),
            Err(e) => {
                kept.get_or_insert(e);
            }
        }
    }
    kept.map_or(Ok(true), Err)
}

/// A carrier's reach, ρ in the module docs.
#[derive(Clone, Copy, Debug)]
pub(super) enum MedialDistance<T> {
    /// The distance to the medial axis.
    Bounded(T),
    /// A plane's: it has no medial axis, and its level sets do not bend.
    Unbounded,
}

/// The carrier's reach; `None` for a kind with no distance here.
pub(super) fn reach<T: Real>(surface: &geom::Surface<T>) -> Option<MedialDistance<T>> {
    match *surface {
        geom::Surface::Sphere { radius, .. } | geom::Surface::Cylinder { radius, .. } => {
            Some(MedialDistance::Bounded(radius))
        }
        geom::Surface::Torus {
            major_radius,
            minor_radius,
            ..
        } => Some(MedialDistance::Bounded(
            minor_radius.min(major_radius - minor_radius),
        )),
        geom::Surface::Plane { .. } => Some(MedialDistance::Unbounded),
        // A sheet's medial axis reaches its apexes, and no distance to
        // it is written yet.
        geom::Surface::Cone { .. }
        | geom::Surface::Spindle { .. }
        | geom::Surface::Nurbs(_)
        | geom::Surface::Approx(_) => None,
    }
}

/// **A point's signed distance from the carrier, and its foot on it**,
/// posed well-conditioned (standing claim 1, `[simple.conditioned]`) and
/// then read as exact: every quantity from `q − origin`, the surface's
/// stored origin; an axis's offset as `|e × n|` and its direction as
/// `n × (e × n)`, never `e − (e·n)n`, which cancels far along it; and the
/// foot from `q` itself, `q − n̂·dist`, never `centre + w·(R/|w|)`, which
/// rounds at the centre's scale however near `q` lies.
pub(super) fn distance<T: Real>(
    surface: &geom::Surface<T>,
    q: Point3<T>,
) -> Option<(T, Point3<T>)> {
    // The foot along the unit normal through `q`, from a vector along it.
    let foot = |along: Vec3<T>, dist: T| q - along.normalize() * dist;
    match *surface {
        geom::Surface::Sphere { frame, radius, .. } => {
            let w = q - frame.origin();
            let dist = w.norm() - radius;
            Some((dist, foot(w, dist)))
        }
        geom::Surface::Cylinder { frame, radius, .. } => {
            let (e, n) = (q - frame.origin(), frame.axis_unit().get());
            let across = e.cross(n);
            let dist = across.norm() - radius;
            Some((dist, foot(n.cross(across), dist)))
        }
        geom::Surface::Torus {
            frame,
            major_radius,
            minor_radius,
            ..
        } => {
            let (e, n) = (q - frame.origin(), frame.axis_unit().get());
            let (h, across) = (e.dot(n), e.cross(n));
            let rho = across.norm();
            let dist = ((rho - major_radius).powi(2) + h.powi(2)).sqrt() - minor_radius;
            // Along `(ρ − R)·ô + h·n`, `ô` the axis offset's direction.
            let along = n.cross(across).normalize() * (rho - major_radius) + n * h;
            Some((dist, foot(along, dist)))
        }
        geom::Surface::Plane { frame, .. } => {
            let n = frame.axis_unit().get();
            let dist = (q - frame.origin()).dot(n);
            Some((dist, foot(n, dist)))
        }
        geom::Surface::Cone { .. }
        | geom::Surface::Spindle { .. }
        | geom::Surface::Nurbs(_)
        | geom::Surface::Approx(_) => None,
    }
}

/// Whether the ball of `radius` about `foot`, on the carrier, holds no
/// point of `face` (module docs).
///
/// The face's boundary is read against the ball before the foot is
/// placed, by the plane's door on a plane and the curved one elsewhere. The ball's radius is at least `escalate`, so a vertex or
/// edge the placement could not tell from the foot lies inside it and
/// answers `false`; the placement is asked only of a foot every member
/// stands clear of.
pub(super) fn ball_off_face<T: Decide + Bounds>(
    y: &Body<T>,
    face: FaceKey,
    foot: Point3<T>,
    radius: T,
    rho: MedialDistance<T>,
    band: Band,
) -> Result<bool, BooleanError> {
    if let MedialDistance::Bounded(rho) = rho
        && !matches!(
            decide("bool_touch_ball_in_reach", Margin::of(rho - radius), band),
            Ok(Sign::Positive)
        )
    {
        return Ok(false);
    }
    let f = y.proven_face(face);
    // Every boundary member clear of the ball, an identity: any member not
    // clear answers `false`, whatever an earlier member read ([`climb`],
    // [`Answering::Any`]); a member in band is kept by its standing.
    let not_clear = climb(Answering::Any, f.boundary(), |member| {
        let clear_of_ball = match member {
            BoundaryMember::Isolated { point, .. } => {
                Ok(clear((point - foot).norm(), radius, band))
            }
            BoundaryMember::Edge { edge, .. } => {
                edge_clear_of_ball(y, edge.key(), foot, radius, band)
            }
        };
        Rung::holds(clear_of_ball.map(|c| !c), (), BooleanError::refusal)
    });
    match not_clear {
        Climb::Answered { answer, .. } => return answer.map(|()| false),
        Climb::Undecided(e) => return Err(e),
        Climb::Passed => {}
    }
    // OffCarrier → not clear (the foot is on the carrier by construction).
    let foot = crate::held::Held::constructed(foot, "the foot of the ball on the face's carrier");
    match crate::contain::placement_on_carrier(y, face, foot, band).map(|(at, _, _)| at) {
        Ok(CurvedPlacement::Trim(Some(FaceContainment::Out))) => Ok(true),
        Ok(_)
        | Err(
            ContainError::EmptyLoop(_)
            | ContainError::LoopUnreadable(_)
            | ContainError::Curved(_)
            | ContainError::RayExhausted(_)
            | ContainError::Uncrossable(_),
        ) => Ok(false),
        // The foot is placed only where every boundary edge's whole
        // carrier (a segment, a circle, an ellipse's annulus) stands
        // clear of a ball of at least `escalate` about it, so the
        // boundary pass reads it definitely off every carrier, and an
        // arc's end is read only at a point on its carrier.
        Err(ContainError::ArcEndClearOfCorners(gaps)) => unreachable!(
            "face {face:?} placed a foot at an arc's end ({gaps:?}), though every boundary \
             carrier stands clear of the ball of radius {radius:?} about it"
        ),
        Err(ContainError::Escalated {
            decision,
            escalation,
            diag,
        }) => Err(BooleanError::Escalated {
            decision: BooleanDecision::Containment {
                decision,
                escalation,
            },
            diag,
        }),
        Err(ContainError::StaleFace(face)) => crate::contain::driver_face_stale(face),
    }
}

/// Whether a lower bound `gap` on a distance from a ball's centre
/// definitely exceeds its `radius`.
pub(super) fn clear<T: Decide>(gap: T, radius: T, band: Band) -> bool {
    matches!(
        decide("bool_touch_edge_clear", Margin::of(gap - radius), band),
        Ok(Sign::Positive)
    )
}

/// **Whether `edge` of `y` stays definitely farther than `radius` from
/// `at`.** Its certified box, padded by the sweep's pad, clear of the
/// ball's box answers at once. Otherwise a lower bound on the distance
/// is decided against `radius`, read on the edge's own carrier
/// ([`crate::splitting::containment::arc_distance_floor`]: a segment,
/// any other carrier's cover, and then a spiric's or a spline's arc's
/// pieces). A carrier with no cover over its span (a spline net carrying
/// poison) is not clear.
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] for an edge with no
/// certified carrier.
pub(super) fn edge_clear_of_ball<T: Decide + Bounds>(
    y: &Body<T>,
    edge: EdgeKey,
    at: Point3<T>,
    radius: T,
    band: Band,
) -> Result<bool, BooleanError> {
    let pad = boxes::sweep_pad(band);
    if !boxes::edge_box(y, edge, pad).overlaps(&boxes::centred_box(at, radius, pad)) {
        return Ok(true);
    }
    let curve = reduce::certified(y.proven_edge(edge))?;
    Ok(arc_clear_of_ball(
        curve.carrier(),
        curve.params(),
        at,
        radius,
        band,
    ))
}

/// **Whether the arc `span` of `carrier` stays definitely farther than
/// `radius` from `at`**: its distance floor
/// ([`crate::splitting::containment::arc_distance_floor`]) decided past
/// `radius`. A carrier with no cover over the span is not clear.
pub(super) fn arc_clear_of_ball<T: Decide>(
    carrier: &geom::Curve3<T>,
    span: (T, T),
    at: Point3<T>,
    radius: T,
    band: Band,
) -> bool {
    crate::splitting::containment::arc_distance_floor(carrier, span, at)
        .is_some_and(|floor| floor.clears(|gap| clear(gap, radius, band)))
}

#[cfg(test)]
#[path = "face_reading_rows.rs"]
mod face_reading_rows;
