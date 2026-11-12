---
id: the-certificate-runs-out-of-pairs-on-eight-finished-tour-bodies
kind: issue
title: The injectivity check runs out of its 65,536 cell pairs on eight finished tour bodies (the die family, the Klein bulb, the snowman) in 2.5-6.3 s, so the at-rest gate would refuse them
status: open
opened: 2026-11-12
priority: P0
cost: M
---


## Finding

Measured by CLEAR's lane clear-gate (`the-at-rest-gate-certifies-embedding`,
its Measured section). Main at `348ec09727`; the body-level question
the gate would ask (`StrictlyPositive`, one body against itself,
`DEFAULT_MAX_CELL_DEPTH`, 65,536 pairs), at `f64`, on bodies the tour
builds and tier 3 passes. Each refuses `Budget(Pairs)`:

| body | faces | refuses after | deepest |
|---|---:|---:|---:|
| `diepips` | 48 | 3.6 s | 24 |
| `diecomposed` | 89 | 6.3 s | 21 |
| `diechamfer` | 68 | 3.3 s | 24 |
| `klein_bulb` | 12 | 2.5 s | 24 |
| `snowman_rolled` | 5 | 2.7 s | 32 |
| `snowman_union`, `snowman_bitten`, `snowman_lens` | 4 | 2.6–3.0 s | 32 |

The planar die with 21 square pips holds in 0.11 s (6,283 pairs), so
face count alone does not explain the cost.
Which face pairs hold the frontier was not read; the receipt's
`across` map (`BoundReport::across`, `clearance.rs`) has it.

## Why P0

`the-at-rest-gate-certifies-embedding` (P0) cannot land its gate while
these bodies refuse: the gate would refuse them at their doors.

## Fix shape

Read `across` for `snowman_union` (four faces) first: a sphere against
a sphere along their seam circle is the likely frontier.

## Diagnosis (CLEAR, lane clear-pairs, 2026-11-12)

### The snowman: a sphere's pole, between the two half-bands of one ball

`snowman_union` (main `c51fc90f58`, `f64`, the gate's question at
65,536 pairs, the frontier instrumented per split): 92,093 splits, and
91,850 of them are the two half-bands of ONE ball against each other
(`2v1`–`5v1` the bottom ball, 47,516; `3v3`–`4v3` the head, 44,334),
the departure arm splitting. The four pairs across the waist split
about 50 times each and certify. At depth 32 every frontier cell has
`v` in `[−π/2, −π/2 + π/256]` (the bottom ball's south pole) or the
mirror strip at the head's north pole, `u` cut into strips 0.025 wide.

Why they never decide: a cell holding the pole vertex is near BOTH
seam meridians and the pole, since every ball round the pole meets
both meridians there. `present_chart` read the pole as a loose shared
vertex that no corner of two clips holds, and answered `Local`, so
every pair split. The pole is a whole side of the sphere's chart, the
latitude line `v = ±π/2`, and no split ever moves a cell off it, so a
polar strip against every polar strip of the other half-band is a
pair forever: the count is quadratic in the strips, which double.
`plane_through` had a second gap there: it took a conic's own plane
only where ONE shared edge is near, and a polar cell is near two (the
two meridians, both arcs of one great circle).

The single ball (2 faces) holds at its root; the union holds once its
poles are read.
