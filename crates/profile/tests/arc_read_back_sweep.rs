//! **The path door's read-back refuses no sound arc as a defect, and
//! none the scene can resolve at all.** At a point scalar the door reads
//! every constructed arc back against what was authored: its start on
//! its carrier, its landing on its end vertex, a `Via` arc's carrier
//! against its through-point, and its branch against the turn it was
//! authored to. A sound construction misses only by its arithmetic's
//! rounding, which the reads bound at 16 ulps of the read's magnitude,
//! so on a scene where that rounding stays under ε every read is Zero
//! and none refuses; and past that, a sound arc may refuse as the
//! scene's but never as a kernel defect. A counterexample search over
//! the arc modes, from tiny turns to near-full ones, at magnitudes from
//! centimetres to 1e300 m and fillets down to the coordinates' own ulp:
//! any `ArcMissesAuthoredPoint` is a counterexample, and so is any
//! read-back refusal on a scene that resolves the read. Beside it, the
//! re-review's two shapes that the gate once called defects, pinned.
#![allow(clippy::panic)]

test_utils::gated_to![
    "crates/profile/src/path.rs",
    "crates/profile/src/path/",
    "crates/profile/src/seg.rs",
    "crates/profile/src/sugar.rs",
    "crates/profile/src/lib.rs",
    "crates/profile/tests/common/",
];

use crate::common::{tol, try_replay_at};
use core::f64::consts::TAU;
use geom_core::Point2;
use profile::{ArcData, ArcSide, ArcSweep, PathError, ReplayErrorKind, Step, Target};
use test_utils::fuzz::Rng;

const ULP: f64 = f64::EPSILON;

/// One drawn arc: its circle, its start angle and its signed turn, and
/// the parameter of its through-point along the turn.
struct Drawn {
    centre: Point2<f64>,
    radius: f64,
    start: f64,
    turn: f64,
    through: f64,
}

impl Drawn {
    fn at(&self, angle: f64) -> Point2<f64> {
        let (s, c) = angle.sin_cos();
        Point2::new(
            self.centre.x + self.radius * c,
            self.centre.y + self.radius * s,
        )
    }

    fn a(&self) -> Point2<f64> {
        self.at(self.start)
    }

    fn b(&self) -> Point2<f64> {
        self.at(self.start + self.turn)
    }

    fn q(&self) -> Point2<f64> {
        self.at(self.start + self.through * self.turn)
    }

    /// The unit tangent of travel at angle `angle`.
    fn tangent(&self, angle: f64) -> (f64, f64) {
        let (s, c) = angle.sin_cos();
        let sense = self.turn.signum();
        (-sense * s, sense * c)
    }

    fn side(&self) -> ArcSide {
        if self.turn > 0.0 {
            ArcSide::Left
        } else {
            ArcSide::Right
        }
    }

    /// The magnitude every read of this arc is taken at, or below: its
    /// radius and the reach of its centre and every authored point.
    fn magnitude(&self) -> f64 {
        let reach = |p: Point2<f64>| p.x.abs().max(p.y.abs());
        [self.centre, self.a(), self.b(), self.q()]
            .into_iter()
            .map(reach)
            .fold(self.radius, f64::max)
    }
}

/// A turn: anywhere in the open full turn, near the full turn, or tiny.
fn draw_turn(rng: &mut Rng) -> f64 {
    let sense = if rng.below(2) == 0 { 1.0 } else { -1.0 };
    let magnitude = match rng.below(3) {
        0 => rng.range(1e-2, TAU - 1e-2),
        1 => TAU - rng.log_uniform(1e-7, 1e-1),
        _ => rng.log_uniform(1e-6, 1e-1),
    };
    sense * magnitude
}

fn draw(rng: &mut Rng, top: f64) -> Drawn {
    let radius = rng.log_uniform(1e-2, top);
    let offset = rng.log_uniform(1e-3, top);
    let heading = rng.range(0.0, TAU);
    Drawn {
        centre: Point2::new(offset * heading.cos(), offset * heading.sin()),
        radius,
        start: rng.range(0.0, TAU),
        turn: draw_turn(rng),
        through: rng.range(0.1, 0.9),
    }
}

