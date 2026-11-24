---
id: apex-reaches-read-a-lever-as-a-window-along-the-axis
kind: issue
title: Coaxial and apex rows read a lever as a window along an axis
status: open
opened: 2026-11-23
priority: P1
cost: M
---

Three rows of `geom_brep::intersect` read a lever's length as a window along an axis, where a lever read back as a length is a margin's business (`geom_core::Lever`'s doc):

- `cone_cylinder_section`, `coc_station_reach`: `Margin::of(below.length() - station.abs())` compares the minted circles' station `R·cot α` with the lever's length, as if the lever were the consumed region's reach along the cone's axis.
- `cone_cylinder_section`, `coc_coaxial`: `separation_range(cone_axis, cylinder_axis, above.length())` reads the axes' separation over `|s| ≤` the lever's length.
- The coaxial-gap reader near line 1934: `separation_range(at_foot, other, lever.length())`.

The function takes `impl Into<LeverBounds<T>>`, which holds no region, so the honest read (the consumed region's window along the axis, `Reach::range_along` from the apex, from below where a definite "inside" is claimed) needs it to take the `&Reach` instead. The dispatcher (`section_route`) has one, and `topo::offset_derive::coaxial_with_cone` would hand its seed edge's region (`Reach::of_span`). The `sizes-are-read-back-out-of-extent-levers` unit left the station check for that reason, and because PR 7442 edits this file's apex reaches.
