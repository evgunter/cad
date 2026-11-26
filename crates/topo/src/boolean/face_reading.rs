//! **The face reading** (C5's curve × face clause; GERM U4 §2.3): the
//! crossing reading `curve_face::cross` answers for a span against a
//! face's carrier, read through the face's trim, once, for every lane that
//! reads a curved face.
//!
//! - **A found meeting** strictly inside the span is placed in the trim
//!   (`reduce::place_crossing`: a cone's far nappe told off first, then
//!   `contain::placement_on_carrier`). One at an end of the span is that
//!   end's: the vertex rows own it.
//! - **A window** holds no point of the face when none of its meetings
//!   with the carrier can be: they are localized into clusters by
//!   bisection (`localize`, a first- and a second-order bound on the
//!   distance over each piece, halving to the tangency floor), and each
//!   cluster is read by its ball. Let `m` be the cluster's middle, `ℓ`
//!   its reach along the carrier at the carrier's speed bound
//!   (`geom_core::bisect::reach`), and `(dist, foot)` the surface's signed
//!   distance and foot at `C(m)` ([`distance`]). Every meeting the
//!   cluster can hold lies within `ℓ + |dist|` of the foot (the distance
//!   is 1-Lipschitz). That radius, widened by `escalate`, must decide
//!   below the carrier's reach ρ ([`reach`]), and then the ball meets the
//!   carrier in one disc; with every boundary vertex and edge of the face
//!   decided clear of the ball and its foot placed `Out`, the face holds
//!   none of that disc ([`ball_off_face`]). A window any cluster of which
//!   the face may hold, or that does not localize, refuses the reading,
//!   naming the window and its why. A carrier with no distance and foot
//!   (a cone, a spline) refuses every window.
//! - **A window whose why is a fault** fails loud
//!   ([`BooleanError::ClassificationInvariant`]) and is never set aside.
//! - **A touch** is one meeting, its in-band arc: read as a window, and
//!   off the face it is no meeting here. On the face it is the lane's
//!   touch treatment (a curved face's typed frontier). A touch at an end
//!   of the span is the end vertex's at its point, but its arc's stretch
//!   inside the span is the span's: that stretch is read as an inside
//!   touch's arc is, and only then is the meeting the vertex's.
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
        // In span order among the others, as `cross` answers them.
        if let Some(on_span) = on_span {
            let apex = geom_brep::curve_face::Meeting {
                point: carrier.eval(apex.at),
                on_span,
                kind: MeetingKind::Cross,
            };
            // Whether the meeting at `b` comes after the one at `a` along
            // the span from its start: an end before or after every inside
            // meeting, two inside ones by their parameters in the span's
            // direction.
            let (t0, t1) = span.params();
            let after = |a: OnSpan<T>, b: OnSpan<T>| {
                use geom_brep::curve_pair::SpanEnd::{Start, Stop};
                match (a, b) {
                    (_, OnSpan::End(Start)) | (OnSpan::End(Stop), _) => false,
                    (OnSpan::End(Start), _) | (_, OnSpan::End(Stop)) => true,
                    (OnSpan::Inside { t: ta }, OnSpan::Inside { t: tb }) => {
                        if t1.hi() < t0.hi() {
                            tb.hi() < ta.hi()
                        } else {
                            ta.hi() < tb.hi()
                        }
                    }
                }
            };
            let at = meetings
                .iter()
                .position(|m| after(apex.on_span, m.on_span))
                .unwrap_or(meetings.len());
            meetings.insert(at, apex);
        }
    }
    let nappe = reduce::cone_nappe(y, face, surface, band)?;
    let mut out = Vec::with_capacity(meetings.len());
    for m in meetings {
        match (m.on_span, m.kind) {
            (OnSpan::End(_), MeetingKind::Cross) => out.push(FaceMeeting::End),
            // The arc's stretch inside the span is the span's: the end's
            // point is the vertex's only once that stretch is off the face.
            (OnSpan::End(_), MeetingKind::Touch { arc }) => {
                let (a, b) = span.params();
                let inside = (arc.0.max(a.min(b)), arc.1.min(a.max(b)));
                if inside.0.hi() < inside.1.hi()
                    && !window_off_face(y, face, surface, carrier, inside, band)?
                {
                    out.push(FaceMeeting::TouchOnFace);
                    break;
                }
                out.push(FaceMeeting::End);
            }
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

/// **The most clusters a window localizes into** ([`window_off_face`]):
/// the most meetings a span of a line or a conic has with these
/// carriers. A conic against a torus is degree 8 in the conic's
/// half-angle tangent, and every other such pair (a line against a
/// torus, any of them against a quadric) is at most 4. A spiric or a
/// spline window meeting the carrier more often than that is not
/// localized, and the window is held. The cap counts clusters, runs of
/// adjacent pieces that do not clear; the halving itself stops at the
/// tangency floor ([`geom_core::bisect::tangency_floor`] of the carrier's
/// reach), and the walk reads within [`geom_core::bisect::PIECE_BUDGET`]
/// pieces, past which the window is held too.
const CLUSTER_BUDGET: usize = 8;

/// **Whether the window `(lo, hi)` of `carrier` holds no point of
/// `face`** (module docs): its meetings with the carrier localized into
/// clusters ([`localize`]), and each cluster read by its ball
/// ([`ball_off_face`]). A cluster reaching `ℓ` about its middle `m`
/// meets the carrier, if at all, within `ℓ + |dist(m)|` of `m`'s foot,
/// and that radius, widened by `escalate`, is its ball's. The window
/// holds no point of the face only when every cluster's ball does: the
/// meetings it can hold lie in the union of the clusters' discs, each off
/// the face. A cluster whose ball does not clear keeps the window
/// (`false`), whatever another read; one whose reading escalated keeps
/// it too, and its escalation is the answer only where no cluster is
/// kept outright. A window the walk cannot localize, and a carrier with
/// no distance and foot here, hold the window (`false`).
///
/// # Errors
///
/// [`ball_off_face`]'s, for a cluster whose ball escalated.
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
    let bend = geom::carrier_speed::curvature_hi(carrier);
    let Some(clusters) = localize(surface, carrier, (speed, bend), rho, (lo, hi), band) else {
        return Ok(false);
    };
    let escalate = T::from_f64(band.escalate());
    // Every cluster off the face, an identity: any cluster not certified
    // off answers `false`, whatever an earlier cluster read ([`climb`],
    // [`Answering::Any`]). A cluster in band is kept by its standing
    // ([`BooleanError::refusal`]), refusing only where every other is off.
    let not_off = climb(Answering::Any, clusters, |(a, b)| {
        let Some((dist, foot, _)) = distance(surface, carrier.eval(geom::mid_param(a, b))) else {
            return Rung::Answers(Ok(()));
        };
        let radius = geom_core::bisect::reach(speed, (a, b)) + dist.abs() + escalate;
        Rung::holds(
            ball_off_face(y, face, foot, radius, rho, band).map(|off| !off),
            (),
            BooleanError::refusal,
        )
    });
    match not_off {
        Climb::Answered { answer, .. } => answer.map(|()| false),
        Climb::Passed => Ok(true),
        Climb::Undecided(e) => Err(e),
    }
}

/// A localization that stopped with no clusters to answer: a piece
/// with no distance, or more clusters than [`CLUSTER_BUDGET`].
struct Unlocalized;

/// **The parameter runs of `(lo, hi)` that may meet the carrier**, in
/// order, the curve's speed and curvature bounds and the carrier's reach
/// given; `None` past the budgets. The window is bisected. A piece
/// reaches at most `ℓ` metres along the curve from its middle `m`
/// ([`geom_core::bisect::reach`] at the speed bound), and it meets the
/// carrier nowhere within `escalate` when a lower bound on `|dist|` over
/// it, less `escalate`, decides positive (`curve_face_window_piece_clear`,
/// in metres). Two bounds are taken, the larger deciding:
///
/// - `|dist(m)| − ℓ`, since the distance is 1-Lipschitz;
/// - where the piece keeps clear of the carrier's medial axis
///   (`curve_face_window_piece_off_axis`), the second-order
///   `|dist(m)| − ℓ·|g| − ℓ²·H/2`, `g` the distance's derivative along
///   the curve at `m` (the unit normal at the foot against the unit
///   tangent) and `H` a bound on its second: `1/(ρ − |dist(m)| − ℓ)` for
///   the carrier's level sets, plus the curve's own curvature
///   ([`geom::carrier_speed::curvature_hi`]). On a plane the level sets
///   are flat and `H` is the curve's curvature alone. A curvature bound
///   is finite only where the curve is `C¹` across the piece: a spline
///   with a crease bounds none, and the first bound decides alone (at
///   `Interval`, a bound unbounded above leaves the second unbounded
///   below, which the larger-of drops).
///
/// Near a tangency the first clears nothing finer than the band; the
/// second clears pieces in proportion to their distance from the touch.
/// A piece neither clears is halved down to the tangency floor of the
/// radius the carrier's level sets part from the curve over (its reach,
/// or on a plane the curve's own radius of curvature; no finite bend sets
/// no floor, and the walk runs to its budget), and the pieces left at the
/// floor run together into clusters.
fn localize<T: Decide + Bounds>(
    surface: &geom::Surface<T>,
    carrier: &geom::Curve3<T>,
    (speed, bend): (geom_core::SupSpeed<T>, T),
    rho: MedialDistance<T>,
    (lo, hi): (T, T),
    band: Band,
) -> Option<Vec<(T, T)>> {
    use geom_core::bisect::Step;
    let parting = match rho {
        MedialDistance::Bounded(rho) => rho,
        MedialDistance::Unbounded if geom_core::is_finite_length(bend) => T::one() / bend,
        MedialDistance::Unbounded => T::zero(),
    };
    let floor = geom_core::bisect::tangency_floor(parting, band.escalate()).hi();
    let escalate = T::from_f64(band.escalate());
    let half = T::from_f64(0.5);
    let mut out: Vec<(T, T)> = Vec::new();
    // Whether a cleared piece came after the last open one: the next open
    // piece starts a cluster.
    let mut broken = true;
    let walk = geom_core::bisect::bisect([(lo, hi)], |(a, b)| {
        let mid = geom::mid_param(a, b);
        let ell = geom_core::bisect::reach(speed, (a, b));
        let Some((dist, _, normal)) = distance(surface, carrier.eval(mid)) else {
            return Step::Stop(Unlocalized);
        };
        let tangent = carrier.deriv(mid);
        let slope = (normal.dot(tangent) / tangent.norm()).abs();
        let lipschitz = dist.abs() - ell;
        // A bend with no finite bound (a spline's crease or cusp) takes
        // the first bound alone; a plane's level sets do not bend.
        let curl = match rho {
            _ if !geom_core::is_finite_length(bend) => None,
            MedialDistance::Unbounded => Some(bend),
            MedialDistance::Bounded(rho) => {
                let axis_gap = rho - dist.abs() - ell;
                matches!(
                    decide(
                        "curve_face_window_piece_off_axis",
                        Margin::of(axis_gap),
                        band
                    ),
                    Ok(Sign::Positive)
                )
                .then(|| T::one() / axis_gap + bend)
            }
        };
        let bound = match curl {
            Some(curl) => lipschitz.max(dist.abs() - ell * slope - ell.powi(2) * curl * half),
            None => lipschitz,
        };
        if let Ok(Sign::Positive) = decide(
            "curve_face_window_piece_clear",
            Margin::of(bound - escalate),
            band,
        ) {
            broken = true;
            return Step::Settled;
        }
        if ell.hi() > floor {
            return Step::Halve([(a, mid), (mid, b)]);
        }
        match out.last_mut() {
            Some(last) if !broken => last.1 = b,
            _ => {
                if out.len() == CLUSTER_BUDGET {
                    return Step::Stop(Unlocalized);
                }
                out.push((a, b));
            }
        }
        broken = false;
        Step::Settled
    });
    match walk {
        Ok(None) => Some(out),
        Ok(Some(Unlocalized)) | Err(_) => None,
    }
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

/// **A point's signed distance from the carrier, its foot on it, and the
/// unit normal there** (the distance's gradient),
/// posed well-conditioned (standing claim 1, `[simple.conditioned]`) and
/// then read as exact: every quantity from `q − origin`, the surface's
/// stored origin; an axis's offset as `|e × n|` and its direction as
/// `n × (e × n)`, never `e − (e·n)n`, which cancels far along it; and the
/// foot from `q` itself, `q − n̂·dist`, never `centre + w·(R/|w|)`, which
/// rounds at the centre's scale however near `q` lies.
pub(super) fn distance<T: Real>(
    surface: &geom::Surface<T>,
    q: Point3<T>,
) -> Option<(T, Point3<T>, Vec3<T>)> {
    // The foot along the unit normal through `q`, from a vector along it.
    let read = |along: Vec3<T>, dist: T| {
        let n = along.normalize();
        Some((dist, q - n * dist, n))
    };
    match *surface {
        geom::Surface::Sphere { frame, radius, .. } => {
            let w = q - frame.origin();
            read(w, w.norm() - radius)
        }
        geom::Surface::Cylinder { frame, radius, .. } => {
            let (e, n) = (q - frame.origin(), frame.axis_unit().get());
            let across = e.cross(n);
            read(n.cross(across), across.norm() - radius)
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
            read(
                n.cross(across).normalize() * (rho - major_radius) + n * h,
                dist,
            )
        }
        geom::Surface::Plane { frame, .. } => {
            let n = frame.axis_unit().get();
            read(n, (q - frame.origin()).dot(n))
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
