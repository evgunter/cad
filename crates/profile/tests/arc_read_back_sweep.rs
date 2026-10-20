//! **The path door's read-back refuses no sound arc the scene can
//! resolve.** At a point scalar the door reads every constructed arc
//! back against what was authored: its landing on its end vertex, and a
//! `Via` arc's carrier against its through-point. A sound construction
//! misses only by its arithmetic's rounding, which the reads bound at
//! 16 ulps of the read's magnitude (8 for the through-point), so on a
//! scene where that rounding stays under ε the read is Zero and never
//! refuses. A counterexample search over the arc modes, from tiny turns
//! to near-full ones, at magnitudes from centimetres to the scene's
//! resolution and past it: any `ArcMissesAuthoredPoint` is a
//! construction losing digits, and any read-back refusal on a scene
//! that resolves the read is the tripwire firing on a sound arc.
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
    let (a, b, q) = (arc.a(), arc.b(), arc.q());
    let (ux, uy) = arc.tangent(arc.start);
    let (vx, vy) = arc.tangent(arc.start + arc.turn);
    let close = |head: Vec<Step<f64>>| {
        let mut steps = head;
        steps.push(Step::LineTo(Target::Start));
        steps
    };
    // A ray through `a` crossing the carrier at a corner, for the arrival
    // forms: the tangent at `a` turned by a third of a radian.
    let corner = if rng.below(2) == 0 { 0.33 } else { -0.33 };
    let (cs, cc) = corner.sin_cos();
    let (dx, dy) = (ux * cc - uy * cs, ux * cs + uy * cc);
    let lead = arc.radius.min(1.0);
    let entry = Point2::new(a.x - dx * lead, a.y - dy * lead);
    let fillet = 0.05 * lead;
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
                Step::LineTo(Target::Point(outward)),
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
                p.starts_with("path_arc_landing") || p.starts_with("path_arc_through_point")
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
/// ulps stay under ε, is a counterexample. Every draw also counts what
/// the door answered, so the log shows how much of the space built.
#[test]
fn the_read_back_refuses_no_sound_arc_the_scene_resolves() {
    let mut rng = test_utils::fuzz::start("arc_read_back_sweep");
    let eps = tol().get().eps;
    let resolved = eps / (16.0 * ULP);
    let (mut built, mut past, mut other) = (0usize, 0usize, 0usize);
    for _ in 0..test_utils::fuzz::scaled(40) {
        let arc = draw(&mut rng, 4.0 * resolved);
        for (mode, steps) in programs(&arc, &mut rng) {
            let refusal = match try_replay_at::<f64>(&steps) {
                Ok(_) => {
                    built += 1;
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
                None => other += 1,
                Some(true) => panic!("{}: the read-back found a defect: {refusal}", what()),
                Some(false) => {
                    assert!(
                        16.0 * arc.magnitude() * ULP > eps,
                        "{}: a scene that resolves the read (magnitude {:e}) refused: {refusal}",
                        what(),
                        arc.magnitude()
                    );
                    past += 1;
                }
            }
        }
    }
    eprintln!(
        "arc_read_back_sweep: {built} built, {past} refused past the scene's resolution, \
         {other} refused by another gate"
    );
}
