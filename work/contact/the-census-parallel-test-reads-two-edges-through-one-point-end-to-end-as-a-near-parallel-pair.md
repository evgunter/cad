---
id: the-census-parallel-test-reads-two-edges-through-one-point-end-to-end-as-a-near-parallel-pair
kind: issue
title: The census's parallel test levers the two edges' line angle at the shorter length, so two edges continuing each other through one point, or lying 3e-8 apart, escalate pm_census_ee_parallel
status: open
opened: 2026-10-08
priority: P0
cost: M
refs: [near-tangent-boolean-results-ship-with-an-escalated-tier-3-census]
---

## What

Found by JOIN's near-tangent census measurement (`near-tangent-boolean-results-ship-with-an-escalated-tier-3-census`, its `## Measured`), on main `047d10d5`, release.

`crates/topo/src/census.rs` `pair_edge_edge` decides `pm_census_ee_parallel`
on `|ea.dir × eb.dir| · min(ea.len, eb.len)`, the angle between the two
LINES levered at the shorter edge. In near-tangent results that reading
lands in band for pairs whose segments are definitely apart, or that meet
only at a point they share:

- **End to end through one point (159 of 207 at ε = 1e-9).** Two edges
  leave one point in opposite directions, collinear to 1e-8. They are
  within K·ε of each other only within K·ε of the point.
  Witness: `vee300 nt e0 a0 d1e-8 cp S`, `EdgeKey(34v1)` and
  `EdgeKey(35v3)` from `v = (2, 0.5, 1)`, lengths 0.576 and 2.0,
  sin θ 1.0e-8. The margin reads 5.79e-9, and the exact value of the
  same formula is the same.
- **Apart (48 of 207).** Segments 3.2e-8 to 2.1e-5 apart. Witness:
  `w345 nt e0 a6 d1e-8 pc U`, `EdgeKey(21v1)` and `EdgeKey(56v1)`,
  margin 1.11e-9, segments 3.15e-8 apart.

Every one is at every tilt from 1e-5 to 1e-9, and at ε = 1e-12 and 1e-6
as well (1e-6: 198 escalations, none a sliver). The margin is exact, so
the arithmetic does not make it. What makes it is the question asked:
"are the lines parallel" where the census wants "do the segments run
together".

The one class-(a) reading at this predicate is a 3.7e-8 edge at
`w345 nt e2 a6 d-3e-8`, which is JOIN's
`a-near-tangent-vertex-lands-within-the-band-of-a-face-and-ships-unrecorded`.

Repro: `NT_DUMP=1 cargo run -p sweep --release --example near_tangent_census_probe | python3 scripts/oracles/near_tangent_census_classify.py`, with `NT_ONLY`/`NT_POSE`/`NT_D` to pick the pose and `CAD_TOLERANCE_EPS` the row. The classifier reproduces each census margin bit for bit in f64 and recomputes it at 60 digits on the same coordinates.

## The shape to give

Read the parallel question where the segments are near each other: the
shorter edge's two ends against the longer edge, as
`the-census-edge-edge-collinear-lane-reads-the-offset-at-the-long-edges-start`
asks of the collinear lane. A pair that meets only at a shared point, in
opposite directions, then reads its two far ends as definitely apart.
`census-edge-overlap-decides-parallel-and-line-gap-one-at-a-time` is the
same lane's two-margin question.
