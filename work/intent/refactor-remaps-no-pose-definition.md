---
id: refactor-remaps-no-pose-definition
kind: issue
title: Split and inline refuse a document whose node reads a pose definition (RemapMiss::Read)
status: open
opened: 2026-10-10
---

## What

`refactor.rs`'s read remap (`rd_output`, `crates/editor-core/src/refactor.rs:364`)
carries a read across a split or an inline only when the variable is an
output of a carried node (`out_map`); any other variable falls to
`source.operation_of(var)`, and a pose definition (`VarDef::Pose`) has no
operation, so it answers `RemapMiss::Read { output: false, .. }`
(`refactor.rs:368`). A document whose node reads a pose definition — a
split whose tool is `Plane { face }`, a profile on a `Through` frame — so
refuses `split` and `inline` wholesale.

## Fix shape

A pose definition is carried like a selection: re-minted in the carried
document with its reads remapped (its face selection's body read through
the node map, its scalars through the fresh table), and refused by name
only when a read leaves the carried set. Filed by INTENT stage 3 A, which
introduced pose definitions and did not widen the refactor.
