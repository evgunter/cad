//! **A crossings-path cut-in is made only where its closing is total.**
//! The output stage closes a sphere cut-in's cut
//! (`topo::boolean::slit`, closing a cut) on two premises: no join chord
//! runs along the cut, and where a cut's piece parts a face bounded by
//! section circles alone, the chart's pole lies beyond one of them. The
//! no-crossings path holds both by construction. On the crossings path
//! (PR 7440's interior-loop cut-in) they are decided before the op
//! re-enters (`interior_cuts::closing_holds`); where one fails the op
//! does not cut in, and the interior-loop guard's refusal stands. It is
//! never a `JoinDesync`, and it is the same in either operand order.
//!
//! Three poses where a premise fails, each in all six ops at the run's
//! ε. In each, the ball's one face carries a first-pass crossing beside
//! the interior loop. The die's slab cuts the `y`-poled ball in the
//! circle `z = 0.7` and, by its bottom, the circle `z = 0` through both
//! poles. Two box unions each hold a box whose plane cuts the ball in an
//! interior circle and a box crossing its face elsewhere. These poses
//! built on the stack before main's cut-in framework carried the closing
//! (`work/lathe/close-cuts-is-not-total-on-crossings-path-cut-ins.md`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use sweep::Revolution;
use sweep::test_support::{brick, finished, revolved_about_y};
use topo::{AtRestBody, BooleanError, BooleanOp, BooleanResult};

fn tol() -> Tol {
    Tol::witness()
}

/// The unit ball revolved about `y`: one face, poles at `(0, ±1, 0)`.
fn ball() -> AtRestBody<f64> {
    let b = revolved_about_y(
        vec![(Point2::new(0.0, -1.0), 1.0), (Point2::new(0.0, 1.0), 0.0)],
        Revolution::Full,
        tol(),
    );
    finished("the ball", b, tol())
}

fn boxed(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> AtRestBody<f64> {
    finished("a box", brick::<f64>(x, y, z, tol()), tol())
}

/// `p ∪ q`, at rest.
fn joined(p: &AtRestBody<f64>, q: &AtRestBody<f64>) -> AtRestBody<f64> {
    match topo::boolean::union(p, q, tol()) {
        Ok(BooleanResult::Body(b)) => finished("the union", b.body.into_body(), tol()),
        other => panic!("the box union builds: {:?}", other.map(|_| ())),
    }
}

/// The poses: the die's slab, and the two box unions.
fn poses() -> Vec<(&'static str, AtRestBody<f64>)> {
    vec![
        (
            "the die's slab",
            boxed((-2.0, 2.0), (-2.0, 2.0), (0.0, 0.7)),
        ),
        (
            "a cap box beside a pole box",
            joined(
                &boxed((-2.0, 2.0), (-2.0, 2.0), (0.6, 2.0)),
                &boxed((-0.2, 0.0), (0.5, 2.0), (0.05, 0.25)),
            ),
        ),
        (
            "a side box and a corner box",
            joined(
                &boxed((-2.0, -0.9), (-1.5, 1.5), (-1.5, 1.5)),
                &boxed((-2.0, -0.2), (-2.0, -0.7), (-2.0, 0.0)),
            ),
        ),
    ]
}

/// The outcome's class: a body (with its volume), the interior-loop
/// guard's refusal, or an escalation. Anything else fails the row.
#[derive(Debug, PartialEq)]
enum Class {
    Body,
    Guard,
    Escalated,
}

fn class(
    label: &str,
    r: Result<BooleanResult<f64>, BooleanError>,
    operands: [&topo::Body<f64>; 2],
) -> (Class, Option<f64>) {
    match r {
        Ok(r @ BooleanResult::Body(_)) => {
            sweep::test_support::cut_trace::assert_no_cut_trace(label, &r, operands);
            let BooleanResult::Body(out) = r else {
                unreachable!("matched a body")
            };
            let v = topo::test_support::pinned_volume(label, &out.body, tol()).volume;
            (Class::Body, Some(v))
        }
        Err(BooleanError::CurvedPairUnsupported {
            site: topo::PairRefusalSite::InteriorLoopGuard,
            ..
        }) => (Class::Guard, None),
        Err(BooleanError::Escalated { .. }) => (Class::Escalated, None),
        other => panic!("{label}: a body, the interior-loop guard or an escalation, not {other:?}"),
    }
}

#[test]
fn a_cut_in_whose_closing_is_not_total_refuses_alike_in_either_order() {
    let ball = ball();
    for (name, other) in poses() {
        for op in [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract] {
            let run = |x: &AtRestBody<f64>, y: &AtRestBody<f64>, order: &str| {
                let label = format!("{name}, {op:?}, {order}, ε {}", tol().eps());
                let r = match op {
                    BooleanOp::Union => topo::boolean::union(x, y, tol()),
                    BooleanOp::Intersect => topo::boolean::intersect(x, y, tol()),
                    BooleanOp::Subtract => topo::boolean::subtract(x, y, tol()),
                };
                let got = class(&label, r, [x, y]);
                println!("[closing] {label}: {got:?}");
                got
            };
            let ab = run(&ball, &other, "ball first");
            let ba = run(&other, &ball, "ball second");
            assert_eq!(
                (ab.0, ba.0),
                (Class::Guard, Class::Guard),
                "{name}, {op:?}: the interior-loop guard's refusal in either order"
            );
        }
    }
}
