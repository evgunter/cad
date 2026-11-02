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

The regularity sweep (`geom_brep::pair_section::spline_implicit::sweep`,
`sweep`/`joined`) declines a pose whose section has no closed loop:
the review's dome beside a saddle
(`spline_implicit/sweep/rows_review.rs`, `dome_and_saddle`) cut at
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

## A curved partner meets it too (2026-11-02, SECTARMS)

The NURBS × implicit arm's stage 1 runs the same sweep against a
cylinder, sphere or torus (`spline_implicit::sweep`, `Phi::Curved`), and
declines a loop-free pose the same way: a planar wall across a ring
torus (`R = 1`, `r = 0.2`) whose rectangle holds the tube's core point
`(1, 0, 0)`, the tube cut by the wall's lower side, reads
`Declined(Enclosed(..))` at the core. Its cells are excluded inside the
tube, the inside does reach the lower side, and the regular cells round
the trace are never halved toward exclusion, so the join finds no chain.
A curved partner's declined pose has no whole reading yet
(`SectionError::RoutesToGeneralRung`), so there the decline costs the
pose its verdict rather than a march. The row
`spline_implicit/curved/rows.rs::a_leaf_through_a_stem_is_loop_free`
stands its wall clear of the core for that reason; with the wall's lower
side at `z = −0.1` it is this row's witness.
