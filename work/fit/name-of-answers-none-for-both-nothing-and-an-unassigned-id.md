---
id: name-of-answers-none-for-both-nothing-and-an-unassigned-id
kind: issue
title: PickIndex::name_of answers None for both the clear value and an id it never assigned, so every reader rebuilds the split
status: open
opened: 2026-09-25
priority: P3
cost: D
---

Filed by `vnews/an-unnamed-id-is-not-nothing` (PR #3249).

## The finding

`crates/viewer/src/pickindex.rs`'s `PickIndex::name_of` returns
`Option<&Result<StableName, HitTestError>>`. Its `None` means two
different things: `IdMap::NOTHING` (the clear value, which
`id.checked_sub(1)?` turns into `None`) and an id the index never
assigned. The second is what a corrupt readback looks like. So every
reader that needs to tell these apart has to rebuild the split itself:

- `crates/viewer/src/idpass.rs`'s `IdAnswer::of` tests `IdMap::NOTHING`
  first, then reads `None` as `IdAnswer::Unassigned`. It is correct
  only because of that ordering.
- `crates/viewer/src/marks.rs` (focus attribution,
  `index.name_of(id)?.as_ref().ok()?`) folds both into a skip. That
  fold is filed on VGEOM as `marks-focus-drops-an-unnamed-patch-from-attribution`.

The edge side already has the shape this wants.
`PickIndex::edge_name_of` returns `Result<&StableName, EdgeNameFault>`,
whose arms (`NotDrawn`, `OutOfRange`, `Unnamed`) each own their words.

## What a fix would be

Have `name_of` return a typed value in the same style, one that keeps
nothing, an unassigned id and an unnamed patch apart. `IdAnswer` then
collapses into it. `idpass.rs` would read the door's answer rather than
re-derive it, and its `Display` would move beside the type, as
`EdgeNameFault`'s already has.

## Fence

`crates/viewer/src/pickindex.rs` (FIT), with the callers in
`idpass.rs` and `marks.rs` following.
