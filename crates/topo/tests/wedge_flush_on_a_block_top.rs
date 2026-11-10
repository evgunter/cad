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
//! bounds lie on the block's plane by `θ·sin φ`, Zero. Carrier identity is
//! the glue door's alone, so the corner refuses on its own reading:
//! `Coincidence(CornerBound)`, the steeper bound's first-order tie, which
//! a smaller tolerance reads off the plane. Where no tolerance a run
//! admits decides it (at ε 1e-12, `θ·sin φ / K` lies below the floor at
//! `k = ±0.5`), it says so; the glue door's in-band reading, which binds
//! nothing, is not quoted. `offer_rows`' `thin_wedge_in_band_over_its_faces`
//! and `flat_wedge_*` rows execute the offer.
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
        // The door reads the pair in band and glues nothing; the corner
        // refuses on its own reading, never on the door's.
        assert!(
            matches!(
                glue,
                Ok(Err(PlaneEqError::Escalated {
                    rung: PlaneRung::Parallel,
                    ..
                }))
            ),
            "{label}: the glue door reads the pair's tilt in band: {glue:?}"
        );
        let bounds = theta.abs() * phi.sin();
        let ops = [
            ("A ∪ B", topo::union(&block, &tool, tol)),
            ("B ∪ A", topo::union(&tool, &block, tol)),
            ("A ∩ B", topo::intersect(&block, &tool, tol)),
            ("B ∩ A", topo::intersect(&tool, &block, tol)),
            ("A ∖ B", topo::subtract(&block, &tool, tol)),
            ("B ∖ A", topo::subtract(&tool, &block, tol)),
        ];
        for (op, out) in ops {
            let Err(BooleanError::Escalated {
                decision: BooleanDecision::Coincidence(Coincide::CornerBound, DeclarationRead::Moot),
                diag,
            }) = out
            else {
                panic!("{label}, {op}: refuses on the corner's own reading: {out:?}");
            };
            assert_eq!(
                diag.predicate,
                Some("bool_chord_side"),
                "{label}, {op}: the steeper bound's first-order tie"
            );
            // A Zero rejected by the gate either definite side passes.
            assert_eq!(
                diag.margin.rejected_sign(),
                Some(geom_core::Sign::Zero),
                "{label}, {op}: a Zero tie"
            );
            assert!(
                format!("{:?}", diag.margin).contains("passes: NonZero"),
                "{label}, {op}: either side passes: {:?}",
                diag.margin
            );
            // The far vertex's coordinates round at the block's few metres,
            // which at ε 1e-12 is a part in a hundred of the smallest rise.
            let m = upper(diag.margin).abs();
            assert!(
                (m - bounds).abs() <= 1e-3 * bounds + 1e-15,
                "{label}, {op}: quotes the bound's margin {bounds}, got {m}"
            );
        }
    }
}
