---
id: a-rulings-end-vertex-read-through-its-rims-image-is-off-the-rulings-exact-line
kind: issue
title: a ruling's end vertex read through its rim's image develops off the ruling's exact line
status: open
opened: 2026-11-17
priority: P3
cost: E
---


Found building `a-run-of-ruling-points-the-development-merges-is-re-split-after-the-triangulation`.

## Finding

`trimmed`'s module docs ("The development") and `trimmed::developed`
state that every point of one ruling develops onto one exact line through
the apex, because every point of one chart column (one `u`, bit for bit)
is an exact multiple of one direction. A ruling's end vertex is not
always in the ruling's column. The trim polygon takes each traversal's
points but its last, so the vertex where a ruling ends and a rim begins
is read through the RIM's image, at the rim's start parameter.

Measured at the default ε, δ 0.01: the wall `(1, 0), (2 + tan 1°, 0),
(2, 1), (1, 1)` revolved a half turn about `y`, placed obliquely, its
cone face's polygon:

    pt 48  u = -1.2246467991473532e-16   edge 13v1 (the ruling)
    pt 0   u = -1.2246467991473532e-16   edge 11v1 (the ruling)
    pt 1   u = 0.0                       edge 7v1  (the rim; the ruling's far end)

The ruling's two points develop at direction `(sin(−1.2·10⁻¹⁶·sin α), 1)`
and its far end at `(0, 1)`: two lines through the apex, `10⁻¹⁸` apart
in direction. Spade's exact predicate does not read the end vertex and
two points of its ruling as collinear, so the rule that no triangle lies
along a ruling is not held by exactness for that triple.

## What it has and has not cost

Nothing measured. A triangle of a ruling's end and two of its points
would be an ear across a boundary point, with no area, and would refuse
`UnresolvedWinding`; no cell of the unit's grids refuses that way on a
cone face at the default ε. The unit reads which neighbour of a run is
along its ruling from the polygon side's carrier (a line), not from the
column, because of this: read by column, 153 of its 1296 default-ε cells
kept refusing.

## The cure seen

Read a vertex between a ruling and a rim through the ruling's image, or
give the polygon's points on one line edge that edge's one `u`.
