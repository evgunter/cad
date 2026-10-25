---
id: demos-row-skips-kernel-changes-to-doors-the-tour-calls
kind: issue
title: The PR gate runs the demos row only when demos/ changes, so a kernel change to a door the tour calls reaches the tour only nightly
status: open
opened: 2026-10-02
priority: P2
cost: E
---


## What

`.github/workflows/ci.yml`'s change filter keys the demos row on
`flag demos "touched '^demos/'"` (the classify step, near line 112),
so the PR gate runs `demos/tour` and `demos/wild` only when the diff
touches `demos/`. A kernel change to a door the tour calls is gated
only by the nightly. Instance: PR 3773 rewrote `topo::query::rim_of`,
which `demos/tour/src/bodies.rs` calls (the bud's mouth rim), and
`RimError`'s arms; its gate skipped the demos row, and its reviewer
ran the tour suite by hand (86/86).

The implementer discipline already asks a lane changing a public
signature to run the two demos' clippy by hand; nothing asks for
their tests, and a behaviour change with no signature change gets no
demos run before merge at all.

## The shape to give

Seed the demos row from the crates the demos depend on (the way the
`topo` row is `seeded topo`), or at least from the crates whose public
doors the tour calls; or state in the discipline that a kernel door
change runs the tour suite before push. Weigh against the row's cost.

## Evidence: the same gap for a tree-wide reader (2026-10-02)

PR 3821 (`51f0b1f29`, `crates/viewer/src/idpass.rs` `NameAndPath`) gated `PKGS=viewer`, so `pncad-py`'s `prose_census` (which reads every crate's source and depends on none of them) never ran; main went red on `every_site_this_census_cannot_decide_is_named_with_its_reason` and the next PR whose closure held `pncad-py` (3803) heard it. That is the cost `work/ciw/latency-cut.md` accepted when it dropped the read reach ("the other seven a PR hears about when it touches their crate, or from the nightly"); main's push runs only prime the cache, so main reads green meanwhile. Fixed by `tquery/prose-census-idpass`.

## Evidence: the mirror direction, a tour stop arriving (2026-10-02)

PR 3787 (`fd031b83c3`) added the `snowman` stop. Its diff was `demos/`,
`docs/tess-budget-data/`, `tools/` and `work/` only, so
`scripts/ci-filter.py` classified it `TIER=aux` with empty `PKGS`, and
`crates/pncad/tests/all.rs::the_north_star_audit_has_a_row_for_every_tour_stop`
(which reads `demos/tour/src/` at run time) did not run. Main went red
on it. This guard can only be tripped by a demos diff, and a demos diff
never puts `pncad` in the closure, so as filtered today it can never
fire on the PR that trips it. The fix may differ from the kernel→demos
direction: the guard could run on the demos row, or `demos/tour/src/**`
could select `pncad`'s `test(north_star)` pair. Fixed by
`show/snowman-audit-row`.

## Main went red: #5499 → #5519, and the profile half (2026-10-21)

Raised to P2: the gap this item describes broke `main`. It has a second
half the filter alone does not close. Even when the `demos` row runs, its
`ci` profile leaves out the rows that walk the tour.

### The instance

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

### What would close it

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
