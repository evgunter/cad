//! `Bvh::ray` (GUI-1 Part A): the IEEE slab-test corners pinned one by
//! one, the documented candidate order, determinism, and a randomized
//! sweep holding the conservative-superset contract two ways —
//! realized (tree) == idealized (per-item slab test, same set and
//! order), and a constructed TRUE intersection is never dropped.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to!["crates/bvh/src/", "crates/geom-core/src/linalg/"];

use bvh::{Aabb, Bvh, Ray, RayCandidate};
use geom_core::{Point3, Vec3};
use test_utils::fuzz;

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

fn ray(origin: [f64; 3], dir: [f64; 3]) -> Ray {
    Ray {
        origin: Point3::new(origin[0], origin[1], origin[2]),
        dir: Vec3::new(dir[0], dir[1], dir[2]),
    }
}

fn items(cands: &[RayCandidate]) -> Vec<usize> {
    cands.iter().map(|c| c.item).collect()
}

/// The historic `0 × ∞ → NaN` trap input: a zero direction component
/// with the origin exactly ON the slab bound. The ray lies in the
/// closed boundary plane, so the box must stay a candidate (the exact
/// `d = 0` arm reads it as inside — no product is ever formed), and
/// the other axes still meter the entry.
#[test]
fn zero_direction_on_slab_boundary_is_a_candidate() {
    let tree = Bvh::build(&[boxed([0.0; 3], [1.0; 3])]);
    // dir.x = 0, origin.x == min_x = 0: on the closed bound = inside.
    let r = ray([0.0, 0.5, -1.0], [0.0, 0.0, 1.0]);
    let out = tree.ray(&r);
    assert_eq!(items(&out), vec![0]);
    // Entry is metered by z: exact 1.0, reported within the documented
    // 4-ULP-per-endpoint conservative widening (and never above it).
    assert!(
        out[0].t_enter <= 1.0 && out[0].t_enter > 1.0 - 1e-12,
        "t_enter {}",
        out[0].t_enter
    );
}

/// A zero direction component with the origin strictly OUTSIDE the
/// slab: the exact `d = 0` comparison arm prunes, and it prunes on
/// BOTH sides — above the slab and below it. (The R2 review caught
/// the pre-fix arithmetic pruning only the above side; both halves
/// are pinned here.)
#[test]
fn zero_direction_outside_slab_prunes() {
    let tree = Bvh::build(&[boxed([0.0; 3], [1.0; 3])]);
    // Above the slab (o > hi).
    let r = ray([2.0, 0.5, -1.0], [0.0, 0.0, 1.0]);
    assert_eq!(tree.ray(&r), Vec::<RayCandidate>::new());
    // Below the slab (o < lo) — the side the pre-fix code kept.
    let r = ray([-2.0, 0.5, -1.0], [0.0, 0.0, 1.0]);
    assert_eq!(tree.ray(&r), Vec::<RayCandidate>::new());
    // Both sides even with NO other axis constraining (a zero-length
    // point ray strictly outside the slab).
    let r = ray([-2.0, 0.5, 0.5], [0.0, 0.0, 0.0]);
    assert_eq!(tree.ray(&r), Vec::<RayCandidate>::new());
    let r = ray([2.0, 0.5, 0.5], [0.0, 0.0, 0.0]);
    assert_eq!(tree.ray(&r), Vec::<RayCandidate>::new());
}

/// The reciprocal-overflow hole (R2's second witness class): a
/// SUBNORMAL direction component used to make `1/d = ∞` and mint a
/// fake-infinite endpoint from a moderate true quotient, pruning a
/// truly hit box. The division spelling computes the true quotient
/// correctly rounded: the box is a candidate with an honest entry.
#[test]
fn subnormal_direction_does_not_prune_a_true_hit() {
    // x: bounds subnormal, d subnormal — true t ∈ [1, 2] exactly.
    let b = boxed([1e-315, -1.0, -1.0], [2e-315, 5.0, 1.0]);
    let tree = Bvh::build(&[b]);
    let r = ray([0.0, 0.5, 0.0], [1e-315, 1.0, 0.0]);
    let out = tree.ray(&r);
    assert_eq!(items(&out), vec![0]);
    assert!(
        out[0].t_enter <= 1.0 && out[0].t_enter > 0.9,
        "t_enter {} lower-bounds the true entry 1.0",
        out[0].t_enter
    );
}

