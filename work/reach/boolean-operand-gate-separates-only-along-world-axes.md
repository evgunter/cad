---
id: boolean-operand-gate-separates-only-along-world-axes
kind: issue
title: The boolean's operand gate clears an unarmed face only by a world-axis box overlap, so whether a pair refuses depends on how the operands are turned
status: open
opened: 2026-10-03
priority: P1
cost: M
---


Found by the split-gate pose lane (`split-gate-reads-a-world-axis-box`),
sweeping for reach tests read in world axes. Unmeasured: the mechanism
is read off the code, and no fixture has been built for it.

## What

`boolean::reduce::first_unsupported_pair` refuses an unarmed face
(`CurvedBooleanUnsupported`) when its padded `boxes::face_box` overlaps
any face box of the other operand (`boxed.overlaps(other_box)`). Both
boxes are axis-aligned in WORLD coordinates, and an overlap test of two
such boxes separates them only along `x̂`, `ŷ` and `ẑ`. Turn the same
pair of operands together and the boxes of a tilted face grow by how it
is turned (a sphere zone's slab spends its radius on every axis it is
not aligned with, `boxes::slab_extent`), so a pair that clears upright
may refuse turned. That is the class the split gate had, in its
pairwise form.

A second site of the same shape, on the same ground:
`boolean::ops`'s sphere extent scan boxes a sphere's section circle as
`foot ± ρ` on every world axis (`circle_box`) and the ball as
`center ± r` (`ball_box`), and clears each against boundary-edge boxes
by world-axis overlap. A circle in a plane tilted off every axis is
boxed as a cube of side `2ρ` there, where its own extent along axis `i`
is `ρ·√(1 − nᵢ²)`.

## The general form

The split gate now reads each face's reach in the frame of the one
direction it is tested along (`boxes::BoxFrame`,
`census::face_reach_in`). A pair has no one direction, but a separating
direction can be proposed and both faces read along it: the axis
between the two boxes' centres, a face normal of a planar partner, or
the other operand's own axes. Two convex sets that are disjoint have a
separating direction; a candidate set that turns with the operands
makes the verdict a fact about the pair, not about the pose. What it
costs is a reach per candidate per face, which the box overlap's
broad phase can keep cheap by testing only the pairs it does not clear.

Measure first: build an unarmed face (a torus rounding, a spline wall)
beside a planar operand, clear of it, and sweep the pair's pose.
