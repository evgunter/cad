---
id: vertices-at-misses-an-edge-split-ulps-off-the-pierce-point
kind: issue
title: near_tangent_battery's edge placement: vertices_at's exact match misses the cube edge's split ulps off v on 324 lines
status: open
opened: 2026-10-07
priority: P3
cost: E
design: true
refs: [a-near-tangent-pierce-reads-two-cones-where-its-link-holds-one]
branch: join/battery-hygiene
---


`sweep/tests/join_pierce_runs_sweep.rs` `near_tangent_battery` reads cones
at `v` in the face placement only. In the edge placement (`v` on the
cube's edge), the cube-first (`ca`) ops hold `v` as the cube edge's
split, 5.6e-17 to 2.5e-15 off `v`, where the corner-first ops keep `v`'s
own bits. So `common/pinch_cones.rs` `vertices_at`, an exact point match,
reads 0 vertices for 1 or 2 cones on 324 lines. A 1e-9 match reads every
one of them one vertex per cone (measured on PR 4250's head). Owed:
decide whether the vertex-edge lane should keep the vertex's point
bitwise in either order, or whether the counter should read the point
within the band; then read cones in the edge placement too.
