---
id: frame-degenerate-escalation-ends-a-poisoned-gauge-without-the-note
kind: issue
title: geom-core frame: FrameError::Degenerate's escalation ends a poisoned margin in the lever with no unreadable-margin note
status: closed
opened: 2026-10-13
priority: P3
cost: E
closed: 2026-10-26
---


(Filed by ENCL from the sweep of `switch-profile-and-editor-core-onto-indeterminate-lever-ending`.)

## What

`crates/geom-core/src/linalg/frame.rs`, `FrameError::Degenerate { indeterminate: Some(i), .. }` renders `i.undecided(input.subject(), lever_recourse(COINCIDENCE_RECOURSE, None))`. The `None` drops the unreadable-margin note, so a poisoned margin ends in the lever alone.

The decision is a frame input's direction length, a gauge and no size the user chose. That calls for the lever-only-at-build ending, `Indeterminate::lever_ending`. It is byte-identical except that it adds `UNREADABLE_MARGIN_NOTE` on a poisoned margin.

**Where the poisoned margin is reachable.** The arm's `Indeterminate` arrives from `UnitVec3Error::Escalated` through `frame.rs`'s error mapping and the reference ladder. At f64 it is never poisoned: `decide_direction_by` refuses a non-finite length before `decide`. At an enclosure, a non-certified decoration on an input component looks reachable as a poisoned margin, which would be the kernel's arithmetic. That is an inference from the code; no test reaches it yet. See `work/scalar/unit-vec-escalations-end-a-poisoned-gauge-without-the-note.md`, which this row shares its cause with.

## Repair shape

Switch it to `i.lever_ending(COINCIDENCE_RECOURSE)`. Pin it on a poisoned margin, at an enclosure if one is constructible, else through `k_stats::decide`, as `rigid_motion.rs`'s `an_undecided_check_ends_in_the_lever_and_notes_a_poisoned_margin` does.

## Closed (2026-10-26, easy-fixes lane)

`crates/geom-core/src/linalg/frame.rs` `FrameError`'s `Display`, the `Degenerate { indeterminate: Some(i), .. }` arm, now ends in `i.lever_ending(COINCIDENCE_RECOURSE)`. Pinned by `frame::tests::a_degenerate_escalation_ends_in_the_lever_and_notes_a_poisoned_margin`, through `k_stats::decide` as the row allows: an in-band margin ends in the lever alone, and a NaN margin ends in the lever followed by `UNREADABLE_MARGIN_NOTE`. No enclosure fixture was built. Verified: `cargo nextest run -p geom-core --lib -E 'test(/poisoned_margin|frame::|unit_vec::/)'`, 27 passed.
