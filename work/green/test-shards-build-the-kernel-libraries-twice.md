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

## The roster moved, 2026-10-29 (GREEN latency lane, owed item 3's second half)

The roster and the split are out of ci.yml's shell and in
`scripts/ci-filter.py` (`APP_SHARD`, `test_shards`), as two new outputs:
`TEST_SCOPE` (the closure plus test-utils) and `TEST_SHARDS` (the matrix).
ci.yml's filter job maps them to its `cargo_scope` and `test_shards`
outputs and no longer computes either. A build with no shard is now a
non-zero exit of the filter, so the classify step goes red where the
paths step did.

- **Same output.** The old shell block, run against the new function on
  439 inputs (`docs`, `--workspace`, every member alone, every member
  beside test-utils, and 400 random sorted member subsets), matched on
  every one, the no-shard error included.
- **Selftest rows** (`_selftest_test_shards`): app-only closure, kernel-only,
  both sides, `--workspace`, docs, lone test-utils, test-utils not added
  twice, and the no-shard error. Breaking the test-utils fold or the error
  check each reds one of them.
- **`ci-filter.py --selftest` ran in no CI job.** It takes 23 s on a hosted
  runner. The lint job runs it when the diff touches `scripts/ci-filter.py`
  or `scripts/slow_set.py`. Those diffs are TIER=all, where `test`, not
  `lint`, is the last job. On docs and aux runs `lint` is the last job, so
  an unconditional step would have added 23 s there. The nightly's
  gate-self-test job runs it every night, because its premise checks read
  the whole tree.
- **The roster is checked against the graph.** `APP_SHARD` is `APP_ROOT`
  (verbs) and every member that depends on it.
  `_selftest_app_shard_premise` reds when the member graph says
  otherwise, so a new dependent of verbs or editor-core cannot land in
  `kernel` silently.

The rebalance (items 1 and 2) is still owed; it is now a one-line edit to
`APP_SHARD` plus its selftest rows.

## Rebalanced, 2026-10-29 (GREEN latency lane, owed items 1 and 2)

**Closure-tier shard times since #5941** (ci.yml `pull_request` runs
from its merge, 10-28 17:43Z, to 10-29 09:00Z; 189 completed runs, 132
with a test shard). A run is counted TIER=all when every subject row
(tools, demos, interval, step import) ran, which only TIER=all forces.
Minutes are job wall, from the jobs API.

| tier | runs | `kernel` median / p90 (build) | `app` median / p90 (build) | slower shard median / p90 |
|---|---|---|---|---|
| closure | 107 kernel, 119 app | 14.4 / 17.9 (11.6) | 11.4 / 12.6 (10.1) | 14.2 / 17.7 |
| all | 13 | 18.9 / 19.4 (13.6) | 12.5 / 13.2 (10.0) | 18.9 / 19.4 |

`kernel` was the longer shard in 92 of the 107 closure runs that had
both, and in all 13 TIER=all runs. No closure run had `kernel` alone;
12 had `app` alone.

**Where the kernel shard's time goes** (local, 4 cores, the ci job's
profile env, deps warm, workspace crates cold: build 13m48s against
13.6 hosted).
- Build: two ~600 s units close it. topo's lib-test runs 225→829 s on
  topo's lib (19→217 s), and sweep's `all` runs 253→826 s. Next are topo
  `all` (391), mesh `all` (169) and mesh's lib-test (169), all
  competing with those two for 4 cores.
- Tests: 289 test-s at default ε, of which sweep is 142, topo 69,
  geom-brep 25, mesh 9.
- The app shard's build (10m28s) is 1,167 unit-s in 628 s, chain-bound
  on editor-core's lib (67→362 s) and `all` (375→628 s), so about 1,300
  unit-s of its cores sit idle.

**The move: mesh's tests into `app`.**
- mesh is the only package whose move fits. Its two test units (338
  unit-s) relieve kernel's contention without a long unit to wait on,
  and its tests run in 9 test-s, so `app` takes the build and almost no
  runtime.
- sweep is too big. The 10-28 run with sweep in `app` measured app 19.2
  and kernel 13.8, because `all` (573 s) moves with it.
- topo would put its 603 s lib-test into `app`, on top of the editor-core
  chain, in every topo-seeded closure.

**The rule.** `app` is verbs, every member depending on it, and
`APP_BALLAST` (mesh). Ballast is a timing choice the member graph
cannot express, so it is a hand-kept list.
`_selftest_app_shard_premise` checks `APP_SHARD` equals the closure plus
the ballast. It reds a ballast entry that is not a member, or is already
in the closure.

**Measured on #6019's own runs** (TIER=all). Hosted runners come in two
speeds: the same kernel build takes 9.3–9.7 min on a fast one and
13.4–14.7 on a slow one. So each shard is compared with its own runner
class.

| run | `kernel` (build) | `app` (build) |
|---|---|---|
| 40847689090 | 17.7 (12.5, slow) | 14.3 (11.7, slow) |
| 40857713276 | 11.9 (8.6, fast) | 14.0 (11.5, slow) |
| 40861550771 | 13.9 (9.9, fast) | 15.2 (12.6, slow) |
| 40864140159 | 18.3 (13.0, slow) | 14.6 (12.0, slow) |

- Kernel's build fell by 0.6–1.2 min on slow runners and 0.2–1.0 on
  fast ones. App's rose by 1.0–2.4 on slow runners (base 9.9–10.7).
- On slow runners the slower shard went from 18.9–19.4 to 17.7–18.3.
  That is about a minute at TIER=all.
- The TIER=all gap left is test runtime, not build. TIER=all runs every
  test at three ε, and kernel's runtime (4.6 min against app's 2.0)
  is half sweep's. No package move short of sweep's shifts it.
- The 17.7 run's `kernel` red is a seeded fuzz in geom, unrelated to
  this change. It is filed on tint's
  `turn-defect-speed-sup-bound-sits-one-ulp-under-an-interval-sample-at-the-ceiling`.

**Closure tier, estimated from those deltas.** Kernel 13.2–13.8 and app
12.4–13.8 at the median (14.4 and 11.4, with kernel's build −0.6 to
−1.2 and app's +1.0 to +2.4), within a minute of each other, which is the
balance criterion here. The estimate is from four runs; whether it
holds is owed item 1 below.

## Owed, 2026-10-29

1. After a day of closure runs on main with mesh in `app`, read the two
   shards' medians as above. Close this row if they are within 1 min of
   each other. Otherwise re-choose `APP_BALLAST` on those numbers.
