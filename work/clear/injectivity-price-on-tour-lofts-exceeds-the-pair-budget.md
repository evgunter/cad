---
id: injectivity-price-on-tour-lofts-exceeds-the-pair-budget
kind: issue
title: The injectivity check needs more than 65536 cell pairs on a wall against itself or against a near-parallel wall of the tour's lily blades and Klein loop, and costs 1 to 10 s where it holds
status: open
opened: 2026-10-31
priority: P0
cost: H
---


## What was found (CARVE's embedding-certificate measurement, 2026-10-31)

The whole-body injectivity question (`StrictlyPositive`, one body
against itself, `DEFAULT_MAX_CELL_DEPTH`, 65536 pairs; release build
with debug assertions on a 4-core box shared with one other build)
does not answer on tour lofts that have no sliver knot (see
`injectivity-monotone-arm-reads-a-sliver-knot-span-it-cannot-bound`):

- `lily_leaf_a`, and `lily_sepal_a`, `lily_sepal_b` and `lily_sepal_c`:
  `Budget(Pairs)` after 55–59 s each. The lily blades build in under
  0.1 s.
- The Klein loop: `Budget(Pairs)` after 33 s. It builds in 1.5 s.

Per pair, at a budget of 8192:

- A wall against itself is still at depth 12–15, with 3500–5800 pairs
  unstarted. That frontier is an area, not a line: the monotone arm
  certifies only cells small against the wall's curvature.
- The Klein loop's outer wall against its near-parallel inner wall
  certifies nothing and is at depth 21–27. The distance test needs
  cells smaller than the gap.

Both grow with the square of extent over cell size.

Where the question does answer, it costs 0.1–9 s against builds of
1–20 ms:

- `skinned`'s `nonuniform_loft`: 0.11 s.
- `twisted_duct`: 1.0 s.
- the arc spine at 0.45–3.5 rad: 0.1–0.65 s.
- `m8_14`'s helices: 3.9 s for one turn at 33 stations, 5.1 s for one
  turn at 65, and 8.9 s for two turns at 65.

`topo::validate_geometric` plus the signed volume of the same body
takes 0.1–1.8 s.

## Home

CLEAR, as price. A door that runs this on every rebuild pays 15 to 500
times the build where it answers. Whether a coarser arm is wanted
(e.g. a cone of normals, or a bounded-turn certificate for a whole
knot span), or a budget sized per body, is CLEAR's call. CARVE's
`self-overlapping-spines-build-and-validate` is the consumer that
measured it.

## Weighed (CLEAR, 2026-11-01): a defect, not a fork

Three designers, one Fable and two Opus, were dispatched together with
the same problem statement. All three, independently, found the premise
wrong: the price is mostly the derivative enclosure, not the geometry.
They converged on:

1. **Fix the enclosure.**
   `rect-jet-forms-the-rational-derivative-in-the-absolute-frame` (P0).
2. **Then the distance test.** `distance-test-separates-boxes-not-directions`
   (P1).
3. **Keep one fixed pair budget per question,** as Ev ruled for
   measurement in PR 6102. Running out of budget refuses typed and never
   reads Ok.
   - A per-body budget is rejected, by all three: it would have grown to
     fit the defect.
   - A coarser arm is rejected, by all three: a normal cone alone passes
     the 370° ribbon, and the monotone arm on a tight enclosure is
     already about as coarse as a sound first-order test gets.

**Left open.** Whether the loft and sweep doors run the question on every
rebuild, at every scalar or only at the certifying ones. That is CARVE's
door and Ev's call, and all three advise deciding it on the measured
numbers after (1), ZEROSIDE and (2) land. This row stays parked on (1)
and (2), and is re-measured then.
