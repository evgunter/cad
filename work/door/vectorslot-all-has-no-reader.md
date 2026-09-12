---
id: vectorslot-all-has-no-reader
kind: issue
title: VectorSlot::ALL is a public hand-written list with no reader anywhere in the workspace
status: open
opened: 2026-09-11
---



Filed by the `BooleanOp::ALL` unit's sweep of the eight unswept
`const ALL` neighbours named in
`boolean-op-has-a-third-hand-written-complete-list`. It is not that
unit's class — it is the neighbouring one — and it is the only hit of
the eight that is dead rather than merely unforced.

## The finding

`crates/editor-core/src/node.rs:409` declares

    pub const ALL: [VectorSlot; 7] = [ … ];

on a `pub` type re-exported through `pncad::document`
(`crates/pncad/src/document.rs:78`). **Nothing in the workspace reads
it.** `rg 'VectorSlot::ALL'` over `crates/`, `demos/`, `tools/` and
`benches/` returns the declaration and nothing else; the type's other
seven uses (`crates/viewer/src/props.rs`, `crates/pncad-py/src/py/doc.rs`,
`crates/viewer/tests/panel_display.rs`) all name variants directly.
`Axis3::ALL` three hundred lines below (`node.rs:1941`) has readers in
production and in two suites, which is what the same shape looks like
when it is earning its place.

So it is a hand-written complete list, unforced, public, and read by
nobody: the class
`work/view/tool-kind-all-and-ordinal-have-no-production-reader` was
about, at a different site. Its doc says it is "for a consumer
enumerating families rather than reading one off a slot", and no such
consumer exists.

## What to decide

Delete it, or find the consumer that should have been reading it —
`SlotId::component` (`node.rs:565`) is the one exhaustive match over
the families, so a census over `ALL` is the natural row if one is
wanted. Whichever way it goes, the list should not stay public and
unread: an unread list is a ratification waiting to be inherited by
whatever is written at that name next, and nothing would notice it
going stale in the meantime.
