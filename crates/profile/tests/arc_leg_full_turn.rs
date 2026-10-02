//! **An endpoint-free arc leg sweeps less than a full turn.** `Sweep`
//! and `ArcLen` refuse an angle not definitely short of 2π
//! (`PathError::ArcSweepPastFullTurn`, `path_arc_sweep_full`): D1 holds
//! a stored sweep to 0 < |Δθ| ≤ 2π, and a full turn at a chain vertex
//! is a zero chord, which `circle` spells. Rows at 2π − 0.01 (builds),
//! 2π − ε/(2r) (a margin of ε/2, refuses), 2π and 3π (refuse), at `f64`
//! and at `Interval`, for both modes.

use geom_core::{Interval, Point2};
use profile::{ArcData, ArcSide, PathError, ReplayErrorKind, Step, Target};

use crate::common::{tol, try_replay_at};

const R: f64 = 0.5;

/// A tip at the origin heading +x, one leg of `leg`, closed straight.
fn program(leg: ArcData<f64>) -> Vec<Step<f64>> {
    vec![
        Step::At(Point2::new(0.0, 0.0)),
        Step::Toward { dx: 1.0, dy: 0.0 },
        Step::ArcTo(leg),
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

/// `Some(true)` builds, `Some(false)` refuses past a full turn, `None`
/// refuses for another reason (named in the panic by the caller).
fn outcome<T: profile::ArcCarrierScalar>(leg: ArcData<f64>) -> Result<bool, String> {
    match try_replay_at::<T>(&program(leg)) {
        Ok(_) => Ok(true),
        Err(e) => match e.kind {
            ReplayErrorKind::Path(PathError::ArcSweepPastFullTurn { .. }) => Ok(false),
            other => Err(format!("{other:?}")),
        },
    }
}

#[test]
fn a_leg_short_of_a_full_turn_builds_and_one_at_or_past_it_refuses() {
    let tau = core::f64::consts::TAU;
    let eps = tol().eps();
    for (angle, builds) in [
        (tau - 0.01, true),
        (tau - 0.5 * eps / R, false),
        (tau, false),
        (1.5 * tau, false),
    ] {
        for (mode, leg) in [("Sweep", sweep(angle)), ("ArcLen", arclen(angle))] {
            for (lane, got) in [
                ("f64", outcome::<f64>(leg)),
                ("Interval", outcome::<Interval>(leg)),
            ] {
                assert_eq!(
                    got,
                    Ok(builds),
                    "{mode} at {angle} rad on {lane}: {}",
                    if builds {
                        "builds"
                    } else {
                        "refuses past a full turn"
                    }
                );
            }
        }
    }
}
