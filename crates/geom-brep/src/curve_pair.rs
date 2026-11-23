//! **The curve-pair meeting reader**: whether, and where, two carrier
//! spans meet (C5 in `crates/geom-brep/README.md`, D3's "One reading per
//! question").
//!
//! - **One point-on-span reading** ([`span`]): a candidate that names a
//!   point is settled by [`Span::contact`] on both spans, at its foot on
//!   each ([`geom::Curve3::foot`], seeded by the candidate's own
//!   parameter). It composes `geom`'s one-carrier reading with the
//!   span's reading of its ends ([`Span::read`]), and decides nothing of
//!   its own. A candidate that names a parameter on each span (the walk's
//!   polished pair, and the pose's closest pair, each polish held to the
//!   two spans) is read as that pair instead: its own gap and each
//!   parameter's place on its span, with no foot ([`Found::read_pair`],
//!   [`Found::read_posed`]).
//! - **One total table** ([`method`]): one exhaustive match over the two
//!   carriers' [`CurveKind`]s, with no wildcard arm, fixing each pair's
//!   method at write time. Line, circle and ellipse pairs are closed
//!   forms in one carrier's own parameter, measured in metres; a pair
//!   with a spiric or spline side is the certified walk ([`walk`]). No
//!   method is reached because another declined.
//! - **One pose** ([`pose()`]): a closed-form pair's classification,
//!   decided once; the outcome takes it as input and decides no pose of
//!   its own.
//! - **One entry point** ([`meet`]): the two spans and the band in, the
//!   meetings or a typed refusal out. Its dispatch is the table.
//!
//! **The band.** A candidate generator's in-band decision escalates and
//! never prunes, and a candidate admitted under a `Zero` reading is
//! placed by the exactly degenerate formula ([`mod@pose`]'s docs). A
//! candidate read in the band on one span and decided off the other is
//! not a meeting; read in the band on both, or on one and on the other,
//! it escalates, carrying every meeting already decided.
//!
//! **Stretches** (C5). A *stretch* is a longest interval of one span
//! nowhere definitely off the other, and a *meeting* a longest interval
//! everywhere definitely on it: a point for a crossing or a touch
//! ([`Meeting`]), a longer interval for a shared run
//! ([`Meetings::shared`]). Each span's ends are read against the other
//! first. A candidate nearer a found meeting than the band's escalation
//! width cannot be decided distinct from it, so it is not read again,
//! and a refusal held at such a point is no refusal of the answer
//! (`ringed`). A stretch holding one meeting is reported as that
//! meeting; two meetings in one stretch escalate (`stretches`). A
//! closed form whose equation is the meeting's own divides the roots of
//! the meetings its ends found out of it; one across a plane, whose roots
//! are crossings of the plane and not meetings, divides nothing out
//! ([`mod@pose`]'s docs, "deflation"). The reader is given
//! no topology: two edges joined at a vertex meet there because their
//! ends meet.
//!
//! **The walk is the closed forms' differential test oracle** (the rows),
//! never a runtime fallback.

pub mod on_surface;
mod polish;
pub mod pose;
pub mod span;
mod walk;

pub use pose::{Candidate, Pose, PoseRefusal, Sense, WindowBound, WindowSpans, pose};
pub use span::{AtFoot, GapMeter, Hit, OffBy, PastBy, Place, Reading, Span, SpanEnd, SpanRefusal};

use geom::CurveKind;
use geom_core::{Band, Decide, Indeterminate, Point3};

use crate::roots::{RootRefusal, Unsettled};

/// How the table reads a pair of kinds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Method {
    /// A closed form in one carrier's parameter, through [`pose()`].
    Closed(Arm),
    /// The certified walk.
    Walk,
}

/// A closed-form arm of the table: which carrier is which, fixed by the
/// kinds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm {
    /// Two lines.
    LineLine,
    /// A line, then a circle or an ellipse.
    LineConic,
    /// A circle or an ellipse, then a line.
    ConicLine,
    /// Two circles.
    CircleCircle,
    /// An ellipse, then a circle.
    EllipseCircle,
    /// A circle, then an ellipse.
    CircleEllipse,
    /// Two ellipses.
    EllipseEllipse,
}

/// **The table**: the method every pair of kinds is read by. Exhaustive,
/// with no wildcard arm: a new [`CurveKind`] is a compile error here until
/// it has a method.
#[must_use]
pub fn method(a: CurveKind, b: CurveKind) -> Method {
    use CurveKind::{Circle, Ellipse, Line, Nurbs, Periodic, Spiric};
    match (a, b) {
        (Line, Line) => Method::Closed(Arm::LineLine),
        (Line, Circle | Ellipse) => Method::Closed(Arm::LineConic),
        (Circle | Ellipse, Line) => Method::Closed(Arm::ConicLine),
        (Circle, Circle) => Method::Closed(Arm::CircleCircle),
        (Ellipse, Circle) => Method::Closed(Arm::EllipseCircle),
        (Circle, Ellipse) => Method::Closed(Arm::CircleEllipse),
        (Ellipse, Ellipse) => Method::Closed(Arm::EllipseEllipse),
        // A periodic cubic walks as every carrier with no closed form
        // does, its pieces read by evaluation.
        (Line | Circle | Ellipse | Spiric | Nurbs | Periodic, Spiric | Nurbs | Periodic)
        | (Spiric | Nurbs | Periodic, Line | Circle | Ellipse) => Method::Walk,
    }
}

