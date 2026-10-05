---
id: a-six-crossing-vertex-pair-nests-its-pairing-and-refuses-pairing-mismatch
kind: issue
title: A vertex pair whose links cross six times pairs them nested, not adjacent in B, and refuses PairingMismatch every op
status: open
opened: 2026-10-05
priority: P0
cost: H
---


## What

Found by PR 4036's dual review (r1 MINOR-1, r2 MINOR 2). A reflex
corner whose link crosses a cube's corner or edge six times refuses
`PairingMismatch` in every op, in both orders, on main and on PR 4036's
head alike. The germs are distinct, and `insert::walk_order`'s tie
refusal never fires there.

Pinned by `join_pierce_runs_sweep::a_six_crossing_notch_corner_refuses_pairing_mismatch_on_distinct_germs`:
a 343° notch (`(0,0) (2,0) (2,0.85) (1,1) (2,1.15) (2,2) (0,2)`, corner
`v = (1, 1, 1)`) on a cube of side 4 at the pierce sweep's `edge i=3
j=0 psi=1`.

## Cause

`insert::plan_null_pairs` pairs the survivors consecutively in A's walk
order, and F12 guard 1 asks that each pair also be adjacent in B's.
Each solid's link round its vertex is a simple closed curve. At four
crossings, both non-crossing matchings fix one cyclic order on both
curves, so the guard cannot fire. At six, a nested matching is legal on
each curve.

Traced (r1): A pairs `(0, 3) (2, 4) (5, 1)`, and B reads them at
`(2, 5) (0, 1) (4, 3)` of six. In r2's words, the side-X arcs
`{16, 25, 34}` and the side-Y arcs `{12, 36, 45}` give B's link the
order `1 6 3 4 5 2`, and neither start pairs adjacently.

Convex six-crossing links build: r2's `hex` sweep has 1 026 n=6 runs,
all `SOUND`.

## Measured

- **r1**, branch `join/reflex-corner-vertex-vertex-review-r1`,
  `crates/sweep/tests/join_vv_review_r1_probes.rs`:
  - 324 runs, all `notch343`, 54 poses on the cube's edge and corner
    (`grid`, `turn` and `hex` batteries);
  - `turn`: 108 runs.
- **r2**, branch `join/reflex-corner-vertex-vertex-review-r2`,
  `crates/sweep/tests/review_r2_vv_probes.rs`:
  - 1 422 runs on `notch` (345°), `notch355` and `reflex315`, on a
    cube's edge or corner;
  - e.g. `notch corner frame=1 eps=1e-2 ax=1 sc U`
    (`R2_SWEEP=tilt`).
- Every one of these runs is `PairingMismatch` on main too.

## Owed

Pair a nested six-crossing corner. In B, an outer pair's run holds
the inner pair's germs either way round, so its runs nest rather than
lie disjoint. That is the shape `reconcile_shared` and `holds_whole`
handle across several pairs. Measure whether that machinery, the join
and the finish carry nested runs of one pair, then build it with every
op at kernel-free volumes. Flip the pin when it builds.
