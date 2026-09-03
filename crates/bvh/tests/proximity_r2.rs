//! R2's independent probes of the M10-5 proximity lane
//! (`Aabb::separation_lo` and the three `within` queries).
//!
//! What the shipped `proximity.rs` sweep cannot reach, stated as its own
//! generator states it: `arb_box` makes finite boxes whose coordinates
//! live inside ±25 m, and the table row beside it covers touching,
//! nested, inverted and poison. NOTHING in that suite lands a box whose
//! per-axis gap is large enough to overflow `norm_squared`, and that is
//! where the "certified LOWER bound" claim is falsifiable.
//!
//! Every row here is a claim about the crate's own contract, derived
//! from the doc-comment on `Aabb::separation_lo` rather than from the
//! shipped assertions.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use bvh::{Aabb, Bvh};

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

/// The exact separation of two axis-aligned boxes, computed in a way
/// that cannot overflow: the per-axis gaps are scaled down by the
/// largest of them before the norm, then scaled back.
fn separation_truth(a: &Aabb, b: &Aabb) -> f64 {
    let gap = |a_min: f64, a_max: f64, b_min: f64, b_max: f64| {
        (b_min - a_max).max(a_min - b_max).max(0.0)
    };
    let g = [
        gap(a.min_x, a.max_x, b.min_x, b.max_x),
        gap(a.min_y, a.max_y, b.min_y, b.max_y),
        gap(a.min_z, a.max_z, b.min_z, b.max_z),
    ];
    let s = g[0].max(g[1]).max(g[2]);
    if s == 0.0 {
        return 0.0;
    }
    let n = (g[0] / s, g[1] / s, g[2] / s);
    s * (n.0 * n.0 + n.1 * n.1 + n.2 * n.2).sqrt()
}

