//! **A tie carried onto several copies in one body** (WIRE): a placed
//! union, the op that carries several copies of a tie onto one body.
//! Disjoint instances keep every candidate, so each instance's tie
//! stays tied.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unreachable
)]

use crate::fixture;
use editor_core::ExtrudeSide;

use editor_core::{
    Entry, EvalOptions, Evaluation, Formula, Node, PatternKind, ProfileDoc, RecipeNodeId,
};
use fixture::{insert, len, on_frame, scl, table};
use geom_core::Tol;

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    fixture::run(doc, &EvalOptions::default())
}

/// A rectangular block: profile on the plane z = `z0`, extruded `dz`.
fn block(
    doc: ProfileDoc,
    (x0, x1): (f64, f64),
    (y0, y1): (f64, f64),
    z0: f64,
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, z0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(dz),
            side: ExtrudeSide::Along,
        },
    )
}

/// `m4_pr3_names_bool`'s U-cutter subtract, rebuilt: a 4×4×4 block
/// less a U whose two prongs cross one wall, leaving two cap
/// fragments that no covariant qualifier separates — the genuine N2
/// tie that suite pins, one candidate in each prong.
fn u_cutter_subtract() -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("wire-product-gather-tie", Tol::witness());
    let (doc, a) = block(doc, (0.0, 4.0), (0.0, 4.0), 0.0, 4.0);
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (2.0, 1.0),
            (6.0, 1.0),
            (6.0, 3.0),
            (2.0, 3.0),
            (2.0, 2.5),
            (5.0, 2.5),
            (5.0, 1.5),
            (2.0, 1.5),
        ]],
    );
    let (doc, b) = insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(2.0),
            side: ExtrudeSide::Along,
        },
    );
    let (doc, sub) = insert(
        doc,
        Node::Subtract {
            from: a.into(),
            tool: b.into(),
            declare: Vec::new(),
        },
    );
    (doc, sub)
}

/// The U-cutter subtract's tie, placed three times by a placed union
/// whose instances are disjoint: each instance's tie keeps BOTH its
/// candidates through the fuse, so the fused table carries one
/// two-candidate `Tied` row per prototype tie per instance — the
/// several-survivor branch of the placed union's narrowing, which no
/// other row reaches.
#[test]
fn a_placed_union_carries_each_instances_tie_with_both_candidates() {
    let (doc, sub) = u_cutter_subtract();
    let (doc, group) = insert(
        doc,
        Node::placed_union(
            sub,
            Formula::count(3),
            PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(10.0),
            },
        )
        .unwrap(),
    );
    let ev = run(&doc);
    let proto: Vec<usize> = table(&ev, sub)
        .iter()
        .filter_map(|(_, e)| match e {
            Entry::Tied(c) => Some(c.len()),
            Entry::Unique(_) => None,
        })
        .collect();
    assert_eq!(proto, vec![2, 2], "the prototype's two-candidate ties");
    let fused: Vec<usize> = table(&ev, group)
        .iter()
        .filter_map(|(_, e)| match e {
            Entry::Tied(c) => Some(c.len()),
            Entry::Unique(_) => None,
        })
        .collect();
    assert_eq!(
        fused,
        vec![2; proto.len() * 3],
        "one two-candidate tie per prototype tie per instance"
    );
}
