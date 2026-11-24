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
//! - where the shape refuses a step for its order, the replay refuses
//!   there in the same class, since no value passes it, unless a value
//!   refused an earlier step, or one the step reads refused it first
//!   and a redraw of the chain's values (its forms kept) shows the
//!   order refusal;
//! - where the replay refuses a zero chord at a step, and does at that
//!   step at every redraw, the shape refuses it there for its order: an
//!   arc that closes on its own entry is the one such case, and it
//!   shares its class with a coincidence of two values, so only the
//!   redraw tells them apart;
//! - where the replay leaves the tip at a state, or closes, the shape
//!   lands there too; and a closed chain's pieces are the record's
//!   where every fillet fits `Positive`, a superset in order otherwise;
//!   there, a piece continues the one before it where the replay
//!   constructed their joint onto one carrier ([`joint_agrees`]).
//!
//! Any other geometry refusal ends the comparison for that chain: it
//! says nothing about the shape past it. The run asserts it compared
//! enough closed chains and order refusals that a generator change
//! cannot leave it vacuous.
#![allow(clippy::panic)]

test_utils::gated_to![
    "crates/profile/src/path.rs",
    "crates/profile/src/path/",
    "crates/profile/src/structure.rs",
    "crates/profile/tests/chain_shape_random.rs",
];