/// The programs that author `arc`, one per arc mode and door shape.
fn programs(arc: &Drawn, rng: &mut Rng) -> Vec<(&'static str, Vec<Step<f64>>)> {
    // Half the draws fillet at the coordinates' own grain: within three
    // decades either side of an ulp of the arc's magnitude.
    let grain = arc.magnitude() * ULP;
    let fine = (rng.below(2) == 0).then(|| rng.log_uniform(grain * 1e-3, grain * 1e3));
    let (a, b, q) = (arc.a(), arc.b(), arc.q());
    let (ux, uy) = arc.tangent(arc.start);
    let (vx, vy) = arc.tangent(arc.start + arc.turn);
    // Home by way of a vertex a chord's length from the chord's midpoint
    // towards the centre, so the arc's ends turn a clear corner at every
    // turn the draw takes, from tiny to near full.
    let mid = Point2::new(0.5 * (a.x + b.x), 0.5 * (a.y + b.y));
    let inward = arc.centre - mid;
    let reach = (b - a).norm().max(1e-3 * arc.radius) / inward.norm();
    let home = Point2::new(mid.x + inward.x * reach, mid.y + inward.y * reach);
    let close = |head: Vec<Step<f64>>| {
        let mut steps = head;
        steps.push(Step::LineTo(Target::Point(home)));
        steps.push(Step::LineTo(Target::Start));
        steps
    };
    // A ray through `a` crossing the carrier at a corner, for the arrival
    // forms: the tangent at `a` turned by a third of a radian.
    let corner: f64 = if rng.below(2) == 0 { 0.33 } else { -0.33 };
    let (cs, cc) = corner.sin_cos();
    let (dx, dy) = (ux * cc - uy * cs, ux * cs + uy * cc);
    let lead = arc.radius.min(1.0);
    let entry = Point2::new(a.x - dx * lead, a.y - dy * lead);
    let fillet = fine.unwrap_or(0.05 * lead);
    let outward = Point2::new(
        b.x + (b.x - arc.centre.x) * 0.5,
        b.y + (b.y - arc.centre.y) * 0.5,
    );
    let n = 2 + rng.below(5);
    vec![
        (
            "via",
            close(vec![
                Step::At(a),
                Step::ArcTo(ArcData::Via {
                    q,
                    target: Target::Point(b),
                }),
            ]),
        ),
        (
            "center",
            close(vec![
                Step::At(a),
                Step::ArcTo(ArcData::Center {
                    c: arc.centre,
                    winding: if arc.turn > 0.0 {
                        ArcSweep::Ccw
                    } else {
                        ArcSweep::Cw
                    },
                    target: Target::Point(b),
                }),
            ]),
        ),
        (
            "bulge",
            close(vec![
                Step::At(a),
                Step::ArcTo(ArcData::Bulge {
                    target: Target::Point(b),
                    b: (arc.turn / 4.0).tan(),
                }),
            ]),
        ),
        (
            "tangent_arc_to",
            close(vec![
                Step::At(a),
                Step::Toward { dx: ux, dy: uy },
                Step::TangentArcTo(Target::Point(b)),
            ]),
        ),
        (
            "arc_onto",
            close(vec![
                Step::At(b),
                Step::ArcOnto(Target::Point(a)),
                Step::Toward { dx: -ux, dy: -uy },
            ]),
        ),
        (
            "sweep",
            close(vec![
                Step::At(a),
                Step::Toward { dx: ux, dy: uy },
                Step::ArcTo(ArcData::Sweep {
                    r: arc.radius,
                    side: arc.side(),
                    angle: arc.turn.abs(),
                }),
            ]),
        ),
        (
            "arc_len",
            close(vec![
                Step::At(a),
                Step::Toward { dx: ux, dy: uy },
                Step::ArcTo(ArcData::ArcLen {
                    r: arc.radius,
                    side: arc.side(),
                    len: arc.radius * arc.turn.abs(),
                }),
            ]),
        ),
        (
            "via incoming, filleted",
            close(vec![
                Step::At(a),
                Step::ArcFillet {
                    spec: ArcData::Via {
                        q,
                        target: Target::Point(b),
                    },
                    radius: fillet,
                },
                Step::Toward {
                    dx: outward.x - b.x,
                    dy: outward.y - b.y,
                },
                Step::FarEndTo(outward),
            ]),
        ),
        (
            "via arrival, filleted",
            close(vec![
                Step::At(entry),
                Step::Toward { dx, dy },
                Step::FilletArc {
                    radius: fillet,
                    spec: ArcData::Via {
                        q,
                        target: Target::Point(b),
                    },
                },
                Step::Toward { dx: vx, dy: vy },
            ]),
        ),
        (
            "circle",
            vec![Step::Circle {
                centre: arc.centre,
                radius: arc.radius,
            }],
        ),
        (
            "circle_split",
            vec![Step::CircleSplit {
                centre: arc.centre,
                radius: arc.radius,
                n,
                phase: arc.start,
            }],
        ),
    ]
}

