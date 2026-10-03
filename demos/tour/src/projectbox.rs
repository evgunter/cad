//! The project-box enclosure (#91 C3): one part carrying the tour's
//! longest boolean-of-boolean chain — cavity subtract, then 6 vent
//! through-slots (each a two-ring tunnel seam through a wall), then 4
//! round screw bosses standing ON the floor, each union declaring what
//! the flush detector finds ([`crate::booleans::try_union_declared`]):
//! the cap-on-floor contact, plus continuations against the disjoint
//! tops of the bosses already standing, which the union does not need
//! (work/tang/flush-detector-offers-disjoint-coplanar-pairs-as-continuations.md),
//! then a through-bore down each boss and out through the floor: 15
//! sequential ops, every one against the closed-form volume oracle (a
//! volume + Seamed-kind gate per op; tier 3′ with declared contacts
//! runs once, on the FINAL body, in `crate::run_body`).
//!
//! Retires the abstract `openbox` stop (this is the cavity story with
//! a real part around it).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use pncad::document::ExtrudeSide;

use pncad::topo::BooleanBody;

use crate::bool_bodies::slab;
use crate::booleans::{check, expect_seamed, try_subtract, try_union_declared};
use crate::scalar::Scalar;
use crate::{SceneBody, Stop, View};
use pncad::authoring::{p2, p3, validated};
use pncad::geom_core::{OrthoFrame, Tol};
use pncad::profile::SketchPlane;
use pncad::sweep::{Extrusion, extrude};

/// The bosses' radius (m).
pub(crate) const BOSS_R: f64 = 0.1875;

/// The floor's top (m): walls and floor are 0.25 thick.
pub(crate) const FLOOR_TOP: f64 = 0.25;

/// The bosses' span in `z`: standing on the floor, [`FLOOR_TOP`] to
/// the boss tops.
pub(crate) const BOSS_Z: (f64, f64) = (FLOOR_TOP, 0.875);

/// The bores' radius (m).
pub(crate) const BORE_R: f64 = 0.09375;

/// The boss axes, `(x, y)`, each boss's bore on the same axis.
pub(crate) const BOSS_AXES: [(f64, f64); 4] = [
    (0.625, 0.625),
    (0.625, 1.375),
    (2.375, 0.625),
    (2.375, 1.375),
];

/// The height of material a bore removes: the boss's column, from the
/// floor's underside (`z = 0`) to the boss top.
pub(crate) const BORED_HEIGHT: f64 = BOSS_Z.1;

/// A rod of radius `r` on the vertical axis through `(cx, cy)`, from
/// `z.0` to `z.1`.
fn rod<S: Scalar>(cx: f64, cy: f64, r: f64, z: (f64, f64), tol: Tol) -> pncad::topo::Body<S> {
    let circle = pncad::profile::circle(p2(cx, cy), S::from_f64(r), tol)
        .expect("the rod radius is positive")
        .into();
    let plane = SketchPlane::from_frame(OrthoFrame::axes_xy(p3(0.0, 0.0, z.0)));
    let profile = validated(plane, vec![circle], tol).expect("the rod profile validates");
    extrude(
        &profile,
        Extrusion::Distance {
            depth: S::from_f64(z.1 - z.0),
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .expect("extrude the rod")
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

    // Interior screw bosses: 4 cylinders standing on the floor, their
    // flush findings declared.
    for (cx, cy) in BOSS_AXES {
        vol += PI * BOSS_R * BOSS_R * (BOSS_Z.1 - BOSS_Z.0);
        acc = expect_seamed(
            "boss union",
            check(
                try_union_declared(&acc.body, &rod(cx, cy, BOSS_R, BOSS_Z, tol), tol),
                vol,
                tol,
            ),
            vol,
        );
        ops += 1;
    }

    // A through-bore down each boss's axis and out through the floor,
    // overshooting into air at both ends.
    for (cx, cy) in BOSS_AXES {
        vol -= PI * BORE_R * BORE_R * BORED_HEIGHT;
        let bore = rod(cx, cy, BORE_R, (-0.125, BOSS_Z.1 + 0.25), tol);
        acc = expect_seamed(
            "boss bore",
            check(try_subtract(&acc.body, &bore, tol), vol, tol),
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
         4 declared boss unions -> 4 bore subtracts), volume within 1e-9 of the closed-form \
         oracle after every op, final V = {vol} (each boss adds pi R^2 x {boss_h}, \
         R = {BOSS_R}; each bore removes pi r^2 x {BORED_HEIGHT}, r = {BORE_R}); \
         each boss stands on the floor; each union declares the detector's findings, \
         its cap-on-floor contact plus continuations against the other bosses' disjoint \
         tops. \
         SECTIONED: {section_note}",
        boss_h = BOSS_Z.1 - BOSS_Z.0,
    );
    Stop {
        name: "projectbox",
        caption: "project box — whole, and sectioned".to_string(),
        montage: true,
        story: "electronics enclosure: cavity, 6 vent through-slots, 4 round floor \
                bosses, 4 through-bores — the tour's longest boolean-of-boolean chain \
                — and beside it the SAME body split by a tilted plane through two \
                bored bosses and pulled apart, a machinist's section whose boss \
                sections are rings around the bores, showing what the whole one hides",
        ops: "extrude the shell, 7 slab cutters, 4 boss rods, 4 bore rods -> 15 sequential \
              subtract/union nodes, each boss union through flush::find_flush_candidates \
              -> declare_all -> union_with; topo::split(tilted plane) -> 2 bodies -> 2 \
              transform nodes; topo::plane_section(same plane) -> regions with holes",
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

    /// **The bored box and its halves pass their tiers, and the above
    /// half is three outer shells.** Building the stop asserts the
    /// chain's per-op volume oracle and Seamed kind, the halves' section
    /// rings and `plane_section`'s areas against their closed forms; this adds what
    /// `crate::run_body` asserts on the tour pass (tier 3′ on the box
    /// with its declared contacts, tier 3 on each half), and the shell
    /// roles the above half's STEP pin rests on: the cut frees the two
    /// bored bosses' tops, so that half is the walls' piece and two
    /// caps, each bounding material.
    #[test]
    fn the_bored_box_and_its_halves_pass_their_tiers_and_the_above_half_is_three_outer_shells() {
        use pncad::topo::ShellRole;
        let tol = Tol::witness();
        let stop = stop(tol);
        let [boxbody, above, below] = &stop.bodies[..] else {
            panic!("the box and its two halves");
        };
        let contacts = boxbody
            .contacts
            .as_ref()
            .expect("the box is a boolean result");
        pncad::topo::validate_pseudomanifold_certificate(&boxbody.body, contacts, tol)
            .expect("the box: tier 3′ with its declared contacts");
        for half in [above, below] {
            pncad::topo::validate_geometric_certificate(&half.body, tol)
                .unwrap_or_else(|e| panic!("{}: tier 3: {e:?}", half.name));
        }
        let shells: Vec<_> = above.body.shells().map(|(k, _)| k).collect();
        let roles: Vec<ShellRole> = pncad::topo::classify_shells_of(&above.body, &shells, tol)
            .expect("the above half's shells classify")
            .into_iter()
            .map(|c| c.role)
            .collect();
        assert_eq!(roles, [ShellRole::Outer; 3], "the above half's shell roles");
    }
}
