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
(`crates/sweep/tests/a_ball_with_no_room_at_its_rim_refuses_typed.rs:247`,
`a_lanterns_shoulder_refuses_its_ball_where_the_offset_traces_part`).
The lantern's shoulder (`crates/sweep/src/test_support.rs:1068`
`lantern`, rim `(0.8, 0.6)`, the unit belly against the cone
`x + y = 1.4`, a sphere–cone row of the sheet reduction) refuses
`BlendError::SpineIrregular` (`crates/sweep/src/blend/mod.rs:1639`,
raised by `crates/sweep/src/blend/battery.rs:674` `spine_regularity`)
through `fillet_edges` at `r = 0.5, 0.6, 0.75` (margins about
`−0.0996`, `−0.463`, `−1.82`), and `RadiusHeadroom` at `r = 0.9`. Its
ball still has room there: the cone stands `1.4/√2 ≈ 0.98995` from the
belly's centre, so the room `(1 − r) − |0.98995 − r|` is about `0.01`
up to `r ≈ 0.98995` and runs out at `r ≈ 0.994975`, where
`BallDoesNotFit` takes over (`r = 0.996` reads `−0.00205`). It builds
at `r = 0.4`.

So `FILLET3_SPINE_RECOURSE` (`crates/sweep/src/blend/mod.rs:1171`) has
a witness at verb level: a spine that folds while the ball's room and
the faces' clearance hold. The followability suite's row
`crates/sweep/tests/blend_recourse_followability.rs:1151`
`the_spine_recourse_has_no_witness_in_this_suite_the_clearance_screen_answers_first`
pins only that its own dome ladder does not reach the sentence; the
constant's doc now names the lantern shoulder and this row.

## What it needs

A followability row on the lantern shoulder: refuse `SpineIrregular`
at `r = 0.5`, then follow "reduce the fillet radius below the spine's
own curvature radius" to a radius that builds and validates (`r = 0.4`
builds on the ladder), and re-word the no-witness row's title to say
what it pins.
