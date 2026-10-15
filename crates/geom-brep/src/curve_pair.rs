//! **The curve-pair meeting reader**: whether, and where, two carrier
//! spans meet (C5 in `crates/geom-brep/README.md`, D3's "One reading per
//! question").
//!
//! - **One point-on-span reading** ([`span`]): every candidate point any
//!   method proposes is settled by [`Span::contact`] on both spans.
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
//! **The walk is the closed forms' differential test oracle** (the rows),
//! never a runtime fallback.

mod polish;
pub mod pose;
pub mod span;
mod walk;

pub use pose::{Candidate, Pose, PoseRefusal, Sense, foot, pose};
pub use span::{Contact, Hit, Span};

use geom::CurveKind;
use geom_core::{Band, Decide, Indeterminate, Point3};

use crate::roots::Unsettled;

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

/// A point on both spans.
#[derive(Clone, Copy, Debug)]
pub struct Meeting<T: geom_core::Real> {
    /// The point.
    pub point: Point3<T>,
    /// Where it sits on the first span.
    pub on_a: Hit<T>,
    /// Where it sits on the second span.
    pub on_b: Hit<T>,
}

/// What [`meet`] found.
#[derive(Clone, Debug)]
pub struct Meetings<T: geom_core::Real> {
    /// `Some` where the pose read the carriers as one curve. `points` are
    /// then the overlap's witnesses that read on both spans, and every
    /// point of the overlap meets; none read on both where the spans
    /// share no run of the curve, which is `Some` with no points.
    pub shared: Option<Sense>,
    /// The meetings, each read on both spans. Two candidates polished onto
    /// one crossing, by the walk or across planes, may repeat it.
    pub points: Vec<Meeting<T>>,
}

/// Why [`meet`] gave no answer.
#[derive(Clone, Copy, Debug)]
pub enum MeetFault {
    /// A decision fell in the band's gap.
    Escalated(Indeterminate),
    /// A root finder, a meter or the walk's budget could not certify the
    /// candidates.
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
}

impl<T: geom_core::Real> From<MeetFault> for MeetRefusal<T> {
    fn from(fault: MeetFault) -> Self {
        Self {
            fault,
            found: Vec::new(),
        }
    }
}

/// **Whether, and where, `a` and `b` meet**: the pair's method from the
/// table, then, for a closed form, its pose and the pose's candidates
/// read on both spans; for the walk, the walk.
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
        Method::Closed(arm) => outcome(&pose::closed(arm, a, b, band)?, a, b, band),
        Method::Walk => walk::meet(a, b, band),
    }
}

/// The meetings a pose's candidates are: each read on both spans.
fn outcome<T: Decide>(
    pose: &Pose<T>,
    a: &Span<'_, T>,
    b: &Span<'_, T>,
    band: Band,
) -> Result<Meetings<T>, MeetRefusal<T>> {
    let (shared, candidates): (Option<Sense>, Vec<Candidate<T>>) = match pose {
        Pose::Coincident { sense, witnesses } => (Some(*sense), witnesses.clone()),
        Pose::Tangent(c) => (None, [vec![*c], pose::end_candidates(a, b)].concat()),
        Pose::Generic(cs) => (None, [cs.clone(), pose::end_candidates(a, b)].concat()),
        Pose::Concentric | Pose::Parallel | Pose::Apart => (None, Vec::new()),
    };
    let mut points = Vec::new();
    let mut held = None;
    for c in &candidates {
        let on_a = a.contact(c.point, c.ta, band).map(Contact::hit);
        let on_b = b.contact(c.point, c.tb, band).map(Contact::hit);
        match (on_a, on_b) {
            (Ok(None), _) | (_, Ok(None)) => {}
            (Ok(Some(on_a)), Ok(Some(on_b))) => points.push(Meeting {
                point: c.point,
                on_a,
                on_b,
            }),
            (Err(diag), _) | (_, Err(diag)) => held = held.or(Some(diag)),
        }
    }
    match held {
        None => Ok(Meetings { shared, points }),
        Some(diag) => Err(MeetRefusal {
            fault: MeetFault::Escalated(diag),
            found: points,
        }),
    }
}

#[cfg(test)]
mod meet_oracle;
#[cfg(test)]
mod meet_rows;
#[cfg(test)]
mod one_curve_rows;
#[cfg(test)]
mod span_rows;
