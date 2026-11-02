//! Reviewer rows for PR 6306: `fused_constructions` against `replay`.
//!
//! Two claims. (1) The construction walk reaches exactly the fused
//! steps the replay reaches: those before its first refusal, plus the
//! refused step itself, and none past a close. (2) Each fillet circle
//! it reports is tangent to the carriers it reports for that step, so
//! the drawn carriers are the ones the fit intersected; and a fillet
//! it reports is one of the replayed loop's own arcs.

use geom_core::{Point2, Tol};
use profile::path::program::{Carrier, FusedConstruction, fused_constructions};
use profile::{ArcData, ArcSide, ArcSweep, Segment, Step, Target, TipState, replay};

/// A tiny deterministic generator, so the sweep is reproducible.
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
    fn pick<T: Copy>(&mut self, xs: &[T]) -> T {
        xs[self.below(xs.len())]
    }
    fn coord(&mut self) -> f64 {
        self.pick(&[-8.0, -5.0, -3.0, -1.0, 0.0, 1.0, 2.5, 4.0, 6.0, 10.0])
    }
    fn point(&mut self) -> Point2<f64> {
        Point2::new(self.coord(), self.coord())
    }
    fn radius(&mut self) -> f64 {
        self.pick(&[0.2, 0.5, 1.0, 2.0, 4.0, 15.0, 200.0])
    }
    fn target(&mut self) -> Target<f64> {
        match self.below(4) {
            0 => Target::Start,
            1 => Target::StartArriving,
            _ => Target::Point(self.point()),
        }
    }
    fn side(&mut self) -> ArcSide {
        self.pick(&[ArcSide::Left, ArcSide::Right])
    }
    fn winding(&mut self) -> ArcSweep {
        self.pick(&[ArcSweep::Ccw, ArcSweep::Cw])
    }
    fn spec(&mut self) -> ArcData<f64> {
        match self.below(6) {
            0 | 1 => ArcData::Center {
                c: self.point(),
                winding: self.winding(),
                target: self.target(),
            },
            2 => ArcData::Via {
                q: self.point(),
                target: self.target(),
            },
            3 => ArcData::Bulge {
                target: self.target(),
                b: self.pick(&[-0.8, -0.3, 0.3, 0.6, 1.5]),
            },
            4 => ArcData::Radius {
                r: self.radius(),
                side: self.side(),
            },
            _ => ArcData::Sweep {
                r: self.radius(),
                side: self.side(),
                angle: self.pick(&[0.5, 1.2, 2.5]),
            },
        }
    }
    fn fused(&mut self) -> Step<f64> {
        let radius = self.radius();
        match self.below(3) {
            0 => Step::FilletArc {
                radius,
                spec: self.spec(),
            },
            1 => Step::ArcFillet {
                spec: self.spec(),
                radius,
            },
            _ => Step::ArcFilletArc {
                spec: self.spec(),
                radius,
                spec2: self.spec(),
            },
        }
    }
    fn step(&mut self) -> Step<f64> {
        match self.below(10) {
            0..=3 => self.fused(),
            4 => Step::At(self.point()),
            5 => Step::Angle(self.pick(&[0.0, 0.7, 1.6, 3.0, -2.0])),
            6 => Step::LineTo(self.target()),
            7 => Step::Fillet {
                radius: self.radius(),
            },
            8 => Step::ArcTo(self.spec()),
            _ => Step::Turn(self.pick(&[0.5, -1.0, 1.5])),
        }
    }
}

fn is_fused(step: &Step<f64>) -> bool {
    matches!(
        step,
        Step::FilletArc { .. } | Step::ArcFillet { .. } | Step::ArcFilletArc { .. }
    )
}

/// How far a circle about `c` of radius `r` is from tangent to `carrier`.
fn tangency_gap(c: Point2<f64>, r: f64, carrier: &Carrier<f64>) -> f64 {
    match *carrier {
        Carrier::Circle { centre, radius } => {
            let d = c.distance(centre);
            let ext = (d - (radius + r)).abs();
            let int = (d - (radius - r).abs()).abs();
            ext.min(int)
        }
        Carrier::Ray { origin, dir } => {
            let v = c - origin;
            let along = v.x * dir.x + v.y * dir.y;
            let perp = (v.x * dir.y - v.y * dir.x).abs();
            // The contact is the foot of the perpendicular: it must be
            // on the ray, not behind its origin.
            let behind = (-along).max(0.0);
            (perp - r).abs() + behind
        }
    }
}

