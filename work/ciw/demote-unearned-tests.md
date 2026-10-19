---
id: demote-unearned-tests
kind: unit
title: Demote the per-PR tests that cost more than a fraction of a second per run and never caught anything
status: open
opened: 2026-10-19
---


The question (Ev, 2026-10-19): CI runs are queueing; where does the time go,
and which tests are not earning their place in the per-PR gate? Ev's bar: a
test that has never gone red after the PR that introduced it, and costs more
than a fraction of a second per PR run (averaged over every PR run, not only
the ones it ran in), does not belong in the per-PR gate.

## Where the time goes

The window is 630 `pull_request` runs of ci.yml, 2026-10-16 02:04Z to
10-19 15:57Z. Job and step times come from `/actions/runs/<id>/jobs`.
Per-test times are the nextest PASS/FAIL lines of 572 `test` and `demos` job
logs.

- **Wall clock.** The median is 21.7 min and the p90 is 36.6 min. The alarm
  in `nightly.yml` is at a median of 20.
- **Queueing.** Over the window, a job waited at most 6 min to start, and at
  the p99 it waited 16 s. On 10-19 from about 15:00Z, 20 runs sat queued
  behind a full 20-slot pool, which is the report that started this. The
  queue comes from job-minutes, not from any one job.
- **Job-minutes.** The mean is 30 per PR run. `test` takes 8,931 min over
  the window (mean 18.9 per run it ran in). Next come `demos` at 2,542,
  `lint` at 2,387, `python` at 1,656, `mesh-budget` at 1,211 and
  `topo-release` at 1,103. Superseded (cancelled) runs burned another
  3,358 min.
- **Inside `test`.** `build` has a median of 13.3 min, which is the floor
  (`the-test-jobs-build-time-candidates-measured`). Next is the seeded
  slow-set step: median 6.0, p90 14.2. The default-ε step is 1.1, the extra
  ε rows 0.5 (p90 2.2) and doc-tests 0.1.
- **Test cost per PR run.** Test-seconds summed over the window and divided
  by all 630 runs come to 1,546 s per run:

  | step | s per run |
  |---|---|
  | seeded slow set | 981 |
  | tour suite | 220 |
  | default ε (all ~14k fast-set tests) | 188 |
  | extra ε | 156 |

  615 tests cost ≥ 0.25 s per run and hold 1,249 s of the 1,546.

## The bar, as applied

- **Threshold.** A test is a candidate at ≥ 0.25 s per PR run, i.e. its
  window total divided by 630.
- **What counts as a catch.**
  - A red in a `pull_request` run of ci.yml. Nightly reds do not count:
    the nightly keeps running every demoted test, so a nightly catch is
    the nightly doing its job.
  - The test must already have existed on main when the run was created
    (`git grep 'fn <name>'` at main's first-parent commit before the run).
    That excludes the introducing PR's own iterations.
  - No test the PR gate keeps may have gone red in the same job.
    Candidates are decided cheapest first, so of two tests that always go
    red together, only the cheaper one stays.
- **Red history.** 2,263 failed test, suite and demos job logs from
  2026-07 to 10-19, out of 2,733 failed jobs. The rest had expired or
  could not be fetched. 925 distinct test names went red.

Result:

| | tests | s per PR run |
|---|---|---|
| candidates | 615 | 1,249 |
| keep per-PR (an unshared PR red) | 45 | 205 |
| demote to nightly-only | 570 | 1,044 |

Of the 570:
- 535 were already in the slow set; they are now in
  `.config/nightly-only.txt`.
- 26 were fast-set tests. Six of them had been deleted or renamed on main
  by 10-19, so 20 are now in the slow set and nightly-only.
- 9 are demo-tour rows, in `demos/tour/.config/nextest.toml`'s `ci`
  profile. These include `tour_runs_green_at_default_eps` and `_at_eps_1e_6`.
  `_at_eps_1e_12` stays, because every one of their PR reds was also its
  red.

What stays per-PR is the 45 with an unshared PR red. The biggest are
`tour_runs_green_at_eps_1e_12`, `certified_cells_run_green_at_default_eps`,
the gallery denotation row, three `m10_9_pins_interval` rows, `sym11`'s
exact-channel row, `mass_props_are_thread_count_invariant`'s serial golden
and `lily`'s finding-13 table.

## Expected effect

These numbers replay the window's logs with the demoted tests removed. A
step's wall time is estimated as max(longest test, cpu-s / 4); for the slow
set this reproduces the measured 6.0-min median as 5.8.

| step | median (min) | p90 (min) |
|---|---|---|
| seeded slow set | 5.8 → 1.3 | 14.2 → 2.4 |
| tour suite | 7.7 → 5.6 | 9.0 → 6.3 |
| extra ε | 0.5 → 0.4 | 2.8 → 1.9 |

`test` is the critical job in most code runs, so the run median should fall
by about 4.5 min, from 21.7 to about 17.

## Blind spots

- **Times come from 3.5 days.** A test added since then, or one in a crate
  rarely seeded in the window, is under-counted.
- **The red-history match is by test name.** It is not by binary, and a
  rename splits a test's history.
- **The "existed on main" check greps the last path segment.** A helper
  with the same name could make an introducing PR's red count as a catch.
  That errs toward keeping the test.
- **Not measured here.** The python suite's per-test cost (8 min a run it
  rides in; unittest prints no per-test times), and the always-on
  `mesh-budget` and `topo-release` rows. Both of those went red on 40+
  branches, so they were not touched.
