---
id: two-homes-cut-a-chart-path-at-the-walls-knot-lines
kind: issue
title: Two homes cut a chart path where it crosses the wall's knot lines: the trimmed lane's chord cuts and the curved General image's breaks
status: open
opened: 2026-11-10
priority: P3
cost: M
---


Filed by `pcert/curved-general-image` (PR 6532, review S7).

## What

Two places find where a path in a spline chart crosses the wall's
interior knot lines and cut the path there.

- **The trimmed area lane.** `geom_brep::props::quad::trimmed`, `Chord::cuts`
  and `CrossingCut`, near :780. It cuts a straight chart chord
  `ℓ(u)` at the chart's `u`-knots and at certified brackets of its
  `v`-knot crossings, in `Interval`, so that each piece is one polynomial.
- **The curved General image.** `geom_brep::edge_nurbs`, `knot_lines`
  and `crossings`, for `general_chart_image`. It breaks a bending
  carrier's foot path where it crosses a knot line whose continuity is
  below the image's degree. The crossing is found by halving between
  projected feet in `f64` (a structure choice), so that each piece of the
  image follows a smooth stretch of the path.

They answer one question, where a chart path leaves a knot span, on two
different objects: a linear chord, read certified, and a projected foot
path, read in `f64` from projections. A fold would need one reading that
serves both: certified brackets for the chord, and a structure-only
placement for the foot path. PR 6532 did not fold them.

## Fix direction

Name one home for "the knot lines a chart path crosses, and where", with
its two readings: certified for a chord, structure for a projected path.
Or record in each why the other is not it.
