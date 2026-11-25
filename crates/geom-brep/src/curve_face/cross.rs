//! **The curve × face crossing reading** ([`super::cross()`]): the pose
//! ([`super::pose()`]) and then the door's one method, the meetings placed
//! against the span's ends, ascending, and decided apart by their chord.
//!
//! The answer is one-sided: a [`Meeting`] is where the span meets the
//! surface, placed by its parameter on the span ([`OnSpan`]), with no
//! reading of the surface's side. A refusal ([`CrossRefusal`]) is read
//! like the boundary pass's located reading (`super::Located`): the
//! meetings it decided are exact outside its windows, and it says
//! nothing inside them. Each window is the pose's or the method's own
//! ([`super::pose::PoseRefusal`]'s rung list, [`Walked`]); one that
//! misses the span is dropped, overlapping ones are one ([`merged`]), and
//! no found meeting lies in one.

use geom_core::k_stats::decide;
use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Real, Sign};

use super::pose::{Formed, Pose, PoseRefusal, Touch};
use super::{Door, Undecided, route};
use crate::curve_pair::{MeetFault, Place, Span, SpanEnd};
use crate::roots::{RootRefusal, Unsettled};

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
    /// Where a spline chart's cells hold it: `None` on an analytic
    /// surface.
    pub chart: Option<OnChart<T>>,
}

/// **Where a meeting with a spline chart is held** ([`Meeting::chart`]):
/// a chart rectangle that holds it and no other meeting, the side the
/// span passes the chart's orientation at it, and the stretch of the
/// span that holds it.
#[derive(Clone, Copy, Debug)]
pub struct OnChart<T> {
    /// The rectangle, `[u0, u1, v0, v1]`.
    pub rect: [f64; 4],
    /// The sign of the span's direction against `S_u × S_v` there.
    pub incidence: Sign,
    /// The span's parameters the rectangle's cell spans along it, which
    /// hold the meeting's: its parameter on the span is their middle.
    pub along: (T, T),
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
    /// The pose refused, over the window its rung confines
    /// ([`PoseRefusal`]).
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

impl<T: Real> CrossRefusal<T> {
    /// **The window a reader of one window reads**: the first whose why is
    /// a fault ([`CrossWhy::fault`]), so no fault is read past behind an
    /// escalation, else the first.
    ///
    /// # Panics
    ///
    /// On a refusal with no window, which `cross` never answers.
    #[must_use]
    pub fn leading(&self) -> &Undecided<T, CrossWhy> {
        self.windows
            .iter()
            .find(|w| w.why.fault().is_some())
            .or_else(|| self.windows.first())
            .unwrap_or_else(|| unreachable!("a crossing refusal holds a window"))
    }
}

/// **What a door's method found, and where it could not read**: the
/// roots it read outside its windows, each within half a turn of the
/// span's middle on a conic, and each window with what refused it. A
/// method with nothing local answers one window over the whole span. A
/// kernel fault is no window: the method's `Err`.
#[derive(Clone, Debug)]
pub(super) struct Walked<T> {
    pub(super) roots: Vec<T>,
    pub(super) windows: Vec<((T, T), MeetFault)>,
}

impl<T: Decide> Walked<T> {
    /// Roots, every one read.
    pub(super) const fn found(roots: Vec<T>) -> Self {
        Self {
            roots,
            windows: Vec::new(),
        }
    }

    /// Nothing local: one window over the whole span `params`.
    pub(super) fn whole(params: (T, T), fault: MeetFault) -> Self {
        Self {
            roots: Vec::new(),
            windows: vec![(super::ascending(params), fault)],
        }
    }

    /// **A root core's answer**: its roots, a miss none, and a refusal one
    /// window over the whole span.
    ///
    /// # Errors
    ///
    /// A fault's readings.
    pub(super) fn of_core<const N: usize>(
        params: (T, T),
        found: Result<crate::roots::Roots<T, N, crate::roots::OnCarrier>, RootRefusal>,
    ) -> Result<Self, &'static str> {
        match found {
            Ok(found) => Ok(Self::found(roots_of(found))),
            Err(RootRefusal::Fault(fault)) => Err(fault.readings()),
            Err(why) => Ok(Self::whole(params, why.into())),
        }
    }

