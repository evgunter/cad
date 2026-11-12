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
