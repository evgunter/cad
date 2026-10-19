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

End each defect `Display` in `KERNEL_DEFECT_ENDING` (one story, D4 ¶1
(i)), so `offer_key` reads the ending alone and its kind list goes.
