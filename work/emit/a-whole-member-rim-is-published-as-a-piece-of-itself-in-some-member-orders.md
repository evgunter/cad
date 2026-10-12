---
id: a-whole-member-rim-is-published-as-a-piece-of-itself-in-some-member-orders
kind: issue
title: A member rim the finished body holds whole is published as a piece of itself (Ends of its own corners) in the member orders that fold a covering member last
status: open
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
