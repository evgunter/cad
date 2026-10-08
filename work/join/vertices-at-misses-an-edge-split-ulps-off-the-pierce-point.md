---
id: vertices-at-misses-an-edge-split-ulps-off-the-pierce-point
kind: issue
title: near_tangent_battery's edge placement: vertices_at's exact match misses the cube edge's split ulps off v on 324 lines
status: open
opened: 2026-10-07
priority: P3
cost: E
refs: [a-near-tangent-pierce-reads-two-cones-where-its-link-holds-one]
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

## Built

Branch `join/battery-hygiene`. This is the kernel fix:
`vertices_at` keeps its exact match.

**Where the ulps came from.** It was not the vertex-on-face lane. The
cube-first ops sweep the cube's edge against the corner's faces.
`contfp` puts the pierce `OnVertex(v)`, and the edge is split at the
parameter read back from `v`. `split_edge` mints `carrier(t)`, which is
2.2e-16 off `v` on `Ltop nt e0 a1 d1e-3 edge`. The merge keeps operand
A's point of the v-v pair, so the cube-first ops ship the split's
point.

**The fix.** `Body::split_edge_onto` (in `topo/src/split.rs`) is
`split_edge` with the new vertex holding a point the caller already
has. Both children certify against that point, so a point off the
carrier past the band still refuses. `boolean/reduce.rs` uses it
where a split lands on an existing point:
- the three `OnVertex(vy)` arms of `sweep_direction` take `vy`'s
  point;
- `split_other_at_point` takes the event point `p`. That is the
  existing vertex in the vertex-on-face lanes, and the piercing edge's
  split in the edge-edge lanes.

**Measured** in release, main `047d10d5` against the head:
- `pierce_runs_battery`, `pinch_runs_battery` and
  `corner_pairs_battery` are byte-identical (4 539, 3 027 and 16 383
  lines).
- `rc_wide_battery`, all 84 shards (40 740 lines): 36 lines move, only
  in the trailing digits of an `Escalated { Coincidence(Sectors,
  Moot) }` margin. Their kinds and verdicts are unchanged.
- `near_tangent_battery`, now read at `v` in the edge placement too
  (7 200 lines): every body reads one vertex per cone (3 368 edge
  lines, 3 299 face lines). The 324 edge lines that read 0 vertices
  before the fix now read one vertex per cone. Three escalation lines
  (`Ltop nt e1 a2 d1e-5 edge ca`) move only in their margin's digits.
  No outcome moves otherwise.

The guard is `join_pierce_runs_sweep.rs`
`an_edge_split_on_the_corner_keeps_its_bits`.
