---
id: a-pure-move-falsifies-sentences-about-structure
kind: issue
title: the split's negative result generalises wrongly: a behaviour-preserving move falsifies sentences about structure, and it falsified three
status: closed
opened: 2026-09-06
refs: [2079, stale-file-citations-after-the-split]
closed: 2026-09-06
pr: 2079
---



Found by the style review of #2079.

## What

`work/view/stale-file-citations-after-the-split.md:191-199` records the
third member's contribution as a negative result:

> this split produced none of those — every hit was a pure re-point,
> because **a move that changes no behaviour cannot falsify a sentence
> about behaviour**. The two halves of this item come apart cleanly
> here.

The premise is true and the conclusion does not follow. A refactor
tracker is not mostly sentences about behaviour; it is mostly sentences
about WHERE things are and WHAT NAMES WHAT, and a pure move falsifies
exactly those. Three were falsified by this one, and all three are
claims rather than numbers:

- `crates/viewer/README.md:323` — the `session::op` row says
  `SessionOp` is *"read by `tools`, `pick`, `frame`, `blend`,
  `combine`, `matetool`, `revolvetool`"*. After the split `pick.rs`
  names `SessionOp` nowhere (grep: zero hits); `pickindex.rs` names it
  seven times. The row lists a reader that is not one and omits the
  one that is — in a file this PR re-read and corrected two other rows
  of.
- `work/view/the-picture-key-never-became-a-type.md:18` — *"asked five
  ways, in `crates/viewer/src/pick.rs` and
  `crates/viewer/src/evalseam.rs`"*: now three files.
- `work/view/focus-marking-is-per-node-not-per-segment.md:54` — the
  unit's *"Viewer ground (`crates/viewer/src/pick.rs`)"* is
  `pickindex.rs` now.

## Why this matters more than the three instances

The negative result is the item's closing argument, and it is what a
future lane will cite to decide that a `<file>.rs:<line>` gate is
sufficient after a move. It is not: a gate on line citations would have
caught none of the three above, and the README row is precisely the
kind of sentence a mechanical check cannot reach. The correct
statement is narrower — *a move cannot falsify a sentence about
behaviour, and reliably falsifies one about location* — and the item's
two halves come apart on a different axis from the one it names.

## Confidence

`sure` on the three sentences being false today. `likely` that the
generalisation as written is the part worth fixing.

## Closed

All three sentences corrected — `crates/viewer/README.md:323`'s
`session::op` reader list now names `pickindex` where it named `pick`;
`the-picture-key-never-became-a-type.md:18` is a three-file list;
`focus-marking-is-per-node-not-per-segment.md` is re-pointed at
`pickindex.rs` in all four places.

The generalisation is rewritten where it lives, in
`stale-file-citations-after-the-split`'s third-member section, to the
statement this item argues for: **a behaviour-preserving move cannot
falsify a sentence about behaviour and reliably falsifies one about
structure** — the half a behaviour-preserving unit is least primed to
look for, since its whole discipline is aimed at the other one. That
also relocates the parent item's two halves onto *what a machine can
reach* rather than *numbers versus claims*, which is the axis this
item found.
