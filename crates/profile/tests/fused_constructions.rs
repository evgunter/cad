//! **What a fused fillet step draws is what its fit was handed**
//! ([`replay_drawn`]): the construction a replay records, against the
//! replay itself. Each fillet circle is tangent to the two sides
//! recorded for it, on the side of each anchor the fit reads it from,
//! and is one of the replayed loop's own arcs; a closed loop draws every
//! fused step's construction, its fillet included, whichever step
//! completes the arrival; a refused one draws none past the refusal;
//! and the replay's own result is [`replay`]'s.

#![allow(clippy::panic)]

test_utils::gated_to![
    "crates/profile/src/path.rs",
    "crates/profile/src/path/",
    "crates/profile/src/structure.rs",
    "crates/profile/tests/common/",
];

use geom_core::{Point2, Tol};
use profile::path::program::{Carrier, FusedConstruction, replay_drawn};
use profile::{ArcData, ArcSide, ArcSweep, Segment, Step, Target, TipState, replay};
use test_utils::fuzz::Rng;

fn pick<T: Copy>(rng: &mut Rng, xs: &[T]) -> T {
    xs[rng.below(xs.len())]
}

fn point(rng: &mut Rng) -> Point2<f64> {
    let coord = [-8.0, -5.0, -3.0, -1.0, 0.0, 1.0, 2.5, 4.0, 6.0, 10.0];
    Point2::new(pick(rng, &coord), pick(rng, &coord))
}

fn radius(rng: &mut Rng) -> f64 {
    pick(rng, &[0.2, 0.5, 1.0, 2.0, 4.0, 15.0, 200.0])
}

fn target(rng: &mut Rng) -> Target<f64> {
    match rng.below(4) {
        0 => Target::Start,
        1 => Target::StartArriving,
        _ => Target::Point(point(rng)),
    }
}

fn spec(rng: &mut Rng) -> ArcData<f64> {
    let side = |rng: &mut Rng| pick(rng, &[ArcSide::Left, ArcSide::Right]);
    match rng.below(6) {
        0 | 1 => ArcData::Center {
            c: point(rng),
            winding: pick(rng, &[ArcSweep::Ccw, ArcSweep::Cw]),
            target: target(rng),
        },
        2 => ArcData::Via {
            q: point(rng),
            target: target(rng),
        },
        3 => ArcData::Bulge {
            target: target(rng),
            b: pick(rng, &[-0.8, -0.3, 0.3, 0.6, 1.5]),
        },
        4 => ArcData::Radius {
            r: radius(rng),
            side: side(rng),
        },
        _ => ArcData::Sweep {
            r: radius(rng),
            side: side(rng),
            angle: pick(rng, &[0.5, 1.2, 2.5]),
        },
    }
}

fn step(rng: &mut Rng) -> Step<f64> {
    let r = radius(rng);
    match rng.below(12) {
        0 => Step::FilletArc {
            radius: r,
            spec: spec(rng),
        },
        1 => Step::ArcFillet {
            spec: spec(rng),
            radius: r,
        },
        2 | 3 => Step::ArcFilletArc {
            spec: spec(rng),
            radius: r,
            spec2: spec(rng),
        },
        4 => Step::At(point(rng)),
        5 => Step::Angle(pick(rng, &[0.0, 0.7, 1.6, 3.0, -2.0])),
        6 => Step::LineTo(target(rng)),
        7 => Step::Fillet { radius: r },
        8 => Step::ArcTo(spec(rng)),
        9 => Step::Toward {
            dx: pick(rng, &[-1.0, 0.0, 1.0]),
            dy: pick(rng, &[-1.0, 1.0]),
        },
        10 => Step::Line(r),
        _ => Step::Turn(pick(rng, &[0.5, -1.0, 1.5])),
    }
}

fn is_fused(step: &Step<f64>) -> bool {
    matches!(
        step,
        Step::FilletArc { .. } | Step::ArcFillet { .. } | Step::ArcFilletArc { .. }
    )
}

/// How far a circle about `c` of radius `r` is from tangent to `side`.
/// A line side is touched at the foot of the perpendicular, which must
/// lie ahead of an incoming side's anchor and behind an arrival's.
fn tangency_gap(c: Point2<f64>, r: f64, side: &Carrier<f64>, incoming: bool) -> f64 {
    match *side {
        Carrier::Circle { centre, radius, .. } => {
            let d = c.distance(centre);
            (d - (radius + r)).abs().min((d - (radius - r).abs()).abs())
        }
        Carrier::Line { anchor, dir } => {
            let v = c - anchor;
            let along = v.x * dir.x + v.y * dir.y;
            let perp = (v.x * dir.y - v.y * dir.x).abs();
            let wrong_way = if incoming { -along } else { along };
            (perp - r).abs() + wrong_way.max(0.0)
        }
    }
}

