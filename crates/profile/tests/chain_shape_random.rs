//! **A chain's shape walks what its replay walks, over random chains.**
//! A counterexample search: chains grown one admitted step at a time
//! from the transition table ([`TipState::verbs`], [`arc_specs_at`]),
//! with random values, every target form and every arc form the row
//! takes. At every prefix, [`chain_shape`] and [`replay`] agree:
//!
//! - where the replay refuses a step on the lattice, the shape refuses
//!   it there on the lattice;
//! - where the replay refuses a step for its order (a structural
//!   `PathError` class), the shape refuses it there for the same class;
//! - where the shape refuses a step for its order, the replay refuses at
//!   that step or before, since no value passes it;
//! - where the replay leaves the tip at a state, or closes, the shape
//!   lands there too; and a closed chain's pieces are the record's
//!   where every fillet fits `Positive`, a superset in order otherwise.
//!
//! A geometry refusal ends the comparison for that chain: it says
//! nothing about the shape past it.
#![allow(clippy::panic)]

test_utils::gated_to![
    "crates/profile/src/path.rs",
    "crates/profile/src/path/",
    "crates/profile/src/structure.rs",
    "crates/profile/tests/chain_shape_random.rs",
];

use geom_core::{Point2, Sign, Tol};
use profile::{
    ArcData, ArcMode, ArcSide, ArcSweep, PathErrorKind, ReplayError, ReplayErrorKind, ShapeFault,
    ShapeFaultKind, SpecForms, Step, StepForm, Target, TargetKind, TipState, Verb, arc_specs_at,
    chain_shape, replay, replay_recording,
};
use test_utils::fuzz::Rng;

/// The `PathError` classes the replay's order refusals raise
/// (`profile::path::order`).
const ORDER: [PathErrorKind; 5] = [
    PathErrorKind::ArcLegOnOpenFillet,
    PathErrorKind::FarEndAnchorWithoutFillet,
    PathErrorKind::SeamRetrimsArcFirstSide,
    PathErrorKind::CircleSplitCount,
    PathErrorKind::UnderdeterminedLeg,
];

/// A point on a coarse grid, so coincidences happen.
fn pt(r: &mut Rng) -> Point2<f64> {
    Point2::new(r.below(11) as f64 - 5.0, r.below(11) as f64 - 5.0)
}
fn tgt(r: &mut Rng, k: TargetKind) -> Target<f64> {
    match k {
        TargetKind::Point => Target::Point(pt(r)),
        TargetKind::Start => Target::Start,
        TargetKind::StartArriving => Target::StartArriving,
    }
}
fn spec(r: &mut Rng, mode: ArcMode, k: Option<TargetKind>) -> ArcData<f64> {
    let t = k.map(|k| tgt(r, k)).unwrap_or(Target::Start);
    let side = if r.below(2) == 0 {
        ArcSide::Left
    } else {
        ArcSide::Right
    };
    match mode {
        ArcMode::Radius => ArcData::Radius {
            r: r.range(0.5, 3.0),
            side,
        },
        ArcMode::Bulge => ArcData::Bulge {
            target: t,
            b: r.range(-0.8, 0.8),
        },
        ArcMode::Via => ArcData::Via {
            q: pt(r),
            target: t,
        },
        ArcMode::Center => ArcData::Center {
            c: pt(r),
            winding: if r.below(2) == 0 {
                ArcSweep::Ccw
            } else {
                ArcSweep::Cw
            },
            target: t,
        },
        ArcMode::Sweep => ArcData::Sweep {
            r: r.range(0.5, 3.0),
            side,
            angle: r.range(0.2, 2.5),
        },
        ArcMode::ArcLen => ArcData::ArcLen {
            r: r.range(0.5, 3.0),
            side,
            len: r.range(0.3, 3.0),
        },
    }
}
fn pick_spec(r: &mut Rng, forms: &SpecForms) -> ArcData<f64> {
    let f = forms.forms();
    let (m, k) = f[r.below(f.len())];
    spec(r, m, k)
}
fn step(r: &mut Rng, verb: Verb, state: TipState) -> Step<f64> {
    let anyk = |r: &mut Rng| TargetKind::ALL[r.below(3)];
    let specs = arc_specs_at(verb, state);
    match verb {
        Verb::At => Step::At(pt(r)),
        Verb::Angle => Step::Angle((r.below(8) as f64) * std::f64::consts::FRAC_PI_4),
        Verb::Toward => Step::Toward {
            dx: r.below(3) as f64 - 1.0,
            dy: r.below(3) as f64 - 1.0 + 0.5,
        },
        Verb::Tangent => Step::Tangent,
        Verb::Cusp => Step::Cusp,
        Verb::Turn => Step::Turn(r.range(-2.0, 2.0)),
        Verb::Line => Step::Line(r.range(0.5, 5.0)),
        Verb::LineTo => {
            let k = anyk(r);
            Step::LineTo(tgt(r, k))
        }
        Verb::ContinueTo => {
            let k = anyk(r);
            Step::ContinueTo(tgt(r, k))
        }
        Verb::TangentArcTo => {
            let k = anyk(r);
            Step::TangentArcTo(tgt(r, k))
        }
        Verb::ArcOnto => {
            let k = anyk(r);
            Step::ArcOnto(tgt(r, k))
        }
        Verb::ArcTo => Step::ArcTo(pick_spec(r, specs[0])),
        Verb::Fillet => Step::Fillet {
            radius: r.range(0.2, 1.5),
        },
        Verb::FilletArc => Step::FilletArc {
            radius: r.range(0.2, 1.5),
            spec: pick_spec(r, specs[0]),
        },
        Verb::ArcFillet => Step::ArcFillet {
            spec: pick_spec(r, specs[0]),
            radius: r.range(0.2, 1.5),
        },
        Verb::ArcFilletArc => Step::ArcFilletArc {
            spec: pick_spec(r, specs[0]),
            radius: r.range(0.2, 1.5),
            spec2: pick_spec(r, specs[1]),
        },
        Verb::FarEndTo => Step::FarEndTo(pt(r)),
        Verb::CloseTo => Step::CloseTo,
        Verb::Circle => Step::Circle {
            centre: pt(r),
            radius: r.range(0.5, 2.0),
        },
        Verb::CircleSplit => Step::CircleSplit {
            centre: pt(r),
            radius: r.range(0.5, 2.0),
            n: r.below(5),
            phase: 0.0,
        },
        Verb::Rectangle => Step::Rectangle {
            centre: pt(r),
            width: r.range(0.5, 2.0),
            height: r.range(0.5, 2.0),
        },
    }
}

