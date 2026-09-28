---
id: eval-kernel-bug-refusals-end-in-a-tag
kind: issue
title: editor-core: NodeErrorKind MissingSlot and VerbArity end in '(a kernel bug)', not the shared kernel-defect ending
status: open
opened: 2026-09-28
---


(ENCL implementer, from the sweep of
`work/encl/kernel-defect-endings-and-repair-labels-have-no-shared-home.md`.)

## What

`editor_core::NodeErrorKind`'s `Display` (`crates/editor-core/src/eval/mod.rs`)
ends two kernel-bug arms in a "(a kernel bug)" tag:

- `MissingSlot` (`eval/mod.rs:2051`): "…which is absent (a kernel bug)".
- `VerbArity` (`eval/mod.rs:2068`): "{refusal} (a kernel bug)".

The tag carries no `There is no way through` marker (zero to
`test_utils::refusal::recourse_markers`) and no report. The ending has
one home now, `geom_core::KERNEL_DEFECT_ENDING` ("There is no way
through: this is a kernel defect; report it").

## Repair shape

Replace each tag with ". {KERNEL_DEFECT_ENDING}".
