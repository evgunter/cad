//! **A cylinder standing inside a block, its caps flush with the
//! block's top and bottom, takes its side from an edge's midpoint**
//! (`topo::boolean::shell_witness`, tier 2). Every vertex lies on a cap
//! circle and so on the block's boundary; the lateral faces are curved,
//! which the face-interior tier does not read; the midpoint of each
//! straight seam edge between the arcs lies strictly inside the block.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use sweep::test_support::{disc_of_arcs, finished};
use topo::{BooleanResult, mass_properties, union};

#[test]
fn a_cylinder_flush_inside_a_block_unions_to_the_block() {
    let tol = Tol::witness();
    let block = topo::test_support::brick((-1.0, 1.0), (-1.0, 1.0), (0.0, 1.0), tol);
    let block = finished("the block", block, tol);
    let cylinder = finished("the cylinder", disc_of_arcs(3, 0.5, 1.0, tol), tol);
    let BooleanResult::Body(union) =
        union(&block, &cylinder, tol).expect("the cylinder's side is decided")
    else {
        panic!("a union of non-empty bodies cannot be empty");
    };
    let volume = mass_properties(&union.body, tol, topo::test_support::pinned_resolution(tol))
        .unwrap()
        .met()
        .expect("the pinned resolution is met")
        .volume;
    assert!(
        (volume - 4.0).abs() < 1e-9,
        "the cylinder adds nothing to the 2 × 2 × 1 block: {volume}"
    );
    assert_eq!(topo::validate_geometric(&union.body, tol), Ok(()));
}

/// **A carried edge-edge row on a wall the union discards is consumed by
/// the removal** (D1): the door admits a curved row, since the one carry
/// asks the census, not a segment question. The row names a top-cap arc
/// and the wall seam that does not meet it (bookkeeping: the two never
/// touch). Unioned with a block it lies flush inside, the cylinder's
/// walls are discarded and its caps merged into the block's faces, so
/// the seam is removed, the row lands on no pair, and the removal
/// consumes it. Unioned with a block far away, both cells are kept, and
/// the same row lands on them by lineage, unread: what consumes it is
/// the removal, never a reading of an untouched pair. Red when the door
/// refuses a curved row, when the carry keeps the row on a cell the
/// union removed, or when it re-reads a row on cells the op kept.
#[test]
fn a_carried_edge_edge_row_on_a_discarded_wall_is_consumed() {
    let tol = Tol::witness();
    let cylinder = finished("the cylinder", disc_of_arcs(3, 0.5, 1.0, tol), tol);
    let ends = |k: topo::EdgeKey| {
        let e = cylinder.get_edge(k).unwrap();
        [e.he_plus, e.he_minus].map(|h| cylinder.half_edge_start_point(h).unwrap())
    };
    let arc = |k: topo::EdgeKey| {
        topo::readback::edge_carrier_kind(&cylinder, k).is_ok_and(|c| c == geom::CurveKind::Circle)
    };
    let top = cylinder
        .edges()
        .map(|(k, _)| k)
        .find(|&k| arc(k) && ends(k)[0].z > 0.5)
        .expect("a top-cap arc");
    let seam = cylinder
        .edges()
        .map(|(k, _)| k)
        .find(|&k| {
            !arc(k)
                && !ends(k)
                    .iter()
                    .any(|p| ends(top).iter().any(|q| (*p - *q).norm() < 1e-9))
        })
        .expect("the seam the arc does not meet");
    let inputs = topo::BooleanInputs {
        records_a: topo::OperandRecords {
            ee: vec![topo::CarriedRecord {
                contact: topo::EeContact { a: top, b: seam },
                record: 0,
            }],
            ..Default::default()
        },
        ..Default::default()
    };
    let with = |block: (f64, f64)| {
        let block = topo::test_support::brick(block, (-1.0, 1.0), (0.0, 1.0), tol);
        let block = finished("the block", block, tol);
        topo::boolean_op_with(
            topo::BooleanOp::Union,
            &cylinder,
            &block,
            &inputs,
            topo::SweepStrategy::Realized,
            tol,
        )
    };
    let got = with((-1.0, 1.0));
    let Ok(BooleanResult::Body(out)) = got else {
        panic!("the union builds: {got:?}");
    };
    assert!(
        out.contacts.is_empty(),
        "the row is consumed: {:?}",
        out.contacts
    );
    let far = with((5.0, 7.0));
    let Ok(BooleanResult::Body(far)) = far else {
        panic!("the far union builds: {far:?}");
    };
    assert_eq!(
        far.contacts.edge_edge().count(),
        1,
        "with both cells kept, the row lands: {:?}",
        far.contacts
    );
}
