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

`None` drops the unreadable-margin note, so a poisoned margin (`MarginDiag::is_invalid`) ends in the lever alone. `Indeterminate::lever_ending(lever)` is the lever-only-at-build ending. It adds `UNREADABLE_MARGIN_NOTE` on a poisoned margin and is otherwise byte-identical.

**Where the poisoned margin is reachable.** At f64 it is not:
- `decide_direction_by` refuses a non-finite `len` (`NonFiniteLength`) before `decide` runs;
- `levered` refuses a non-finite arm (`Arm(None)`) before deciding it;
- `Margin::levered(len, arm)` of two finite positive lengths overflows at worst to `+inf`, which `decide` reads as definite, never as poison.

At an enclosure (`Interval`), `Decide::sign_within` reads any margin that is not `is_certified` as `MarginDiag::INVALID`. `is_finite_length` asks only whether `x - x` is poison. So a component or arm whose enclosure carries a non-certified decoration from upstream arithmetic (for example a partial domain clamp) looks reachable as a poisoned margin into both arms. That is an inference from the code; no test reaches it yet. Where it is reached, the cause is the kernel's arithmetic, so the note is true there.

## Repair shape

- `UnitVec3Error::Escalated` decides a direction's length, a gauge. The same decision ends through `lever_ending` at sweep's axis door and at editor-core's `DirectionNorm`. Switch it to `source.lever_ending(COINCIDENCE_RECOURSE)`.
- `LeveredUnitError::Arm` decides the caller's arm, which is a length. If that length is one the user chose, it is sized and wants `source.ending(..)`. If not, it wants `lever_ending`. Decide which before switching it.
- Pin both arms on a poisoned margin: through the door at an enclosure if one is constructible, else through `k_stats::decide`, as `rigid_motion.rs`'s `an_undecided_check_ends_in_the_lever_and_notes_a_poisoned_margin` does.

## Note (2026-10-26, easy-fixes lane)

The first bullet is done: `UnitVec3Error::Escalated` now ends in `source.lever_ending(COINCIDENCE_RECOURSE)`. It is pinned by `unit_vec::tests::an_escalated_length_ends_in_the_lever_and_notes_a_poisoned_margin` through `k_stats::decide`. `LeveredUnitError::Arm(Some(_))` is unchanged. It still needs the sized-or-gauge decision the second bullet asks for: its callers lever by a cylinder radius the user chose (`topo::boolean::boxes`, `BOX_CYLINDER_AXIS`), by a reach off the model's box (`topo::boolean::join`, `reach.lever_from(origin)`; `separating.rs`), and by a torus reach (`geom_brep::props::curved`). That is not one answer.
