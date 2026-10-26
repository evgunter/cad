---
id: a-point-probe-reads-no-nurbs-face-a-box-clears
kind: issue
title: point_in_solid refuses any selection holding a NURBS face, even for a point its certified boxes clear, so a loft disjoint from or around the other operand refuses at the containment fallback
status: open
opened: 2026-10-26
priority: P1
cost: E
---



Filed by SPLICE's `nurbs-edge-crossing-rung-is-the-ring-composite`
lane.

## What

`point_in_solid` refuses any selection that holds a NURBS face, as a
property of the selection and before any ray
(`boolean/solid_contain.rs`, `uncrossable`, `ray_crosses`). That holds
even for a query point the selection's certified boxes clear by
definitely more than the band.

With the crossing rung in, a spline edge's box is its control net's
hull (`boxes::EdgeBoxRule::ControlHull`). The fallback's NURBS re-gate
reads each NURBS face reach first (`ops.rs`, `sphere_extent_scan`). So
a loft far from a brick reaches the containment fallback's vertex
probe, and it refuses there: `sweep`'s `spline_spiric_operands` census
(the loft prism and the lofted cylinder, placements far and around) and
`s16_box_soundness::a_lofted_operand_reaches_the_fallbacks_re_gate`
read `Containment(KindUnsupported { kind: Nurbs })`.

## A fix

A box exit in `point_in_faces`: a query point definitely outside every
face's certified reach (`census::face_reach_in`, the box rules' one
map, a NURBS face's its control net) lies in the region at infinity,
and takes the selection's side there. The far placements of both
populations then build, against their closed-form oracles.
