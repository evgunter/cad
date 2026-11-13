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

## The cause, measured beside the curved lane (2026-11-12, `sectarms/spline-curved-at-rest`)

The rest lane's image is the coarser object, not its tolerance: it is
the degree-1 interpolant of 33 feet (`edge_nurbs::chart_image`), which
misses any foot path that bends in the chart by its chords' sag. The
curved-partner lane (`curved_nurbs_limbs`) shares this lane's body and
reads the pcurve mint's climbed image instead
(`general_chart_image_sampled`); with the degree-1 image a ball's arc
across the flat wall refuses the same way, `ImageMiss` at 4.6e-4 m. The
plane's lane was left on its degree-1 image here. What rest's image of
a fitted arc should be, for both lanes, is the choice
`a-curved-partners-arc-is-imaged-at-rest-no-finer-than-the-mints-finest-rung`
records.
