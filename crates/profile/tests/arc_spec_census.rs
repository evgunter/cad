//! **The lattice's two tables, walked cell by cell against the replay.**
//!
//! `Verb::states` and `arc_specs_at` are read off the declarations the
//! replay driver is expanded from, so they cannot list a row the
//! driver lacks. What they CAN get wrong is the one thing written by
//! hand beside a row: the `specs [..]` annotation naming which
//! dispatcher an arm's spec goes through. This suite puts every
//! (state, verb) cell — and, for an arc-spec verb, every (spec
//! position, mode, target form) cell under it — in front of `replay`,
//! and holds that the replay refuses a cell as a lattice violation
//! exactly when the tables say it does.
//!
//! A cell the tables admit may still refuse on its numbers (a
//! `PathError`): that is the geometry's question, not the lattice's,
//! and it counts as admitted. The numbers below are chosen so the
//! first spec of a two-spec step resolves, because the second is
//! dispatched only after it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::{
    ArcData, ArcMode, ArcSide, ArcSweep, ReplayError, ReplayErrorKind, SpecForms, Step, Target,
    TargetKind, TipState, Verb, arc_specs_at, replay,
};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// A program that leaves the tip in `state` — exhaustive, so a state
/// the lattice gains has to be given a way in before this compiles.
fn prefix(state: TipState) -> Vec<Step<f64>> {
    let at = Step::At(p2(0.0, 0.0));
    let open = || vec![at, Step::Angle(0.0), Step::Fillet { radius: 1.0 }];
    let arrival = |spec| vec![at, Step::Angle(0.0), Step::FilletArc { radius: 1.0, spec }];
    let radius = ArcData::Radius {
        r: 3.0,
        side: ArcSide::Left,
    };
    match state {
        TipState::Entry => vec![],
        TipState::Open => open(),
        TipState::Angle => [open(), vec![Step::Angle(1.5)]].concat(),
        TipState::PlainPoint => vec![at],
        TipState::DirectedPoint => vec![at, Step::LineTo(Target::Point(p2(10.0, 0.0)))],
        TipState::DirectedPlain => vec![at, Step::Angle(0.0)],
        TipState::DirectedIncoming => vec![
            at,
            Step::LineTo(Target::Point(p2(10.0, 0.0))),
            Step::Turn(0.5),
        ],
        TipState::RadiusArrival => arrival(radius),
        TipState::RadiusArrivalAt => [arrival(radius), vec![Step::At(p2(10.0, 10.0))]].concat(),
        TipState::RadiusArrivalDir => [arrival(radius), vec![Step::Angle(1.5)]].concat(),
        TipState::ViaArrival => arrival(ArcData::Via {
            q: p2(8.0, 6.0),
            target: Target::Point(p2(10.0, 10.0)),
        }),
        TipState::ViaArrivalStart => arrival(ArcData::Via {
            q: p2(8.0, 6.0),
            target: Target::Start,
        }),
        TipState::Closed => vec![
            at,
            Step::LineTo(Target::Point(p2(10.0, 0.0))),
            Step::LineTo(Target::Point(p2(0.0, 10.0))),
            Step::LineTo(Target::Start),
        ],
    }
}