/// **The falsification**: at coordinates past `~1.34e154` the per-axis
/// gaps square to infinity inside `Vec3::norm`, the norm comes back
/// `inf`, and four `next_down`s off infinity land on `f64::MAX`. The
/// "certified LOWER bound on the Euclidean distance" is then an
/// enormous OVER-claim — `1.8e308` reported for boxes `1.7e200` apart.
///
/// `Vec3::norm`'s own doc-comment states the overflow ("components
/// beyond ~1e154 overflow `norm_squared` to ∞"); what is missing is a
/// guard at the one call site that promises a bound in the other
/// direction.
#[test]
fn separation_lo_overclaims_when_the_gap_overflows_the_norm() {
    let big = 1.0e200f64;
    let a = boxed([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
    let b = boxed([big, big, big], [big * 2.0, big * 2.0, big * 2.0]);
    let truth = separation_truth(&a, &b);
    let lo = a.separation_lo(&b);
    assert!(
        truth.is_finite() && truth > 1.0e200,
        "the fixture really is ~1.7e200 apart: {truth}"
    );
    assert!(
        lo > 1.0e308,
        "the shipped bound saturates (four `next_down`s off infinity) rather than \
         bounding: {lo}"
    );
    assert!(
        lo > truth,
        "R2 finding: separation_lo {lo} EXCEEDS the real separation {truth}, so it is not \
         a lower bound at this magnitude"
    );
}

/// The consequence at the query door, which is what makes the row above
/// more than an arithmetic curiosity: a pair whose true separation is
/// well inside `pad` is pruned away, so a consumer that reads
/// `pairs_within` as "every pair that could be within `pad`" is handed
/// an answer that silently dropped one.
#[test]
fn the_overflowing_pair_is_dropped_from_a_proximity_query() {
    let big = 1.0e200f64;
    let a = boxed([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
    let b = boxed([big, big, big], [big * 2.0, big * 2.0, big * 2.0]);
    let pad = 1.0e250; // far larger than the true 1.7e200 separation
    assert!(
        separation_truth(&a, &b) < pad,
        "the pair really is within the pad"
    );
    let left = Bvh::build(&[a]);
    let right = Bvh::build(&[b]);
    assert_eq!(
        left.pairs_within(&right, pad),
        Vec::<(usize, usize)>::new(),
        "R2 finding: a pair within the pad is not a candidate"
    );
}

/// Where the saturation actually begins, so the finding carries its own
/// threshold rather than one chosen magnitude: `norm_squared` overflows
/// once a single gap exceeds `sqrt(f64::MAX)`.
#[test]
fn the_overclaim_threshold_is_the_square_root_of_max() {
    let root = f64::MAX.sqrt(); // ~1.34e154
    let below = boxed([root * 0.5, 0.0, 0.0], [root * 0.5, 0.0, 0.0]);
    let above = boxed([root * 1.5, 0.0, 0.0], [root * 1.5, 0.0, 0.0]);
    let origin = boxed([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
    assert!(
        origin.separation_lo(&below).is_finite() && origin.separation_lo(&below) < root,
        "below the threshold the bound is sound"
    );
    assert!(
        origin.separation_lo(&above) > 1.0e308,
        "above it every separation saturates near f64::MAX: {}",
        origin.separation_lo(&above)
    );
}

/// The stated ulp budget, re-derived. The doc-comment argues "under 2.6
/// ulps" from three squares and two sums — it omits the rounding of the
/// three SUBTRACTIONS that produce the gaps, each of which is a further
/// half-ulp of relative error carried into a square. The honest budget
/// is ~3.5 ulps against a 4-ulp shave, which still holds; the row exists
/// so the margin is measured rather than asserted.
///
/// This is evidence-only: it measures the shipped arithmetic on a table
/// of separations and reports the worst shortfall observed, and it
/// gates only on the direction (never above the truth).
#[test]
fn the_four_ulp_shave_is_measured_against_the_truth() {
    let mut worst_ulps = 0.0f64;
    for &(ax, bx) in &[
        (1.0f64, 2.0f64),
        (0.0, 1.0e-8),
        (-3.5, 3.5),
        (1.0e8, 1.0e8 + 7.0),
        (0.0, 1.0),
        (0.0, 1.0 + f64::EPSILON),
    ] {
        for scale in [1.0f64, 1.0e-6, 1.0e6, 1.0e12] {
            let a = boxed([ax * scale, ax * scale, ax * scale], [ax * scale; 3]);
            let b = boxed([bx * scale, bx * scale, bx * scale], [bx * scale; 3]);
            let lo = a.separation_lo(&b);
            let truth = separation_truth(&a, &b);
            assert!(
                lo <= truth,
                "the bound must never exceed the truth: {lo} vs {truth} at scale {scale}"
            );
            if truth > 0.0 {
                let ulp = truth.next_up() - truth;
                worst_ulps = worst_ulps.max((truth - lo) / ulp);
            }
        }
    }
    println!("[r2] worst observed shortfall: {worst_ulps} ulps of the truth");
}

/// **Partial poison: sound, but not for the reason its comment gives.**
///
/// The comment at the site reads: *"Spelled rather than left to the
/// fold: `f64::max` IGNORES NaN, so a poison bound would come out of it
/// as the other operand and silently claim a separation."* The code
/// under it is `let g = (b_min - a_max).max(a_min - b_max);` — the very
/// `f64::max` the comment names — and the `g.is_nan()` guard runs AFTER
/// it, so it cannot prevent what the comment says it prevents. The
/// guard fires only when BOTH differences are NaN, i.e. on the all-NaN
/// `Aabb::poison()` the shipped table happens to test.
///
/// R2 chased that as a soundness hole and it is NOT one, for a reason
/// the comment does not give: at most one of the two candidate gaps can
/// be positive, and each is computed from a DIFFERENT pair of bounds,
/// so a non-NaN survivor is always a separation the surviving bounds
/// justify on their own. The four partial-poison shapes are walked
/// below and the direction holds every time.
///
/// The finding is therefore about the comment: it claims a mechanism
/// the code does not have, and the argument that makes the code correct
/// is written nowhere.
#[test]
fn partial_poison_stays_sound_for_a_reason_the_comment_does_not_give() {
    let far = boxed([99.0, 99.0, 99.0], [100.0, 100.0, 100.0]);
    // Both x bounds NaN, y and z overlapping `far`: the guard fires on
    // x and the other axes contribute nothing.
    let overlapping_poison = boxed([f64::NAN, 99.0, 99.0], [f64::NAN, 100.0, 100.0]);
    assert!(far.overlaps(&overlapping_poison));
    assert_eq!(far.separation_lo(&overlapping_poison), 0.0);
    assert_eq!(overlapping_poison.separation_lo(&far), 0.0);

    // Both x bounds NaN, y and z genuinely distant: a positive claim,
    // justified by the y/z bounds alone. `overlaps` prunes it too, so
    // the crate's two box predicates agree.
    let distant_poison = boxed([f64::NAN, 0.0, 0.0], [f64::NAN, 1.0, 1.0]);
    let claimed = far.separation_lo(&distant_poison);
    println!("[r2] one-axis poison, distant on the others: {claimed}");
    assert!(claimed > 0.0 && claimed <= 98.0 * 2.0f64.sqrt());
    assert!(!far.overlaps(&distant_poison));

    // ONE bound NaN, the survivor on the FAR side: `f64::max` swallows
    // the NaN and takes the other candidate, which the surviving bounds
    // justify.
    let half_poison = boxed([f64::NAN, 0.0, 0.0], [1.0, 1.0, 1.0]);
    let lo = far.separation_lo(&half_poison);
    assert!(
        lo <= 98.0 * 3.0f64.sqrt(),
        "every point of that box has x <= 1, y <= 1, z <= 1: {lo}"
    );
    assert_eq!(lo, half_poison.separation_lo(&far), "and it is symmetric");

    // ONE bound NaN, the survivor on the NEAR side: that axis'
    // surviving candidate is negative, so x contributes 0 and the
    // answer is carried entirely by the other two axes.
    let near_poison = boxed([0.0, 0.0, 0.0], [f64::NAN, 1.0, 1.0]);
    let lo = far.separation_lo(&near_poison);
    println!("[r2] near-side single-NaN bound: {lo}");
    assert!(
        (lo - 98.0 * 2.0f64.sqrt()).abs() < 1.0e-9,
        "the x axis claims nothing and y, z carry 98 each: {lo}"
    );

    // The all-NaN box, which is the shape the shipped table tests.
    assert_eq!(Aabb::poison().separation_lo(&Aabb::poison()), 0.0);
    assert_eq!(far.separation_lo(&Aabb::poison()), 0.0);
}

/// An infinite bound is not NaN and does not go through the poison
/// gate — it goes through the arithmetic. Both directions are checked:
/// a box that reaches to infinity overlaps everything (separation 0),
/// and a box placed AT infinity produces an infinite gap whose norm is
/// infinite, so the saturation row above is its shape too.
#[test]
fn infinite_bounds_answer_in_the_safe_direction() {
    let unit = boxed([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
    let unbounded = boxed(
        [f64::NEG_INFINITY; 3],
        [f64::INFINITY, f64::INFINITY, f64::INFINITY],
    );
    assert_eq!(
        unit.separation_lo(&unbounded),
        0.0,
        "a box containing everything is at distance zero from everything"
    );
    let at_infinity = boxed([f64::INFINITY; 3], [f64::INFINITY; 3]);
    let lo = unit.separation_lo(&at_infinity);
    println!("[r2] separation from a box at infinity: {lo}");
    assert!(lo.is_finite(), "the answer stays a finite number: {lo}");
}

/// `within`'s pruning must never contradict the per-item bound it is
/// derived from, over a table of pads that straddle the shipped
/// comparison EXACTLY — `separation_lo(query) <= pad` is the leaf
/// filter, so a pad landing precisely on an item's own bound is the
/// boundary case the range generator will not hit.
#[test]
fn a_pad_exactly_on_an_items_bound_keeps_the_item() {
    let boxes: Vec<Aabb> = (0..8)
        .map(|i| {
            let f = f64::from(i);
            boxed([f * 3.0, 0.0, 0.0], [f * 3.0 + 1.0, 1.0, 1.0])
        })
        .collect();
    let tree = Bvh::build(&boxes);
    let query = boxed([-5.0, 0.0, 0.0], [-4.0, 1.0, 1.0]);
    for (i, b) in boxes.iter().enumerate() {
        let exact = b.separation_lo(&query);
        assert!(
            tree.within(&query, exact).contains(&i),
            "item {i} at its own bound {exact} must survive a pad of exactly that"
        );
        // And one ulp under it drops exactly this item, never a nearer
        // one: the comparison is non-strict on the item side.
        let under = exact.next_down();
        assert!(
            !tree.within(&query, under).contains(&i) || exact == 0.0,
            "item {i} must not survive a pad below its own bound"
        );
    }
}
