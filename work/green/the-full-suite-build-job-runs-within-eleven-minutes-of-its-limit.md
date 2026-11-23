---
id: the-full-suite-build-job-runs-within-eleven-minutes-of-its-limit
kind: issue
title: The nightly's full-suite build job ran 75 to 109 minutes against its 120-minute limit, and a timeout there leaves every eps row without a run
status: closed
opened: 2026-11-14
closed: 2026-11-22
priority: P2
cost: E
---

Filed by GREEN's `green/nightly-jobs` lane (2026-11-14), from the step
records of six nightlies while it read the jobs that do not finish.

## What

`full-suite-build` (`.github/workflows/nightly.yml`, `timeout-minutes:
120`) builds and archives every test binary at opt-level 2, then runs
the doc-tests, the slow-set name check, the topo scalpel and wasm32.

| nightly | head | build and archive | whole job |
|---|---|---|---|
| 42342466395 (11-07) | `45cae9d7` | 89 min | 100 min |
| 42567046151 (11-09) | `836426e0` | 83 min | 93 min |
| 42761779270 (11-10) | `50cfcf49` | 90 min | 101 min |
| 42965378060 (11-11) | `b2916558` | 67 min | 75 min |
| 43161026932 (11-12) | `da77d875` | 93 min | 105 min |
| 43371638371 (11-13) | `e36675c1` | 97 min | 109 min |
| 43561237293 (11-14) | `ff2b5f5d` | 98 min | 111 min |

On 11-13 the scalpel took 10 min of the rest. If the build step reaches
the limit, no archive is uploaded and none of the five full-suite legs
runs: every slow test at every eps goes unreported that night, and the
job's own red says only that it timed out.

## What would settle it

Where the 67 to 97 minutes go (the `nightly-opt2` cache's hit rate, the
opt-level 2 compile itself, the archive's compression), from the
step's own log on a run where the cache missed and one where it hit.
Moving the scalpel and wasm32 into their own job would buy the archive
those 11 minutes cheaply, but the build is the part that grows.

## Measured (2026-11-22, PR 7365)

The cache is not the variable. rust-cache restores only the 349 MB
dependency cache, hit or partial, so every workspace crate compiles cold
every night. Builds on the same image still range from 63 to 105 min.

Cargo `--timings` on probe run 44976645461 (4 cores) shows where the
time goes:

- The critical path is `editor-core` lib (28-35 min at opt 2), then its
  `tests/all.rs` (37-51 min).
- Viewer, pncad and pncad-py take 76 of the 410 unit-minutes at opt 2 and
  19 at opt 1, and their tests ran 1.0 and 1.1 min. Opt 1 saves about 55
  unit-minutes, about 13 wall-minutes on 4 cores.
- The other probe's build took 73 min against 104 min, but most of that
  gap is runner speed, not the leaf crates. Its non-leaf units also summed
  69 unit-minutes less, and the leaf change cannot account for those. On
  the nightly, editor-core lib took 2222 s with the leaf crates at opt 1,
  longer than the baseline probe's 2123 s at opt 2.

PR 7365 makes the opt-1 change and moves the topo scalpel (6-11 min) into
its own job. On slow runners `build and archive` went from 100.5-104.8 min
to 95.7 min, so of the job's 16-20 saved minutes about 10 come from the
scalpel move and the rest from opt 1. The 120-minute limit is unchanged.

## Closed (2026-11-22, PR 7365)

Nightly 44989234943 ran this job with the fix:

- Build step 95.8 min, whole job 98.3 min, so 22 min under the limit.
  Before, the job took 113-118 min.
- That runner was a slow one: `editor-core` lib took 2222 s, against 2123
  s on the slow baseline probe and 1693 s on the fast one. On the fast
  probe runner the same configuration built in 73 min.
- It also restored no dependency cache ("No cache found"), where main's
  nightlies restore about 349 MB, so the 98.3 min is pessimistic.
- The topo scalpel ran green as its own job in 11.4 min.

The rest of the build is limited by 4 cores' throughput and by the
editor-core chain, filed as
`the-full-suite-build-is-cpu-bound-on-four-cores-at-ninety-five-minutes-on-a-slow-runner`.
