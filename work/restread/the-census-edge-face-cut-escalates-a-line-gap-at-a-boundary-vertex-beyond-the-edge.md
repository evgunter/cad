---
id: the-census-edge-face-cut-escalates-a-line-gap-at-a-boundary-vertex-beyond-the-edge
kind: issue
title: The census's edge-face cut and vertex-edge lanes escalate a vertex's gap off the edge's line before reading its span, so a face vertex 0.58 to 1.0 beyond the edge's end escalates pm_census_ef_cut_gap
status: open
opened: 2026-10-08
priority: P0
cost: E
refs: [near-tangent-boolean-results-ship-with-an-escalated-tier-3-census]
---

## What

Found by JOIN's near-tangent census measurement (`near-tangent-boolean-results-ship-with-an-escalated-tier-3-census`, its `## Measured`), on main `047d10d5`, release.

`crates/topo/src/census.rs` `ef_overlap_cells` decides `pm_census_ef_cut_gap`,
a face boundary vertex's distance from the edge's LINE, and pushes an
escalation before reading the vertex's span along the edge.
`on_edge_interior` does the same with `pm_census_ve_line_gap`. In
near-tangent results the vertex lies within the band of the line, past
the edge's end:

- 151 escalations at ε = 1e-9 (d = ±1e-8, ±1e-9, one at −1e-5). Every
  vertex is 0.58 to 1.0 from the segment.
- 60 at ε = 1e-12 and 346 at ε = 1e-6, the same shape.
- Witness: `Ltop nt e2 a8 d1e-9 pc U`, `EdgeKey(34v1)` against
  `FaceKey(10v3)`, vertex `VertexKey(19v1)`. The gap reads 2.0e-9 (exact
  the same), and the vertex is 1.005 from the segment.

The margin is exact; the line is the proxy.

Repro: `NT_DUMP=1 cargo run -p sweep --release --example near_tangent_census_probe | python3 scripts/oracles/near_tangent_census_classify.py`, with `NT_ONLY`/`NT_POSE`/`NT_D` to pick the pose and `CAD_TOLERANCE_EPS` the row. The classifier reproduces each census margin bit for bit in f64 and recomputes it at 60 digits on the same coordinates.

## The shape to give

Read the span first. A vertex whose span along the edge is definitely
outside `[0, len]` by more than K·ε cuts nothing there, whatever its line
gap. The same reorder serves `on_edge_interior`.
