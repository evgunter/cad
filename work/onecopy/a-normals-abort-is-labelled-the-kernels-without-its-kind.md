---
id: a-normals-abort-is-labelled-the-kernels-without-its-kind
kind: issue
title: A face normal's abort is labelled the kernel's (off the surface) by callers that assume OffSurface is the only abort; DegenerateTorus (row 1(a)) aborts too
status: closed
opened: 2026-11-09
priority: P2
cost: E
refs: [6655, 6648, 6660, 6652]
closed: 2026-11-09
pr: 6689
---


Filed by `onecopy/census-normal-refusals` (PR 6660), from the
orchestrator's reading of 6660's review.

## What

Since main's 498685d1 `NormalAtError::refusal()` aborts on two
variants: `OffSurface` (DESIGN `[failure.taxonomy]` row 4, a kernel
fault) and `DegenerateTorus` (row 1(a), an invalid input). Callers that
render any abort as a kernel fault, or as "off the surface", mislabel
the second:

- `topo` `boolean/slit.rs` `walls_inside`'s `normal` closure (PR 6655):
  an abort is `ClassificationInvariant` "an edge's midpoint reads off
  the surface of a face it lies in or bounds".
- `topo` `boolean/join.rs` `HoldingUnread::aborted`, its `Normal(_)` arm
  (PR 6648, which marks it with a `// Gap:` comment), once it merges
  main: "a section chord's holding face reads it off the surface it was
  placed on".
- `topo` `census.rs` `normal_aborts` (PR 6660): correct at rest only
  through the premise that tier 3 refuses a degenerate torus before
  `census_verdict`. The Boolean reaches the same readers on a result
  whose operands only tier 2 admitted:
  `boolean/ops.rs` `coincident_wedges_disjoint` →
  `census::line_wedges` → `census::wedge`. There a degenerate-torus
  operand's wedge renders `TouchUnread::NormalFault`'s kernel-defect
  sentence, which is false of an invalid input.

Its sibling in the same ladder, `seam_germ_side`'s region normal (the
`chord` closure, `topo` `boolean/join.rs`), already words a degenerate
torus on its own by matching `NormalAtError::DegenerateTorus` ("a section
chord's region is a torus outside the ring convention, which the germ
pair's frame refuses first") and every other abort as off the surface.
So one ladder ends the same refusal two ways: the region normal by its
own variant match, the holding face's normal as "off the surface".

Related: `work/splitdoor/boolean-and-census-spell-the-normal-at-refusal-and-the-cut-order-by-hand.md`
(P3) counts six hand mappings of `NormalAtError` to a caller's error and
asks for one door per error family. This row is the part of that class
the fault kind decides (which ending an abort takes); the method owed
here is the piece that door reads, and that row's remaining mappings
(`Escalated` → an escalation, a kept refusal → a typed no-normal) stay
on it.

## Owed

Whether an abort is a kernel fault (row 4) or an invalid input (row
1(a)) is the error's own fact: one method on `NormalAtError` (an
`ending()` or a fault kind) that all three read, and `seam_germ_side`'s
variant match with them, pinned over every variant. Branch
`onecopy/normal-fault-kind`, once PRs 6655 and 6660 land. Coordinate with PR 6652, which makes `TorusRefusal::refusal()`
the home of the degenerate-torus class.
