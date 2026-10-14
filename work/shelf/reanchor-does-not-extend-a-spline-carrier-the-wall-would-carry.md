---
id: reanchor-does-not-extend-a-spline-carrier-the-wall-would-carry
kind: issue
title: replace_face's re-anchor refuses ReanchorPastCarrierEnd where an edge's spline carrier ends at the moved face even though the wall surface extends past it
status: open
opened: 2026-10-06
priority: P3
cost: M
---


Filed by the `shell/lofted-wall-seam` lane. Measured on the M7-8 cube
(`crates/topo/tests/fixture/m7_8.rs`), pinned by
`crates/topo/tests/spline_reanchor_rows.rs`,
`an_outward_cap_offset_runs_past_the_spline_carriers_end`.

## Measured

The cube's `y = 0` wall is a described NURBS patch spanning
`[-1, 2]²` in `x, z`, wider than the cube; its four edges ride degree-1
NURBS carriers minted only as long as the edges. Offsetting the top cap
(or a side) inward re-anchors them (`Ok`); offsetting it outward by
0.25 refuses `ReanchorPastCarrierEnd { gap: 0.25 }` — the moved vertex
is past the CARRIER's end, though the wall surface would carry the
edge on to it.

## Why

`replace_face::plan_reanchors` re-reads an endpoint as the carrier's
foot and never rebuilds the carrier. For a lofted wall seam the
carrier ends where its patch does and the refusal is the whole truth;
for a plane × NURBS edge whose wall extends, re-deriving the carrier
over the longer range (the plane × NURBS lane mints that class) would
let the outward offset through.

## Fix

At a `ReanchorPastCarrierEnd` on an edge whose description is an
intrinsic intersection of surfaces that extend past the carrier's
domain, re-mint the carrier over the moved range through the edge's
own lane instead of refusing. Not needed by any measured shell (the
shell offsets inward); filed so the refusal is not read as the wall's
limit.
