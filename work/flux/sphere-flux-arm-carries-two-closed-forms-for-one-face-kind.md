---
id: sphere-flux-arm-carries-two-closed-forms-for-one-face-kind
kind: issue
title: The sphere flux arm measures a tilted-circle face by Gauss-Bonnet and every other face by the iso-rectangle form, where the first subsumes the second
status: open
opened: 2026-10-02
priority: P1
cost: M
design: true
---


## What

Found by the `reach/tilted-sphere-pair` lane, which added the second.
`geom_brep::props::curved::sphere` dispatches on
`sphere_loop_has_tilted_circle`: a face with a boundary circle tilted
against the chart takes `sphere_circle_loop` (area by Gauss–Bonnet over
its circle arcs — constant geodesic curvature per arc, turning angles
at the vertices — flux `R·Area` under the sense bit plus `c·A⃗`); every
other sphere face keeps the iso-rectangle arm (`sphere_boundary`, the
rim and meridian parse, `props_rim_level`, the wedge and two-band
branches).

Gauss–Bonnet is exact for every one-loop sphere face bounded by circle
arcs, so it measures every face the iso arm measures, and the L-shaped
faces the iso arm refuses (`props_rim_interior_side`, issue 1598) too.
Two closed forms for one face kind is the P1 shape: the S58 guards the
iso arm carries exist because its formula is only right on a
rectangle, and a face measured by the other formula needs none of them.

## The question

Retire the sphere branch of the iso arm in favour of the loop form (the
rimless band's and the wedge's `Δu` derivations, and the rim-side
premises, then have no consumer on the sphere; their bits move by
rounding), or keep the iso arm where it applies and state why. What the
two encodings of the radial side mean on the loop form — it reads the
sense bit, as the rimless band does, so `boundary_material_sign` has
nothing to cross-check it against — is part of the same answer.
