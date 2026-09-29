---
id: edit-refusal-roster-renders-no-program-or-step-id-arm
kind: issue
title: edit: the status-line refusal roster renders none of EditError's three program and step-id arms
status: review
pr: 3492
rides_with: edit-refusals-short-of-the-shape-guard
branch: edit/part-refusal-recourse
opened: 2026-09-29
---


(EDIT, found by the recourse unit of
`edit-refusals-short-of-the-shape-guard` while checking its roster.)

## What

`crates/viewer/tests/refusal_concision_edits.rs`
`every_edit_refusal_renders_within_the_budget` says it renders every
`EditError` arm through the viewer's `Refusal::Edit` wrapper, and
`edit_refusals` builds no row for three of them:

- `EditError::SetProgramOnNonProfile` ("node N holds no profile
  program, so it has no program to set"), raised by `DocEdit::SetProgram`
  in `crates/editor-core/src/edit.rs` `apply_maintaining`;
- `EditError::StepIdsRefused` over each `program::StepIdFault` arm
  (seven), raised by `settle_step_ids` at `SetProgram` and by
  `mint_step_ids` at `InsertNode`;
- `EditError::NameStepNeverMinted`, raised by `check_name_steps` at
  `InsertNode`, `Rebind` and the appearance doors.

So none of them is held to `test_utils::refusal::problems`, and by
reading their `Display` none states a recourse: each would red the
zero-recourse check the moment it had a row.

## Repair shape

Add the rows (one per `StepIdFault` arm, the way `Roots` and
`InvalidDistribution` are rendered over each fault), then give each arm
the recourse its raise site supports, reading the site first. A
`StepIdFault` is forwarded, so the recourse belongs in the
`StepIdsRefused` wrapper, matched on the fault, as `Roots` does.

## Ruled (2026-09-29, EDIT orchestrator) — rides with `edit-refusals-short-of-the-shape-guard`

One unit with the feature-tree half; the spec is in that row.

## Built (2026-09-29, PR 3492)

The roster renders `SetProgramOnNonProfile`, `NameStepNeverMinted`, and `StepIdsRefused` over every `StepIdFault` arm an edit door raises, through an exhaustive witness chain. `NotMinted` has no row, and the chain states why. Each states the recourse its raise site supports; the `StepIdsRefused` wrapper matches on the fault (`step_ids_recourse`). The wrapper's old `node N's program step ids:` clause was a label shape, and is reworded.
