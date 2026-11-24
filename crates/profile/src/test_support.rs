//! Fixture helpers, compiled under `test` / `test-support` only.

use geom_core::{Point2, Real};

use crate::{ArcData, ArcSide, ProfileLoop, Step, Target, TipState, Verb};

/// The loop a chain of (position, bulge) pairs lowers to — each vertex
/// with the bulge of the segment leaving it, the last one's closing
/// back to the first. A [`ProfileLoop`] stores no joints: validation
/// decides every tangent junction on it and records each one.
///
/// It forwards to the lowering the lattice's emission layer uses for
/// `arc_to(Bulge)` and computes nothing of its own, so a loop written
/// here stores the same vertices and segments, bit for bit, as the same
/// chain of `arc_to(Bulge)` legs emitted by the lattice. It holds no
/// run tolerance, so unlike the lattice it registers none of the
/// lowered arcs' endpoint facts (`crate::lower_arc`). A bulge of exactly zero
/// (either sign) is a line; any other bulge is an arc, finite or not,
/// and [`crate::Profile::validate`] decides what the table is. It records
/// no direction: the table holds its points and bulges only, and
/// validation decides each segment's directions from them.
pub fn bulge_loop<T: geom_core::Decide>(chain: Vec<(Point2<T>, T)>) -> ProfileLoop<T> {
    ProfileLoop::from_chain(crate::lower_chain(&chain))
}

/// How far past its boundary a fillet lever's value still fits exactly
/// at `band`: the reach the fillet doors offer their levers within.
#[must_use]
pub fn exact_fit_reach(band: geom_core::Band) -> f64 {
    crate::sugar::exact_fit_reach(band)
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
        TipState::ArcOnto => vec![Step::ArcOnto(Target::Point(Point2::new(20.0, 10.0)))],
        TipState::ArcOntoStart => vec![Step::ArcOnto(Target::Start)],
        TipState::Closed => vec![
            Step::LineTo(Target::Point(Point2::new(0.0, 10.0))),
            Step::LineTo(Target::Start),
        ],
    })
}

/// **A program that leaves the tip in `state`**: the shortest lead
/// that takes each state, then [`way_in`]. The fused and fillet ways
/// in follow `at, angle 0`, which puts the tip at the origin heading
/// `+x`; `turn`, `arc_onto` and the closing legs need a leg end, `at`
/// then a leg to `(10, 0)`.
#[must_use]
pub fn prefix(state: TipState) -> Vec<Step<f64>> {
    let at = Step::At(Point2::new(0.0, 0.0));
    let lead = match state {
        TipState::Entry => return Vec::new(),
        TipState::PlainPoint => return vec![at],
        TipState::DirectedPlain => return vec![at, Step::Angle(0.0)],
        TipState::DirectedPoint
        | TipState::DirectedIncoming
        | TipState::ArcOnto
        | TipState::ArcOntoStart
        | TipState::Closed => {
            vec![at, Step::LineTo(Target::Point(Point2::new(10.0, 0.0)))]
        }
        _ => prefix(TipState::DirectedPlain),
    };
    [lead, way_in(state).unwrap_or_default()].concat()
}

/// **Every scalar a validated profile stores but its directions**, in one
/// fixed order: the plane's twelve placement components, then per loop
/// each vertex, each segment's ends, and an arc's centre, radius and
/// sweep. The walk the rows that hold one canonical form against another
/// read bits off. The directions a kind holds are left out: a lift
/// carries the `f64` mint's directions as points, where validating at an
/// interval scalar mints them with the scalar's own width, so the two
/// forms differ there by construction (`validated_map`'s lift row).
/// Their own rows hold them (`recorded_directions`).
#[must_use]
pub fn stored_scalars<T: Real>(vp: &crate::ValidatedProfile<T>) -> Vec<T> {
    let mut out = vp.plane().placement().components().to_vec();
    for lp in vp.loops() {
        for v in lp.vertices() {
            out.extend(v.to_array());
        }
        for s in lp.segments() {
            out.extend([s.start.x, s.start.y, s.end.x, s.end.y]);
            if let crate::SegmentKind::Arc { arc, .. } = s.kind {
                out.extend([arc.centre.x, arc.centre.y, arc.radius, arc.sweep]);
            }
        }
    }
    out
}

