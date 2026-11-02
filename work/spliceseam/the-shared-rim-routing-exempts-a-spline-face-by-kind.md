---
id: the-shared-rim-routing-exempts-a-spline-face-by-kind
kind: issue
title: The boolean's shared-rim routing refuses a spline face by kind: its first-order screen reads classify_dihedral at stations it has no chart parameters for
status: open
opened: 2026-10-27
priority: P2
cost: M
---


Filed by the SPLICE lane that gave the result's seam a spline reading
(`the-results-seam-between-two-spline-walls-is-read-by-no-dihedral`),
from its sweep of `classify_dihedral`'s callers.

## What

`topo::boolean::rim_wedge::classify_shared_rim` routes a full-circle
shared rim by a first-order screen at `CERT_SAMPLES` stations around the
circle, then a material arm. Both read implicit forms, so a spline face
on either side is refused by kind before either runs:
`Indeterminate { predicate: "dihedral_wedge", margin: Invalid }`
(the "exempt by KIND" block above the screen). A lofted or swept body
whose wall closes on a circular rim reaches it.

## The likely shape

The screen is `geom_brep::classify_seam` at each station, with each
spline face read at the station's parameters on its own chart. The
routine takes surfaces and a rim, not the edge, so it has no pcurve to
read them off; the rim's half-edges' pcurves (`topo::pcurves::pcurve_of`)
are where they are, as `ops::seam_face` reads them for a seam's witness.
The material arm needs the same chart normals, signed by each face's
sense, over the same bracketed arm `classify_seam` reads (`NurbsLane::chart_arm`'s
lower bound at the station, and the reach).

Moved from SPLICE to SPLICESEAM on 2026-10-28, by `git mv` in SPLICE's split along its seams (Track size; Ev, in chat, 2026-10-28).
