---
id: an-ellipse-stated-as-another-charts-image-on-a-spline-chart-takes-no-general-lane
kind: issue
title: An ellipse described as another chart's image or as scaffolding on a spline chart refuses IsoUnsupported at the mint's catch-all instead of taking the general lane
status: open
opened: 2026-11-04
priority: P2
cost: C
---


Filed by `pcert/curved-general-image` (the row
`a-curved-carriers-degree-one-general-image-ends-as-a-kernel-bug`), from
its sweep of the mint's lane choices on a spline chart.

## What

`topo::pcurves` `nurbs_iso_derive` dispatches on the description form
before the carrier. A circle reaches the arc rim in every form, and its
fall-through is the general lane (`general`). A line reaches
`iso_schedule` in every form. An ellipse (or any other curved carrier
with no net) reaches the general lane only through an `Intersection` or
`OnPair` description (`iso_schedule("pcurve_iso_seam_column", …)`). In
the other forms (`Chart` naming another chart with an image that is not
an `IsoLine`, `Scaffold`, `TangentIntersection`) it falls to the
catch-all arm, which reads the chart's incidence and otherwise refuses
`IsoUnsupported` ("no iso derivation for this locus on a NURBS chart").

The same ellipse on the same wall therefore mints its `General` image
or refuses, by the form its description takes. E5 made this
form-independent for circles (its T2).

## Fix

Send a curved carrier in the catch-all's forms to the same `general`
closure the arc rim and `iso_schedule` share. The `TangentIntersection`
form may want its own ruling, since a tangency's locus is a structural
reading. A row: an ellipse on a spline wall described as the plane
cap's chart image takes the general lane.
