---
id: path-kernel-bug-refusals-end-in-a-tag
kind: issue
title: profile: PathError UnderdeterminedLeg and OverdeterminedJunction end in '(a kernel bug)', not the shared kernel-defect ending
status: closed
closed: 2026-10-26
opened: 2026-09-28
priority: P3
cost: E
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

**Superseded in direction (ENCL, 2026-10-20): read "Evidence: the repair is the shape D2 retires" below before acting on this section.** Ending these arms in `KERNEL_DEFECT_ENDING` is the shape the parent census retires.


Replace each tag with ". {KERNEL_DEFECT_ENDING}".

## Evidence: the repair is the shape D2 retires (ENCL, 2026-10-20)

`work/encl/a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids.md` holds that a typed refusal ending in `KERNEL_DEFECT_ENDING` is wrong
in both of `[failure]`'s classes. `UnderdeterminedLeg` and
`OverdeterminedJunction` are in the census's second pass, and are
unclassified there. They say the authoring surface rules them out. If
that holds, they are class 2 and panic (`[failure.taxonomy]` row 4),
with their tests moved to `#[should_panic]`. If an authoring path
reaches one, it is class 1 and names its lever. Either way, it does
not take the constant. `constructed-tangent-joint-refusals-are-typed-kernel-defects`,
on this slate, carries the two `ProfileError` arms that already take
the constant.

## Re-homed at PATHS's close (2026-10-21)

Moved from `work/paths/` to PROFILEPROSE (`work/profileprose/`) when PATHS closed. PATHS's live rows went to four programs opened at this close and cut along the priority seam (PROFILE, PROFILEREST, PROFILETAIL, PROFILEPROSE; `docs/doc-ledger/paths-leaves-the-tracker.md`). The item's id, content and history are unchanged.

## Closed (2026-10-26, tracker sweep)

**Superseded by `constructed-tangent-joint-refusals-are-typed-kernel-defects`**
(this slate). This row's own repair (end the arms in
`KERNEL_DEFECT_ENDING`) was retired by ENCL on 2026-10-20 (the section
above), and what remains — classify `UnderdeterminedLeg` and
`OverdeterminedJunction` under `[failure.taxonomy]`, panic if class 2,
name a lever if class 1 — is that row's "Second-pass sites", with the
raisers counted and the `map_refusal`/`FILLET_LEG_EXTENT_RECOURSE` probe
it owes. The "(a kernel bug)" tags still stand at
`crates/profile/src/path.rs:2174` and `:2179`, and go with that row's
fix.
