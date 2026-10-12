---
id: a-line-parallel-to-a-cone-generator-has-no-root-lane
kind: issue
title: A line edge parallel to a cone's generator, a generator lying on the cone included, is not answered by the line x cone lane: its quadratic degenerates and the lead rung keeps the frontier
status: open
opened: 2026-10-06
priority: P2
cost: M
refs: [4135]
---


Left by the lane that gave a cone face its crossing lane
(`an-edge-crossing-a-cone-face-has-no-root-lane`).

## What

`reduce::line_cone_roots` decides the line × cone quadratic
`G = A t² + 2B t + C` (`solid_contain::line_cone_quadratic`). Its lead
rung (`bool_line_cone_lead`, `A/|dir|²` levered by the segment's
length) reads ZERO for a line parallel to a generator, where `G`
degenerates to a linear function with at most one root, and the lane
answers `CircleRoots::Uncertain`: the frontier
(`CurvedPierceUnsupported`). Two poses take it:

- a line parallel to a generator and off the cone: one crossing (of
  the nappe the line runs along, or none), the linear root
  `t = −C/2B` read with its own depth rung;
- a GENERATOR lying on the cone (`A = B = C = 0`): an edge on the
  carrier, which a wall's ruling answers `SpanVerdict::Constant` and
  the endpoint arms then read as an ON event or a cosurface question.

Row: `reduce::line_cone_rows::a_miss_a_graze_and_a_ruling` pins the
first pose's refusal. No finished body has been built that reaches
either through the crossing sweep (`topo::sweep_split`).

## 2026-10-09, a finished body reaches the second pose (VERBS-CONE U7)

With `Cone` on the operand roster, the preview cone of
the cone admission's spec (`docs/doc-ledger/germ-verbs-cone-spec.md`) against a quarter frustum sector of its
own carrier (`(0.1, 0.2) (0.8, 0.2) (0.4, 0.6) (0.1, 0.6)` revolved
`π/2` about `y`) refuses B ∖ A `CurvedPierceUnsupported` on the
sector's generator edge, which lies on the cone, and the other three
ops `CrossingAtConeApex` on the cone's seam generator, whose line runs
through the sector face's apex. Pinned as a refusal in
`sweep/tests/cone_operand_rows.rs`
(`a_tangent_plane_and_a_sector_on_the_cone_face_refuse`): a cone sector
lying on a cone face is a coincidence D10 holds, so the row stays a
refusal whatever this item lands.

## 2026-10-11, a prism edge along a cone's ruling (FLUSH)

A square prism whose corner edge is a ruling of a frustum's wall (the
frustum about `y`, radius 1 at `y = 0` to `0.5` at `y = 2`, the edge at
azimuth `0.7` between `y = 0.5` and `1.5`, the square opening into the
cone) refuses `CurvedPierceUnsupported` on that edge in all six ops
and orders, before any record is carried: the second pose above,
reached by a finished planar operand. Pinned as a refusal in
`sweep/tests/a_ruling_through_a_box_corner_edge.rs`
(`a_prism_edge_along_a_cone_ruling_refuses_at_the_crossing_layer`).
The cylinder's twin of the pose builds, and once this lane answers
the cone's, the cone's owes what that file's cylinder test checks:
the edge's two ends recorded as v-f rests on the piece of the wall
that holds them (`boolean::ops::Descendants::holder`).
