---
id: a-param-jump-that-swaps-the-outer-loop-renumbers-names-unreported
kind: issue
title: A SetParam that jumps a hole past its outer loop (or flips a loop's sense) renumbers every name on the swapped loops, unreported
status: open
opened: 2026-09-24
priority: P0
cost: D
---


Since PR #3147, published profile names carry CANONICAL indices:
- the loop order is outer first, then holes in description order;
- each loop is traversed in its canonical sense from its authored start.

So a name moves with two of canonicalization's decisions: which loop is
outer, and each loop's orientation. Neither can change under a
continuous edit without passing through a state validation refuses. The
loops would have to cross, or a loop would have to become a sliver.

A `SetParam`, though, jumps. For example: `plate_param`-like, a square
with a circular hole whose radius is a parameter, raised in one edit
until the circle encloses the square. The new state validates, the circle
becomes canonical loop 0, and every name on either loop now denotes the
other loop's entities. No maintenance row reports it. The same is true
of a vertex moved in one edit to the other side of its loop, which flips
the sense and maps `s ↦ n−1−s`.

Before #3147 names were program-anchored, and this class did not
exist for the loop order. It is the cost side of Ev's Option-2 ruling on
PR 3102, stated in `crates/editor-core/src/eval/anchor.rs`'s module
docs.

Found while writing
`edit_set_program::a_program_that_replays_but_does_not_validate_keeps_its_names`.
With hole radius 1.5 the old program validated with its roles swapped,
and the door correctly rebound every name to the at-rest numbering.

Candidate fix: `SetParam` (and every slot edit) compares the profile's
canonical role/orientation vector before and after, and reports or
remaps the names like SetProgram does. It needs the same replay-and-anchor
read the SetProgram door now makes.

## Re-banded P0 at merge (orchestrator)

A name that renumbers without being reported is a silent rebind. By
`work/README.md`'s bands that is a live wrong answer, P0, however rare
the edit that triggers it.
