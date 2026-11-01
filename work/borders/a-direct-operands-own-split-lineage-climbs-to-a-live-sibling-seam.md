---
id: a-direct-operands-own-split-lineage-climbs-to-a-live-sibling-seam
kind: issue
title: Borders reads a Direct operand's own split lineage up to a live sibling edge and credits a piece with the sibling's discard seam
status: open
opened: 2026-10-26
priority: P2
cost: M
refs: [an-intersect-of-an-intersect-names-a-three-way-coincident-face-apart-in-one-member-order, borders-records-a-join-twice-and-reads-lineage-three-ways]
---


## What

`editor-core` `emit_union_borders::the_silhouette_letters_name_through_an_intersect_of_an_intersect`
(H on xy +z 3, T on yz +x 2, C on zx +y 3; `two` = H ∩ T, `three` =
C ∩ `two`). In the member order `T ∩ H` then `two ∩ C`, the piece of
`three`'s y 3 face over x 1.5..2, z 2.5..3 is named
`("y=3.00", ["x=1.50", "z=2.50"])`. The other three orders name it
`("y=3.00", ["x=1.50"])`, at ε 1e-9, 1e-6 and 1e-12 alike (measured
on `d82dc04c`). The pin asserts only the three agreeing orders.

**The three agreeing orders are right.** The y 3 parent of `three` is
`Merged` {C's cap, `two`'s y 3 face}. Neither holds region across that
piece's z 2.5 edge: below it, at x 1.5..2, z 0.5..2.5, is outside C's
cap, and `two`'s y 3 face is only the two pieces [0,0.5] and [1.5,2] ×
[2.5,3]. H's leg end below z 2.5 was lost in `two`'s boolean, not in
`three`'s. So nothing `three` discarded borders that edge, and z 2.5
is not a divider wall of the piece.

## Cause (measured)

The topo rows agree across the orders: `three`'s discard rows in
`H ∩ T, two ∩ C` and `T ∩ H, two ∩ C` are the same two C fragments
(the L below z 2.5 and the middle [0.5,1.5] × [2.5,3]), and `two`'s
rows are identical wherever its member order is. No containment
fallback runs on this path, so `BooleanNaming::uncut` is empty.

What differs is the names layer's chase, in `borders.rs`
`Obstacles::walls`, `reached` → `Obstacles::up_lineage`. In the odd
order, `three`'s operand A is `two` = T ∩ H, held `Direct`, so its
edges keep `two`'s own body provenance. T's one z 2.5 edge at y 3 was
split in `two`. The piece over x 0..0.5 kept the key (`12v1` in the
dump), and the piece over x 1.5..2 (`28v1`) carries
`Provenance::SplitEdge { edge: 12v1 }`. In `three`, `12v1` is a seam
of C's L-shaped discard (its `bordered_through` stretch along
x 0..0.5, z 2.5). `reached(28v1)` walks up to `12v1`, finds it in
`seam_of`, and credits piece B with that discard. That discard
borders piece A too, so it divides, and B gains the z 2.5 wall.

In the order `H ∩ T` the z 2.5 edges are section edges `two` minted
per H leg (`Mef` provenance, no common ancestor), so the walk stops.
In `C ∩ two`, `two` is the grafted B: the walk from that edge reaches
no discard (measured), so the edge is not credited.

So `up_lineage` treats "descends from an edge that is now a seam" as
"lies along that seam". That holds when the seam was recorded before
the split (a later fold step splitting an earlier step's seam). It is
false when the split is older than the seam: the ancestor key is then
a live sibling, and the seam names only the sibling's span. The walk
reads `body.edge_provenance_of` on the result, which carries a Direct
operand's own history. That is the first of the three lineage sources
`borders-records-a-join-twice-and-reads-lineage-three-ways` names.

## Next

Make the lineage walk answer "lies along" only for hops recorded at
or after the seam's step. For example, stop at a live ancestor (a
live edge key names its own span, not its parent's), or key
`split_from` by the recording step and read only the hops newer than
the seam. Then add `(true, true)` to the silhouette pin's orders, and
check that the six naming suites and `name_invariance` still agree.
