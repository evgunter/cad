//! **A wedge flush on a block's top, tilted in band over the two faces,
//! refuses every op at its corner's coplanar lump, on its bounds' margin.**
//!
//! `test_support::wedge_on_a_block_top`: a wedge of opening φ cornered on
//! a block's top, its face in the block's top plane tilted about the
//! wedge's edge by `θ = k·ε`, standing on the block (height 1) or sunk
//! 0.5 into it: at 5° with `k ∈ {±1.2, ±2}`, and at φ ∈ {2°, 5°, 20°,
//! 60°, 90°} with `k = ±0.5`.
//!
//! Read over both faces (the consumed extent, topo README C2), the two
//! carriers stand apart in the sliver band: the block's far corners stand
//! off the wedge's plane by more than θ. So the glue door leaves the pair
//! unglued (D10, Booleans: the sliver band refuses), and every op meets
//! the pair at the wedge's corner, in `vtxfac`'s coplanar lump, where its
//! bounds lie on the block's plane by `θ·sin φ`, Zero. At `|k| ≥ 1.2` the
//! sector's tilt is in band at its reach; at `|k| = 0.5` it is Zero there
//! too and the corner reads the pair one plane, its offset off the
//! block's a rounding residue. Either way it refuses
//! `Coincidence(Sectors)` quoting the bounds' margin, which is the one a
//! smaller tolerance clears; where no tolerance a run admits decides it
//! (at ε 1e-12, `θ·sin φ / K` lies below the floor at `k = ±0.5`), it
//! refuses on the glue door's in-band reading instead; `offer_rows`' `thin_wedge_in_band_over_its_faces`
//! and `flat_wedge_*` rows execute it, and
//! `the_in_band_wedge_is_in_band_at_the_glue_door` shows the glue door's
//! own offer would be false.
//!
//! `wedge_flush_review_probes.rs` holds the ignored battery over other
//! openings, wedge lengths, blocks and tilts.

#![allow(clippy::expect_used, clippy::panic)]

use geom_core::test_support::upper;
use geom_core::{Band, Tol};
use topo::test_support::{face_facing, finished, wedge_on_a_block_top};
use topo::{BooleanDecision, BooleanError, Coincide, DeclarationRead, PlaneEqError, PlaneRung};

#[test]
fn a_wedge_tilted_in_band_over_a_block_top_refuses_every_op_on_its_bounds_margin() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the witness band");
    // (opening, k): the tilt in band at the arm, and inside the zero band
    // there at every opening.
    let tilts = [1.2, -1.2, 2.0, -2.0].map(|k| (5.0, k)).into_iter().chain(
        [2.0, 5.0, 20.0, 60.0, 90.0]
            .into_iter()
            .flat_map(|opening| [(opening, 0.5), (opening, -0.5)]),
    );
    // (pose, height, depth, the wedge face's facing)
    let poses = [("standing", 1.0, 0.0, -1.0), ("sunk", 0.5, 0.5, 1.0)];
    for ((opening, k), (pose, height, depth, facing)) in
        tilts.flat_map(|tilt| poses.map(|pose| (tilt, pose)))
    {
        let phi = f64::to_radians(opening);
        let theta = k * band.zero();
        let (block, tool) = wedge_on_a_block_top(opening, theta, height, depth, tol);
        let (block, tool) = (finished("block", block, tol), finished("tool", tool, tol));
        let label = format!("{pose} at {opening}°, θ = {k}ε");
        let glue = topo::flush_pair_relation(
            &block,
            face_facing(&block, 1.0),
            &tool,
            face_facing(&tool, facing),
            band,
        );
        let door = match glue {
            Ok(Err(PlaneEqError::Escalated {
                rung: PlaneRung::Parallel,
                diag,
            })) => diag,
            other => panic!("{label}: the glue door reads the pair's tilt in band: {other:?}"),
        };
        let bounds = theta.abs() * phi.sin();
        // A tilt Zero at the arm reaches the carrier ladder, which reads
        // the pair one plane: where no tolerance a run admits decides the
        // bounds (their offer at or below the floor), the corner lies on
        // the block's plane at every tolerance, and the refusal names the
        // glue door's reading. A tilt in band refuses on the bounds first.
        let binding = if k.abs() > 1.0 || band.offer_deciding(bounds).is_some() {
            ("bool_sector_coplanar", bounds)
        } else {
            ("bool_plane_parallel", upper(door.margin))
        };
        let ops = [
            ("A ∪ B", topo::union(&block, &tool, tol)),
            ("B ∪ A", topo::union(&tool, &block, tol)),
            ("A ∩ B", topo::intersect(&block, &tool, tol)),
            ("B ∩ A", topo::intersect(&tool, &block, tol)),
            ("A ∖ B", topo::subtract(&block, &tool, tol)),
            ("B ∖ A", topo::subtract(&tool, &block, tol)),
        ];
        for (op, out) in ops {
            // Each admits the class the pair's senses make it
            // (`Settles`); `recl`'s `unglued_tilt`, which quotes the
            // bounds' predicate and margin, admits none.
            let Err(BooleanError::Escalated {
                decision: BooleanDecision::Coincidence(which, DeclarationRead::Settles(_)),
                diag,
            }) = out
            else {
                panic!("{label}, {op}: refuses on a coincidence: {out:?}");
            };
            let (predicate, margin) = binding;
            let want = if predicate == "bool_sector_coplanar" {
                Coincide::Sectors
            } else {
                Coincide::OnPlanes
            };
            assert_eq!(which, want, "{label}, {op}: the binding reading's question");
            assert_eq!(
                diag.predicate,
                Some(predicate),
                "{label}, {op}: its predicate"
            );
            let m = upper(diag.margin).abs();
            assert!(
                (m - margin).abs() <= 1e-3 * margin,
                "{label}, {op}: quotes the binding margin {margin}, got {m}"
            );
        }
    }
}
