---
id: an-intersect-of-an-intersect-names-a-three-way-coincident-face-apart-in-one-member-order
kind: issue
title: The silhouette letters' intersect of an intersect names one piece of the y 3 face with an extra wall when both intersects take the other member order
status: closed
opened: 2026-10-20
priority: P2
cost: M
refs: [5368, a-direct-operands-own-split-lineage-climbs-to-a-live-sibling-seam]
branch: flush/three-way-coincident-names
pr: 5733
closed: 2026-10-26
---


## What

`editor-core` `emit_union_borders::the_silhouette_letters_name_through_an_intersect_of_an_intersect`
builds the tour's silhouette: an H on xy extruded +z 3, a T on yz extruded
+x 2, a C on zx extruded +y 3; `two` = H ∩ T and `three` = C ∩ `two`. It
reads each piece of `three` named by its walls as its own plane with its
walls' planes (`plane_splits`, walls read through the tables that minted
them).

Measured on PR 5368's frozen head `6b55b2286e` and on the fix pass after
it, identically: `H ∩ T` then `C ∩ two`, `T ∩ H` then `C ∩ two`, and
`H ∩ T` then `two ∩ C` name every piece alike. `T ∩ H` then `two ∩ C`
differs in one piece of the y 3 face: `("y=3.00", ["x=1.50"])` there
against `("y=3.00", ["x=1.50", "z=2.50"])` in the other three. The y 3
plane is coincident three ways: the H's top end, the T's bar end and the
C's cap. The pin asserts the three agreeing orders.

## Next

Dump `three`'s discard rows and substituted rows in the two orders and
find which discard brings the z 2.5 wall to the piece over x 1.5..2: a
border read across the merged `two` face's coincident fragments, or the
emitter's chase through `two`'s own table. Which is right decides whether
this is FLUSH's or EMIT's.

## Measured (2026-10-26, `d82dc04c`)

PR 5706 did not make the orders agree. With the fourth order added to
the pin, `T ∩ H, two ∩ C` still names the y 3 piece over x 1.5..2
`("y=3.00", ["x=1.50", "z=2.50"])`. The other three orders name it
`("y=3.00", ["x=1.50"])`, at ε 1e-9, 1e-6 and 1e-12 alike. Every
other piece agrees in all four orders. Note that this row's What has
the two readings swapped: the odd order is the one with z 2.5.

No containment fallback runs: all eight booleans (two per order) take
the main path, so `BooleanNaming::uncut` is empty. The topo rows
agree. `three`'s discard rows in both `two ∩ C` orders are the same
two C fragments, and `two`'s rows are identical for one member
order. The first record that differs is in the names layer:
`borders.rs` `Obstacles::walls`'s `reached` walk (via
`Obstacles::up_lineage`) from the piece's z 2.5 edge. In the odd order
`two` is `three`'s Direct operand A, and that edge carries `two`'s own
`SplitEdge` provenance onto the sibling edge over x 0..0.5, which kept
T's edge key. In `three` that sibling is a seam of C's L-shaped
discard, which borders the other piece too, so the walk makes it a
divider for this piece.

**Which reading is right.** The three agreeing orders. `three`'s y 3
parent is `Merged` {C's cap, `two`'s y 3 face}, and neither holds
region across that edge. The region below it at x 1.5..2 was H's leg
end, lost in `two`'s boolean, not in `three`'s.

## Resolution

BORDERS' ground, not FLUSH's: the boolean's rows are the same in every
order, and the chase through the operand's own lineage is what differs.
It is filed as
`work/borders/a-direct-operands-own-split-lineage-climbs-to-a-live-sibling-seam.md`,
with the dump and a fix direction. The pin keeps the three agreeing
orders and now cites that row.
