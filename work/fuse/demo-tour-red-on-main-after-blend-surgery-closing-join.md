---
id: demo-tour-red-on-main-after-blend-surgery-closing-join
kind: issue
title: demo-tour is red on main since PR 4353: blend surgery's closing join removes a vertex and edge per fillet, and the tour's teapot and die censuses were not re-baselined
status: closed
opened: 2026-10-09
priority: P0
cost: E
closed: 2026-10-09
pr: 4374
---

Filed by the SHELL orchestrator, 2026-10-09; found on SHELL's PR 4356,
which touches nothing the failure reads. Bisected over first-parent merges
on main: 206fd7d9d4 (PR 4331) good, f54d887db8 (PR 4353, FUSE step 3 C,
"blend surgery ends with the join") the first bad.

- `demos/tour/tests/teapot_document.rs:393`
  (`one_request_builds_the_kernels_body`): census (12, 21, 11), expected
  (14, 23, 11).
- `demos/tour/src/diefillet.rs:672` (`eps_regression::tour_runs_green_at_*`):
  (89, 174, 108), expected (89, 195, 129) — 21 fewer edges and vertices,
  one per pip fillet.

`blend_surgery` (`crates/sweep/src/blend/surgery.rs`) now ends with
`join_edges_within` over the carved shells; each join kills one vertex and
one edge. PR 4353 re-baselined every other census but not `demos/tour`,
its own cargo root, whose CI job its change filter skipped.

SHELL's PR 4356 ports the re-baseline (after checking the joins are the
trimline/foot pairs). Owed here: confirm, and close once main carries it.


## Closed

2026-10-09, PR 4374 (FUSE, `fuse/tour-pins-follow-the-blend-join`): the teapot and die pins re-baselined to the closing join, the same numbers SHELL's PR 4356 ported after checking the joins (the teapot's trimline half-arcs, one per pip fillet on the die).
