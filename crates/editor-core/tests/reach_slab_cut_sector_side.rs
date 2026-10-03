//! **A planar slab through a cylinder wall answers its closed form.**
//!
//! Two fixtures, each the first move of a common part:
//!
//! - the crenellation a user carving a round tower reaches — a
//!   rectangular slab subtracted through a cylindrical drum;
//! - a round boss on a plate crossed by a slab, unioned in every member
//!   order.
//!
//! In both, the slab's edges pierce the cylinder wall transversally,
//! and each pierce vertex's sector bounds have a definite first-order
//! side; the pierce rings join, and the rows check the volumes. One
//! door remains admitted, in the boss's member orders only: a later
//! member's containment probe reading a boss wall the slab left with a
//! hole (`Containment(WallOutlineUnsupported)`,
//! `work/contact/point-in-solid-refuses-a-ringed-cylinder-wall.md`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use editor_core::ExtrudeSide;

use crate::corpus::body_of;
use crate::docm7_union_declare::{block, failure, run};
use crate::emit_shared_rim_several::permutations;
use crate::fixture::{frame, insert, len};
use editor_core::{
    BooleanOp, Evaluation, LoopProgram, Node, NodeErrorKind, ProfileDoc, ProfileProgram,
    RecipeNodeId,
};
use geom_core::Tol;
use topo::{BooleanError, PointInSolidError};

/// A disc of radius `r` about `(cx, cy)` on the plane `z = z0`,
/// extruded `dz`.
fn disc(doc: ProfileDoc, lp: LoopProgram, z0: f64, dz: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, plane) = insert(doc, frame([0.0, 0.0, z0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
    let (doc, p) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![lp],
            ids: Vec::new(),
        }),
    );
    insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(dz),
            side: ExtrudeSide::Along,
        },
    )
}

/// The area of the band `|t| ≤ a` across a disc of radius `r`.
fn strip(a: f64, r: f64) -> f64 {
    2.0 * (a * (r * r - a * a).sqrt() + r * r * (a / r).asin())
}

/// The node's volume. A refusal fails the row, naming it.
fn volume(ev: &Evaluation<f64>, n: RecipeNodeId, what: &str) -> f64 {
    if let Some(e) = failure(ev, n) {
        panic!("{what}: refused {e:?}");
    }
    topo::mass_properties(body_of(ev, n), Tol::witness())
        .expect("the volume integrates")
        .volume
}

/// The node's volume, or the containment probe's ringed-wall door.
/// Any other refusal fails the row, naming it.
fn volume_or_ringed_wall_door(ev: &Evaluation<f64>, n: RecipeNodeId, what: &str) -> Option<f64> {
    match failure(ev, n) {
        None => Some(volume(ev, n, what)),
        Some(NodeErrorKind::Boolean(BooleanError::Containment(
            PointInSolidError::WallOutlineUnsupported { .. },
        ))) => None,
        Some(other) => panic!("{what}: neither a body nor the ringed wall's door: {other:?}"),
    }
}

/// **The round crenellation.** A drum `r = 13 mm`, `z ∈ [28, 36] mm`,
/// less a slab `40 × 6 mm` from `z = 31 mm` up past its top: the drum
/// less the band `|y| ≤ 3 mm` across its disc, over the top 5 mm.
#[test]
fn a_slab_cut_through_a_drum_answers_its_volume() {
    let (r, z0, h) = (0.013, 0.028, 0.008);
    let (half_t, cut_z) = (0.003, 0.031);
    let doc = ProfileDoc::empty_derived("round_crenellation", Tol::witness());
    let (doc, drum) = disc(doc, LoopProgram::circle(0.0, 0.0, r).unwrap(), z0, h);
    let (doc, cutter) = block(doc, (-0.020, 0.020), (-half_t, half_t), cut_z, 0.008);
    let (doc, n) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a: drum,
            b: cutter,
            declare: Vec::new(),
        },
    );
    let truth = PI * r * r * h - strip(half_t, r) * (z0 + h - cut_z);
    let v = volume(&run(&doc), n, "drum ∖ slab");
    assert!(
        (v - truth).abs() < 1e-9 * truth,
        "drum ∖ slab: {v} vs {truth}"
    );
}

/// **The round boss crossed by a slab, in every member order: four
/// build, two stop typed.** The
/// plate `[0,3] × [0,2] × [0,1]`; the boss `r = 0.6` about `(1.5, 1)`,
/// `z ∈ [0.44, 2.24]`; the slab `x ∈ [1.4, 1.6]`, `y ∈ [−1, 3]`,
/// `z ∈ [0.5, 2]`. The union is the plate, the boss above it, the
/// slab's two ends below the plate's top beyond its `y` sides, and the
/// slab above the plate less the band it shares with the boss. The two
/// orders that union the plate last stop at the ringed-wall door.
#[test]
fn a_slab_across_a_round_boss_builds_in_four_orders_and_stops_typed_in_two() {
    let (r, cx, cy) = (0.6, 1.5, 1.0);
    let doc = ProfileDoc::empty_derived("round_boss_slab", Tol::witness());
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 2.0), 0.0, 1.0);
    let (doc, boss) = disc(
        doc,
        LoopProgram::circle_split(cx, cy, r, 3, 0.0).unwrap(),
        0.44,
        1.8,
    );
    let (doc, slab) = block(doc, (1.4, 1.6), (-1.0, 3.0), 0.5, 1.5);
    let truth = 6.0 + PI * r * r * (2.24 - 1.0) + 0.2 * 2.0 * 0.5 + (0.2 * 4.0 - strip(0.1, r));
    let ids = [plate, boss, slab];
    for order in permutations(&[0, 1, 2]) {
        let (doc, n) = insert(
            doc.clone(),
            Node::Union {
                members: order.iter().map(|&i| ids[i]).collect(),
                declare: Vec::new(),
            },
        );
        let what = format!("order {order:?}");
        // Boss and slab first leave the boss wall ringed for the plate's
        // probe to read; every other order builds.
        let ringed_first = order[2] == 0;
        match volume_or_ringed_wall_door(&run(&doc), n, &what) {
            Some(v) => {
                assert!(!ringed_first, "{what}: built past the ringed-wall door");
                assert!((v - truth).abs() < 1e-9 * truth, "{what}: {v} vs {truth}");
            }
            None => assert!(ringed_first, "{what}: stopped at the ringed-wall door"),
        }
    }
}
