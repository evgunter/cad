//! **What a fillet step draws is what its fit was handed**
//! ([`replay_drawn`]): the construction a replay records, against the
//! replay itself. Each fillet circle is tangent to the two sides
//! recorded for it, on the side of each anchor the fit reads it from,
//! and is one of the replayed loop's own arcs; a closed loop draws every
//! fillet step's construction, its fillet included, whichever step
//! completes the arrival; a refused one draws every fillet step the
//! replay opened and none past the refusal, and an anchor-fit refusal
//! draws the candidate circle each of its entries reports; and the
//! replay's own result is [`replay`]'s.

#![allow(clippy::panic)]

test_utils::gated_to![
    "crates/profile/src/path.rs",
    "crates/profile/src/path/",
    "crates/profile/src/structure.rs",
    "crates/profile/tests/common/",
];

use geom_core::{Point2, Tol};
use profile::path::program::{Carrier, FilletConstruction, FilletFit, replay_drawn};
use profile::{
    ArcData, ArcSide, ArcSweep, CornerReason, PathError, ReplayErrorKind, Segment, Step, Target,
    TipState, replay,
};
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

fn is_fillet(step: &Step<f64>) -> bool {
    matches!(
        step,
        Step::Fillet { .. }
            | Step::FilletArc { .. }
            | Step::ArcFillet { .. }
            | Step::ArcFilletArc { .. }
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

/// The circles a record draws: its fillet, or each overrunning
/// candidate.
fn circles(c: &FilletConstruction<f64>) -> Vec<Carrier<f64>> {
    match &c.fit {
        Some(FilletFit::Fillet(fillet)) => vec![*fillet],
        Some(FilletFit::Overrun(overruns)) => overruns.iter().map(|o| o.circle).collect(),
        None => Vec::new(),
    }
}

fn scale(c: &FilletConstruction<f64>) -> f64 {
    let mut s = c.radius.abs().max(1.0);
    for k in [Some(c.incoming), c.arrival]
        .into_iter()
        .flatten()
        .chain(circles(c))
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
/// pushed onto `out`. Answers how many fillet circles, and how many
/// overrunning ones, it checked.
fn check(steps: &[Step<f64>], tol: Tol, out: &mut Vec<String>) -> (usize, usize) {
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
        if c.step >= reach || !is_fillet(&steps[c.step]) {
            out.push(format!("NOT A REACHED FILLET STEP {c:?} prog={steps:?}"));
        }
    }
    // Every fillet step before the one refused completed, so it opened
    // its incoming side and has a record; a closed loop has one for
    // every fillet step.
    let opened = match &replayed {
        Ok(_) => steps.len(),
        Err(e) => e.step,
    };
    let mut got: Vec<usize> = built
        .iter()
        .map(|c| c.step)
        .filter(|&s| s < opened)
        .collect();
    got.sort_unstable();
    let want: Vec<usize> = (0..opened).filter(|&i| is_fillet(&steps[i])).collect();
    if got != want {
        out.push(format!(
            "OPENED STEPS DRAW want={want:?} got={got:?} prog={steps:?}"
        ));
    }
    if replayed.is_ok() {
        for c in &built {
            if c.arrival.is_none() || !matches!(c.fit, Some(FilletFit::Fillet(_))) {
                out.push(format!("CLOSED LOOP, NOT WHOLE {c:?} prog={steps:?}"));
            }
        }
    }
    let mut circles = (0, 0);
    for c in &built {
        let band = 1e-7 * scale(c);
        match &c.fit {
            None => {}
            Some(FilletFit::Fillet(fillet)) => {
                let Carrier::Circle { centre, radius, .. } = *fillet else {
                    out.push(format!("FILLET NOT A CIRCLE {c:?}"));
                    continue;
                };
                circles.0 += 1;
                if (radius - c.radius.abs()).abs() > band {
                    out.push(format!("FILLET RADIUS {radius} vs {} {c:?}", c.radius));
                }
                let Some(arrival) = c.arrival else {
                    out.push(format!("FILLET WITH NO ARRIVAL {c:?} prog={steps:?}"));
                    continue;
                };
                for (name, side, incoming) in
                    [("incoming", c.incoming, true), ("arrival", arrival, false)]
                {
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
            Some(FilletFit::Overrun(overruns)) => {
                circles.1 += overruns.len();
                check_overruns(steps, c, overruns, &replayed, band, out);
            }
        }
    }
    circles
}

/// An overrun record against the refusal it stands beside: the replay
/// refused on the anchor fit, with an entry at each recorded corner,
/// and each candidate circle has the record's radius and touches both
/// recorded sides at its tangent points (which lie past an anchor, so
/// no side of the anchor is asked of them).
fn check_overruns(
    steps: &[Step<f64>],
    c: &FilletConstruction<f64>,
    overruns: &[profile::path::program::Overrun<f64>],
    replayed: &Result<profile::ConstructedLoop<f64>, profile::ReplayError<f64>>,
    band: f64,
    out: &mut Vec<String>,
) {
    let entries: Vec<Point2<f64>> = match replayed {
        Err(profile::ReplayError {
            kind: ReplayErrorKind::Path(PathError::NoCornerOfPair { corners, .. }),
            ..
        }) => corners
            .iter()
            .filter(|e| matches!(e.reason, CornerReason::AnchorOutsideTrimmedExtent { .. }))
            .map(|e| e.at)
            .collect(),
        _ => Vec::new(),
    };
    let Some(arrival) = c.arrival else {
        out.push(format!("OVERRUN WITH NO ARRIVAL {c:?} prog={steps:?}"));
        return;
    };
    for o in overruns {
        if !entries.iter().any(|at| at.distance(o.corner) < band) {
            out.push(format!(
                "OVERRUN NOT A REFUSED CORNER {o:?} {replayed:?} prog={steps:?}"
            ));
        }
        let Carrier::Circle { centre, radius, .. } = o.circle else {
            out.push(format!("OVERRUN NOT A CIRCLE {o:?}"));
            continue;
        };
        if (radius - c.radius.abs()).abs() > band {
            out.push(format!("OVERRUN RADIUS {radius} vs {} {c:?}", c.radius));
        }
        for (side, touch) in [(c.incoming, o.tangents[0]), (arrival, o.tangents[1])] {
            let on_side = match side {
                Carrier::Circle {
                    centre: sc,
                    radius: sr,
                    ..
                } => (touch.distance(sc) - sr).abs(),
                Carrier::Line { anchor, dir } => {
                    let v = touch - anchor;
                    (v.x * dir.y - v.y * dir.x).abs()
                }
            };
            let on_circle = (touch.distance(centre) - radius).abs();
            if on_side > band || on_circle > band {
                out.push(format!(
                    "OVERRUN TANGENT OFF ITS CARRIERS side={on_side:e} circle={on_circle:e} \
                     {o:?} prog={steps:?}"
                ));
            }
        }
    }
}

/// **Counterexample search** over generated chains: each from a prefix
/// that puts the tip in some lattice state, then a few steps, half of
/// them closed straight to the start.
#[test]
fn a_replays_drawing_agrees_with_the_replay_over_generated_chains() {
    let tol = Tol::witness();
    let mut rng = test_utils::fuzz::start("fillet_constructions");
    let states = profile::test_support::every_state();
    let mut out = Vec::new();
    let (mut fillets, mut overruns) = (0, 0);
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
        let (fillet, overrun) = check(&steps, tol, &mut out);
        fillets += fillet;
        overruns += overrun;
    }
    eprintln!("fillet circles checked: {fillets}, overrunning ones: {overruns}");
    assert!(
        out.is_empty(),
        "{} disagreements ({}): {:#?}",
        out.len(),
        test_utils::fuzz::replay(),
        &out[..out.len().min(20)]
    );
}

/// The census corpus with each fillet step's radius scaled and its
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
            Step::Fillet { radius } => Step::Fillet { radius: radius * k },
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
    let (mut fillets, mut overruns) = (0, 0);
    for closed in crate::common::coverage_corpus() {
        let steps = closed.program.clone();
        for (i, s) in steps.iter().enumerate() {
            if !is_fillet(s) {
                continue;
            }
            for k in [0.1, 0.5, 1.0, 1.7, 3.0, 50.0] {
                for flip in [false, true] {
                    let mut p = steps.clone();
                    p[i] = perturbed(p[i], k, flip);
                    let (fillet, overrun) = check(&p, tol, &mut out);
                    fillets += fillet;
                    overruns += overrun;
                }
            }
        }
    }
    assert!(fillets >= 40, "the corpus resolves fillets: {fillets}");
    assert!(
        overruns >= 10,
        "the corpus refuses fillets on the anchor fit: {overruns}"
    );
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
        panic!("one fillet step, one construction: {built:?}")
    };
    assert_eq!(
        c.step, 0,
        "drawn for the fillet step, not the completing one"
    );
    assert!(
        matches!(c.incoming, Carrier::Circle { centre, radius, .. }
            if centre.distance(Point2::new(0.0, 0.0)) < 1e-12 && (radius - 5.0).abs() < 1e-12),
        "the incoming circle: {c:?}"
    );
    assert!(
        matches!(c.arrival, Some(Carrier::Line { anchor, dir })
            if anchor.distance(Point2::new(0.0, 3.0)) < 1e-12 && (dir.x + 1.0).abs() < 1e-12),
        "the arrival line through the bound point: {c:?}"
    );
    let Some(FilletFit::Fillet(Carrier::Circle { centre, radius, .. })) = c.fit else {
        panic!("the fillet circle: {c:?}")
    };
    assert!(
        on_loop(&lp, centre, radius, 1e-9),
        "and it is the loop's: {c:?}"
    );
}

/// The ray-east, fillet, arrive-north program of radius `r`, closed:
/// the corner is `(3, 0)` and the arrival anchor `(3, 2)`.
fn bare_fillet(r: f64) -> Vec<Step<f64>> {
    vec![
        Step::At(Point2::new(0.0, 0.0)),
        Step::Toward { dx: 1.0, dy: 0.0 },
        Step::Fillet { radius: r },
        Step::Toward { dx: 0.0, dy: 1.0 },
        Step::FarEndTo(Point2::new(3.0, 2.0)),
        Step::LineTo(Target::Start),
    ]
}

fn near(a: Point2<f64>, x: f64, y: f64) -> bool {
    a.distance(Point2::new(x, y)) < 1e-12
}

/// **The bare `fillet(r)` draws its construction**: both lines, and
/// the fillet circle the loop holds.
///
/// Red if a line fillet's resolution records nothing.
#[test]
fn a_bare_fillet_draws_its_lines_and_its_fillet_circle() {
    let tol = Tol::witness();
    let (replayed, built) = replay_drawn(&bare_fillet(0.5), tol);
    let Ok(lp) = replayed else {
        panic!("the fixture closes: {replayed:?}")
    };
    let [c] = built.as_slice() else {
        panic!("one fillet step, one construction: {built:?}")
    };
    assert_eq!(c.step, 2, "filed against the fillet step");
    assert!(
        matches!(c.incoming, Carrier::Line { anchor, dir } if near(anchor, 0.0, 0.0) && dir.x == 1.0),
        "the incoming ray: {c:?}"
    );
    assert!(
        matches!(c.arrival, Some(Carrier::Line { anchor, dir }) if near(anchor, 3.0, 2.0) && dir.y == 1.0),
        "the arrival line through its anchor: {c:?}"
    );
    let Some(FilletFit::Fillet(Carrier::Circle { centre, radius, .. })) = c.fit else {
        panic!("the fillet circle: {c:?}")
    };
    assert!(near(centre, 2.5, 0.5) && radius == 0.5, "{c:?}");
    assert!(
        on_loop(&lp, centre, radius, 1e-9),
        "and it is the loop's: {c:?}"
    );
}

/// **An anchor-fit refusal draws the candidate that overran**, filed
/// against the fillet step while the refusal names the binder that
/// fixed the arrival: at r = 2.5 the setback (2.5 on a right angle)
/// outruns the arrival's 2 m, the trim's tangent points are (0.5, 0)
/// and (3, 2.5), and the circle is about (0.5, 2.5).
///
/// Red if a refused line fillet's fit records no answer, or records it
/// against the refusing step.
#[test]
fn an_anchor_fit_refusal_draws_its_overrunning_circle() {
    let tol = Tol::witness();
    let (replayed, built) = replay_drawn(&bare_fillet(2.5), tol);
    let Err(refused) = replayed else {
        panic!("the arrival leg is too short: {replayed:?}")
    };
    assert_eq!(refused.step, 4, "refused at the arrival's far-end binder");
    let [c] = built.as_slice() else {
        panic!("one fillet step, one construction: {built:?}")
    };
    assert_eq!(c.step, 2, "filed against the fillet step");
    assert!(
        c.arrival.is_some(),
        "the arrival was bound before the fit: {c:?}"
    );
    let Some(FilletFit::Overrun(overruns)) = &c.fit else {
        panic!("the overrun: {c:?}")
    };
    let [o] = overruns.as_slice() else {
        panic!("one corner: {overruns:?}")
    };
    assert!(near(o.corner, 3.0, 0.0), "{o:?}");
    assert!(
        near(o.tangents[0], 0.5, 0.0) && near(o.tangents[1], 3.0, 2.5),
        "{o:?}"
    );
    assert!(
        matches!(o.circle, Carrier::Circle { centre, radius, .. } if near(centre, 0.5, 2.5) && radius == 2.5),
        "{o:?}"
    );
}

/// **A step refused between its two sides still draws the first**: the
/// arrival's director has no direction, so the replay refuses there,
/// and the fillet's incoming ray is on record with no arrival and no
/// fit.
///
/// Red if the incoming side is recorded only when the fit is handed
/// both.
#[test]
fn a_fillet_refused_before_its_arrival_draws_its_incoming_side() {
    let tol = Tol::witness();
    let mut steps = bare_fillet(0.5);
    steps[3] = Step::Toward { dx: 0.0, dy: 0.0 };
    let (replayed, built) = replay_drawn(&steps, tol);
    let Err(refused) = replayed else {
        panic!("a zero director refuses: {replayed:?}")
    };
    assert_eq!(refused.step, 3);
    let [c] = built.as_slice() else {
        panic!("one fillet step, one construction: {built:?}")
    };
    assert!(
        c.step == 2
            && matches!(c.incoming, Carrier::Line { anchor, .. } if near(anchor, 0.0, 0.0))
            && c.arrival.is_none()
            && c.fit.is_none(),
        "{c:?}"
    );
}
