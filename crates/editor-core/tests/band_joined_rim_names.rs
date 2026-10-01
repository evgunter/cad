//! **A band carved across joints is named by its chain's edge set.**
//!
//! A declared union of two x-offset blocks merges its flush walls and
//! caps, and the merge keeps the vertices where the operands' rims
//! met: each long edge of the result is a chain of collinear links on
//! the same two faces. Filleting or chamfering every edge carves each
//! such chain as ONE band face, and the emitter names it [`RoleSeg::BandFace`] of
//! the chain's source edge names — a set, the same covariant identity
//! a closed rim's band takes.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    BooleanOp, CancelToken, EntityKind, EvalOptions, Evaluation, Node, ProfileDoc, RecipeNodeId,
    RoleSeg, evaluate,
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
            profile: p,
            distance: len(1.0),
        },
    )
}

#[test]
fn a_joined_band_is_named_by_its_chains_edge_set() {
    let doc = ProfileDoc::empty_derived("band_joined_rim_names", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0));
    let (doc, b) = block(doc, (0.5, 2.0));
    let (doc, decl) = declare_x_offset_flush(doc, a, b);
    let (doc, u) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b,
            declare: Some(decl),
        },
    );
    let ev = run(&doc);
    let edges: Vec<_> = table(&ev, u)
        .iter()
        .map(|(n, _)| n.clone())
        .filter(|n| n.kind == EntityKind::Edge)
        .collect();
    assert!(
        edges.len() > 12,
        "the long edges are split: {}",
        edges.len()
    );
    for chamfer in [false, true] {
        let node = if chamfer {
            Node::chamfer(u, len(0.125), edges.clone())
        } else {
            Node::fillet(u, len(0.125), edges.clone())
        };
        assert_named(&doc, node, &edges, chamfer);
    }
}

/// The blend node's table: four joined bands of three links each, all
/// union edges, and one plain band per unsplit edge.
fn assert_named(
    doc: &ProfileDoc,
    node: Node<editor_core::ProfileProgram>,
    edges: &[editor_core::StableName],
    chamfer: bool,
) {
    let (doc, f) = insert(doc.clone(), node);
    let ev = run(&doc);
    let t = table(&ev, f);
    let bands: Vec<Vec<_>> = t
        .iter()
        .filter_map(|(n, _)| match n.path.first() {
            Some(RoleSeg::BandFace(set)) if n.kind == EntityKind::Face && n.node == f => {
                Some(set.clone())
            }
            _ => None,
        })
        .collect();
    assert_eq!(
        bands.len(),
        4,
        "chamfer {chamfer}: one joined band per long edge: {bands:?}"
    );
    for set in &bands {
        // Split at x = 0.5 and x = 1, where the operands' rims ended:
        // three links, two joints.
        assert_eq!(
            set.len(),
            3,
            "chamfer {chamfer}: a joined band spans its links: {set:?}"
        );
        let mut sorted = set.clone();
        sorted.sort();
        assert_eq!(&sorted, set, "the set is canonical");
        for e in set {
            assert!(edges.contains(e), "a member is a union edge: {e:?}");
        }
    }
    let blend_faces = t
        .iter()
        .filter(|(n, _)| matches!(n.path.first(), Some(RoleSeg::BlendFace(_))))
        .count();
    assert_eq!(blend_faces, 8, "the eight unsplit edges keep one band each");
}