fn scale(c: &FusedConstruction<f64>) -> f64 {
    let mut s = c.radius.abs().max(1.0);
    for k in [Some(c.incoming), Some(c.arrival), c.fillet]
        .into_iter()
        .flatten()
    {
        if let Carrier::Circle { centre, radius, .. } = k {
            s = s.max(radius).max(centre.x.abs()).max(centre.y.abs());
        }
    }
    s
}

fn on_loop(lp: &profile::ProfileLoop<f64>, centre: Point2<f64>, radius: f64, band: f64) -> bool {
    lp.segments().iter().any(|s| match s {
        Segment::Arc(a) => a.centre.distance(centre) < band && (a.radius - radius).abs() < band,
        Segment::Line => false,
    })
}

/// Every disagreement between one program's drawing and its replay,
/// pushed onto `out`. Answers how many fillet circles it checked.
fn check(steps: &[Step<f64>], tol: Tol, out: &mut Vec<String>) -> usize {
    let (drawn_replay, built) = replay_drawn(steps, tol);
    let replayed = replay(steps, tol);
    if format!("{drawn_replay:?}") != format!("{replayed:?}") {
        out.push(format!("REPLAY DIFFERS prog={steps:?}"));
    }
    let reach = match &replayed {
        Ok(_) => steps.len(),
        Err(e) => (e.step + 1).min(steps.len()),
    };
    for c in &built {
        if c.step >= reach || !is_fused(&steps[c.step]) {
            out.push(format!("NOT A REACHED FUSED STEP {c:?} prog={steps:?}"));
        }
    }
    if replayed.is_ok() {
        let mut got: Vec<usize> = built.iter().map(|c| c.step).collect();
        got.sort_unstable();
        let want: Vec<usize> = (0..steps.len()).filter(|&i| is_fused(&steps[i])).collect();
        if got != want {
            out.push(format!(
                "CLOSED LOOP DRAWS want={want:?} got={got:?} prog={steps:?}"
            ));
        }
        for c in built.iter().filter(|c| c.fillet.is_none()) {
            out.push(format!("CLOSED LOOP, NO FILLET {c:?} prog={steps:?}"));
        }
    }
    let mut fillets = 0;
    for c in &built {
        let Some(fillet) = c.fillet else { continue };
        let Carrier::Circle { centre, radius, .. } = fillet else {
            out.push(format!("FILLET NOT A CIRCLE {c:?}"));
            continue;
        };
        fillets += 1;
        let band = 1e-7 * scale(c);
        if (radius - c.radius.abs()).abs() > band {
            out.push(format!("FILLET RADIUS {radius} vs {} {c:?}", c.radius));
        }
        for (name, side, incoming) in [
            ("incoming", c.incoming, true),
            ("arrival", c.arrival, false),
        ] {
            let gap = tangency_gap(centre, radius, &side, incoming);
            if gap > band {
                out.push(format!(
                    "NOT TANGENT {name} gap={gap:e} {c:?} prog={steps:?}"
                ));
            }
        }
        if let Ok(lp) = &replayed
            && !on_loop(lp, centre, radius, band)
        {
            out.push(format!("FILLET NOT IN LOOP {c:?} prog={steps:?}"));
        }
    }
    fillets
}

/// **Counterexample search** over generated chains: each from a prefix
/// that puts the tip in some lattice state, then a few steps, half of
/// them closed straight to the start.
#[test]
fn a_replays_drawing_agrees_with_the_replay_over_generated_chains() {
    let tol = Tol::witness();
    let mut rng = test_utils::fuzz::start("fused_constructions");
    let states = profile::test_support::every_state();
    let mut out = Vec::new();
    let mut fillets = 0;
    for _ in 0..test_utils::fuzz::scaled(6_000) {
        let state = pick(&mut rng, &states);
        let mut steps = if state == TipState::Closed {
            Vec::new()
        } else {
            profile::test_support::prefix(state)
        };
        for _ in 0..=rng.below(3) {
            steps.push(step(&mut rng));
        }
        if rng.below(2) == 0 {
            steps.push(Step::LineTo(Target::Start));
        }
        fillets += check(&steps, tol, &mut out);
    }
    eprintln!("fillet circles checked: {fillets}");
    assert!(
        out.is_empty(),
        "{} disagreements ({}): {:#?}",
        out.len(),
        test_utils::fuzz::replay(),
        &out[..out.len().min(20)]
    );
}

