//! **A wedge flush on a block's top, tilted in band over the two faces,
//! refuses every op at the corner, on its bounds' margin.**
//!
//! A `3 × 4.5 × 1` block, and a parallelepiped cornered on its top at
//! `(0.5, 0.2, 1)` by a 5° wedge angle, its face in the block's top
//! plane tilted about the wedge's edge by `θ = k·ε`, `k ∈ {±1.2, ±2}`;
//! the parallelepiped stands on the block (height 1) or is sunk 0.5 into
//! it.
//!
//! Read over both faces (the consumed extent, topo README C2), the two
//! carriers stand apart in the sliver band: the block's far corners stand
//! off the wedge's plane by ~2.3θ. So the glue door leaves the pair
//! unglued (D10, Booleans: the sliver band refuses), and every op reaches
//! the corner, where the wedge's bounds lie on the block's plane by
//! `θ·sin 5°`, Zero, while its plane reads tilted at the sector's reach:
//! `Coincidence(Sectors)`, quoting the bounds' margin. That margin is the
//! one a smaller tolerance clears (`offer_rows`'
//! `thin_wedge_in_band_over_its_faces` executes it); the glue door's,
//! ~30× larger, is not.

#![allow(clippy::expect_used, clippy::panic)]

use geom_core::test_support::upper;
use geom_core::{Band, Point3, Tol, Vec3};
use topo::test_support::{brick, finished, mapped_cube};
use topo::{
    Body, BooleanDecision, BooleanError, CarrierDesc, Coincide, PlaneEqError, PlaneRung,
    face_carrier,
};

/// The one face of `body` whose plane's normal leans along `facing`
/// (`+1` up, `−1` down).
fn face_facing(body: &Body<f64>, facing: f64) -> topo::FaceKey {
    let hits: Vec<_> = body
        .faces()
        .map(|(k, _)| k)
        .filter(|&k| {
            matches!(face_carrier(body, k),
                Some(CarrierDesc::Plane { normal, .. }) if normal.z * facing > 0.99)
        })
        .collect();
    assert_eq!(hits.len(), 1, "one face faces {facing}");
    hits[0]
}

#[test]
fn a_wedge_tilted_in_band_over_a_block_top_refuses_every_op_on_its_bounds_margin() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the witness band");
    let phi = 5.0_f64.to_radians();
    let p = Point3::new(0.5, 0.2, 1.0);
    let block = finished(
        "block",
        brick::<f64>((0.0, 3.0), (-2.0, 2.5), (0.0, 1.0), tol),
        tol,
    );
    // (pose, height, depth, the wedge face's facing)
    for (pose, height, depth, facing) in [("standing", 1.0, 0.0, -1.0), ("sunk", 0.5, 0.5, 1.0)] {
        for k in [1.2, -1.2, 2.0, -2.0] {
            let theta = k * band.zero();
            let (ea, eb) = (
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(phi.cos(), phi.sin(), theta * phi.sin()),
            );
            let tool = finished(
                "tool",
                mapped_cube::<f64>(
                    move |u, v, w| p + ea * u + eb * v + Vec3::new(0.0, 0.0, height * w - depth),
                    tol,
                ),
                tol,
            );
            let label = format!("{pose} at θ = {k}ε");
            match topo::flush_pair_relation(
                &block,
                face_facing(&block, 1.0),
                &tool,
                face_facing(&tool, facing),
                false,
                band,
            ) {
                Ok(Err(PlaneEqError::Escalated {
                    rung: PlaneRung::Parallel,
                    diag,
                })) => {
                    let m = upper(diag.margin);
                    assert!(
                        m > band.zero() && m < band.escalate(),
                        "{label}: the glue door's tilt {m} is in the sliver band"
                    );
                }
                other => panic!("{label}: the glue door reads the pair in band: {other:?}"),
            }
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
                match out {
                    Err(BooleanError::Escalated {
                        decision: BooleanDecision::Coincidence(Coincide::Sectors, _),
                        diag,
                    }) => {
                        let m = upper(diag.margin).abs();
                        assert!(
                            (m - bounds).abs() <= 1e-3 * bounds,
                            "{label}, {op}: quotes the bounds' margin {bounds}, got {m}"
                        );
                    }
                    other => panic!("{label}, {op}: refuses at the corner: {other:?}"),
                }
            }
        }
    }
}
