//! **A band along a joined flush edge is named by that one edge.**
//!
//! A declared union of two x-offset blocks merges its flush walls and
//! caps, and its output stage joins the vertices where the operands'
//! rims met (maximal edges): each long edge of the result is ONE edge,
//! named for the set of the two blocks' rims it spans. Filleting or
//! chamfering every edge carves each long edge as one band, and the
//! emitter names it [`RoleSeg::BlendFace`] of that edge, as it names
//! every band over one edge.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::AuthoredNode;
use editor_core::ExtrudeSide;

use editor_core::{
    CancelToken, EntityKind, EvalOptions, Evaluation, Node, ProfileDoc, RecipeNodeId, RoleSeg,
    evaluate,
};
use fixture::{declare_x_offset_flush, insert, len, on_frame, table};
use geom_core::Tol;

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// A unit-section block over `x`, extruded one unit from z = 0.
fn block(doc: ProfileDoc, (x0, x1): (f64, f64)) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(x0, 0.0), (x1, 0.0), (x1, 1.0), (x0, 1.0)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    )
}

#[test]
fn a_band_along_a_joined_flush_edge_is_named_by_that_edge() {
    let doc = ProfileDoc::empty_derived("band_joined_rim_names", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0));
    let (doc, b) = block(doc, (0.5, 2.0));
    let decl = declare_x_offset_flush(&doc, a, b);
    let (doc, u) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
            declare: decl,
        },
    );
    let ev = run(&doc);
    let edges: Vec<_> = table(&ev, u)
        .iter()
        .map(|(n, _)| n.clone())
        .filter(|n| n.kind == EntityKind::Edge)
        .collect();
    assert_eq!(edges.len(), 12, "the union is a box: {edges:?}");
    let long: Vec<_> = edges
        .iter()
        .filter_map(|n| match n.path.as_slice() {
            [RoleSeg::Merged(set)] => Some(set.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(long.len(), 4, "the four long edges are joined: {edges:?}");
    for set in &long {
        let (a_read, b_read) = (fixture::out(&doc, a), fixture::out(&doc, b));
        let sides: Vec<_> = set
            .iter()
            .map(|c| match c.path.as_slice() {
                [RoleSeg::From { read, of: _ }] if *read == a_read => 'a',
                [RoleSeg::From { read, of: _ }] if *read == b_read => 'b',
                _ => panic!("a set constituent is an operand edge: {c:?}"),
            })
            .collect();
        assert_eq!(sides, ['a', 'b'], "one rim of each block: {set:?}");
    }
    for chamfer in [false, true] {
        let node = if chamfer {
            Node::chamfer(u, len(0.125), edges.clone())
        } else {
            Node::fillet(u, len(0.125), edges.clone())
        };
        assert_named(&doc, node, &edges, chamfer);
    }
}

/// The blend node's table: one band per union edge, named for it, and
/// no band of several links.
fn assert_named(
    doc: &ProfileDoc,
    node: AuthoredNode,
    edges: &[editor_core::StableName],
    chamfer: bool,
) {
    let (doc, f) = insert(doc.clone(), node);
    let ev = run(&doc);
    let t = table(&ev, f);
    let bands: Vec<_> = t
        .iter()
        .filter_map(|(n, _)| match n.path.first() {
            Some(RoleSeg::BlendFace(e)) if n.kind == EntityKind::Face && n.node == f => {
                Some(e.name().clone())
            }
            _ => None,
        })
        .collect();
    let mut want = edges.to_vec();
    want.sort();
    let mut got = bands.clone();
    got.sort();
    assert_eq!(got, want, "chamfer {chamfer}: one band per union edge");
    let chains = t
        .iter()
        .filter(|(n, _)| matches!(n.path.first(), Some(RoleSeg::BandFace(_))))
        .count();
    assert_eq!(chains, 0, "chamfer {chamfer}: no edge is a chain of links");
}
