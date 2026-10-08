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
either through the crossing sweep (`topo::sweep_split_admitting_cones`).
