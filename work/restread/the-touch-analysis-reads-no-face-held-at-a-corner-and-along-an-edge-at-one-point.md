---
id: the-touch-analysis-reads-no-face-held-at-a-corner-and-along-an-edge-at-one-point
kind: issue
title: The touch analysis reads no face held at a corner and along an edge at one point, so a solid touching itself where its face pinches refuses BoundaryCrossesItselfUndecided
status: open
priority: P2
cost: M
opened: 2026-10-21
---

Found by `tier-3-prime-reads-no-side-for-a-face-touching-a-vertex-of-its-own-solid`'s
lane (branch `restread/point-touch-winding`).

## What

`Sector::of` (`crates/topo/src/census.rs`) builds no sector for a face
whose loops hold the touch point both at a corner and inside an edge,
or inside two edges, and refuses `TouchUnread::Unanalysed`: "touches
itself there in a way no sector reads". `face_piece`'s `also` names
other CORNER passes only, so a corner's piece has no way to end at an
edge pass's incoming ray, nor an edge's piece at a corner's.

Since the self-touch reading (`sweep_self_touch_points`) reads every
point where a solid touches itself, a face pinched at such a point
refuses the whole solid `BoundaryCrossesItselfUndecided { why:
Unanalysed }`. Before it, the point was not read at all.

## Witness

`vertex_on_edge_records::a_split_at_the_crossing_records_its_vertex_on_the_other_edge`
(`crates/topo/tests/vertex_on_edge_records.rs`): the skew-crossing
cube and box united with the slab `x ∈ [0.5, 0.6]` are one solid
touching itself at `(0.5, 0, 1)`. The slab's face `x = 0.5` holds the
point at its corner (the split vertex) and inside the box's edge,
which lies in that plane: `Sector::of` reads `holds [Corner(v),
Edge(a, b)]` on that one face.

## The fix

Let `also` carry edge passes as well as corner passes. An edge pass's
edge does not block the other passes' pieces. Each pass's sector ends
at the first incoming ray of ANY pass counterclockwise from its own
outgoing ray: a corner's incoming edge, or the start of an edge
through the point. The cross-solid analysis takes the same reading.
