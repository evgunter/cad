---
id: a-ray-segment-closest-approach-has-no-home-in-geom
kind: issue
title: viewer's ray_segment_closest is a 3D line-line closest approach with no home in geom
status: open
opened: 2026-11-06
---


Split out of `a-planar-points-foot-on-a-segment-has-no-home` by `onecopy/planar-foot`, on the orchestrator's ruling.

`crates/viewer/src/pickindex.rs` `ray_segment_closest` finds the segment parameter nearest a pick ray. It is the closed form of two lines' closest approach (`(ab·a·o − aa·s·o) / (aa·bb − ab²)`, held to `[0, 1]`, with a relative parallel guard), over `f64`. `geom::Line` reads a point against a line. It has no reading of a line against a line, so this has no kernel home to move to.

Before you fix it, decide whether a line–line closest approach belongs on `geom::Line`, for example `Line::closest_to(&Line) -> (T, T)`. Grep the kernel for the same closed form first: line–line closest runs are spelled in `topo` (`census.rs`'s collinear run near `:5866`, `link.rs`'s `segment_cross_point`), so the class may be wider than the viewer. Or decide that the viewer's screen-picking reading stays its own, as `segment_distance_px` does, and say why.
