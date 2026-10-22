---
id: the-side-readers-aims-decide-exact-ties-at-a-documents-nominal
kind: issue
title: The side reader's aims land on exact ties at symmetric nominals
status: parked
priority: P2
opened: 2026-10-19
blocked_on: [a-ray-meets-a-curved-faces-edge-in-band-by-each-faces-own-trim]
cost: H
---

Since PR 5119, check 7 reads each solid with the side reader's aimed
rays (`crates/topo/src/side_reader.rs`, `aim_at`), so every at-rest
gate takes the aims' own decisions. Two aims sit on symmetric points of
the stored geometry, where those decisions tie exactly at a document's
nominal:

- a planar face aims at its certified interior point
  (`crates/topo/src/stands.rs`, `planar_interior_point`), whose in-face
  walk decides `point_in_loop_arm` against an axis-aligned schedule
  direction: `die_chamfer` reads it exactly 0 at decision 1617 of 4547;
- a cylinder, cone or torus face aims at the middle of its chart window
  (or the point (π, π)), where the ray's foot on a circular edge decides
  `curve_foot_branch`: `cut_cylinder` reads two exactly 0.

The reader's answer turns on none of them (`[verdicts]`), and its
decisions are recorded under one family (`topo::SIDE_READER_FAMILY`,
`geom_core::k_stats::in_family`, tagged on the symbolic recorder too),
which `editor-core`'s N5 flip lane and `revalue_corpus` read apart, so
no row moves. What remains is the instrument representing the tie at
all.

**The fix is the edge-meeting ray**
(`work/flush/a-ray-meets-an-edge-in-band-as-half-a-crossing-for-each-face.md`),
which decides each edge once for both faces it bounds and stops
representing those ties.

## Measured on the edge-meeting ray

`crates/editor-core/tests/revalue_corpus.rs` `reader_ties_at_the_nominal`
(ignored) lists every decision under `topo::SIDE_READER_FAMILY` whose
margin is exactly 0 in a build at the nominal:
- `die_chamfer`: 2 on main (`point_in_loop_arm`, decisions 1616 and
  1647), **0** with the edge-meeting ray. A line-bounded planar face aims
  at its loop's mean point, which asks nothing, and is crossed edge by
  edge or by the turn of its loops (`ray_crossings`), so no in-face walk
  casts an auxiliary direction. The planar half is retired.
- `cut_cylinder`: 90 on main and 90 with it (`curve_foot_branch` 12,
  `point_in_arc_loop_side` 6, and the structural wall and chart reads
  `bool_wall_iso_rim`, `bool_wall_iso_meridian`, `bool_wall_section_seat`,
  `pcurve_chart_radial_moving`). The curved aims and trims are unchanged,
  so the wall half waits on the curved edge-meeting ray.
