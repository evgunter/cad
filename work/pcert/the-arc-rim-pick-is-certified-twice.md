---
id: the-arc-rim-pick-is-certified-twice
kind: issue
title: nurbs_iso_derive certifies an Intersection circle's arc-rim pick and the mint certifies the same row again
status: open
opened: 2026-10-14
priority: P4
cost: M
---


Found by the review of PR 5097 (`pcert/spline-pair-any-carrier`).

`topo::pcurves::nurbs_iso_derive`'s circle fall-through certifies the
arc rim's pick (`PcurveCache::certify`) to decide whether the circle is
that row or falls through to `General`, and returns the pick as a
`Pcurve`. The mint then certifies the same row again (`restate`). Only
an `Intersection` circle on a spline chart reaches this arm, so the cost
is one extra closed-form certification per such rim. Returning the
certified row and letting the mint take it would remove the second.