    /// **A walk read past its refusals** ([`crate::half_angle::ReadPast`]):
    /// its roots and its windows, a refusal of the turn one window over
    /// the whole span.
    ///
    /// # Errors
    ///
    /// A fault's readings, of the turn or of a piece.
    pub(super) fn of_walk(
        params: (T, T),
        found: Result<crate::half_angle::ReadPast<T>, RootRefusal>,
    ) -> Result<Self, &'static str> {
        match found {
            Ok(walk) => Ok(Self {
                roots: walk.roots,
                windows: walk
                    .refused
                    .into_iter()
                    .map(|(window, why)| match why {
                        RootRefusal::Fault(fault) => Err(fault.readings()),
                        why => Ok((window, why.into())),
                    })
                    .collect::<Result<_, _>>()?,
            }),
            Err(RootRefusal::Fault(fault)) => Err(fault.readings()),
            Err(why) => Ok(Self::whole(params, why.into())),
        }
    }

    /// **The windows, each the band's escalation wider than where its
    /// meetings lie** ([`super::escalation_reach`]): a method's window
    /// holds its unread meetings and no more.
    fn widened(&self, span: &Span<'_, T>, band: Band) -> impl Iterator<Item = ((T, T), MeetFault)> {
        let reach = super::escalation_reach(span, band);
        self.windows
            .iter()
            .map(move |&((lo, hi), fault)| ((lo - reach, hi + reach), fault))
    }

    /// **The windows that meet the span**, widened, each piece of each
    /// where it meets the span ([`super::on_span`]: W3, at every turn), in
    /// the method's order.
    pub(super) fn on_span(&self, span: &Span<'_, T>, band: Band) -> Vec<((T, T), MeetFault)> {
        self.widened(span, band)
            .flat_map(|(window, fault)| {
                super::on_span(span, window, band)
                    .into_iter()
                    .map(move |piece| (piece, fault))
            })
            .collect()
    }
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
    let pose = super::pose::pose(span, surface, band).map_err(|undecided| CrossRefusal {
        found: Vec::new(),
        windows: vec![Undecided {
            window: undecided.window,
            why: CrossWhy::Pose(undecided.why),
        }],
    })?;
    let params = span.params();
    let fault = |readings| Walked::whole(params, MeetFault::Contradiction { readings });
    let desync = || Walked::whole(params, MeetFault::Desync);
    let walked = match pose {
        Pose::Lies | Pose::Apart => {
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
                // decided definitely positive at some turn): that part is
                // the window ([`finish`] holds it to the span).
                windows.push(Undecided {
                    window: touch.reach,
                    why: CrossWhy::Method {
                        door,
                        fault: MeetFault::Unsettled(Unsettled::Tangency),
                    },
                });
            }
            return finish(span, pose, meetings, windows, band);
        }
        Pose::Through(apex) => match door {
            Door::ConicQuadric => super::conic_quadric::through_the_apex(span, surface, apex, band)
                .unwrap_or_else(fault),
            // A line through the apex meets the cone there alone: its
            // residual is `A·(t − t_apex)²`.
            Door::LineRoots => Walked::found(Vec::new()),
            Door::PlaneCrossing
            | Door::ConicTorus
            | Door::SpiricPlane
            | Door::SplineComposite
            | Door::SplineFace
            | Door::Spiric
            | Door::Sheet => desync(),
        },
        Pose::Isolated(form) => match door {
            Door::PlaneCrossing => plane_method(form.0).unwrap_or_else(desync),
            Door::LineRoots => match *span.carrier() {
                geom::Curve3::Line(line) => {
                    super::line_quadric::line_method(line, params, surface, form.0, band)
                        .unwrap_or_else(fault)
                }
                _ => desync(),
            },
            Door::ConicQuadric => {
                super::conic_quadric::conic_quadric_method(span, surface, form.0, band)
                    .unwrap_or_else(fault)
            }
            Door::ConicTorus => {
                super::circle_torus::torus_method(span, surface, form.0, band).unwrap_or_else(fault)
            }
            Door::SpiricPlane => match form.0 {
                Formed::SpiricPlane { line, across, sine } => {
                    super::spiric_plane::spiric_plane_method(span, (line, across, sine), band)
                        .unwrap_or_else(fault)
                }
                _ => desync(),
            },
            Door::SplineFace => return line_patch_method(span, surface, pose, band),
            Door::SplineComposite | Door::Spiric | Door::Sheet => desync(),
        },
    };
    let windows = walked
        .widened(span, band)
        .map(|(window, fault)| Undecided {
            window,
            why: CrossWhy::Method { door, fault },
        })
        .collect::<Vec<_>>();
    let (meetings, placed) = place(span, &walked.roots, band).read(door, None);
    finish(
        span,
        pose,
        meetings,
        windows.into_iter().chain(placed).collect(),
        band,
    )
}

