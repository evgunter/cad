//! **A cylinder standing inside a block, its caps flush with the
//! block's top and bottom, takes its side from an edge's midpoint**
//! (`topo::boolean::shell_witness`, tier 2). Every vertex lies on a cap
//! circle and so on the block's boundary; the lateral faces are curved,
//! which the face-interior tier does not read; the midpoint of each
//! straight seam edge between the arcs lies strictly inside the block.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use sweep::test_support::disc_of_arcs;
use topo::flush::{declare_all, find_flush_candidates};
use topo::{BooleanResult, mass_properties, union_with};

#[test]
fn a_cylinder_flush_inside_a_block_unions_to_the_block() {
    let tol = Tol::witness();
    let block = topo::test_support::brick((-1.0, 1.0), (-1.0, 1.0), (0.0, 1.0), tol);
    let cylinder = disc_of_arcs(3, 0.5, 1.0, tol);
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
