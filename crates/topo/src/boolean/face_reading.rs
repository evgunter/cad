//! **The face reading** (C5's curve × face clause; GERM U4 §2.3): the
//! crossing reading `curve_face::cross` answers for a span against a
//! face's carrier, read through the face's trim, once, for every lane that
//! reads a curved face.
//!
//! - **A found meeting** strictly inside the span is placed in the trim
//!   (`reduce::place_crossing`: a cone's far nappe told off first, then
//!   `contain::placement_on_carrier`). One at an end of the span is that
//!   end's: the vertex rows own it.
//! - **A window** holds no point of the face when its ball does not
//!   (`window_off_face`): let `m` be the window's middle, `ℓ` its reach
//!   along the carrier at the carrier's speed bound
//!   (`geom_core::bisect::reach`), and `(dist, foot)` the surface's signed
//!   distance and foot at `C(m)` ([`distance`]). Every meeting the window
//!   can hold lies within `ℓ + |dist|` of the foot (the distance is
//!   1-Lipschitz). That radius, widened by `escalate`, must decide below
//!   the carrier's reach ρ ([`reach`]), and then the ball meets the carrier
//!   in one disc; with every boundary vertex and edge of the face decided
//!   clear of the ball and its foot placed `Out`, the face holds none of
//!   that disc ([`ball_off_face`]). Any other window refuses the reading,
//!   naming the window and its why. A carrier with no distance and foot (a
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
use geom_core::{Band, Bounds, Decide, Margin, Point3, Real, Rounded, Sign, Vec3};

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

/// **Whether the window `(lo, hi)` of `carrier` holds no point of
/// `face`** (module docs): its ball, about the foot of its middle, read
/// against the face. A carrier with no distance and foot here holds the
/// window (`false`).
///
/// # Errors
///
/// [`ball_off_face`]'s.
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
    let m = carrier.eval(geom::mid_param(lo, hi));
    let Some((dist, foot)) = distance(surface, m) else {
        return Ok(false);
    };
    let radius =
        geom_core::bisect::reach(speed, (lo, hi)) + dist.magnitude() + T::from_f64(band.escalate());
    ball_off_face(y, face, foot, radius, rho, band)
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

/// **A point's signed distance from the carrier, carried with its
/// rounding, and its foot on it** (standing claim 1: re-posed, not
/// charged). Every quantity is read from `q − origin`, the surface's
/// stored origin taken exact, as running [`Rounded`] values: an axis's
/// offset as `|e × n|`, never `e − (e·n)n`, which cancels far along it;
/// and the foot from `q` itself, `q − n̂·dist`, never `centre + w·(R/|w|)`,
/// which rounds at the centre's scale however near `q` lies. The ball
/// reads the distance's upper end (`Rounded::magnitude`), so a gap inside
/// its own rounding widens the ball rather than being read away.
pub(super) fn distance<T: Real>(
    surface: &geom::Surface<T>,
    q: Point3<T>,
) -> Option<(Rounded<T>, Point3<T>)> {
    use geom_core::running::{cross, dot, exact_vec};
    let from = |origin: Point3<T>| {
        let (qv, o) = (
            exact_vec(Vec3::new(q.x, q.y, q.z)),
            exact_vec(Vec3::new(origin.x, origin.y, origin.z)),
        );
        [qv[0] - o[0], qv[1] - o[1], qv[2] - o[2]]
    };
    let length = |v: [Rounded<T>; 3]| v[0].hypot(v[1]).hypot(v[2]);
    let plain = |v: [Rounded<T>; 3]| Vec3::new(v[0].value, v[1].value, v[2].value);
    let exact = Rounded::exact;
    // The unit normal at the foot, `n̂`, from a vector along it.
    let foot = |along: Vec3<T>, dist: Rounded<T>| q - along.normalize() * dist.value;
    match *surface {
        geom::Surface::Sphere { frame, radius, .. } => {
            let w = from(frame.origin());
            let dist = length(w) - exact(radius);
            Some((dist, foot(plain(w), dist)))
        }
        geom::Surface::Cylinder { frame, radius, .. } => {
            let e = from(frame.origin());
            let n = exact_vec(frame.axis_unit().get());
            let off = cross(n, cross(e, n));
            let dist = length(cross(e, n)) - exact(radius);
            Some((dist, foot(plain(off), dist)))
        }
        geom::Surface::Torus {
            frame,
            major_radius,
            minor_radius,
            ..
        } => {
            let e = from(frame.origin());
            let n = exact_vec(frame.axis_unit().get());
            let (h, off) = (dot(e, n), cross(n, cross(e, n)));
            let rho = length(cross(e, n));
            let dist = (rho - exact(major_radius)).hypot(h) - exact(minor_radius);
            // Along `(ρ − R)·ô + h·n`, `ô` the axis offset's direction.
            let along =
                plain(off).normalize() * (rho - exact(major_radius)).value + plain(n) * h.value;
            Some((dist, foot(along, dist)))
        }
        geom::Surface::Plane { frame, .. } => {
            let n = exact_vec(frame.axis_unit().get());
            let dist = dot(from(frame.origin()), n);
            Some((dist, foot(plain(n), dist)))
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
