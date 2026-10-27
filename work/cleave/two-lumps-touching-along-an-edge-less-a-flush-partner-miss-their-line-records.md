---
id: two-lumps-touching-along-an-edge-less-a-flush-partner-miss-their-line-records
kind: issue
title: Two lumps touching along an edge, less a flush partner over part of the edge, build without the records of their touch: tier 3' refuses UnrecordedContact
status: open
opened: 2026-10-26
priority: P2
cost: M
refs: [a-union-beside-a-slit-misses-an-edge-overlap-record, boolean-door-runs-the-census-over-its-result]
---

Found by the flush-partners-at-a-slit lane (`cleave/flush-partners-at-a-slit-build`),
the same on main `be2b7a139c`, at ε default, 1e-6 and 1e-12, at rest,
turned, and flipped and turned.

## What

`x` is the box `[0, 1]³` ∪ a second lump touching it along the edge
`x = 1, z = 1`: the box `[1, 2] × [0, 1] × [1, 2]` (an S) or the wedge
over the `(x, z)` triangle `(1, 1), (2, 1), (2, 2)`. `y` is the box
`[0.3, 1.7] × [0.2, 0.8] × [1, 1.5]` or `× [0.8, 1]`, flush with a face
of one lump and covering the middle of the line. `x − y` builds right
through tier 3 and the closed-form volume (1.79 and 1.916 for the S,
1.365 and 1.416 for the wedge), its two lumps still touching along the
line's two stretches either side of `y`, but its contact records hold
no row for them: tier 3′ refuses `UnrecordedContact` `VertexOnEdge` and
`EdgeEdgeOverlap` there.

The Boolean door certifies tier 3; its census over the result is
parked on REACHHOLD (`work/reachhold/boolean-door-runs-the-census-over-its-result.md`).

## Owed

Carry the lumps' touch along the line through the op as records, or
read them from the result.

