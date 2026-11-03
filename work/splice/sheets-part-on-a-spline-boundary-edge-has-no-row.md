---
id: sheets-part-on-a-spline-boundary-edge-has-no-row
kind: issue
title: reduce::sheets_part's held parameter on a spline boundary edge has no row: a covered edge off the carrier meeting a spline edge of a curved analytic face
status: open
opened: 2026-11-03
priority: P3
cost: M
---


Filed by the split-parameter lane (`splice/split-parameter`), from PR
6441's review.

## What

`topo::boolean::reduce::sheets_part` reads the boundary edge's tangent
at the meeting's held parameter (`carrier_cross::Held`) where one is
carried, else at the carrier's read-back (`point_param`). The held arm
is measured on circle and ellipse boundary edges: 63 readings across
`crates/sweep/tests`' `sheets_part_on_slanted_rims`, `face_face_touch`
and `full_turn_bore_mate`. Reading the tangent at the covered edge's own
`t` there instead turns three `sheets_part_on_slanted_rims` rows red.
No row reaches it with a spline boundary edge, which is the case the held
parameter exists for. That case has no read-back, so without the held
parameter it refuses `PointSplitCarrierUnsupported`.

## The row owed

A curved analytic face (a cylinder wall) bounded in part by a NURBS
edge lying on its carrier, and a covered edge off the carrier (a rim of
a tangent wall, as in `sheets_part_on_slanted_rims`) meeting that edge.
The row pins the parting reading at the held parameter, and the
refusal where the parameter is dropped.

The producer is unmeasured. Such a face is the result of a boolean whose
section with a spline face lies on an analytic carrier. Whether one
exists today is the first thing to measure.
