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

## Evidence: the repair is the shape D2 retires (ENCL, 2026-10-20)

`work/encl/a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids.md` holds that a typed refusal ending in `KERNEL_DEFECT_ENDING` is wrong
in both of `[failure]`'s classes. `UnderdeterminedLeg` and
`OverdeterminedJunction` say the authoring surface rules them out,
which makes them class 2: they panic (`[failure.taxonomy]` row 4),
with their tests moved to `#[should_panic]`, rather than taking the
constant. If an authoring path does reach one, it is class 1 and names
its lever. `constructed-tangent-joint-refusals-are-typed-kernel-defects`,
on this slate, carries the two `ProfileError` arms that already take
the constant.
