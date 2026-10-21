---
id: kernel-prs-never-walk-the-tours-wall-probes
kind: issue
title: A kernel PR that retires a tour wall goes red first on main: no per-PR row walks the tour
status: open
priority: P2
opened: 2026-10-21
---


## Finding

#5499 (`mesh`: the trimmed lane cuts along a lofted circle's C0 crease)
retired the Klein bottle's wall 8, which pinned that refusal
(`demos/tour/src/klein.rs`, the `walls::wall` call for "tessellate the loop
lofted with `circle` sections"). `walls::wall` panics on a wall that no
longer refuses, by design. #5499 went green and merged, and `main`'s tour
panicked from d1e5e7f3 until #5519 retired the probe.

No per-PR row could have caught it:

- `.github/workflows/ci.yml`'s `demos` job runs only when the diff touches
  `demos/` (`scripts/ci-filter.py`, the aux prefixes; that script is
  tcost's, the workflows are ciw's). #5499 touched
  `crates/mesh/` only.
- Even when it runs, its `ci` profile (`demos/tour/.config/nextest.toml`)
  leaves out `eps_regression::tour_runs_green_at_default_eps` and
  `..._at_eps_1e_6`, the two rows that walk the tour and with it every
  scene's wall probes.

- The tour BINARY runs per-PR only in `.github/workflows/render.yml`
  (`cd demos/tour && cargo run --release -- ../out`), which a PR reaches
  only when its head commit's subject carries `[render]` (ci.yml's `render
  tag` step). None of #5499's five commit subjects did, so no render ran.

So the wall probes, whose purpose (`demos/tour/src/walls.rs` module docs)
is to go red the moment a kernel change moves a frontier, are read only by
the nightly, after the change that moved the frontier has merged.

## What would close it

**Cost, measured.** `eps_regression::tour_runs_green_at_default_eps` takes
115 s at release (local, on top of the `demos` job's own release build). A
walls-only row should be a fraction of that, since most of the walk is
scene builds and exports.

The kernel PR that retires a wall is the one that should go red, because
retiring the probe belongs in that PR. Two shapes, for the lane to weigh
against the per-PR budget:

1. A per-PR row that runs the walls only: a `demo-tour walls` mode (or a
   test) that calls each scene's `wall_probes` without the scene builds,
   selected whenever the diff's closure reaches a crate the tour depends
   on.
2. Keep `tour_runs_green_at_default_eps` out of the `ci` profile, but run
   it per-PR when the closure reaches `mesh`, `sweep` or the other tour
   dependencies the walls exercise.

The teapot already has a suite row for its walls
(`teapot::wall_probes_run_here::every_teapot_wall_is_attempted_by_the_suite`),
and the `ci` profile excludes it too. The Klein bottle has none.
