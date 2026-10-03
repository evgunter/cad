---
id: undeclared-anti-parallel-touch-builds-a-scaffold-at-rest-in-one-op
kind: issue
title: An undeclared anti-parallel touch at a wedge builds a body only the final validator catches, in one op/order per pose
status: open
opened: 2026-10-03
---


## What

The PR 3962 review's `rv_coplanar_battery`
(`crates/sweep/tests/join_reflex_wedge_review_probes.rs` on branch
`join/edge-edge-reflex-wedge-review`) runs pairs of prisms over irregular
star profiles that share a vertical corner edge. One flank of `b` lies
along one of `a`'s flank lines, pointing the opposite way: an
anti-parallel touch along that wall. With the coplanar pair
**undeclared**, 24 runs build a body that only the final validator
rejects, `ResultInvalid { errors: [ScaffoldAtRest { .. }] }`. On the
base of PR 3962 the same runs refused `ClassificationInvariant` at the
edge-edge membership.

For each pose, exactly one op in one operand order does this. The other
five runs, and all six runs of the flush-declared twin, are SOUND
(tier 2, tier 3′, certificate, closed-form volume). The poses
(`θa θb φb op`) are listed below. Each one fails under all three of the
battery's maps (`id`, `rot`, `proj`), so 8 × 3 = 24 runs:

- `40 200 180 S_ba`, `40 200 20 S_ba`
- `100 200 180 S_ba`, `100 200 80 S_ba`
- `230 170 180 S_ab`, `230 170 240 S_ab`
- `300 170 180 S_ab`, `300 170 310 S_ab`

These are re-run on the PR 3962 fix-pass head, merged with main after
JOIN-3. To reproduce, check out the review branch and run
`cargo test --release -p sweep --test all rv_coplanar_battery --
--ignored --nocapture`, then grep `ResultInvalid`.

The result is loud, not wrong: no body ships. What is odd is the
asymmetry, one subtraction in one operand order. Nobody has
root-caused it yet.