use geom_core::{Point2, Sign, Tol};
use profile::{
    ArcData, ArcForm, ArcMode, ArcSide, ArcSweep, PathErrorKind, ReplayErrorKind, Segment,
    ShapeFault, ShapeFaultKind, SpecForms, Step, StepForm, Target, TargetKind, TipState, Verb,
    arc_specs_at, chain_shape, replay, replay_recording,
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

/// An arc spec of form `form`, with fresh values.
fn spec_of(r: &mut Rng, form: ArcForm) -> ArcData<f64> {
    match form {
        ArcForm::Radius(side) => ArcData::Radius {
            r: r.range(0.5, 3.0),
            side,
        },
        ArcForm::Bulge(k) => ArcData::Bulge {
            target: tgt(r, k),
            b: r.range(-0.8, 0.8),
        },
        ArcForm::Via(k) => ArcData::Via {
            q: pt(r),
            target: tgt(r, k),
        },
        ArcForm::Center(winding, k) => ArcData::Center {
            c: pt(r),
            winding,
            target: tgt(r, k),
        },
        ArcForm::Sweep(side) => ArcData::Sweep {
            r: r.range(0.5, 3.0),
            side,
            angle: r.range(0.2, 2.5),
        },
        ArcForm::ArcLen(side) => ArcData::ArcLen {
            r: r.range(0.5, 3.0),
            side,
            len: r.range(0.3, 3.0),
        },
    }
}

/// `step` with its values redrawn and its form kept.
fn redrawn(r: &mut Rng, step: &Step<f64>) -> Step<f64> {
    match step.form() {
        StepForm::At => Step::At(pt(r)),
        StepForm::Angle => Step::Angle((r.below(8) as f64) * std::f64::consts::FRAC_PI_4),
        StepForm::Toward => Step::Toward {
            dx: r.below(3) as f64 - 1.0,
            dy: r.below(3) as f64 - 1.0 + 0.5,
        },
        StepForm::Tangent => Step::Tangent,
        StepForm::Cusp => Step::Cusp,
        StepForm::Turn => Step::Turn(r.range(-2.0, 2.0)),
        StepForm::Line => Step::Line(r.range(0.5, 5.0)),
        StepForm::LineTo(k) => Step::LineTo(tgt(r, k)),
        StepForm::ContinueTo(k) => Step::ContinueTo(tgt(r, k)),
        StepForm::ArcTo(a) => Step::ArcTo(spec_of(r, a)),
        StepForm::TangentArcTo(k) => Step::TangentArcTo(tgt(r, k)),
        StepForm::ArcOnto(k) => Step::ArcOnto(tgt(r, k)),
        StepForm::Fillet => Step::Fillet {
            radius: r.range(0.2, 1.5),
        },
        StepForm::FilletArc(a) => Step::FilletArc {
            radius: r.range(0.2, 1.5),
            spec: spec_of(r, a),
        },
        StepForm::ArcFillet(a) => Step::ArcFillet {
            spec: spec_of(r, a),
            radius: r.range(0.2, 1.5),
        },
        StepForm::ArcFilletArc(a, b) => Step::ArcFilletArc {
            spec: spec_of(r, a),
            radius: r.range(0.2, 1.5),
            spec2: spec_of(r, b),
        },
        StepForm::FarEndTo => Step::FarEndTo(pt(r)),
        StepForm::CloseTo => Step::CloseTo,
        StepForm::Circle => Step::Circle {
            centre: pt(r),
            radius: r.range(0.5, 2.0),
        },
        StepForm::CircleSplit { n } => Step::CircleSplit {
            centre: pt(r),
            radius: r.range(0.5, 2.0),
            n,
            phase: 0.0,
        },
        StepForm::Rectangle => Step::Rectangle {
            centre: pt(r),
            width: r.range(0.5, 2.0),
            height: r.range(0.5, 2.0),
        },
        StepForm::Lowered(never) => match never {},
    }
}

/// How many redraws of a chain's values a refusal is read at.
const REDRAWS: usize = 8;

/// The replay's refusal of `steps` with their values redrawn and their
/// forms kept, [`REDRAWS`] times.
fn redraws(r: &mut Rng, steps: &[Step<f64>]) -> Vec<Option<(usize, profile::PathError<f64>)>> {
    (0..REDRAWS)
        .map(|_| {
            let again: Vec<Step<f64>> = steps.iter().map(|s| redrawn(r, s)).collect();
            match replay(&again, Tol::witness()) {
                Err(error) => {
                    let at = error.raised_at();
                    match error.kind {
                        ReplayErrorKind::Path(e) => Some((at, e)),
                        ReplayErrorKind::Transition { .. } => None,
                    }
                }
                Ok(_) => None,
            }
        })
        .collect()
}

/// A chord that closes on itself: exactly zero, as a coincidence of two
/// grid points or a tip that is the entry makes it.
fn zero_chord(e: &profile::PathError<f64>) -> bool {
    matches!(e, profile::PathError::DegenerateArcChord { chord }
        if crate::common::told(chord) == 0.0)
}

/// What one prefix's comparison reached.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Compared {
    /// Both closed the chain, and their pieces were compared; the
    /// continuing joints a generic one held against the replay, by
    /// construction ([`Kind`]).
    Closed([usize; KINDS]),
    /// Both refused the step order, at one step in one class.
    Order,
    /// Both refused on the lattice (`true` where the chain merely ended
    /// open, so it may grow).
    Lattice(bool),
    /// The replay refused for a value: nothing compared past it.
    Geometry,
}

/// Whether `piece` of `steps` is a straight segment by construction,
/// at every value: a straight leg, a straight fillet run, a rectangle's
/// side. An arc can be straight at some values (a zero bulge), which is
/// a coincidence, not a construction.
fn straight(steps: &[Step<f64>], piece: profile::Piece) -> bool {
    use profile::PieceRole as R;
    matches!(
        (piece.role, steps[piece.step].form()),
        (
            R::Leg,
            StepForm::Line | StepForm::LineTo(_) | StepForm::ContinueTo(_) | StepForm::FarEndTo,
        ) | (R::RunIn, StepForm::Fillet | StepForm::FilletArc(_))
            | (R::RunOut, StepForm::Fillet | StepForm::ArcFillet(_))
            | (R::Piece(_), StepForm::Rectangle)
    )
}

