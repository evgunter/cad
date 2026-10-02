//! The project-box enclosure (#91 C3): one part carrying the tour's
//! longest boolean-of-boolean chain — cavity subtract, then 6 vent
//! through-slots (each a two-ring tunnel seam through a wall), then 4
//! interior screw bosses unioned to the floor (inset-overlap, the
//! table-leg pattern), then a round through-bore down each boss and
//! out through the floor: 15 sequential ops, every one against the
//! closed-form volume oracle (a volume + Seamed-kind gate per op; tier
//! 3′ with declared contacts runs once, on the FINAL body, in
//! `crate::run_body`).
//!
//! The bosses are square: a real enclosure's are round, and this
//! chain does not attempt them. Coordinates follow the #91 design
//! rule: no two operand planes coincide anywhere in the chain (all
//! features offset in 1/16 steps).
//!
//! Retires the abstract `openbox` stop (this is the cavity story with
//! a real part around it).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::topo::BooleanBody;

use crate::bool_bodies::slab;
use crate::booleans::{check, expect_seamed, try_subtract, try_union};
use crate::scalar::Scalar;
use crate::{SceneBody, Stop, View};
use pncad::authoring::{p2, p3, validated};
use pncad::geom_core::{OrthoFrame, Tol};
use pncad::profile::SketchPlane;
use pncad::sweep::{Extrusion, extrude};

/// The boss bores' radius (m).
pub(crate) const BORE_R: f64 = 0.09375;

/// The boss axes, `(x, y)`: the centres of the bosses' squares.
pub(crate) const BORE_AXES: [(f64, f64); 4] = [
    (0.625, 0.625),
    (0.625, 1.375),
    (2.375, 0.625),
    (2.375, 1.375),
];

/// A rod of radius [`BORE_R`] on the vertical axis through `(cx, cy)`,
/// from `z = -0.125` below the floor to `z = 1.125` above the boss tops.
fn bore<S: Scalar>(cx: f64, cy: f64, tol: Tol) -> pncad::topo::Body<S> {
    let circle = pncad::profile::circle(p2(cx, cy), S::from_f64(BORE_R), tol)
        .expect("the bore radius is positive")
        .into();
    let plane = SketchPlane::from_frame(OrthoFrame::axes_xy(p3(0.0, 0.0, -0.125)));
    let profile = validated(plane, vec![circle], tol).expect("the bore profile validates");
    extrude(&profile, Extrusion::Distance(S::from_f64(1.25)), tol)
        .expect("extrude the bore")
        .body
}

/// Builds the 15-op enclosure chain, generic (the Probe sweep runs the
/// same ops); returns the final body and its closed-form volume.
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

    // Interior screw bosses: 4, each 0.375 square about its bore axis,
    // unioned to the floor with a 1/16 overlap INTO it (flush contact
    // would refuse — ladder rung (b)).
    for (cx, cy) in BORE_AXES {
        let boss = slab(
            (cx - 0.1875, cx + 0.1875),
            (cy - 0.1875, cy + 0.1875),
            (0.1875, 0.875),
            tol,
        );
        vol += 0.375 * 0.375 * 0.625;
        acc = expect_seamed(
            "boss union",
            check(try_union(&acc.body, &boss, tol), vol, tol),
            vol,
        );
        ops += 1;
    }

    // A through-bore down each boss's axis and out through the floor:
    // it removes the boss's whole column, floor to boss top.
    for (cx, cy) in BORE_AXES {
        vol -= core::f64::consts::PI * BORE_R * BORE_R * 0.875;
        acc = expect_seamed(
            "boss bore",
            check(try_subtract(&acc.body, &bore(cx, cy, tol), tol), vol, tol),
            vol,
        );
        ops += 1;
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
    let (section_bodies, section_note) = crate::cutaway::sectioned_beside(&acc.body, tol);
    let note = format!(
        "15 sequential boolean nodes on ONE part (subtract -> 6 tunnel subtracts -> \
         4 boss unions -> 4 bore subtracts), volume within 1e-9 of the closed-form \
         oracle after every op, final V = {vol} (the bores remove pi r^2 x 0.875 \
         each, r = {BORE_R}); the bosses are square, a real enclosure's are round; \
         no two operand planes coincide anywhere in the chain (the #91 design \
         rule). SECTIONED: {section_note}"
    );
    Stop {
        name: "projectbox",
        caption: "project box — whole, and sectioned".to_string(),
        montage: true,
        story: "electronics enclosure: cavity, 6 vent through-slots, 4 floor bosses, \
                4 through-bores — the tour's longest boolean-of-boolean chain — and \
                beside it the SAME body split by a tilted plane through two bored \
                bosses and pulled apart, a machinist's section whose boss sections \
                are rings around the bores, showing what the whole one hides",
        ops: "extrude 11 cutters/bosses + 4 bore rods -> 15 sequential subtract/union \
              nodes; topo::split(tilted plane) -> 2 bodies -> 2 transform nodes; \
              topo::plane_section(same plane) -> regions with holes",
        delta: 1e-2,
        note: Some(note),
        // The SECTION's camera, not the box's. A merged cell has one
        // camera, and the two subjects do not want the same one: the
        // box alone was framed at elev 33 / azim -125, which puts the
        // cut faces 2.5 degrees off EDGE-ON (foreshortening 0.044) and
        // makes a machinist's section into a pair of slivers. The
        // cutaway's own choice — "the section normal ~48 degrees off
        // the view direction" (#91 revision note 6) — is the demanding
        // constraint, so it wins: at elev 20 / azim 55 the section
        // normal is 44.8 degrees off the view, foreshortening 0.705,
        // and the below half's cut faces the camera with the cavity,
        // vents and boss sections open to it. The whole box is a box
        // from any azimuth; the section is only a section from this
        // one.
        view: View {
            elev: 20.0,
            azim: 55.0,
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

#[cfg(test)]
mod tests {
    use super::*;

    /// The stop's own build: the chain's per-op volume oracle, the
    /// section's ring census and `plane_section`'s closed-form areas
    /// are all asserted inside it.
    #[test]
    fn the_bored_box_builds_and_its_section_rings_each_half() {
        let stop = stop(Tol::witness());
        assert_eq!(stop.bodies.len(), 3, "the box and its two halves");
    }
}
