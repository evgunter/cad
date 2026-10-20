---
id: an-intersect-of-an-intersect-names-a-three-way-coincident-face-apart-in-one-member-order
kind: issue
title: The silhouette letters' intersect of an intersect names one piece of the y 3 face with an extra wall when both intersects take the other member order
status: open
opened: 2026-10-20
priority: P2
cost: M
refs: [5368]
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