/// An entry GENUINELY beyond `f64` range (the division overflows on a
/// true quotient > MAX): the box stays a candidate and `t_enter` is
/// the finite ≈`MAX` clamp — never `+∞` (the documented lower-bound
/// and finiteness claims at their extreme).
#[test]
fn entry_beyond_f64_range_stays_a_candidate_with_finite_t_enter() {
    let b = boxed([1e300, -1.0, -1.0], [1.5e300, 1.0, 1.0]);
    let tree = Bvh::build(&[b]);
    // True t ≈ [1e330, 1.5e330]: beyond MAX, still a real intersection.
    let r = ray([0.0, 0.0, 0.0], [1e-30, 0.0, 0.0]);
    let out = tree.ray(&r);
    assert_eq!(items(&out), vec![0]);
    assert!(out[0].t_enter.is_finite(), "t_enter is never +∞");
    assert!(
        out[0].t_enter > 1e308,
        "the clamp is ≈ MAX, a valid lower bound"
    );
}

/// A negative-zero direction component behaves like positive zero
/// (`−0.0 == 0.0` takes the exact arm): on-boundary stays a candidate.
#[test]
fn negative_zero_direction_on_boundary_is_a_candidate() {
    let tree = Bvh::build(&[boxed([0.0; 3], [1.0; 3])]);
    let r = ray([0.0, 0.5, -1.0], [-0.0, 0.0, 1.0]);
    assert_eq!(items(&tree.ray(&r)), vec![0]);
}

/// A ray originating inside a box enters at exactly `t = 0` (the fold
/// starts at the ray's own domain floor).
#[test]
fn origin_inside_box_enters_at_zero() {
    let tree = Bvh::build(&[boxed([0.0; 3], [1.0; 3])]);
    let r = ray([0.5, 0.5, 0.5], [1.0, 0.0, 0.0]);
    let out = tree.ray(&r);
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].t_enter, 0.0);
}

/// A zero-extent axis (a plane box) is still hittable: the slab
/// degenerates to `near == far` before the outward widening.
#[test]
fn zero_extent_box_is_hittable() {
    let tree = Bvh::build(&[boxed([0.0, 0.0, 0.5], [1.0, 1.0, 0.5])]);
    let r = ray([0.5, 0.5, -1.0], [0.0, 0.0, 1.0]);
    let out = tree.ray(&r);
    assert_eq!(items(&out), vec![0]);
    assert!(
        out[0].t_enter <= 1.5 && out[0].t_enter > 1.5 - 1e-12,
        "t_enter {}",
        out[0].t_enter
    );
}

/// Poison boxes are always candidates (NaN never witnesses
/// disjointness), entering at the domain floor `0`.
#[test]
fn poison_box_is_always_a_candidate() {
    let mut boxes = vec![boxed([0.0; 3], [1.0; 3]); 20];
    boxes[7] = Aabb::poison();
    let tree = Bvh::build(&boxes);
    // A ray pointing away from every real box still returns the
    // poison item.
    let r = ray([500.0, 500.0, 500.0], [1.0, 0.0, 0.0]);
    let out = tree.ray(&r);
    assert_eq!(items(&out), vec![7]);
    assert_eq!(out[0].t_enter, 0.0);
}

