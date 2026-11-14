---
id: the-full-suite-build-job-runs-within-eleven-minutes-of-its-limit
kind: issue
title: The nightly's full-suite build job ran 75 to 109 minutes against its 120-minute limit, and a timeout there leaves every eps row without a run
status: open
opened: 2026-11-14
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
