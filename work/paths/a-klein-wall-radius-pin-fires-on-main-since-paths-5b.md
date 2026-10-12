---
id: a-klein-wall-radius-pin-fires-on-main-since-paths-5b
kind: issue
title: demo-tour's Klein findings-entry-10 pin fires on main since PR 3774: every eps_regression row is red
status: closed
closed: 2026-10-06
opened: 2026-10-06
priority: P0
cost: E
refs: [4083]
---


## The finding

The four `demo-tour::eps_regression` rows panic on bare `origin/main`:
`tour_runs_green_at_default_eps`, `…_at_eps_1e_6`, `…_at_eps_1e_12`
and `certified_cells_run_green_at_default_eps`. Each fails at
`demos/tour/src/klein.rs:876`:

> the two outer-wall cylinders now carry the SAME radius. If the PATHS
> lattice grew a tangent-straight-leg-to-an-anchor (`.tangent().to(p)`),
> or the revolve stopped reconstructing a wall radius from swept
> endpoints, findings entry 10 has retired — delete it and this pin.

The FUSE 3953 lane bisected it and the orchestrator recorded the result:
- `tour_runs_green_at_default_eps` passes at `33e5000fb^1` and fails at
  `33e5000fb`.
- `33e5000fb` is the merge of PR 3774 (PATHS 5b, "constructions store
  the carriers they build"). Its "authored radii stored as their
  magnitude" fits the pin's second arm: the wall radius is no longer
  reconstructed from swept endpoints.

It showed first on PR CI (PR 3953, run 37482995087, job 112336079637).
Every PR that merges main is red on it.

## What is owed

The pin's text says what to do: findings entry 10 has retired, so
delete the entry and the pin. That belongs to the moving PR's owner,
with what moved and why (the demos/tour pin convention,
`work/tess/program.md` keep_out). The tour scenes are SHELL's by
courtesy (`work/suite/program.md`), so tell SHELL.

Before deleting, confirm that both radii now equal the authored
`R + WALL/2`, not two equal wrong values.

## Closed (2026-10-06)

Fixed on main by 65b1b0a8 ("demos/tour: port the klein tripwire's
retirement", SHELL, PR 4168). Both outer walls now carry the authored
radius. Findings entry 10 and its pin are retired, and the
two-hole-plate tolerance study that PR 3774 moved is re-baselined
there. That commit also records that PR 3774's CI skipped demos.
