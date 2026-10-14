//! **A wedge flush on a block's top, tilted in band over the two faces,
//! refuses every op at its corner's coplanar lump, on its bounds' margin.**
//!
//! `test_support::wedge_on_a_block_top`: a 5° wedge cornered on a block's
//! top, its face in the block's top plane tilted about the wedge's edge
//! by `θ = k·ε`, `k ∈ {±1.2, ±2}`, standing on the block (height 1) or
//! sunk 0.5 into it.
//!
//! Read over both faces (the consumed extent, topo README C2), the two
//! carriers stand apart in the sliver band: the block's far corners stand
//! off the wedge's plane by ~2.3θ. So the glue door leaves the pair
//! unglued (D10, Booleans: the sliver band refuses), and every op meets
//! the pair at the wedge's corner, in `vtxfac`'s coplanar lump: the
//! sector's tilt is in band at its reach while its bounds lie on the
//! block's plane by `θ·sin 5°`, Zero. It refuses `Coincidence(Sectors)`
//! quoting that bounds' margin, which is the one a smaller tolerance
//! clears; `offer_rows`' `thin_wedge_in_band_over_its_faces` executes it,
//! and `the_in_band_wedge_is_in_band_at_the_glue_door` shows the glue
//! door's own offer would be false.
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
    let phi = 5.0_f64.to_radians();
    // (pose, height, depth, the wedge face's facing)
    for (pose, height, depth, facing) in [("standing", 1.0, 0.0, -1.0), ("sunk", 0.5, 0.5, 1.0)] {
        for k in [1.2, -1.2, 2.0, -2.0] {
            let theta = k * band.zero();
            let (block, tool) = wedge_on_a_block_top(5.0, theta, height, depth, tol);
            let (block, tool) = (finished("block", block, tol), finished("tool", tool, tol));
            let label = format!("{pose} at θ = {k}ε");
            let glue = topo::flush_pair_relation(
                &block,
                face_facing(&block, 1.0),
                &tool,
                face_facing(&tool, facing),
                false,
                band,
            );
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
                // The coplanar lump admits the class the pair's senses
                // make it (`Settles`); `recl`'s `unglued_tilt`, which
                // quotes the same predicate and margin, admits none.
                match out {
                    Err(BooleanError::Escalated {
                        decision:
                            BooleanDecision::Coincidence(Coincide::Sectors, DeclarationRead::Settles(_)),
                        diag,
                    }) => {
                        assert_eq!(
                            diag.predicate,
                            Some("bool_sector_coplanar"),
                            "{label}, {op}: the bounds' predicate"
                        );
                        let m = upper(diag.margin).abs();
                        assert!(
                            (m - bounds).abs() <= 1e-3 * bounds,
                            "{label}, {op}: quotes the bounds' margin {bounds}, got {m}"
                        );
                    }
                    other => panic!("{label}, {op}: refuses at the coplanar lump: {other:?}"),
                }
            }
        }
    }
}
