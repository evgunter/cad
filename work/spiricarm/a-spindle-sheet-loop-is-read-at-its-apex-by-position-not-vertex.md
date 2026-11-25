---
id: a-spindle-sheet-loop-is-read-at-its-apex-by-position-not-vertex
kind: issue
title: A spindle sheet loop passing an apex point twice is measured by position, with no reading of whether both passes are one vertex
status: open
opened: 2026-11-25
priority: P3
cost: E
---


Filed by LATHE (`lathe/pinched-cone-loop`, PR 7585's review fix pass), from the sweep the review asked for of props' other pinch readers, on SPIRICARM's slate because the torus sheet lanes are SPIRICARM's.

## What

`sheet_apex_jumps` (`crates/geom-brep/src/props/curved.rs`, at `props_sheet_apex_junction`) reads a junction between two meridians as an apex pass wherever its point is decided on the axis. Each such pass gets its own jump, and nothing asks which vertex the pass is at.

The cone's sibling reader had the same split, and PR 7585's review demonstrated it: a cone loop passing its apex point at two distinct vertices, one per cone of material meeting there, was measured, and the body built where tier 3′ and the mesher refuse it. The cone now reads the passes by vertex tag (`require_apex_lobes`, `props_cone_apex_vertices`), which agrees with `topo::loop_points`' pinch (`crates/topo/src/loop_points.rs`, `the_props_tags_part_a_group_exactly_where_it_pinches`).

No witness has been built for a spindle sheet. The question is whether a sheet face's loop can pass one apex twice. If it can, the case of two distinct vertices there needs the same identity reading. If it cannot, a pass at a second vertex should refuse by name.

## Where to look

Collect each junction's vertex tag (`into.end`) per apex (upper and lower). Refuse a second tag at one apex, under a name of its own, as the cone does.
