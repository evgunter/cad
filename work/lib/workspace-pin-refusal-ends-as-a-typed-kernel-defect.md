---
id: workspace-pin-refusal-ends-as-a-typed-kernel-defect
kind: issue
title: pncad: the workspace's Pin refusal ends in KERNEL_DEFECT_ENDING for a serializer defect only a kernel bug reaches
status: open
opened: 2026-10-20
parent: a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids
priority: P3
cost: E
---

A child of the census in `work/encl/a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids.md` (as of main 5df6e6c854); why a typed `KERNEL_DEFECT_ENDING` is wrong in either D2 class is that row's "What" and "Shape of the work".

## Sites

- `crates/pncad/src/workspace.rs` `resolve_recourse`,
  `WorkspaceError::Pin` (~717). Doors: the workspace's open scan and
  its per-resolution scan. **Class 2, likely**: per the arm's comment,
  computing a pin over a document that has just loaded fails only on a
  serializer defect, because the load already validated the document
  the file gave.

## Repair shape

- **Class 2**: `unreachable!` or `debug_assert` per `[failure.taxonomy]` rows 4–5, answering row 0 first; the tests that pin the text move to `#[should_panic]`.

Re-check each `likely` or `unsure` argument before acting on it; one that fails moves the site to class 1.
