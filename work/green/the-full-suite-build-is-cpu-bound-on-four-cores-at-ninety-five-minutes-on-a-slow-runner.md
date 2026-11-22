---
id: the-full-suite-build-is-cpu-bound-on-four-cores-at-ninety-five-minutes-on-a-slow-runner
kind: issue
title: The nightly's full-suite build is CPU-bound on one 4-core runner, so a slow runner still takes 95 of its 120 minutes
status: open
opened: 2026-11-22
priority: P2
cost: M
---


Filed by GREEN's `green/nightly-build` lane (PR 7365), from the build
job of nightly 44989234943, job 137167113008, whose timings are in that
run's `full-suite-build-timings` artifact.

## What

PR 7365 added swap, moved viewer, pncad and pncad-py to opt-level 1 and
gave the topo scalpel its own job. On that run the job finished in 98
min, against 114-118 min before. Its build step still took 96 min:

- The units sum to 376 unit-minutes on 4 cores, so the build is limited
  by throughput (376 / 4 is about 94 min).
- The critical path takes about the same time: `editor-core` lib takes
  37 min at opt 2, then its `tests/all.rs` takes 52 min, ending at 95.4
  min.

On the faster runner of probe run 44976645461, the same configuration
built in 73 min. So on a slow runner the margin is about 22 min. That run
also restored no dependency cache, so a normal night is a little faster.

**Trend.** On slow runners `build and archive` took 89 min (11-07), 97
(11-13), then 100-105 (11-17 to 11-21), before this change took about 10
min off it. Runner speed confounds that series by up to 1.5x, so the rate
is between nothing and about 1 min a day. At the top of that range, 22
min lasts about three weeks.

**Band.** P2. A build-step timeout costs every eps leg its run, which is
the P1 harm of the shutdown row, so this cannot wait for the next charter.
But the runway is weeks, not days, and
`the-nightly-build-job-holds-over-five-scheduled-main-nights` reads the
build step's time on every one of its five nights. If those show the
margin under 10 min, raise this to P1.

## What would settle it

Either lever cuts the work on one runner, and each has a cost:

- Split the archive across two runners, for example kernel test binaries
  in one and editor-core and its dependents in the other. Each eps leg
  would then run two archives and report one junit, and
  `full-suite-coverage` would read both listings.
- Lower the opt-level of the editor-core integration binary. It runs
  16-29 test-minutes per leg, so this needs the legs' timings at opt 1
  first.

Weigh either against the runner-minutes a second build job costs.
