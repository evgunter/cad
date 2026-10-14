---
id: the-census-spline-speed-floor-refuses-whole-arcs
kind: issue
title: The census reads a spline edge only where its control-net speed floor meters a length over the whole span; a single-span rational arc whose derivative net turns past a right angle refuses whole
status: open
opened: 2026-10-13
priority: P3
cost: M
---

Found by FUSE's review of PR 4946 (`census-curved-record-lanes-read-circles-only`, branch `fuse/census-reads-every-curved-edge`).

`census::curved::read` meters a spline edge's span by
`NurbsCurve3::speed_lower_bound`, a floor over the whole control net,
and refuses the edge typed where that floor meters no length over the
span (`pm_census_curved_speed_floor`). A floor can be zero on a regular
curve: a single-span rational semicircle's derivative net turns past a
right angle, its hull holds the origin, and the floor is zero, so every
record on such an edge refuses. The census's "every kind" is narrower
than that for splines; `read`'s doc says so. A span-local floor (per
knot span, or over the edge's own parameter window) is the fix.
