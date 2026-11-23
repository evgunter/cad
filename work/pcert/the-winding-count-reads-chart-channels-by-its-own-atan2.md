---
id: the-winding-count-reads-chart-channels-by-its-own-atan2
kind: issue
title: pair_section's winding count reads the cylinder and torus channels by its own atan2
status: open
opened: 2026-11-04
priority: P3
cost: C
---


Found by the sweep of `the-projected-image-inverts-chart-points-by-a-second-spelling` (D3, "One reading per question"). The chart's channels are spelled once, in `geom::ChartChannels::read` (`crates/geom/src/chart_point.rs`), the table `geom::ChartInverse::point` reads; it decides nothing, and the caller names each angle (in its window, or on a branch with `geom::Azimuth::about`).

## What

`crates/geom-brep/src/pair_section/winds.rs`: `Channel::coords` and `Channel::angle` (~186–204; frames built in `channels` ~236) read a cylinder's or torus's azimuth as `atan2(w·(axis × u_ref), w·u_ref)` and a torus's tube angle as `atan2(h, ρ − R)` at `f64`, to count windings from end-to-end differences.

## Owed

Read them through `geom::ChartChannels::read`: no band, no decision, and only differences are used, so any branch serves (`Azimuth::about` with a fixed centre).
