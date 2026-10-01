//! **A split reads a twice-crossed operand edge's lineage across both
//! halves, and names what it leaves.** The clipped cylinder
//! (`test_support::clipped_cylinder`) crosses its start rim arc twice
//! and leaves its wall and start cap one piece per side, so no face
//! piece shares a side with another. The arc's middle piece lies Above
//! and its outer two Below, and a Below piece descends through the
//! middle piece's key, which only the Above half holds.
//!
//! The two Below pieces of the rim are pieces of one parent edge on one
//! side, told apart by their ends (N2's `Ends`); the two crossings are
//! one name on each side, ranked along the rim by its carrier's own
//! parameter (N2's crossing ordinal), so a crossing has one rank on
//! both sides.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use editor_core::test_support::clipped_cylinder;
use editor_core::{
    CancelToken, CapEnd, EntityKey, EntityKind, Entry, EvalOptions, PieceRole, ProfileEdgeRef,
    Qualifier, RoleSeg, SplitHalf, SplitSide, StableName, ValuePayload, evaluate,
};
use geom_core::Tol;

use crate::fixture::point;

#[test]
fn a_rim_arc_crossed_twice_names_its_pieces_by_their_ends_and_ranks_its_crossings() {
    let (doc, [ext, _, split]) = clipped_cylinder(Tol::witness());
    let ev = evaluate::<f64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    let value = ev
        .value(split)
        .unwrap_or_else(|| panic!("the split names: {:?}", ev.nodes.get(&split)));
    let ValuePayload::Split { above, below } = &value.payload else {
        panic!("a split value");
    };
    let body = |s: &SplitSide<f64>| match s {
        SplitSide::Body(b) => std::sync::Arc::clone(b),
        SplitSide::Empty => panic!("both halves carry material"),
    };
    let halves = [body(above), body(below)];
    let rim = |name: &StableName| {
        name.node == ext
            && matches!(
                name.path.as_slice(),
                [RoleSeg::RimEdge(
                    CapEnd::Start,
                    ProfileEdgeRef::Piece {
                        role: PieceRole::Piece(0),
                        ..
                    }
                )]
            )
    };
    // (side, kind) → the rows on the start rim's Piece(0), by tail.
    let mut pieces: BTreeMap<SplitHalf, Vec<StableName>> = BTreeMap::new();
    let mut crossings: BTreeMap<(SplitHalf, u32), [f64; 3]> = BTreeMap::new();
    for (name, entry) in value.name_table.iter() {
        let Entry::Unique(at) = entry else {
            panic!("no row ties: {name:?} {entry:?}");
        };
        let half = &halves[at.body as usize];
        match (name.kind, name.path.as_slice()) {
            (EntityKind::Edge, [RoleSeg::SplitFragment { side, parent }, tail @ ..])
                if rim(parent) =>
            {
                assert!(
                    matches!(tail, [] | [RoleSeg::Fragment(Qualifier::Ends(_))]),
                    "{name:?}"
                );
                pieces.entry(*side).or_default().push(name.clone());
            }
            (
                EntityKind::Vertex,
                [
                    RoleSeg::CrossingVertex { side, edge },
                    RoleSeg::Fragment(Qualifier::OrderAlong { rank, of: 2 }),
                ],
            ) if rim(edge) => {
                let EntityKey::Vertex(v) = at.key else {
                    panic!("a vertex row names a vertex: {name:?}");
                };
                let p = point(half, v);
                crossings.insert((*side, *rank), [p.x, p.y, p.z]);
            }
            (EntityKind::Vertex, [RoleSeg::CrossingVertex { edge, .. }, ..]) if rim(edge) => {
                panic!("a crossing of the rim is ranked one of two: {name:?}")
            }
            _ => {}
        }
    }
    let above_pieces = &pieces[&SplitHalf::Above];
    assert_eq!(
        above_pieces.len(),
        1,
        "the middle piece is the Above half's one piece of the rim: {above_pieces:?}"
    );
    assert_eq!(
        above_pieces[0].path.len(),
        1,
        "one piece takes no qualifier"
    );
    let below_pieces = &pieces[&SplitHalf::Below];
    assert_eq!(
        below_pieces.len(),
        2,
        "the outer two pieces lie Below, each its own name: {below_pieces:?}"
    );
    for piece in below_pieces {
        assert!(
            matches!(
                piece.path.last(),
                Some(RoleSeg::Fragment(Qualifier::Ends(ends))) if ends.len() == 2
                    && ends.iter().any(|e| matches!(
                        e.path.first(),
                        Some(RoleSeg::CrossingVertex { side: SplitHalf::Below, .. })
                    ))
            ),
            "a Below piece ends at a Below crossing: {piece:?}"
        );
    }
    assert_eq!(
        crossings.len(),
        4,
        "two ranked crossings a side: {crossings:?}"
    );
    for rank in 0..2 {
        let (a, b) = (
            crossings[&(SplitHalf::Above, rank)],
            crossings[&(SplitHalf::Below, rank)],
        );
        let gap = (0..3).map(|i| (a[i] - b[i]).abs()).fold(0.0, f64::max);
        assert!(
            gap < 1e-9,
            "crossing #{rank} is one place on both sides: {a:?} {b:?}"
        );
    }
    let (first, second) = (
        crossings[&(SplitHalf::Above, 0)],
        crossings[&(SplitHalf::Above, 1)],
    );
    assert!(
        (first[0] - second[0]).abs() > 0.5,
        "the two crossings are the two ends of the chord at y = 0.2: {first:?} {second:?}"
    );
    // Rank 0 is the crossing first along the rim as the extrude stores
    // it: on a semicircle the chord from the start grows along the arc,
    // so it is the crossing nearer the rim's start.
    let ext_body = crate::corpus::body_of(&ev, ext);
    let rim_edge = ext_body
        .edges()
        .map(|(e, _)| e)
        .find(|&e| {
            ev.value(ext)
                .unwrap()
                .name_table
                .name_of(&editor_core::EntityRef {
                    body: 0,
                    key: EntityKey::Edge(e),
                })
                .is_some_and(&rim)
        })
        .expect("the extrude's start rim Piece(0)");
    let start = point(ext_body, crate::fixture::ends(ext_body, rim_edge)[0]);
    let from_start = |c: [f64; 3]| ((c[0] - start.x).powi(2) + (c[1] - start.y).powi(2)).sqrt();
    assert!(
        from_start(first) < from_start(second),
        "rank 0 is the crossing first along the rim's stored direction: {first:?} {second:?} \
         from {start:?}"
    );
}