/// **The spline face door's line method** ([`super::line_patch`]): the
/// crossings it holds, placed on the span as every door's roots are, each
/// with its cell's rectangle and incidence; its unread stretches as
/// windows.
fn line_patch_method<T: Decide>(
    span: &Span<'_, T>,
    surface: &geom::Surface<T>,
    pose: Pose<T>,
    band: Band,
) -> Result<Crossings<T>, CrossRefusal<T>> {
    use super::line_patch::{PatchRefusal, read};
    let door = Door::SplineFace;
    let whole = |fault| CrossRefusal {
        found: Vec::new(),
        windows: vec![Undecided {
            window: super::ascending(span.params()),
            why: CrossWhy::Method { door, fault },
        }],
    };
    let (&geom::Curve3::Line(line), Some(chart)) = (span.carrier(), surface.spline_chart()) else {
        return Err(whole(MeetFault::Desync));
    };
    let (found, unread) = match read(&line, span.params(), chart, band) {
        Ok(found) => (found, Vec::new()),
        Err(PatchRefusal::Stretches { found, unread }) => (found, unread),
        Err(PatchRefusal::Whole(fault)) => return Err(whole(fault)),
    };
    let two = T::from_f64(2.0);
    let roots: Vec<T> = found
        .iter()
        .map(|c| (c.advance.0 + c.advance.1) / two)
        .collect();
    let charts: Vec<OnChart<T>> = found
        .iter()
        .map(|c| OnChart {
            rect: c.rect,
            incidence: c.incidence,
            along: c.advance,
        })
        .collect();
    let (meetings, placed) = place(span, &roots, band).read_charted(door, None, &charts);
    let windows = unread
        .into_iter()
        .map(|u| Undecided {
            window: u.window,
            why: CrossWhy::Method {
                door,
                fault: u.fault,
            },
        })
        .chain(placed)
        .collect();
    finish(span, pose, meetings, windows, band)
}

/// **The answer, or the refusal where any window stands** (W1, W3, W4):
/// each window held to the span where it meets it ([`super::on_span`],
/// at every turn); a found meeting the band cannot read outside a window
/// is taken into it, so no found meeting lies in one; then windows that
/// overlap or touch within the band are one ([`merged`]).
pub(super) fn finish<T: Decide>(
    span: &Span<'_, T>,
    pose: Pose<T>,
    meetings: Vec<(Meeting<T>, T)>,
    windows: Vec<Undecided<T, CrossWhy>>,
    band: Band,
) -> Result<Crossings<T>, CrossRefusal<T>> {
    let mut windows: Vec<Undecided<T, CrossWhy>> = windows
        .into_iter()
        .flat_map(|w| {
            super::on_span(span, w.window, band)
                .into_iter()
                .map(move |window| Undecided { window, why: w.why })
        })
        .collect();
    if windows.is_empty() {
        return Ok(Crossings {
            pose,
            meetings: meetings.into_iter().map(|(meeting, _)| meeting).collect(),
        });
    }
    let speed = span.speeds().0.get();
    let mut found = Vec::with_capacity(meetings.len());
    for (meeting, t) in meetings {
        match windows.iter().position(|w| {
            super::undecided_over(core::slice::from_ref(w), (t, t), speed, band).is_some()
        }) {
            // Taken into the window as a window of its own escalation
            // about it (W2), held to the span.
            Some(k) => {
                let (lo, hi) = windows[k].window;
                let reach = super::escalation_reach(span, band);
                let (t0, t1) = super::ascending(span.params());
                windows[k].window = ((t - reach).max(t0).min(lo), (t + reach).min(t1).max(hi));
            }
            None => found.push(meeting),
        }
    }
    Err(CrossRefusal {
        found,
        windows: merged(span, windows, band),
    })
}

