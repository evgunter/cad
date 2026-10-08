---
id: census-backstop-clears-a-curved-pair-only-along-world-axes
kind: issue
title: The census backstop clears a cross-solid pair with a curved side only by a gap along x, y or z, so CurvedWithinReach depends on the pose
status: open
opened: 2026-10-06
priority: P1
cost: M
---


Found by the operand-gate pose lane
(`boolean-operand-gate-separates-only-along-world-axes`) sweeping
`topo/src` for box readings that decide a verdict. Unmeasured: the
lane's fixtures (`crates/sweep/tests/operand_gate_pose.rs`, whose ∪
results are two-solid bodies with a cone wall or a sphere beside a
brick) pass the at-rest gate at every pose they are turned through,
so no row reds it yet.

## What

`census`'s arm 1 (the cross-solid proximity backstop, `census.rs`, the
loop deciding `census_backstop_gap`) clears a pair with a curved side
when the two `face_reach` boxes, read in WORLD axes, are definitely
apart along `x`, `y` or `z`, and otherwise records
`CensusUndecidable { what: CurvedWithinReach }`. The reach box of a
turned curved face widens by how it is turned, so the same two solids
can be cleared upright and refused turned, and an at-rest body that is
a legal operand at one pose is not at another.

## The general form

Read the pair along directions that turn with it, as the boolean's
narrow phase does (`boolean::separating::apart`, at the census's own
scalar already). Measure first: two solids of one body, a cone wall
clear of a brick by a gap smaller than the cone's world-box growth
under a turn, swept through poses.
