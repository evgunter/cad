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
shared rim through the whole-edge fold (`geom_brep::edge_wedge_over`
over `geom_brep::WedgeWalk::around_circle`, `CERT_SAMPLES` phases round
the circle), whose first-order and material reads both read implicit
forms, so a spline face on either side is routed by kind before the fold
runs: `RimRouting::SplineFace`, which `boolean::route_rim` refuses as
`BooleanError::TangencyUnsupported`. A lofted or swept body
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

Moved from SPLICESEAM to SPLICE on 2026-11-02, by `git mv`, when SPLICESEAM merged back into SPLICE: the track budget now counts only the P0, P1 and unbanded rows a sitting dispatches (Ev, in chat, 2026-11-02), so the four tracks of SPLICE's 2026-10-28 split came in under one budget.
