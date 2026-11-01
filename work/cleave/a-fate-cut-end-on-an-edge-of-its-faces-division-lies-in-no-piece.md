---
id: a-fate-cut-end-on-an-edge-of-its-faces-division-lies-in-no-piece
kind: issue
title: A fate cut's end on an edge of its face's division, at no vertex of it, lies inside no one piece of its face
status: open
opened: 2026-11-01
priority: P1
cost: M
refs: [a-fate-cut-touch-end-at-a-division-vertex-lies-in-no-piece, a-fate-cut-end-between-two-touching-lumps-lies-in-no-piece]
---


Found by the sweep of branch `cleave/fate-touch-end-at-a-division-vertex`
(the blind spot of `fate::lies`' two callers, the one its own fix
leaves): the same on main `336465c32b` and on the branch.

## What

`x` is the box `[0, 1]³` ∪ the box `[1, 2] × [0.5, 1.5] × [1, 2]`,
touching along `x = 1, z = 1, y ∈ [0.5, 1]` (an S cut short, so the
second lump's corner `(1, 0.5, 1)` lies inside the box's top edge
`x = 1, z = 1`, and the box's corner `(1, 1, 1)` inside the second
lump's bottom edge), either union order. `y` is the box
`[−0.5, 2.5] × [−0.5, 2.5]` (or `× [0.5, 1.5]`, or `[0.5, 1.5]²`) over
`[1, 1.5]` or `[0.5, 1]`, flush with a face of each lump on `z = 1`.
`y − x` (partner above) and `y ∪ x` (partner below) refuse
`ClassificationInvariant` "a fate cut's end lies inside no one piece
of its face": 16 cells per pose, at every pose of
`test_support::meeting::poses`, at ε default, 1e-6 and 1e-12.

## Cause

The refusing site is the fate cut's pierced corner (`fate::cut`, the
`KeptAt::Pierced` arm with no pierce ring, `Lies::Nowhere`). Measured
there: the point lies inside none of the kept face's pieces and at no
vertex of their loops, but on an edge of two of them (`contfp` reads
`OnEdge` in both). `lies` places a point inside one piece or at
division vertices (`piece_holding` reads the latter by direction); a
point inside an edge of the division has no reading.

`cut`'s touch placement (`End::Touch`) reads the same `lies` and refuses
the same way at such a point; no witness reaches it yet. There the
chord's end needs the division's edge split at the point, not a lone
vertex.

## Owed

Read the piece whose side of the division's edge holds the dropped
sector's direction (the pierced corner), or refuse typed; and the same
for a touching end, splitting the edge where the chord ends on it.
