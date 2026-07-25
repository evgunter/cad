//! The project-box enclosure (#91 C3): one part carrying the tour's
//! longest boolean-of-boolean chain — cavity subtract, then 6 vent
//! through-slots (each a two-ring tunnel seam through a wall), then 4
//! interior screw bosses unioned to the floor (inset-overlap, the
//! table-leg pattern), then 4 square pilot pockets into the boss tops:
//! 15 sequential ops, every one against the exact dyadic volume
//! oracle, tier 3′ with declared contacts after every op.
//!
//! Square-only honesty: real enclosures want ROUND bosses and drilled
//! pilot holes — cylindrical boolean operands are M5 (`gate_planar`
//! refuses curved operands today); everything here is square and says
//! so. Coordinates follow the #91 design rule: no two operand planes
//! coincide anywhere in the chain (all features offset in 1/16 steps).
//!
//! Retires the abstract `openbox` stop (this is the cavity story with
//! a real part around it).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use topo::BooleanBody;

use crate::bool_bodies::slab;
use crate::booleans::{check, expect_seamed, try_subtract, try_union};
use crate::{SceneBody, Stop, View};

/// Builds the enclosure; returns the stop and a clone of the final
/// body (the cutaway stop splits it).
pub fn stop() -> (Stop, topo::Body<f64>) {
    // Outer shell 3 x 2 x 1.5, walls/floor 0.25.
    let outer = slab((0.0, 3.0), (0.0, 2.0), (0.0, 1.5));
    let cavity = slab((0.25, 2.75), (0.25, 1.75), (0.25, 2.0));
    let mut vol = 9.0 - 2.5 * 1.5 * 1.25;
    let mut acc: BooleanBody<f64> =
        expect_seamed("cavity subtract", check(try_subtract(&outer, &cavity), vol), vol);
    let mut ops = 1;

    // Vent through-slots: 3 per long wall, cut clean through both wall
    // faces (two-ring tunnel seams); cutters overshoot into air.
    let xs = [(0.5, 0.875), (1.3125, 1.6875), (2.125, 2.5)];
    for &x in &xs {
        for y in [(-0.25, 0.5), (1.5, 2.25)] {
            let cutter = slab(x, y, (0.5, 1.25));
            vol -= 0.375 * 0.25 * 0.75;
            acc = expect_seamed("vent slot", check(try_subtract(&acc.body, &cutter), vol), vol);
            ops += 1;
        }
    }

    // Interior screw bosses: 4, unioned to the floor with a 1/16
    // overlap INTO it (flush contact would refuse — ladder rung (b)).
    let bx = [(0.4375, 0.8125), (2.1875, 2.5625)];
    let by = [(0.4375, 0.8125), (1.1875, 1.5625)];
    for &x in &bx {
        for &y in &by {
            let boss = slab(x, y, (0.1875, 0.875));
            vol += 0.375 * 0.375 * 0.625;
            acc = expect_seamed("boss union", check(try_union(&acc.body, &boss), vol), vol);
            ops += 1;
        }
    }

    // Square pilot pockets, centered in each boss top (round pilot
    // HOLES are the M5 upgrade).
    for &x in &bx {
        for &y in &by {
            let px = (x.0 + 0.09375, x.1 - 0.09375);
            let py = (y.0 + 0.09375, y.1 - 0.09375);
            let pocket = slab(px, py, (0.5625, 1.0625));
            vol -= 0.1875 * 0.1875 * 0.3125;
            acc = expect_seamed("pilot pocket", check(try_subtract(&acc.body, &pocket), vol), vol);
            ops += 1;
        }
    }
    assert_eq!(ops, 15);

    let body_for_cutaway = acc.body.clone();
    let note = format!(
        "15 sequential boolean nodes on ONE part (subtract -> 6 tunnel subtracts -> \
         4 boss unions -> 4 pocket subtracts), exact volume after every op, final \
         V = {vol}; square-only honesty: round bosses/pilot holes are M5 \
         (gate_planar refuses curved operands); no two operand planes coincide \
         anywhere in the chain (the #91 design rule)"
    );
    let s = Stop {
        name: "projectbox",
        caption: "project box".to_string(),
        montage: true,
        story: "electronics enclosure: cavity, 6 vent through-slots, 4 floor bosses, \
                4 pilot pockets — the tour's longest boolean-of-boolean chain",
        ops: "extrude 15 cutters/bosses -> 15 sequential subtract/union nodes",
        delta: 1e-2,
        note: Some(note),
        view: View { elev: 33.0, azim: -125.0, up: 'z' },
        bodies: vec![SceneBody::seamed(
            "projectbox",
            [0.40, 0.60, 0.72],
            acc.body,
            acc.contacts,
        )],
    };
    (s, body_for_cutaway)
}
