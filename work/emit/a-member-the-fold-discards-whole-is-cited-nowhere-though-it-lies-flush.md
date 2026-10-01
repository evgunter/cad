---
id: a-member-the-fold-discards-whole-is-cited-nowhere-though-it-lies-flush
kind: issue
title: A union member the fold discards whole, inside the accumulation and flush with it, is cited nowhere, while folded earlier it parents the merged faces it lies in
status: open
opened: 2026-10-01
priority: P1
cost: M
---



## What

`r4tri` in `crates/editor-core/tests/emit_union_rim_piece_ranks.rs`:
`a` = x 0..1, `b` = x 0.5..1.5, `c` = x 0.8..2, all over y 0..1 and
z 0..1, every pair declared flush. Until the CLEAVE uncut-shell witness
(`crates/topo/src/boolean/shell_witness.rs`), orders `[a, c, b]` and
`[c, a, b]` refused `Boolean(Containment(RayExhausted))`
(`work/cleave/a-contained-flush-operand-with-every-vertex-on-the-boundary-refuses-as-ray-exhausted.md`).
They now fuse to the x 0..2 block, like the other four orders, but they
publish a different name set.

In those two orders the last fold step is `(a ∪ c) ∪ b`, and the
containment fallback (`ops.rs`, `fallback`) classifies `b` In and
discards it whole: the step's result is the accumulation. So `b` is
cited nowhere in the table. In the four orders that fold `b` before
`a` or `c` is complete, `b` is a parent of the merged caps and y-walls,
and its rim pieces and corners are named.

## Measured (CLEAVE, 2026-10-01, on `cleave/interior-witness`)

`a_name_two_member_orders_both_publish_denotes_the_same_geometry`, with
the absences grouped by (case, order). Every order of `r4tri` lacks 16
names that some other order publishes, 96 in all. Every fused order of
`r4trig` lacks 31, over 12 orders, 372 in all. In `[a, c, b]` the 16
absent names are:
- 4 faces: `Merged([a cap, b cap, c cap])` at each end, and the two
  y-walls `Merged([a, b, c])`. The order publishes `Merged([a, c])`
  for those faces instead.
- 4 rim-edge fragments of `b` (`OrderAlong { rank: 0 | 2, of: 3 }`).
  The order names those stretches as `c`'s rim pieces instead.
- 8 corners of `b` (`CapVertex`).

No name denotes different geometry in two orders, since the
`signature` equality in the same row holds. The difference is names
present in some orders and absent in others. `KNOWN_ABSENT` in that
file pins the counts, and `a_flush_union_publishes_one_table_in_every_member_order`
leaves the two unions out.

## What a fix decides

Whether a member discarded whole, but lying flush on faces the result
keeps, is a parent of those faces (the declared pair says its face is
coplanar with the kept face), or whether the orders that fold it before
the others should stop citing it. Either one makes the table
order-free.
