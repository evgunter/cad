---
id: test-shards-build-the-kernel-libraries-twice
kind: issue
title: the two test shards each build the shared kernel libraries, about 600 job-min a day: build them once and hand them to both shards
status: open
opened: 2026-10-28
priority: P0
cost: M
parent: pr-ci-wall-clock-root-cause-measured
---


## What

#5941 splits the per-PR `test` job into an `app` and a `kernel` shard.
Both shards build the kernel libraries the app crates depend on, so a
code run pays that build twice: about +600 job-min a day by the
latency lane's estimate, about 2 points of a 20-slot day on top of the
28% PR runs use. The queue is empty now (p90 0.1 min), but at 45-74%
of capacity in early October the p90 queue reached 9-13 min, so the
headroom is worth keeping (the user, in chat, 2026-10-28).

## Owed

1. Measure the duplicated part: with a hosted `cargo build --timings`
   per shard, find how many of each shard's build minutes are the
   kernel libraries both shards compile.
2. Prototype a three-job layout: one job builds the shared libraries
   once and hands its `target/` to both shards (as an artifact or a
   run-keyed cache), and each shard builds only its own test binaries.
   Cargo decides a workspace crate is stale from source mtimes, and a
   fresh checkout resets them, so restore mtimes from git in the shard
   jobs (or the shards rebuild everything). Measure the handoff
   overhead: job setup plus `target/` upload and download.
3. Keep it if it saves job-minutes without adding more than about a
   minute to the median wall-clock. Otherwise rebalance the shards to
   shrink the shared part, and record the numbers here either way.

## Measured, 2026-10-28 (GREEN latency lane)

**How.** A throwaway step on `green/latency-shared-kernel-build` ran
`cargo test --no-run --timings <shard scope>` before each shard's build
and printed the timings report's units plus the workspace rlib names. It
ran on a measurement-only PR, #5953, closed unmerged. Run 40731565431 is
TIER=all, so both shards build their half of `--workspace`, with #5941's
roster (sweep in `kernel`).

| shard | build wall | ws lib units (unit-s) | ws test units (unit-s) | dep libs (unit-s, cached) | rlib bytes |
|---|---|---|---|---|---|
| kernel | 709 s | 13 (284) | 38 (1,854) | 171 (66) | 0.75 GB |
| app | 620 s | 18 (531, of which editor-core 291) | 14 (666) | 77 (32) | 1.29 GB |

**The shards' kernel libraries are different artifacts.** Every shared
workspace lib has a different hash in each shard, for example
`libtopo-0e24eec10f5e4608.rlib` in kernel and `libtopo-a2a6de035a0397b9.rlib`
in app, and likewise `geom_core`, `geom`, `geom_brep`, `bvh`, `profile`,
`mesh`, `sweep`, `step_export`, `step_import` and `stl`. Only `quantity` and
`test_utils` match.
- Cargo resolves features over the selected packages and their
  dev-dependencies, so the two selections unify the kernel's features
  differently.
- A `target/` built once cannot serve both shards: whichever selection
  it was built for, the other shard rebuilds every lib. Sharing one build
  first needs both shards to resolve the same features, for example a
  workspace-wide feature pin. That is a design change with its own cost,
  not a CI layout.

**The duplicated part is small, and mostly off the critical path.**
- In the app shard, the kernel libs it rebuilds (geom-core through sweep,
  mesh, step-*, stl) cost about 200 unit-s.
- Its critical path waits on them only until about 54 s: editor-core's
  lib starts at 54 s on sweep's rmeta and runs to 345 s, then
  editor-core's `all` test runs to 620 s. The app shard is chain-bound
  on editor-core.
- So a perfect handoff could save at most about 1 min of app wall-clock
  and about 200 unit-s of job time.

**Declined: the three-job layout.**
- It puts a lib-build job in series before both shards: job setup
  (about 0.5 min), cache restore, the kernel libs (topo's lib alone runs
  17→203 s in the kernel shard), and an upload of 0.5–1 GB of rlibs.
- Each shard then pays a download before it can start.
- That is at least +2 min of wall-clock against at most −1 min. With the
  hashes differing, it would not even hit without the feature change
  above.
- It fails the bar (no more than +1 min of median wall-clock) on these
  numbers, so it was not prototyped.

**What the split actually costs in job-minutes.** Measured on #5941's
TIER=all runs, app + kernel came to 25.0, 31.4, 31.3 and 31.4 job-min,
against the single job's median of 27.6. That is about +4 a run at
TIER=all, where both shards also run the whole suite at three ε. Part of
it is per-job overhead (runner setup, cache restore, nextest install,
doc-tests), not the libraries. The closure tier (70% of runs) is
unmeasured until #5941 is on main.

**The imbalance is the bigger lever.** At TIER=all `kernel` is the long
shard: 18.8–19.1 min in three of four runs (build 13.3–13.8, default ε
1.5, extra ε 3.1), against `app` at 12.2–12.8.
- `kernel` is throughput-bound. topo's lib-test (424 s), topo `all`
  (308) and sweep `all` (307) compete for 4 cores, and sweep `all` waits
  until 402 s to start.
- `app` is chain-bound on editor-core with cores to spare.
- One run with sweep in `app` measured app 19.2 (build 14.3) and kernel
  13.8, so the right boundary lies between the two layouts.

## Owed, 2026-10-28

1. After #5941 merges, read the closure-tier shard times from a day of
   real PR runs. The alarm is a closure-tier statistic (70% of runs), and
   at closure scope the kernel shard drops geom-core, geom, geom-brep and
   profile's tests, so a TIER=all balance may not be the closure balance.
2. Rebalance on that data. Candidates: sweep's `all` binary, or mesh's
   two test units (278 unit-s), into `app`, which has idle cores while
   editor-core's lib compiles.
3. With the rebalance, move the shard roster and split out of ci.yml's
   shell into `scripts/ci-filter.py`, the single implementation of change
   classification, as a `TEST_SHARDS` output with synthetic-diff
   selftests (#5941's review). The roster is a second, untested
   classification in ci.yml until then.
