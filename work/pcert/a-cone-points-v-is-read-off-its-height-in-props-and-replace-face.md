---
id: a-cone-points-v-is-read-off-its-height-in-props-and-replace-face
kind: issue
title: props and replace_face read a cone point's v as h / cos α, which is the foot's only on the surface
status: open
opened: 2026-11-04
priority: P2
cost: M
---


Found by the sweep of `the-projected-image-inverts-chart-points-by-a-second-spelling` (D3, "One reading per question"). The chart's channels are spelled once, in `geom::ChartChannels::read` (`crates/geom/src/chart_point.rs`), the table `geom::ChartInverse::point` reads; it decides nothing, and the caller names each angle (in its window, or on a branch with `geom::Azimuth::about`).

## What

A cone point's `v` is read as `(p − apex)·axis / cos α`, its height over `cos α`, where the chart's reading is the foot on the generator, `h·cos α + σ·ρ·sin α`. The two agree on the surface only: a point `δ` off along the normal reads `δ·tan α` off (the miss PR 5659 fixed in the reading itself).

- `crates/geom-brep/src/props/curved.rs`: `cone_boundary` (generator endpoints ~3097–3098, the rim centre ~3117) and `cone_loops`'s `slant` closure (~2202).
- `crates/topo/src/replace_face.rs`: `transport_curve`'s cone parallel (~1153), and `cone_v_range` (~2316–2375), a range of the same spelling over a carrier's hull, where a NURBS control point is off the surface by construction.

## Owed

On a rim centre the reading does not apply as it stands: the centre is on the axis (`ρ = 0`), where it answers `h·cos α`, not the rim's `v`. Read the rim's level at a rim point, or state the centre's `h / cos α` as the rim's own closed form (it is exact for a circle on the cone), and read the endpoints and the range through the table. Say whether `cone_v_range`'s hull is sound off the surface.