/// **How a continuing piece was constructed**, which the random run
/// counts so a generator change cannot leave a construction unreached.
#[derive(Clone, Copy, Debug)]
enum Kind {
    /// A `line` off a leg end.
    Line,
    /// A `line` after `.tangent()`.
    Tangent,
    /// A fillet's run in off its leg.
    RunIn,
    /// `continue_to`, whose target the generator rarely puts on the ray.
    ContinueTo,
    /// A station of a `circle` or a `circle_split`.
    Station,
}

const KINDS: usize = 5;

/// The construction of `piece`, which continues the piece before it.
fn kind(steps: &[Step<f64>], piece: profile::Piece) -> Kind {
    match (piece.role, &steps[piece.step]) {
        (profile::PieceRole::RunIn, _) => Kind::RunIn,
        (profile::PieceRole::Piece(_), _) => Kind::Station,
        (_, Step::ContinueTo(_)) => Kind::ContinueTo,
        _ if piece.step > 0 && matches!(steps[piece.step - 1], Step::Tangent) => Kind::Tangent,
        _ => Kind::Line,
    }
}

/// **The verbs that construct a continuation at `state`**, after the
/// step `last`: a leg end's, and `.tangent()`'s. The run picks among
/// them half the time where there are any, and closes a leg end with a
/// `line_to(Start)` a quarter of the time, since a uniform pick of the
/// table's verbs closes a chain with a straight continuation in it a
/// handful of times a run.
fn continuing(state: TipState, last: Option<&Step<f64>>) -> &'static [Verb] {
    match (state, last) {
        (TipState::DirectedPoint, _) => &[Verb::Line, Verb::Tangent, Verb::Fillet, Verb::FilletArc],
        (TipState::DirectedIncoming, Some(Step::Tangent)) => {
            &[Verb::Line, Verb::Fillet, Verb::FilletArc]
        }
        _ => &[],
    }
}

/// Whether the replay constructed the joint at the start of segment
/// `k` of `lp` between two arcs on one circle: the same centre and
/// radius to the rounding two constructions of one circle leave.
fn on_one_circle(lp: &profile::ConstructedLoop<f64>, k: usize) -> bool {
    let segs = lp.segments();
    let n = segs.len();
    let constructed = lp.constructed_joints().binary_search(&k).is_ok();
    match (&segs[(k + n - 1) % n], &segs[k]) {
        (Segment::Arc(a), Segment::Arc(b)) => {
            constructed
                && a.centre.distance(b.centre) + (a.radius - b.radius).abs()
                    <= 1e-9 * (1.0 + a.radius)
        }
        _ => false,
    }
}

/// The joint at the start of piece `k` of a generic closed chain, its
/// segment `k` of `lp`, against whether the shape says the piece
/// continues the one before: between two pieces straight by
/// construction, exactly where the replay constructed the joint and
/// they run one way; between two arcs, exactly where it constructed
/// the joint on one circle ([`on_one_circle`]) at every value. A coarse
/// grid also puts two arcs on one circle by value (a `via` point at the
/// first arc's radius from its centre), so a joint the shape calls a
/// corner and the replay put on one circle is a miss only where every
/// redraw of the chain's values that closes it alike keeps it there.
fn joint_agrees(
    r: &mut Rng,
    steps: &[Step<f64>],
    shape: &profile::ChainShape,
    lp: &profile::ConstructedLoop<f64>,
    k: usize,
) -> bool {
    let (v, segs) = (lp.vertices(), lp.segments());
    let n = segs.len();
    let p = (k + n - 1) % n;
    let continues = shape.continues[k];
    let constructed = lp.constructed_joints().binary_search(&k).is_ok();
    match (&segs[p], &segs[k]) {
        (Segment::Line, Segment::Line)
            if straight(steps, shape.pieces[p]) && straight(steps, shape.pieces[k]) =>
        {
            let (a, b) = (v[k] - v[p], v[(k + 1) % n] - v[k]);
            continues == (constructed && a.x * b.x + a.y * b.y > 0.0)
        }
        (Segment::Arc(_), Segment::Arc(_)) if continues => on_one_circle(lp, k),
        (Segment::Arc(_), Segment::Arc(_)) if on_one_circle(lp, k) => {
            let mut closed = 0;
            for _ in 0..REDRAWS {
                let again: Vec<Step<f64>> = steps.iter().map(|s| redrawn(r, s)).collect();
                let Ok((lp, record)) = replay_recording(&again, Tol::witness()) else {
                    continue;
                };
                if record.pieces != shape.pieces || lp.segments().len() != n {
                    continue;
                }
                if !on_one_circle(&lp, k) {
                    return true;
                }
                closed += 1;
            }
            closed == 0
        }
        _ => !continues,
    }
}

