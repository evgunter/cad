//! The deterministic tree: arena-order build, fixed split rule with
//! total tie-breaks (documented at [`Bvh::build`], D9-cited), no hash
//! iteration, no parallel build in v1, fixed leaf constant
//! [`LEAF_SIZE`]. Queries prune only (crate docs: the conservative-
//! superset contract) and return candidates in ascending input order.

use crate::aabb::{Aabb, Axis};
use crate::ray::{Ray, RayCandidate};

/// The fixed leaf-size constant (C10: named, fixed, not tuned): a
/// range of at most this many items becomes a leaf.
pub const LEAF_SIZE: usize = 8;

/// One node, preorder-stored: a node's left child (when internal) is
/// the next node in the arena; the right child's index is explicit.
#[derive(Clone, Debug)]
enum Node {
    /// A leaf: `items[start .. start + count]` (indices into the
    /// build-time permutation, which holds input indices).
    Leaf {
        /// Start of the leaf's range in the item permutation.
        start: usize,
        /// Number of items in the leaf.
        count: usize,
        /// Hull of the leaf's item boxes.
        aabb: Aabb,
    },
    /// An internal node; left child is at `self + 1` (preorder).
    Inner {
        /// Arena index of the right child.
        right: usize,
        /// Hull of the subtree's item boxes.
        aabb: Aabb,
    },
}

/// A deterministic AABB tree over `n` items, addressed by their input
/// (arena) index `0..n`. Payloads live caller-side in parallel arrays
/// (crate docs: the SSI-cell seam).
#[derive(Clone, Debug)]
pub struct Bvh {
    nodes: Vec<Node>,
    /// The build permutation: input indices, leaf ranges contiguous.
    items: Vec<usize>,
    /// The item boxes, in input order (leaf hits re-test against the
    /// exact per-item box, not just the leaf hull).
    boxes: Vec<Aabb>,
}

impl Bvh {
    /// Builds the tree from item boxes **in input (arena) order** —
    /// the input index is the identity queries hand back.
    ///
    /// # The split rule (fixed, total — D9)
    ///
    /// At each internal node, over the range's item **centroids**
    /// ([`Aabb::centroid`]):
    ///
    /// 1. Split axis = the axis of largest centroid-bounds extent;
    ///    ties (compared with `f64::total_cmp`, strictly-greater to
    ///    switch) keep the **lower axis index** (X < Y < Z).
    /// 2. Order the range by `(centroid on that axis under
    ///    `total_cmp`, then input index ascending)` — a total order
    ///    for every input including NaN (poison sorts by IEEE total
    ///    order; determinism never depends on box validity).
    /// 3. Split at the median position `len / 2` (floor). Both halves
    ///    are non-empty for `len ≥ 2`, so recursion strictly shrinks
    ///    and terminates structurally.
    ///
    /// A range of at most [`LEAF_SIZE`] items is a leaf. Node hulls
    /// fold left-to-right over the range in its post-ordering order
    /// (fixed association order, D9). No parallelism, no hashing.
    pub fn build(boxes: &[Aabb]) -> Self {
        let mut items: Vec<usize> = (0..boxes.len()).collect();
        let mut nodes = Vec::new();
        if !boxes.is_empty() {
            build_range(boxes, &mut nodes, &mut items, 0);
        }
        Self {
            nodes,
            items,
            boxes: boxes.to_vec(),
        }
    }

    /// The number of items the tree was built over.
    pub fn len(&self) -> usize {
        self.boxes.len()
    }

    /// Whether the tree is empty.
    pub fn is_empty(&self) -> bool {
        self.boxes.is_empty()
    }

    /// All input indices whose item box overlaps `query`, in
    /// **ascending input order** (a subsequence of the arena order —
    /// the D9-relevant contract; tree shape never leaks into the
    /// result). Conservative by [`Aabb::overlaps`]: poison boxes are
    /// always candidates.
    pub fn overlapping(&self, query: &Aabb) -> Vec<usize> {
        let mut out = Vec::new();
        // Explicit stack, fixed order (right pushed first, left
        // visited first); order is irrelevant to the sorted result but
        // fixed anyway (D9 discipline).
        let mut stack = Vec::new();
        if !self.nodes.is_empty() {
            stack.push(0usize);
        }
        while let Some(idx) = stack.pop() {
            let Some(node) = self.nodes.get(idx) else {
                // Unreachable: child indices are minted by the build.
                continue;
            };
            match node {
                Node::Leaf { start, count, aabb } => {
                    if !aabb.overlaps(query) {
                        continue;
                    }
                    for &item in self.items.iter().skip(*start).take(*count) {
                        if self.boxes.get(item).is_some_and(|b| b.overlaps(query)) {
                            out.push(item);
                        }
                    }
                }
                Node::Inner { right, aabb } => {
                    if !aabb.overlaps(query) {
                        continue;
                    }
                    stack.push(*right);
                    stack.push(idx + 1);
                }
            }
        }
        // Each item lives in exactly one leaf, so this is a
        // permutation sort, never a dedup.
        out.sort_unstable();
        out
    }