/// A meeting at a point, a crossing or a touch: a point of both spans,
/// and where it sits on each. A shared run, the other kind of meeting,
/// is [`Meetings::shared`].
#[derive(Clone, Copy, Debug)]
pub struct Meeting<T: geom_core::Real> {
    /// The point.
    pub point: Point3<T>,
    /// Where it sits on the first span.
    pub on_a: Hit<T>,
    /// Where it sits on the second span.
    pub on_b: Hit<T>,
}

/// What [`meet`] found (C5's stretch rule, `crates/geom-brep/README.md`).
#[derive(Clone, Debug)]
pub struct Meetings<T: geom_core::Real> {
    /// `Some` where the pose read the carriers as one curve: the spans'
    /// meeting is then the shared run of that curve. `points` are the
    /// run's witnesses that read on both spans; none read on both where
    /// the spans share no run of the curve, which is `Some` with no
    /// points.
    pub shared: Option<Sense>,
    /// The meetings at a point, each read on both spans and each the one
    /// meeting of its stretch: a candidate nearer a found meeting than
    /// the band's escalation width is not read again, and a stretch
    /// holding two meetings escalates.
    pub points: Vec<Meeting<T>>,
}

/// Why [`meet`] gave no answer.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MeetFault {
    /// A decision fell in the band's gap. Ranked by its margin.
    Escalated(Indeterminate),
    /// A root finder read the pair in the band's gap where they can meet
    /// ([`RootRefusal::Touch`]): a meeting in band, which escalates where
    /// the spans are the model's and is the probe's where the caller
    /// chose one. Ranked by its margin.
    Touch(Indeterminate),
    /// A root finder, a meter or the walk's budget reached its limit
    /// before it could certify the candidates. Unsettled: no margin, and
    /// a coarser band may settle it.
    Unsettled(Unsettled),
    /// A spline's piece walk could not run, reading a point on a span. As
    /// the walk's refusal stands ([`geom::PieceWalkRefusal::refusal`]).
    Unwalked(geom::PieceWalkRefusal),
    /// A pose the pair's arm has no closed form for. A limit.
    Unsupported {
        /// The pose.
        pose: &'static str,
    },
    /// A carrier is not of the kind its arm reads. An abort: a kernel
    /// fault.
    Desync,
    /// Two readings of the pose contradict each other, which their
    /// definitions rule out but for rounding: a kernel fault, never a
    /// pose. An abort.
    Contradiction {
        /// The readings.
        readings: &'static str,
    },
}

impl MeetFault {
    /// What this refusal is to a ladder over the readings that meet it
    /// ([`crate::ladder::Refusal`]): its group's ([`MeetKind::refusal`]).
    ///
    /// # Errors
    ///
    /// [`crate::ladder::Aborts`] for a carrier the arm misread or two
    /// readings that contradict: a kernel fault, never a pose.
    pub fn refusal(&self) -> Result<crate::ladder::Refusal, crate::ladder::Aborts> {
        self.kind().refusal()
    }
}

/// **What a [`MeetFault`] is to a consumer that does not tell its
/// limits apart** ([`MeetFault::kind`]): the one grouping of its arms,
/// so no consumer spells the group by hand.
#[derive(Clone, Copy, Debug)]
pub enum MeetKind {
    /// A decision fell in the band's gap ([`MeetFault::Escalated`]).
    Escalated(Indeterminate),
    /// A meeting in band ([`MeetFault::Touch`]).
    Touch(Indeterminate),
    /// The reading did not run to an answer, and no margin says how far
    /// it was ([`MeetUnread`]).
    Unread(MeetUnread),
    /// A kernel fault, never a pose: the carrier misread, or the readings
    /// that contradict.
    Fault {
        /// What faulted.
        what: &'static str,
    },
}

impl MeetKind {
    /// What this group is to a ladder ([`crate::ladder::Refusal`]): an
    /// escalation or a touch ranked by its margin, an unread reading as
    /// it stands ([`MeetUnread::refusal`]), a kernel fault an abort. The
    /// one table of a meeting refusal's standing.
    ///
    /// # Errors
    ///
    /// [`crate::ladder::Aborts`] for a kernel fault.
    pub fn refusal(self) -> Result<crate::ladder::Refusal, crate::ladder::Aborts> {
        use crate::ladder::{Aborts, Refusal};
        match self {
            Self::Escalated(diag) | Self::Touch(diag) => Ok(Refusal::Margin(diag)),
            Self::Unread(why) => why.refusal(),
            Self::Fault { .. } => Err(Aborts),
        }
    }
}

