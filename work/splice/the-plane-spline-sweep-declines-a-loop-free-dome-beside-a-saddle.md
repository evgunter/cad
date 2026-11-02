---
id: the-plane-spline-sweep-declines-a-loop-free-dome-beside-a-saddle
kind: issue
title: The plane × spline regularity sweep declines a loop-free dome beside a saddle cut below the saddle
status: open
opened: 2026-10-31
priority: P3
cost: M
refs: [6150]
---


## What

The regularity sweep (`geom_brep::pair_section::plane_spline::sweep`,
`sweep`/`joined`) declines a pose whose section has no closed loop:
the review's dome beside a saddle
(`plane_spline/sweep/rows_review.rs`, `dome_and_saddle`) cut at
`z = 0.012`, below the saddle. It reads `Declined(Enclosed(..))` at the
dome's top (printed by `review_dome_and_saddle_below_the_saddle`, run
with `--run-ignored only`).

The top's cells are excluded above the plane, and the region above the
plane does reach the rectangle's `u = 1` side past the saddle. But the
join walks excluded cells only, and the shared recursion
(`ssi::exhaust::sweep_chart_regular`, the `Regular` duty) stops at a
cell it reads regular: a regular cell that is not also excluded is
never halved toward exclusion. So the chain of above-plane cells from
the top to the side has gaps, and the top is read as enclosed.

The decline is safe: the pose falls to C3's whole reading.

## What closing it takes

Refine a regular, non-excluded cell toward exclusion where it borders
an excluded cell that is not yet joined, or join through regular cells
whose height is certified on the excluded side. Either is a second
refinement pass beside the shared ladder, bought for completeness only.
Pin the dome-and-saddle cut below its saddle as `LoopFree`.
