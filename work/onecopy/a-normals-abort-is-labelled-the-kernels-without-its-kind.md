---
id: a-normals-abort-is-labelled-the-kernels-without-its-kind
kind: issue
title: A face normal's abort is labelled the kernel's (off the surface) by callers that assume OffSurface is the only abort; DegenerateTorus (row 1(a)) aborts too
status: open
opened: 2026-11-09
priority: P2
cost: E
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
  (PR 6648), once it merges main: "a section chord's holding face reads
  it off the surface it was placed on".
- `topo` `census.rs` `normal_aborts` (PR 6660): correct at rest only
  through the premise that tier 3 refuses a degenerate torus before
  `census_verdict`. The Boolean reaches the same readers on a result
  whose operands only tier 2 admitted:
  `boolean/ops.rs` `coincident_wedges_disjoint` →
  `census::line_wedges` → `census::wedge`. There a degenerate-torus
  operand's wedge renders `TouchUnread::NormalFault`'s kernel-defect
  sentence, which is false of an invalid input.

## Owed

Whether an abort is a kernel fault (row 4) or an invalid input (row
1(a)) is the error's own fact: one method on `NormalAtError` (an
`ending()` or a fault kind) that all three read, pinned over every
variant. Coordinate with PR 6652, which makes `TorusRefusal::refusal()`
the home of the degenerate-torus class.
