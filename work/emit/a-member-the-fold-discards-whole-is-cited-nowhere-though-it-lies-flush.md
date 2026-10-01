---
id: a-member-the-fold-discards-whole-is-cited-nowhere-though-it-lies-flush
kind: issue
title: A union member the fold discards whole, inside the accumulation and flush with it, is cited nowhere, while folded earlier it parents the merged faces it lies in
status: open
opened: 2026-10-01
priority: P1
cost: M
design: true
needs_ev: true
branch: emit/fold-discarded-member
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

## Re-measured (CLEAVE, 2026-10-01, origin/main `9d0b86ec2` merged)

Main now names some member-edge pieces by their ends
(`Fragment(Ends([..]))`), and those ends cite `b`'s corners. With that
change the absences grow to 168 over `r4tri` and 564 over `r4trig`.
The cause is the same: `b` is cited nowhere in the orders that discard
it whole. `KNOWN_ABSENT` pins both the counts and an FNV digest of the
absent (order, name) set. The same row asserts that every fused order of
the two cases publishes one multiset of entity geometries, and that
assertion holds.

## Root cause (emit lane, 2026-10-01, origin/main `a153d1826`)

The fallback is one instance, not the cause. A union's parents are
the member faces a finished face descends from (`emit_union::Fold::step`,
reading `BooleanNaming`'s merge groups and result-face descent). Where
a member's flush face is wholly covered by a coplanar kept face, the
kernel keeps only one copy of the coincident region, and which copy
depends on which side is operand A. The covered face is cited when
its member is operand A and dropped when it is operand B, with or
without the containment fallback. A scratch probe over two declared
unions shows it with only two members:

- `a` = x 0..2, `b` = x 0.5..1.5, both y 0..1, z 0..1, all four
  families declared (`b` inside `a`). Order `[a, b]` publishes all
  six faces as `a`'s, and `b` is cited nowhere. Order `[b, a]`
  publishes both caps and both y-walls as `Merged([a, b])`.
- `a` as above, `b` = x 0.5..1.5, y 0..1, z 0..2, declared on the
  two y-walls and the bottom cap only. `b` pokes out of the top, so
  no fallback is involved. Its bottom face is wholly covered by `a`'s.
  Order `[a, b]` names the bottom `a`'s alone. Order `[b, a]` names
  it `Merged([a, b])`. The y-walls, where `b` contributes surface,
  are `Merged([a, b])` in both orders.

Neither two-member case is in `emit_union_rim_piece_ranks`'s corpus,
so `KNOWN_ABSENT` measures only the three-member face of the class.

## The fork

Making the table order-free means choosing what a merged face cites:

1. **Every declared-flush member face lying on the kept face**,
   whether or not it contributes surface. `b` is cited in every order
   of all four cases above. The kernel's coincident-copy discard (and
   the fallback, whose finish drops cross-operand declared pairs as
   "inapplicable") would have to record the dropped copy as a merge
   constituent, or the union would have to link it from the declared
   pairs it holds.
2. **Only member faces that contribute surface to the finished face.**
   `b` is cited in no order of `r4tri` (its cap is covered by
   `a ∪ c`) and in neither two-member case. That needs a rule for
   faces that cover each other exactly (two coincident placements),
   where "contributes" picks no side.

Each one changes which members a merged face cites, and so the names
published in some orders today. That is a change to N3's reading of
"declared-coincident faces merge", so it is weighed before a lane
builds it.

