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
about 1e-15 of the distance of the points it reads from the origin.
Inferred from that ratio, not traced: it is the width of two points
evaluated hundreds of metres out, compared as if they could disagree. The junction gap compares two
different carriers' ends, so it is a real comparison, but its slack is
`3ε` at any distance.

Unmeasured: whether these reads would decide if the points were read
relative to the face's own frame rather than the origin.

## More evidence: `f64`, ε = 1e-9, a 10 km wall (2026-11-19)

From `oneround/section-area`'s end-to-end probe, merge base `7b752a7a`.
The plate of
`work/boxfinish/a-corner-cut-tens-of-eps-deep-finishes-with-a-definite-volume-of-the-wrong-sign.md`
at `R = 1e4`, split by the plane of normal `(1, 1, 0.05)` (normalised)
through `(10,1,1) − d·n`, at `d` = 1e-5 and 1e-3:

`Finish(ResultNotAtRest { side: Above, findings: [VolumeUncomputable { source: Face { face: FaceKey(5v1), source: Escalated { cause: Indeterminate { margin: 3.2841857285915325e-9, band: (1e-9, 1e-8), predicate: "props_chart_junction_closed" }, check: Inventory } } }] })`

(`3.3435036126743817e-9` at `d` = 1e-3). The section's edge on the arc
wall is an ellipse of semi-axes `(283019.4, 10000)` whose centre is
2.0e5 from the box. The same cuts on the plate at `R = 20` return `Ok`,
and on `R = 1e4` with normals `(1,1,1)`, `(0.05,1,1)` and
`(1,0.3,0.02)` they return `Ok` too. So this junction gap is not only
an `Interval`, ε = 1e-12 reading: a corner cut 10 µm to 1 mm deep on a
legal far wall refuses at rest at the default ε.

