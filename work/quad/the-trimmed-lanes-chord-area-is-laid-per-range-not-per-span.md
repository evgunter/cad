---
id: the-trimmed-lanes-chord-area-is-laid-per-range-not-per-span
kind: issue
title: the trimmed lane's chord-area pass is laid per chord range from the chart's lowest v, so a tiled chart is not read on the whole chart's cells
status: open
opened: 2026-10-30
---


(QUAD per-span lane, PR 6092.)

## What

`chord_polygon_area` (`crates/geom-brep/src/props/quad/trimmed.rs`) integrates the area under each sub-chord as a column from the chart's lowest `v` (`kvb[0]`) up to the chord, in `chord_area_cuts` cells: the range's own `CHORD_AREA_CELLS = 64`-piece grid plus the chart knots inside it. Both the column's base and its grid are properties of the whole chart and the chord's range, not of the knot spans the cells lie in, so a trimmed chart split into per-span tiles reads different cells from the whole (a tile's columns start at its own lowest `v`). Nothing in this pass is priced in the lane's wall (`OuterRule::round_cost` counts images and iso chords only).

## Measured

PR 6092 first made this pass per span, through the per-span `knot_aligned_cuts`: 64 cells per chart span in `u` under each chord, and 64 per `v` span in every column, about 576² cells under a chord of a 9 × 9 chart against about 64 × 73 on main. On `q16_the_composite_arm_holds_an_independent_reference_across_knots` (release, this box) that took the row from 30.6 s to 69.7 s. The PR put the pass back on a per-range grid (`chord_area_cuts`, main's cut rule less the hull-block edges this pass never read), and the row is at 30.5 s again.

## Owed

A per-span-consistent chord-area rule: the column base taken per `v` span (each span's own Green primitive), the cells per span, and the pass's cost in the lane's round cost or its wall. That is a change to the Green decomposition, not a constant.