/// **A meeting reading that did not run to an answer**, with no margin
/// to say how far it was ([`MeetKind::Unread`]): the fault itself,
/// narrowed to [`MeetFault::Unsettled`], [`MeetFault::Unwalked`] or
/// [`MeetFault::Unsupported`] by its one constructor ([`Self::of`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MeetUnread(MeetFault);

impl MeetUnread {
    /// `fault` where it is a reading that did not run to an answer.
    #[must_use]
    pub const fn of(fault: MeetFault) -> Option<Self> {
        match fault {
            MeetFault::Unsettled(_) | MeetFault::Unwalked(_) | MeetFault::Unsupported { .. } => {
                Some(Self(fault))
            }
            MeetFault::Escalated(_)
            | MeetFault::Touch(_)
            | MeetFault::Desync
            | MeetFault::Contradiction { .. } => None,
        }
    }

    /// The fault it is.
    #[must_use]
    pub const fn fault(self) -> MeetFault {
        self.0
    }

    /// Its standing: the finder's ([`Unsettled::refusal`]), the walk's
    /// ([`geom::PieceWalkRefusal::refusal`]), or a limit for a pose with
    /// no closed form.
    ///
    /// # Errors
    ///
    /// [`crate::ladder::Aborts`] where the finder's does
    /// ([`Unsettled::refusal`]).
    pub fn refusal(self) -> Result<crate::ladder::Refusal, crate::ladder::Aborts> {
        match self.0 {
            MeetFault::Unsettled(why) => why.refusal(),
            MeetFault::Unwalked(why) => Ok(why.refusal()),
            MeetFault::Unsupported { .. } => Ok(crate::ladder::Refusal::Limit),
            MeetFault::Escalated(_)
            | MeetFault::Touch(_)
            | MeetFault::Desync
            | MeetFault::Contradiction { .. } => {
                unreachable!("a MeetUnread holds an unread arm (MeetUnread::of)")
            }
        }
    }

    /// The one sentence it is rendered by: what the finder met
    /// ([`Unsettled::sentence`]), the walk's
    /// ([`geom::PieceWalkRefusal::sentence`]), or the pose.
    #[must_use]
    pub fn sentence(self) -> &'static str {
        match self.0 {
            MeetFault::Unsettled(why) => why.sentence(),
            MeetFault::Unwalked(why) => why.sentence(),
            MeetFault::Unsupported { pose } => pose,
            MeetFault::Escalated(_)
            | MeetFault::Touch(_)
            | MeetFault::Desync
            | MeetFault::Contradiction { .. } => {
                unreachable!("a MeetUnread holds an unread arm (MeetUnread::of)")
            }
        }
    }

    /// The pose the arm has no closed form for, where that is why.
    #[must_use]
    pub const fn pose(self) -> Option<&'static str> {
        match self.0 {
            MeetFault::Unsupported { pose } => Some(pose),
            _ => None,
        }
    }

    /// The recourse its rendering ends with: its standing's
    /// ([`crate::ladder::Refusal::recourse`]). A reason that aborts is a
    /// payload that mixes classes and aborts until it is split (a meeting
    /// at a spline's crease): its ending is the unbuilt split's, no way
    /// through yet, not a kernel defect.
    #[must_use]
    pub fn recourse(self) -> crate::ladder::Ending {
        match self.refusal() {
            Ok(standing) => standing.recourse(),
            Err(crate::ladder::Aborts) => crate::ladder::Ending::NotYet,
        }
    }
}

impl MeetFault {
    /// This refusal's group ([`MeetKind`]).
    #[must_use]
    pub const fn kind(&self) -> MeetKind {
        match self {
            Self::Escalated(diag) => MeetKind::Escalated(*diag),
            Self::Touch(diag) => MeetKind::Touch(*diag),
            Self::Unsettled(_) | Self::Unwalked(_) | Self::Unsupported { .. } => {
                MeetKind::Unread(MeetUnread(*self))
            }
            Self::Desync => MeetKind::Fault {
                what: "a carrier the meeting table misread",
            },
            Self::Contradiction { readings } => MeetKind::Fault { what: readings },
        }
    }
}

impl From<RootRefusal> for MeetFault {
    fn from(refusal: RootRefusal) -> Self {
        match refusal {
            RootRefusal::Escalated(diag) => Self::Escalated(diag),
            RootRefusal::Touch(diag) => Self::Touch(diag),
            RootRefusal::Unsettled(why) => Self::Unsettled(why),
            RootRefusal::Fault(fault) => Self::Contradiction {
                readings: fault.readings(),
            },
        }
    }
}

impl From<geom::PieceWalkRefusal> for MeetFault {
    fn from(why: geom::PieceWalkRefusal) -> Self {
        Self::Unwalked(why)
    }
}

impl From<Indeterminate> for MeetFault {
    fn from(diag: Indeterminate) -> Self {
        Self::Escalated(diag)
    }
}

