---
id: a-planar-points-foot-on-a-segment-has-no-home
kind: issue
title: A planar point's foot on a segment has no home beside geom::Line, so ray_walk's RaySpace, step-import's chart and the viewer's picker each spell it
status: open
opened: 2026-11-06
---


Found by the sweep of `onecopy/feet-and-params`, which homed a 3D point's foot on a segment as `geom::Line::param_within` / `foot_within` and ported the 3D spellings its sweep and review found onto it (the review's `step-import` `adopt.rs` `line_column_match` among them). `geom::Line` is a line in space, so the sweep's planar hits had no home to move to:

- `crates/topo/src/ray_walk.rs` `on_segment_margin`: `clamp(w·e / e·e, 0, 1)` by `max`/`min`, generic over `RaySpace` (`Point3` and `Point2`). Its 3D instance is the same reading as `Line::foot_within`. Its planar instance has no counterpart.
- `crates/step-import/src/chart.rs` `boundary_distance`: `((p − a)·ab / |ab|²).clamp(0, 1)` on `Point2<f64>`, guarded against a segment of no length.
- `crates/viewer/src/pickindex.rs` `segment_distance_px`: a screen-space `clamp(0, 1)` foot. Its neighbour `ray_segment_closest` is another class: a 3D ray-to-segment closest approach, with no home in `geom` either.
- `crates/topo/src/link.rs` `RoundCone::lowest_on`: `−w·u / u·u` held to `[0, 1]`. That is a foot taken in the plane square to the cone's axis (`across`), so it is planar in that projected space, not a 3D foot on `Line`.

Test-only planar spellings beside them, which may stay as independent oracles: `chart_region.rs` `strictly_in`'s helper, `splitting/spline_arc.rs` and `splitting/spiric_arc.rs` rows and fuzzers, `census/pierce_tilted.rs` `signed`, and `sweep/src/blend/open/end_face.rs` `to_segment` in its tests.

Before you fix it, decide the home. One option is a planar line beside `geom::Line`. The other is `RaySpace` growing the reading for both of its spaces, which would put `ray_walk`'s 3D instance onto `Line::foot_within` and its planar instance beside it. The viewer's screen space may stay its own, since it is not kernel geometry; say so if it does.

## Closed (`onecopy/planar-foot`)

**Home: `geom_core::Line2`**, the planar line carrier beside `Arc2` (the orchestrator's ruling, option a). Its semantics follow `geom::Line` under ruling A′:
- the parameter readings (`param`, `param_within`, `foot_within`) divide by `|dir|²`;
- the metric readings (`height`, `side`, `distance`) divide by `|dir|`.

Ported:
- `topo` `ray_walk`: `RaySpace` gains `segment_foot`, whose two impls read `geom::Line::foot_within` and `Line2::foot_within`. `on_segment_margin` reads it, and the trait's `offset`, `scale` and `norm_squared` go.
- `step-import` `chart.rs` `boundary_distance`: `Line2::foot_within`, guarding an edge of no length.
- `profile` (the orchestrator's ruling: no second planar home):
  - `seg.rs` reads through `Seg::chord_line()`: `chord_side`, `line_span`, the line×arc foot and crossings, `nearer_end`, and the ray's side and its arc test. `carrier_line_circle_margin` takes a `Line2`.
  - Also ported: `path.rs`'s pending run-out `across` and the via arc's chord offset, `path/family.rs`'s chord offset, `path/arc_fillet.rs`'s ray travel, and `sugar.rs`'s straight setbacks.

Stays, with reasons:
- `profile` `seg.rs`: the line×arc pair's and the ray×arc test's signed side `h`, read on the stored unit direction with no division. `h` feeds a cancelling `√(r² − h²)`, so a last-place move moves a crossing by about `√(2r·δh)`. Through `Line2::side`, `tangent_arc_near_a_full_turn::the_rows_three_radii_keep_their_end_vertex` turned R 1.36e4's validated loop into a touch. The reason is at both sites.
- `viewer` `pickindex.rs` `segment_distance_px`: screen pixels over `[f64; 2]`, not kernel geometry.
- `topo` `link.rs` `RoundCone::lowest_on`: a foot in the plane square to the cone's axis, over 3D vectors, not `Point2`.
- The test-only oracles listed above.

Filed:
- `ray_segment_closest`: `a-ray-segment-closest-approach-has-no-home-in-geom`.
- The 3D inline heights the sweep found: `a-points-height-along-a-unit-line-is-spelled-inline`.
