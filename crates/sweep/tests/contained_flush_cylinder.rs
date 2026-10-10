//! **A cylinder standing inside a block, its caps flush with the
//! block's top and bottom, takes its side from an edge's midpoint**
//! (`topo::boolean::shell_witness`, tier 2). Every vertex lies on a cap
//! circle and so on the block's boundary; the lateral faces are curved,
//! which the face-interior tier does not read; the midpoint of each
//! straight seam edge between the arcs lies strictly inside the block.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use sweep::test_support::{disc_of_arcs, finished};
use topo::flush::{declare_all, find_flush_candidates};
use topo::{BooleanResult, mass_properties, union_with};

#[test]
fn a_cylinder_flush_inside_a_block_unions_to_the_block() {
    let tol = Tol::witness();
    let block = topo::test_support::brick((-1.0, 1.0), (-1.0, 1.0), (0.0, 1.0), tol);
    let block = finished("the block", block, tol);
    let cylinder = finished("the cylinder", disc_of_arcs(3, 0.5, 1.0, tol), tol);
    let decls = declare_all(&find_flush_candidates(&block, &cylinder, tol).expect("decides"));
    let BooleanResult::Body(union) =
        union_with(&block, &cylinder, &decls, tol).expect("the cylinder's side is decided")
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

/// **A carried edge-edge row on an arc refuses at the door**: edge-split
/// lineage reads each edge of the row as the segment between its ends,
/// which only a line edge is. Two arcs of the cylinder's top cap, carried
/// as an edge-edge row, refuse typed before any op runs. Red when the
/// door admits a curved edge-edge row.
#[test]
fn a_carried_edge_edge_row_on_an_arc_refuses_at_the_door() {
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
    let mut decls = declare_all(&find_flush_candidates(&cylinder, &block, tol).expect("decides"));
    decls.carried_a.ee = vec![topo::CarriedRecord { contact: topo::EeContact {
        a: arcs[0],
        b: arcs[1],
    }, record: 0 }];
    let got = union_with(&cylinder, &block, &decls, tol).map(|_| ());
    assert!(
        matches!(
            got,
            Err(topo::BooleanError::InvalidDeclaration {
                operand: topo::Operand::A,
                what: "carried e-e edge is not a certified line",
            })
        ),
        "{got:?}"
    );
}
