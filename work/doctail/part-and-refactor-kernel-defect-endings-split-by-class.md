---
id: part-and-refactor-kernel-defect-endings-split-by-class
kind: issue
title: editor-core: six part, refactor and edit refusals end in KERNEL_DEFECT_ENDING; a caller's PartResolver reaches ReferenceCycle, the rest are bug-only
status: open
opened: 2026-10-20
parent: a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids
priority: P3
cost: M
---

A child of the census in `work/encl/a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids.md` (as of main 5df6e6c854); why a typed `KERNEL_DEFECT_ENDING` is wrong in either D2 class is that row's "What" and "Shape of the work".

## Sites

- `crates/editor-core/src/eval/parts.rs` `PartFault` `Display`,
  `PartProduct` where `product_recourse` says `KernelDefect` (~426).
  Door: evaluating an instantiate node. **Class 2, likely**, for the
  kinds `EvaluationOfAnotherDocument`, `UnknownNode`, `RootFailed` and
  `RootPoisoned`. Its `ContactLineage` kind is in D10's held territory
  (contact records): it is classified, and is not this row's.
- `parts.rs` `PartFault` `Display`, `ReferenceCycle` (~445). Door:
  evaluation with a caller's `PartResolver`, a `pub trait` that
  `EvalOptions` carries. **Class 1, sure** (editor-core `tests/part_depth_bound.rs` ~266
  builds such a resolver, `Unpinned`): the evaluator does not
  check that the document a resolver returns hashes to its pin
  (`parts.rs`, the `.resolve(doc_ref, tol)` call, ~1031). So a resolver
  that returns a document referring back closes a loop. The arm's
  premise, "a store that checks pins cannot hold a loop", holds for the
  pncad store and not for the trait. The lever is to check the pin at
  the seam and refuse as `ResolveFault::PinMismatch`, naming the
  resolver. After that, the cycle is class 2.
- `parts.rs` `PartFault` `Display`, `NotEntered` (~457). **Class 2,
  unsure**: the census pass enters every part before the run reads it.
  But the `enter_census` doc (~632) says a cancel leaves rows that
  refuse `NotEntered`, so a conversion must first prove that nothing
  reads them after a cancel.
- `crates/editor-core/src/refactor.rs` `RefactorError` `Display`, `Pin`
  (~1203). Door: the split-part refactor. **Class 2, likely**: the part
  replayed cleanly through the edit doors, and the save validator
  agrees.
- `refactor.rs` `ReplayTail` `Display`, the `defect` closure (~1746: one
  site, used by the arms listed under it). Doors: the split-part and
  inline refactors. **Class 2, likely**: each replayed edit re-writes
  what a validated document already holds.
- `crates/editor-core/src/edit.rs` `step_ids_recourse`, `NotMinted`
  (~3254). Door: `EditError::StepIdsRefused`. **Class 2, likely**: no
  edit door raises it. `InsertNode` raises `Preminted` and `SetProgram`
  the shape and keep arms, and the load door renders
  `SnapshotError::StepIds` itself. Row 0 applies: the edit error's
  fault type can leave `NotMinted` out.

Two more sites on this program's ground are HELD (parameters):
`edit.rs` `InvalidDistribution { NonFinite }` (~4186) and `range.rs`
`Derivation` (~542). Both are class 2, likely. They are not this row's.

## Repair shape

- **Class 2**: `unreachable!` or `debug_assert` per `[failure.taxonomy]` rows 4–5, answering row 0 first; the tests that pin the text move to `#[should_panic]`.
- **Class 1**: the decision's own lever, in its own words (`[tolerance.recourse]` (i)), with a test that reaches it through the door.

Re-check each `likely` or `unsure` argument before acting on it; one that fails moves the site to class 1.