/// [`meet`]'s refusal, with every meeting it had decided before it: a
/// consumer may read those, never the refusal, as an answer (D4).
#[derive(Clone, Debug)]
pub struct MeetRefusal<T: geom_core::Real> {
    /// Why.
    pub fault: MeetFault,
    /// Meetings decided before the refusal: a subset of the spans'
    /// meetings, never the complete set.
    pub found: Vec<Meeting<T>>,
    /// As [`Meetings::shared`]: `Some` where the pose read the carriers
    /// as one curve, so `found` holds the overlap's witnesses.
    pub shared: Option<Sense>,
}

impl<T: geom_core::Real> From<MeetFault> for MeetRefusal<T> {
    fn from(fault: MeetFault) -> Self {
        Self {
            fault,
            found: Vec::new(),
            shared: None,
        }
    }
}

/// **Whether, and where, `a` and `b` meet**: the pair's method from the
/// table, then, for a closed form, each span's ends read against the
/// other, the pose given the meetings so found, and the pose's
/// candidates read on both spans; for the walk, the walk.
///
/// # Errors
///
/// [`MeetRefusal`], as [`MeetFault`] says.
pub fn meet<T: Decide>(
    a: &Span<'_, T>,
    b: &Span<'_, T>,
    band: Band,
) -> Result<Meetings<T>, MeetRefusal<T>> {
    match method(a.carrier().kind(), b.carrier().kind()) {
        Method::Closed(arm) => closed(arm, a, b, band),
        Method::Walk => walk::meet(a, b, band),
    }
}

/// [`meet`] for a closed-form arm. The ends come first, so the pose
/// divides the roots they found out of its equation ([`pose::closed`]);
/// a pair read as one curve meets along its shared run, which takes the
/// ends' meetings with it.
fn closed<T: Decide>(
    arm: Arm,
    a: &Span<'_, T>,
    b: &Span<'_, T>,
    band: Band,
) -> Result<Meetings<T>, MeetRefusal<T>> {
    let mut found = Found::default();
    for c in pose::end_candidates(a, b) {
        found.read(a, b, c, band);
    }
    let known: Vec<_> = found.points.iter().map(|m| candidate(a, b, m)).collect();
    let pose = pose::closed(arm, a, b, &known, band).map_err(|fault| MeetRefusal {
        fault,
        found: found.points.clone(),
        shared: None,
    })?;
    match pose {
        Pose::Coincident { sense, witnesses } => {
            let mut run = Found::default();
            for c in witnesses {
                run.read(a, b, c, band);
            }
            run.finish(a, b, Some(sense), band)
        }
        Pose::Tangent(c) => {
            posed(&mut found, a, b, c, band);
            found.finish(a, b, None, band)
        }
        Pose::Generic(cs) => {
            for c in cs {
                posed(&mut found, a, b, c, band);
            }
            found.finish(a, b, None, band)
        }
        Pose::Concentric | Pose::Parallel | Pose::Apart => found.finish(a, b, None, band),
    }
}

/// **A posed candidate read**: the pose's closest pair, its window on the
/// spans, as that pair ([`Found::read_posed`]); a slanted pair in one
/// plane, whose stretch may reach past a span, as the walk's pair is
/// ([`Found::read_pair`], [`WindowSpans`]); any other at its
/// point ([`Found::read`]). A pose read with the spans swapped gives back
/// the other carrier's point, so the pair's point is the first carrier's
/// at its parameter.
fn posed<T: Decide>(
    found: &mut Found<T>,
    a: &Span<'_, T>,
    b: &Span<'_, T>,
    c: pose::Candidate<T>,
    band: Band,
) {
    let Some(bound) = c.bound else {
        found.read(a, b, c, band);
        return;
    };
    let c = pose::Candidate {
        point: a.carrier().eval(c.ta),
        ..c
    };
    match bound.spans {
        WindowSpans::Held => found.read_posed(a, b, c, band),
        WindowSpans::MayPass => {
            let places = (PlaceRead::of(a, c.ta, band), PlaceRead::of(b, c.tb, band));
            found.read_pair(a, b, c, places, band);
        }
    }
}

/// A meeting as a candidate: its point and its parameter on each carrier.
fn candidate<T: Decide>(a: &Span<'_, T>, b: &Span<'_, T>, m: &Meeting<T>) -> Candidate<T> {
    Candidate {
        point: m.point,
        ta: a.at(m.on_a),
        tb: b.at(m.on_b),
        bound: None,
    }
}

/// **Whether `point` lies nearer a meeting already found than the band's
/// escalation width** (`curve_pair_meet_same`): it cannot be decided
/// distinct from that meeting, so it is not read again (C5). Only a
/// distance decided past the band reads a new point.
fn ringed<T: Decide>(found: &[Meeting<T>], point: Point3<T>, band: Band) -> bool {
    ringed_by(found.iter().map(|m| m.point), point, band)
}

/// [`ringed`] by the meetings at `found`.
fn ringed_by<T: Decide>(
    found: impl IntoIterator<Item = Point3<T>>,
    point: Point3<T>,
    band: Band,
) -> bool {
    found.into_iter().any(|m| {
        geom_core::k_stats::decide(
            "curve_pair_meet_same",
            geom_core::Margin::norm3(m - point),
            band,
        ) != Ok(geom_core::Sign::Positive)
    })
}

