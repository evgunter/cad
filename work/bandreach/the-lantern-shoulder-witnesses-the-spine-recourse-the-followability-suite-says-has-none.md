---
id: the-lantern-shoulder-witnesses-the-spine-recourse-the-followability-suite-says-has-none
kind: issue
title: sweep: the lantern's sphere–cone shoulder refuses SpineIrregular at r = 0.5–0.75, a witness the spine recourse's followability row says no fixture reaches
status: open
opened: 2026-11-10
priority: P3
cost: E
---


## Finding

Found while laddering every curved-rim family through its ball's room
(`crates/sweep/tests/a_ball_with_no_room_at_its_rim_refuses_typed.rs`,
`a_lanterns_rims_refuse_their_balls_where_the_offset_traces_part`).
The lantern's shoulder (`test_support::lantern`, rim `(0.8, 0.6)`, the
unit belly against the cone `x + y = 1.4`, a sphere–cone row of the
sheet reduction) refuses `BlendError::SpineIrregular` through
`fillet_edges` at `r = 0.5, 0.6, 0.75` (margins about `−0.0996`,
`−0.463`, `−1.82`), and `RadiusHeadroom` at `r = 0.9`. Its ball still
has room there (`|1 − r| − |1.4 − r√2|/√2 > 0` until about `r = 1.19`).

So `FILLET3_SPINE_RECOURSE` has a witness at verb level.
`crates/sweep/src/blend/mod.rs` `FILLET3_SPINE_RECOURSE`'s doc and
`crates/sweep/tests/blend_recourse_followability.rs`
`the_spine_recourse_has_no_witness_in_this_suite_the_clearance_screen_answers_first`
say no fixture in that suite reaches it, which stays true of that
suite's dome ladder, and ask for "a body whose spine folds while
clearance stays ample". The lantern shoulder is one.

## What it needs

A followability row on the lantern shoulder: refuse at `r = 0.5`, then
follow "reduce the fillet radius below the spine's own curvature
radius" to a radius that builds and validates (the ladder builds at
`r = 0.4`). Re-word the constant's doc and the no-witness row to match.
