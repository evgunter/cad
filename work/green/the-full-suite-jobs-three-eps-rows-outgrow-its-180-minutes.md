---
id: the-full-suite-jobs-three-eps-rows-outgrow-its-180-minutes
kind: issue
title: The nightly's full-suite job runs its three ε rows in one 180-minute job, and since PR 6011 they no longer fit
status: open
opened: 2026-10-30
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
  review: `work/sectrung/torus-germ-rows-march-for-an-hour-at-a-fine-band.md`):
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
