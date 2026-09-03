//! The proximity lane: [`Aabb::separation_lo`] as a certified lower
//! bound, and the three `within` queries as conservative supersets of
//! the brute-force answer.
//!
//! **What this suite instantiates, and what it cannot.** The
//! `T: Bounds` construction door is exercised HERE at `T = f64` only:
//! `Bounds: Real`, so a bracket scalar cannot be minted in a test crate
//! without an entire `Real` implementation, and the crate's own
//! dependency carries no interval feature. The certified-scalar
//! instantiation — the one the door exists for — is pinned where the
//! scalar lives, in `editor-core`'s clearance suite, against the same
//! doors. What is checked here is everything scalar-independent: the
//! box arithmetic's direction, the queries' conservativeness against
//! brute force, and determinism.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use bvh::{Aabb, Bvh};
use geom_core::Point3;
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

fn arb_box() -> impl Strategy<Value = Aabb> {
    let c = -20.0..20.0f64;
    let e = 0.0..5.0f64;
    ((c.clone(), c.clone(), c), (e.clone(), e.clone(), e))
        .prop_map(|((x, y, z), (a, b, c))| boxed([x - a, y - b, z - c], [x + a, y + b, z + c]))
}

/// A point of the box at the three unit-interval coordinates.
fn point_in(b: &Aabb, (s, t, r): (f64, f64, f64)) -> [f64; 3] {
    [
        b.min_x + s * (b.max_x - b.min_x),
        b.min_y + t * (b.max_y - b.min_y),
        b.min_z + r * (b.max_z - b.min_z),
    ]
}

fn dist(p: [f64; 3], q: [f64; 3]) -> f64 {
    let d = [p[0] - q[0], p[1] - q[1], p[2] - q[2]];
    d[0].mul_add(d[0], d[1].mul_add(d[1], d[2] * d[2])).sqrt()
}

// The sweep's blind spot, stated once for every property below: the
// generator makes finite, non-inverted, axis-aligned boxes of bounded
// extent, so it never lands a degenerate, inverted or poison box. Those
// are the table tests that follow.
proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// The whole claim of `Aabb::separation_lo`: no pair of points, one
    /// from each box, is closer than the reported bound.
    #[test]
    fn separation_lo_never_exceeds_a_real_separation(
        a in arb_box(),
        b in arb_box(),
        pa in (0.0..1.0f64, 0.0..1.0f64, 0.0..1.0f64),
        pb in (0.0..1.0f64, 0.0..1.0f64, 0.0..1.0f64),
    ) {
        let lo = a.separation_lo(&b);
        prop_assert!(lo >= 0.0, "a separation is never negative: {lo}");
        prop_assert_eq!(lo, b.separation_lo(&a), "the bound is symmetric");
        let d = dist(point_in(&a, pa), point_in(&b, pb));
        prop_assert!(
            lo <= d,
            "separation_lo {lo} exceeds a real point pair's distance {d}"
        );
    }

    /// The tree's proximity query against brute force, on the same
    /// boxes: exactly the items whose own bound says they are within
    /// `pad`, in ascending order. The tree's node pruning must change
    /// which boxes are EXAMINED and nothing else.
    #[test]
    fn within_reproduces_the_brute_force_set(
        boxes in prop::collection::vec(arb_box(), 1..40),
        query in arb_box(),
        pad in 0.0..3.0f64,
    ) {
        let tree = Bvh::build(&boxes);
        let brute: Vec<usize> = boxes
            .iter()
            .enumerate()
            .filter(|(_, b)| b.separation_lo(&query) <= pad)
            .map(|(i, _)| i)
            .collect();
        prop_assert_eq!(tree.within(&query, pad), brute);
    }

    /// Conservativeness, the property the clearance lane rests on: a
    /// pair whose true separation is at most `pad` is always a
    /// candidate.
    #[test]
    fn pairs_within_never_drops_a_close_pair(
        left in prop::collection::vec(arb_box(), 1..12),
        right in prop::collection::vec(arb_box(), 1..12),
        pad in 0.0..3.0f64,
    ) {
        let (a, b) = (Bvh::build(&left), Bvh::build(&right));
        let pairs = a.pairs_within(&b, pad);
        for (i, x) in left.iter().enumerate() {
            for (j, y) in right.iter().enumerate() {
                if x.separation_lo(y) <= pad {
                    prop_assert!(
                        pairs.contains(&(i, j)),
                        "pair ({i}, {j}) is within {pad} and was not a candidate"
                    );
                }
            }
        }
        let mut sorted = pairs.clone();
        sorted.sort_unstable();
        prop_assert_eq!(pairs, sorted, "candidates are ascending by construction");
    }
}

