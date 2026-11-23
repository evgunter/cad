//! **The n-ary door reads a face of a re-charted sphere shell through
//! that shell's input faces**: two pip balls a step crosses along their
//! poles' normal plane are turned about their centres before the step
//! runs (`topo::BooleanNaming::recharted`), so the accumulation's turned
//! faces carry keys the re-chart minted. The fold's member lineage reads
//! each such face as every face of the input shell it was turned over
//! (`topo::FaceSource::inputs`), as the step's holdings do.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Tol, Vec3};
use sweep::test_support::{ball_poled_z, finished};
use topo::AtRestBody;

fn tol() -> Tol {
    Tol::witness()
}

const R: f64 = 0.09;
/// The overlapping pair's centres stand this far apart along `y`.
const D: f64 = 0.08;

fn ball(x: f64, y: f64) -> AtRestBody<f64> {
    finished(
        "a pip ball",
        ball_poled_z(R, Vec3::new(x, y, 1.0), tol()),
        tol(),
    )
}

fn ball_volume() -> f64 {
    4.0 / 3.0 * std::f64::consts::PI * R.powi(3)
}

/// The lens two balls of radius `R` whose centres stand `D` apart share.
fn lens_volume() -> f64 {
    std::f64::consts::PI * (4.0 * R + D) * (2.0 * R - D).powi(2) / 12.0
}

fn nary(op: topo::NaryOp, bodies: &[AtRestBody<f64>]) -> topo::NaryRun<f64> {
    let inputs: Vec<topo::MemberInputs> = (0..bodies.len())
        .map(|rank| topo::MemberInputs {
            rank: u32::try_from(rank).unwrap(),
            ..topo::MemberInputs::default()
        })
        .collect();
    let members: Vec<topo::NaryMember<'_, f64>> = bodies
        .iter()
        .zip(&inputs)
        .map(|(body, inputs)| topo::NaryMember { body, inputs })
        .collect();
    topo::nary_boolean(op, &members, topo::SweepStrategy::Realized, tol())
}

/// A row: its name, the op, its members, the step whose operand A (the
/// accumulation) the re-chart turns, and the closed-form volume.
type Row = (&'static str, topo::NaryOp, Vec<AtRestBody<f64>>, usize, f64);

#[test]
fn a_fold_over_balls_a_step_re_charts_builds_at_its_closed_form() {
    let (b, lens) = (ball_volume(), lens_volume());
    let pair = || vec![ball(0.5, 0.5), ball(0.5, 0.5 + D)];
    let rows: [Row; 3] = [
        (
            "the pair's union",
            topo::NaryOp::Union,
            pair(),
            0,
            2.0 * b - lens,
        ),
        (
            "the pair's intersect",
            topo::NaryOp::Intersect,
            pair(),
            0,
            lens,
        ),
        (
            "two apart, then the pair",
            topo::NaryOp::Union,
            vec![
                ball(0.28, 0.28),
                ball(0.72, 0.72),
                ball(0.5, 0.5),
                ball(0.5, 0.5 + D),
            ],
            2,
            4.0 * b - lens,
        ),
    ];
    for (what, op, bodies, turned, want) in rows {
        let run = nary(op, &bodies);
        assert!(
            matches!(run.end, topo::NaryEnd::Built),
            "{what}: the fold builds: {:?}",
            run.end
        );
        assert!(
            !run.steps[turned].body.naming.recharted[0].is_empty(),
            "{what}: the step joining member {} turns the accumulation's shell",
            turned + 1
        );
        let last = &run.steps.last().unwrap().body.body;
        let got = topo::test_support::pinned_volume(what, last, tol()).volume;
        assert!(
            (got - want).abs() <= 1e-6 * want,
            "{what}: volume {got}, closed form {want}"
        );
    }
}

/// **A turned face of the joining member enters the lineage as its input
/// shell's faces**: a ball, then a member of two balls one of which
/// crosses it, so the step joining that member turns the member's
/// crossing ball (operand B) under a key the graft mints, then a ball
/// coincident with that crossing ball. The pairwise judgement's verdict
/// on the coincident pair is routed to the last step through the member
/// faces the accumulation's faces descend from; read through the turned
/// key alone it reaches no face, the last step meets the coincident
/// sphere ungiven and refuses another way. The coincident shell is a
/// typed refusal of its own.
#[test]
fn a_verdict_on_a_turned_member_face_reaches_the_next_step() {
    let pair = nary(topo::NaryOp::Union, &[ball(0.9, 0.9), ball(0.5, 0.5 + D)]);
    assert!(
        matches!(pair.end, topo::NaryEnd::Built),
        "two apart: the pair builds: {:?}",
        pair.end
    );
    let two_shells = finished(
        "two shells",
        pair.steps.last().unwrap().body.body.clone(),
        tol(),
    );
    let run = nary(
        topo::NaryOp::Union,
        &[ball(0.5, 0.5), two_shells, ball(0.5, 0.5 + D)],
    );
    assert!(
        !run.steps[0].body.naming.recharted[1].is_empty(),
        "the step joining the two-shell member turns its crossing ball"
    );
    assert!(
        matches!(
            &run.end,
            topo::NaryEnd::Refused(topo::NaryRefusal::Step { member: 2, error })
                if matches!(**error, topo::BooleanError::CoincidentShell { .. })
        ),
        "the step joining the coincident ball refuses the coincident shell, given the \
         judgement's verdict: {:?}",
        run.end
    );
}
