---
id: loft-anchors-every-section-with-section-zeros-map
kind: issue
title: wire_loft anchors the whole emitted name table on section 0's LoopAnchor, so a section authored rotated or reversed relative to section 0 is named by section 0's permutation
status: closed
opened: 2026-09-16
priority: P0
cost: H
branch: emit/loft-anchors
pr: 3147
closed: 2026-09-24
---

Filed by EDIT's DM8 unit
(`work/edit/authored-step-to-canonical-segment-map-has-no-home`, PR
2759) from its style review's Claim 1. `crates/editor-core/src/eval/`
is WIRE's ground, so the finding goes here rather than being widened
there. The limitation is stated in the code — *"PINNED LIMITATION
(reported; review NOTE)"* above `first_naming` in
`eval::wire::wire_loft` — and **was never filed**, which is what this
row repairs: a note in a comment is not a scheduled row.

## The finding

`wire_loft` (`crates/editor-core/src/eval/wire.rs`) walks the sections,
takes each one's `ProfileNaming` from `section_of`, and keeps only the
FIRST:

- the loft emitter's profile refs are canonical `(loop, segment)`
  indices of the SECTION combinatorics;
- sections must correspond, so the first section's anchor is used as
  the canonical→program rewrite for the whole emitted table
  (`anchored(table, &first_naming)`).

So for every section other than section 0, the published
`ProfileEdgeRef` is anchored in SECTION 0's permutation rather than its
own. A section authored rotated or reversed relative to section 0 is
named by a map that is not its own, and a consumer asking about one of
ITS authored steps is off by the difference between the two
permutations.

## Why DM8 makes it worth scheduling

`ProfileProgram::profile_edges_of` (`crates/editor-core/src/program.rs`)
answers a document slot's `(loop_, step)` with the published
`ProfileEdgeRef`s — the coordinate VIEW's focus marking and the
chain-radius consumer both read. "Published" means "after the anchor
rewrite", so the door inherits exactly this limitation: its answer is
program-anchored for section 0 of a loft and section-0-anchored for
every other section. Nothing in the door can repair it, because the
refs the names carry are the ones the rewrite published; the repair is
here. The door's doc says so and cites this row.

Before DM8 the limitation had one reader (the loft's own naming); now
it has a door whose contract has to state it.

## What the answer has to settle

Whether the emitted table carries a PER-SECTION anchor (the refs are
per-section already, so the rewrite has the information it needs), or
whether the kernel's "sections must correspond" requirement is
strengthened so that a non-corresponding section refuses rather than
being named by section 0's map. The second is cheaper and may be the
right answer; it is a decision about what a loft admits, not a
threading change.

A row that goes red either way: a two-section loft whose second section
is authored reversed relative to the first, asking for a wall the
second section's own step swept.
