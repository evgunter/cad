---
id: one-segment-ellipse-rims-tangent-at-a-point-refuse-where-circles-build
kind: issue
title: One-segment ellipse rims tangent at a point refuse the union typed (no join arm copies a closed ellipse); the two-arc form builds it, failing tier 3′
status: open
opened: 2026-10-27
priority: P1
cost: H
---

Found by PR 5808's claims review (`one-segment-tangent-rims-refuse-where-their-two-arc-forms-build`).

## The pose

The boss `r = 0.3` and the hole `r = 0.1` about `(0.4, 0)` of
`a_closed_conic_in_the_partners_face.rs` `a_boss_over_a_hole_by_pose`,
both cylinders tilted `0.3` rad about `x` through the origin and cut by
`z = 0` (the boss intersected with the slab `z ∈ [0, 1]`, the plate
`[−1, 1]² × [−0.5, 0]` less the tilted bore). Both rims are ellipses
tangent at `(0.3, 0, 0)`.

## What it reads

In the one-segment form the union refuses typed in both orders, at
rest and turned, as `main` does:

- vertices there (hole `π`, boss `0`): `GermEdgeCarrierUnsupported`
  (`boolean/sectors.rs` `one_segment`: a closed edge that is not a line
  or a circle);
- the boss's there alone (both `0`): `Join(SingleSiteSectionLoop { count: 1 })`
  (`boolean/join.rs` `closes_at_its_site`: a closed edge that is not a
  circle has no copy).

Pinned by `one_segment_ellipse_rims_tangent_outside_refuse_the_union_typed`.
The two-arc form's union builds at the closed form but fails tier 3′
(measured by the review's probe, on `main` and the PR alike), so neither
form is right yet. The differences build at the closed form with a
volume pad over the `SOUND` budget (`4.4e-7` m³).

## What it needs

`chord_join.rs` `closed_edge_spec` copies only a circle
(`arc_along` → `EdgeCurveSpec::arc_of_circle`); a closed ellipse needs the
same copy on its own carrier, and `closed_reach` (which reads a circle
only) and `closes_at_its_site`'s guard then widen to it.

## Slate triage

2026-11-01: moved from `work/cleave/` by CLEAVE's split on its semantic seams (Ev, in chat: CLEAVE measured 114 budget points against 30). TANGRIM holds the tangent curved rims and crescents; id and body unchanged. (CLEAVE orchestrator)
