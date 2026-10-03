---
id: a-subtract-through-a-pinch-line-drops-the-pinch-row-at-its-cut
kind: issue
title: A subtract that cuts one side of a pinch line records no contact where the cut leaves the pinch: neither end of either v-v row there resolves live
status: open
opened: 2026-10-02
priority: P1
cost: M
---



## What

Pinch = `union_with(q1, q3)`, `q1 = brick((0,1),(0,1),(0.5,1.5))` and
`q3 = brick((-1,0),(-1,0),(0.5,1.5))`, touching along the z-axis. Its
two v-v rows are carried in as `carried_a` (`Rest`). B is `prism_z`
over the triangle (-1,1), (1,-1), (1,1), z ∈ (0.75, 1.25): its diagonal
wall holds the axis, and it holds `q1`'s side.

`pinch − B` builds at 1.5. It cuts `q1` in two and leaves `q3` whole, so
the pinch survives over z ∈ (0.5, 0.75) with a new pair of vertices at
(0,0,0.75). 3′ refuses the result: `UndeclaredContact` `VertexVertex`
at (0,0,0.75), and `EdgeEdgeOverlap` at (0,0,0.625). `pinch ∪ B`,
`pinch ∩ B` and `B ∪ pinch` pass 3′. On main every one of the four
refused `CorruptOperand` (`work/fuse/a-flush-partner-folded-onto-an-edge-contact-refuses-corrupt-operand.md`).

## Cause, as far as traced

The reduction's v-v rows at (0,0,0.75) are the two pinch vertices
(A 22v1 and 26v1 in the run measured), each paired with B's vertex 10v1.
`remap_contacts` (`crates/topo/src/boolean/ops.rs`) groups rows that
share an end and records every two distinct live ends of a group. Here
`Descendants::live_vertex` resolves neither A 22v1 (`q1`'s split
vertex, which the cut consumes) nor B 10v1 (B's own vertex, which a
subtract does not keep). It does resolve A 26v1 (`q3`'s split vertex).
So the group has one live end, and the result's vertex at that
point, which closes `q1`'s lower piece, is in no row. Which entity
the result's vertex descends from was not traced. That lineage is
what the remap needs.

## Owed

Trace the cut corner's lineage under subtract and give the group its
second live end. Pin it with the case above added to
`a_face_through_a_pinch_line_builds_in_every_op` (subtract, `diag2`
span z ∈ (0.75, 1.25)).

## Fixed on `fuse/shared-vertex-crossings` (PR 3927), pending merge

The kept corner at (0,0,0.75) is a null-edge copy of the cut brick's
split vertex, and no row named it. `remap_contacts` now takes each v-v
group's null-edge copies in with its ends, and the subtract passes 3′.
The witness is pinned as `pinch ∖ upper wall` in
`a_face_through_a_pinch_line_builds_in_every_op`, and is red with the
copies left out (measured). Close with that PR's merge.
