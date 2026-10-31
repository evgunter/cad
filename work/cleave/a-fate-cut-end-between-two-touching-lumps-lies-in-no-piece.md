---
id: a-fate-cut-end-between-two-touching-lumps-lies-in-no-piece
kind: issue
title: A flush partner over two lumps touching along an edge, the lumps second, refuses ClassificationInvariant in ∪ and ∖: a fate cut's end lies inside no one piece of its face
status: closed
closed: 2026-10-27
pr: 5812
opened: 2026-10-25
priority: P1
cost: M
branch: cleave/flush-partners-at-a-slit-build
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

## Cause

The refusing site is the fate cut's pierced corner, not an edge's end:
a dropped sector of a lump's face, held wholly by the partner's flush
face at the lumps' shared point (`fate::KeptAt::Pierced`), where the
partner's face is divided into pieces and no pierce ring serves the
point (`boolean/fate.rs:1737`). `fragment_at` (`:895`) reads the piece
holding the point strictly inside it; the point is a vertex of the
face's division, the corner of the second lump's imprint, so it lies
inside none.

## Closed

2026-10-27, PR 5812. Where the point lies inside no piece but at
vertices of their loops (`fate::lies`, each vertex once), the piece is
the one whose corner there holds the dropped sector's direction
(`fate::piece_holding`, `place`, the ring arm's helper too). `y − x` and `y ∪ x` build for the S and the wedge
above, both partners, as the closed form says, tier 3 and 3′, one edge
along the line, at ε default, 1e-6 and 1e-12, at rest, turned and
flipped and turned. Pinned by
`a_keel_on_the_floor_subtracts_as_a_slit::a_flush_partner_over_two_lumps_touching_along_an_edge_builds`.
`x ∪ y` still refuses `PinchRaysInAFace`, a typed construction gap,
filed with its siblings as
`two-lumps-touching-along-an-edge-refuse-pinch-rays-against-a-flush-partner`
(P1: an operand order that refuses where the other builds).
