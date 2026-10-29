//! **A drum wall pierced by two balls is divided by a carried chord
//! while it holds the other ball's ring**: the two balls (radius 0.25,
//! centred on the wall at `(0.5, 0, 0.4)` and `(−0.5, 0, −0.4)`) are one
//! operand, so the join closes one loop's island on the wall face that
//! already carries the other loop's ring, and re-homes that ring across
//! a run of spline chords cut from the carried section
//! (`topo::chord_join`'s `path_ring_side`, which a cylinder face shares
//! with the sphere, the cone and the torus).
//!
//! Each of the six runs (∪, ∩ and both differences in both operand
//! orders) builds SOUND against the closed-form volume or refuses past
//! the join at its pinned door
//! (`crate::cylinder_sphere_frame::door`): every one reaches the result
//! gate's volume or the spline edge join, the doors the single-ball
//! poses stop at.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use crate::common::differential::outcome;
use crate::common::germ_pair::cyl;
use crate::conic_edge_curved_face::ball;
use crate::cylinder_sphere_frame::{Door, FULL, door, oracle, run};
use geom_core::Tol;
use sweep::test_support::finished;
use topo::BooleanOp;

/// **Every run of the two balls against the drum builds SOUND or
/// refuses at its pinned door past the join**, in the order ∪, ∩, ∖, each
/// drum first and then the balls first.
#[test]
fn a_wall_holding_one_balls_ring_is_divided_by_the_others_chord() {
    use Door::{Join, Sphere};
    let tol = Tol::witness();
    let (big, c) = (0.25, [0.5, 0.0, 0.4]);
    let drum = finished("the drum", cyl(0.5, 1.0), tol);
    let balls = topo::union(&ball(big, c), &ball(big, [-c[0], -c[1], -c[2]]), tol)
        .unwrap()
        .body()
        .unwrap()
        .body
        .clone();
    // The drum is symmetric under the point reflection that swaps the
    // balls, so each cuts it alike.
    let (va, vb, vi) = (
        PI * 0.25 * 2.0,
        2.0 * 4.0 / 3.0 * PI * big.powi(3),
        2.0 * oracle(big, c, FULL),
    );
    let runs = [
        ("A ∪ B", BooleanOp::Union, false, va + vb - vi, Sphere),
        ("B ∪ A", BooleanOp::Union, true, va + vb - vi, Sphere),
        ("A ∩ B", BooleanOp::Intersect, false, vi, Join),
        ("B ∩ A", BooleanOp::Intersect, true, vi, Join),
        ("A ∖ B", BooleanOp::Subtract, false, va - vi, Join),
        ("B ∖ A", BooleanOp::Subtract, true, vb - vi, Join),
    ];
    for (label, op, swapped, want, pinned) in runs {
        let (x, y) = if swapped {
            (&balls, &drum)
        } else {
            (&drum, &balls)
        };
        let got = run(op, x, y);
        if let Err(e) = &got {
            assert_eq!(door(e), Some(pinned), "{label}: refused {e:?}");
        }
        let line = outcome(got, want, tol);
        println!("{label}: {line}");
        assert!(
            line.starts_with("ERR") || line.contains("SOUND") || line == "EMPTY ok",
            "{label}: {line}"
        );
    }
}
