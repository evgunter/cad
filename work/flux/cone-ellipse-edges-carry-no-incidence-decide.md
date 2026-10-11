---
id: cone-ellipse-edges-carry-no-incidence-decide
kind: issue
title: The cone's vector-area lane reads an ellipse edge as the plane section it is stated to be, with no incidence decide
status: dispatched
priority: P3
cost: M
refs: [4696]
opened: 2026-10-11
---


## What

`geom_brep::props::curved::require_on_cone` decides every line and circle
of a cone face on the cone, under the iso parse's names
(`props_meridian_generator`, `props_meridian_apex`,
`props_circle_axis_class`, `props_rim_fit`, the rim incidence). An
`Ellipse` edge passes undecided: the vector-area form reads it as the
plane section it is stated to be. This was true before PR 4696 too: the
ellipse path read endpoints only, and the iso parse refused ellipses
outright.

A cheap exact decide exists. Five points determine a conic, so five
well-spread points of the carrier ellipse (not of the edge's span) on
the cone make it the section. They would be decided against the cone's
point residual at the band, as `OffSurface`. Whether tier 2's
edge-on-surface check already covers it for bodies at rest decides the
priority. (FLUX cone-vector-area lane)
