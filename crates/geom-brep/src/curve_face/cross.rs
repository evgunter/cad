//! **The curve × face crossing reading** ([`super::cross()`]): the pose
//! ([`super::pose()`]) and then the door's one method, the meetings placed
//! against the span's ends, ascending, and decided apart by their chord.
//!
//! The answer is one-sided: a [`Meeting`] is where the span meets the
//! surface, placed by its parameter on the span ([`OnSpan`]), with no
//! reading of the surface's side. A refusal ([`CrossRefusal`]) is read
//! like the boundary pass's located reading (`super::Located`): the
//! meetings it decided are exact outside its windows, and it says
//! nothing inside them. In U3 a pose refusal and a method's refusal are
//! each one window over the whole span; a root in band of an end, and
//! two roots in band of each other, are local windows.

use geom_core::k_stats::decide;
use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Real, Sign};

use super::pose::{Formed, Pose, PoseRefusal, Touch};
use super::{Door, Undecided, route};
use crate::curve_pair::{MeetFault, Place, Span, SpanEnd};
use crate::roots::Unsettled;

/// **Where a meeting sits on the span**: strictly inside it, at its
/// parameter, or at one of its ends. A root placed by its parameter has
/// no on-carrier reading past the span, so there is no `Past`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OnSpan<T> {
    /// Strictly inside the span.
    Inside {
        /// The carrier's parameter, within half a turn of the span's
        /// middle on a conic.
        t: T,
    },
    /// Within the band of an end's point.
    End(SpanEnd),
}

/// **What a meeting is** ([`Meeting`]).
#[derive(Clone, Copy, Debug)]
pub enum MeetingKind<T> {
    /// A crossing: the residual changes sign there.
    Cross,
    /// The pose's double root ([`Pose::Tangent`]): `arc` holds every span
    /// parameter about it whose point is within the band of the surface.
    Touch {
        /// The in-band arc, never understated on the span
        /// ([`super::pose::Touch::arc`]).
        arc: (T, T),
    },
}

/// **One meeting of a span with a surface.**
#[derive(Clone, Copy, Debug)]
pub struct Meeting<T: Real> {
    /// The carrier's point at the meeting.
    pub point: Point3<T>,
    /// Where it sits on the span.
    pub on_span: OnSpan<T>,
    /// A crossing or a touch.
    pub kind: MeetingKind<T>,
}

/// **The crossing reading's answer** ([`super::cross()`]): the pose, and
/// the meetings, ascending along the span and decided apart by their
/// chord. `Lies` and `Apart` hold none, `Tangent` its one touch where it
/// is on the span, `Through` the meetings other than the singular point.
#[derive(Clone, Debug)]
pub struct Crossings<T: Real> {
    /// The pose.
    pub pose: Pose<T>,
    /// The meetings.
    pub meetings: Vec<Meeting<T>>,
}

/// **Why a stretch of the span was not read** ([`CrossRefusal`]).
#[derive(Clone, Copy, Debug)]
pub enum CrossWhy {
    /// The pose refused: one window over the whole span.
    Pose(PoseRefusal),
    /// The door's method, or the placement of what it found, refused.
    Method {
        /// The door.
        door: Door,
        /// What refused.
        fault: MeetFault,
    },
}

impl CrossWhy {
    /// **The window's fault, where it is one**: readings that contradict,
    /// or a carrier or surface an arm does not read. A kernel fault,
    /// never a stretch the band leaves undecided: a reader fails loud on
    /// it rather than set it aside.
    #[must_use]
    pub fn fault(self) -> Option<&'static str> {
        match self {
            Self::Pose(PoseRefusal::Fault(what)) => Some(what),
            Self::Method {
                fault: MeetFault::Contradiction { readings },
                ..
            } => Some(readings),
            Self::Method {
                fault: MeetFault::Desync,
                ..
            } => Some("a carrier read by an arm not its own"),
            Self::Pose(_) | Self::Method { .. } => None,
        }
    }

    /// **The window's diagnostics, where it was read in the band**: a
    /// pose rung's, or the placement's or a method's escalation (a root in
    /// band of an end, two roots the band cannot order). `None` for a
    /// window no margin reads: a limit, an unbuilt or enclosed arm, a
    /// fault.
    #[must_use]
    pub fn diag(self) -> Option<Indeterminate> {
        match self {
            Self::Pose(PoseRefusal::InBand { diag, .. })
            | Self::Method {
                fault: MeetFault::Escalated(diag) | MeetFault::Touch(diag),
                ..
            } => Some(diag),
            Self::Pose(_) | Self::Method { .. } => None,
        }
    }
}