fn scale(c: &FusedConstruction<f64>) -> f64 {
    let mut s = c.radius.abs().max(1.0);
    for k in [c.incoming, c.arrival, c.fillet].into_iter().flatten() {
        if let Carrier::Circle { centre, radius } = k {
            s = s.max(radius).max(centre.x.abs()).max(centre.y.abs());
        }
    }
    s
}

thread_local! {
    static RESOLVED: std::cell::RefCell<std::collections::BTreeSet<String>> = Default::default();
}

/// Reports every disagreement on one program.
fn check(steps: &[Step<f64>], tol: Tol, out: &mut Vec<String>, tally: &mut [usize; 6]) {
    let replayed = replay(steps, tol);
    let built = fused_constructions(steps, tol);
    // Where the replay's walk stops: the refused step (inclusive), or
    // the whole program.
    let reach = match &replayed {
        Ok(_) => steps.len(),
        Err(e) => (e.step + 1).min(steps.len()),
    };
    let want: Vec<usize> = (0..reach).filter(|&i| is_fused(&steps[i])).collect();
    let got: Vec<usize> = built.iter().map(|c| c.step).collect();
    let closed_early = matches!(&replayed, Err(e)
        if matches!(e.kind, profile::ReplayErrorKind::Transition { state: TipState::Closed, .. }));
    // Past a close the replay refuses the next step as a transition at
    // `Closed`; the construction walk documents stopping at the close.
    let want: Vec<usize> = if closed_early {
        want.into_iter().filter(|&i| i + 1 < reach).collect()
    } else {
        want
    };
    if want != got {
        // A step past a close is a refusal the replay reports at i+1;
        // the construction walk stops at the close. Record it apart.
        out.push(format!(
            "STEPS{} replay={:?} want={want:?} got={got:?} prog={steps:?}",
            if closed_early { "(after close)" } else { "" },
            replayed.as_ref().err().map(|e| e.step),
        ));
    }
    for c in &built {
        tally[0] += 1;
        if c.incoming.is_some() {
            tally[1] += 1;
        }
        if c.arrival.is_some() {
            tally[2] += 1;
        }
        let refused_here = matches!(&replayed, Err(e) if e.step == c.step);
        if refused_here {
            tally[4] += 1;
            if c.incoming.is_some() {
                tally[5] += 1;
            }
        }
        if refused_here && c.fillet.is_some() {
            out.push(format!("FILLET ON REFUSAL {c:?} prog={steps:?}"));
        }
        let Some(Carrier::Circle { centre, radius }) = c.fillet else {
            if c.fillet.is_some() {
                out.push(format!("FILLET NOT A CIRCLE {c:?}"));
            }
            continue;
        };
        tally[3] += 1;
        let mode = |a: &ArcData<f64>| format!("{:?}", a.mode());
        let key = match steps[c.step] {
            Step::FilletArc { spec, .. } => format!("fillet_arc(ray, {})", mode(&spec)),
            Step::ArcFillet { spec, .. } => format!("arc_fillet({}, line)", mode(&spec)),
            Step::ArcFilletArc { spec, spec2, .. } => {
                format!("arc_fillet_arc({}, {})", mode(&spec), mode(&spec2))
            }
            _ => unreachable!(),
        };
        let at = if c.step == 0 {
            "Entry".to_owned()
        } else {
            format!("after {:?}", steps[c.step - 1].verb())
        };
        RESOLVED.with(|r| r.borrow_mut().insert(format!("{key} {at}")));
        let band = 1e-7 * scale(c);
        if (radius - c.radius.abs()).abs() > band {
            out.push(format!(
                "FILLET RADIUS {radius} vs authored {} {c:?}",
                c.radius
            ));
        }
        for (name, side) in [("incoming", c.incoming), ("arrival", c.arrival)] {
            if let Some(side) = side {
                let gap = tangency_gap(centre, radius, &side);
                if gap > band {
                    out.push(format!(
                        "NOT TANGENT {name} gap={gap:e} {c:?} prog={steps:?}"
                    ));
                }
            }
        }
        if let Ok(lp) = &replayed {
            let on_loop = lp.segments().iter().any(|s| match s {
                Segment::Arc(a) => {
                    a.centre.distance(centre) < band && (a.radius - radius).abs() < band
                }
                Segment::Line => false,
            });
            if !on_loop {
                out.push(format!("FILLET NOT IN LOOP {c:?} prog={steps:?}"));
            }
        }
    }
}

