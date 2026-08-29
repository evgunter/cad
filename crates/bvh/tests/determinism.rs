//! The tree's own idealized/realized pin (PERF-PLAN §4.4, at the
//! smallest scale): brute-force box filtering is the ten-line
//! definition of the candidate set; the tree must reproduce it
//! EXACTLY (same set, ascending order) on arbitrary inputs, and the
//! build must be bit-deterministic (same input ⇒ same `Debug` form).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use bvh::{Aabb, Bvh};
use proptest::prelude::*;

fn boxed(min: [f64; 3], max: [f64; 3]) -> Aabb {
    Aabb {
        min_x: min[0],
        min_y: min[1],
        min_z: min[2],
        max_x: max[0],
        max_y: max[1],
        max_z: max[2],
    }
}

/// The idealized candidate set: brute-force filter, input order.
fn brute(boxes: &[Aabb], query: &Aabb) -> Vec<usize> {
    boxes
        .iter()
        .enumerate()
        .filter(|(_, b)| b.overlaps(query))
        .map(|(i, _)| i)
        .collect()
}

fn arb_box() -> impl Strategy<Value = Aabb> {
    let c = -100.0..100.0f64;
    let e = 0.0..50.0f64;
    ((c.clone(), c.clone(), c), (e.clone(), e.clone(), e))
        .prop_map(|((x, y, z), (a, b, c))| boxed([x - a, y - b, z - c], [x + a, y + b, z + c]))
}

proptest! {
    /// Realized (tree) == idealized (brute force), set AND order.
    #[test]
    fn tree_query_equals_brute_force(
        boxes in prop::collection::vec(arb_box(), 0..80),
        query in arb_box(),
    ) {
        let tree = Bvh::build(&boxes);
        prop_assert_eq!(tree.overlapping(&query), brute(&boxes, &query));
    }

    /// Same input ⇒ bit-identical tree (Debug form pins arena layout,
    /// split choices, and permutation — the D9 build determinism).
    #[test]
    fn build_is_deterministic(boxes in prop::collection::vec(arb_box(), 0..80)) {
        let a = Bvh::build(&boxes);
        let b = Bvh::build(&boxes);
        prop_assert_eq!(format!("{a:?}"), format!("{b:?}"));
    }
}

#[test]
fn empty_tree_answers_empty() {
    let tree = Bvh::build(&[]);
    assert!(tree.is_empty());
    assert_eq!(tree.overlapping(&Aabb::poison()), Vec::<usize>::new());
}

#[test]
fn poison_item_is_never_pruned() {
    let mut boxes = vec![boxed([0.0; 3], [1.0; 3]); 20];
    boxes[7] = Aabb::poison();
    let tree = Bvh::build(&boxes);
    // A query far away from every real box still returns the poison
    // item: NaN can never witness disjointness.
    let far = boxed([500.0; 3], [501.0; 3]);
    assert_eq!(tree.overlapping(&far), vec![7]);
}

#[test]
fn poison_query_returns_everything() {
    let boxes = vec![boxed([0.0; 3], [1.0; 3]); 13];
    let tree = Bvh::build(&boxes);
    assert_eq!(
        tree.overlapping(&Aabb::poison()),
        (0..13).collect::<Vec<_>>()
    );
}

#[test]
fn touching_boxes_overlap_and_identical_centroids_split_by_index() {
    // 20 identical boxes: every split falls back to the input-index
    // tie-break; the query must still return all of them, ascending.
    let boxes = vec![boxed([0.0; 3], [1.0; 3]); 20];
    let tree = Bvh::build(&boxes);
    // Touching at a corner counts as overlap (closed boxes).
    let corner = boxed([1.0; 3], [2.0; 3]);
    assert_eq!(tree.overlapping(&corner), (0..20).collect::<Vec<_>>());
}

#[test]
fn padded_grows_strictly_and_never_shrinks() {
    let b = boxed([0.0; 3], [1.0; 3]);
    let p = b.padded(0.5);
    assert!(p.min_x <= -0.5 && p.min_x > -0.5 - 1e-12);
    assert!(p.max_x >= 1.5 && p.max_x < 1.5 + 1e-12);
    // Conservative-only: a negative pad poisons rather than shrinks.
    let neg = b.padded(-0.1);
    assert!(neg.min_x.is_nan());
    assert!(neg.overlaps(&boxed([900.0; 3], [901.0; 3])));
}

/// **Two inputs the 80-item property rows do not reach**, both asserted
/// against the same brute-force oracle above.
///
/// Neither row pins the SPLIT RULE — nothing can, from outside: a
/// query re-tests the exact item box at every leaf it reaches, so any
/// partition of the range answers the same candidate set, and a
/// changed rule shows up as tree shape and cost, not as a wrong
/// answer. What these two DO pin is the thing a partition can break
/// and a sort cannot: the permutation. A leaf range that lost an
/// item, double-counted one, or ran off its `start .. start + count`
/// window is invisible at 80 items and on distinct centroids, and is
/// exactly what the universe query below would show.
///
/// Every box here is centred on the origin, so the centroid-bounds
/// extent is zero on all three axes and the centroid term ties for
/// every pair — the input-index tie-break is the whole of the ranking.
#[test]
fn co_located_centroids_keep_the_whole_permutation() {
    let boxes: Vec<Aabb> = (0..200)
        .map(|i| {
            let e = 1.0 + f64::from(i);
            boxed([-e, -e, -e], [e, e, e])
        })
        .collect();
    let tree = Bvh::build(&boxes);
    // Inside every box, inside the outermost few only, disjoint from
    // all of them.
    for query in [
        boxed([-0.5, -0.5, -0.5], [0.5, 0.5, 0.5]),
        boxed([190.0, 0.0, 0.0], [190.5, 0.5, 0.5]),
        boxed(
            [1.0e6, 1.0e6, 1.0e6],
            [1.0e6 + 1.0, 1.0e6 + 1.0, 1.0e6 + 1.0],
        ),
    ] {
        assert_eq!(
            tree.overlapping(&query),
            brute(&boxes, &query),
            "the tree and brute force disagree on {query:?}"
        );
    }
}

/// The same equality at a depth the 80-item rows never reach: 20 000
/// items is ~11 levels of partitioning, and the second query covers
/// every box, so it reads back the entire permutation in ascending
/// input order.
#[test]
fn a_deep_tree_answers_the_brute_force_set_and_its_whole_permutation() {
    // A deterministic spread, no rng: the three axes get different
    // extents so the split axis actually moves between levels.
    let boxes: Vec<Aabb> = (0..20_000u32)
        .map(|i| {
            let t = f64::from(i);
            let c = [
                t * 0.618_034 % 100.0,
                t * 0.414_214 % 60.0,
                t * 0.302_775 % 30.0,
            ];
            let e = 0.05 + (t * 0.1 % 0.5);
            boxed(
                [c[0] - e, c[1] - e, c[2] - e],
                [c[0] + e, c[1] + e, c[2] + e],
            )
        })
        .collect();
    let tree = Bvh::build(&boxes);
    for query in [
        boxed([10.0, 10.0, 10.0], [12.0, 12.0, 12.0]),
        boxed([-1.0, -1.0, -1.0], [101.0, 61.0, 31.0]),
        boxed([500.0, 500.0, 500.0], [501.0, 501.0, 501.0]),
    ] {
        assert_eq!(tree.overlapping(&query), brute(&boxes, &query));
    }
    let everything = boxed([-1.0, -1.0, -1.0], [101.0, 61.0, 31.0]);
    assert_eq!(tree.overlapping(&everything).len(), boxes.len());
}
