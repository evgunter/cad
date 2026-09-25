---
id: three-walks-over-the-name-carrying-edges
kind: issue
title: the name-carrying edge set (Transform, Part, split pass-through) is walked in two places and a third is proposed; it wants one home
status: open
opened: 2026-09-24
priority: P1
cost: D
refs: [the-solve-accepts-a-body-placed-under-two-roots, product-refuses-naming-when-one-instance-is-placed-under-two-roots]
---

Recorded from PR 3142's review (Q1), filed by the GATHER two-roots lane.

**The finding.** Two walks go down the same set of name-carrying edges
(the ones that pass names through verbatim), each choosing its own
subset, and a third has been proposed:

1. `crates/editor-core/src/product.rs`, `placed_under_two_roots` (~975):
   from each product root it follows `Transform.input` (whole) and
   `Part.of` (any selection), carrying the selection down through
   transforms. It stops at everything else, including a split.
2. `crates/editor-core/src/mate/member.rs`, `walk` (~165): from a mate
   reference's operand down to the name's head, it follows
   `Transform.input` and `Part.of` with an `Instance` selection only.
   A `Part` naming a split half stops it.
3. MSOLVE's `work/msolve/the-solve-accepts-a-body-placed-under-two-roots.md`
   proposes that the solve refuse the same shape. Unless it calls (1),
   that would be a third walk.

The two walks agree on Transform and disagree on `Part(SplitHalf)`.
Each is right for its own question: a half is a different body to a
mate, but the same body twice to the gather. What has no single home
is the edge set itself: which node kinds carry names verbatim (N1:
Transform, a `Part`'s projection, a split's intact pass-through). Each
walk re-derives that set from prose. A new pass-through op (or one that
stops being one) has to be taught to every walk separately, and nothing
reds if one is missed.

**What would close it.** One function over `Node` that answers
"verbatim edge, and with what selection", owned beside N1's
definition. Each walk would then be a fold over it that picks its own
stopping rule. Not a rewrite of either walk's semantics.

