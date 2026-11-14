---
id: a-far-walls-junctions-and-ruling-incidence-escalate-at-the-interval-scalar
kind: issue
title: A far legal cylinder or torus wall's junction closure, ruling incidence and meridian fit escalate at Interval at eps 1e-12, on the width of points read hundreds of metres out
status: open
opened: 2026-11-14
priority: P3
cost: M
---


Found by BOXFINISH's edge-turns review (branch `boxfinish/edge-turns`,
PR 6946), beside
`a-carried-arcs-miss-is-charged-its-ends-enclosure-width-at-the-interval-scalar`.

## Measured

The setup is the same as that row's: hand-built legal walls through
`curved_face_loops`, frame at (30, −110, 200), axis (1, 1, 1)/√3, at
`Interval` and ε = 1e-12. The pins citing this row are in
`crates/geom-brep/tests/far_walls_of_many_arcs.rs`.

- **`props_chart_junction_closed`:**
  - cylinder r 410, 40 / 200 arcs a rim: `[0, 1.08e-12]` / `[0, 1.04e-12]`;
  - the whole-rim cylinder band r 990: `[0, 1.28e-12]`;
  - torus major 300, 6 / 40 / 200 arcs a parallel: `[0, 1.20e-12]` / `1.14e-12` / `1.02e-12`;
  - torus major 600, 1 / 6 / 40 / 200 arcs: `[0, 1.75e-12]` to `[0, 1.07e-12]`.
- **`props_meridian_on_surface`** (the ruling's incidence): cylinder
  r 990, every arc count, `[0, 1.48e-12]`.
- **`props_meridian_fit`:** torus major 990, every arc count,
  `[0, 1.02e-12]`.

At ε = 1e-9 and 1e-6 every one of these walls measures. Each margin is
about 1e-15 of the distance of the points it reads from the origin:
it is the width of two points evaluated hundreds of metres out,
compared as if they could disagree. The junction gap compares two
different carriers' ends, so it is a real comparison, but its slack is
`3ε` at any distance.

Unmeasured: whether these reads would decide if the points were read
relative to the face's own frame rather than the origin.
