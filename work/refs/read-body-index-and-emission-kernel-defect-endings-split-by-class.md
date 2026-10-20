---
id: read-body-index-and-emission-kernel-defect-endings-split-by-class
kind: issue
title: editor-core: read_body's NoSuchBody ends in KERNEL_DEFECT_ENDING though the caller's index reaches it; the emission category is bug-only
status: open
opened: 2026-10-20
parent: a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids
priority: P3
cost: M
---

A child of the census in `work/encl/a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids.md` (as of main 5df6e6c854); why a typed `KERNEL_DEFECT_ENDING` is wrong in either D2 class is that row's "What" and "Shape of the work".

## Sites

- `crates/editor-core/src/names/interrogate.rs` `InterrogateError`
  `Display`, `NoSuchBody` (~183). Doors: `pub fn
  Evaluation::read_body(var, index)` (`eval/mod.rs` ~255), and the
  interrogation doors. **Class 1, sure**: the index is the caller's,
  and any index past the node's outputs reaches it. The lever is to
  name a body the node outputs, giving the count it has.
  `tests/display_contract.rs` pins the ending, and moves with it.
- `crates/editor-core/src/names/emit.rs` `Category::ending`, `Emission`
  (~659). This is one site covering nine variants: `Duplicate`,
  `EntityNamed`, `KindMismatch`, `ShortTie`, `Unnamed`,
  `MissingUpstream`, `Emission`, `SplitLineage`, `Settle`. Door: every
  node's name emission. **Class 2, unsure**: the type's own category 1
  is "an emission BUG", but about 151 raisers were not re-derived. Check
  each one before it panics.

## Repair shape

- **Class 2**: `unreachable!` or `debug_assert` per `[failure.taxonomy]` rows 4–5, answering row 0 first; the tests that pin the text move to `#[should_panic]`.
- **Class 1**: the decision's own lever, in its own words (`[tolerance.recourse]` (i)), with a test that reaches it through the door.

Re-check each `likely` or `unsure` argument before acting on it; one that fails moves the site to class 1.
