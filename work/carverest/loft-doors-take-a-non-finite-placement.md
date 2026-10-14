---
id: loft-doors-take-a-non-finite-placement
kind: issue
title: the loft doors take a non-finite placement: an infinite z refuses ReversedStacking (reorder the sections) and a NaN escalates
status: open
opened: 2026-10-06
priority: P3
cost: M
---


Found by PR 4186's review (2026-10-06).

Nothing in `crates/sweep/src/skin.rs` `validate_loft` or the profile
door refuses a non-finite placement, so with the stacking fold run
first (`crates/sweep/src/loft.rs` `build`) a placement with an
infinite z component reaches `stacking_fold` and refuses
`LoftError::ReversedStacking`, whose recourse is "reorder the
sections": false, since no order of the sections fixes it. A NaN
component escalates as `StackingEscalated`, a "too close to call" about
a value that is not a number. A non-finite placement is an input
fault, and it wants its own refusal at the door, before any decision
reads it (the same holds for `loft_geometry`, `loft_parameters` and
`sweep_geometry`, which take placements too).
