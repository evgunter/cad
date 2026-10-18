---
id: the-census-edge-face-cut-escalates-a-line-gap-at-a-boundary-vertex-beyond-the-edge
kind: issue
title: The census's edge-face cut and vertex-edge lanes escalate a vertex's gap off the edge's line before reading its span, so a face vertex 0.58 to 1.0 beyond the edge's end escalates pm_census_ef_cut_gap
status: closed
opened: 2026-10-08
priority: P0
cost: E
refs: [4335]
closed: 2026-10-17
---

## What

Found by JOIN's near-tangent census measurement (`near-tangent-boolean-results-ship-with-an-escalated-tier-3-census`, its `## Measured`), on main `047d10d5`, release.

`crates/topo/src/census.rs` `ef_overlap_cells` decides `pm_census_ef_cut_gap`,
a face boundary vertex's distance from the edge's LINE, and pushes an
escalation before reading the vertex's span along the edge.
`on_edge_interior` did the same with `pm_census_ve_line_gap`, until FUSE's stage c3 (PR 5227) moved it onto `Span::contact`, which reads the span first. In
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

## Closed

`census::ef_overlap_cells` now cuts the edge at a boundary vertex
through `on_edge_interior_reading`, the one point-on-span reading
(`Span::contact`): it reads the span first, and reads the gap off the
line only where the foot is inside the segment. A vertex at or past an
end is that end's own cut, or no cut, whatever its gap. Pass 2's
`on_edge_interior` already read the same way, through
`Span::contact` (PR 5227). Measured on the probe's default grid: at
ε = 1e-9, `pm_census_ef_cut_gap` drops from 411 to 0; at 1e-12, from
21 to 0; at 1e-6, from 854 to 0. That row no longer exists. The cut's
reading now escalates under `curve_carrier_*` / `curve_pair_span_*`, and
those read 0 on main and 0 on the branch at all three rows, from pass 4
or anywhere else on the grid. Pinned by `sweep`'s
`near_tangent_census_edge_face_lane` on per-row poses (`Ltop nt e2 a8
d1e-9` among them).

The crossing reading behind the same pairs also escalated:
`edge_crossings`' `meet` of the edge against a straight boundary edge
leaving its end the other way escalated `curve_pair_pose_offset`, and
went from 411 to 0 at 1e-9. A straight boundary edge that shares an end
point with the edge now contributes no crossing cut. Two straight
segments from one point meet nowhere else unless they run together,
and a shared run is no cut either.