/// The cases the generator cannot reach, stated as a table: touching,
/// nested, inverted (empty) and poison boxes.
#[test]
fn the_degenerate_boxes_answer_zero() {
    let unit = boxed([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
    let touching = boxed([1.0, 0.0, 0.0], [2.0, 1.0, 1.0]);
    let nested = boxed([0.25, 0.25, 0.25], [0.5, 0.5, 0.5]);
    let inverted = boxed([1.0, 1.0, 1.0], [0.0, 0.0, 0.0]);
    for other in [touching, nested, inverted, Aabb::poison()] {
        assert_eq!(
            unit.separation_lo(&other),
            0.0,
            "a box this cannot separate answers zero, never a claim"
        );
        assert_eq!(other.separation_lo(&unit), 0.0);
    }
    // A poison box is a candidate of every proximity query, at every
    // pad — the crate's fail-safe direction.
    let far = boxed([99.0, 99.0, 99.0], [100.0, 100.0, 100.0]);
    let tree = Bvh::build(&[Aabb::poison(), unit]);
    assert_eq!(tree.within(&touching, 0.0), vec![0, 1]);
    assert_eq!(tree.within(&far, 0.0), vec![0]);
    // A pad that is not a distance answers everything rather than
    // pruning the tree bare.
    assert_eq!(tree.within(&far, -1.0), vec![0, 1]);
    assert_eq!(tree.within(&far, f64::NAN), vec![0, 1]);
}

/// A separation the bound must actually SEE: two unit boxes a metre
/// apart on each axis are `√3` apart, and the bound is within a few
/// ulps below it.
#[test]
fn separation_lo_is_tight_to_a_few_ulps() {
    let a = boxed([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
    let b = boxed([2.0, 2.0, 2.0], [3.0, 3.0, 3.0]);
    let truth = 3.0f64.sqrt();
    let lo = a.separation_lo(&b);
    assert!(lo <= truth, "the bound never exceeds the truth: {lo} vs {truth}");
    assert!(
        truth - lo <= 8.0 * f64::EPSILON,
        "the bound is tight, not merely sound: {lo} vs {truth}"
    );
}

/// `self_pairs_within` reports each unordered pair once, never a
/// self-pair, in ascending order.
#[test]
fn self_pairs_are_unordered_and_reported_once() {
    let boxes = [
        boxed([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]),
        boxed([0.5, 0.0, 0.0], [1.5, 1.0, 1.0]),
        boxed([9.0, 9.0, 9.0], [10.0, 10.0, 10.0]),
    ];
    let tree = Bvh::build(&boxes);
    assert_eq!(tree.self_pairs_within(0.0), vec![(0, 1)]);
    // The far box is `8√3 ≈ 13.86` away on the diagonal, not 8: the
    // query's reach is Euclidean, not per-axis. A pad that clears it
    // pairs everything, still once each and still ascending.
    assert_eq!(tree.self_pairs_within(13.0), vec![(0, 1)]);
    assert_eq!(tree.self_pairs_within(14.0), vec![(0, 1), (0, 2), (1, 2)]);
}

/// The `T: Bounds` door at `T = f64` is the vertex-extent constructor
/// it always was: same boxes, same tree, same `Debug` form.
#[test]
fn build_bounded_at_f64_is_the_existing_construction() {
    let clouds = vec![
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 2.0, 3.0),
            Point3::new(-1.0, 0.5, 1.0),
        ],
        vec![Point3::new(4.0, 4.0, 4.0)],
    ];
    let pad = 0.125;
    let by_hand: Vec<Aabb> = clouds
        .iter()
        .map(|c| Aabb::from_points(c.iter().copied()).unwrap().padded(pad))
        .collect();
    let door = Bvh::build_bounded(clouds.clone(), pad);
    assert_eq!(door.boxes(), by_hand.as_slice());
    assert_eq!(format!("{door:?}"), format!("{:?}", Bvh::build(&by_hand)));
}

/// An item nothing described is the poison box, not a dropped item:
/// the arena index every query answers in is the input index, so an
/// empty cloud must still occupy its slot.
#[test]
fn an_empty_cloud_is_a_poison_item_not_a_missing_one() {
    let clouds: Vec<Vec<Point3<f64>>> = vec![
        vec![Point3::new(0.0, 0.0, 0.0)],
        vec![],
        vec![Point3::new(5.0, 5.0, 5.0)],
    ];
    let tree = Bvh::build_bounded(clouds, 0.0);
    assert_eq!(tree.len(), 3);
    let far = boxed([100.0, 100.0, 100.0], [101.0, 101.0, 101.0]);
    assert_eq!(tree.within(&far, 0.0), vec![1], "poison never prunes");
}

/// Determinism (D9): the same items build the same tree and answer the
/// same candidates, and the answer does not leak tree shape — a query
/// answered over a permuted build carries the permuted indices and
/// nothing else.
#[test]
fn the_proximity_queries_are_deterministic() {
    let boxes: Vec<Aabb> = (0..37)
        .map(|i| {
            let f = f64::from(i);
            boxed([f * 0.5, 0.0, 0.0], [f * 0.5 + 0.75, 1.0, 1.0])
        })
        .collect();
    let tree = Bvh::build(&boxes);
    let first = tree.self_pairs_within(0.25);
    for _ in 0..4 {
        assert_eq!(Bvh::build(&boxes).self_pairs_within(0.25), first);
    }
    let reversed: Vec<Aabb> = boxes.iter().rev().copied().collect();
    let rev_tree = Bvh::build(&reversed);
    let n = boxes.len() - 1;
    let mut mapped: Vec<(usize, usize)> = first
        .iter()
        .map(|&(i, j)| {
            let (a, b) = (n - i, n - j);
            (a.min(b), a.max(b))
        })
        .collect();
    mapped.sort_unstable();
    assert_eq!(rev_tree.self_pairs_within(0.25), mapped);
}
