---
id: a-reflex-corner-of-a-face-with-no-implicit-carrier-has-no-reading
kind: issue
title: A cell at a reflex corner of a torus, spindle or spline face is read by no separator, since only the face's own carrier reads a reflex corner and implicit_of builds none for those kinds
status: open
opened: 2026-11-22
priority: P2
cost: H
---


## Finding

Filed by EMBED's lane `embed/iso-end` (2026-11-22), where it was the
option weighed and not built.

At a reflex corner `V` of a face `F`, the face's part of a cell holding
`V` is the cell less a quadrant, or the whole cell with `V` on a side.
No first-order separator that holds the shared edge's carrier signs it:

- `planes_through`, `graph_of` and the partner's carrier vanish on the
  shared edge's chart line past `V`, which is `F`'s interior, and `F`'s
  part lies on both sides of that line;
- at a vertex alone, a separator through `V` that is transverse to `F`
  cuts `F` along a curve through `V`, one of whose tangent rays enters
  `F`'s 270°.

The corner is read only by `F`'s own carrier, in the departure arm's
carrier separator (`departure`, `implicit_of`), which reads the
partner's presentation and never `F`'s. `implicit_of`
(`crates/topo/src/injective.rs`) builds a carrier for a plane, a
cylinder, a sphere and a cone only. **A torus, a spindle, a NURBS face
(and an `Approx` one) at a reflex corner has no reading at any depth**:
the pair splits to `Budget(Depth)` and refuses `Uncertified { Departure }`.

The argument's one premise is that a separator holds the shared edge's
carrier and departs at first order. A second-order separator (one
whose zero set on `F` ends at `V`, departing quadratically) or one that
holds no carrier would escape it. The engine builds neither.

## Population

None of the 83 tests of `a-cell-past-an-iso-edges-end-is-clipped-by-its-whole-line`
is one (their reflex faces are cylinders and planes). Not searched for on the census's torus and spline buckets: the
torus buckets traced so far exit at `vacuous`
(`vacuous-reads-a-cell-at-a-corner-local-at-every-depth`), not here.
P2 until a body is found that reaches it.

## What a reading would take

Cut the cell once at `V`'s coordinate line rather than bisect it, a
split rule in `clearance` (`split_by`). Each child pair is then
certified alone. The child beside the shared edge has a side that is
edge for part of its length and the face's interior past the edge's
end, read against a partner whose zero set is the shared cell, by the
face's injectivity by kind, as `Zero::Past` is read
(`partial_zero`, `past_read`). The other child meets the shared cell at
`V` alone, and the vertex plane (`planes_through`'s vertex arm) reads
it. Each child still needs its partner presented.
