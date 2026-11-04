---
id: a-meeting-on-the-charts-edge-nearer-a-spans-end-than-the-walks-floor-refuses
kind: issue
title: A meeting on a spline chart's domain edge just past a span's end, nearer it than the walk's floor box, refuses Floor though it is past the band of the end
status: open
opened: 2026-11-04
priority: P3
cost: M
---

Filed by `splice/teapot-1e6-floor`, the residue of its walk fix.

## What

`geom_brep::curve_face::spline_face::isolate` reads the carrier
`END_REACH · escalate / pace` past each end of the span. A box at the
floor that lies wholly past an end and stands past the band of the end's
point now settles (`Pair::past_an_end`), so a meeting on the chart's
domain edge out there is left to the span that holds it. A box at the
floor that **straddles** the end still refuses, and the floor's side,
`√(rho · escalate) / 8`, is not tied to the reach: on a long span at a
fine band it is wider than the reach. A meeting on the chart's edge
past the end, past the band of it but nearer it than a floor box, then
refuses `Unsettled::Floor` although no answer of the span's lies there.

## Measured

`spline_face/past_end_rows.rs`,
`regression_a_crossing_within_a_floor_box_of_the_end_still_refuses`
pins it at every ε row: the arc of radius 0.25 crossing the flat
patch's edge `x = 1`, its span a quarter radian long and ending
`4 · escalate` short of the crossing, refuses `Floor`. With the span
half a reach short at ε 1e-12 (rho ≈ 0.4 m: floor 2.5e-7 m against a
reach of 1.0e-8 m) it refuses the same way; at 1e-9 and 1e-6 it reads
clear. No demo row reaches it today: teapot wall 3's crossing stands
4.47 mm from its span's end, outside the reach at 1e-9 and 1e-12.

## Shape of a fix, unweighed

Cut the walk's blocks at the span's ends, so no box straddles one; an
end's meeting is then the end's box's on both sides. That changes every
end meeting's isolation, which is why it was left out of the walk fix.

