//! **A split reads a twice-crossed operand edge's lineage across both
//! halves.** The plane through `(0, 0.2, 0)` with normal `(0, 1, -1)`
//! crosses a cylinder's start rim arc twice and leaves its wall and
//! start cap one piece per side, so no face ranking is asked for. The
//! arc's middle piece lies Above and its outer two Below, and a Below
//! piece descends through the middle piece's key, which only the Above
//! half holds.
//!
//! The split still refuses: the pieces and crossing vertices a
//! twice-crossed edge leaves on one side share one name, and the split
//! has no multiplicity rule for them
//! (`work/emit/a-split-mints-a-twice-crossed-edges-pieces-under-one-name.md`).
//! The refusal pinned here is that collision, spelled on the extrude's
//! rim edge, which only a lineage chased to the rim can spell; when the
//! rule lands, this test asserts the names instead.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{frame, insert, len, scl};
use editor_core::{
    CancelToken, CapEnd, Datum, EvalOptions, Evaluation, LoopProgram, NamingError, Node,
    NodeErrorKind, ProfileDoc, ProfileProgram, RecipeNodeId, RoleSeg, evaluate,
};
use geom_core::Tol;

/// A cylinder, `circle(0, 0, 0.5)` extruded 1.0, split by the plane
/// through `(0, 0.2, 0)` with normal `(0, 1, -1)`: the extrude's id,
/// the split's, and the evaluation.
fn clipped_cylinder() -> (Evaluation<f64>, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("emit_split_edge_lineage", Tol::witness());
    let (doc, plane) = insert(doc, frame([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![LoopProgram::circle(0.0, 0.0, 0.5).expect("finite")],
            ids: Vec::new(),
        }),
    );
    let (doc, ext) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    let (doc, tool) = insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(0.0), len(0.2), len(0.0)],
            normal: [scl(0.0), scl(1.0), scl(-1.0)],
        }),
    );
    let (doc, split) = insert(doc, Node::Split { target: ext, tool });
    let ev = evaluate::<f64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    (ev, ext, split)
}

#[test]
fn a_rim_arc_crossed_twice_is_spelled_by_the_rim() {
    let (ev, ext, split) = clipped_cylinder();
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
    let cited = match name.path.as_slice() {
        [RoleSeg::CrossingVertex { edge, .. }] => edge,
        [RoleSeg::SplitFragment { parent, .. }] => parent,
        other => panic!("the collision is a crossing vertex or an edge piece: {other:?}"),
    };
    assert_eq!(name.node, split, "the split minted the colliding name");
    assert_eq!(
        cited.node, ext,
        "the collision cites the extrude: {cited:?}"
    );
    assert!(
        matches!(cited.path.as_slice(), [RoleSeg::RimEdge(CapEnd::Start, _)]),
        "the collision cites the start rim arc: {cited:?}"
    );
}