/// One prefix compared: `Err` with what disagreed.
fn compare(r: &mut Rng, steps: &[Step<f64>]) -> Result<Compared, String> {
    let forms: Vec<StepForm> = steps.iter().map(Step::form).collect();
    let shape = chain_shape(&forms);
    let replayed = replay_recording(steps, Tol::witness());
    // Each refusal at the step the replay was applying, which is the
    // step the shape walk reached.
    let replayed_at = replayed.as_ref().map_err(|e| (e.raised_at(), &e.kind));
    let say = |what: &str| {
        Err(format!(
            "{what}: {steps:?}\nshape {shape:?}\nreplay {:?}",
            replayed.as_ref().map(|_| ())
        ))
    };
    match (&shape, &replayed_at) {
        (Ok(shape), Ok((lp, record))) => {
            let generic = record
                .fillets
                .iter()
                .all(|f| f.fit_in == Sign::Positive && f.fit_out == Sign::Positive);
            let mut rest = shape.pieces.iter();
            let ordered = record.pieces.iter().all(|p| rest.any(|q| q == p));
            if (generic && shape.pieces != record.pieces) || !ordered {
                return say("the pieces differ");
            }
            let disagrees =
                || (0..shape.continues.len()).find(|&k| !joint_agrees(r, steps, shape, lp, k));
            if let Some(k) = generic.then(disagrees).flatten() {
                return say(&format!(
                    "the shape says piece {k} continues the one before it ({}), against the \
                     replay's construction: {:?}",
                    shape.continues[k], shape.continues
                ));
            }
            let mut kinds = [0; KINDS];
            if generic {
                for k in (0..shape.continues.len()).filter(|&k| shape.continues[k]) {
                    kinds[kind(steps, shape.pieces[k]) as usize] += 1;
                }
            }
            Ok(Compared::Closed(kinds))
        }
        (_, Err((step, ReplayErrorKind::Transition { state, verb }))) => match shape {
            Err(ShapeFault {
                step: s,
                state: t,
                kind: ShapeFaultKind::Lattice { verb: v },
            }) if s == *step && t == *state && v == *verb => Ok(Compared::Lattice(verb.is_none())),
            _ => say("the replay refuses on the lattice"),
        },
        (_, Err((step, ReplayErrorKind::Path(e)))) if ORDER.contains(&e.kind()) => match shape {
            Err(ShapeFault {
                step: s,
                kind: ShapeFaultKind::Order(f),
                ..
            }) if s == *step && f.kind() == e.kind() => Ok(Compared::Order),
            _ => say("the replay refuses the order"),
        },
        (
            Err(ShapeFault {
                step,
                kind: ShapeFaultKind::Order(f),
                ..
            }),
            _,
        ) => match &replayed_at {
            Err((s, ReplayErrorKind::Path(e))) if s == step && e.kind() == f.kind() => {
                Ok(Compared::Order)
            }
            // A value refused an earlier step: nothing reached this one.
            Err((s, _)) if s < step => Ok(Compared::Geometry),
            // A value the step reads refused it before its order was
            // asked; at some other values the order refusal stands.
            Err((s, ReplayErrorKind::Path(_)))
                if s == step
                    && redraws(r, steps)
                        .iter()
                        .flatten()
                        .any(|(s, e)| s == step && e.kind() == f.kind()) =>
            {
                Ok(Compared::Order)
            }
            _ => say("the shape refuses an order the replay does not refuse there in its class"),
        },
        (Ok(_) | Err(ShapeFault { .. }), Err((step, ReplayErrorKind::Path(e))))
            if zero_chord(e)
                && redraws(r, steps)
                    .iter()
                    .all(|at| at.as_ref().is_some_and(|(s, e)| s == step && zero_chord(e))) =>
        {
            say("the replay refuses a degenerate chord at every value, and the shape passes it")
        }
        (
            Err(ShapeFault {
                kind: ShapeFaultKind::Lattice { .. },
                ..
            }),
            Ok(_),
        ) => say("the shape refuses a chain that closes"),
        // A geometry refusal: nothing to compare.
        _ => Ok(Compared::Geometry),
    }
}