/// **Every fact a validated profile decided**, rendered per loop: its
/// role, its tangent, cusp and decided joints, and each segment's kind
/// and turn.
#[must_use]
pub fn decided_facts<T: Real>(vp: &crate::ValidatedProfile<T>) -> String {
    use core::fmt::Write as _;
    let mut out = String::new();
    for lp in vp.loops() {
        let _ = write!(
            out,
            "{:?} {:?} {:?} {:?}:",
            lp.role(),
            lp.tangent_joints(),
            lp.cusp_joints(),
            lp.decided_joints()
        );
        for s in lp.segments() {
            match s.kind {
                crate::SegmentKind::Line { .. } => out.push_str(" L"),
                crate::SegmentKind::Arc { turn, .. } => {
                    let _ = write!(out, " {turn:?}");
                }
            }
        }
        out.push('\n');
    }
    out
}

/// One arc verb's loop as its door stores it, `q` the quantity that
/// fixes the arc's radius: a radius, a bulge, a through-point's or a
/// target's coordinate, a leg's length.
///
/// # Panics
///
/// On a verb [`ARC_VERBS`] does not name, and where a door refuses its
/// fixture.
#[must_use]
#[allow(clippy::too_many_lines, clippy::unwrap_used, clippy::panic)]
pub fn arc_verb_loop<T: crate::ArcCarrierScalar>(
    verb: &str,
    q: T,
    t: geom_core::Tol,
) -> ProfileLoop<T> {
    let l = T::from_f64;
    let p = |x: f64, y: f64| Point2::new(l(x), l(y));
    match verb {
        "circle" => crate::circle(p(0.3, 0.2), q, t).unwrap().loop_.into_loop(),
        "split circle" => crate::circle_split(p(1.0, 0.5), q, 3, l(0.25), t)
            .unwrap()
            .loop_
            .into_loop(),
        "bulge" | "bulge cw" => crate::Open
            .at(p(0.0, 0.0))
            .line_to(p(4.0, 0.0), t)
            .unwrap()
            .arc_to(
                crate::Bulge {
                    p: p(4.0, 4.0),
                    b: q,
                },
                t,
            )
            .unwrap()
            .line_to(p(0.0, 4.0), t)
            .unwrap()
            .line_to(crate::Start, t)
            .unwrap()
            .loop_
            .into_loop(),
        "via" | "via cw" => crate::Open
            .at(p(0.0, 0.0))
            .line_to(p(4.0, 0.0), t)
            .unwrap()
            .arc_to(
                crate::Via {
                    q: Point2::new(q, l(2.0)),
                    p: p(4.0, 4.0),
                },
                t,
            )
            .unwrap()
            .line_to(p(0.0, 4.0), t)
            .unwrap()
            .line_to(crate::Start, t)
            .unwrap()
            .loop_
            .into_loop(),
        "tangent" | "tangent cw" => {
            let far = if verb == "tangent" { 2.0 } else { -2.0 };
            crate::Open
                .at(p(0.0, 0.0))
                .line_to(p(2.0, 0.0), t)
                .unwrap()
                .tangent()
                .tangent_arc_to(Point2::new(l(3.0), q), t)
                .unwrap()
                .line_to(p(0.0, far), t)
                .unwrap()
                .line_to(crate::Start, t)
                .unwrap()
                .loop_
                .into_loop()
        }
        "onto" => crate::Open
            .at(p(0.0, 0.0))
            .toward(l(0.0), l(1.0), t)
            .unwrap()
            .line(q, t)
            .unwrap()
            .toward(l(1.0), l(0.0), t)
            .unwrap()
            .tangent_arc_to(p(0.25, 1.0), t)
            .unwrap()
            .arc_onto(crate::Start, t)
            .unwrap()
            .toward(l(-1.0), l(0.0), t)
            .unwrap()
            .loop_
            .into_loop(),
        "sweep" | "sweep cw" => {
            let side = if verb == "sweep" {
                crate::ArcSide::Left
            } else {
                crate::ArcSide::Right
            };
            crate::Open
                .at(p(0.0, 0.0))
                .angle(l(0.0), t)
                .unwrap()
                .arc_to(
                    crate::Sweep {
                        r: q,
                        side,
                        angle: l(core::f64::consts::FRAC_PI_2),
                    },
                    t,
                )
                .unwrap()
                .line_to(p(-1.0, if verb == "sweep" { 3.0 } else { -3.0 }), t)
                .unwrap()
                .line_to(crate::Start, t)
                .unwrap()
                .loop_
                .into_loop()
        }
        "fillet" => {
            let (east, north) = (0.0_f64, core::f64::consts::FRAC_PI_2);
            let (west, south) = (core::f64::consts::PI, -north);
            crate::Open
                .at(p(0.0, -1.0))
                .angle(l(east), t)
                .unwrap()
                .fillet(q, t)
                .unwrap()
                .at(p(1.0, 0.0), t)
                .unwrap()
                .angle(l(north), t)
                .unwrap()
                .fillet(q, t)
                .unwrap()
                .at(p(0.0, 1.0), t)
                .unwrap()
                .angle(l(west), t)
                .unwrap()
                .fillet(q, t)
                .unwrap()
                .at(p(-1.0, 0.0), t)
                .unwrap()
                .angle(l(south), t)
                .unwrap()
                .fillet(q, t)
                .unwrap()
                .to(crate::Start, t)
                .unwrap()
                .loop_
                .into_loop()
        }
        "arc len" | "arc len cw" => {
            let side = if verb == "arc len" {
                crate::ArcSide::Left
            } else {
                crate::ArcSide::Right
            };
            crate::Open
                .at(p(0.0, 0.0))
                .angle(l(0.0), t)
                .unwrap()
                .arc_to(
                    crate::ArcLen {
                        r: q,
                        side,
                        len: l(1.5),
                    },
                    t,
                )
                .unwrap()
                .line_to(p(-1.0, if verb == "arc len" { 3.0 } else { -3.0 }), t)
                .unwrap()
                .line_to(crate::Start, t)
                .unwrap()
                .loop_
                .into_loop()
        }
        // The `Radius` arrival, an arc extension off the tip, with a
        // fillet between it and a straight arrival: the extension's arc
        // and a fillet in its carrier form.
        "radius" => crate::Open
            .at(p(2.0, -1.0))
            .toward(l(0.0), l(1.0), t)
            .unwrap()
            .line(l(1.0), t)
            .unwrap()
            .arc_fillet(
                crate::Radius {
                    r: q,
                    side: crate::ArcSide::Left,
                },
                l(0.5),
                t,
            )
            .unwrap()
            .toward(l(-1.0), l(0.0), t)
            .unwrap()
            .at(p(-1.0, 1.0), t)
            .unwrap()
            .line(l(1.0), t)
            .unwrap()
            .line_to(crate::Start, t)
            .unwrap()
            .loop_
            .into_loop(),
        other => panic!("no such verb: {other}"),
    }
}

/// Each arc verb with the box its defining quantity ranges over.
pub const ARC_VERBS: [(&str, f64, f64); 15] = [
    ("circle", 0.05, 0.005),
    ("split circle", 0.4, 0.04),
    ("bulge", 0.4, 0.05),
    ("bulge cw", -0.3, 0.05),
    ("via", 5.0, 0.1),
    ("via cw", 3.5, 0.1),
    ("tangent", 1.0, 0.1),
    ("tangent cw", -1.0, 0.1),
    ("onto", 2.0, 0.05),
    ("sweep", 1.0, 0.05),
    ("sweep cw", 1.0, 0.05),
    ("fillet", 0.25, 0.02),
    ("arc len", 1.0, 0.05),
    ("arc len cw", 1.0, 0.05),
    ("radius", 2.0, 1.0e-4),
];