    /// All input indices whose item box the ray may intersect over
    /// `t ∈ [0, ∞)` — the viewport-picking query (crate docs).
    ///
    /// Conservative by [`Ray::slab_enter`]: a returned candidate may
    /// be a miss (the consumer re-tests exactly), but a leaf whose box
    /// the ray truly intersects is never dropped — poison boxes and
    /// poison rays are always candidates. Node hulls only prune:
    /// each per-axis hull interval contains the corresponding item
    /// interval (the slab arithmetic is monotone in the bounds, NaN
    /// axes are skipped on both, and hulls propagate item NaN), so a
    /// hull prune never drops an item its own slab test would accept —
    /// the result is a function of the item boxes only, independent of
    /// tree shape.
    ///
    /// **Candidate order (documented contract)**: ascending
    /// [`RayCandidate::t_enter`] under `f64::total_cmp`, ties broken
    /// by ascending input index. `t_enter` is never NaN (the slab fold
    /// starts at `0.0` and only ever takes non-NaN bounds), so
    /// `total_cmp` agrees with the naive order and is used purely to
    /// keep the sort total by construction. Same query, same tree ⇒
    /// bit-identical output (the crate's determinism posture).
    pub fn ray(&self, ray: &Ray) -> Vec<RayCandidate> {
        let mut out = Vec::new();
        // Same fixed traversal shape as `overlapping`; order is
        // irrelevant to the sorted result but fixed anyway (D9
        // discipline).
        let mut stack = Vec::new();
        if !self.nodes.is_empty() {
            stack.push(0usize);
        }
        while let Some(idx) = stack.pop() {
            let Some(node) = self.nodes.get(idx) else {
                // Unreachable: child indices are minted by the build.
                continue;
            };
            match node {
                Node::Leaf { start, count, aabb } => {
                    if ray.slab_enter(aabb).is_none() {
                        continue;
                    }
                    for &item in self.items.iter().skip(*start).take(*count) {
                        if let Some(t_enter) = self.boxes.get(item).and_then(|b| ray.slab_enter(b))
                        {
                            out.push(RayCandidate { item, t_enter });
                        }
                    }
                }
                Node::Inner { right, aabb } => {
                    if ray.slab_enter(aabb).is_none() {
                        continue;
                    }
                    stack.push(*right);
                    stack.push(idx + 1);
                }
            }
        }
        out.sort_unstable_by(|a, b| a.t_enter.total_cmp(&b.t_enter).then(a.item.cmp(&b.item)));
        out
    }
}

/// Recursive build over `items[..]` (a contiguous range of the final
/// permutation starting at absolute offset `base`); returns the
/// arena index of the subtree's root. Depth is ≤ log₂(n) + 1 (the
/// median split halves exactly), so recursion is safe.
fn build_range(boxes: &[Aabb], nodes: &mut Vec<Node>, items: &mut [usize], base: usize) -> usize {
    let hull = items
        .iter()
        .filter_map(|&i| boxes.get(i))
        .fold(None::<Aabb>, |acc, b| {
            Some(match acc {
                None => *b,
                Some(a) => a.hull(b),
            })
        })
        .unwrap_or_else(Aabb::poison);

    if items.len() <= LEAF_SIZE {
        nodes.push(Node::Leaf {
            start: base,
            count: items.len(),
            aabb: hull,
        });
        return nodes.len() - 1;
    }

    // Split rule steps 1–3 (see `Bvh::build` docs).
    let axis = split_axis(boxes, items);
    items.sort_unstable_by(|&a, &b| {
        let ca = boxes.get(a).map_or(f64::NAN, |x| x.centroid(axis));
        let cb = boxes.get(b).map_or(f64::NAN, |x| x.centroid(axis));
        ca.total_cmp(&cb).then(a.cmp(&b))
    });
    let mid = items.len() / 2;

    let this = nodes.len();
    // Placeholder, patched below once the right child's index exists.
    nodes.push(Node::Inner {
        right: 0,
        aabb: hull,
    });
    let (left_items, right_items) = items.split_at_mut(mid);
    let left = build_range(boxes, nodes, left_items, base);
    debug_assert_eq!(left, this + 1, "preorder invariant: left child is next");
    let right = build_range(boxes, nodes, right_items, base + mid);
    if let Some(n) = nodes.get_mut(this) {
        *n = Node::Inner { right, aabb: hull };
    }
    this
}

/// Step 1 of the split rule: the axis of largest centroid-bounds
/// extent; strictly-greater (under `total_cmp`) to switch axes, so
/// ties keep the lower axis index.
fn split_axis(boxes: &[Aabb], items: &[usize]) -> Axis {
    let extent = |axis: Axis| -> f64 {
        let mut lo = f64::INFINITY;
        let mut hi = f64::NEG_INFINITY;
        for &i in items {
            if let Some(b) = boxes.get(i) {
                let c = b.centroid(axis);
                if c.is_nan() {
                    return f64::NAN;
                }
                // Raw f64::min/max are sound HERE: the split-axis
                // choice affects tree SHAPE only, never membership
                // (queries re-test exact item boxes); NaN is already
                // handled above. aabb.rs's NaN-propagating folds are
                // for membership-bearing boxes.
                lo = lo.min(c);
                hi = hi.max(c);
            }
        }
        hi - lo
    };
    let (ex, ey, ez) = (extent(Axis::X), extent(Axis::Y), extent(Axis::Z));
    let mut axis = Axis::X;
    let mut best = ex;
    if ey.total_cmp(&best) == core::cmp::Ordering::Greater {
        axis = Axis::Y;
        best = ey;
    }
    if ez.total_cmp(&best) == core::cmp::Ordering::Greater {
        axis = Axis::Z;
    }
    axis
}
