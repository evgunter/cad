---
id: the-projected-box-forms-spell-the-point-reading-apart
kind: issue
title: The projected image's box forms are a hand-kept Interval parallel of the chart's channel table, with nothing tying the two
status: open
opened: 2026-11-04
priority: P2
cost: M
---


Found by PR 6500's FULL review (fix list Z3).

## What

PR 6500 routed the projected image's point channels (`ProjectedImage::raw`) through `geom::ChartChannels::read`, the table `geom::ChartInverse::point` reads. The image's box forms still spell the same channels by hand as ranges, in `crates/geom-brep/src/pcurve_cache/projected.rs`:

- `ProjectedImage::window`: the azimuth by `atan2_range` plus `θ_k`, the sphere latitude by `atan2` over the corners of `(ρ, z)`, the cone's `v` as `scale(z, cos) + scale(scale(ρ, σ), sin)`, the tube by `atan2_range` plus `φ_k`;
- `frame_box_on` (the radial box rotated by `θ_k` and flipped through the nappe);
- `tube_box`;
- `rho_range`.

They are the interval extension of the table and must enclose ψ. Nothing ties the two except the review's probe (48,570 boxes, 0 outside), which is not kept. A change to one of the table's channels that is not mirrored here would leave the chart box (`Pcurve::chart_box`) and the period gate (`sweep_box`) reading a different function from the one the image evaluates.

## Owed

Either derive the box forms from the table (a range-valued reading of `ChartChannels`, with the quarter rotation through `geom::Quarter`, which `rotate_box` already takes), or keep the hand spelling and add a kept row: each box form encloses `raw` at points sampled in the box, on every chart, at both nappes and at every centre (the review's `probe_box_encloses_eval` on `review/6500-full`).
