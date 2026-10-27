---
id: a-meeting-on-a-closed-charts-seam-is-not-isolated
kind: issue
title: The spline-face arm isolates no meeting on a closed chart's seam, so a span crossing a revolved NURBS face there refuses
status: open
opened: 2026-10-27
priority: P2
cost: M
refs: [an-analytic-edge-against-a-nurbs-face-has-no-crossing-lane, a-spline-faces-trim-reads-nothing-at-a-closed-seam-or-a-pole]
---

Filed by the SPLICE lane that built the analytic span × spline face
arm (`geom_brep::curve_face::spline_face`).

## What

The arm walks boxes of `(t, u, v)` inside the chart's knot domain, so
a meeting exactly on the domain's edge lies on a box's face and the
Krawczyk test (the image strictly inside the box) cannot isolate it.
On an open chart that edge is the face's boundary, read first by the
boundary pass, and a floor box on it within reach of an end of the
span is that end's. On a chart CLOSED in a channel (a revolved NURBS
face, a loft closed round its section) the domain's edge is a seam
interior to the face: a span crossing the face on the seam stops at a
floor box no meeting holds and refuses
`SplineCrossingUnsettled { why: Tangency }`.

## The likely fix

Read a closed channel's boxes across the seam, the chart continued a
period on (the copies `spline_region` already reads a query at), so a
box straddling the seam is two windows of one box.
