//! **An endpoint-free arc leg sweeps definitely less than a full
//! turn.** `Sweep` and `ArcLen` refuse an angle whose arc length short
//! of the full circle, `r(2π − θ)`, is not decided positive
//! (`PathError::ArcSweepNotShortOfFullTurn`, `path_arc_sweep_full`):
//! D1 holds a stored sweep to 0 < |Δθ| ≤ 2π, and a full turn at a chain
//! vertex is a zero chord, which `circle` spells. Rows on both sides of
//! the gate's band, at `f64` and at `Interval`, for both modes: 2π − 0.01
//! and 2π − 20ε/r build, 2π − 2ε/r escalates on the gate's own
//! predicate, and 2π − ε/(2r), 2π and 3π refuse.

use geom_core::{Interval, Point2};
use profile::{ArcData, ArcSide, PathError, ReplayErrorKind, Step, Target};

use crate::common::{tol, try_replay_at};

const R: f64 = 0.5;

/// A tip at the origin heading +x, one leg of `leg`, then a far vertex
/// and back, so a leg that ends near its start still closes a loop.
fn program(leg: ArcData<f64>) -> Vec<Step<f64>> {
    vec![
        Step::At(Point2::new(0.0, 0.0)),
        Step::Toward { dx: 1.0, dy: 0.0 },
        Step::ArcTo(leg),
        Step::LineTo(Target::Point(Point2::new(-5.0 * R, -4.0 * R))),
        Step::LineTo(Target::Start),
    ]
}

fn sweep(angle: f64) -> ArcData<f64> {
    ArcData::Sweep {
        r: R,
        side: ArcSide::Left,
        angle,
    }
}

fn arclen(angle: f64) -> ArcData<f64> {
    ArcData::ArcLen {
        r: R,
        side: ArcSide::Left,
        len: R * angle,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Outcome {
    Builds,
    NotShortOfFullTurn,
    EscalatesAtTheGate,
}

fn outcome<T: profile::ArcCarrierScalar>(leg: ArcData<f64>) -> Result<Outcome, String> {
    match try_replay_at::<T>(&program(leg)) {
        Ok(_) => Ok(Outcome::Builds),
        Err(e) => match e.kind {
            ReplayErrorKind::Path(PathError::ArcSweepNotShortOfFullTurn { .. }) => {
                Ok(Outcome::NotShortOfFullTurn)
            }
            ReplayErrorKind::Path(PathError::Escalated { source })
                if source.predicate == Some("path_arc_sweep_full") =>
            {
                Ok(Outcome::EscalatesAtTheGate)
            }
            other => Err(format!("{other:?}")),
        },
    }
}

#[test]
fn a_leg_short_of_a_full_turn_builds_and_one_at_or_past_it_refuses() {
    let tau = core::f64::consts::TAU;
    let eps = tol().eps();
    for (angle, want) in [
        (tau - 0.01, Outcome::Builds),
        (tau - 20.0 * eps / R, Outcome::Builds),
        (tau - 2.0 * eps / R, Outcome::EscalatesAtTheGate),
        (tau - 0.5 * eps / R, Outcome::NotShortOfFullTurn),
        (tau, Outcome::NotShortOfFullTurn),
        (1.5 * tau, Outcome::NotShortOfFullTurn),
    ] {
        for (mode, leg) in [("Sweep", sweep(angle)), ("ArcLen", arclen(angle))] {
            for (lane, got) in [
                ("f64", outcome::<f64>(leg)),
                ("Interval", outcome::<Interval>(leg)),
            ] {
                assert_eq!(got, Ok(want), "{mode} at {angle} rad on {lane}");
            }
        }
    }
}
