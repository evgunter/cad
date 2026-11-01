---
id: a-wall-islands-planar-closing-reads-its-run-on-the-chart
kind: issue
title: A wall face's ring-lane island closed by a planar chord winds on the chart (chart_island_winding), so a run holding a spline chord refuses EdgeCarrierUnsupported (RingRun)
status: open
opened: 2026-10-29
priority: P2
cost: M
refs: [a-cylinder-face-re-homes-its-rings-on-a-chart-that-reads-no-spline-chord]
---

Found by `sect/rehome-by-path` (2026-10-29), in its sweep for the
cylinder's other readers of a ring-lane run on the chart.

Unmeasured: no body is known to reach it.

## What

`boolean::join::ring_run_ccw` winds a ring-lane island before its
`mef`. A wall face closed by a planar chord (`IslandClosing::Wall`)
winds on the chart, `chord_join::chart_island_winding`, which walks
the run's edges through `run_azimuth_images`: a closed-form azimuth
box per edge, so a spline edge has none. So `ring_run_ccw` refuses
any run holding a spline edge (`run_spline_edge`) under every closing
but `IslandClosing::Carried`, typed as
`BooleanError::EdgeCarrierUnsupported { site: RingRun }`. The same
guard also covers `IslandClosing::Quadric` (a sphere, cone or torus
face), whose winding, `chord_join::path_island_winding`, already reads
a spline run edge (`run_loop_arcs`, `LoopArc::Spline`) on a sphere and
a torus.

A wall that a carried chord divided earlier in the join (a ball
piercing it) and that a planar chord then closes an island on (a box
or a plane-faced operand piercing it too, in the same operation) would
reach it.

## Done when

A wall face's island closed by a plane section winds by the path, as
`IslandClosing::Carried` and `IslandClosing::Quadric` do
(`path_island_winding_closed_by` with the plane section as the closing
curve), or `chart_island_winding` reads a projected image; the
spline guard narrows to the closings that cannot read one; and a row
closes a wall island by a planar chord across a run holding a spline
chord.