/// **The crossing reading, read past what it could not read**: the
/// meetings decided outside the windows, never the complete set, and the
/// windows of the span it could not read. Exact outside the windows,
/// silent inside them.
#[derive(Clone, Debug)]
pub struct CrossRefusal<T: Real> {
    /// The meetings decided outside every window, ascending.
    pub found: Vec<Meeting<T>>,
    /// The stretches not read.
    pub windows: Vec<Undecided<T, CrossWhy>>,
}

/// **The pose, then the door's one method** ([`super::cross()`]).
///
/// # Errors
///
/// [`CrossRefusal`]: its windows, and the meetings outside them.
pub fn cross<T: Decide>(
    span: &Span<'_, T>,
    surface: &geom::Surface<T>,
    band: Band,
) -> Result<Crossings<T>, CrossRefusal<T>> {
    let door = route(span.carrier().kind(), surface.kind()).door;
    let whole = |why| CrossRefusal {
        found: Vec::new(),
        windows: vec![Undecided {
            window: span.params(),
            why,
        }],
    };
    let pose = super::pose::pose(span, surface, band).map_err(|why| whole(CrossWhy::Pose(why)))?;
    let method = |fault: MeetFault| whole(CrossWhy::Method { door, fault });
    let roots: Vec<T> = match pose {
        Pose::Lies(_) | Pose::Apart => {
            return Ok(Crossings {
                pose,
                meetings: Vec::new(),
            });
        }
        Pose::Tangent(touch) => {
            let placed = place(span, &[touch.at], band);
            let past = placed.meetings.is_empty() && placed.refused.is_empty();
            let (meetings, mut windows) = placed.read(door, Some(touch));
            if past {
                // A touch past the span is not `Apart` only where its reach
                // at `Kε` covers part of the span (`pose`'s `reaches`,
                // decided definitely positive): that part is the window,
                // never nothing and never a point.
                let (t0, t1) = span.params();
                windows.push(Undecided {
                    window: (t0.min(t1).max(touch.reach.0), t1.max(t0).min(touch.reach.1)),
                    why: CrossWhy::Method {
                        door,
                        fault: MeetFault::Unsettled(Unsettled::Tangency),
                    },
                });
            }
            return finish(pose, meetings, windows);
        }
        Pose::Through(apex) => match door {
            Door::ConicQuadric => {
                match super::conic_quadric::through_the_apex(span, surface, apex, band) {
                    Ok(found) => roots_of(found),
                    Err(why) => return Err(method(MeetFault::Unsettled(why))),
                }
            }
            // A line through the apex meets the cone there alone: its
            // residual is `A·(t − t_apex)²`.
            Door::LineRoots => Vec::new(),
            Door::PlaneCrossing
            | Door::ConicTorus
            | Door::SplineComposite
            | Door::SplineFace
            | Door::Spiric
            | Door::Sheet => return Err(method(MeetFault::Desync)),
        },
        Pose::Isolated(form) => match door {
            Door::PlaneCrossing => plane_method(form.0).map_err(method)?,
            Door::LineRoots => {
                let &geom::Curve3::Line(line) = span.carrier() else {
                    return Err(method(MeetFault::Desync));
                };
                match super::line_quadric::line_method(line, span.params(), surface, form.0, band) {
                    Ok(found) => roots_of(found),
                    Err(refusal) => return Err(method(refusal.into())),
                }
            }
            Door::ConicQuadric => {
                match super::conic_quadric::conic_quadric_method(span, surface, form.0, band) {
                    Ok(Ok(found)) => roots_of(found),
                    Ok(Err(why)) => return Err(method(MeetFault::Unsettled(why))),
                    Err(refusal) => return Err(method(door_fault(refusal))),
                }
            }
            Door::ConicTorus => {
                match super::circle_torus::torus_method(span, surface, form.0, band) {
                    Ok(Ok(found)) => roots_of(found),
                    Ok(Err(why)) => return Err(method(MeetFault::Unsettled(why))),
                    Err(refusal) => match refusal.without_torus() {
                        Ok(refusal) => return Err(method(door_fault(refusal))),
                        Err(torus) => return Err(whole(CrossWhy::Pose(PoseRefusal::Torus(torus)))),
                    },
                }
            }
            Door::SplineComposite | Door::SplineFace | Door::Spiric | Door::Sheet => {
                return Err(method(MeetFault::Desync));
            }
        },
    };
    let (meetings, windows) = place(span, &roots, band).read(door, None);
    finish(pose, meetings, windows)
}

