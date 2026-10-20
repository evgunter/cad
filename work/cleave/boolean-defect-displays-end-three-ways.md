---
id: boolean-defect-displays-end-three-ways
kind: issue
title: TornComponent, SeamOrientation and ZipCorrespondence end '(kernel bug)', not the defect ending, so offer_key reads them as typed refusals
status: open
opened: 2026-10-11
priority: P3
cost: E
---


## What

Found by the review of PR 4625. `topo/src/lib.rs` `test_support::offer_key`
(~572–591) reads a Boolean refusal as the kernel's own defect where its
text ends in `KERNEL_DEFECT_ENDING` or its kind is
`ClassificationInvariant` or `JoinDesync`. Three more defect variants
end their `Display` (`boolean/mod.rs`, the `BooleanError` `Display`)
with their own words instead, and no kind list names them:

- `TornComponent`: "… of both sides (kernel bug)";
- `SeamOrientation`: "… (orientation chain broke — kernel bug)";
- `ZipCorrespondence`: "seam zip correspondence failed: {what} (kernel bug)".

`JoinDesync`'s own "(kernel bug or corrupt reduction)" is the fourth
spelling, held only by its kind's place in the list. Every harness that
reads `offer_key`'s defect bit, `sweep`'s `near_tangent_pierce_poses`
among them, takes the three as typed refusals.

## Owed

**Superseded in direction (ENCL, 2026-10-20): read "Evidence: the owed repair is the shape D2 retires" below before acting on this section.** Ending these arms in `KERNEL_DEFECT_ENDING` is the shape the parent census retires.


End each defect `Display` in `KERNEL_DEFECT_ENDING` (one story, D4 ¶1
(i)), so `offer_key` reads the ending alone and its kind list goes.

## Evidence: the owed repair is the shape D2 retires (ENCL, 2026-10-20)

`work/encl/a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids.md` holds that a typed refusal ending in `KERNEL_DEFECT_ENDING` is wrong
in both of `[failure]`'s classes. If input reaches the state, the
ending's claim is false. If only a bug reaches it, the state must panic.
So ending `TornComponent`, `SeamOrientation` and `ZipCorrespondence` in
the constant converts a tag into the laundering that row is retiring.
Classify each one instead: panic if it is bug-only (its text already
says "kernel bug"), or give it its decision's lever if input reaches it.
`classification-invariant-family-types-bug-only-states-against-d9-row-4`
is the home for that classification. Once the defect variants panic,
`offer_key`'s kind list goes away of its own accord.
