---
id: an-end-on-a-spline-face-is-placed-in-space-not-on-its-sheet
kind: issue
title: The spline-face door places an edge's end on a spline face by its point in space, so a v-f end is not read on its own sheet
status: open
opened: 2026-10-27
priority: P2
cost: M
---

Filed by the SPLICE interior-arm lane (PR 5826), from its dual review
(r1 m4).

## What

`topo::boolean::reduce::spline_interior` places each MEETING at the
chart point it was found at (`spline_region::spline_chart_placement`),
so it is read on its own sheet
(`spline_face_rows::a_meeting_is_placed_on_the_sheet_it_was_found_on`).
With no meeting in the face it records the span's ENDS through
`contain::curved_face_placement_decided`, which reads the end in space
(`spline_region::spline_face_placement`): In where any foot within the
band lies in the trim. On a chart that passes within the band of
itself (a hairpin, or a sheet beside another), an end ON one sheet
outside the trim reads In through the other, and records a v-f contact
for a face it does not touch.

## The likely fix

The walk already isolates a meeting within the band of each end on
the chart's sheet it lies on, with its chart point
(`curve_face::spline_face::isolate`; `settle` drops it as the end's).
Hand that chart point on, and place the end by the trim there, as a
meeting is placed. An end whose meeting is not isolated is refused by
the walk already.

Moved from SPLICE to SPLICEFACE on 2026-10-28, by `git mv` in SPLICE's split along its seams (Track size; Ev, in chat, 2026-10-28).
