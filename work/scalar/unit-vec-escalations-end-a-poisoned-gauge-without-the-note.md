---
id: unit-vec-escalations-end-a-poisoned-gauge-without-the-note
kind: issue
title: geom-core unit_vec: UnitVec3Error::Escalated and LeveredUnitError::Arm end a poisoned margin in the lever with no unreadable-margin note
status: open
opened: 2026-10-13
---


(Filed by ENCL from the sweep of `switch-profile-and-editor-core-onto-indeterminate-lever-ending`.)

## What

Two `Display` arms in `crates/geom-core/src/linalg/unit_vec.rs` render an `Indeterminate` through `undecided` and end it in `lever_recourse(COINCIDENCE_RECOURSE, None)`:
- `UnitVec3Error::Escalated` (subject `DIRECTION_LENGTH_SUBJECT`);
- `LeveredUnitError::Arm(Some(_))` ("the arm a direction's length is levered by").

`None` drops the unreadable-margin note, so a poisoned margin (`MarginDiag::is_invalid`) ends in the lever alone. `Indeterminate::lever_ending(lever)` is the lever-only-at-build ending, which adds `UNREADABLE_MARGIN_NOTE` on a poisoned margin and is otherwise byte-identical.

## Repair shape

- `UnitVec3Error::Escalated` decides a direction's length, a gauge. The same decision ends through `lever_ending` at sweep's axis door and at editor-core's `DirectionNorm`. Switch it to `source.lever_ending(COINCIDENCE_RECOURSE)`.
- `LeveredUnitError::Arm` decides the caller's arm, which is a length. If that length is one the user chose, it is sized and wants `source.ending(..)`. If not, it wants `lever_ending`. Decide which before switching it.
- Pin both arms on a NaN margin through `k_stats::decide`, as `rigid_motion.rs`'s `an_undecided_check_ends_in_the_lever_and_notes_a_poisoned_margin` does.
