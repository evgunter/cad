---
id: klein-wall-3-at-1e-12-floors-the-bulbs-edge-against-the-loops-one-section-wall
kind: issue
title: Klein wall 3 at ε 1e-12 refuses SplineCrossingUnsettled { why: Floor } once the loop's circle sections are one segment: the bulb's edge walked against the loop's one-wall NURBS face floors before the germ frame
status: open
opened: 2026-11-21
priority: P3
cost: M
---



Found by PROFILE's `circle-lowers-to-one-segment` (branch `profile/circle-lowers-to-one-segment`), which lowers `circle()` to one full-turn segment, so each of the Klein loop's `circle` sections lofts to one NURBS wall instead of two.

## What

`demos/tour/src/klein.rs` wall 3 (`union(bulb, top)`, the annular mate) pins `GermFrameUnsupported { a_kind: Cone, b_kind: Nurbs }` (`wall3_pinned`). At the default ε it still refuses there (`demos/tour/tests/klein.rs` `the_bottles_two_boolean_walls_name_the_pairs_the_scene_claims`). At ε `1e-12` (`demos/tour/tests/eps_regression.rs` `tour_runs_green_at_eps_1e_12`) it now stops one door earlier:

`SplineCrossingUnsettled { operand: A, edge: EdgeKey(2v1), face: FaceKey(2v1), face_of: B, why: Floor }`

an edge of the bulb walked against the loop's one-section NURBS wall, a piece at the walk's floor whose enclosure the band cannot decide. The same shape as the closed `teapot-wall-3-at-1e-6-floors-the-meridian-against-a-spout-wall`. The bottle cannot be one body either way (wall 3 is a documented wall); what moved is which door says so at the tightest row. Which bulb edge `2v1` is, and why the two-section wall did not floor, is not measured.

## What closing it takes

Read the floored piece: whether the walk's floor is the edge's tangency to the wall or the full-turn wall's enclosure width, then build the crossing or keep the floor typed. `wall3_pinned` admits this refusal beside the germ frame's, naming this row.
