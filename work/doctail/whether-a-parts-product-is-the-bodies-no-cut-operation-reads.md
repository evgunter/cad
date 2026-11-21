---
id: whether-a-parts-product-is-the-bodies-no-cut-operation-reads
kind: issue
title: whether a split part's product is the ⊔ of the bodies in its world no cut operation reads as a body, the picture's drawing rule
status: open
opened: 2026-11-17
priority: P3
cost: M
design: true
---


## What

`split` (`crates/editor-core/src/refactor.rs`, `split`, "The part's
product") makes the part's product its one world copy, or the `⊔` of
its world copies: the one the cut holds where it holds a `⊔` of exactly
them that something reads, else one minted in document order
(ASSEMBLY.md A4 *Split*). Every cut world placement's copy is in it,
whether or not a cut operation consumes that copy.

The picture's drawing rule (SPACES' PR 6265, `Picture::of` in
`crates/viewer/src/world.rs`, its `reads_as_body` seats) draws every
body no operation reads as a body. Read the same way, the part's product
would be the `⊔` of the bodies in its world that no cut operation reads
as a body: a cut `u = a ∪ b` would make `u` the product, not
`a′ ⊔ b′`.

## The concern to weigh

A remainder mate on a face of `a` when the cut `u = a ∪ b` consumes `a`.
Today the instance's copy carries `a`'s piece (the product `⊔` holds
`a′`), so the mate's selection re-authors on the instance's copy
(`split`, the `crossing_reads` and `crossing_poses` loops; `in_world`).
Under the drawing rule the product would be `u`, which carries `a`'s
face only where the union keeps it, so that mate would cross only where
`u`'s name table holds the face, and would otherwise refuse
(`NameOutsidePartWorld`). The product slot reading `u` across the cut
(`SplitError::ProductReadSevered` today) is the same question from the
other side.

## Owed

A ruling (`design: true`): keep A4's "every world copy" product, or
adopt the drawing rule for a part's product, and what a remainder read
of a consumed copy then crosses as. Weigh with two designers first.
