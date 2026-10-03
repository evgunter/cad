---
id: split-gate-zone-ignores-the-azimuth-window
kind: issue
title: The split gate bounds a sphere face by its whole latitude zone, so a cut clear of a partial-azimuth cap refuses
status: open
opened: 2026-10-03
priority: P1
cost: M
---


Found by PR 3982's dual review (reviewer R1), and disclosed in that PR.

## What

`splitting/classify.rs` `sphere_zone_reach` bounds a sphere face by
`zone_extent` over its LATITUDE window alone. The trim it reads
(`solid_contain::sphere_chart_trim`) also pins an azimuth window
(`SphereChartTrim::az`), which the zone ignores: a cap revolved through
part of a turn is bounded as the whole ring of latitudes.

Measured by R1 on the capped cylinder (the unit cylinder under the cap
of the sphere of radius 5/4 about `(0, 1/4)`) revolved through 2.0 rad:
87 of 156 cuts clear of the face refuse
`CurvedBooleanUnsupported { kind: Sphere }`, in every pose and at every
scale. Classified against the full turn, all 156 split. The trim
certified on all 1416 calls, so the window was there to read.

This is sound looseness: the ring contains the face.

## The general form

The support of a sphere's latitude-and-azimuth rectangle along a
direction has the same shape as the torus rectangle's
(`classify::torus_rect_extent`). A sphere point is
`c + r·(cos v·ê(u) + sin v·â)`, so `e·p − e·c = r·cos v·A·cos(u − u*) + r·sin v·a`.
`cos v ≥ 0` on the sphere chart, so the best azimuth does not depend on
the latitude: take `M` as the best `A·cos(u − u*)` over the azimuth
window, and the latitude part is `r·√(M² + a²)·cos(v − v*)` over the
latitude window. It is `torus_rect_extent` with `R = 0`. Fold it with
the zone into `FaceBoxRule` (`boxes/split-gate-sphere-zone-folds-into-face-box-rule`)
rather than adding a third gate-local spelling.
