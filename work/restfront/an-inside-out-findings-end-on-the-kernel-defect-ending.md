---
id: an-inside-out-findings-end-on-the-kernel-defect-ending
kind: issue
title: NegativeVolume and ShellWinding end on the kernel-or-file defect ending, though an operand refused for them has an operand recourse
status: open
opened: 2026-10-11
priority: P3
cost: E
---



Found by the review of CLEAVE's PR 4725 (`cleave/missed-ray-side`).

## What

`BooleanError::InsideOutOperand` and `SplitReduceError::InsideOutOperand`
render as "the … operand is inside-out … Recourse: build it with its
faces pointing outward, or revert it"
(`validate.rs` `Unfinished::INSIDE_OUT_REFUSAL`). The findings they
carry render on their own as tier-3 text:
- `ValidationError::NegativeVolume`: "a solid encloses negative volume,
  so it is inside-out. {DEFECT}";
- `ValidationError::ShellWinding`: "… count as material {bounded}
  times, not 0 or 1. {DEFECT}".

`DEFECT` is `geom_core::KERNEL_OR_FILE_DEFECT_ENDING`, which says the
kernel or a damaged file is at fault. On an operand the caller built
inside-out, that contradicts the operand recourse it sits under. The
same findings on a Boolean result (`ResultInvalid`) are the kernel's,
so the ending is right there.

PR 4725 gave its own new finding, `ValidationError::SliverInsideOut`,
the operand recourse. It is raised only on operands.

## The question

Should the ending follow the subject the finding is read on, with an
operand's findings ending on its recourse and a result's on `DEFECT`?
That would mean a finding text the refusal composes rather than one
fixed per variant. `classify_point_in_solid` and
`census::Undecided::what` already choose an ending by reading, which
is a precedent.