/// **W4**: windows that overlap, or stand apart by a gap that is not
/// definitely positive (`curve_face_window`, at the span's least speed),
/// are one window over their hull, ascending. Its why is a fault's
/// ([`CrossWhy::fault`]) ahead of anything else, otherwise the earlier
/// window's (by its start, `curve_face_window`; a level start keeps the
/// one read first).
pub(super) fn merged<T: Decide>(
    span: &Span<'_, T>,
    windows: Vec<Undecided<T, CrossWhy>>,
    band: Band,
) -> Vec<Undecided<T, CrossWhy>> {
    let speed = span.speeds().0.get();
    let positive = |gap: T| matches!(super::window_gap(gap, speed, band), Ok(Sign::Positive));
    let before =
        |a: &Undecided<T, CrossWhy>, b: &Undecided<T, CrossWhy>| positive(b.window.0 - a.window.1);
    let mut out: Vec<Undecided<T, CrossWhy>> = Vec::with_capacity(windows.len());
    for mut window in windows {
        while let Some(k) = out
            .iter()
            .position(|o| !before(o, &window) && !before(&window, o))
        {
            let held = out.remove(k);
            let why = match (held.why.fault(), window.why.fault()) {
                (Some(_), _) => held.why,
                (None, Some(_)) => window.why,
                (None, None) if positive(held.window.0 - window.window.0) => window.why,
                (None, None) => held.why,
            };
            window = Undecided {
                window: (
                    held.window.0.min(window.window.0),
                    held.window.1.max(window.window.1),
                ),
                why,
            };
        }
        let at = out
            .iter()
            .position(|o| before(&window, o))
            .unwrap_or(out.len());
        out.insert(at, window);
    }
    out
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

/// **The plane arm's method**: the roots its pose computed in closed
/// form. `None` for a form of another arm.
fn plane_method<T: Decide>(form: Formed<T>) -> Option<Walked<T>> {
    match form {
        Formed::PlaneLine { root } => Some(Walked::found(vec![root])),
        Formed::PlaneConic { roots } => Some(Walked::found(roots.to_vec())),
        _ => None,
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
    /// Readings that contradict: a kernel fault.
    Contradiction(&'static str),
}

/// Two roots apart past the band by their chord read level along the span.
const ORDER_CONTRADICTS: &str =
    "two roots apart past the band by their chord read level along the span";

/// [`Placed::read`]'s answer: each meeting with its parameter, and the
/// windows.
type Read<T> = (Vec<(Meeting<T>, T)>, Vec<Undecided<T, CrossWhy>>);

/// **The roots placed on the span** ([`place`]): the meetings, ascending,
/// and the roots it could not place, each with its window.
pub(super) struct Placed<T: Real> {
    pub(super) meetings: Vec<(T, OnSpan<T>)>,
    pub(super) refused: Vec<((T, T), PlaceWhy<T>)>,
    points: Vec<Point3<T>>,
    /// Each meeting's root, by its index among the roots placed.
    roots: Vec<usize>,
}

impl<T: Decide> Placed<T> {
    /// The meetings, each a crossing or the pose's touch with its
    /// parameter, and the windows, each the method's.
    fn read(self, door: Door, touch: Option<Touch<T>>) -> Read<T> {
        self.read_charted(door, touch, &[])
    }

    /// [`Self::read`], each meeting held where `charts` (each root's,
    /// by its index among the roots placed) says a spline chart holds it.
    fn read_charted(self, door: Door, touch: Option<Touch<T>>, charts: &[OnChart<T>]) -> Read<T> {
        let kind = touch.map_or(MeetingKind::Cross, |t| MeetingKind::Touch { arc: t.arc });
        let meetings = self
            .meetings
            .into_iter()
            .zip(self.points)
            .zip(self.roots)
            .map(|(((t, on_span), point), root)| {
                (
                    Meeting {
                        point,
                        on_span,
                        kind,
                        chart: charts.get(root).copied(),
                    },
                    t,
                )
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
                        PlaceWhy::Contradiction(readings) => MeetFault::Contradiction { readings },
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
    let reach = super::escalation_reach(span, band);
    let mut inside: Vec<(T, OnSpan<T>, Point3<T>, usize)> = Vec::new();
    let mut refused = Vec::new();
    for (index, &root) in roots.iter().enumerate() {
        let t = if periodic {
            geom_core::period::Period::angle().near(root, mid)
        } else {
            root
        };
        match span.place_param(t, band) {
            Ok(Place::Inside) => inside.push((t, OnSpan::Inside { t }, carrier.eval(t), index)),
            Ok(Place::End(end)) => inside.push((t, OnSpan::End(end), carrier.eval(t), index)),
            Ok(Place::Past) => {}
            Err(diag) => refused.push(((t - reach, t + reach), PlaceWhy::End(diag))),
        }
    }
    // Insertion by decided order: a pair the band cannot part is a
    // window over both.
    let mut ordered: Vec<(T, OnSpan<T>, Point3<T>, usize)> = Vec::with_capacity(inside.len());
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
                // Apart past the band by their chord, which is at most
                // their arc: level along the span is a contradiction.
                Ok(Sign::Zero) => {
                    refused.push((window, PlaceWhy::Contradiction(ORDER_CONTRADICTS)));
                    ordered.remove(at - 1);
                    continue 'next;
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
    let indices = ordered.iter().map(|m| m.3).collect();
    Placed {
        meetings: ordered.into_iter().map(|(t, on, _, _)| (t, on)).collect(),
        refused,
        points,
        roots: indices,
    }
}
