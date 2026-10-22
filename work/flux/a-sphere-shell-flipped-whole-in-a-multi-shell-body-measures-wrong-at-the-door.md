---
id: a-sphere-shell-flipped-whole-in-a-multi-shell-body-measures-wrong-at-the-door
kind: issue
title: Flipping a whole sphere shell of a body with several shells measures a positive wrong volume at the door, with no refusal
status: closed
opened: 2026-10-21
priority: P2
cost: M
refs: [mass-properties-measures-a-flipped-seam-region-without-tier-3]
closed: 2026-10-22
pr: 5590
---


## Measured

Found by PR 5556's review (`review/5556-01D2cmWe`,
`crates/sweep/tests/review_5556_probes.rs`, `review_5556_flip_parity`
and `review_5556_shell_door`). It is pre-existing on the merge base.

A body of more than one shell, such as a ball less a smaller ball inside
it (`unit a-b small_in`), has a sphere shell whose faces close a whole
sphere. Flip every face of that one shell:
- tier 3 refuses the body (its shell winding);
- `topo::mass_properties` measures a POSITIVE volume, wrong, with no
  refusal.

The review counts 28 such flips per ε over its corpus. A whole-sphere
seam region reads its census once under either common bit, so the seam
pass (`crates/topo/src/props.rs`, `seam_refusals`) says nothing of it.
The flipped shell's volume changes sign, and the body's total stays
positive.

`crates/sweep/tests/sphere_face_flips_are_refused.rs`'s parity reads
"tier 3 refuses ⇒ the door refuses or measures ≤ 0", which holds for a
body of one shell only, and says so.

## What closes it

A door reading of each shell's own orientation, where a shell's volume
sign contradicts its role, as check 7 and check 10 read them. Or the
door typed `&AtRestBody` (Ev's PR 4822 decision), so that no unvalidated
body reaches it.

## Closed

By F1 (Ev's PR 4822 decision): `topo::mass_properties` takes
`&AtRestBody`, and a body with a flipped shell does not finish, so the
typed door never reads one. The witness is kept as a raw-door row,
`a_whole_flipped_shell_of_two_does_not_finish_and_the_raw_door_measures_it_wrong`
(`crates/sweep/tests/the_door_reads_seam_regions_as_tier_3_does.rs`).
That row flips each shell of the hollow ball in turn and asserts:
- neither flip passes `AtRestBody::validate`;
- `test_support::measure_unvalidated` still measures the inner shell's
  flip at the solid ball's volume.
