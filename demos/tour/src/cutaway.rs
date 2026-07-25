//! The cutaway section (#91 C4): the tour's first `topo::split` — the
//! project box (itself a 15-op boolean result) split by a TILTED
//! plane, both halves validated independently, then translated apart
//! along the section normal by rigid transforms (re-minted witnesses,
//! #84) and rendered as a machinist's section pair.
//!
//! Replaces the void box's translucency hack as the honest way to
//! show an interior. Feasibility probed 2026-07-25: split of a
//! boolean result passes all tiers on both sides, on main.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point3, Vec3};
use topo::splitting::{SplitPart, SplitPlane, split};

use crate::{SceneBody, Stop, View};

pub fn stops(boxbody: &topo::Body<f64>) -> Vec<Stop> {
    // A tilted section plane through the box interior: normal
    // (0.75, 0.1875, 1) — no axis alignment, crosses walls, bosses,
    // and cavity floor.
    let normal = Vec3::new(0.75, 0.1875, 1.0);
    let plane = SplitPlane {
        origin: Point3::new(1.5, 1.0, 0.75),
        normal,
    };
    let res = split(boxbody, &plane).expect("split of the boolean-result box");
    let (SplitPart::Body(above), SplitPart::Body(below)) = (&res.above, &res.below) else {
        panic!("the section plane crosses the box: both sides must be bodies");
    };

    // Volumes: the two halves partition the box exactly.
    let v_box = topo::mass_properties(boxbody).expect("box props").volume;
    let v_above = topo::mass_properties(above).expect("above props").volume;
    let v_below = topo::mass_properties(below).expect("below props").volume;
    let gap = (v_above + v_below - v_box).abs();
    assert!(gap < 1e-9, "split halves must partition the volume (gap {gap:.3e})");

    // Pull the halves apart along the (unnormalized) section normal:
    // rigid transforms re-mint every moved witness (#84).
    let n = normal * (0.55 / normal.norm());
    let moved_above = topo::transform_rigid(above, &Affine3::translation(n))
        .expect("translate above half");
    let moved_below = topo::transform_rigid(below, &Affine3::translation(-n))
        .expect("translate below half");

    let note = format!(
        "first `topo::split` in the tour, ON a 15-op boolean result; section plane \
         normal (0.75, 0.1875, 1) — tilted, no axis alignment; minted section faces \
         on both sides; halves partition the volume exactly ({v_above:.6} + \
         {v_below:.6} = {v_box:.6}, gap {gap:.1e}); halves then moved apart by \
         rigid transforms (edge witnesses re-minted, #84) and revalidated"
    );
    vec![Stop {
        name: "cutaway",
        caption: "cutaway (split + move)".to_string(),
        montage: true,
        story: "the project box split by a tilted plane and pulled apart — a \
                machinist's section pair showing bosses, pockets, and wall sections",
        ops: "topo::split(projectbox, tilted plane) -> 2 bodies -> 2 transform nodes",
        delta: 1e-2,
        note: Some(note),
        // Split output is not a boolean result: no declared contacts
        // ride it, so both halves go through the plain tier-3 gate.
        view: View { elev: 26.0, azim: -125.0, up: 'z' },
        bodies: vec![
            SceneBody::plain("cutaway_above", [0.40, 0.60, 0.72], moved_above),
            SceneBody::plain("cutaway_below", [0.78, 0.60, 0.35], moved_below),
        ],
    }]
}