/// Whether a refusal is the read-back's, and which: `Some(true)` for the
/// kernel-defect refusal, `Some(false)` for the scene-resolution sibling
/// or an escalation of one of its reads.
fn read_back(kind: &ReplayErrorKind<f64>) -> Option<bool> {
    match kind {
        ReplayErrorKind::Path(PathError::ArcMissesAuthoredPoint { .. }) => Some(true),
        ReplayErrorKind::Path(PathError::ArcMissBelowSceneResolution { .. }) => Some(false),
        ReplayErrorKind::Path(PathError::Escalated { source })
            if source.predicate.is_some_and(|p| {
                [
                    "path_arc_start_on_carrier",
                    "path_arc_landing",
                    "path_arc_through_point",
                    "path_arc_branch",
                ]
                .iter()
                .any(|read| p.starts_with(read))
            }) =>
        {
            Some(false)
        }
        _ => None,
    }
}

/// **No arc mode's sound arc is refused by the read-back where the
/// scene resolves it**: drawn arcs up to four times the magnitude at
/// which a landing read's 16 ulps reach ε, by every mode. A defect
/// refusal anywhere, or any read-back refusal at a magnitude whose 16
/// ulps stay under ε, is a counterexample, and all of them are reported
/// together. Every draw also counts, per mode, what the door answered,
/// so the log shows how much of the space built.
#[test]
fn the_read_back_refuses_no_sound_arc_the_scene_resolves() {
    let mut rng = test_utils::fuzz::start("arc_read_back_sweep");
    let eps = tol().get().eps;
    let resolved = eps / (profile::READ_BACK_LANDING_ULPS * ULP);
    let mut tally: std::collections::BTreeMap<&str, [usize; 3]> = Default::default();
    let mut found: Vec<String> = Vec::new();
    for _ in 0..test_utils::fuzz::scaled(40) {
        // A quarter of the draws run on past the scene's resolution, up
        // to 1e300 m, where a rim read overflows.
        let top = if rng.below(4) == 0 {
            1e300
        } else {
            4.0 * resolved
        };
        let arc = draw(&mut rng, top);
        for (mode, steps) in programs(&arc, &mut rng) {
            let row = tally.entry(mode).or_default();
            let refusal = match try_replay_at::<f64>(&steps) {
                Ok(_) => {
                    row[0] += 1;
                    continue;
                }
                Err(e) => e,
            };
            let what = || {
                format!(
                    "{mode}: centre {:?}, radius {:e}, start {}, turn {:e}, through {}; {}",
                    arc.centre,
                    arc.radius,
                    arc.start,
                    arc.turn,
                    arc.through,
                    test_utils::fuzz::replay()
                )
            };
            match read_back(&refusal.kind) {
                None => row[2] += 1,
                Some(true) => found.push(format!(
                    "{}: the read-back found a defect: {refusal}",
                    what()
                )),
                Some(false) if profile::READ_BACK_LANDING_ULPS * arc.magnitude() * ULP <= eps => {
                    found.push(format!(
                        "{}: a scene that resolves the read (magnitude {:e}) refused: {refusal}",
                        what(),
                        arc.magnitude()
                    ))
                }
                Some(false) => row[1] += 1,
            }
        }
    }
    for (mode, [built, past, other]) in &tally {
        eprintln!(
            "arc_read_back_sweep: {mode}: {built} built, {past} refused past the scene's \
             resolution, {other} refused by another gate"
        );
    }
    assert!(found.is_empty(), "{}", found.join("\n"));
}

