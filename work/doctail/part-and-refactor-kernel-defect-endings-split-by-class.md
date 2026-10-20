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

(ENCL implementer, from the census in
`work/encl/a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids.md`,
on main 5df6e6c854.)

`[failure]` (D2) splits every state in two: one input reaches, which
is a typed refusal whose ending must name the decision's lever, and one
only a kernel bug reaches, which MUST panic ("downgrading one … to a
typed error launders a bug into a supported outcome"). A typed refusal
ending in `geom_core::KERNEL_DEFECT_ENDING` is therefore wrong in either
class. The one exception is a kernel approximation limit, which takes
`[tolerance.recourse]`'s last resort (`KERNEL_LIMIT_RECOURSE` /
`KERNEL_LIMIT_LAST_RESORT`), not this ending.

## Sites

- `crates/editor-core/src/eval/parts.rs` `PartFault` `Display`,
  `PartProduct` where `product_recourse` says `KernelDefect` (~426).
  Door: evaluating an instantiate node. **Class 2, likely**, for the
  kinds `EvaluationOfAnotherDocument`, `UnknownNode`, `RootFailed` and
  `RootPoisoned`. Its `ContactLineage` kind is in D10's held territory
  (contact records): it is classified, and is not this row's.
- `parts.rs` `PartFault` `Display`, `ReferenceCycle` (~445). Door:
  evaluation with a caller's `PartResolver`, a `pub trait` that
  `EvalOptions` carries. **Class 1, likely**: the evaluator does not
  check that the document a resolver returns hashes to its pin
  (`parts.rs`, the `.resolve(doc_ref, tol)` call, ~1031). So a resolver
  that returns a document referring back closes a loop. The arm's
  premise, "a store that checks pins cannot hold a loop", holds for the
  pncad store and not for the trait. The lever is to check the pin at
  the seam and refuse as `ResolveFault::PinMismatch`, naming the
  resolver. After that, the cycle is class 2.
- `parts.rs` `PartFault` `Display`, `NotEntered` (~457). **Class 2,
  likely**: the census pass enters every part before the run reads it.
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

- **Class 2** (only a kernel bug reaches it): `unreachable!` or
  `debug_assert` per `[failure.taxonomy]` rows 4–5, answering row 0
  first (a type that cannot hold the state is preferred). The tests
  that pin the refusal's text move to `#[should_panic]`, or are deleted
  where they only built the error value by hand.
- **Class 1** (input reaches it): the decision's own lever, written in
  its own words (`[tolerance.recourse]` (i)), and a test that reaches it
  through the door.
- **Class 3** (an approximation limit): the last-resort recourse.

Re-check each `likely` or `unsure` argument before acting on it: one
that fails moves the site to class 1.
