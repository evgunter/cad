---
id: a-nested-pairing-at-a-shared-vertex-refuses-shared-vertex-crossings
kind: issue
title: A vertex pair whose pairing nests in B's walk order, at a B vertex another crossing pair shares, refuses SharedVertexCrossings (102 pinch-battery lines)
status: open
opened: 2026-10-07
priority: P1
cost: M
refs: [a-vertex-two-crossing-pairs-cut-is-the-in-end-of-one-null-edge-and-the-out-end-of-another, a-six-crossing-vertex-pair-nests-its-pairing-and-refuses-pairing-mismatch]
---


## What

Found while building
`a-vertex-two-crossing-pairs-cut-is-the-in-end-of-one-null-edge-and-the-out-end-of-another`.
It is pre-existing; that build does not move these lines.

`join_pierce_runs_sweep::pinch_runs_battery` (`notch343` against a
two-cube corner pinch) refuses `SharedVertexCrossings { operand: B }`
on 102 lines: every op at 34 poses with the pinch first (`ba`). There
the notch's corner is B's vertex, and both pinch cubes' vertices pair
with it. One of those vertex pairs crosses six times and pairs nested
in B's walk order (`insert::b_runs`). `insert::reconcile_pass` refuses
any nested plan at a shared vertex up front: turning a nested run to
clear the other pair's cuts would make it hold the rest of its own
plan. Example: `i=0 j=0 k=2` (`psi = 2.2`), pinned by
`join_pierce_runs_sweep::a_nested_pairing_at_a_shared_vertex_refuses_typed`.
With the notch first, the same poses build `SOUND`.

## Candidate route (unmeasured)

`insert::hang_in_turned` now places a run that a turned run of its own
plan holds: it mints at the turned run's copy. A nested plan already
mints a held run at its holder's copy (`Held`, `b_runs`). So the
refusal might lift if the reconcile reads a nested plan's runs as one
laminar family: turn only an outermost run, and nest the rest under the
holders the two readings give. That needs a depth above one and
holders from both readings, which nothing builds today.
