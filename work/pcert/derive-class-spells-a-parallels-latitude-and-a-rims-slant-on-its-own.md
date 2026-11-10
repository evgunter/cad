---
id: derive-class-spells-a-parallels-latitude-and-a-rims-slant-on-its-own
kind: issue
title: derive_class reads a sphere parallel's latitude by asin and a cone rim's slant by its own helper, beside the chart's one table
status: open
opened: 2026-11-04
priority: P2
cost: M
---


Found by the sweep of `the-projected-image-inverts-chart-points-by-a-second-spelling` (D3, "One reading per question"). The chart's channels are spelled once, in `geom::ChartChannels::read` (`crates/geom/src/chart_point.rs`), the table `geom::ChartInverse::point` reads; it decides nothing, and the caller names each angle (in its window, or on a branch with `geom::Azimuth::about`).

## What

`derive_class` (`crates/geom-brep/src/pcurve_cache.rs`) reads its circle classes' phases through `Azimuth` and `place.phase`, but two channels are spelled on their own:

- the sphere parallel's latitude as `polar = (wa / radius).asin()` (~10208), where the table reads `atan2(h, ρ)`;
- the cone rim's slant through `rim_slant` (~5921, `h·cos α + σ·ρ·sin α`, also read by `incidence` ~6127), the table's cone `v` written a second time.

## Owed

Read both through `geom::ChartChannels` (a circle point's frame-local components, then the window), or state why a class's closed form is exempt. `pcert/one-ruling-reading` is in `derive_class`'s line arms; take this after it lands.
