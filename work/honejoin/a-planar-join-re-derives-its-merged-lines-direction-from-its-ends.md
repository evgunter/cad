---
id: a-planar-join-re-derives-its-merged-lines-direction-from-its-ends
kind: issue
title: join_one re-derives a merged planar edge's line from its two end points where both members held a certified line carrier
status: open
opened: 2026-11-14
priority: P3
cost: E
---


Found by BOXFINISH's sweep for `boxfinish/sweep-frames` (the shape "a stored direction re-derived from points the construction placed"), not measured over a box.

`crates/topo/src/boolean/edge_join.rs` `join_one`, the planar arm: the kept edge's spec is `EdgeCurveSpec::line_between(m.start, m.end)`, a line whose direction and span are re-derived from the merged member's two end points. Both edges being joined hold certified `Curve3::Line` carriers on one line (and the faces' normals give `n_f × n_g`); the curved arm and `fate.rs` / `chord_join.rs` already restate a held carrier through `EdgeCurveSpec::segment_of_line(carrier, t0, t1)`. Over a parameter box the end points' width enters the stored direction, which tier 3 then reads as tilt.

**Repair shape.** Restate the kept edge on its own carrier over the merged parameter span (`segment_of_line`), as the curved arm does.
