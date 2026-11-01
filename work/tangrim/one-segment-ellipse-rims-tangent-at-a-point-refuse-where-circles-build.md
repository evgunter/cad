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
rest and turned, with both vertices there (hole `π`, boss `0`) and with
the boss's there alone (both `0`): `Join(SingleSiteSectionLoop { count: 1 })`
(`boolean/join.rs` `closes_at_its_site`: a closed edge that is not a
circle has no copy). Each closed rim's germ is read from its own
carrier against the other's faces (`boolean/sectors.rs` `closed_reach`,
since PR 6177); before, with both vertices there, the two rims read as
one segment and refused `GermEdgeCarrierUnsupported` (`one_segment`: a
closed edge that is not a line or a circle).

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

## Built (2026-11-01, `tangrim/one-segment-ellipse-rims`)

Measured first on `main` `7979553590`, at ε = 1e-6, 1e-9 and 1e-12:
- **The union.** One segment refused `Join(SingleSiteSectionLoop { count: 1 })`, raised in `closes_at_its_site`: `site_cell` read the closed ellipse as `Closed(None)`, and `chord_join.rs` `closed_edge_spec` took only a circle. The two-arc form built. The readers below the join (`closed_reach`, `slit::carrier_on_surface`) already read an ellipse.
- **Tier 3′.** On every union, both forms, it escalates `CensusEscalated { predicate: "curve_pair_coplanar_sub_side" }`: the census reads the two coplanar rims' touch through the curve-pair reader. That is `work/cpair/two-coplanar-ellipses-have-no-meeting-method.md`'s open touch, and the evidence is added there.
- **The pad.** The tilted boss alone, before any op, measures a volume pad of `4.4e-7` (one segment, ε = 1e-9), `2.9e-5` at 1e-6 and `8.6e-10` at 1e-12. The holed plate measures `1.3e-7` at 1e-9. The differences give back an operand, so they carry its pad. At 1e-12 every op is within the `SOUND` budget.

The fix widens `site_cell` and `closed_edge_spec` to a closed ellipse: the copy is the edge's own carrier and span, stated in the face's chart. Every op in both forms, both orders, at rest and turned, now builds at its closed form, tiers 1–3 and the certificate, at all three ε. Each passes tier 3′ except the unions (cpair's touch above). Pinned by `ellipse_rims_tangent_outside_build_every_op`, which replaces `one_segment_ellipse_rims_tangent_outside_refuse_the_union_typed`.

Left:
- the unions' tier 3′, on cpair's row;
- the rims with no vertex at the touch, which refuse every op `CurvedPierceUnsupported` (`ellipse-rims-tangent-with-no-vertex-at-the-touch-refuse-curved-pierce`).
