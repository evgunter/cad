---
id: run-side-arc-rule-reads-only-the-run-at-each-end
kind: issue
title: The run-side arc rule refuses at a reflex run end, where a reading of the face's whole sector would answer
status: review
opened: 2026-10-02
priority: P3
cost: M
pr: 3985
branch: reach/arc-from-pairing
---


## What

Found by the `reach/tilted-sphere-pair` lane, which wrote the rule.
`chord_join::select_arc_by_run_side` decides, at each run end, which
candidate arc leaves on the run's left (`split_arc_run_side`) and
requires both ends to agree. Leaving on the run's left is exact where
the run end is a smooth point or a convex corner of the divided face —
the face's sector there is at most a half-turn and the two candidates
leave in opposite directions. At a REFLEX corner the face's corner
between run and chord can exceed a half-turn, and the wrong candidate
can leave on the run's left; two reflex ends misleading the same way
would select the wrong arc.

The rule therefore gates each end it reads (review fix pass): the turn
from the boundary arriving at the end to the boundary leaving it must
be a left turn or none (`run_corner_opens`: `split_arc_run_corner`,
`split_arc_run_cusp`), and a reflex corner or cusp refuses typed,
`ArcSideCase::ReflexRunEnd`. No measured pose reaches it: every chord
end the shipped poses produce is a pierce in an edge's interior.

## What a fix owes

A reading that answers at a reflex end instead of refusing: the
candidate inside the face's whole sector there (the arriving and
leaving boundary tangents bound it), with a fixture whose pierce lands
on a reflex corner of a sphere face (a face notched by a prior cut,
pierced at the notch's corner).

## Retired (PR 3985, `reach/arc-from-pairing`)

The rule is gone; the chord takes the germs' arc and reads no run, so no
corner gate exists. The reflex-notch pierce this item asked for is
`crates/sweep/tests/snowman.rs`
`a_bar_through_a_ball_builds_under_every_boolean`: the bar's corner left
a reflex run end (`ReflexRunEnd`, every op), and every op now builds to
its slice-integral volume.
