---
id: name-of-answers-none-for-both-nothing-and-an-unassigned-id
kind: issue
title: PickIndex::name_of answers None for both the clear value and an id it never assigned, so every reader rebuilds the split
status: closed
closed: 2026-10-22
branch: fit/id-answer-split
opened: 2026-09-25
priority: P3
cost: M
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
  first, then reads `None` as `IdAnswer::Undrawn`. It is correct
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
reads the door's split rather than rebuilding it. It does not collapse
into it whole: `IdAnswer::Undrawn` also holds an id the index assigned
to a part whose root the display view hides, which `name_of` names,
and that half is a question about the view, not the index. `idpass.rs` would read the door's answer rather than
re-derive it, and its `Display` would move beside the type, as
`EdgeNameFault`'s already has.

## Fence

`crates/viewer/src/pickindex.rs` (FIT), with the callers in
`idpass.rs` and `marks.rs` following.

## Closed — `name_of` answers `Result<&StableName, PatchNameFault>` (fit/id-answer-split)

`PickIndex::name_of` refuses with `pickindex::PatchNameFault`, which has
three arms: `Nothing` (the clear value), `Unassigned { id }` (what a
corrupt readback looks like) and `Unnamed { id, error }` (the naming
layer's bug arm). It follows the `EdgeNameFault` shape, and its `Say`
and `Display` sit beside the type. `idpass::IdAnswer::of` now matches
on the door's arms. It no longer tests `IdMap::NOTHING` first, and
`IdAnswer` says nothing and an unnamed patch in the fault's words.
`IdAnswer::Undrawn` still covers the view half: a hidden part's id,
which the index does name. `tests/pick_windows.rs`'s hand-walked model
now spells the three arms, so the differential over `0..=len + 2`
covers `Nothing` and `Unassigned` separately.

`marks::focus` reads `name_of(id).ok()`. It iterates only assigned ids,
so the one arm it drops is `Unnamed`. That drop is still the VGEOM row
`work/vgeom/marks-focus-drops-an-unnamed-patch-from-attribution.md`,
which stays open: a count needs a surface on the `sync_scene` seam, so
the new type does not make it a one-liner.
