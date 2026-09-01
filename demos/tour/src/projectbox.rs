//! The project-box enclosure (#91 C3): one part carrying the tour's
//! longest boolean-of-boolean chain — cavity subtract, then 6 vent
//! through-slots (each a two-ring tunnel seam through a wall), then 4
//! interior screw bosses unioned to the floor (inset-overlap, the
//! table-leg pattern), then 4 square pilot pockets into the boss tops:
//! 15 sequential ops, every one against the exact dyadic volume
//! oracle (a volume + Seamed-kind gate per op; tier 3′ with declared
//! contacts runs once, on the FINAL body, in `crate::run_body`).
//!
//! Square-only honesty: real enclosures want ROUND bosses and drilled
//! pilot holes; this chain does not attempt them, and says so. Not
//! because a blanket gate forbids curved operands — the operand gate
//! (`topo`'s `reduce::gate_operand_pairs`) admits `Cylinder` and
//! `Sphere` faces; the curved refusals live per C5 arm, at the sites
//! that exercise one. Everything here is square. Coordinates follow the #91 design rule: no two operand planes
//! coincide anywhere in the chain (all features offset in 1/16 steps).
//!
//! Retires the abstract `openbox` stop (this is the cavity story with
//! a real part around it).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::topo::BooleanBody;

use crate::bool_bodies::slab;
use crate::booleans::{check, expect_seamed, try_subtract, try_union};
use crate::scalar::Scalar;
use crate::{SceneBody, Stop, View};
use pncad::geom_core::Tol;

/// Builds the 15-op enclosure chain, generic (the Probe sweep runs the
/// same ops); returns the final body and its exact volume.
pub(crate) fn build<S: Scalar>(tol: Tol) -> (BooleanBody<S>, f64) {
    // Outer shell 3 x 2 x 1.5, walls/floor 0.25.
    let outer: pncad::topo::Body<S> = slab((0.0, 3.0), (0.0, 2.0), (0.0, 1.5), tol);
    let cavity = slab((0.25, 2.75), (0.25, 1.75), (0.25, 2.0), tol);
    let mut vol = 9.0 - 2.5 * 1.5 * 1.25;
    let mut acc: BooleanBody<S> = expect_seamed(
        "cavity subtract",
        check(try_subtract(&outer, &cavity, tol), vol, tol),
        vol,
    );
    let mut ops = 1;

    // Vent through-slots: 3 per long wall, cut clean through both wall
    // faces (two-ring tunnel seams); cutters overshoot into air.
    let xs = [(0.5, 0.875), (1.3125, 1.6875), (2.125, 2.5)];
    for &x in &xs {
        for y in [(-0.25, 0.5), (1.5, 2.25)] {
            let cutter = slab(x, y, (0.5, 1.25), tol);
            vol -= 0.375 * 0.25 * 0.75;
            acc = expect_seamed(
                "vent slot",
                check(try_subtract(&acc.body, &cutter, tol), vol, tol),
                vol,
            );
            ops += 1;
        }
    }

    // Interior screw bosses: 4, unioned to the floor with a 1/16
    // overlap INTO it (flush contact would refuse — ladder rung (b)).
    let bx = [(0.4375, 0.8125), (2.1875, 2.5625)];
    let by = [(0.4375, 0.8125), (1.1875, 1.5625)];
    for &x in &bx {
        for &y in &by {
            let boss = slab(x, y, (0.1875, 0.875), tol);
            vol += 0.375 * 0.375 * 0.625;
            acc = expect_seamed(
                "boss union",
                check(try_union(&acc.body, &boss, tol), vol, tol),
                vol,
            );
            ops += 1;
        }
    }

    // Square pilot pockets, centered in each boss top (round pilot
    // HOLES are the M5 upgrade).
    for &x in &bx {
        for &y in &by {
            let px = (x.0 + 0.09375, x.1 - 0.09375);
            let py = (y.0 + 0.09375, y.1 - 0.09375);
            let pocket = slab(px, py, (0.5625, 1.0625), tol);
            vol -= 0.1875 * 0.1875 * 0.3125;
            acc = expect_seamed(
                "pilot pocket",
                check(try_subtract(&acc.body, &pocket, tol), vol, tol),
                vol,
            );
            ops += 1;
        }
    }
    assert_eq!(ops, 15);
    (acc, vol)
}

/// The tour stop — the whole enclosure and, beside it, the same body
/// sectioned. ONE cell: the section is not a second part, it is this
/// part with its interior shown, and two independently-scaled panels
/// are the one arrangement that stops a reader laying the halves back
/// onto the whole.
pub fn stop(tol: Tol) -> Stop {
    let (acc, vol) = build::<f64>(tol);
    let body_for_cutaway = acc.body.clone();
    let (section_bodies, section_note) = crate::cutaway::sectioned_beside(&body_for_cutaway, tol);
    let note = format!(
        "15 sequential boolean nodes on ONE part (subtract -> 6 tunnel subtracts -> \
         4 boss unions -> 4 pocket subtracts), volume matching the dyadic oracle \
         after every op (observed bit-exact, gated 1e-9), final V = {vol}; square-only honesty: round bosses/pilot holes are not \
         attempted here (curved operands are gated per C5 arm, not by a blanket \
         operand gate); no two operand planes coincide \
         anywhere in the chain (the #91 design rule). SECTIONED: {section_note}"
    );
    Stop {
        name: "projectbox",
        caption: "project box — whole, and sectioned".to_string(),
        montage: true,
        story: "electronics enclosure: cavity, 6 vent through-slots, 4 floor bosses, \
                4 pilot pockets — the tour's longest boolean-of-boolean chain — and \
                beside it the SAME body split by a tilted plane and pulled apart, a \
                machinist's section showing the bosses, pockets and wall sections the \
                whole one hides",
        ops: "extrude 15 cutters/bosses -> 15 sequential subtract/union nodes; \
              topo::split(tilted plane) -> 2 bodies -> 2 transform nodes",
        delta: 1e-2,
        note: Some(note),
        view: View {
            elev: 33.0,
            azim: -125.0,
            up: 'z',
        },
        bodies: core::iter::once(SceneBody::seamed(
            "projectbox",
            [0.40, 0.60, 0.72],
            acc.body,
            acc.contacts,
        ))
        .chain(section_bodies)
        .collect(),
    }
}