#[test]
fn fused_constructions_agree_with_replay_over_a_random_sweep() {
    let tol = Tol::witness();
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    let states = profile::test_support::every_state();
    let mut out = Vec::new();
    let mut tally = [0usize; 6];
    for _ in 0..60_000 {
        let state = rng.pick(&states);
        let mut steps = if state == TipState::Closed {
            Vec::new()
        } else {
            profile::test_support::prefix(state)
        };
        for _ in 0..(1 + rng.below(3)) {
            steps.push(rng.step());
        }
        if rng.below(2) == 0 {
            steps.push(Step::LineTo(Target::Start));
        }
        check(&steps, tol, &mut out, &mut tally);
    }
    RESOLVED.with(|r| {
        for k in r.borrow().iter() {
            eprintln!("RESOLVED {k}");
        }
    });
    out.sort();
    out.dedup_by(|a, b| a.get(..60) == b.get(..60));
    eprintln!(
        "constructions={} incoming={} arrival={} fillet={} refused_at_fused={} of_which_incoming={}",
        tally[0], tally[1], tally[2], tally[3], tally[4], tally[5]
    );
    for line in out.iter().take(40) {
        eprintln!("{line}");
    }
    assert!(out.is_empty(), "{} disagreements", out.len());
}

/// The vesica: two radius-2 circles about (±1, 0), its top corner
/// rounded by `arc_fillet_arc` from the entry.
fn vesica(radius: f64, w1: ArcSweep, w2: ArcSweep) -> Vec<Step<f64>> {
    let about = |x: f64, to: f64, winding| ArcData::Center {
        c: Point2::new(x, 0.0),
        winding,
        target: Target::Point(Point2::new(to, 0.0)),
    };
    vec![
        Step::ArcFilletArc {
            spec: about(-1.0, 1.0, w1),
            radius,
            spec2: about(1.0, -1.0, w2),
        },
        Step::LineTo(Target::Start),
    ]
}

#[test]
fn fused_constructions_on_the_vesica_print() {
    let tol = Tol::witness();
    for (r, w1, w2) in [
        (0.5, ArcSweep::Ccw, ArcSweep::Ccw),
        (5.0, ArcSweep::Ccw, ArcSweep::Ccw),
        (0.5, ArcSweep::Cw, ArcSweep::Ccw),
        (0.5, ArcSweep::Ccw, ArcSweep::Cw),
    ] {
        let steps = vesica(r, w1, w2);
        let replayed = replay(&steps, tol);
        eprintln!(
            "vesica r={r} {w1:?}/{w2:?}: replay={:?}\n  built={:?}",
            replayed
                .as_ref()
                .map(|_| "ok")
                .map_err(|e| format!("{e:?}")),
            fused_constructions(&steps, tol)
        );
    }
    // A later step completes an arc_fillet: the fillet resolves on the
    // `toward` (or the leg), and the loop closes. What is drawn for it?
    let arc_fillet = Step::ArcFillet {
        spec: ArcData::Center {
            c: Point2::new(0.0, 0.0),
            winding: ArcSweep::Ccw,
            target: Target::Point(Point2::new(5.0, 0.0)),
        },
        radius: 0.5,
    };
    let steps = vec![
        arc_fillet,
        Step::At(Point2::new(0.0, 3.0)),
        Step::Toward { dx: -1.0, dy: 0.0 },
        Step::Line(3.0),
        Step::LineTo(Target::Start),
    ];
    let replayed = replay(&steps, tol);
    let built = fused_constructions(&steps, tol);
    eprintln!(
        "arc_fillet completed later: replay={:?}\n  built={built:?}",
        replayed
            .as_ref()
            .map(|l| l.segments().to_vec())
            .map_err(|e| format!("{e:?}")),
    );
    assert!(replayed.is_ok());
    assert!(
        built[0].fillet.is_none(),
        "an arc_fillet whose fillet a later step resolves PINNED: draws no fillet circle even though the loop holds it: {built:?}"
    );
}