/// The answer, or the refusal where any window stands.
fn finish<T: Real>(
    pose: Pose<T>,
    meetings: Vec<Meeting<T>>,
    windows: Vec<Undecided<T, CrossWhy>>,
) -> Result<Crossings<T>, CrossRefusal<T>> {
    if windows.is_empty() {
        Ok(Crossings { pose, meetings })
    } else {
        Err(CrossRefusal {
            found: meetings,
            windows,
        })
    }
}

/// A root core's answer's roots: none for a miss. The pose has decided
/// `Lies`, so an `On` from a method is a contradiction the caller never
/// sees: the method reads no constant residual.
fn roots_of<T: Real, const N: usize>(
    found: crate::roots::Roots<T, N, crate::roots::OnCarrier>,
) -> Vec<T> {
    match found {
        crate::roots::Roots::Certified(found) => found.roots().to_vec(),
        crate::roots::Roots::Miss | crate::roots::Roots::On(_) => Vec::new(),
    }
}

/// A root door's refusal, as a meeting reading's.
fn door_fault(refusal: super::circle_roots::DoorRefusal) -> MeetFault {
    match refusal {
        super::circle_roots::DoorRefusal::Escalated(diag) => MeetFault::Escalated(diag),
        super::circle_roots::DoorRefusal::Invariant(readings) => {
            MeetFault::Contradiction { readings }
        }
        super::circle_roots::DoorRefusal::Torus(never) => match never {},
    }
}

/// **The plane arm's method**: the roots its pose computed in closed
/// form.
fn plane_method<T: Real>(form: Formed<T>) -> Result<Vec<T>, MeetFault> {
    match form {
        Formed::PlaneLine { root } => Ok(vec![root]),
        Formed::PlaneConic { roots } => Ok(roots.to_vec()),
        _ => Err(MeetFault::Desync),
    }
}

/// **Why the placement could not read a root** ([`Placed`]).
#[derive(Clone, Copy, Debug)]
pub(super) enum PlaceWhy<T> {
    /// A root's place against an end in the band's gap.
    End(Indeterminate),
    /// Two roots within the band of each other by their chord: one
    /// meeting the band cannot part, at the first's parameter, a window
    /// over both.
    Together(T),
    /// Two roots' chord, or their order, in the band's gap.
    Apart(Indeterminate),
}

/// **The roots placed on the span** ([`place`]): the meetings, ascending,
/// and the roots it could not place, each with its window.
pub(super) struct Placed<T: Real> {
    pub(super) meetings: Vec<(T, OnSpan<T>)>,
    pub(super) refused: Vec<((T, T), PlaceWhy<T>)>,
    points: Vec<Point3<T>>,
}

