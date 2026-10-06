---
id: demos-red-on-main-klein-pin-retired-and-certified-cells-moved
kind: issue
title: demo-tour eps_regression is red on main: klein findings entry 10 retired and the certified-cells header moved
status: open
opened: 2026-10-06
priority: P0
cost: E
---


Filed by the SHELL orchestrator, 2026-10-06. Found on PR 4151, which touches nothing under `demos/`.

`demos (tour + wild)` fails four `demo-tour::eps_regression` rows on
origin/main at `364b8aefa`: `tour_runs_green_at_eps_1e_12`,
`tour_runs_green_at_default_eps`, `tour_runs_green_at_eps_1e_6` and
`certified_cells_run_green_at_default_eps`. A release build of
`demo-tour` on a clean main worktree reproduces both panics. The
same rows were green on main at `78bee3ac`, which PR 4151's run
37471881878 carried.

1. **The klein scene's findings pin.** It panics at
   `demos/tour/src/klein.rs:876`: "the two outer-wall cylinders now
   carry the SAME radius ... findings entry 10 has retired — delete it
   and this pin". The pin names the cause it expects: "the revolve
   stopped reconstructing a wall radius from swept endpoints". That
   is what PR #3774 (`store-constructed-carriers`: constructions store
   the carriers they build; outputs move by ulps) did.
   **Fix:** retire findings entry 10 and its pin, as the message says,
   and update the scene's findings prose.
2. **The certified-cells header.** It panics at
   `demos/tour/src/tolerance.rs:393`: the leaf counts are now
   (211, 301) against the header's (193, 319) at 512 leaves.
   **Fix:** re-measure and re-baseline the header, and say in the PR
   what moved and why.

Suspects, between `78bee3ac` and `364b8aefa`:
- #3774 (PATHS 5b), most likely for both rows;
- #4136 (BAND, a lamina full revolve's plane annulus is one face).

The CI `test` job on main is fast and path-scoped, so main's own runs
do not run the demos job and stayed green. Every PR that does run
it inherits this red until it is fixed.
