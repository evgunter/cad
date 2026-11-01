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

## Measured after the four fixes (2026-11-01)

Main at 37b8832, with JETFRAME #6250, ZEROSIDE #6234, HEADING #6235
and DIRSUPPORT #6254. Nothing on the question's path (`topo::clearance`,
`topo::injective`, `geom`, `geom-core`) changed between that commit and
3759cfa. The question is the row's: `StrictlyPositive`, the body
against itself, `DEFAULT_MAX_CELL_DEPTH`, 65536 pairs. Release builds
with debug assertions on (the tour's profile also arms overflow checks;
the workspace's, which built the helices in `crates/sweep`, does not),
pinned to one core of an otherwise idle 4-core box. CARVE's scratch harness, re-pointed and not committed.

"Pairs split" is the receipt's `splits`, "certified" its `certified`,
"refused" its `refused`. "Build" is the loft or sweep call alone. The
Klein loop's `finished` validation adds 4.07 s to its 7.6 ms. The
three sepals build together, in 8.4 ms.

| body | verdict | split | certified | refused | deepest | cert | build | cert / build |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| `lily_leaf_a` | Holds | 3410 | 1058 | 0 | 22 | 27.2 s | 5.6 ms | 4900× |
| `lily_leaf_b` | Holds | 311 | 104 | 0 | 11 | 2.68 s | 2.0 ms | 1300× |
| `lily_leaf_c` | Holds | 359 | 106 | 0 | 12 | 3.41 s | 1.8 ms | 1900× |
| `lily_sepal_a` | Holds | 1102 | 347 | 0 | 18 | 9.94 s | 2.8 ms | 3500× |
| `lily_sepal_b` | Holds | 1206 | 369 | 0 | 17 | 10.5 s | 2.8 ms | 3700× |
| `lily_sepal_c` | Holds | 1241 | 385 | 0 | 19 | 11.0 s | 2.8 ms | 3900× |
| Klein loop | Holds | 48162 | 14888 | 0 | 21 | 46.8 s | 7.6 ms | 6200× (11× with `finished`) |
| `twisted_loft` | Holds | 9403 | 2765 | 0 | 25 | 4.95 s | 1.4 ms | 3500× |
| `twisted_tube` | Holds | 765 | 257 | 0 | 9 | 3.13 s | 4.0 ms | 780× |
| `s_duct` | Holds | 189 | 92 | 0 | 6 | 1.16 s | 4.8 ms | 240× |
| `nonuniform_loft` | Holds | 388 | 163 | 0 | 16 | 0.20 s | 1.9 ms | 105× |
| helix, 1 turn / 33 | Holds | 558 | 220 | 0 | 9 | 3.68 s | 14 ms | 260× |
| helix, 1 turn / 65 | Holds | 558 | 220 | 0 | 9 | 5.58 s | 25 ms | 230× |
| helix, 2 turns / 65 | Holds | 1249 | 444 | 0 | 11 | 8.42 s | 24 ms | 350× |

Every body now answers, and none refuses. The four that ran out of
budget before (`lily_leaf_a` and the three sepals) take 1100–3400
splits. The Klein loop takes 48162, 73% of the budget, so it is the one
body still near the line. The pair count is no longer what costs. Per
split, the price is 0.5 ms on `twisted_loft` and `nonuniform_loft`,
1 ms on the Klein loop, and 6–10 ms on the lily blades and the helices.
The helix at 65 stations takes the same 558 splits as at 33 but 1.5×
the time, so the per-split price grows with the net the cell is cut
from.

A callgrind profile of the one-turn helix at 33 stations places the
time:
- 71% is inside `injective::departure`.
- 57% of everything is `nurbs::read_net`, which restricts the face's
  NURBS net to the cell by blossoming
  (`SpanRow::restrict` / `restrict_derivative`, from `rect_jet` and
  `rect_net`). The certified blossom step alone,
  `Interval::between`, is 47%.
- The separator and the distance arm, `separated_by_support` and
  `split_by`, are about 8% and 6%.

What dominates now is restricting the net, cell by cell, in certified
interval arithmetic, rather than the geometry or the budget.

Where the question answers, it still costs 100–6000× the build, between
0.2 s and 47 s. That is still too dear to pay on every rebuild. The
"Left open" question above can now be decided on these numbers.
