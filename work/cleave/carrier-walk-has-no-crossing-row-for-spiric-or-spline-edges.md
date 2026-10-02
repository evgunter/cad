---
id: carrier-walk-has-no-crossing-row-for-spiric-or-spline-edges
kind: issue
title: The in-plane carrier walk has no crossing row for a spiric or spline edge, so a planar face bounded by one refuses containment near that edge
status: open
opened: 2026-10-01
priority: P2
cost: M
---

Filed by TANG's re-measurement of `work/tang/arc-aware-point-in-loop.md`
(2026-10-01): that row's item 3 remainder, left once its circle and
ellipse halves were answered.

`splitting::containment::point_in_carrier_loop` and `carrier_loop_side`
cross lines on their chords and circle and ellipse arcs on their conics
(`ConicArc::of` in `carrier_loop`). A `Spiric` or `Nurbs` edge has no
crossing row. `carrier_loop` holds it as `LoopEdge::Unrowed { center,
reach }`: the spiric's arc as a ball around its midpoint sized by its
speed bound, and the spline as its control hull's ball. The walk answers
only along a ray that definitely misses every such ball. A point inside
one, or a point every scheduled ray from which could meet one, gets
`None`, and the callers refuse it:

- `ContainError::ArcLoopUnsupported` from `boolean::contain::contfp`,
  surfaced as `BooleanError::ArcLoopContainmentUnsupported` by
  `boolean::reduce` and `boolean::ops`;
- `PointInSolidError::EdgeCarrierUnsupported` from
  `boolean::solid_contain::point_in_face`. Through `point_in_solid` it
  reaches the join's role probe (`boolean::shell_witness::complex_side`)
  as `BooleanError::Containment`, because `inconclusive` does not list
  it. Tier 3's planar-face witness (`certified_in_face`) instead discards
  the candidate.

**The body class is reachable.** Built bodies have planar faces bounded by
a spiric. `crates/sweep/tests/pis_arc_capped_poses.rs`,
`a_spiric_bounded_face_refuses_only_within_its_reach`, finds one on the
shelled vessel's cavity and pins `EdgeCarrierUnsupported` at the spiric's
midpoint. Nobody has measured whether a public boolean reaches the
refusal, which needs a containment probe that lands inside the spiric's
ball.

**What a fix needs**: a crossing row for each carrier, folded into
`carrier_walk` beside the conic row: a certified ray × spiric root count
in the section's plane, and a ray × spline count by subdivision against
the control hull. How the callers name today's `None` is a separate
question, filed as `work/cleave/carrier-walk-none-is-answered-four-ways.md`.
