//! **The n-ary door reads a piece of a cut-in sphere face through the
//! face it was cut from**: two pip balls a step crosses off every edge
//! have their sphere faces cut along a meridian before the step re-enters
//! (`topo::BooleanNaming::chart_cuts`), so the accumulation's faces are
//! pieces of the cut faces under keys the cut minted. The fold's member
//! lineage reads each piece as the face it divides (`topo::FaceSource::root`),
//! as the step's holdings do.

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
/// accumulation) is cut in, and the closed-form volume.
type Row = (&'static str, topo::NaryOp, Vec<AtRestBody<f64>>, usize, f64);

#[test]
fn a_fold_over_balls_a_step_cuts_in_builds_at_its_closed_form() {
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
    for (what, op, bodies, cut, want) in rows {
        let run = nary(op, &bodies);
        assert!(
            matches!(run.end, topo::NaryEnd::Built),
            "{what}: the fold builds: {:?}",
            run.end
        );
        assert!(
            !run.steps[cut].body.naming.chart_cuts[0].is_empty(),
            "{what}: the step joining member {} cuts the accumulation's ball in",
            cut + 1
        );
        let last = &run.steps.last().unwrap().body.body;
        let got = topo::test_support::pinned_volume(what, last, tol()).volume;
        assert!(
            (got - want).abs() <= 1e-6 * want,
            "{what}: volume {got}, closed form {want}"
        );
    }
}

/// **A cut piece of the joining member enters the lineage as the face it
/// divides**: a ball, then a member of two balls one of which crosses it,
/// so the step joining that member cuts the member's crossing ball
/// (operand B) in under a key the cut mints, then a ball coincident with
/// that crossing ball. The pairwise judgement's verdict on the coincident
/// pair is routed to the last step through the member faces the
/// accumulation's faces descend from; read through the piece's key alone
/// it reaches no face, the last step meets the coincident sphere ungiven
/// and refuses another way. The coincident shell is a typed refusal of
/// its own.
#[test]
fn a_verdict_on_a_cut_member_face_reaches_the_next_step() {
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
        !run.steps[0].body.naming.chart_cuts[1].is_empty(),
        "the step joining the two-shell member cuts its crossing ball in"
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
