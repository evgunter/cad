---
id: a-split-through-a-vertex-whose-above-runs-nest-crosses-its-section-face
kind: issue
title: A split through a vertex whose above runs nest under one builds a section face whose corners there overlap
status: open
opened: 2026-10-08
priority: P1
cost: M
---


Found by TANG's sweep for `nested-pierce-runs-have-no-ring-order`
(2026-10-08), the split lane's sibling of the pierce ring.

## What

`topo::split` of `topo::test_support::meeting::apex_pyramid` on the
plane through its apex at `MEET`, normal +z, builds both sides at their
closed-form volumes, but each side's section face has two corners at the
apex that overlap (`meeting::corners_disjoint` fails), in every one of
`meeting::poses()`:

| base | runs above, about +z | above | below | corners |
|---|---|---|---|---|
| a single crossing | 1 | 0.015 | 0.030 | disjoint |
| a zigzag, two arches up | 2, apart | 0.010 | 0.025 | disjoint |
| a crown, three arches up | 3, apart | 0.015 | 0.040 | disjoint |
| `meeting::comb` | 3, two nested under one | 0.040 | 0.015 | **overlap** (both sides) |
| `meeting::arch_cone` | 3, one inside another inside a third | 0.050 | 0.030 | **overlap** (both sides) |

The volumes are right. Tier 3 does see the crossed loop: check 9's
corner arm returns `ValidationError::PinchCornerCrossed` from
`validate_geometric` on every crossed side (20 of 20, PR 4300's review).
The split ships the body with no refusal only because `split` does not
run tier 3 on its sides (`splitting/mod.rs`, `split`'s doc: a pinch
side's touching pieces carry contacts it declares nowhere). A cheap
backstop until the order is read: run the corner check alone on split
sides, and refuse a side that fails it.

## The likely cause

`splitting/insert.rs` `insert_null_edges` mints one null edge per above
run at the vertex, and the section face is closed through them in an
order that assumes the runs' germ intervals lie one after another about
the plane's normal. The above runs are chords of the vertex's link that
do not cross but can nest. The boolean's pierce ring had the same
assumption: `boolean/vtxfac.rs` now sorts the runs' germs clockwise
(`germ_order`) and hangs its struts as the tree that sort gives
(`crate::null::ring_tree`), a star where one region borders every run,
a path or deeper tree otherwise. The split lane wants the same reading,
or a typed refusal, rather than a
crossed section face.

## Rows owed

The table above as a row, each pose, asserting `corners_disjoint` on
both sides with the closed-form volumes (the pyramid's volume is a
third of half a unit times the base polygon's area, split by `z = 0`).

The boolean's reading is now the pure `crate::null::ring_tree(order,
root)` (germs in clockwise order to the tree of struts, star or not),
which the split lane can call. Both designers who weighed the pierce
ring judged this split half the more urgent: it ships an unsound body
where the boolean refused.