/// A meeting's two sides: where its point sits on each span.
type Sides<T> = (Hit<T>, Hit<T>);

/// A point read on both spans: a meeting's two sides, `None` off either,
/// or the first refusal.
type Both<T> = Result<Option<Sides<T>>, geom::CarrierRefusal>;

/// Two point-on-span readings as one [`Both`].
fn both<T>(
    on_a: Result<Option<Hit<T>>, geom::CarrierRefusal>,
    on_b: Result<Option<Hit<T>>, geom::CarrierRefusal>,
) -> Both<T> {
    match (on_a, on_b) {
        (Ok(None), _) | (_, Ok(None)) => Ok(None),
        (Ok(Some(on_a)), Ok(Some(on_b))) => Ok(Some((on_a, on_b))),
        (Err(diag), _) | (_, Err(diag)) => Err(diag),
    }
}

/// **A parameter's place on its span, read once**, at the walk's stop
/// (`walk::at_end`) and at the pose's polished pair
/// ([`Found::read_posed`]): the place [`Span::place_param`] reads, and the
/// record of reading it, held apart until the pair read knows whether its
/// outcome rests on it ([`pair_both`]).
pub(super) struct PlaceRead {
    pub(super) place: Result<Place, Indeterminate>,
    pub(super) record: geom_core::k_stats::Detached,
}

impl PlaceRead {
    /// `u`'s place on `span`, read with its record held apart.
    pub(super) fn of<T: Decide>(span: &Span<'_, T>, u: T, band: Band) -> Self {
        let (place, record) = geom_core::k_stats::detached(|| span.place_param(u, band));
        Self { place, record }
    }
}

/// A decided gap at a polished pair, handed from the pair read to the
/// window it composes with ([`window_off`]), which reads no gap again.
type GapRead = Result<geom_core::Decided, Indeterminate>;

/// **How [`Found::read_with`] reads a polished pair on both spans**, with
/// each parameter's place read once.
enum CandidateRead {
    /// As the pair `(c.ta, c.tb)` ([`Found::read_pair`], [`pair_both`]),
    /// boxed: two place records are much larger than a point.
    Pair(Box<(PlaceRead, PlaceRead)>),
    /// As the pose's closest pair, its polish held to the spans
    /// ([`Found::read_posed`]): [`Self::Pair`], with a window that holds
    /// no pair past either span, so nothing is read past one.
    Posed(Box<(PlaceRead, PlaceRead)>),
}

impl CandidateRead {
    /// A pair's place records superseded, where the candidate reads
    /// nothing: their verdicts and samples stay in the log as superseded,
    /// no escalation of theirs left that no answer rests on.
    fn supersede(self) {
        let (Self::Pair(places) | Self::Posed(places)) = self;
        let (on_a, on_b) = *places;
        geom_core::k_stats::splice_superseded(on_a.record);
        geom_core::k_stats::splice_superseded(on_b.record);
    }
}

/// **A pair read on both spans**: the gap between the first carrier's
/// point at `c.ta`, `c.point` ([`Found::read_pair`]), and the second's at
/// `c.tb`, decided against the band
/// (`curve_pair_walk_pair_gap`), and each parameter's place on its span,
/// read once at the stop (`places`, [`PlaceRead`]). A gap in the band is
/// a meeting, its side on each span the parameter's place: inside, on the
/// branch about the span's middle with the gap's margin, or at an end; a
/// parameter past its span is off it. A gap decided positive is no meeting
/// at the pair. The places' records are spliced where the outcome rests on
/// them (a gap in the band) and superseded where it does not, so no
/// escalation of theirs is left that the answer does not rest on. No foot
/// is read: the parameters are the pair's own. The decided gap is
/// returned with the reading, for the window to compose with.
fn pair_both<T: Decide>(
    a: &Span<'_, T>,
    b: &Span<'_, T>,
    c: Candidate<T>,
    (on_a, on_b): (PlaceRead, PlaceRead),
    band: Band,
) -> (Both<T>, GapRead) {
    let gap = geom_core::k_stats::decide_reported(
        "curve_pair_walk_pair_gap",
        geom_core::Margin::norm3(b.carrier().eval(c.tb) - c.point),
        band,
    );
    let Ok(geom_core::Decided {
        sign: geom_core::Sign::Zero,
        margin,
    }) = gap
    else {
        geom_core::k_stats::splice_superseded(on_a.record);
        geom_core::k_stats::splice_superseded(on_b.record);
        let read = match gap {
            Ok(_) => Ok(None),
            Err(diag) => Err(diag.into()),
        };
        return (read, gap);
    };
    geom_core::k_stats::splice(on_a.record);
    geom_core::k_stats::splice(on_b.record);
    let side = |span: &Span<'_, T>, t: T, place: Result<Place, Indeterminate>| {
        Ok(match place? {
            Place::Inside => Some(Hit::On {
                t: span.branch(t),
                margin,
            }),
            Place::End(which) => Some(Hit::End(which)),
            Place::Past => None,
        })
    };
    (
        both(side(a, c.ta, on_a.place), side(b, c.tb, on_b.place)),
        gap,
    )
}

