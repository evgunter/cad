---
id: the-split-checks-no-postcondition-that-each-sides-cells-lie-in-its-half-space
kind: issue
title: The split checks no postcondition that each side's cells lie in its closed half-space; the sides rest on cached vertex verdicts
status: open
opened: 2026-11-23
priority: P1
cost: M
---

Filed with the parted-record build (Ev, PR 7489;
`a-record-the-split-parts-has-no-consumption-rule`), on the
orchestrator's word.

## What

The carry now consumes a record the split parts when each of its cells
reaches the cut by a vertex the split decided ON
(`crates/topo/src/boolean/records.rs` `reaches_the_cut`). That leans on
each side holding only material in its own closed half-space: a record
whose cells the split put on the wrong side, but which touch an ON
vertex, would be consumed as a meeting across the cut. Nothing checks
that postcondition.

- The sides are sorted by the reduce's cached per-vertex verdicts
  (`SplitReduction::sides`), read again by
  `crates/topo/src/splitting/finish.rs` `classify_shell` (cut path) and
  `whole_body_side` (uncut path); no cell of a finished side is measured
  against the plane afterwards.
- The side's gate re-measures nothing against the plane either:
  `validate_closed` reads topology and orientation, and
  `AtRestPolicy::finish` confirms the records it is handed and tiers 1
  to 3, none of which knows the plane.

## Direction

A postcondition on each side before its gate: every vertex the side
holds is ON or on the side's half-space by the reduce's verdict, and
every face's sample point (or every edge's midpoint) lies in the closed
half-space at the run's band, refusing typed where it does not. Pin it
with a mutant that sends a shell to the wrong side.
