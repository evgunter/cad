---
id: a-fillets-run-out-rides-by-segment-kind-only
kind: issue
title: A fillet's pending run out is claimed by the next emission of the same segment kind; that it lies on the arrival carrier is guaranteed only by the chain lattice, not checked
status: closed
opened: 2026-09-25
priority: P3
cost: E
branch: emit/runout-carrier
closed: 2026-09-29
---


## The finding

This comes from the re-review of PR 3223 (S1). `PendingRunOut::rides`
(`crates/profile/src/path.rs`) compares only segment kind: a straight
emission rides a ray run out, and an arc rides a circle run out. That
is correct today only because the chain lattice refuses any straight
emission from a directed tip that leaves the arrival ray. Every such
continuation refuses as "not a legal chain-lattice walk".

`common::pinned`'s `assert_runs_ride_their_carriers` has the same
limit: it checks straight against arc, not collinearity or the same
circle.

If the lattice ever admitted `turn` or `line_to` after a directed `at`,
the fillet would silently claim a segment on another carrier. That is
PR 3223's B1 alias again.

## Fix direction

Make `rides` geometric: test that the emission lies on the arrival
carrier (collinear with the ray, or the same circle), and refuse
otherwise. Tighten the corpus pin the same way.

## Closed — PR 3266

`PendingRunOut::rides` now decides geometrically, under
`path_run_out_carrier`:

- **Ray run out:** it must end on the arrival ray's line and advance
  from the head.
- **Circle run out:** it must leave the head in the arrival's sense.
  The largest radial miss of its end, apex and two quarter points
  from the arrival circle must be zero. Those are point deviations,
  so they stay well-conditioned at any chord and any sweep.
- **Undecidable margin:** it escalates.

The corpus pin `assert_runs_ride_their_carriers` makes the same
measurement. Unit rows cover these cases:

- short arcs (chord 1e-7 and 1e-8);
- a backward arc;
- a major arc off the circle;
- a backward straight segment.

The end-to-end row is `arc_fillet::a_short_closing_run_out_on_the_arrival_circle_is_named_the_run_out`.
A conditioning defect found on the way is filed as
`work/paths/a-short-run-outs-stored-chord-reads-a-declared-fillet-joint-transversal.md`.
