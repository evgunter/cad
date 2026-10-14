---
id: cylinder-pair-germ-has-no-join-arm
kind: issue
title: A wall × wall germ pair has no join arm: parallel cylinders that pierce stop at CurvedBooleanUnsupported
status: closed
opened: 2026-10-02
closed: 2026-10-04
pr: 4031
refs: [4031]
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

## Closed

The join's cylinder × cylinder arm (JOIN,
`parallel-cylinder-germ-pair-has-no-join-arm`, PR 4031)
splits both walls against the pair's radical plane, which holds both
rulings, so each side's chord is its own wall's ruling. The rows here
build at the closed form under ∪, ∩ and both differences:
`tang_circle_cylinder.rs`,
`parallel_cylinders_that_pierce_build_at_the_closed_form`.
