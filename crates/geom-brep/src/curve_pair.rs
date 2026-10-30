//! **The curve-pair meeting reader**: whether, and where, two carrier
//! spans meet (C5 in `crates/geom-brep/README.md`, D3's "One reading per
//! question").
//!
//! - **One point-on-span reading** ([`span`]): every candidate point any
//!   method proposes is settled by [`Span::contact`] on both spans, at its
//!   foot on each ([`geom::Curve3::foot`], seeded by the candidate's own
//!   parameter). It composes `geom`'s one-carrier reading with the
//!   span's reading of its ends ([`Span::read`]), and decides nothing of
//!   its own.
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

mod polish;
pub mod pose;
pub mod span;
mod walk;

pub use pose::{Candidate, Pose, PoseRefusal, Sense, pose};
pub use span::{Hit, Place, Reading, Span, SpanEnd};

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
    use CurveKind::{Circle, Ellipse, Line, Nurbs, Spiric};
    match (a, b) {
        (Line, Line) => Method::Closed(Arm::LineLine),
        (Line, Circle | Ellipse) => Method::Closed(Arm::LineConic),
        (Circle | Ellipse, Line) => Method::Closed(Arm::ConicLine),
        (Circle, Circle) => Method::Closed(Arm::CircleCircle),
        (Ellipse, Circle) => Method::Closed(Arm::EllipseCircle),
        (Circle, Ellipse) => Method::Closed(Arm::CircleEllipse),
        (Ellipse, Ellipse) => Method::Closed(Arm::EllipseEllipse),
        (Line | Circle | Ellipse | Spiric | Nurbs, Spiric | Nurbs)
        | (Spiric | Nurbs, Line | Circle | Ellipse) => Method::Walk,
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
#[derive(Clone, Copy, Debug)]
pub enum MeetFault {
    /// A decision fell in the band's gap.
    Escalated(Indeterminate),
    /// A root finder read the pair in the band's gap where they can meet
    /// ([`RootRefusal::Touch`]): a meeting in band, which escalates where
    /// the spans are the model's and is the probe's where the caller
    /// chose one.
    Touch(Indeterminate),
    /// A root finder, a meter or the walk's budget reached its limit
    /// before it could certify the candidates.
    Unsettled(Unsettled),
    /// A pose the pair's arm has no closed form for.
    Unsupported {
        /// The pose.
        pose: &'static str,
    },
    /// A carrier is not of the kind its arm reads.
    Desync,
    /// Two readings of the pose contradict each other, which their
    /// definitions rule out but for rounding: a kernel fault, never a
    /// pose.
    Contradiction {
        /// The readings.
        readings: &'static str,
    },
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
            found.read(a, b, c, band);
            found.finish(a, b, None, band)
        }
        Pose::Generic(cs) => {
            for c in cs {
                found.read(a, b, c, band);
            }
            found.finish(a, b, None, band)
        }
        Pose::Concentric | Pose::Parallel | Pose::Apart => found.finish(a, b, None, band),
    }
}

/// A meeting as a candidate: its point and its parameter on each carrier.
fn candidate<T: Decide>(a: &Span<'_, T>, b: &Span<'_, T>, m: &Meeting<T>) -> Candidate<T> {
    Candidate {
        point: m.point,
        ta: a.at(m.on_a),
        tb: b.at(m.on_b),
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

/// A point read on both spans: a meeting's two sides, `None` off either,
/// or the first refusal.
type Both<T> = Result<Option<(Hit<T>, Hit<T>)>, geom::CarrierRefusal>;

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

/// **The meetings a reader has found, and the refusals it holds**, read
/// candidate by candidate under C5's stretch rule.
struct Found<T: geom_core::Real> {
    points: Vec<Meeting<T>>,
    held: Vec<(Point3<T>, geom::CarrierRefusal)>,
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
    /// `c` read on both spans at its own parameters, unless it is ringed.
    fn read(&mut self, a: &Span<'_, T>, b: &Span<'_, T>, c: Candidate<T>, band: Band) {
        self.offer(
            c.point,
            || {
                both(
                    a.contact(c.point, c.ta, band),
                    b.contact(c.point, c.tb, band),
                )
            },
            band,
        );
    }

    /// `point` read on both spans by `read`, unless it is ringed.
    fn offer(&mut self, point: Point3<T>, read: impl FnOnce() -> Both<T>, band: Band) {
        if ringed(&self.points, point, band) {
            return;
        }
        match read() {
            Ok(None) => {}
            Ok(Some((on_a, on_b))) => self.points.push(Meeting { point, on_a, on_b }),
            Err(diag) => self.held.push((point, diag)),
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
            .map(|&(_, diag)| carrier_fault(diag));
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
                let end = b.carrier().eval(b.end(which));
                match geom_core::k_stats::decide_reported(
                    "curve_pair_span_end_chord",
                    geom_core::Margin::norm3(q - end),
                    band,
                ) {
                    Ok(chord) => chord,
                    Err(diag) => return Err(MeetFault::Escalated(diag)),
                }
            }
            Err(refusal) => return Err(carrier_fault(refusal)),
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

/// **Two parameters compared, read exactly** (the least pair of positive
/// `f64`s) under `row`: the sign of `s − t`, `None` where an enclosure
/// straddles. A parameter is no distance the band resolves, so the band
/// plays no part.
pub(super) fn compare<T: Decide>(row: &'static str, s: T, t: T) -> Option<geom_core::Sign> {
    geom_core::k_stats::decide(row, geom_core::Margin::of(s - t), Band::least()).ok()
}

/// A point-on-span refusal as the meeting's: an escalation as itself, a
/// spline's walk past its budget as unsettled, and a spline with no
/// speed bound as a pose the reader does not serve.
pub(crate) fn carrier_fault(refusal: geom::CarrierRefusal) -> MeetFault {
    match refusal {
        geom::CarrierRefusal::Escalated(diag) | geom::CarrierRefusal::Straddle(diag) => {
            MeetFault::Escalated(diag)
        }
        geom::CarrierRefusal::Exhausted => MeetFault::Unsettled(Unsettled::Exhausted),
        geom::CarrierRefusal::Unbounded => MeetFault::Unsupported {
            pose: "a spline whose speed bound is zero or poison",
        },
    }
}

#[cfg(test)]
mod meet_oracle;
#[cfg(test)]
mod meet_rows;
#[cfg(test)]
mod one_curve_rows;
#[cfg(test)]
mod plane_rows;
#[cfg(test)]
mod span_rows;
