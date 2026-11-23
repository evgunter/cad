//! Review probes for PR 7516: the n-ary door over re-charted shells,
//! beyond the PR's three rows. Each row prints its outcome; a panic in
//! the door is the finding.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Tol, Vec3};
use sweep::test_support::{ball_poled_z, block, finished};
use topo::AtRestBody;

fn tol() -> Tol {
    Tol::witness()
}
const R: f64 = 0.09;

fn ball(x: f64, y: f64) -> AtRestBody<f64> {
    finished("ball", ball_poled_z(R, Vec3::new(x, y, 1.0), tol()), tol())
}
fn bv() -> f64 {
    4.0 / 3.0 * std::f64::consts::PI * R.powi(3)
}
fn lens(d: f64) -> f64 {
    std::f64::consts::PI * (4.0 * R + d) * (2.0 * R - d).powi(2) / 12.0
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

fn report(what: &str, op: topo::NaryOp, bodies: Vec<AtRestBody<f64>>, want: Option<f64>) {
    let run = nary(op, &bodies);
    let turned: Vec<_> = run
        .steps
        .iter()
        .map(|s| {
            (
                s.member,
                !s.body.naming.recharted[0].is_empty(),
                !s.body.naming.recharted[1].is_empty(),
            )
        })
        .collect();
    let vol = match run.end {
        topo::NaryEnd::Built => {
            let last = &run.steps.last().unwrap().body.body;
            Some(topo::test_support::pinned_volume(what, last, tol()).volume)
        }
        _ => None,
    };
    eprintln!("PROBE {what}: end {:?}; steps (member, A turned, B turned) {turned:?}; volume {vol:?}, want {want:?}", run.end);
    if let (Some(v), Some(w)) = (vol, want) {
        assert!((v - w).abs() <= 1e-6 * w, "{what}: {v} vs {w}");
    }
}

#[test]
fn probe_chain_of_three() {
    report(
        "chain of three",
        topo::NaryOp::Union,
        vec![ball(0.5, 0.5), ball(0.5, 0.58), ball(0.5, 0.69)],
        Some(3.0 * bv() - lens(0.08) - lens(0.11)),
    );
}

#[test]
fn probe_rechart_twice_same_ball() {
    // member 2 crosses ball 1 (step 1 re-charts it); member 3 crosses ball 1 again from the other side.
    report(
        "ball crossed twice",
        topo::NaryOp::Union,
        vec![ball(0.28, 0.28), ball(0.5, 0.5), ball(0.5, 0.58), ball(0.5, 0.42)],
        None,
    );
}

#[test]
fn probe_apart_then_cross_a_member_side() {
    // the shell that crosses is the joining member's; the accumulation holds two far shells
    report(
        "two far, then a ball crossing the second",
        topo::NaryOp::Union,
        vec![ball(0.28, 0.28), ball(0.5, 0.5), ball(0.72, 0.72), ball(0.72, 0.80)],
        Some(4.0 * bv() - lens(0.08)),
    );
}

#[test]
fn probe_intersect_three() {
    report(
        "intersect three",
        topo::NaryOp::Intersect,
        vec![ball(0.5, 0.5), ball(0.5, 0.58), ball(0.5, 0.54)],
        None,
    );
}

#[test]
fn probe_block_then_ball() {
    let blk = finished("block", block::<f64>(1.0, 1.0, 1.0, tol()), tol());
    report(
        "block then a crossing pair",
        topo::NaryOp::Union,
        vec![blk, ball(0.5, 0.5), ball(0.5, 0.58)],
        None,
    );
}

fn two_shell(a: (f64, f64), b: (f64, f64)) -> AtRestBody<f64> {
    let run = nary(topo::NaryOp::Union, &[ball(a.0, a.1), ball(b.0, b.1)]);
    assert!(matches!(run.end, topo::NaryEnd::Built));
    let body = run.steps.last().unwrap().body.body.clone();
    finished("two shells", body, tol())
}

#[test]
fn probe_b_side_two_shell_member_then_coincident_ball() {
    report(
        "ball, two-shell member crossing it, then a ball coincident with the member's crossing ball",
        topo::NaryOp::Union,
        vec![ball(0.5, 0.5), two_shell((0.9, 0.9), (0.5, 0.58)), ball(0.5, 0.58)],
        Some(3.0 * bv() - lens(0.08)),
    );
}

#[test]
fn probe_b_side_two_shell_member_alone() {
    report(
        "ball, two-shell member crossing it",
        topo::NaryOp::Union,
        vec![ball(0.5, 0.5), two_shell((0.9, 0.9), (0.5, 0.58))],
        Some(3.0 * bv() - lens(0.08)),
    );
}
