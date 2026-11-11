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
//! orders) builds at the closed-form volume
//! (`crate::cylinder_sphere_frame::at_its_volume`), as the single-ball
//! poses do: no legal operand, since a later boolean's point-in-solid
//! walk cannot read a chorded sphere face
//! (`work/inside/point-in-solid-refuses-a-sphere-face-bounded-by-a-spline-chord.md`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use crate::common::germ_pair::cyl;
use crate::conic_edge_curved_face::ball;
use crate::cylinder_sphere_frame::{Ends, FULL, at_its_volume, oracle, pad_ceiling, run};
use geom_core::Tol;
use sweep::test_support::finished;
use topo::BooleanOp;

/// Regression guards on each run's volume half-width, `(ε, width)`, not
/// soundness bounds: four times the widest measured over the six runs
/// (debug, 2026-11-11), rounded up: `2.943e-7` at ε 1e-6, `1.0315e-10`
/// at 1e-9 and `1.3935e-12` at 1e-12. Any other ε reads the resolution
/// ceiling alone.
const RINGED_PAD_GUARDS: [(f64, f64); 3] = [(1e-6, 1.2e-6), (1e-9, 4.2e-10), (1e-12, 5.6e-12)];

/// **Every run of the two balls against the drum builds at its volume**,
/// in the order ∪, ∩, ∖, each drum first and then the balls first.
#[test]
fn a_wall_holding_one_balls_ring_is_divided_by_the_others_chord() {
    let tol = Tol::witness();
    let (big, c) = (0.25, [0.5, 0.0, 0.4]);
    let (r, h) = (0.5, 1.0);
    let drum = finished("the drum", cyl(r, h), tol);
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
        ("A ∪ B", BooleanOp::Union, false, va + vb - vi),
        ("B ∪ A", BooleanOp::Union, true, va + vb - vi),
        ("A ∩ B", BooleanOp::Intersect, false, vi),
        ("B ∩ A", BooleanOp::Intersect, true, vi),
        ("A ∖ B", BooleanOp::Subtract, false, va - vi),
        ("B ∖ A", BooleanOp::Subtract, true, vb - vi),
    ];
    // The operands' area: the drum's wall (`z ∈ [−h, h]`) and caps, and
    // the two balls' spheres.
    let ceiling =
        pad_ceiling(2.0 * PI * r * 2.0 * h + 2.0 * PI * r * r + 2.0 * 4.0 * PI * big * big);
    let mut off = Vec::new();
    for (label, op, swapped, want) in runs {
        let (x, y) = if swapped {
            (&balls, &drum)
        } else {
            (&drum, &balls)
        };
        off.extend(at_its_volume(
            label,
            run(op, x, y),
            want,
            Ends::Builds,
            ceiling,
            &RINGED_PAD_GUARDS,
        ));
    }
    assert!(off.is_empty(), "{off:#?}");
}
