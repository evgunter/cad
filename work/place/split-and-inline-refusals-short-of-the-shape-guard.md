---
id: split-and-inline-refusals-short-of-the-shape-guard
kind: issue
title: Split and inline refusals short of the shape guard
status: closed
opened: 2026-10-01
priority: P3
cost: E
closed: 2026-10-02
branch: place/split-inline-recourse
pr: 3872
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

## Closed

Every listed arm now ends on one `Recourse:` read off its raising site
in `crates/editor-core/src/refactor.rs` (`split`, `inline`): a cut
edit (add the kept endpoint, leave the cut one out), a name repair
(`Rebind`, `ClearAppearance`), a parameter or tolerance edit
(`SetDocParam`, `SetTolerance`), or a repair in the referenced
document followed by `UpdateReference`. `FILED_NO_RECOURSE` and its
plumbing are deleted from `refusal_concision_refactor.rs`.

- `Split/Pin` ends on `geom_core::KERNEL_DEFECT_ENDING`: the part
  replayed clean through the edit doors, so a pin the save validator
  refuses is this module's defect. It forwards the persist refusal's
  stage-stripped sentence (`Staged::sentence`), since its own clause
  already names the stage, and the `("Split/Pin", "persist")` label
  allowance is gone with it.
- `Inline/Unresolved` follows the evaluation door's `PartFault`
  rendering: an ε-seam failure states the shared
  `part::EPSILON_SEAM_RECOURSE` (now one constant for both doors); a
  pin or lookup failure forwards the store's sentence, which carries
  its own recourse. The roster samples the ε-seam case.
- No `StepMapDiverged` arm exists in either enum, so there was nothing
  to give the kernel-defect ending.
