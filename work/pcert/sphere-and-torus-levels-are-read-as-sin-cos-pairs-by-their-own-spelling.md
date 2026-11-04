---
id: sphere-and-torus-levels-are-read-as-sin-cos-pairs-by-their-own-spelling
kind: issue
title: props, replace_face and the sphere box read a latitude or tube angle as their own (sin, cos) pair or atan2
status: open
opened: 2026-11-04
priority: P3
cost: M
---


Found by the sweep of `the-projected-image-inverts-chart-points-by-a-second-spelling` (D3, "One reading per question"). The chart's channels are spelled once, in `geom::ChartChannels::read` (`crates/geom/src/chart_point.rs`), the table `geom::ChartInverse::point` reads; it decides nothing, and the caller names each angle (in its window, or on a branch with `geom::Azimuth::about`).

## What

A sphere's latitude or a torus's tube angle is read off a point by its own spelling, often as a `(sin, cos)` pair rather than an angle:

- `crates/geom-brep/src/props/curved.rs`: `sphere_boundary`'s `latitude` closure (~4852) and rim pair (~4906–4907), `(h/R, ρ/R)`; `torus_classify` (~5615), `torus_ends` (~4994) and `torus_chart` → `torus_angle` (~5139, branch by `props_torus_lift_branch`); `torus_chart_loop`'s `tube` / `tube_arms` / `about_core` (~5381–5406).
- `crates/topo/src/replace_face.rs`: `transport_curve`'s torus parallel `(cos v, sin v) = ((r − R)/r, h/r)` (~1213).
- `crates/topo/src/boolean/boxes.rs`: `sphere_window`'s `latitude` (~1611), `atan2(h, ρ)` of the `(h, ρ)` pairs `solid_contain.rs`'s `sphere_chart_trim` stores (~1332).

## Owed

Decide whether the pair is a channel of its own (props keeps it for precision near the poles and compares levels by chord), in which case `ChartChannels` should hand the direction out and these read it; otherwise read the angle through the table.