/// The census corpus with each fused step's radius scaled and its
/// sides flipped, which resolves far more fillets than the generator.
#[test]
fn a_replays_drawing_agrees_with_the_replay_over_the_perturbed_corpus() {
    let tol = Tol::witness();
    let flip_spec = |spec: ArcData<f64>| match spec {
        ArcData::Center { c, winding, target } => ArcData::Center {
            c,
            winding: match winding {
                ArcSweep::Ccw => ArcSweep::Cw,
                ArcSweep::Cw => ArcSweep::Ccw,
            },
            target,
        },
        ArcData::Radius { r, side } => ArcData::Radius {
            r,
            side: match side {
                ArcSide::Left => ArcSide::Right,
                ArcSide::Right => ArcSide::Left,
            },
        },
        other => other,
    };
    let perturbed = |step: Step<f64>, k: f64, flip: bool| {
        let f = |spec| if flip { flip_spec(spec) } else { spec };
        match step {
            Step::FilletArc { radius, spec } => Step::FilletArc {
                radius: radius * k,
                spec: f(spec),
            },
            Step::ArcFillet { spec, radius } => Step::ArcFillet {
                spec: f(spec),
                radius: radius * k,
            },
            Step::ArcFilletArc {
                spec,
                radius,
                spec2,
            } => Step::ArcFilletArc {
                spec: f(spec),
                radius: radius * k,
                spec2: f(spec2),
            },
            other => other,
        }
    };
    let mut out = Vec::new();
    let mut fillets = 0;
    for closed in crate::common::coverage_corpus() {
        let steps = closed.program.clone();
        for (i, s) in steps.iter().enumerate() {
            if !is_fused(s) {
                continue;
            }
            for k in [0.1, 0.5, 1.0, 1.7, 3.0, 50.0] {
                for flip in [false, true] {
                    let mut p = steps.clone();
                    p[i] = perturbed(p[i], k, flip);
                    fillets += check(&p, tol, &mut out);
                }
            }
        }
    }
    assert!(fillets >= 40, "the corpus resolves fillets: {fillets}");
    assert!(
        out.is_empty(),
        "{} disagreements: {:#?}",
        out.len(),
        &out[..out.len().min(20)]
    );
}

/// **An `arc_fillet` a later step completes draws its construction**:
/// the fillet resolves on the `toward` that binds the line arrival, two
/// steps after the fused step, and the drawing has it — the incoming
/// circle, the arrival line through the bound point, and the fillet
/// circle the loop holds.
#[test]
fn an_arc_fillet_a_later_step_completes_draws_its_fillet_circle() {
    let tol = Tol::witness();
    let steps = vec![
        Step::ArcFillet {
            spec: ArcData::Center {
                c: Point2::new(0.0, 0.0),
                winding: ArcSweep::Ccw,
                target: Target::Point(Point2::new(5.0, 0.0)),
            },
            radius: 0.5,
        },
        Step::At(Point2::new(0.0, 3.0)),
        Step::Toward { dx: -1.0, dy: 0.0 },
        Step::Line(3.0),
        Step::LineTo(Target::Start),
    ];
    let (replayed, built) = replay_drawn(&steps, tol);
    let Ok(lp) = replayed else {
        panic!("the fixture closes: {replayed:?}")
    };
    let [c] = built.as_slice() else {
        panic!("one fused step, one construction: {built:?}")
    };
    assert_eq!(
        c.step, 0,
        "drawn for the fused step, not the completing one"
    );
    assert!(
        matches!(c.incoming, Carrier::Circle { centre, radius, .. }
            if centre.distance(Point2::new(0.0, 0.0)) < 1e-12 && (radius - 5.0).abs() < 1e-12),
        "the incoming circle: {c:?}"
    );
    assert!(
        matches!(c.arrival, Carrier::Line { anchor, dir }
            if anchor.distance(Point2::new(0.0, 3.0)) < 1e-12 && (dir.x + 1.0).abs() < 1e-12),
        "the arrival line through the bound point: {c:?}"
    );
    let Some(Carrier::Circle { centre, radius, .. }) = c.fillet else {
        panic!("the fillet circle: {c:?}")
    };
    assert!(
        on_loop(&lp, centre, radius, 1e-9),
        "and it is the loop's: {c:?}"
    );
}
