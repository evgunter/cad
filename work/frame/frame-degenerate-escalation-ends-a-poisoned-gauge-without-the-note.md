---
id: frame-degenerate-escalation-ends-a-poisoned-gauge-without-the-note
kind: issue
title: geom-core frame: FrameError::Degenerate's escalation ends a poisoned margin in the lever with no unreadable-margin note
status: open
opened: 2026-10-13
---


(Filed by ENCL from the sweep of `switch-profile-and-editor-core-onto-indeterminate-lever-ending`.)

## What

`crates/geom-core/src/linalg/frame.rs`, `FrameError::Degenerate { indeterminate: Some(i), .. }` renders `i.undecided(input.subject(), lever_recourse(COINCIDENCE_RECOURSE, None))`. The `None` drops the unreadable-margin note, so a poisoned margin ends in the lever alone.

The decision is a frame input's direction length or independence, a gauge and no size the user chose. That is the lever-only-at-build ending, `Indeterminate::lever_ending`, which is byte-identical except that it adds `UNREADABLE_MARGIN_NOTE` on a poisoned margin.

## Repair shape

Switch it to `i.lever_ending(COINCIDENCE_RECOURSE)`. Pin it on a NaN margin through `k_stats::decide`, as `rigid_motion.rs`'s `an_undecided_check_ends_in_the_lever_and_notes_a_poisoned_margin` does.
