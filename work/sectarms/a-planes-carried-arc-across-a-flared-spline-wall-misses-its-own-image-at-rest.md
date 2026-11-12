---
id: a-planes-carried-arc-across-a-flared-spline-wall-misses-its-own-image-at-rest
kind: issue
title: A plane's carried arc across a flared spline wall refuses at rest: the plane x spline lane's own image misses the carrier's foot path
status: open
opened: 2026-11-12
priority: P1
cost: M
---



Filed by the `sectarms/join-reads-spline-implicit` lane.

## What

A brick across the loft prism's flared wall (`x ∈ [−0.25, 0.35]`,
`y ∈ [−1.3, −0.8]`, `z ∈ [0.75, 1.25]`) is now joined on the plane ×
spline section's carried component: both the plane's chord and the
wall's are cut from the whole reading's certified carrier. The chord's
edge then refuses at rest in `edge_nurbs::plane_nurbs_limbs`:
`PlaneNurbs(Limb { limb: ImageMiss, margin: 6.0e-6 m })` (3.3e-6 m for
the brick at the rim). The lane's own image of the carrier, fitted
through its feet (`chart_image`), misses the carrier's foot path at a
break by more than the run tolerance; the carrier is not shown off the
surfaces.

So a carrier the 1×2 trace certified with limb 2 at about `1e-16` m
(the plane's case in `carry_rows`) does not re-certify at rest on the
plane × spline lane, though C2 says rest re-derives the door's
certificate. Either the rest lane's image is too coarse for a cut arc
of a trace carrier, or the two lanes' limb-2 tolerances differ.

Witness: `a_brick_across_a_lofted_wall_is_joined_on_its_carried_arcs`
(same file as above) pins `CertifyError::PlaneNurbs(_)` for it.
