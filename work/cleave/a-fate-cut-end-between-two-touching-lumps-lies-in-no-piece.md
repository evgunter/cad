---
id: a-fate-cut-end-between-two-touching-lumps-lies-in-no-piece
kind: issue
title: A flush partner over two lumps touching along an edge, the lumps second, refuses ClassificationInvariant in ∪ and ∖: a fate cut's end lies inside no one piece of its face
status: open
opened: 2026-10-25
priority: P1
cost: M
---


Found by PR 5545's review probe (`cleave/fate-cut-ends-at-a-slit-tip`);
the same on main `42af7c0b48` and on the branch, at ε default, 1e-6
and 1e-12, at rest, turned, and flipped and turned.

## What

`x` is the box `[0,1]³` ∪ a second lump touching it along the edge
`x = 1, z = 1`: the box `[1,2] × [0,1] × [1,2]` (an S, opposite faces
across the line) or the wedge over the `(x, z)` triangle
`(1,1), (2,1), (2,2)` (lump 2 above, tops coplanar). `y` is the box
`[−0.5, 2.5] × [−0.5, 1.5] × [1, 1.5]` or `× [0.5, 1]`, flush with a
face of each lump on the line's plane. `y − x` and `y ∪ x` refuse
`ClassificationInvariant` "a fate cut's end lies inside no one piece of
its face" (`boolean/fate.rs` `fragment_at`), 8 cells per pose; `x ∪ y`
refuses `PinchRaysInAFace`, and `x − y`, `x ∩ y`, `y ∩ x` build.

## Owed

Place the cut's end where the lumps' touching vertex lies on the
dropped face's division, or refuse typed.
