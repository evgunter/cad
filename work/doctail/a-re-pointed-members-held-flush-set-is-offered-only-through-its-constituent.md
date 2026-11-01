---
id: a-re-pointed-members-held-flush-set-is-offered-only-through-its-constituent
kind: issue
title: After a union member is re-pointed, a held Merged{a, b} flush edge is offered its new name only two hops away, through a's constituent, and b's constituent offers nothing
status: open
opened: 2026-10-20
priority: P3
cost: M
---

Filed by WIRE's `wire-mintfirst` lane (branch
`wire/union-reads-not-mint-order`, PR 5488), from that PR's review.

## What

Since PR 5488, a union names an edge or vertex that several members hold
`Merged` of every member entity that holds it
(`emit_union::Flush`). Re-pointing a member through
`DocEdit::SetMembers` gives it a new read, so a stretch named
`Merged{From(a, e), From(b, e')}` becomes
`Merged{From(a, e), From(b2, e')}`. That is a recipe edit (DM4), and
N5's offers are the repair. But a reference held to the old set is not
offered the new one directly:

- the review measured `union_flush_fuses`' L-placed blocks after
  re-pointing `b` to its re-drawing: 26 of 39 held names vanish. The
  old set's new name is reached only in two hops, by unmerging to
  `From(a, e)` and then taking the sets that list it;
- `From(b, e')` offers nothing, since `b` is no longer read.

## What it needs

The offers rule for a vanished `Merged` set (`crates/editor-core/src/resolve/mod.rs`,
"Unmerge" and the sets that cover a name) to offer the live sets that list any of
its surviving constituents in one step, with a test over the re-point
above.
