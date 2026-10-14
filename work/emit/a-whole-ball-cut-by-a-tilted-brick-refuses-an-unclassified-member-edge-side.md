---
id: a-whole-ball-cut-by-a-tilted-brick-refuses-an-unclassified-member-edge-side
kind: issue
title: A whole ball intersected or united with a tilted brick that cuts a cap off it refuses UNCLASSIFIED_SIDE at a union vertex
status: open
opened: 2026-10-13
priority: P2
cost: M
---


## What

Found by the sweep that closed
`a-sphere-cut-in-cap-fails-its-names-missing-upstream` (branch
`emit/naming-refusals`, 2026-10-13). Measured on origin/main's sources
too (17915e5cf6), so the defect predates that change.

Take the unit ball (a bulge-1 semicircle revolved a full turn about y)
and the brick of
`crates/editor-core/tests/emit_sphere_cut_in_cap.rs`'s `brick_toward`:
0.4 × 0.4 × 0.3, near face 0.985 from the origin toward latitude 68°,
azimuth 50°. The brick cuts a cap of height 0.015 off the ball.
`Intersect` and `Union` of the two, ball first, refuse:

`Naming(Emission("a member edge in a union vertex's name has a side no pairwise judgement classified against the members the name cites"))`

That is `emit_union::UNCLASSIFIED_SIDE`. `Subtract` (ball minus
brick) names. Unmeasured: the brick-first order, which vertex refuses
and through which member edge, and whether the kernel re-charted the
ball (`apply_recuts`) or cut it in (`apply_cut_ins`). The lens rows of
the closed row build the same cap on the lens's trimmed face and name
it up to its conventional vertex. So look first at what a whole
sphere's seam does at the circle's vertex.
