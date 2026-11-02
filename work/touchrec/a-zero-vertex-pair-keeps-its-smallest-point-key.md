---
id: a-zero-vertex-pair-keeps-its-smallest-point-key
kind: issue
title: A vertex pair the boolean decides Zero moves onto its smallest live point key, an arena-order choice, not the glue's kept side
status: closed
closed: 2026-11-01
opened: 2026-10-27
priority: P1
cost: M
branch: touchrec/kept-side-reads-the-glue
pr: 6210
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

2026-11-01: re-banded P2 → P1. This row is a remaining instance of the closed P1 `the-kept-side-of-a-glued-pair-is-chosen-in-three-places` (PR 5848), whose sweep filed it: a second place choosing which description a glued pair keeps, beside `glue::kept_first` — "two implementations of one underlying logic", P1 by `work/README.md` Priority. (TOUCHREC orchestrator)

## Closed

PR 6210. `zip::share_points` reads `ops::PointOwners` (which operand's
point each result key holds, from the graft) and moves a class onto its
smallest live key holding the glue's kept side's point, else its
smallest live key; the seam classes and the record classes share that
one order. The premise "seam classes hold bitwise-equal points" was
false (two A keys an ulp apart in `union_flush_onto_edge_contact`), and
the seam call's key-number order could put an A vertex on a lower B key;
both are gone. Pinned by
`a_zero_vertex_pair_keeps_the_kept_sides_point::a_zero_corner_kiss_keeps_the_first_operands_point_over_a_lower_key`.
Filed alongside: `work/touchrec/operands-convert-to-array-indices-by-hand.md`.