/// **The meetings a reader has found, and the refusals it holds**, read
/// candidate by candidate under C5's stretch rule.
struct Found<T: geom_core::Real> {
    points: Vec<Meeting<T>>,
    held: Vec<(Point3<T>, MeetFault)>,
}

impl<T: geom_core::Real> Default for Found<T> {
    fn default() -> Self {
        Self {
            points: Vec::new(),
            held: Vec::new(),
        }
    }
}

impl<T: Decide> Found<T> {
    /// **`c` read on both spans at its own point**, unless it is ringed:
    /// a candidate whose point speaks for itself (an end, a witness, an
    /// exact root), read by [`Span::contact`] on each span. A candidate
    /// with a window is a polished pair, read as that pair
    /// ([`Self::read_pair`], [`Self::read_posed`]); handed here it is a
    /// desync, held at its point.
    fn read(&mut self, a: &Span<'_, T>, b: &Span<'_, T>, c: Candidate<T>, band: Band) {
        if c.bound.is_some() {
            self.held.push((c.point, MeetFault::Desync));
            return;
        }
        self.offer(
            c.point,
            || {
                both(
                    a.contact(c.point, c.ta, band).map_err(|r| r.refused()),
                    b.contact(c.point, c.tb, band).map_err(|r| r.refused()),
                )
            },
            band,
        );
    }

    /// **A polished pair read as that pair** (the curve-pair rule in
    /// `crates/geom-brep/README.md`): `c` names a parameter on each span,
    /// `c.ta` on `a` with `c.point` its point and `c.tb` on `b`, and is
    /// read by [`pair_both`], with no foot, under its window: no meeting
    /// where its floor reads past the band (`curve_pair_pose_closest`);
    /// otherwise a gap in the band is a meeting, and an Off prunes only
    /// where the window's run lies past a span ([`Span::past_by`], by the
    /// reach and the run arm). Any other Off is a straddle of the floor
    /// and the gap at the pair (`curve_pair_meet_window`), and escalates.
    fn read_pair(
        &mut self,
        a: &Span<'_, T>,
        b: &Span<'_, T>,
        c: Candidate<T>,
        places: (PlaceRead, PlaceRead),
        band: Band,
    ) {
        self.read_with(a, b, c, CandidateRead::Pair(Box::new(places)), band);
    }

    /// **The pose's closest pair read as that pair** (`pose`'s module
    /// docs, "Held to the spans"): [`Self::read_pair`], each parameter's
    /// place read once here, where the polish held it to its span. Its
    /// window holds no pair past either span, so an Off is not read past
    /// one ([`Span::past_by`]): it is no meeting where the floor clears,
    /// and otherwise escalates.
    ///
    /// **No end snap**, unlike the walk's stop (`walk::at_end`). A pair
    /// the span stops is clamped exactly to the end, so the hold, the
    /// one-sided window and the place all read that end. A pair within
    /// rounding of an end but not clamped to it reads two-sided, a window
    /// bounding the larger set, and the side it is reported at is its
    /// place's, whichever end that names. The walk snaps because its
    /// pieces' polish lands within rounding of a joint it must read as
    /// held.
    fn read_posed(&mut self, a: &Span<'_, T>, b: &Span<'_, T>, c: Candidate<T>, band: Band) {
        let places = (PlaceRead::of(a, c.ta, band), PlaceRead::of(b, c.tb, band));
        self.read_with(a, b, c, CandidateRead::Posed(Box::new(places)), band);
    }

    /// [`Self::read_pair`] and [`Self::read_posed`]'s one composition: the
    /// pair read on both spans ([`pair_both`]), which also hands the gap
    /// it decided to the window, which composes with it and does not read
    /// it again.
    /// A ringed candidate, and one whose floor reads past the band, reads
    /// nothing; a pair's place records are then superseded, since no
    /// answer rests on them (a ring and a floor read `c.point`, which the
    /// snap they steered placed).
    fn read_with(
        &mut self,
        a: &Span<'_, T>,
        b: &Span<'_, T>,
        c: Candidate<T>,
        how: CandidateRead,
        band: Band,
    ) {
        let posed = matches!(how, CandidateRead::Posed(_));
        let on_both = |how: CandidateRead| -> (Both<T>, GapRead) {
            let (CandidateRead::Pair(places) | CandidateRead::Posed(places)) = how;
            pair_both(a, b, c, *places, band)
        };
        let Some(bound) = c.bound else {
            self.offer(c.point, || on_both(how).0, band);
            return;
        };
        if ringed(&self.points, c.point, band) {
            how.supersede();
            return;
        }
        let floor = geom_core::k_stats::decide_reported(
            "curve_pair_pose_closest",
            geom_core::Margin::of(bound.floor),
            band,
        );
        if matches!(
            floor,
            Ok(geom_core::Decided {
                sign: geom_core::Sign::Positive,
                ..
            })
        ) {
            how.supersede();
            return;
        }
        self.offer(
            c.point,
            || match on_both(how) {
                (Ok(None), gap) => window_off(a, b, c, bound, (floor, gap), posed, band),
                (read, _) => read.map_err(MeetFault::from),
            },
            band,
        );
    }