fn with_radius(step: Step<f64>, k: f64, flip: bool) -> Step<f64> {
    let flip_spec = |spec: ArcData<f64>| match spec {
        ArcData::Center { c, winding, target } if flip => ArcData::Center {
            c,
            winding: match winding {
                ArcSweep::Ccw => ArcSweep::Cw,
                ArcSweep::Cw => ArcSweep::Ccw,
            },
            target,
        },
        ArcData::Radius { r, side } if flip => ArcData::Radius {
            r,
            side: match side {
                ArcSide::Left => ArcSide::Right,
                ArcSide::Right => ArcSide::Left,
            },
        },
        other => other,
    };
    match step {
        Step::FilletArc { radius, spec } => Step::FilletArc {
            radius: radius * k,
            spec: flip_spec(spec),
        },
        Step::ArcFillet { spec, radius } => Step::ArcFillet {
            spec: flip_spec(spec),
            radius: radius * k,
        },
        Step::ArcFilletArc {
            spec,
            radius,
            spec2,
        } => Step::ArcFilletArc {
            spec: flip_spec(spec),
            radius: radius * k,
            spec2: flip_spec(spec2),
        },
        other => other,
    }
}

/// The census corpus, each fused step's radius scaled and its sides
/// flipped: every resolved fillet must be tangent to the carriers
/// drawn for it, and a resolved Center arrival / arc incoming must be
/// one of the loop's arcs.
#[test]
fn fused_constructions_agree_with_replay_over_the_corpus_perturbed() {
    let tol = Tol::witness();
    let mut out = Vec::new();
    let mut tally = [0usize; 6];
    let mut programs = 0;
    let mut modes = std::collections::BTreeSet::new();
    for closed in crate::common::coverage_corpus() {
        let steps = closed.program.clone();
        for (i, s) in steps.iter().enumerate() {
            if !is_fused(s) {
                continue;
            }
            for k in [0.1, 0.5, 1.0, 1.7, 3.0, 50.0] {
                for flip in [false, true] {
                    let mut p = steps.clone();
                    p[i] = with_radius(p[i], k, flip);
                    let replayed = replay(&p, tol);
                    if let Ok(lp) = &replayed {
                        for c in fused_constructions(&p, tol) {
                            modes.insert(
                                format!("{:?}", p[c.step])
                                    .split(' ')
                                    .next()
                                    .unwrap()
                                    .to_owned()
                                    + &format!(
                                        " in={} arr={} fil={}",
                                        c.incoming.is_some(),
                                        c.arrival.is_some(),
                                        c.fillet.is_some()
                                    ),
                            );
                            for (name, side) in [("incoming", c.incoming), ("arrival", c.arrival)] {
                                if let Some(Carrier::Circle { centre, radius }) = side {
                                    let band = 1e-7 * scale(&c);
                                    let found = lp.segments().iter().any(|s| matches!(s,
                                        Segment::Arc(a) if a.centre.distance(centre) < band && (a.radius - radius).abs() < band));
                                    if !found {
                                        out.push(format!(
                                            "{name} CARRIER NOT IN LOOP {c:?} prog={p:?}"
                                        ));
                                    }
                                }
                            }
                        }
                    }
                    check(&p, tol, &mut out, &mut tally);
                    programs += 1;
                }
            }
        }
    }
    eprintln!(
        "programs={programs} constructions={} incoming={} arrival={} fillet={} refused_at_fused={} of_which_incoming={}",
        tally[0], tally[1], tally[2], tally[3], tally[4], tally[5]
    );
    for m in &modes {
        eprintln!("MODE {m}");
    }
    for line in out.iter().take(40) {
        eprintln!("{line}");
    }
    assert!(out.is_empty(), "{} disagreements", out.len());
}