/// One prefix compared: `Err` with what disagreed.
fn compare(steps: &[Step<f64>]) -> Result<bool, String> {
    let forms: Vec<StepForm> = steps.iter().map(Step::form).collect();
    let shape = chain_shape(&forms);
    let replayed = replay_recording(steps, Tol::witness());
    let say = |what: &str| {
        Err(format!(
            "{what}: {steps:?}\nshape {shape:?}\nreplay {:?}",
            replayed.as_ref().map(|_| ())
        ))
    };
    match (&shape, &replayed) {
        (Ok(shape), Ok((_, record))) => {
            let generic = record
                .fillets
                .iter()
                .all(|f| f.fit_in == Sign::Positive && f.fit_out == Sign::Positive);
            let mut rest = shape.pieces.iter();
            let ordered = record.pieces.iter().all(|p| rest.any(|q| q == p));
            if (generic && shape.pieces != record.pieces) || !ordered {
                return say("the pieces differ");
            }
            Ok(true)
        }
        (
            _,
            Err(ReplayError {
                step,
                kind: ReplayErrorKind::Transition { state, verb },
            }),
        ) => match shape {
            Err(ShapeFault {
                step: s,
                state: t,
                kind: ShapeFaultKind::Lattice { verb: v },
            }) if s == *step && t == *state && v == *verb => Ok(verb.is_none()),
            _ => say("the replay refuses on the lattice"),
        },
        (
            _,
            Err(ReplayError {
                step,
                kind: ReplayErrorKind::Path(e),
            }),
        ) if ORDER.contains(&e.kind()) => match shape {
            Err(ShapeFault {
                step: s,
                kind: ShapeFaultKind::Order(f),
                ..
            }) if s == *step && f.kind() == e.kind() => Ok(false),
            _ => say("the replay refuses the order"),
        },
        (
            Err(ShapeFault {
                step,
                kind: ShapeFaultKind::Order(_),
                ..
            }),
            _,
        ) => match &replayed {
            Err(ReplayError { step: s, .. }) if s <= step => Ok(false),
            _ => say("the shape refuses an order the replay passes"),
        },
        (
            Err(ShapeFault {
                kind: ShapeFaultKind::Lattice { .. },
                ..
            }),
            Ok(_),
        ) => say("the shape refuses a chain that closes"),
        // A geometry refusal: nothing to compare.
        _ => Ok(false),
    }
}

#[test]
fn a_chains_shape_walks_what_its_replay_walks() {
    let mut r = test_utils::fuzz::start("chain-shape-random");
    for _ in 0..test_utils::fuzz::scaled(4_000) {
        let mut steps: Vec<Step<f64>> = Vec::new();
        let mut state = TipState::Entry;
        for _ in 0..9 {
            let verbs: Vec<Verb> = state.verbs().collect();
            let verb = verbs[r.below(verbs.len())];
            steps.push(step(&mut r, verb, state));
            if let Err(what) = compare(&steps) {
                panic!("{what}\n{}", test_utils::fuzz::replay());
            }
            let forms: Vec<StepForm> = steps.iter().map(Step::form).collect();
            state = match (chain_shape(&forms), replay(&steps, Tol::witness())) {
                (
                    Err(ShapeFault {
                        step,
                        state,
                        kind: ShapeFaultKind::Lattice { verb: None },
                    }),
                    Err(_),
                ) if step == steps.len() => state,
                _ => break,
            };
        }
    }
}
