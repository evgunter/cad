---
id: the-crossing-rung-answers-for-a-third-solids-crossing-inside-a-pairs-overlap
kind: issue
title: The crossing rung lets a confirmed pair answer for a crossing of a third solid's edges inside the pair's overlap
status: open
opened: 2026-10-23
---


`census.rs` `ee_cross_backed`, the `EdgeEdgeCross` rung, looks for a
pair among every recorded or confirmed face pair. The pair's carrier must
hold the crossing point and both crossing edges (`pair_holds_point`,
`pair_holds_edges`), and since PR 5605 the crossing must lie in the
closure of the pair's overlap (`in_overlap_closure`). Nothing requires
the two crossing edges to bound the pair's faces: the README's Crossing
backability says so ("found by carrier, not incidence").

So two edges of a third solid that lie in the seat's plane and cross
inside the seat's overlap are backed by the seat's pair. Such a third
solid has material in the seat's plane inside the overlap, where the
two seated solids already meet. Whether that is a touch the pair stands
for, or interference the crossing should name, is unread. No pose has
been built.

Pre-existing: the rung had this shape before PR 5605, which only added
the location test. Filed from PR 5605's dual review (R2, note 3).

Done when: the rung requires each crossing edge to bound a face of the
pair, or a row shows a third solid's crossing inside an overlap and
states why the pair answers for it.
