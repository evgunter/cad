---
id: a-planar-points-foot-on-a-segment-has-no-home
kind: issue
title: A planar point's foot on a segment has no home beside geom::Line, so ray_walk's RaySpace, step-import's chart and the viewer's picker each spell it
status: dispatched
opened: 2026-11-06
---


Found by the sweep of `onecopy/feet-and-params`, which homed a 3D point's foot on a segment as `geom::Line::param_within` / `foot_within` and ported the 3D spellings its sweep and review found onto it (the review's `step-import` `adopt.rs` `line_column_match` among them). `geom::Line` is a line in space, so the sweep's planar hits had no home to move to:

- `crates/topo/src/ray_walk.rs` `on_segment_margin`: `clamp(w·e / e·e, 0, 1)` by `max`/`min`, generic over `RaySpace` (`Point3` and `Point2`). Its 3D instance is the same reading as `Line::foot_within`. Its planar instance has no counterpart.
- `crates/step-import/src/chart.rs` `boundary_distance`: `((p − a)·ab / |ab|²).clamp(0, 1)` on `Point2<f64>`, guarded against a segment of no length.
- `crates/viewer/src/pickindex.rs` `segment_distance_px`: a screen-space `clamp(0, 1)` foot. Its neighbour `ray_segment_closest` is another class: a 3D ray-to-segment closest approach, with no home in `geom` either.
- `crates/topo/src/link.rs` `RoundCone::lowest_on`: `−w·u / u·u` held to `[0, 1]`. That is a foot taken in the plane square to the cone's axis (`across`), so it is planar in that projected space, not a 3D foot on `Line`.

Test-only planar spellings beside them, which may stay as independent oracles: `chart_region.rs` `strictly_in`'s helper, `splitting/spline_arc.rs` and `splitting/spiric_arc.rs` rows and fuzzers, `census/pierce_tilted.rs` `signed`, and `sweep/src/blend/open/end_face.rs` `to_segment` in its tests.

Before you fix it, decide the home. One option is a planar line beside `geom::Line`. The other is `RaySpace` growing the reading for both of its spaces, which would put `ray_walk`'s 3D instance onto `Line::foot_within` and its planar instance beside it. The viewer's screen space may stay its own, since it is not kernel geometry; say so if it does.
