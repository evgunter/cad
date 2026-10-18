---
id: a-rigid-move-levers-its-certificate-at-the-source-bodys-reach
kind: issue
title: transform_rigid and the boolean graft certify at the source body's wedge reach, where tier 3 reads the moved body's
status: open
opened: 2026-10-16
priority: P3
cost: E
---

`Body::transform_rigid` (`crates/topo/src/transform.rs`, the `reach` before `policy_lane::certify`) and the boolean's graft (`crates/topo/src/boolean/combine.rs`) certify each moved edge at the wedge reach read off the SOURCE body (`topo::wedge_reach(body, …)`), because `out` is half mapped mid-walk. Tier 3 then reads the moved body's own reach. A rigid motion keeps every distance, so the two agree up to the rounding of mapping the vertices: an edge whose wedge margin sits within that rounding of a band edge can certify at the op and read differently at rest, or the reverse. Not observed; no row reaches it.

To close: either read both at the source with a comment bounding the rounding (the vertex map's error times the dihedral's sine, far under ε for any lever the suite builds), or read the reach from `out` after the walk. Filed from U-lever's second review (PR 5204, F6).

