---
id: a-whole-member-rim-is-published-as-a-piece-of-itself-in-some-member-orders
kind: issue
title: A member rim the finished body holds whole is published as a piece of itself (Ends of its own corners) in the member orders that fold a covering member last
status: closed
closed: 2026-10-13
opened: 2026-10-12
priority: P2
cost: M
---



## What

Found while building the touch ruling (branch `emit/touch-crossing`).
The orders of the covered-contact row `{a, b, h}`
(`crates/editor-core/tests/emit_union_rim_piece_ranks.rs`,
`a_contact_b_covers_decides_in_every_order`) that main refused now
fuse.

**The two tables.** The fused orders publish two tables.
- `[a, b, h]` and `[b, a, h]` name `a`'s two top rims along `x` as
  `From(a, RimEdge) + Fragment(Ends(a's two corners))`. Each is a piece
  whose ends are the member edge's own two ends.
- The other four orders name the same finished edges as the bare
  `From(a, RimEdge)`.

The geometry is the same in both tables. A finished edge that is a
member edge whole takes the bare name (`Flush::holds_whole`), so the
first spelling is wrong.

This is not new. On main (acffb186a5), `[a, b, h]` and `[b, a, h]`
already disagreed with `[b, h, a]` and `[h, b, a]`. The two orders this
branch turns from `Emission` into builds join the second class. The row
pins both classes as measured.

## Next

Trace why the collapse keeps `Ends` on a piece that covers its member
edge, in the orders that fold `a` and `b` before `h`.

## Closed (branch `emit/union-order`, 2026-10-13)

**Cause.** In `[a, b, h]` and `[b, a, h]` the fold meets `a`'s and
`b`'s flush top rims first and joins them into one edge. `h` then cuts
that edge, so the fold spells the piece over `a` as
`Merged([a's rim, b's rim]) + Fragment(Ends)`. That spelling is not a
`From(m, e)` row, so `emit_union::group_member_edges` took the arm for
any other edge row that lies within a member edge, and that arm set
"whole" to false without reading it. The group was therefore always a
`Lone::Piece` and took `Ends`, even where the finished edge is the
member edge whole.

**Fix.** That arm now reads `Flush::holds_whole` against the least
member edge the row lies within, as the `From` arm does. So the name
comes from the finished body however the fold spelled the row.

**Measured.** `a_contact_b_covers_decides_in_every_order` now pins one
table across all 6 orders of `{a, b, h}`, where before it had two
classes (2 orders and 4).