impl<T: Decide> Placed<T> {
    /// The meetings, each a crossing or the pose's touch, and the
    /// windows, each the method's.
    fn read(
        self,
        door: Door,
        touch: Option<Touch<T>>,
    ) -> (Vec<Meeting<T>>, Vec<Undecided<T, CrossWhy>>) {
        let kind = touch.map_or(MeetingKind::Cross, |t| MeetingKind::Touch { arc: t.arc });
        let meetings = self
            .meetings
            .into_iter()
            .zip(self.points)
            .map(|((_, on_span), point)| Meeting {
                point,
                on_span,
                kind,
            })
            .collect();
        let windows = self
            .refused
            .into_iter()
            .map(|(window, why)| Undecided {
                window,
                why: CrossWhy::Method {
                    door,
                    fault: match why {
                        PlaceWhy::End(diag) => MeetFault::Escalated(diag),
                        PlaceWhy::Apart(diag) => MeetFault::Touch(diag),
                        PlaceWhy::Together(_) => MeetFault::Unsettled(Unsettled::Tangency),
                    },
                },
            })
            .collect();
        (meetings, windows)
    }
}

/// **The roots placed on the span** (`curve_face_cross_*`): each root,
/// named within half a turn of the span's middle on a periodic carrier,
/// read against the span's ends by the span's own reading
/// ([`Span::place_param`]): inside, at an end by its chord, past (no
/// meeting), or in band of an end (a window of the escalation band about
/// it, at the span's least speed). The meetings inside are then ordered
/// ascending, each pair decided apart by its chord
/// (`curve_face_cross_apart`) and ordered by its parameters
/// (`curve_face_cross_order`, at the carrier's greatest speed, a chord
/// being at most its arc): two within the band of each other are a
/// window over both, neither kept: the found meetings sit outside every
/// window.
pub(super) fn place<T: Decide>(span: &Span<'_, T>, roots: &[T], band: Band) -> Placed<T> {
    let carrier = span.carrier();
    let (t0, t1) = span.params();
    let mid = geom::mid_param(t0, t1);
    let periodic = carrier.conic().is_some();
    let reach = T::from_f64(band.escalate()) / span.speeds().0.get();
    let mut inside: Vec<(T, OnSpan<T>, Point3<T>)> = Vec::new();
    let mut refused = Vec::new();
    for &root in roots {
        let t = if periodic {
            geom_core::period::Period::angle().near(root, mid)
        } else {
            root
        };
        match span.place_param(t, band) {
            Ok(Place::Inside) => inside.push((t, OnSpan::Inside { t }, carrier.eval(t))),
            Ok(Place::End(end)) => inside.push((t, OnSpan::End(end), carrier.eval(t))),
            Ok(Place::Past) => {}
            Err(diag) => refused.push(((t - reach, t + reach), PlaceWhy::End(diag))),
        }
    }
    // Insertion by decided order: a pair the band cannot part is a
    // window over both.
    let mut ordered: Vec<(T, OnSpan<T>, Point3<T>)> = Vec::with_capacity(inside.len());
    'next: for item in inside {
        let mut at = ordered.len();
        while at > 0 {
            let before = &ordered[at - 1];
            let window = (before.0.min(item.0), before.0.max(item.0));
            // A pair the band cannot part is one window over both, and
            // neither is a found meeting: found meetings sit outside every
            // window.
            match decide(
                "curve_face_cross_apart",
                Margin::norm3(item.2 - before.2),
                band,
            ) {
                Ok(Sign::Zero) => {
                    refused.push((window, PlaceWhy::Together(before.0)));
                    ordered.remove(at - 1);
                    continue 'next;
                }
                Ok(Sign::Positive | Sign::Negative) => {}
                Err(diag) => {
                    refused.push((window, PlaceWhy::Apart(diag)));
                    ordered.remove(at - 1);
                    continue 'next;
                }
            }
            match decide(
                "curve_face_cross_order",
                Margin::metered_sup(item.0 - before.0, span.speeds().1),
                band,
            ) {
                Ok(Sign::Positive) => break,
                Ok(Sign::Negative) => at -= 1,
                Ok(Sign::Zero) => {
                    unreachable!("roots apart past the band read level along the span")
                }
                Err(diag) => {
                    refused.push((window, PlaceWhy::Apart(diag)));
                    ordered.remove(at - 1);
                    continue 'next;
                }
            }
        }
        ordered.insert(at, item);
    }
    let points = ordered.iter().map(|m| m.2).collect();
    Placed {
        meetings: ordered.into_iter().map(|(t, on, _)| (t, on)).collect(),
        refused,
        points,
    }
}