/// A poison ray (NaN direction) prunes nothing — every box is a
/// candidate, in the documented (t_enter = 0, so index-ascending)
/// order.
#[test]
fn poison_ray_returns_everything() {
    let boxes = vec![boxed([0.0; 3], [1.0; 3]); 5];
    let tree = Bvh::build(&boxes);
    let r = ray([0.0; 3], [f64::NAN, 0.0, 1.0]);
    assert_eq!(items(&tree.ray(&r)), vec![0, 1, 2, 3, 4]);
}

/// A box entirely behind the origin is pruned (its exit parameter is
/// definitely negative — disjoint from the `t ≥ 0` domain). This pins
/// pruning QUALITY; the contract itself would also allow keeping it.
#[test]
fn box_behind_origin_is_pruned() {
    let tree = Bvh::build(&[boxed([-2.0; 3], [-1.0; 3])]);
    let r = ray([0.0; 3], [1.0, 1.0, 1.0]);
    assert_eq!(tree.ray(&r), Vec::<RayCandidate>::new());
}

/// The documented candidate order: ascending `t_enter`, ties broken by
/// ascending input index. Direction length scales `t_enter` (no hidden
/// normalization).
#[test]
fn candidates_order_by_entry_then_index() {
    let boxes = vec![
        boxed([1.0, -1.0, -1.0], [2.0, 1.0, 1.0]),
        boxed([3.0, -1.0, -1.0], [4.0, 1.0, 1.0]),
        boxed([1.0, -1.0, -1.0], [2.0, 1.0, 1.0]), // duplicate of 0: a tie
    ];
    let tree = Bvh::build(&boxes);
    let out = tree.ray(&ray([0.0; 3], [1.0, 0.0, 0.0]));
    assert_eq!(items(&out), vec![0, 2, 1]);
    assert_eq!(
        out[0].t_enter, out[1].t_enter,
        "duplicate boxes tie exactly"
    );
    // Unnormalized direction: doubling |dir| halves every t_enter.
    let out2 = tree.ray(&ray([0.0; 3], [2.0, 0.0, 0.0]));
    assert_eq!(items(&out2), vec![0, 2, 1]);
    assert!(out2[0].t_enter <= 0.5 && out2[0].t_enter > 0.5 - 1e-12);
}

/// The idealized ray candidate set: per-item slab test, sorted by the
/// documented order. `Bvh::ray` must reproduce it EXACTLY — which also
/// pins that tree shape never leaks into the result.
fn brute(boxes: &[Aabb], r: &Ray) -> Vec<RayCandidate> {
    let mut out: Vec<RayCandidate> = boxes
        .iter()
        .enumerate()
        .filter_map(|(item, b)| {
            r.slab_enter(b)
                .map(|t_enter| RayCandidate { item, t_enter })
        })
        .collect();
    // The documented order, restated INDEPENDENTLY of `Bvh::ray`'s
    // own comparator on purpose: sharing one comparator would blind
    // this oracle to a tie-break regression (both sides would change
    // together). Keep the duplication.
    out.sort_unstable_by(|a, b| a.t_enter.total_cmp(&b.t_enter).then(a.item.cmp(&b.item)));
    out
}