/// Every state some verb has a row at, plus `Closed` (which none has):
/// read off the table rather than listed, so a state the table gains
/// is walked here without an edit.
fn every_state() -> Vec<TipState> {
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

/// One step of `verb` with numbers that are not degenerate anywhere in
/// [`prefix`]'s frame; `None` for the arc-spec verbs, which
/// [`arc_step`] builds per cell.
fn sample_step(verb: Verb) -> Option<Step<f64>> {
    let p = p2(3.0, 4.0);
    Some(match verb {
        Verb::At => Step::At(p),
        Verb::Angle => Step::Angle(1.0),
        Verb::Toward => Step::Toward { dx: 1.0, dy: 1.0 },
        Verb::Tangent => Step::Tangent,
        Verb::Cusp => Step::Cusp,
        Verb::Turn => Step::Turn(0.5),
        Verb::Line => Step::Line(2.0),
        Verb::LineTo => Step::LineTo(Target::Point(p)),
        Verb::ContinueTo => Step::ContinueTo(Target::Point(p)),
        Verb::TangentArcTo => Step::TangentArcTo(Target::Point(p)),
        Verb::Fillet => Step::Fillet { radius: 1.0 },
        Verb::FarEndTo => Step::FarEndTo(p),
        Verb::CloseTo => Step::CloseTo,
        Verb::Circle => Step::Circle {
            centre: p,
            radius: 1.0,
        },
        Verb::CircleSplit => Step::CircleSplit {
            centre: p,
            radius: 1.0,
            n: 4,
            phase: 0.0,
        },
        Verb::ArcTo | Verb::FilletArc | Verb::ArcFillet | Verb::ArcFilletArc => return None,
    })
}

/// How many arc specs a step of `verb` carries.
fn spec_count(verb: Verb) -> usize {
    match verb {
        Verb::ArcTo | Verb::FilletArc | Verb::ArcFillet => 1,
        Verb::ArcFilletArc => 2,
        _ => 0,
    }
}

fn arc_step(verb: Verb, specs: &[ArcData<f64>]) -> Step<f64> {
    match (verb, specs) {
        (Verb::ArcTo, &[spec]) => Step::ArcTo(spec),
        (Verb::FilletArc, &[spec]) => Step::FilletArc { radius: 1.0, spec },
        (Verb::ArcFillet, &[spec]) => Step::ArcFillet { spec, radius: 1.0 },
        (Verb::ArcFilletArc, &[spec, spec2]) => Step::ArcFilletArc {
            spec,
            radius: 1.0,
            spec2,
        },
        other => panic!("no arc step {other:?}"),
    }
}

/// A spec of `mode` at target form `target`. The `Center` spec's
/// centre is equidistant from the origin and its `Point` target, which
/// is where the entry's fused incoming starts.
fn spec(mode: ArcMode, target: Option<TargetKind>) -> ArcData<f64> {
    let target = match target {
        None | Some(TargetKind::Point) => Target::Point(p2(5.0, 0.0)),
        Some(TargetKind::Start) => Target::Start,
        Some(TargetKind::StartArriving) => Target::StartArriving,
    };
    match mode {
        ArcMode::Radius => ArcData::Radius {
            r: 3.0,
            side: ArcSide::Left,
        },
        ArcMode::Bulge => ArcData::Bulge { target, b: 0.5 },
        ArcMode::Via => ArcData::Via {
            q: p2(2.5, 2.5),
            target,
        },
        ArcMode::Center => ArcData::Center {
            c: p2(2.5, 0.0),
            winding: ArcSweep::Ccw,
            target,
        },
        ArcMode::Sweep => ArcData::Sweep {
            r: 3.0,
            side: ArcSide::Left,
            angle: 1.0,
        },
        ArcMode::ArcLen => ArcData::ArcLen {
            r: 3.0,
            side: ArcSide::Left,
            len: 2.0,
        },
    }
}

/// Every (mode, target form) cell of the spec vocabulary.
fn every_form() -> Vec<(ArcMode, Option<TargetKind>)> {
    let mut forms = Vec::new();
    for &mode in ArcMode::ALL {
        if spec(mode, None).target().is_some() {
            forms.extend(TargetKind::ALL.iter().map(|&kind| (mode, Some(kind))));
        } else {
            forms.push((mode, None));
        }
    }
    forms
}

/// Whether the replay refused `steps`' last step as a lattice
/// violation of that verb.
fn lattice_refuses(steps: &[Step<f64>]) -> bool {
    let at = steps.len() - 1;
    matches!(
        replay(steps, Tol::witness()),
        Err(ReplayError {
            step,
            kind: ReplayErrorKind::Transition { verb: Some(_), .. },
        }) if step == at
    )
}

#[test]
fn every_prefix_reaches_its_state() {
    for state in every_state() {
        let steps = prefix(state);
        let reached = match replay(&steps, Tol::witness()) {
            Ok(_) => TipState::Closed,
            Err(ReplayError {
                step,
                kind: ReplayErrorKind::Transition { state, verb: None },
            }) if step == steps.len() => state,
            other => panic!("prefix for {state:?} did not replay to a tip: {other:?}"),
        };
        assert_eq!(reached, state);
    }
}

/// **`Verb::states` is the driver's row set.** Every verb that carries
/// no arc spec, at every state: refused as a lattice violation exactly
/// when the table lists no row there.
#[test]
fn verb_states_is_the_replays_row_set() {
    for state in every_state() {
        for &verb in Verb::ALL {
            let Some(step) = sample_step(verb) else {
                continue;
            };
            let steps = [prefix(state), vec![step]].concat();
            let listed = verb.states().contains(&state);
            assert_eq!(
                lattice_refuses(&steps),
                !listed,
                "{verb:?} at {state:?}: the table says {}",
                if listed { "a row" } else { "no row" },
            );
        }
    }
}

/// **`arc_specs_at` is the dispatch each row's spec goes through.**
/// Every arc-spec verb, at every state, every spec position, every
/// (mode, target form): refused as a lattice violation exactly when
/// the forms at that position do not list it. The other positions
/// carry the first form their own dispatcher admits.
#[test]
fn arc_specs_at_is_the_replays_spec_dispatch() {
    for state in every_state() {
        for &verb in Verb::ALL {
            let count = spec_count(verb);
            if count == 0 {
                continue;
            }
            let table: &[&SpecForms] = arc_specs_at(verb, state);
            let has_row = verb.states().contains(&state);
            assert_eq!(
                table.len(),
                if has_row { count } else { 0 },
                "{verb:?} at {state:?}: one forms entry per spec, where there is a row",
            );
            for position in 0..count {
                for (mode, target) in every_form() {
                    let specs: Vec<ArcData<f64>> = (0..count)
                        .map(|i| {
                            if i == position {
                                return spec(mode, target);
                            }
                            let &(m, t) = table
                                .get(i)
                                .and_then(|forms| forms.forms().first())
                                .unwrap_or(&(mode, target));
                            spec(m, t)
                        })
                        .collect();
                    let steps = [prefix(state), vec![arc_step(verb, &specs)]].concat();
                    let admitted = table
                        .get(position)
                        .is_some_and(|forms| forms.admits(mode, target));
                    assert_eq!(
                        lattice_refuses(&steps),
                        !admitted,
                        "{verb:?} at {state:?}, spec {position} = ({mode:?}, {target:?}): \
                         the table {} it",
                        if admitted { "admits" } else { "refuses" },
                    );
                }
            }
        }
    }
}
