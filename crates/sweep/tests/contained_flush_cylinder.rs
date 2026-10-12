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
    let volume = mass_properties(&union.body, tol).unwrap().volume;
    assert!(
        (volume - 4.0).abs() < 1e-9,
        "the cylinder adds nothing to the 2 × 2 × 1 block: {volume}"
    );
    assert_eq!(topo::validate_geometric(&union.body, tol), Ok(()));
}

/// **A carried edge-edge row on arcs the union swallows is consumed**
/// (D1): the door admits a curved row, since the one carry asks the
/// census, not a segment question; the union discards the cylinder's cap
/// arcs, which lie in the block's boundary under its kept faces, so the
/// row (bookkeeping, two arcs of one cap) lands on no pair and the
/// removal consumes it. Red when the door refuses a curved row, or the
/// carry keeps the row on a cell the union removed.
#[test]
fn a_carried_edge_edge_row_on_swallowed_arcs_is_consumed() {
    let tol = Tol::witness();
    let block = topo::test_support::brick((-1.0, 1.0), (-1.0, 1.0), (0.0, 1.0), tol);
    let block = finished("the block", block, tol);
    let cylinder = finished("the cylinder", disc_of_arcs(3, 0.5, 1.0, tol), tol);
    let arcs: Vec<topo::EdgeKey> = cylinder
        .edges()
        .filter(|&(k, _)| {
            topo::readback::edge_carrier_kind(&cylinder, k)
                .is_ok_and(|c| c == geom::CurveKind::Circle)
        })
        .map(|(k, _)| k)
        .collect();
    assert!(arcs.len() >= 2, "the caps are rings of arcs");
    let inputs = topo::BooleanInputs {
        records_a: topo::OperandRecords {
            ee: vec![topo::CarriedRecord {
                contact: topo::EeContact {
                    a: arcs[0],
                    b: arcs[1],
                },
                record: 0,
            }],
            ..Default::default()
        },
        ..Default::default()
    };
    let got = topo::boolean_op_with(
        topo::BooleanOp::Union,
        &cylinder,
        &block,
        &inputs,
        topo::SweepStrategy::Realized,
        tol,
    );
    let Ok(BooleanResult::Body(out)) = got else {
        panic!("the union builds: {got:?}");
    };
    assert!(
        out.contacts.cites().all(|c| c
            .iter()
            .all(|b| !matches!(b, topo::Backing::Carried { .. }))),
        "the row is consumed: {:?}",
        out.contacts
    );
}