/// Randomized sweep (shape: counterexample search — varying seed,
/// counts on the effort dial, per `memories/test-suite-cost.md`).
/// Three properties per draw:
///
/// 1. realized == idealized (set AND order) — [`brute`];
/// 2. determinism: the same query twice is bit-identical;
/// 3. conservative superset against TRUE geometry: a ray constructed
///    through a point inside a chosen box (`t = 1` lands inside) must
///    list that box among its candidates.
#[test]
fn sweep_matches_brute_force_and_never_misses_true_hits() {
    let mut rng = fuzz::start("bvh::ray conservative-superset sweep");
    let mut exposure = test_utils::vacuity::Exposure::new("bvh::ray sweep");
    for case in 0..fuzz::scaled(60) {
        let n = rng.below(40) + 1;
        let mut boxes = Vec::with_capacity(n);
        for _ in 0..n {
            let c = [
                rng.range(-100.0, 100.0),
                rng.range(-100.0, 100.0),
                rng.range(-100.0, 100.0),
            ];
            // Zero extent on an axis with probability ~1/4 each.
            let e = [
                if rng.below(4) == 0 {
                    0.0
                } else {
                    rng.range(0.0, 50.0)
                },
                if rng.below(4) == 0 {
                    0.0
                } else {
                    rng.range(0.0, 50.0)
                },
                if rng.below(4) == 0 {
                    0.0
                } else {
                    rng.range(0.0, 50.0)
                },
            ];
            boxes.push(boxed(
                [c[0] - e[0], c[1] - e[1], c[2] - e[2]],
                [c[0] + e[0], c[1] + e[1], c[2] + e[2]],
            ));
        }
        // A poison item with probability ~1/4.
        if rng.below(4) == 0 {
            let k = rng.below(n);
            boxes[k] = Aabb::poison();
        }
        let tree = Bvh::build(&boxes);

        let origin = [
            rng.range(-200.0, 200.0),
            rng.range(-200.0, 200.0),
            rng.range(-200.0, 200.0),
        ];
        // The true-hit construction: a point inside a chosen box,
        // reached at t = 1 (poison boxes have no inside; skip those).
        let target = rng.below(n);
        let b = boxes[target];
        // A poison target has no inside point: the constructed ray
        // then carries NaN (a poison ray — itself a corner worth
        // sweeping), and only the true-hit assertion is skipped.
        let target_is_real = !b.min_x.is_nan();
        let inside = [
            rng.range(0.0, 1.0).mul_add(b.max_x - b.min_x, b.min_x),
            rng.range(0.0, 1.0).mul_add(b.max_y - b.min_y, b.min_y),
            rng.range(0.0, 1.0).mul_add(b.max_z - b.min_z, b.min_z),
        ];
        let mut dir = [
            inside[0] - origin[0],
            inside[1] - origin[1],
            inside[2] - origin[2],
        ];
        // Zero direction components (the corner under test) with
        // probability ~1/8 each — the target stays truly hit only if
        // the origin already shares that coordinate, so re-aim the
        // origin onto the inside point's coordinate when snapping.
        let mut origin = origin;
        for a in 0..3 {
            if rng.below(8) == 0 {
                dir[a] = 0.0;
                origin[a] = inside[a];
            }
        }
        let r = ray(origin, dir);

        let got = tree.ray(&r);
        let want = brute(&boxes, &r);
        assert_eq!(
            got,
            want,
            "case {case}: realized == idealized; {}",
            fuzz::replay()
        );
        assert_eq!(
            got,
            tree.ray(&r),
            "case {case}: determinism; {}",
            fuzz::replay()
        );
        // A zero direction is legal input (the ray is a point; the
        // target box contains it, closed) — the exact d = 0 arm reads
        // the point as inside, so the true-hit claim holds there too.
        if target_is_real {
            exposure.note("true-hit case");
            assert!(
                got.iter().any(|c| c.item == target),
                "case {case}: true hit of box {target} dropped ({r:?}, {b:?}); {}",
                fuzz::replay()
            );
        }
        if !got.is_empty() {
            exposure.note("nonempty candidate set");
        }
    }
    exposure.report();
    // Anti-vacuity floors (memories/test-suite-cost + test_utils::vacuity):
    // stated against the effort-1 floor of the dial (60 cases; a target
    // is poison — skipping the true-hit assertion — with probability
    // 1/(4n) ≤ 1/4 per case), so a run below these floors did not
    // exercise the contract, not merely got unlucky.
    exposure.require(
        "true-hit case",
        16,
        "the conservative-superset claim needs constructed true hits to bite",
    );
    exposure.require(
        "nonempty candidate set",
        16,
        "the realized == idealized row needs nonempty sets to compare",
    );
}
