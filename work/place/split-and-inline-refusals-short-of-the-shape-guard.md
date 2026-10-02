---
id: split-and-inline-refusals-short-of-the-shape-guard
kind: issue
title: Split and inline refusals short of the shape guard
status: open
opened: 2026-10-01
priority: P3
cost: E
---

## What

`crates/editor-core/tests/refusal_concision_refactor.rs` holds every
`SplitError` and `InlineError` arm to `test_utils::refusal::problems`,
the standard `EditError`'s arms meet
(`crates/viewer/tests/refusal_concision_edits.rs`). The arms P2 built
or touched state their recourse. These older arms render with none, and
are admitted by exact row id in that file's `FILED_NO_RECOURSE`, under
the comment naming this file:

- split (`crates/editor-core/src/refactor.rs`, `SplitError`'s
  `Display`): `EmptyCut`, `UnknownCutNode`, `PartIdCollides`,
  `SeveredEdge`, `OperandSeveredFromMate`, `UncutParamReference`,
  `PartNameReachesRemainder`, `NameStraddlesCut`, `NameOnDroppedStep`,
  `BodyNameCrossesCut`, `Pin`, `StepMapDiverged`;
- inline (`InlineError`'s `Display`): `UnknownNode`, `NotAnInstance`,
  `InstanceConsumed`, `Unresolved`, `EpsilonSeam`,
  `PartCarriesMetadata`, `ParamConflict`, `InstanceBodyNameReferenced`,
  `ForeignInstanceName`, `NameOnDroppedStep`, `StrandedPartName`,
  `StepMapDiverged`.

Several already say what to do in prose ("widen the cut", "repair the
stranded reference before splitting") and need only the `Recourse:`
form; `StepMapDiverged` says "which is a kernel bug" and wants
`geom_core::KERNEL_DEFECT_ENDING`; `Split/Pin` forwards a `persist:`
refusal under its stage word.

## Done when

Each arm states one recourse (or "There is no way through"), and its
row leaves `FILED_NO_RECOURSE`.