/// **A plane crossing a cylinder's wall twice names the wall's two
/// pieces on one side by the edges they keep.** The cylinder (radius
/// 0.3, its wall the profile's two semicircular pieces) is split by the
/// plane y = 0.15, which crosses the `+y` half-wall twice: the strip
/// between the two crossing lines lies Above, and the two outer strips
/// Below. Each Below strip keeps a stretch of both rims and one lateral
/// edge, a different one each, so N2's `Keeps` tells them apart; no
/// plane is read.
#[test]
fn a_wall_crossed_twice_names_its_same_side_pieces_by_the_edges_they_keep() {
    use editor_core::{Datum, Node, ProfileDoc};

    use crate::emit_union_borders::cylinder;
    use crate::fixture::{insert, len, scl};

    let doc = ProfileDoc::empty_derived("wall-crossed-twice", Tol::witness());
    let (doc, ext) = cylinder(doc, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], 1.0);
    let (doc, tool) = insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(0.0), len(0.15), len(0.0)],
            normal: [scl(0.0), scl(1.0), scl(0.0)],
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
    let value = ev
        .value(split)
        .unwrap_or_else(|| panic!("the split names: {:?}", ev.nodes.get(&split)));
    let mut kept: BTreeMap<StableName, Vec<Vec<StableName>>> = BTreeMap::new();
    for (name, entry) in value.name_table.iter() {
        assert!(
            matches!(entry, Entry::Unique(_)),
            "no row ties: {name:?} {entry:?}"
        );
        if let [
            RoleSeg::SplitFragment {
                side: SplitHalf::Below,
                parent,
            },
            RoleSeg::Fragment(Qualifier::Keeps(edges)),
        ] = name.path.as_slice()
        {
            assert_eq!(name.kind, EntityKind::Face, "{name:?}");
            kept.entry(parent.name().clone())
                .or_default()
                .push(edges.clone());
        }
    }
    let [(wall, pieces)] = kept.iter().collect::<Vec<_>>()[..] else {
        panic!("one parent has two Below pieces: {kept:?}");
    };
    assert_eq!(wall.node, ext, "{wall:?}");
    assert!(
        matches!(wall.path.as_slice(), [RoleSeg::Lateral(_)]),
        "the parent is a wall face: {wall:?}"
    );
    assert_eq!(pieces.len(), 2, "{pieces:?}");
    assert_ne!(pieces[0], pieces[1], "the two pieces keep different edges");
    for edges in pieces {
        let rims = edges
            .iter()
            .filter(|e| matches!(e.path.as_slice(), [RoleSeg::RimEdge(..)]))
            .count();
        let laterals = edges
            .iter()
            .filter(|e| matches!(e.path.as_slice(), [RoleSeg::LateralEdge(_)]))
            .count();
        assert_eq!(
            (rims, laterals, edges.len()),
            (2, 1, 3),
            "a Below strip keeps both rims and one lateral edge: {edges:?}"
        );
    }
}
