---
id: pick-split-sweep-missed-two-live-view-rows
kind: issue
title: the pick.rs split's tracker sweep missed two open VIEW rows that its own stated pattern would have matched
status: closed
opened: 2026-09-06
refs: [2079, stale-file-citations-after-the-split]
closed: 2026-09-06
pr: 2079
---



Found by the style review of #2079.

## What

#2079 discloses its sweep pattern: *"the grep was over `pick.rs`,
`pick::`, `PickIndex`, `PickCache`, `IndexInputs`, `PickKinds`,
`EDGE_PICK_RADIUS_PX`, `IdMap` and `evalseam::Generation` across
`work/`, `docs/`, `memories/`, `scripts/`, `.github/` and `demos/`"*,
and its stated blind spot is *"a citation that names neither a path nor
a moved symbol"*. Two open rows on VIEW's own slate name both, and
neither was corrected or announced:

- `work/view/focus-marking-is-per-node-not-per-segment.md:15,21,26` —
  `pick::focus` three times, and `:54` — *"Viewer ground
  (`crates/viewer/src/pick.rs`)"*. `focus` is `pickindex.rs:2076` now,
  and the item's whole subject moved with it.
- `work/view/the-picture-key-never-became-a-type.md:18` — *"asked five
  ways, in `crates/viewer/src/pick.rs` and
  `crates/viewer/src/evalseam.rs`"*. The first of the five,
  `PickIndex::current_for`, is `pickindex.rs:756` now, so the
  two-file list is a three-file list.

Both are `status: open`, both are inside VIEW's fence, and both name a
moved symbol or a path — inside the disclosed pattern, not outside it.
The blind spot the PR states is therefore not the one that bit.

## Class, not instance

The five rows that WERE corrected are the five citing
`crates/viewer/src/pick.rs:NNNN` — a file-and-line shape. The two
missed rows cite a bare path or a `module::symbol`. That is the shape
to sweep for, and the place to look is every `work/` row naming a
symbol that moved: `focus`, `highlight`, `edge_overlay`,
`edge_segments`, `edge_id_segments`, `cursor_projection`, `IdMap`,
`PickKinds`, `EdgePick`, `PickError`, `EdgeNameFault`, `PatchId`,
`EdgeId`. Sweeping only these two would be a half-fix.

## Confidence

`sure`.

## Closed

Both rows corrected, and the class swept rather than the two instances:
three separate greps — `pick::`-any-symbol, bare
`crates/viewer/src/pick.rs`, and each of the sixteen moved type names —
across `work/`, `docs/` and `memories/`, with every hit's item status
checked before disposition. That turned up one further out-of-fence row
the first pass missed, `work/fix/error-types-with-no-display-class.md`,
now announced with the others.

The blind spot is restated on
`stale-file-citations-after-the-split`, and it is not a pattern gap:
the first pass's `grep` over `work/view/*.md` was **malformed** (an
unescaped `|`, printing `command not found` above output read as a
result), and the disposition step then acted only on the
`pick.rs:NNNN` shape. The pattern was right and the reading of it was
not — which is a worse failure than a narrow pattern and worth the
item.
