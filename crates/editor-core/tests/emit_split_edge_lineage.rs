//! **A split reads a twice-crossed operand edge's lineage across both
//! halves.** The clipped cylinder (`test_support::clipped_cylinder`)
//! crosses its start rim arc twice and leaves its wall and start cap
//! one piece per side, so no face ranking is asked for. The arc's
//! middle piece lies Above and its outer two Below, and a Below piece
//! descends through the middle piece's key, which only the Above half
//! holds.
//!
//! The split still refuses: the pieces and crossing vertices a
//! twice-crossed edge leaves on one side share one name, and the split
//! has no multiplicity rule for them
//! (`work/emit/a-split-mints-a-twice-crossed-edges-pieces-under-one-name.md`).
//! The refusal pinned here is the first collision, spelled on the
//! extrude's rim edge, which only a lineage chased to the rim can
//! spell; when the rule lands, this test asserts the names instead.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::test_support::clipped_cylinder;
use editor_core::{
    CancelToken, CapEnd, EvalOptions, NamingError, NodeErrorKind, PieceRole, ProfileEdgeRef,
    RoleSeg, SplitHalf, evaluate,
};
use geom_core::Tol;

#[test]
fn a_rim_arc_crossed_twice_is_spelled_by_the_rim() {
    let (doc, [ext, _, split]) = clipped_cylinder(Tol::witness());
    let ev = evaluate::<f64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    assert!(
        ev.value(ext).is_some(),
        "the extrude evaluates: {:?}",
        ev.nodes.get(&ext)
    );
    let kind = ev
        .nodes
        .get(&split)
        .and_then(|n| n.error())
        .map(|e| &e.kind);
    let Some(NodeErrorKind::Naming(NamingError::Duplicate { name })) = kind else {
        panic!("the split refuses the one-side collision, not {kind:?}");
    };
    assert_eq!(name.node, split, "the split minted the colliding name");
    let [
        RoleSeg::CrossingVertex {
            side: SplitHalf::Above,
            edge,
        },
    ] = name.path.as_slice()
    else {
        panic!("the collision is the Above crossing vertices: {name:?}");
    };
    assert_eq!(edge.node, ext, "the collision cites the extrude: {edge:?}");
    assert!(
        matches!(
            edge.path.as_slice(),
            [RoleSeg::RimEdge(
                CapEnd::Start,
                ProfileEdgeRef::Piece {
                    role: PieceRole::Piece(0),
                    ..
                }
            )]
        ),
        "the collision cites the start rim's Piece(0): {edge:?}"
    );
}
