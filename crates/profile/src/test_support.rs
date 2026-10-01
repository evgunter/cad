//! Fixture helpers, compiled under `test` / `test-support` only.

use geom_core::{Point2, Real};

use crate::{ArcData, ArcSide, ProfileLoop, Step, Target, TipState, Verb};

/// The loop a chain of (position, bulge) pairs lowers to — each vertex
/// with the bulge of the segment leaving it, the last one's closing
/// back to the first — with no declared-tangent joints.
///
/// It forwards to the lowering the lattice's emission layer uses for
/// `arc_to(Bulge)` and computes nothing of its own, so a loop written
/// here stores the same vertices, segments and bulges, bit for bit, as
/// the same chain emitted by the lattice. A bulge of exactly zero
/// (either sign) is a line; any other bulge is an arc, finite or not,
/// and [`crate::Profile::validate`] decides what the table is.
pub fn bulge_loop<T: Real>(chain: Vec<(Point2<T>, T)>) -> ProfileLoop<T> {
    ProfileLoop::lower(&chain, Vec::new())
}

/// **Every tip state some verb has a row at, plus `Closed`** (which
/// none has): read off the transition table rather than listed, so a
/// state the table gains is walked without an edit.
#[must_use]
pub fn every_state() -> Vec<TipState> {
    let mut states = vec![TipState::Closed];
    for &verb in Verb::ALL {
        for &state in verb.states() {
            if !states.contains(&state) {
                states.push(state);
            }
        }
    }
    states
}

/// **The steps that take a leg end into `state`** — exhaustive, so a
/// state the lattice gains has to be given a way in before this
/// compiles. `None` for the entry, which no step reaches.
///
/// Every step leaves its geometry pending or is a line, but for
/// `DirectedPlain`'s last, which closes a fillet onto the line
/// `x + y = 20` heading `(-1, 1)`: a leg end near the origin, heading
/// `+x` or `+y`, meets it far enough ahead for the fillet to fit, so
/// the way in holds at [`prefix`]'s scale and at a centimetre's.
#[must_use]
pub fn way_in(state: TipState) -> Option<Vec<Step<f64>>> {
    let fillet = Step::Fillet { radius: 1.0 };
    let arrival = |spec| Step::FilletArc { radius: 1.0, spec };
    let radius = ArcData::Radius {
        r: 3.0,
        side: ArcSide::Left,
    };
    let via = |target| ArcData::Via {
        q: Point2::new(8.0, 6.0),
        target,
    };
    let beyond = Step::At(Point2::new(-5.0, 25.0));
    Some(match state {
        TipState::Entry => return None,
        TipState::Open => vec![fillet],
        TipState::Angle => vec![fillet, Step::Angle(1.5)],
        TipState::PlainPoint => vec![fillet, beyond],
        TipState::DirectedPoint => vec![],
        TipState::DirectedPlain => vec![fillet, beyond, Step::Angle(0.75 * core::f64::consts::PI)],
        TipState::DirectedIncoming => vec![Step::Turn(0.5)],
        TipState::RadiusArrival => vec![arrival(radius)],
        TipState::RadiusArrivalAt => vec![arrival(radius), Step::At(Point2::new(10.0, 10.0))],
        TipState::RadiusArrivalDir => vec![arrival(radius), Step::Angle(1.5)],
        TipState::ViaArrival => vec![arrival(via(Target::Point(Point2::new(10.0, 10.0))))],
        TipState::ViaArrivalStart => vec![arrival(via(Target::Start))],
        TipState::Closed => vec![
            Step::LineTo(Target::Point(Point2::new(0.0, 10.0))),
            Step::LineTo(Target::Start),
        ],
    })
}

/// **A program that leaves the tip in `state`**: the shortest lead
/// that takes each state, then [`way_in`]. The fused and fillet ways
/// in follow `at, angle 0`, which puts the tip at the origin heading
/// `+x`; `turn` and the closing legs need a leg end, `at` then a leg to
/// `(10, 0)`.
#[must_use]
pub fn prefix(state: TipState) -> Vec<Step<f64>> {
    let at = Step::At(Point2::new(0.0, 0.0));
    let lead = match state {
        TipState::Entry => return Vec::new(),
        TipState::PlainPoint => return vec![at],
        TipState::DirectedPlain => return vec![at, Step::Angle(0.0)],
        TipState::DirectedPoint | TipState::DirectedIncoming | TipState::Closed => {
            vec![at, Step::LineTo(Target::Point(Point2::new(10.0, 0.0)))]
        }
        _ => prefix(TipState::DirectedPlain),
    };
    [lead, way_in(state).unwrap_or_default()].concat()
}