/// The fused `Via` incoming with a fillet near or below its coordinates'
/// ulp, on carriers from 1e6 to 1e18 m: the re-review's shapes (R =
/// 7.9e8 m with a 1e-7 m fillet at the default ε; R = 1e13 to 5e16 m
/// with 1e-4 to 0.1 m), which the door once refused as kernel defects
/// on its start read. They build, or refuse as the scene's; never as a
/// defect.
#[test]
fn sub_ulp_fillets_on_far_via_carriers_are_never_defects() {
    let mut defects = Vec::new();
    for k in 0..=24 {
        let r = 10f64.powf(6.0 + 0.5 * f64::from(k));
        for f in [1e-7, 1e-6, 1e-5, 1e-4, 1e-3, 1e-2, 0.1] {
            for (t0, t1) in [(0.4, 1.0), (1.0, 2.5), (2.0, 5.5)] {
                let at = |t: f64| Point2::new(r * t.cos(), r * t.sin());
                let (a, q, b) = (at(0.0), at(t0), at(t1));
                let outward = Point2::new(b.x * 1.5, b.y * 1.5);
                let mid = Point2::new(0.5 * (a.x + b.x), 0.5 * (a.y + b.y));
                let steps = vec![
                    Step::At(a),
                    Step::ArcFillet {
                        spec: ArcData::Via {
                            q,
                            target: Target::Point(b),
                        },
                        radius: f,
                    },
                    Step::Toward {
                        dx: outward.x - b.x,
                        dy: outward.y - b.y,
                    },
                    Step::FarEndTo(outward),
                    Step::LineTo(Target::Point(Point2::new(mid.x * 0.2, mid.y * 0.2))),
                    Step::LineTo(Target::Start),
                ];
                if let Err(e) = try_replay_at::<f64>(&steps)
                    && read_back(&e.kind) == Some(true)
                {
                    defects.push(format!("R {r:e}, fillet {f:e}, turns {t0}/{t1}: {e}"));
                }
            }
        }
    }
    assert!(defects.is_empty(), "{}", defects.join("\n"));
}

/// **A read past 1.3e154 m overflows, and is never a defect**: every
/// mode's arc at magnitudes from 1e150 to 1e300 m, where `‖p − c‖`
/// squares past the range and a miss reads infinite. Each builds or
/// refuses as the scene's.
#[test]
fn reads_past_overflow_are_never_defects() {
    // The seed fixes `programs`' corner, split count and fillet draws,
    // so the rows are the same arcs every run.
    let mut rng = test_utils::fuzz::pinned("arc_read_back_overflow", 0x5449);
    let mut defects = Vec::new();
    for e in [150, 155, 160, 200, 250, 300] {
        let m = 10f64.powi(e);
        for turn in [0.5, 3.0, -5.0, TAU - 1e-3] {
            let arc = Drawn {
                centre: Point2::new(0.3 * m, -0.2 * m),
                radius: 0.4 * m,
                start: 0.7,
                turn,
                through: 0.4,
            };
            for (mode, steps) in programs(&arc, &mut rng) {
                if let Err(err) = try_replay_at::<f64>(&steps)
                    && read_back(&err.kind) == Some(true)
                {
                    defects.push(format!("{mode} at 1e{e} m, turn {turn}: {err}"));
                }
            }
        }
    }
    assert!(defects.is_empty(), "{}", defects.join("\n"));
}