    /// `point` read on both spans by `read`, unless it is ringed.
    fn offer<E: Into<MeetFault>>(
        &mut self,
        point: Point3<T>,
        read: impl FnOnce() -> Result<Option<Sides<T>>, E>,
        band: Band,
    ) {
        if ringed(&self.points, point, band) {
            return;
        }
        match read() {
            Ok(None) => {}
            Ok(Some((on_a, on_b))) => self.points.push(Meeting { point, on_a, on_b }),
            Err(why) => self.held.push((point, why.into())),
        }
    }

    /// **The answer**: a refusal held at a point no meeting rings
    /// refuses, carrying the meetings; and, but on a shared run, two
    /// meetings in one stretch escalate (`stretches`).
    fn finish(
        self,
        a: &Span<'_, T>,
        b: &Span<'_, T>,
        shared: Option<Sense>,
        band: Band,
    ) -> Result<Meetings<T>, MeetRefusal<T>> {
        let held = self
            .held
            .iter()
            .find(|(q, _)| !ringed(&self.points, *q, band))
            .map(|&(_, fault)| fault);
        let fault = match (held, shared) {
            (Some(fault), _) => Some(fault),
            (None, Some(_)) => None,
            (None, None) => stretches(a, b, &self.points, band)
                .and_then(|()| {
                    let swapped: Vec<_> = self
                        .points
                        .iter()
                        .map(|m| Meeting {
                            point: m.point,
                            on_a: m.on_b,
                            on_b: m.on_a,
                        })
                        .collect();
                    stretches(b, a, &swapped, band)
                })
                .err(),
        };
        match fault {
            None => Ok(Meetings {
                shared,
                points: self.points,
            }),
            Some(fault) => Err(MeetRefusal {
                fault,
                found: self.points,
                shared,
            }),
        }
    }
}

/// **A polished pair read off a span** ([`Found::read_pair`],
/// [`Found::read_posed`]): no meeting where the window's run lies past
/// either span ([`Span::past_by`], by the reach and the run arm; the
/// walk's pair alone, `posed` false); otherwise the escalation, the
/// straddle of the floor against the gap the pair read decided where both
/// decide, and else whichever reading did not. Where every reading
/// decided and none cleared, the gap at the pair reads zero while a
/// parameter's place read it past its span and no end read near: the two
/// readings disagree at rounding, and that refuses typed,
/// `Meter { row: "curve_pair_meet_window_contact" }`.
fn window_off<T: Decide>(
    a: &Span<'_, T>,
    b: &Span<'_, T>,
    c: Candidate<T>,
    bound: pose::WindowBound<T>,
    (floor, gap): (Result<geom_core::Decided, Indeterminate>, GapRead),
    posed: bool,
    band: Band,
) -> Result<Option<Sides<T>>, MeetFault> {
    let q = b.carrier().eval(c.tb);
    // The pose's pair is held to the spans, and so is its window: no
    // pair of it lies past either span, and nothing is read past one.
    let sides = if posed {
        [PastBy::Not, PastBy::Not]
    } else {
        // The run arm's premise (`walk`'s module docs, "The reach and the
        // run, past a span"): the polished pair is itself a pair of the
        // run, its gap in the zero band, read on the gap the pair read
        // decided.
        let in_run = gap.is_ok_and(|g| g.sign == geom_core::Sign::Zero);
        let (run_a, run_b) = (
            in_run.then(|| (b.carrier(), bound.bend)),
            in_run.then(|| (a.carrier(), bound.bend)),
        );
        [
            a.past_by(c.point, c.ta, (bound.reach, run_a), band),
            b.past_by(q, c.tb, (bound.reach, run_b), band),
        ]
    };
    if sides.contains(&PastBy::Clear) {
        return Ok(None);
    }
    let near = sides.iter().find_map(|side| match side {
        PastBy::Near(diag) => Some(*diag),
        PastBy::Not | PastBy::Clear => None,
    });
    Err(MeetFault::Escalated(match (floor, gap, near) {
        (Ok(lower), Ok(upper), _) if upper.sign == geom_core::Sign::Positive => {
            geom_core::k_stats::straddle("curve_pair_meet_window", lower, upper, band)
        }
        (_, _, Some(diag)) | (Err(diag), _, None) | (Ok(_), Err(diag), None) => diag,
        (Ok(_), Ok(_), None) => {
            return Err(MeetFault::Unsettled(Unsettled::Meter {
                row: "curve_pair_meet_window_contact",
            }));
        }
    }))
}

