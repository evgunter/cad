---
id: chart-region-ray-exhausted-tells-the-point-to-leave-a-boundary-it-is-off
kind: issue
title: chart-region's RayExhausted tells the point to move off a boundary its pre-pass already placed it off
status: open
opened: 2026-10-06
priority: P4
cost: E
---

## What

`ChartRegionError::RayExhausted`'s own `Display` (`chart_region.rs`,
`impl Display for ChartRegionError`, ~`:419`) says "move the point off
the boundary it grazes". But the containment walk that raises it
(`chart_region.rs`, the polygon walk ending `Err(ChartRegionError::RayExhausted)`,
~`:2905-2924`) first runs `ray_parity::on_boundary` and answers
`OnBoundary` there, so a point that reaches the ray schedule is already
off the boundary. The grazing is the rays' doing: they meet a vertex
or an edge of the trim polygon, and the point does not touch it.
The recourse rests on a premise that is false.

This is the same class as PR 4055 (`ContainError::RayExhausted`) and
PR 4076 (tier 3's arms, the census, and `PointInSolidError`'s
`Display`). Tier 3's own reading of this arm (`validate.rs`
`classify_chart_region`) was corrected in 4076. This row covers the
lower `Display`, which is CHART ground.

## Done when

The text says what happened (the point is off the boundary, and every
scheduled ray grazed a vertex or edge of it) and drops "the boundary it
grazes". Whatever pins it is updated to match.

