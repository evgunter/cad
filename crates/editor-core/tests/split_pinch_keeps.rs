//! A Split whose plane passes through a pinch: one half holds the
//! operand corner as several vertices, each `OnToolVertex { of }` +
//! `Fragment(Keeps)` over the corner's own edges it is an end of a
//! piece of in that half (NAMES N2, *Vertices*), and the corner's
//! on-plane edge, held once per piece, is told apart by its `Ends`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;

use crate::fixture::{self, insert, len, on_frame, scl};
use editor_core::{
    CancelToken, Datum, EvalOptions, Evaluation, ExtrudeSide, Node, ProfileDoc, Qualifier,
    RecipeNodeId, RoleSeg, SplitHalf, StableName, evaluate,
};
use geom_core::Tol;

/// The notched block of `topo/tests/m3_pr3_split.rs`: the V-notch tip
/// `(4, 1)` has both neighbours above `y = 1`.
const NOTCHED: &[(f64, f64)] = &[
    (0.0, 0.0),
    (8.0, 0.0),
    (8.0, 2.0),
    (7.0, 1.0),
    (6.0, 1.0),
    (5.0, 2.0),
    (4.0, 1.0),
    (3.0, 2.0),
    (0.0, 2.0),
];

/// Its mirror image: the tip `(4, 1)` has both neighbours below.
const MIRRORED: &[(f64, f64)] = &[
    (0.0, 0.0),
    (3.0, 0.0),
    (4.0, 1.0),
    (5.0, 0.0),
    (6.0, 1.0),
    (7.0, 1.0),
    (8.0, 0.0),
    (8.0, 2.0),
    (0.0, 2.0),
];

/// The notched pentagon: the apex `(1, 1)` has both neighbours above.
const PENTAGON: &[(f64, f64)] = &[(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (1.0, 1.0), (0.0, 2.0)];

/// `outline` extruded 1 along z and split by the plane `y = 1` with
/// normal `(0, ny, 0)`.
fn split_at_y1(outline: &[(f64, f64)], ny: f64) -> (Evaluation<f64>, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("split_pinch_keeps", Tol::witness());
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![outline.to_vec()],
    );
    let (doc, prism) = insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    );
    let (doc, plane) = insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(0.0), len(1.0), len(0.0)],
            normal: [scl(0.0), scl(ny), scl(0.0)],
        }),
    );
    let (doc, sp) = insert(
        doc,
        Node::Split {
            target: prism.into(),
            tool: plane.into(),
        },
    );
    let ev = evaluate::<f64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    assert!(
        ev.value(sp).is_some(),
        "the split names with no refusal: {:?}",
        ev.node_error(sp)
    );
    (ev, sp)
}

/// `side`'s names, each spelled with the split's node read as `as_node`
/// (two documents that differ only in the plane's normal mint the split
/// under different digests).
fn names(
    ev: &Evaluation<f64>,
    sp: RecipeNodeId,
    side: SplitHalf,
    as_node: RecipeNodeId,
) -> BTreeSet<String> {
    let (from, to) = (format!("{sp:?}"), format!("{as_node:?}"));
    fixture::half(ev, sp, side)
        .iter()
        .map(|(n, _)| format!("{n:?}").replace(&from, &to))
        .collect()
}

/// The `Keeps` sets of `side`'s qualified corner copies.
fn copies(ev: &Evaluation<f64>, sp: RecipeNodeId, side: SplitHalf) -> Vec<Vec<StableName>> {
    let mut kept = Vec::new();
    for (n, _) in fixture::half(ev, sp, side).iter() {
        match n.path.as_slice() {
            [RoleSeg::OnToolVertex { .. }] => {}
            [
                RoleSeg::OnToolVertex { .. },
                RoleSeg::Fragment(Qualifier::Keeps(k)),
            ] => kept.push(k.clone()),
            [RoleSeg::OnToolVertex { .. }, ..] => panic!("an unexpected copy name {n:?}"),
            _ => {}
        }
    }
    kept
}

/// Each pinch copy keeps its own rim and the tip's lateral edge: two
/// lines, one shared with its twin, one not.
fn assert_tip_copies(kept: &[Vec<StableName>], what: &str) {
    assert_eq!(kept.len(), 4, "{what}: two copies at each cap: {kept:?}");
    for k in kept {
        assert_eq!(
            k.len(),
            2,
            "{what}: a copy keeps its rim and the tip edge: {k:?}"
        );
        assert!(
            k.iter()
                .any(|n| matches!(n.path.first(), Some(RoleSeg::LateralEdge(_)))),
            "{what}: the tip's lateral edge is one of them: {k:?}"
        );
    }
    let distinct: BTreeSet<_> = kept.iter().collect();
    assert_eq!(
        distinct.len(),
        4,
        "{what}: no two copies keep alike: {kept:?}"
    );
}

/// The pinched half is `pinched`; the other holds one copy per corner.
fn assert_pinch(outline: &[(f64, f64)], pinched: SplitHalf, what: &str) {
    let other = match pinched {
        SplitHalf::Above => SplitHalf::Below,
        SplitHalf::Below => SplitHalf::Above,
    };
    let (ev, sp) = split_at_y1(outline, 1.0);
    let kept = copies(&ev, sp, pinched);
    assert_tip_copies(&kept, what);
    let kept_other = copies(&ev, sp, other);
    assert!(
        kept_other.is_empty(),
        "{what}: a corner held once per half keeps its bare name: {kept_other:?}"
    );
    let (flipped, fsp) = split_at_y1(outline, -1.0);
    assert_eq!(
        names(&ev, sp, pinched, sp),
        names(&flipped, fsp, other, sp),
        "{what}: flipping the normal swaps the halves' tables"
    );
    assert_eq!(
        names(&ev, sp, other, sp),
        names(&flipped, fsp, pinched, sp),
        "{what}: flipping the normal swaps the halves' tables"
    );
}

#[test]
fn a_split_through_a_notch_tip_tells_its_copies_apart_by_keeps() {
    assert_pinch(NOTCHED, SplitHalf::Above, "notched");
}

#[test]
fn a_mirrored_split_through_a_notch_tip_tells_its_copies_apart_by_keeps() {
    assert_pinch(MIRRORED, SplitHalf::Below, "mirrored");
}

#[test]
fn a_split_at_a_notched_pentagons_apex_tells_its_copies_apart_by_keeps() {
    assert_pinch(PENTAGON, SplitHalf::Above, "pentagon");
}
