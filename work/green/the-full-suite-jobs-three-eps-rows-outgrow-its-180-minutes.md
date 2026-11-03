---
id: the-full-suite-jobs-three-eps-rows-outgrow-its-180-minutes
kind: issue
title: The nightly's full-suite job runs its three ε rows in one 180-minute job, and since PR 6011 they no longer fit
status: closed
opened: 2026-10-30
closed: 2026-11-02
priority: P1
cost: E
---


Filed by SECT's `sect-seamslow` lane (branch `sect/seam-region-speed`,
2026-10-30), which un-holds the seam-region door rows at 1e-12 and was
asked to measure the job's headroom.

## Measured

`full-suite` (`.github/workflows/nightly.yml`, `timeout-minutes: 180`)
builds once and runs the whole suite at three ε rows in one step. Its
step times, from the jobs API of the scheduled nightlies (minutes):

| night | build | ε rows | after (doc-tests, names, scalpel, wasm32) | job |
|---|---|---|---|---|
| 10-24 | 60 | 82 | 5 | 148 |
| 10-25 | 61 | 93 | 5 | 160 |
| 10-26 | 64 | 95 | 6 | 166 |
| 10-27 | 65 | 99 | 6 | 171 |
| 10-28 | 50 | 66 | 6 | 123 |
| 10-29 | 73 | > 106, cancelled at the limit | — | 180 |
| 10-30 (`39bbdfc777`, PR 6011 in) | 51 | 78 | 6 | 135 |
| 10-31 (`6d3aa0fa7a`) | 76 | > 104, cancelled at the limit | — | 180 |

The seam-region door row was held out at 1e-12 on 10-30 and 10-31.
The build alone swings 50–76 minutes from night to night, which is as
much as the difference between a night that fits and one that does not.
The per-row split is in the job log only, which this lane could not
read past its tail (the log blob host is outside the session proxy), so
the ε rows are one figure here. On 10-27 the job ended 9 minutes inside
its limit.

What lands on top of those nights:

- PR 6011's torus germ rows at 1e-12 (opt-level 2, measured by its dual
  review: `work/sect/torus-germ-rows-march-for-an-hour-at-a-fine-band.md`):
  the tilted-rod spins row 1 015–1 770 s, the drum × donut row 400–700 s,
  the two donuts 520–900 s. That is 32–56 test-minutes, 8–14 minutes of
  the row on the runner's four test threads, and up to 30 minutes if the
  spins row is the last to start (it is one test). Their default-ε and
  1e-6 costs are not measured here.
- The seam-region door rows, un-held at 1e-12 by `sect/seam-region-speed`
  and split into five tests: on a 4-vCPU dev box at opt-level 2, 92 s at
  the default ε, 17 s at 1e-6, 620 s at 1e-12, no test over 146 s (the
  runner ran main's single row in 136 s at the default ε where that box
  takes 117 s). Against 10-28's 52 s that is about 4 minutes of the rows'
  wall clock.

The last two nights already carry PR 6011's rows. On 10-30 the job took
135 minutes, and on 10-31 it was cancelled with the build at 76 minutes
and the ε rows still running at 104. Un-holding the door rows adds 3–4
minutes to a job that already fails on a slow night: 10-30's shape
comes to about 139 minutes and fits, while a 76-minute build with
10-31's rows does not fit at any row cost.

## What closing it takes

The job's limit is a budget for one runner, and the work is three
independent rows, so the change is to stop running them in series:
build once and archive (`cargo nextest archive`), then run each ε row
as its own job from the archive (`--archive-file`), each with its own
limit, the steps after the rows in the default row's job. Wall clock is
about build + the slowest row + the steps after, near two hours on
these nights, and the build is paid once. A matrix over ε that builds
in each leg is the smaller edit at three times the build minutes.
Raising the limit only moves the date this row is filed again.

## The per-row split (GREEN's slow-test census, 2026-11-02)

Read from the full job logs through the jobs API (`gh api
repos/evgunter/cad/actions/jobs/<id>/logs`, which the session proxy
serves). Each row's wall clock is its nextest `Summary`; test-seconds
is the sum of every result line.

| night (run) | build | default | 1e-6 | 1e-12 | after | job |
|---|---|---|---|---|---|---|
| 10-31 (41172334943, cancelled) | 76 | 31.3 | 23.3 | > 48.6, cancelled at 10 864 of 16 745 tests | — | 180 |
| 11-01 (41276693129) | 52 | 17.2 | 13.4 | 40.3 | 6 | 129 |

- **The rows are saturated, not tail-bound**: on 11-01 every row's wall
  clock is its test-seconds over four threads to the tenth of a minute
  (4 122, 3 209 and 9 672 test-s). No single test sets a floor, so
  every test-second taken out is a quarter-second of wall clock off.
- **The 10-31 runner was about 1.75× slower per test** than 11-01's
  (the median ratio over the 154 tests above 5 s at the default ε is
  1.78, at 1e-6 1.72), as well as 24 minutes slower to build. The job's
  fit is a coin toss on the runner it draws.
- **1e-12 is the row to cut**: 40 of 71 row-minutes on 11-01, and
  sweep is 64 % of it (6 153 test-s). The 25 tests at or above 60 s are
  5 234 test-s, 54 % of the row; the top three (the tilted-rod spins row
  1 105 s, drum × donut 592 s, two donuts 574 s) are 23 %.
- **Debug assertions are not the cost**: side-by-side runs at opt-level
  2 with assertions and overflow checks on and off agree within 2 % on
  seven runs of six of the slowest tests; the numbers are on
  `work/sect/a-ball-and-cylinder-boolean-costs-seconds-per-op-in-an-optimized-build.md`.

Under the split below, the 1e-12 leg is that row alone: about 40
minutes on an 11-01 runner and about 70 on a 10-31 one, inside its
120-minute limit, with the build paid once before it.

## Closed

`green/full-suite-split` (2026-11-02). `nightly.yml`'s `full-suite-build`
builds once at opt-level 2 (the same env and `nightly-opt2` cache key),
writes a `cargo nextest archive`, uploads it as `full-suite-archive` (one
day), then runs the steps that were after the rows: doc-tests, the slow-set
name check, the topo scalpel, wasm32. `full-suite` is a matrix over default,
1e-6 and 1e-12: each leg checks out the tree, downloads the archive and runs
`cargo nextest run --archive-file … --workspace-remap "$GITHUB_WORKSPACE"
--profile ci --ignore-default-filter --no-fail-fast`, with
`scripts/nextest-failures.py` and the failing-tests step per leg and a
120-minute limit each. Each leg uploads its junit report as
`full-suite-junit-<eps>`, so per-test times outlive the log. The steps after
the rows sit in the build job rather than the default row's: they need the
warm target directory, and the default leg has none.