#[test]
fn a_chains_shape_walks_what_its_replay_walks() {
    let mut r = test_utils::fuzz::start("chain-shape-random");
    let chains = test_utils::fuzz::scaled(4_000);
    let (mut closed, mut orders) = (0, 0);
    let mut continuing_joints = [0; KINDS];
    for _ in 0..chains {
        let mut steps: Vec<Step<f64>> = Vec::new();
        let mut state = TipState::Entry;
        for _ in 0..9 {
            let verbs: Vec<Verb> = state.verbs().collect();
            let steered = continuing(state, steps.last());
            let next = match r.below(4) {
                0 | 1 if !steered.is_empty() => {
                    let verb = steered[r.below(steered.len())];
                    step(&mut r, verb, state)
                }
                2 if state == TipState::DirectedPoint => Step::LineTo(Target::Start),
                _ => {
                    let verb = verbs[r.below(verbs.len())];
                    step(&mut r, verb, state)
                }
            };
            steps.push(next);
            match compare(&mut r, &steps) {
                Ok(Compared::Closed(kinds)) => {
                    closed += 1;
                    for (n, k) in continuing_joints.iter_mut().zip(kinds) {
                        *n += k;
                    }
                }
                Ok(Compared::Order) => orders += 1,
                Ok(Compared::Lattice(_) | Compared::Geometry) => {}
                Err(what) => panic!("{what}\n{}", test_utils::fuzz::replay()),
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
    // Effort 1 compares about 1,400 of each over 4,000 chains.
    assert!(
        closed >= chains / 4 && orders >= chains / 4,
        "the run compared {closed} closed chains and {orders} order refusals over {chains} \
         chains: the generator no longer reaches them"
    );
    eprintln!(
        "[chain-shape-random] continuing joints held, by construction: {continuing_joints:?}"
    );
}

/// **Every construction that continues a carrier, at random values**:
/// a counterexample search over chains built to close with one
/// continuation in them — a box whose bottom side is a leg and a
/// continuation of it (a `line`, `.tangent()` then a `line`,
/// `continue_to`, a fillet's run in into the right side), or a circle
/// cut at stations — each joint held against the replay's construction
/// ([`joint_agrees`]). The random chains above reach the straight
/// continuations a few times a run; this reaches each a few hundred
/// times, and asserts it did.
#[test]
fn every_continuing_construction_is_the_replays_at_random_values() {
    let mut r = test_utils::fuzz::start("chain-shape-continuations");
    let chains = test_utils::fuzz::scaled(1_000);
    let mut held = [0; KINDS];
    for _ in 0..chains {
        let o = Point2::new(r.range(-5.0, 5.0), r.range(-5.0, 5.0));
        let (a, b, h) = (r.range(0.5, 3.0), r.range(0.5, 3.0), r.range(0.5, 3.0));
        let corner = Point2::new(o.x + a + b, o.y);
        let north = Step::Toward { dx: 0.0, dy: 1.0 };
        let mut steps = vec![
            Step::At(o),
            Step::Toward { dx: 1.0, dy: 0.0 },
            Step::Line(a),
        ];
        match r.below(KINDS) {
            0 => steps.extend([Step::Line(b), north, Step::Line(h)]),
            1 => steps.extend([Step::Tangent, Step::Line(b), north, Step::Line(h)]),
            2 => steps.extend([
                Step::ContinueTo(Target::Point(corner)),
                north,
                Step::Line(h),
            ]),
            3 => {
                let radius = r.range(0.1, 0.9) * b.min(h);
                steps.extend([
                    Step::Fillet { radius },
                    north,
                    Step::At(Point2::new(corner.x, corner.y + radius)),
                    Step::Line(h - radius),
                ]);
            }
            _ => {
                let (centre, radius) = (o, r.range(0.5, 2.0));
                steps = vec![match r.below(2) {
                    0 => Step::Circle { centre, radius },
                    _ => Step::CircleSplit {
                        centre,
                        radius,
                        n: 2 + r.below(5),
                        phase: r.range(-3.0, 3.0),
                    },
                }];
            }
        }
        if steps.len() > 1 {
            steps.extend([
                Step::LineTo(Target::Point(Point2::new(o.x, o.y + h))),
                Step::LineTo(Target::Start),
            ]);
        }
        let forms: Vec<StepForm> = steps.iter().map(Step::form).collect();
        let shape = chain_shape(&forms).unwrap_or_else(|e| {
            panic!(
                "the shape walks {steps:?}: {e:?}\n{}",
                test_utils::fuzz::replay()
            )
        });
        let (lp, record) = replay_recording(&steps, Tol::witness()).unwrap_or_else(|e| {
            panic!(
                "the chain replays {steps:?}: {e:?}\n{}",
                test_utils::fuzz::replay()
            )
        });
        assert_eq!(
            shape.pieces,
            record.pieces,
            "every piece of {steps:?} is drawn\n{}",
            test_utils::fuzz::replay()
        );
        for k in 0..shape.continues.len() {
            assert!(
                joint_agrees(&mut r, &steps, &shape, &lp, k),
                "joint {k} of {steps:?} is the replay's construction: {:?}\n{}",
                shape.continues,
                test_utils::fuzz::replay()
            );
            if shape.continues[k] {
                held[kind(&steps, shape.pieces[k]) as usize] += 1;
            }
        }
    }
    // Each kind is drawn about a fifth of the time.
    assert!(
        held.iter().all(|&n| n >= chains / 10),
        "the run held {held:?} continuing joints of each construction over {chains} chains: \
         the generator no longer reaches them all"
    );
}

/// **Each order refusal, by name**, at the replay and at the shape
/// alike: the step and the class, where a random chain would reach it
/// only by chance. A fused verb's incoming arc from a fillet's open
/// arrival is the replay's own finding of this program (it used to
/// overwrite the open fillet silently); an arc closing on its own entry
/// shares its class with a coincidence of values, so the random rows
/// compare it only through redraws.
#[test]
fn each_order_refusal_refuses_at_the_replay_and_the_shape_alike() {
    let p = Point2::new;
    let east = Step::Toward { dx: 1.0, dy: 0.0 };
    let open_fillet = vec![
        Step::At(p(0.0, 0.0)),
        east,
        Step::Fillet { radius: 0.5 },
        Step::At(p(4.0, 2.0)),
    ];
    let with = |head: &[Step<f64>], tail: &[Step<f64>]| -> Vec<Step<f64>> {
        head.iter().chain(tail).copied().collect()
    };
    let rows: Vec<(&str, Vec<Step<f64>>, usize, PathErrorKind)> = vec![
        (
            "a fused arc incoming from a fillet's open arrival",
            with(
                &open_fillet,
                &[Step::ArcFillet {
                    spec: ArcData::Via {
                        q: p(5.0, 4.0),
                        target: Target::Point(p(4.0, 6.0)),
                    },
                    radius: 0.5,
                }],
            ),
            4,
            PathErrorKind::ArcLegOnOpenFillet,
        ),
        (
            "an arc leg from a fillet's open arrival",
            with(
                &open_fillet,
                &[Step::ArcTo(ArcData::Via {
                    q: p(5.0, 4.0),
                    target: Target::Point(p(4.0, 6.0)),
                })],
            ),
            4,
            PathErrorKind::ArcLegOnOpenFillet,
        ),
        (
            "arc_onto(StartArriving) from the entry itself",
            vec![Step::At(p(1.0, 2.0)), Step::ArcOnto(Target::StartArriving)],
            1,
            PathErrorKind::DegenerateArcChord,
        ),
        (
            "arc_onto(Start) from the entry, directed after",
            vec![Step::At(p(1.0, 2.0)), Step::ArcOnto(Target::Start), east],
            2,
            PathErrorKind::DegenerateArcChord,
        ),
        (
            "a bulge arc to the start from the entry itself",
            vec![
                Step::At(p(1.0, 2.0)),
                Step::ArcTo(ArcData::Bulge {
                    target: Target::Start,
                    b: 0.3,
                }),
            ],
            1,
            PathErrorKind::DegenerateArcChord,
        ),
        (
            "a line to the start from the entry itself",
            vec![Step::At(p(1.0, 2.0)), Step::LineTo(Target::Start)],
            1,
            PathErrorKind::UnderdeterminedLeg,
        ),
        (
            "a far-end anchor with no fillet open",
            vec![east, Step::FarEndTo(p(2.0, 0.0))],
            1,
            PathErrorKind::FarEndAnchorWithoutFillet,
        ),
        (
            "a circle split into one arc",
            vec![Step::CircleSplit {
                centre: p(0.0, 0.0),
                radius: 1.0,
                n: 1,
                phase: 0.0,
            }],
            0,
            PathErrorKind::CircleSplitCount,
        ),
    ];
    for (what, steps, at, kind) in rows {
        let forms: Vec<StepForm> = steps.iter().map(Step::form).collect();
        match chain_shape(&forms) {
            Err(ShapeFault {
                step,
                kind: ShapeFaultKind::Order(f),
                ..
            }) => assert_eq!((step, f.kind()), (at, kind), "{what}: the shape"),
            other => panic!("{what}: the shape refuses the order, got {other:?}"),
        }
        match replay(&steps, Tol::witness()).map_err(|e| (e.raised_at(), e.kind)) {
            Err((step, ReplayErrorKind::Path(e))) => {
                assert_eq!((step, e.kind()), (at, kind), "{what}: the replay")
            }
            other => panic!(
                "{what}: the replay refuses the order, got {:?}",
                other.err()
            ),
        }
    }
}

/// **Each joint a chain constructs onto one carrier, by name**: the
/// shape's `continues` on chains that reach each row of it, held
/// against the replay's own construction ([`joint_agrees`]). A
/// straight piece departing along a straight piece's own ray continues
/// it (a line off a leg end, `.tangent()` then a line, `continue_to`,
/// a fillet's run in off its leg); a direction bound by a `turn`, a
/// director or a `line_to`'s target is a corner, as is a line leaving
/// an arc; every station of a circle continues it, as both joints of a
/// lone arc closed by a tangent arc off its own tangent do.
#[test]
fn each_continuation_the_shape_reads_is_the_replays_construction() {
    let p = Point2::new;
    let east = Step::Toward { dx: 1.0, dy: 0.0 };
    let north = Step::Toward { dx: 0.0, dy: 1.0 };
    let close = [
        Step::LineTo(Target::Point(p(0.0, 2.0))),
        Step::LineTo(Target::Start),
    ];
    let side = |middle: &[Step<f64>]| -> Vec<Step<f64>> {
        let mut steps = vec![Step::At(p(0.0, 0.0)), east, Step::Line(1.0)];
        steps.extend_from_slice(middle);
        steps.extend_from_slice(&[north, Step::Line(2.0)]);
        steps.extend_from_slice(&close);
        steps
    };
    type Row = (&'static str, Vec<Step<f64>>, Vec<bool>);
    let rows: Vec<Row> = vec![
        (
            "a station kept on a side",
            side(&[Step::Line(1.0)]),
            vec![false, true, false, false, false],
        ),
        (
            "a declared continuation",
            side(&[Step::ContinueTo(Target::Point(p(2.0, 0.0)))]),
            vec![false, true, false, false, false],
        ),
        (
            "a tangent kept",
            side(&[Step::Tangent, Step::Line(1.0)]),
            vec![false, true, false, false, false],
        ),
        (
            "a line_to's end continued",
            [
                vec![
                    Step::At(p(0.0, 0.0)),
                    Step::LineTo(Target::Point(p(1.0, 0.0))),
                    Step::Line(1.0),
                    north,
                    Step::Line(2.0),
                ],
                close.to_vec(),
            ]
            .concat(),
            vec![false, true, false, false, false],
        ),
        (
            "a turn",
            side(&[Step::Turn(0.5), Step::Line(1.0)]),
            vec![false; 5],
        ),
        (
            "a fillet's run in off its leg",
            [
                vec![
                    Step::At(p(0.0, 0.0)),
                    east,
                    Step::Line(1.0),
                    Step::Fillet { radius: 0.5 },
                    north,
                    Step::At(p(2.0, 0.5)),
                    Step::Line(1.5),
                ],
                close.to_vec(),
            ]
            .concat(),
            vec![false, true, false, false, false, false],
        ),
        (
            "a fillet opened at the entry",
            [
                vec![
                    Step::At(p(0.0, 0.0)),
                    east,
                    Step::Fillet { radius: 0.5 },
                    north,
                    Step::FarEndTo(p(2.0, 2.0)),
                ],
                close.to_vec(),
            ]
            .concat(),
            vec![false; 5],
        ),
        (
            "a line off an arc",
            vec![
                Step::At(p(0.0, 0.0)),
                east,
                Step::ArcTo(ArcData::Sweep {
                    r: 1.0,
                    side: ArcSide::Left,
                    angle: core::f64::consts::FRAC_PI_2,
                }),
                Step::Line(1.0),
                Step::LineTo(Target::Start),
            ],
            vec![false; 3],
        ),
        (
            "a split circle",
            vec![Step::CircleSplit {
                centre: p(0.0, 0.0),
                radius: 1.0,
                n: 4,
                phase: 0.0,
            }],
            vec![true; 4],
        ),
        (
            "a circle",
            vec![Step::Circle {
                centre: p(0.0, 0.0),
                radius: 1.0,
            }],
            vec![true; 2],
        ),
        (
            "an arc closed tangent to itself",
            vec![
                Step::At(p(0.0, 0.0)),
                east,
                Step::ArcTo(ArcData::Sweep {
                    r: 1.0,
                    side: ArcSide::Left,
                    angle: core::f64::consts::FRAC_PI_2,
                }),
                Step::Tangent,
                Step::TangentArcTo(Target::StartArriving),
            ],
            vec![true; 2],
        ),
    ];
    // The redraws a corner the replay put on one circle asks for; no
    // row reaches one, so the seed is never read.
    let mut r = test_utils::fuzz::Rng::from_seed(0);
    for (what, steps, want) in rows {
        let forms: Vec<StepForm> = steps.iter().map(Step::form).collect();
        let shape =
            chain_shape(&forms).unwrap_or_else(|e| panic!("{what}: the shape walks: {e:?}"));
        assert_eq!(shape.continues, want, "{what}: the shape's continuations");
        let (lp, record) = replay_recording(&steps, Tol::witness())
            .unwrap_or_else(|e| panic!("{what}: the chain replays: {e:?}"));
        assert_eq!(shape.pieces, record.pieces, "{what}: every piece is drawn");
        for k in 0..want.len() {
            assert!(
                joint_agrees(&mut r, &steps, &shape, &lp, k),
                "{what}: joint {k} is the replay's construction"
            );
        }
    }
}
