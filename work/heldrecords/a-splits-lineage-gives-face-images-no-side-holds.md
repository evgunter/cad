---
id: a-splits-lineage-gives-face-images-no-side-holds
kind: issue
title: The split's lineage gives face images that neither side holds
status: open
opened: 2026-10-29
---


Measured by PR 5981's delta review (`splitdoor/pinch-corner-split-review2`).

## What

In all 1200 cells the review built through
`crates/topo/tests/review_pinch_corner_split.rs`'s sweep (the L-prism ∪
tilted cube pinch unions, two to four cones, planes through the pinch),
`SplitResult::lineage` gives some operand face an image that neither side
holds. Zero-fusion cells do it too, so it is not the pinch fusion's.

The face rows are unchanged from main: `crates/topo/src/splitting/finish.rs`,
`SplitResult::lineage`, the loop over `roots(&fragments)`, which states each
face-division root as `Fate::Divided` over every piece its
`face_fragments` descent names, read through the side's merges and joins.
Which pieces no side holds, and whether the rows mean them to be dead
(a piece a later stage killed without a substitution) or the lineage
should drop them, is unmeasured: the intent is unknown.

## Owed

Measure which pieces are dead and which stage killed each, then either
give the kill its lineage row or state why the carry may read past it.
