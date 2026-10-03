---
id: cylinder-pair-germ-has-no-join-arm
kind: issue
title: A wall × wall germ pair has no join arm: parallel cylinders that pierce stop at CurvedBooleanUnsupported
status: open
opened: 2026-10-02
---


## What

Two parallel unit cylinders, `z ∈ [0, 2]` and `z ∈ [0.5, 2.5]`, axes
`d` apart (`d ∈ {0.3, 0.8, 1.2, 1.6, 1.9}`): each rim circle pierces the
other wall, the circle × cylinder root lane certifies where, the
pierce's sector side certifies, and every op refuses
`CurvedBooleanUnsupported { kind: Cylinder }` at the join's germ-pair
dispatch (`crates/topo/src/boolean/join.rs`, the fixpoint's `GermLane`
match: no arm for a cylinder × cylinder germ pair). The section between
the two walls is a pair of rulings (parallel axes), so the chord lane
the pair needs is the straight one; what is missing is the lane's
section datum for a wall × wall pair (each side's chords lie on the
OTHER wall, not in a plane), and the ring lane's closing chord on a
wall island (`chord_join::chart_island_winding` takes a plane).

Until 2026-10-02 the same rows refused earlier, at the pierce ring
(`work/tang/pierce-ring-has-no-join-arm.md`); that unit's join arm
moved them here. Pinned by
`crates/sweep/tests/tang_circle_cylinder.rs`,
`parallel_cylinders_that_pierce_reach_the_cylinder_pair_join`, which
goes red when the arm lands and should then become builds at the
closed form (the lens of two unit discs `d` apart, times the 1.5 m of
shared height, added to and subtracted from the two cylinders).
