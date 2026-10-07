---
id: sphere-window-re-reads-the-loop-on-every-reach
kind: issue
title: The sphere window re-walks the loop, the material sign and the chart trim on every reach read, with no memo
status: open
opened: 2026-10-06
priority: P3
cost: M
---


Found by the REACH review of PR 4123 (NOTE 4, by inspection; cost not
measured).

`boxes.rs` `sphere_reach` calls `sphere_window` every time it runs.
`sphere_window` flattens the outer loop (`props::loop_edges`), reads
the material sign (`geom_brep::props::boundary_material_sign`) and
runs `solid_contain.rs` `sphere_chart_trim`: the class walk, the
`chord_join::face_azimuth_window` loop walk, `wrap_rims` and
`latitude_extremes`. None of this depends on the frame. Yet
`census.rs` `face_reach_in` reads it once per face per aimed frame,
and the split gate (`splitting/classify.rs`, through `face_reach_in`)
reads it once per face per cut. A sweep of many cuts or many aims over
one body therefore repeats an identical, frame-free window read.

**The unit:** measure first. A split or separation run over a sphere
body with many aims, timed with and without one window read per face,
says whether a memo pays. If it does, hoist the window (`SphereWindow`)
out of the frame loop. Either key it per face for one body snapshot,
or hand the frame-free window to `sphere_reach` from a caller that
reads it once. Make no cache that can outlive the body it was read
from.
