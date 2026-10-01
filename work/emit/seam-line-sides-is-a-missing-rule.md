---
id: seam-line-sides-is-a-missing-rule
kind: issue
title: SeamLineSides: a chain along a seam known only by name refuses when its faces' names do not settle the pair's sides
status: open
opened: 2026-09-24
priority: P3
cost: D
---

## What

`NamingError::SeamLineSides { node, edge }`
(`crates/editor-core/src/names/emit.rs`) is raised by `seam_line_dir` in
`crates/editor-core/src/names/emit_topo.rs`. Two rankers know a seam
only by its NAME: the descent ranker, for the pieces of a seam a later
step cut, and the seam-vertex carrier. Each has to work out which of the
seam edge's two faces is the pair's `a` side. It does this by matching
the faces' names against the pair, through the pass-through wrappers
that `crates/editor-core/src/names/seam_pair.rs` lists.

The sides stay unsettled, and the rank refuses, in two cases:
- **Neither assignment fits.** A face was renamed by a node that the
  wrapper list does not see through, so it descends from neither side.
  The split's `SplitFragment` was one such node until PR #3125 added
  it.
- **Both assignments fit.** Each face is a merged face whose
  constituents come from both sides.

A pair whose two sides carry the same NAME (two placements of one
prototype) does not reach this refusal. That pair names no side, so its
pieces rank along their own carrier.

## Why this is a missing rule, not a bug

The body is sound and the recipe is legal. What is missing is a rule
for orienting that line. `SharedRim` is the precedent for this
classification.

## Band

P3 while no document is known to reach it. No fixture in the tree does:
PR #3125's rows and the reviewer's seven 4-member fixtures all name.
Move it to P0 once a legal document is found that refuses here.

## Fix shape (measure first)

Name matching alone cannot settle either case. Two options:
- Carry the seam's minted side structurally: record at the seam's mint
  which face was A, in a form that survives the pass-throughs.
- Fall back to the edge's own carrier. This is consistent with the
  union's collapse only where the collapse cannot swap the pair (an
  equal pair), so it is not a general answer.
