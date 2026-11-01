---
id: a-zero-vertex-pair-keeps-its-smallest-point-key
kind: issue
title: A vertex pair the boolean decides Zero moves onto its smallest live point key, an arena-order choice, not the glue's kept side
status: open
opened: 2026-10-27
priority: P2
cost: M
---


Found by the sweep of `the-kept-side-of-a-glued-pair-is-chosen-in-three-places`
for other places that choose which description a glued pair keeps.

## What

`ops::record_classes` (crates/topo/src/boolean/ops.rs) decides each
vertex-vertex record pair on two point keys Zero at the band, and
`zip::share_points` (crates/topo/src/boolean/zip.rs) then moves every
vertex of a class onto "its smallest live key". A Zero pair's two
points may differ inside the band, so this chooses which point the
glued vertex pair keeps, and it chooses by arena order: ordinarily
operand A's (the result arena is A's clone), B's wherever a grafted
point took a slot below A's.

DESIGN.md `[one-way.places]` lists "Which description a glued pair
keeps | decided once, at the glue | [coincidence.booleans]": the
first-written operand's, read from `boolean::glue::kept_first`, which
the zip's fused vertices and the merge already read.

## Owed

Each record class keeps the point of its first-written operand's
vertex, read through the glue's decision; the smallest key stays only
as the order among keys of one operand. The classes from the seams
(`zip::point_classes`) hold bitwise-equal points, so only the record
classes move a value.

## Slate triage

2026-11-01: moved from `work/cleave/` by CLEAVE's split on its semantic seams (Ev, in chat: CLEAVE measured 114 budget points against 30). TOUCHREC holds a result's records of its own touches and the glue's kept copy; id and body unchanged. (CLEAVE orchestrator)
