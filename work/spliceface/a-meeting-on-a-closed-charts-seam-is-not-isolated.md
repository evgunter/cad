---
id: a-meeting-on-a-closed-charts-seam-is-not-isolated
kind: issue
title: The spline-face arm isolates no meeting on its chart's domain edge, so a span crossing a closed chart's seam, or ending on an open chart's edge, refuses
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
Krawczyk test (the image strictly inside the box) cannot isolate it. A
box at the floor that no meeting holds refuses
(`SplineCrossingUnsettled { why: Tangency }`), wherever it lies, ends of
the span included. PR 5826's first head dropped such a box where it lay
on the chart's edge within reach of an end; its review showed that
silently lost a tangency beside an open chart's edge, and a transverse
crossing on a closed chart's seam
(`review_r2_a_crossing_on_a_closed_charts_seam_near_an_end_is_not_dropped`).
The skip is gone, so both cases now refuse, and nothing is dropped.

What remains is the refusal:
- **a closed chart's seam** (a revolved NURBS face, a loft closed round
  its section): the domain's edge is interior to the face, and a span
  crossing the face there refuses;
- **an open chart's edge**: a span whose end lies on the face's
  boundary, a fragment the boundary pass split there among them, has
  its end's meeting on the domain edge, and refuses for want of a box
  that isolates it.

## The likely fix

Read a closed channel's boxes across the seam, the chart continued a
period on (the copies `spline_region` already reads a query at), so a
box straddling the seam is two windows of one box. On an open chart's
edge, isolate the end's meeting on the chart continued past its domain
(its end Bézier cells' polynomials), or accept a floor box decided
within the band of the boundary edge the boundary pass read at that
end.

Moved from SPLICE to SPLICEFACE on 2026-10-28, by `git mv` in SPLICE's split along its seams (Track size; Ev, in chat, 2026-10-28).