/// **Whether each meeting is the one meeting of its stretch** (C5): two
/// meetings next along `a` lie in two stretches where the point of `a`
/// halfway between them, read on `b`, is definitely off it. A point
/// there on `b` too, or one the band cannot read, leaves the spans
/// nowhere definitely apart between the two as far as the reading can
/// say, so whether they are two crossings or one run is undecided: it
/// escalates (`curve_pair_meet_stretch`). On a span with no ends the last
/// meeting is next to the first, across the carrier's seam. A stretch is
/// one span's, so the answer reads them along each span in turn.
fn stretches<T: Decide>(
    a: &Span<'_, T>,
    b: &Span<'_, T>,
    points: &[Meeting<T>],
    band: Band,
) -> Result<(), MeetFault> {
    let mut order: Vec<&Meeting<T>> = Vec::with_capacity(points.len());
    for m in points {
        let t = a.at(m.on_a);
        let at = order
            .iter()
            .position(|o| before(t, a.at(o.on_a)))
            .unwrap_or(order.len());
        order.insert(at, m);
    }
    let mut between: Vec<_> = order
        .windows(2)
        .map(|pair| {
            let [m, n] = [pair[0], pair[1]];
            let ta = geom::mid_param(a.at(m.on_a), a.at(n.on_a));
            (m, n, ta, geom::mid_param(b.at(m.on_b), b.at(n.on_b)))
        })
        .collect();
    if let (false, [first, .., last]) = (a.ended(), &order[..]) {
        // Halfway along the arc from the last over the seam to the first.
        let (t0, t1) = a.params();
        let half = ((t1 - a.at(last.on_a)) + (a.at(first.on_a) - t0)) * T::from_f64(0.5);
        let past = a.at(last.on_a) + half - t1;
        let ta = past.select_le_zero(a.at(last.on_a) + half, t0 + past);
        between.push((*last, *first, ta, b.at(first.on_b)));
    }
    for (m, n, ta, tb) in between {
        let q = a.carrier().eval(ta);
        let on = match b.contact(q, tb, band) {
            Ok(None) => continue,
            Ok(Some(Hit::On { margin, .. })) => geom_core::Decided {
                sign: geom_core::Sign::Zero,
                margin,
            },
            Ok(Some(Hit::End(which))) => {
                // Only an ended span reads a point at an end, so the end
                // is one of its two.
                let nth = usize::from(which == SpanEnd::Stop);
                let Some((_, end)) = b.ends().nth(nth) else {
                    return Err(MeetFault::Contradiction {
                        readings: "a point read at an end of a span with no ends",
                    });
                };
                match geom_core::k_stats::decide_reported(
                    "curve_pair_span_end_chord",
                    geom_core::Margin::norm3(q - end),
                    band,
                ) {
                    Ok(chord) => chord,
                    Err(diag) => return Err(MeetFault::Escalated(diag)),
                }
            }
            Err(refusal) => return Err(refusal.into()),
        };
        let apart = geom_core::k_stats::decide_reported(
            "curve_pair_meet_same",
            geom_core::Margin::norm3(m.point - n.point),
            band,
        )?;
        // Two meetings both stand only apart past the band, and a point
        // read on a span is read in its zero band.
        if on.sign != geom_core::Sign::Zero || apart.sign != geom_core::Sign::Positive {
            return Err(MeetFault::Contradiction {
                readings: "two meetings of one stretch read apart and a point between them read on",
            });
        }
        return Err(MeetFault::Escalated(geom_core::k_stats::straddle(
            "curve_pair_meet_stretch",
            on,
            apart,
            band,
        )));
    }
    Ok(())
}

/// Whether the parameter `s` definitely precedes `t`
/// ([`compare`]).
fn before<T: Decide>(s: T, t: T) -> bool {
    compare("curve_pair_meet_order", s, t) == Some(geom_core::Sign::Negative)
}

/// [`geom_core::k_stats::param_order`], `None` where an enclosure
/// straddles.
pub(super) fn compare<T: Decide>(name: &'static str, s: T, t: T) -> Option<geom_core::Sign> {
    geom_core::k_stats::param_order(name, s, t).ok()
}

impl From<geom::CarrierRefusal> for MeetFault {
    /// A point-on-span refusal as the meeting's: an escalation as itself,
    /// a walk that could not run as its own [`From`].
    fn from(refusal: geom::CarrierRefusal) -> Self {
        refusal
            .escalation()
            .map_or_else(Self::from, Self::Escalated)
    }
}

impl From<SpanRefusal> for MeetFault {
    /// A point-on-span refusal as the meeting's: the refusal it is
    /// ranked by ([`SpanRefusal::refused`]) as its own
    /// ([`From<geom::CarrierRefusal>`]).
    fn from(refusal: SpanRefusal) -> Self {
        refusal.refused().into()
    }
}

#[cfg(test)]
mod lever_rows;
#[cfg(test)]
mod meet_oracle;
#[cfg(test)]
mod meet_rows;
#[cfg(test)]
mod one_curve_rows;
#[cfg(test)]
mod order_rows;
#[cfg(test)]
mod plane_rows;
#[cfg(test)]
mod span_rows;
#[cfg(test)]
mod touch_spread_rows;
