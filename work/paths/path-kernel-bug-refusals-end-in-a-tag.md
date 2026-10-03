---
id: path-kernel-bug-refusals-end-in-a-tag
kind: issue
title: profile: PathError UnderdeterminedLeg and OverdeterminedJunction end in '(a kernel bug)', not the shared kernel-defect ending
status: open
opened: 2026-09-28
---


(ENCL implementer, from the sweep of
`work/encl/kernel-defect-endings-and-repair-labels-have-no-shared-home.md`.)

## What

`profile::PathError`'s `Display` (`crates/profile/src/path.rs`) ends
two arms in a "(a kernel bug)" tag:

- `UnderdeterminedLeg` (`path.rs:2026`): "…which the authoring surface
  should rule out (a kernel bug)".
- `OverdeterminedJunction` (`path.rs:2031`): the same.

No `There is no way through` marker (zero to
`test_utils::refusal::recourse_markers`) and no report. The ending has
one home now, `geom_core::KERNEL_DEFECT_ENDING`.

## Repair shape

Replace each tag with ". {KERNEL_DEFECT_ENDING}".
